//! Übersicht: which months are recorded, per employee and year.

use crate::context::App;
use crate::ui::{MonthCell, Nav, OverviewPage, OverviewRow, OverviewStat, Page};
use cashflow_core::Period;
use cashflow_data::RecordedMonths;
use chrono::{Datelike, Local};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::cell::Cell;
use std::rc::Rc;
use uuid::Uuid;

const HEADERS: [&str; 12] = ["JAN", "FEB", "MÄR", "APR", "MAI", "JUN", "JUL", "AUG", "SEP", "OKT", "NOV", "DEZ"];

thread_local! {
    static YEAR: Cell<i32> = Cell::new(Local::now().year());
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<OverviewPage>();
    page.set_month_headers(ModelRc::new(VecModel::from(HEADERS.map(SharedString::from).to_vec())));

    let weak = Rc::downgrade(app);
    page.on_previous_year(move || {
        if let Some(app) = weak.upgrade() {
            YEAR.set(YEAR.get() - 1);
            refresh(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_next_year(move || {
        if let Some(app) = weak.upgrade() {
            YEAR.set(YEAR.get() + 1);
            refresh(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_open(move |employee_id, month, kind| {
        let Some(app) = weak.upgrade() else { return };
        let (Ok(employee_id), Ok(period)) = (Uuid::parse_str(&employee_id), Period::new(YEAR.get(), month as u32 + 1))
        else {
            return;
        };
        let target = if kind == 0 { Page::Payroll } else { Page::Time };
        if target == Page::Payroll {
            super::payroll::preselect(employee_id, period);
        } else {
            super::time::preselect(employee_id, period);
        }
        app.ui().global::<Nav>().invoke_navigate(target);
    });
}

pub fn refresh(app: &Rc<App>) {
    let year = YEAR.get();
    app.ui().global::<OverviewPage>().set_year(year.to_string().into());
    app.run_db(
        "Übersicht wird geladen …",
        move |db| async move { db.recorded_months(year).await },
        move |app, months| show(app, year, &months),
    );
}

fn show(app: &Rc<App>, year: i32, months: &RecordedMonths) {
    let today = Local::now().date_naive();
    let current_month = if today.year() == year { today.month() as i32 - 1 } else { -1 };
    let cache = app.cache.borrow();
    let mut employees: Vec<_> = cache.employees.iter().filter(|e| e.data.is_employed_in_year(year)).collect();
    employees.sort_by_key(|e| e.data.sort_name().to_lowercase());

    let mut expected = 0;
    let mut payroll_done = 0;
    let mut time_done = 0;
    let rows: Vec<OverviewRow> = employees
        .iter()
        .map(|employee| {
            let cells: Vec<MonthCell> = Period::months_of(year)
                .map(|period| {
                    let month = period.month();
                    let active = employee.data.employment_start.is_none_or(|start| start <= period.last_day())
                        && employee.data.employment_end.is_none_or(|end| end >= period.first_day());
                    let cell = MonthCell {
                        payroll: months.has_payroll(employee.id, month),
                        time: months.has_time(employee.id, month),
                        active,
                    };
                    // Count months that are due (up to the current month).
                    if active && period.first_day() <= today {
                        expected += 1;
                        payroll_done += cell.payroll as i32;
                        time_done += cell.time as i32;
                    }
                    cell
                })
                .collect();
            let employer = cache.employer(employee.data.employer_id).map(|e| e.data.name.as_str()).unwrap_or("");
            OverviewRow {
                id: employee.id.to_string().into(),
                name: employee.data.sort_name().into(),
                subtitle: format!("{} · {employer}", employee.data.personnel_number).into(),
                months: ModelRc::new(VecModel::from(cells)),
            }
        })
        .collect();

    let stats = vec![
        OverviewStat {
            label: "Mitarbeiter".into(),
            value: employees.len().to_string().into(),
            detail: format!("beschäftigt in {year}").into(),
        },
        OverviewStat {
            label: "Löhne erfasst".into(),
            value: format!("{payroll_done} / {expected}").into(),
            detail: "fällige Monate bis heute".into(),
        },
        OverviewStat {
            label: "Zeiten erfasst".into(),
            value: format!("{time_done} / {expected}").into(),
            detail: "fällige Monate bis heute".into(),
        },
    ];

    let ui = app.ui();
    let page = ui.global::<OverviewPage>();
    page.set_current_month(current_month);
    page.set_rows(ModelRc::new(VecModel::from(rows)));
    page.set_stats(ModelRc::new(VecModel::from(stats)));
}
