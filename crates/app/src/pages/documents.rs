//! Dokumente: payslips and yearly journals as PDF.

use crate::context::App;
use crate::output::{self, Job};
use crate::selection;
use crate::ui::DocumentsPage;
use cashflow_core::Period;
use chrono::{Datelike, Local};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct State {
    /// Employees in the order of the payslip combo box.
    employee_ids: Vec<Uuid>,
    years: Vec<i32>,
    initialized: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<DocumentsPage>();
    page.set_months(selection::month_labels());

    let weak = Rc::downgrade(app);
    page.on_create_payslip(move || {
        if let Some(app) = weak.upgrade() {
            create_payslip(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_create_payroll_journal(move || {
        if let Some(app) = weak.upgrade() {
            create_journals(&app, false);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_create_time_journal(move || {
        if let Some(app) = weak.upgrade() {
            create_journals(&app, true);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_open_folder(move || {
        if let Some(app) = weak.upgrade() {
            output::open_pdf_folder(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_change_folder(move || {
        if let Some(app) = weak.upgrade()
            && output::choose_pdf_folder(&app)
        {
            refresh_folder(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_open_last(move || {
        if let (Some(app), Some(path)) = (weak.upgrade(), output::last_file()) {
            output::open_path(&app, &path);
        }
    });
}

pub fn refresh(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<DocumentsPage>();
    let years = selection::years();
    let cache = app.cache.borrow();
    let mut employees: Vec<_> = cache.employees.iter().collect();
    employees.sort_by_key(|e| e.data.sort_name().to_lowercase());
    let today = Local::now().date_naive();
    let labels: Vec<SharedString> = employees
        .iter()
        .map(|e| {
            let suffix = if e.data.has_left_before(today) { " (ausgeschieden)" } else { "" };
            format!("{}{suffix}", e.list_label()).into()
        })
        .collect();
    let mut journal_labels = vec![SharedString::from("Alle Mitarbeiter")];
    journal_labels.extend(labels.iter().cloned());

    let previous_ids = STATE.with_borrow(|s| s.employee_ids.clone());
    let selected_payslip = previous_ids.get(page.get_payslip_employee() as usize).copied();
    let selected_journal = (page.get_journal_employee() > 0)
        .then(|| previous_ids.get(page.get_journal_employee() as usize - 1).copied())
        .flatten();
    let ids: Vec<Uuid> = employees.iter().map(|e| e.id).collect();

    page.set_employees(ModelRc::new(VecModel::from(labels)));
    page.set_journal_employees(ModelRc::new(VecModel::from(journal_labels)));
    page.set_years(selection::year_labels(&years));

    drop(cache);
    let initialized = STATE.with_borrow(|s| s.initialized);
    if initialized {
        page.set_payslip_employee(selected_payslip.map(|id| selection::index_of(&ids, &id)).unwrap_or(-1));
        page.set_journal_employee(selected_journal.map(|id| selection::index_of(&ids, &id) + 1).unwrap_or(0));
    } else {
        let period = selection::default_period();
        let first = selection::first_employed(app, &ids, period).map(|id| selection::index_of(&ids, &id));
        page.set_payslip_employee(first.unwrap_or(if ids.is_empty() { -1 } else { 0 }));
        page.set_payslip_month(period.month() as i32 - 1);
        page.set_payslip_year(selection::index_of(&years, &period.year()));
        page.set_journal_employee(0);
        page.set_journal_year(selection::index_of(&years, &(today.year() - 1)));
    }
    STATE.with_borrow_mut(|s| {
        s.employee_ids = ids;
        s.years = years;
        s.initialized = true;
    });
    refresh_folder(app);
}

/// Updates the folder line (also called after PDFs were written).
pub fn refresh_folder(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<DocumentsPage>();
    page.set_folder(app.config.borrow().pdf_dir().display().to_string().into());
    page.set_last_file(output::last_file().map(|p| p.display().to_string()).unwrap_or_default().into());
}

fn year_at(index: i32) -> Option<i32> {
    STATE.with_borrow(|s| s.years.get(index as usize).copied())
}

fn create_payslip(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<DocumentsPage>();
    let employee = STATE.with_borrow(|s| s.employee_ids.get(page.get_payslip_employee() as usize).copied());
    let Some(employee) = employee else {
        app.notify_error("Bitte einen Mitarbeiter wählen.");
        return;
    };
    let Some(period) =
        year_at(page.get_payslip_year()).and_then(|y| Period::new(y, page.get_payslip_month() as u32 + 1).ok())
    else {
        return;
    };
    app.run_db(
        "Lohn wird geladen …",
        move |db| async move { db.payroll_record(employee, period).await },
        move |app, record| match record {
            Some(record) => output::create(app, employee, Job::Payslip(Box::new(record))),
            None => {
                let name = app.cache.borrow().employee(employee).map(|e| e.data.full_name()).unwrap_or_default();
                app.notify_error(format!("Für {name} ist {period} noch kein Lohn erfasst (siehe „Lohnerfassung“)."));
            }
        },
    );
}

/// Creates the yearly wage (`time == false`) or time journal for one or all employees.
fn create_journals(app: &Rc<App>, time: bool) {
    let ui = app.ui();
    let page = ui.global::<DocumentsPage>();
    let Some(year) = year_at(page.get_journal_year()) else { return };
    let selection = page.get_journal_employee();
    let targets: Vec<_> = {
        let cache = app.cache.borrow();
        let ids = STATE.with_borrow(|s| s.employee_ids.clone());
        let chosen: Vec<Uuid> = if selection <= 0 {
            cache.employees.iter().filter(|e| e.data.is_employed_in_year(year)).map(|e| e.id).collect()
        } else {
            ids.get(selection as usize - 1).copied().into_iter().collect()
        };
        chosen
            .into_iter()
            .filter_map(|id| {
                let employee = cache.employee(id)?.clone();
                let employer = cache.employer(employee.data.employer_id)?.clone();
                Some((employer, employee))
            })
            .collect()
    };
    if targets.is_empty() {
        app.notify_error(format!("{year} war kein Mitarbeiter beschäftigt."));
        return;
    }
    let base = app.config.borrow().pdf_dir();
    let single = targets.len() == 1;
    app.run_db(
        if time { "Zeitjournale werden erstellt …" } else { "Lohnjournale werden erstellt …" },
        move |db| async move {
            let mut created = Vec::new();
            let mut skipped = Vec::new();
            for (employer, employee) in targets {
                let job = if time {
                    Job::TimeJournal(year, db.time_records_of_year(employee.id, year).await?)
                } else {
                    Job::PayrollJournal(year, db.payroll_records_of_year(employee.id, year).await?)
                };
                let empty = match &job {
                    Job::TimeJournal(_, records) => records.is_empty(),
                    Job::PayrollJournal(_, records) => records.is_empty(),
                    Job::Payslip(_) => false,
                };
                if empty {
                    skipped.push(employee.data.full_name());
                    continue;
                }
                let base = base.clone();
                let result =
                    tokio::task::spawn_blocking(move || output::render_and_save(&base, &employer, &employee, &job))
                        .await
                        .unwrap_or_else(|error| Err(error.to_string()));
                created.push(result);
            }
            Ok((created, skipped))
        },
        move |app, (created, skipped): (Vec<Result<std::path::PathBuf, String>>, Vec<String>)| {
            let mut paths = Vec::new();
            for result in created {
                match result {
                    Ok(path) => paths.push(path),
                    Err(error) => app.notify_error(error),
                }
            }
            if !skipped.is_empty() {
                let what = if time { "Zeiten" } else { "Löhne" };
                if single {
                    app.notify_error(format!("Für {} sind {year} keine {what} erfasst.", skipped.join(", ")));
                } else {
                    app.notify_info(format!("Ohne {what} in {year} (übersprungen): {}.", skipped.join(", ")));
                }
            }
            output::finished(app, &paths);
        },
    );
}
