//! Lohnerfassung: wage entry per employee and month, with live calculation.

use crate::context::App;
use crate::forms::{self, FieldSpec, SectionSpec};
use crate::output::{self, Job};
use crate::selection;
use crate::ui::{KeyValueItem, PayrollPage};
use cashflow_core::money::{format_eur, format_flexible, round_cents};
use cashflow_core::payroll::calculate;
use cashflow_core::validate::format_date;
use cashflow_core::{Contribution, Employee, PayrollDraft, PayrollRecord, Period, Rates, TimeRecord, ValidationErrors};
use chrono::Local;
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct State {
    employee_ids: Vec<Uuid>,
    years: Vec<i32>,
    /// Data of the selected month (None until loaded).
    loaded: Option<Loaded>,
    draft: PayrollDraft,
    hint: String,
    errors: Option<ValidationErrors>,
    dirty: bool,
}

#[derive(Clone)]
struct Loaded {
    employee: Uuid,
    period: Period,
    record: Option<PayrollRecord>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn sections() -> Vec<SectionSpec> {
    vec![SectionSpec::new(
        "Bezüge",
        vec![
            FieldSpec::date("statement_date", "Abrechnungsdatum").required(),
            FieldSpec::amount("base_pay", "Brutto-Bezüge").required(),
            FieldSpec::amount("extra_pay", "Sonstige Brutto-Bezüge (Regiestunden)"),
            FieldSpec::amount("payout", "Auszahlungsbetrag").placeholder("= Gesamt-Brutto"),
        ],
    )]
}

/// Opens the page for a given employee and month (from the overview).
pub fn preselect(employee: Uuid, period: Period) {
    selection::set_employee(Some(employee));
    selection::set_period(period);
    STATE.with_borrow_mut(|s| {
        s.loaded = None;
        s.dirty = false;
    });
}

pub fn is_dirty() -> bool {
    STATE.with_borrow(|s| s.dirty)
}

pub fn discard_state() {
    STATE.with_borrow_mut(|s| {
        s.dirty = false;
        s.loaded = None;
    });
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<PayrollPage>();
    page.set_months(selection::month_labels());

    let weak = Rc::downgrade(app);
    page.on_employee_changed(move |index| {
        let Some(app) = weak.upgrade() else { return };
        let id = STATE.with_borrow(|s| s.employee_ids.get(index as usize).copied());
        guard_unsaved(&app, move |app| {
            selection::set_employee(id);
            load(app);
        });
    });

    let weak = Rc::downgrade(app);
    page.on_period_changed(move |month, year| {
        let Some(app) = weak.upgrade() else { return };
        let year = STATE.with_borrow(|s| s.years.get(year as usize).copied());
        let Some(period) = year.and_then(|y| Period::new(y, month as u32 + 1).ok()) else { return };
        guard_unsaved(&app, move |app| {
            selection::set_period(period);
            refresh(app);
        });
    });

    let weak = Rc::downgrade(app);
    page.on_edited(move |key, value| {
        let Some(app) = weak.upgrade() else { return };
        STATE.with_borrow_mut(|s| {
            if s.draft.get(&key) != Some(value.as_str()) {
                s.draft.set(&key, value.as_str());
                s.dirty = true;
            }
        });
        // Only the calculation is updated while typing (keeps the input focus).
        show_calculation(&app);
    });

    let weak = Rc::downgrade(app);
    page.on_save(move || {
        if let Some(app) = weak.upgrade() {
            save(&app, false);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_save_and_print(move || {
        if let Some(app) = weak.upgrade() {
            save(&app, true);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_print(move || {
        let Some(app) = weak.upgrade() else { return };
        if is_dirty() {
            app.notify_info("Es gibt ungespeicherte Änderungen. Bitte zuerst speichern.");
            return;
        }
        if let Some(Loaded { employee, record: Some(record), .. }) = STATE.with_borrow(|s| s.loaded.clone()) {
            output::create(&app, employee, Job::Payslip(Box::new(record)));
        }
    });
    let weak = Rc::downgrade(app);
    page.on_delete(move || {
        if let Some(app) = weak.upgrade() {
            delete(&app);
        }
    });
}

fn guard_unsaved(app: &Rc<App>, action: impl FnOnce(&Rc<App>) + 'static) {
    if is_dirty() {
        app.confirm_or(
            "Ungespeicherte Änderungen",
            "Die Eingaben für diesen Monat wurden noch nicht gespeichert. Verwerfen?",
            "Verwerfen",
            move |app| {
                STATE.with_borrow_mut(|s| s.dirty = false);
                action(app);
            },
            show,
        );
    } else {
        action(app);
    }
}

pub fn refresh(app: &Rc<App>) {
    let period = selection::current_period();
    let years = selection::years();
    let keep = selection::current_employee();
    let (ids, _) = selection::employees_for_year(app, period.year(), keep);
    STATE.with_borrow_mut(|s| {
        s.years = years;
        // Keep the selection; otherwise start with the first employee.
        if keep.is_none_or(|id| !ids.contains(&id)) {
            selection::set_employee(selection::first_employed(app, &ids, period).or(ids.first().copied()));
        }
        s.employee_ids = ids;
    });
    load(app);
}

fn load(app: &Rc<App>) {
    let (employee, period) = (selection::current_employee(), selection::current_period());
    let Some(employee) = employee else {
        STATE.with_borrow_mut(|s| s.loaded = None);
        show(app);
        return;
    };
    let already = STATE
        .with_borrow(|s| s.loaded.as_ref().is_some_and(|l| l.employee == employee && l.period == period) && s.dirty);
    if already {
        show(app);
        return;
    }
    app.run_db(
        "Monat wird geladen …",
        move |db| async move {
            let (record, time) =
                tokio::try_join!(db.payroll_record(employee, period), db.time_record(employee, period))?;
            Ok((record, time))
        },
        move |app, (record, time)| {
            let Some(person) = app.cache.borrow().employee(employee).cloned() else { return };
            let (draft, hint) = match &record {
                Some(record) => (PayrollDraft::from_entry(&record.entry), String::new()),
                None => suggestion(&person, period, time.as_ref()),
            };
            STATE.with_borrow_mut(|s| {
                s.loaded = Some(Loaded { employee, period, record });
                s.draft = draft;
                s.hint = hint;
                s.errors = None;
                s.dirty = false;
            });
            show(app);
        },
    );
}

/// Prefills a new month from the master data and the time entry.
fn suggestion(employee: &Employee, period: Period, time: Option<&TimeRecord>) -> (PayrollDraft, String) {
    let today = Local::now().date_naive();
    let statement_date = today.clamp(period.first_day(), period.last_day());
    let mut hints = Vec::new();
    let base_pay = match employee.data.monthly_salary {
        Some(salary) => {
            hints.push(format!("Brutto-Bezüge aus der monatlichen Vergütung ({})", format_eur(salary)));
            cashflow_core::money::format_amount(salary)
        }
        None => String::new(),
    };
    let extra_pay = match (time.map(|t| t.entry.extra_hours), employee.data.hourly_rate) {
        (Some(hours), Some(rate)) if !hours.is_zero() => {
            let amount = round_cents(hours * rate);
            hints.push(format!(
                "Regiestunden aus der Zeiterfassung: {} Std. × {} = {}",
                format_flexible(hours, 0, 2),
                format_eur(rate),
                format_eur(amount)
            ));
            cashflow_core::money::format_amount(amount)
        }
        _ => "0,00".into(),
    };
    let hint =
        if hints.is_empty() { String::new() } else { format!("Vorschlag – bitte prüfen: {}.", hints.join("; ")) };
    (PayrollDraft { statement_date: format_date(statement_date), base_pay, extra_pay, payout: String::new() }, hint)
}

/// Rates used for the month: the stored snapshot, or the employee's current rates.
fn rates_for(app: &Rc<App>, loaded: &Loaded) -> Option<Rates> {
    match &loaded.record {
        Some(record) => Some(record.entry.rates),
        None => app.cache.borrow().employee(loaded.employee).map(|e| e.data.rates),
    }
}

fn show(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<PayrollPage>();
    let (period, employee) = (selection::current_period(), selection::current_employee());
    let (_, labels) = selection::employees_for_year(app, period.year(), employee);
    STATE.with_borrow(|s| {
        page.set_employees(labels);
        page.set_employee_index(employee.map(|id| selection::index_of(&s.employee_ids, &id)).unwrap_or(-1));
        page.set_years(selection::year_labels(&s.years));
        page.set_year_index(selection::index_of(&s.years, &period.year()));
        page.set_month_index(period.month() as i32 - 1);
        page.set_ready(s.loaded.is_some());
        page.set_error(s.errors.as_ref().map(|e| e.to_string()).unwrap_or_default().into());
        page.set_hint(s.hint.as_str().into());
        page.set_form(forms::build(&sections(), |key| s.draft.get(key).unwrap_or_default(), s.errors.as_ref()));
    });
    let Some(loaded) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    let cache = app.cache.borrow();
    let Some(person) = cache.employee(loaded.employee) else { return };
    let employer = cache.employer(person.data.employer_id);

    page.set_exists(loaded.record.is_some());
    let mut status = match &loaded.record {
        Some(record) => format!(
            "Für {} ist bereits ein Lohn gespeichert (Abrechnungsdatum {}). Speichern überschreibt ihn; es gelten die damals gespeicherten Beitragssätze.",
            loaded.period,
            format_date(record.entry.statement_date)
        ),
        None => format!("{} ist für {} noch nicht erfasst.", loaded.period, person.data.full_name()),
    };
    let employed = person.data.employment_start.is_none_or(|start| start <= loaded.period.last_day())
        && person.data.employment_end.is_none_or(|end| end >= loaded.period.first_day());
    if !employed {
        status.push_str(" Achtung: In diesem Monat besteht laut Stammdaten keine Beschäftigung.");
    }
    page.set_status(status.into());

    let optional_eur = |value: Option<rust_decimal::Decimal>| value.map(format_eur).unwrap_or_else(|| "–".into());
    let info = vec![
        line("Personalnummer", &person.data.personnel_number),
        line("Arbeitgeber", employer.map(|e| e.data.name.as_str()).unwrap_or("–")),
        line(
            "Betriebsnummer",
            employer.map(|e| e.data.company_number.as_str()).filter(|s| !s.is_empty()).unwrap_or("–"),
        ),
        line("Monatliche Vergütung", &optional_eur(person.data.monthly_salary)),
        line("Stundensatz Regiestunden", &optional_eur(person.data.hourly_rate)),
        line("Krankenkasse", if person.data.health_insurer.is_empty() { "–" } else { &person.data.health_insurer }),
    ];
    page.set_info(ModelRc::new(VecModel::from(info)));
    drop(cache);
    show_calculation(app);
}

/// Live preview of gross pay and contributions from the current input.
fn show_calculation(app: &Rc<App>) {
    let Some(loaded) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    let Some(rates) = rates_for(app, &loaded) else { return };
    let draft = STATE.with_borrow(|s| s.draft.clone());
    let base = cashflow_core::money::parse_amount(&draft.base_pay).ok();
    let extra = if draft.extra_pay.trim().is_empty() {
        Some(rust_decimal::Decimal::ZERO)
    } else {
        cashflow_core::money::parse_amount(&draft.extra_pay).ok()
    };
    let dash = || "–".to_string();
    let mut lines = vec![
        line("Brutto-Bezüge", &base.map(format_eur).unwrap_or_else(dash)),
        line("Sonstige Bezüge", &extra.map(format_eur).unwrap_or_else(dash)),
    ];
    match base.zip(extra) {
        Some((base, extra)) => {
            let result = calculate(base, extra, &rates);
            lines.push(strong("Gesamt-Brutto", &format_eur(result.gross_total)));
            for contribution in Contribution::ALL {
                lines.push(line(
                    &format!("{} ({} %)", contribution.label(), format_flexible(rates.get(contribution), 0, 4)),
                    &format_eur(result.contributions.get(contribution)),
                ));
            }
            lines.push(strong("Abgaben Arbeitgeber", &format_eur(result.total_contribution)));
            let payout = if draft.payout.trim().is_empty() {
                Some(result.gross_total)
            } else {
                cashflow_core::money::parse_amount(&draft.payout).ok()
            };
            if let Some(payout) = payout {
                if payout < result.gross_total {
                    lines.push(line("Abzüge", &format_eur(result.gross_total - payout)));
                }
                lines.push(strong("Auszahlung", &format_eur(payout)));
            }
        }
        None => lines.push(line("Gesamt-Brutto", "Eingabe prüfen")),
    }
    app.ui().global::<PayrollPage>().set_calculation(ModelRc::new(VecModel::from(lines)));
}

fn line(key: &str, value: &str) -> KeyValueItem {
    KeyValueItem { key: key.into(), value: value.into(), strong: false }
}

fn strong(key: &str, value: &str) -> KeyValueItem {
    KeyValueItem { key: key.into(), value: value.into(), strong: true }
}

fn save(app: &Rc<App>, print: bool) {
    let Some(loaded) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    let Some(rates) = rates_for(app, &loaded) else { return };
    let validated = STATE.with_borrow(|s| s.draft.validate(loaded.employee, loaded.period, rates));
    let entry = match validated {
        Ok(entry) => entry,
        Err(errors) => {
            STATE.with_borrow_mut(|s| s.errors = Some(errors));
            show(app);
            return;
        }
    };
    app.run_db(
        "Lohn wird gespeichert …",
        move |db| async move { db.save_payroll(&entry).await },
        move |app, saved| {
            let employee = saved.entry.employee_id;
            STATE.with_borrow_mut(|s| {
                if let Some(loaded) = s.loaded.as_mut() {
                    loaded.record = Some(saved.clone());
                }
                s.draft = PayrollDraft::from_entry(&saved.entry);
                s.hint.clear();
                s.errors = None;
                s.dirty = false;
            });
            app.notify_success(format!("Lohn für {} gespeichert.", saved.entry.period));
            show(app);
            if print {
                output::create(app, employee, Job::Payslip(Box::new(saved)));
            }
        },
    );
}

fn delete(app: &Rc<App>) {
    let Some(Loaded { record: Some(record), period, .. }) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    app.confirm(
        "Lohn löschen?",
        &format!("Der gespeicherte Lohn für {period} wird gelöscht. Bereits erstellte PDFs bleiben erhalten."),
        "Löschen",
        move |app| {
            app.run_db(
                "Lohn wird gelöscht …",
                move |db| async move { db.delete_payroll_record(record.id).await },
                move |app, ()| {
                    app.notify_success(format!("Lohn für {period} gelöscht."));
                    discard_state();
                    load(app);
                },
            );
        },
    );
}
