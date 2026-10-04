//! Mitarbeiter: list, search, create, edit, delete.

use crate::context::App;
use crate::forms::{self, FieldSpec, SectionSpec, none_choice};
use crate::ui::{EmployeesPage, ListItem};
use cashflow_core::{Contribution, EmployeeData, EmployeeDraft, Gender, TransitionZone, ValidationErrors};
use chrono::Local;
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct State {
    /// `None` while nothing is open; `Some(None)` for a new employee.
    open: Option<Option<Uuid>>,
    draft: EmployeeDraft,
    errors: Option<ValidationErrors>,
    dirty: bool,
    search: String,
    show_former: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn sections(app: &App) -> Vec<SectionSpec> {
    let mut employers: Vec<(String, String)> = vec![(String::new(), "– bitte wählen –".into())];
    employers.extend(app.cache.borrow().employers.iter().map(|e| (e.id.to_string(), e.data.name.clone())));
    let mut genders = vec![none_choice()];
    genders.extend(Gender::ALL.iter().map(|g| (g.code().to_string(), g.label().to_string())));
    let mut zones = vec![none_choice()];
    zones.extend(TransitionZone::ALL.iter().map(|z| (z.code().to_string(), z.label().to_string())));
    let rates = Contribution::ALL.iter().map(|c| FieldSpec::percent(c.key(), c.label()).required()).collect();

    vec![
        SectionSpec::new(
            "Persönliche Daten",
            vec![
                FieldSpec::text("first_name", "Vorname").required(),
                FieldSpec::text("last_name", "Nachname").required(),
                FieldSpec::text("birth_name", "Geburtsname"),
                FieldSpec::date("birth_date", "Geburtsdatum"),
                FieldSpec::choice("gender", "Geschlecht", genders),
                FieldSpec::text("nationality", "Staatsangehörigkeit"),
            ],
        ),
        SectionSpec::new(
            "Anschrift",
            vec![
                FieldSpec::text("street", "Straße").wide(),
                FieldSpec::text("house_number", "Hausnummer"),
                FieldSpec::text("postal_code", "Postleitzahl"),
                FieldSpec::text("city", "Ort").wide(),
            ],
        ),
        SectionSpec::new(
            "Beschäftigung",
            vec![
                FieldSpec::choice("employer_id", "Arbeitgeber", employers).required().wide(),
                FieldSpec::text("personnel_number", "Personalnummer").required(),
                FieldSpec::text("occupation", "Berufsbezeichnung"),
                FieldSpec::date("employment_start", "Beschäftigungsbeginn"),
                FieldSpec::date("employment_end", "Austritt"),
                FieldSpec::amount("monthly_salary", "Monatliche Vergütung"),
                FieldSpec::amount("hourly_rate", "Stundensatz Regiestunden").suffix("€/Std."),
            ],
        ),
        SectionSpec::new(
            "Sozialversicherung und Steuer",
            vec![
                FieldSpec::text("social_security_number", "SV-Nummer").placeholder("12 150388 W 042"),
                FieldSpec::text("tax_id", "Steuer-ID").placeholder("11 Ziffern"),
                FieldSpec::text("health_insurer", "Krankenkasse").wide(),
                FieldSpec::text("person_group", "Personengruppe (PGRS)").placeholder("z. B. 109"),
                FieldSpec::text("contribution_group_key", "Beitragsgruppe (BGRS)").placeholder("z. B. 6500"),
                FieldSpec::text("activity_key", "Tätigkeitsschlüssel").placeholder("9 Ziffern"),
                FieldSpec::choice("transition_zone", "Übergangsbereich (Gleitzone)", zones).wide(),
            ],
        ),
        SectionSpec::described(
            "Beitragssätze",
            "Pauschale Abgaben des Arbeitgebers in Prozent vom Bruttolohn. Neue Mitarbeiter erhalten die \
             Standardwerte aus den Einstellungen. Bereits abgerechnete Monate behalten ihre damaligen Sätze.",
            rates,
        ),
    ]
}

pub fn is_dirty() -> bool {
    STATE.with_borrow(|s| s.dirty)
}

pub fn discard_state() {
    STATE.with_borrow_mut(|s| {
        s.open = None;
        s.dirty = false;
        s.errors = None;
    });
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<EmployeesPage>();

    let weak = Rc::downgrade(app);
    page.on_select(move |id| {
        let Some(app) = weak.upgrade() else { return };
        let Ok(id) = Uuid::parse_str(&id) else { return };
        guard_unsaved(&app, move |app| open(app, Some(id)));
    });

    let weak = Rc::downgrade(app);
    page.on_create(move || {
        if let Some(app) = weak.upgrade() {
            guard_unsaved(&app, |app| open(app, None));
        }
    });

    page.on_edited(|key, value| {
        STATE.with_borrow_mut(|s| {
            if s.draft.get(&key) != Some(value.as_str()) {
                s.draft.set(&key, value.as_str());
                s.dirty = true;
            }
        });
    });

    let weak = Rc::downgrade(app);
    page.on_save(move || {
        if let Some(app) = weak.upgrade() {
            save(&app);
        }
    });

    let weak = Rc::downgrade(app);
    page.on_discard(move || {
        if let Some(app) = weak.upgrade() {
            guard_unsaved(&app, |app| {
                discard_state();
                show(app);
            });
        }
    });

    let weak = Rc::downgrade(app);
    page.on_delete(move || {
        if let Some(app) = weak.upgrade() {
            delete(&app);
        }
    });

    let weak = Rc::downgrade(app);
    page.on_search(move |text| {
        let Some(app) = weak.upgrade() else { return };
        STATE.with_borrow_mut(|s| s.search = text.to_lowercase());
        show_list(&app);
    });

    let weak = Rc::downgrade(app);
    page.on_show_former_changed(move |show| {
        let Some(app) = weak.upgrade() else { return };
        STATE.with_borrow_mut(|s| s.show_former = show);
        show_list(&app);
    });
}

fn guard_unsaved(app: &Rc<App>, action: impl FnOnce(&Rc<App>) + 'static) {
    if is_dirty() {
        app.confirm(
            "Ungespeicherte Änderungen",
            "Die Änderungen an diesem Mitarbeiter wurden noch nicht gespeichert. Verwerfen?",
            "Verwerfen",
            move |app| {
                STATE.with_borrow_mut(|s| s.dirty = false);
                action(app);
            },
        );
    } else {
        action(app);
    }
}

pub fn refresh(app: &Rc<App>) {
    let exists = |id: Uuid| app.cache.borrow().employee(id).is_some();
    STATE.with_borrow_mut(|s| {
        if let Some(Some(id)) = s.open
            && !exists(id)
            && !s.dirty
        {
            s.open = None;
        }
    });
    show(app);
}

fn open(app: &Rc<App>, id: Option<Uuid>) {
    let draft = {
        let cache = app.cache.borrow();
        match id {
            Some(id) => match cache.employee(id) {
                Some(employee) => EmployeeDraft::from_data(&employee.data),
                None => return,
            },
            None => {
                let mut draft = EmployeeDraft::new_with_defaults(&cache.defaults);
                if let [only] = cache.employers.as_slice() {
                    draft.employer_id = only.id.to_string();
                }
                draft
            }
        }
    };
    STATE.with_borrow_mut(|s| {
        s.open = Some(id);
        s.draft = draft;
        s.errors = None;
        s.dirty = false;
    });
    show(app);
}

fn show(app: &Rc<App>) {
    show_list(app);
    let ui = app.ui();
    let page = ui.global::<EmployeesPage>();
    let specs = sections(app);
    STATE.with_borrow(|s| {
        page.set_editing(s.open.is_some());
        page.set_is_new(matches!(s.open, Some(None)));
        page.set_selected_id(match s.open {
            Some(Some(id)) => id.to_string().into(),
            _ => "".into(),
        });
        let name = format!("{} {}", s.draft.first_name.trim(), s.draft.last_name.trim());
        page.set_title(match s.open {
            Some(None) => "Neuer Mitarbeiter".into(),
            _ if name.trim().is_empty() => "Mitarbeiter".into(),
            _ => name.trim().into(),
        });
        page.set_error(match &s.errors {
            Some(errors) => errors.to_string().into(),
            None => "".into(),
        });
        page.set_show_former(s.show_former);
        page.set_form(forms::build(&specs, |key| s.draft.get(key).unwrap_or_default(), s.errors.as_ref()));
    });
}

fn show_list(app: &Rc<App>) {
    let (search, show_former) = STATE.with_borrow(|s| (s.search.clone(), s.show_former));
    let today = Local::now().date_naive();
    let cache = app.cache.borrow();
    let items: Vec<ListItem> = cache
        .employees
        .iter()
        .filter(|e| show_former || !e.data.has_left_before(today))
        .filter(|e| {
            search.is_empty()
                || e.data.full_name().to_lowercase().contains(&search)
                || e.data.personnel_number.to_lowercase().contains(&search)
        })
        .map(|e| {
            let employer = cache.employer(e.data.employer_id).map(|x| x.data.name.as_str()).unwrap_or("");
            let former = e.data.has_left_before(today);
            ListItem {
                id: e.id.to_string().into(),
                title: e.data.sort_name().into(),
                subtitle: format!("{} · {employer}", e.data.personnel_number).into(),
                badge: if former { "ausgeschieden".into() } else { "".into() },
                muted: former,
            }
        })
        .collect();
    app.ui().global::<EmployeesPage>().set_items(ModelRc::new(VecModel::from(items)));
}

fn save(app: &Rc<App>) {
    if app.cache.borrow().employers.is_empty() {
        app.notify_error("Bitte zuerst unter „Arbeitgeber“ einen Arbeitgeber anlegen.");
        return;
    }
    let (open, validated) = STATE.with_borrow(|s| (s.open, s.draft.validate()));
    let Some(id) = open else { return };
    let data: EmployeeData = match validated {
        Ok(data) => data,
        Err(errors) => {
            STATE.with_borrow_mut(|s| s.errors = Some(errors));
            show(app);
            return;
        }
    };
    app.run_db(
        "Mitarbeiter wird gespeichert …",
        move |db| async move {
            match id {
                Some(id) => db.update_employee(id, &data).await,
                None => db.create_employee(&data).await,
            }
        },
        move |app, saved| {
            app.notify_success(format!("{} wurde gespeichert.", saved.data.full_name()));
            STATE.with_borrow_mut(|s| {
                s.open = Some(Some(saved.id));
                s.draft = EmployeeDraft::from_data(&saved.data);
                s.errors = None;
                s.dirty = false;
            });
            app.reload_cache(show);
        },
    );
}

fn delete(app: &Rc<App>) {
    let Some(Some(id)) = STATE.with_borrow(|s| s.open) else { return };
    let name = app.cache.borrow().employee(id).map(|e| e.data.full_name()).unwrap_or_default();
    app.run_db(
        "Prüfe gespeicherte Monate …",
        move |db| async move { db.count_employee_records(id).await },
        move |app, counts| {
            let message = format!(
                "{name} wird zusammen mit {} Lohn- und {} Zeitmonaten endgültig gelöscht. Bereits erstellte PDFs \
                 bleiben erhalten.\n\nLohnunterlagen sind mehrere Jahre aufzubewahren. Für ausgeschiedene \
                 Mitarbeiter genügt meist ein Austrittsdatum.",
                counts.payroll_months, counts.time_months
            );
            app.confirm("Mitarbeiter löschen?", &message, "Endgültig löschen", move |app| {
                app.run_db(
                    "Mitarbeiter wird gelöscht …",
                    move |db| async move { db.delete_employee(id).await },
                    move |app, ()| {
                        app.notify_success(format!("{name} wurde gelöscht."));
                        discard_state();
                        app.reload_cache(show);
                    },
                );
            });
        },
    );
}
