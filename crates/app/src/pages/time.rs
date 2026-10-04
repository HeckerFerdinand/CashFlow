//! Zeiterfassung: working hours, Regie hours, vacation and sick days per month.

use crate::context::App;
use crate::forms::{self, FieldSpec, SectionSpec};
use crate::selection;
use crate::ui::{KeyValueItem, TimePage};
use cashflow_core::money::{format_flexible, parse_decimal};
use cashflow_core::{Period, TimeDraft, TimeRecord, ValidationErrors};
use rust_decimal::Decimal;
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct State {
    employee_ids: Vec<Uuid>,
    years: Vec<i32>,
    loaded: Option<Loaded>,
    draft: TimeDraft,
    errors: Option<ValidationErrors>,
    dirty: bool,
}

#[derive(Clone)]
struct Loaded {
    employee: Uuid,
    period: Period,
    record: Option<TimeRecord>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn sections() -> Vec<SectionSpec> {
    vec![
        SectionSpec::new(
            "Arbeitszeit",
            vec![
                FieldSpec::number("hours_worked", "Arbeitszeit", "Std.").required(),
                FieldSpec::number("extra_hours", "Arbeitszeit Regie", "Std."),
            ],
        ),
        SectionSpec::new(
            "Abwesenheit",
            vec![
                FieldSpec::number("vacation_days", "Urlaubstage", "Tage"),
                FieldSpec::number("sick_days", "Krankheitstage", "Tage"),
            ],
        ),
    ]
}

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
    let page = ui.global::<TimePage>();
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
        show_summary(&app);
    });

    let weak = Rc::downgrade(app);
    page.on_save(move || {
        if let Some(app) = weak.upgrade() {
            save(&app);
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
    let keep = selection::current_employee();
    let (ids, _) = selection::employees_for_year(app, period.year(), keep);
    STATE.with_borrow_mut(|s| {
        s.years = selection::years();
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
    app.run_db(
        "Monat wird geladen …",
        move |db| async move { db.time_record(employee, period).await },
        move |app, record| {
            let draft = record.as_ref().map(|r| TimeDraft::from_entry(&r.entry)).unwrap_or_default();
            STATE.with_borrow_mut(|s| {
                s.loaded = Some(Loaded { employee, period, record });
                s.draft = draft;
                s.errors = None;
                s.dirty = false;
            });
            show(app);
        },
    );
}

fn show(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<TimePage>();
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
        page.set_form(forms::build(&sections(), |key| s.draft.get(key).unwrap_or_default(), s.errors.as_ref()));
        if let Some(loaded) = &s.loaded {
            let name = app.cache.borrow().employee(loaded.employee).map(|e| e.data.full_name()).unwrap_or_default();
            page.set_exists(loaded.record.is_some());
            page.set_status(
                match loaded.record {
                    Some(_) => {
                        format!("Für {} sind bereits Zeiten gespeichert. Speichern überschreibt sie.", loaded.period)
                    }
                    None => format!("{} ist für {name} noch nicht erfasst.", loaded.period),
                }
                .into(),
            );
        }
    });
    show_summary(app);
}

fn show_summary(app: &Rc<App>) {
    let draft = STATE.with_borrow(|s| s.draft.clone());
    let hours = |text: &str| if text.trim().is_empty() { Some(Decimal::ZERO) } else { parse_decimal(text, 2).ok() };
    let days = |text: &str| if text.trim().is_empty() { Some(Decimal::ZERO) } else { parse_decimal(text, 1).ok() };
    let show = |value: Option<Decimal>, unit: &str, decimals: u32| match value {
        Some(v) => format!("{} {unit}", format_flexible(v, 0, decimals)),
        None => "–".into(),
    };
    let worked = hours(&draft.hours_worked);
    let extra = hours(&draft.extra_hours);
    let lines = vec![
        item("Arbeitszeit", &show(worked, "Std.", 2), false),
        item("Arbeitszeit Regie", &show(extra, "Std.", 2), false),
        item("Gesamtarbeitszeit", &show(worked.zip(extra).map(|(a, b)| a + b), "Std.", 2), true),
        item("Urlaubstage", &show(days(&draft.vacation_days), "Tage", 1), false),
        item("Krankheitstage", &show(days(&draft.sick_days), "Tage", 1), false),
    ];
    app.ui().global::<TimePage>().set_summary(ModelRc::new(VecModel::from(lines)));
}

fn item(key: &str, value: &str, strong: bool) -> KeyValueItem {
    KeyValueItem { key: key.into(), value: value.into(), strong }
}

fn save(app: &Rc<App>) {
    let Some(loaded) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    let validated = STATE.with_borrow(|s| s.draft.validate(loaded.employee, loaded.period));
    let entry = match validated {
        Ok(entry) => entry,
        Err(errors) => {
            STATE.with_borrow_mut(|s| s.errors = Some(errors));
            show(app);
            return;
        }
    };
    app.run_db(
        "Zeiten werden gespeichert …",
        move |db| async move { db.save_time(&entry).await },
        move |app, saved| {
            STATE.with_borrow_mut(|s| {
                if let Some(loaded) = s.loaded.as_mut() {
                    loaded.record = Some(saved.clone());
                }
                s.draft = TimeDraft::from_entry(&saved.entry);
                s.errors = None;
                s.dirty = false;
            });
            app.notify_success(format!("Zeiten für {} gespeichert.", saved.entry.period));
            show(app);
        },
    );
}

fn delete(app: &Rc<App>) {
    let Some(Loaded { record: Some(record), period, .. }) = STATE.with_borrow(|s| s.loaded.clone()) else { return };
    app.confirm(
        "Zeiten löschen?",
        &format!("Die gespeicherten Zeiten für {period} werden gelöscht."),
        "Löschen",
        move |app| {
            app.run_db(
                "Zeiten werden gelöscht …",
                move |db| async move { db.delete_time_record(record.id).await },
                move |app, ()| {
                    app.notify_success(format!("Zeiten für {period} gelöscht."));
                    discard_state();
                    load(app);
                },
            );
        },
    );
}
