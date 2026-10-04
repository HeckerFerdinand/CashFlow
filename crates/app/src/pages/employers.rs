//! Arbeitgeber: list, search, create, edit, delete.

use crate::context::App;
use crate::forms::{self, FieldSpec, SectionSpec};
use crate::ui::{EmployersPage, ListItem};
use cashflow_core::{EmployerData, EmployerDraft, ValidationErrors};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Default)]
struct State {
    /// `None` while nothing is open; `Some(None)` for a new employer.
    open: Option<Option<Uuid>>,
    draft: EmployerDraft,
    errors: Option<ValidationErrors>,
    dirty: bool,
    search: String,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn sections() -> Vec<SectionSpec> {
    vec![
        SectionSpec::new(
            "Arbeitgeber",
            vec![
                FieldSpec::text("name", "Name").required().wide(),
                FieldSpec::text("representative", "Vertretung / Zusatz").wide().placeholder("z. B. vertreten durch …"),
                FieldSpec::text("company_number", "Betriebsnummer").placeholder("8 Ziffern"),
                FieldSpec::text("tax_number", "Steuernummer"),
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
    let page = ui.global::<EmployersPage>();

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
}

/// Runs `action` right away, or after confirming that edits may be dropped.
fn guard_unsaved(app: &Rc<App>, action: impl FnOnce(&Rc<App>) + 'static) {
    if is_dirty() {
        app.confirm(
            "Ungespeicherte Änderungen",
            "Die Änderungen an diesem Arbeitgeber wurden noch nicht gespeichert. Verwerfen?",
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
    // Drop the selection if the employer no longer exists.
    let exists = |id: Uuid| app.cache.borrow().employer(id).is_some();
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
    let draft = match id {
        Some(id) => match app.cache.borrow().employer(id) {
            Some(employer) => EmployerDraft::from_data(&employer.data),
            None => return,
        },
        None => EmployerDraft::default(),
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
    let page = ui.global::<EmployersPage>();
    STATE.with_borrow(|s| {
        page.set_editing(s.open.is_some());
        page.set_is_new(matches!(s.open, Some(None)));
        page.set_selected_id(match s.open {
            Some(Some(id)) => id.to_string().into(),
            _ => "".into(),
        });
        page.set_title(match s.open {
            Some(None) => "Neuer Arbeitgeber".into(),
            _ if s.draft.name.trim().is_empty() => "Arbeitgeber".into(),
            _ => s.draft.name.trim().into(),
        });
        page.set_error(match &s.errors {
            Some(errors) => errors.to_string().into(),
            None => "".into(),
        });
        page.set_form(forms::build(&sections(), |key| s.draft.get(key).unwrap_or_default(), s.errors.as_ref()));
    });
}

fn show_list(app: &Rc<App>) {
    let search = STATE.with_borrow(|s| s.search.clone());
    let cache = app.cache.borrow();
    let items: Vec<ListItem> = cache
        .employers
        .iter()
        .filter(|e| {
            search.is_empty()
                || e.data.name.to_lowercase().contains(&search)
                || e.data.address.city.to_lowercase().contains(&search)
        })
        .map(|e| {
            let staff = cache.employees.iter().filter(|p| p.data.employer_id == e.id).count();
            ListItem {
                id: e.id.to_string().into(),
                title: e.data.name.as_str().into(),
                subtitle: e.data.address.single_line().into(),
                badge: match staff {
                    0 => "".into(),
                    1 => "1 Mitarbeiter".into(),
                    n => format!("{n} Mitarbeiter").into(),
                },
                muted: false,
            }
        })
        .collect();
    app.ui().global::<EmployersPage>().set_items(ModelRc::new(VecModel::from(items)));
}

fn save(app: &Rc<App>) {
    let (open, validated) = STATE.with_borrow(|s| (s.open, s.draft.validate()));
    let Some(id) = open else { return };
    let data: EmployerData = match validated {
        Ok(data) => data,
        Err(errors) => {
            STATE.with_borrow_mut(|s| s.errors = Some(errors));
            show(app);
            return;
        }
    };
    app.run_db(
        "Arbeitgeber wird gespeichert …",
        move |db| async move {
            match id {
                Some(id) => db.update_employer(id, &data).await,
                None => db.create_employer(&data).await,
            }
        },
        move |app, saved| {
            app.notify_success(format!("„{}“ wurde gespeichert.", saved.data.name));
            STATE.with_borrow_mut(|s| {
                s.open = Some(Some(saved.id));
                s.draft = EmployerDraft::from_data(&saved.data);
                s.errors = None;
                s.dirty = false;
            });
            app.reload_cache(show);
        },
    );
}

fn delete(app: &Rc<App>) {
    let Some(Some(id)) = STATE.with_borrow(|s| s.open) else { return };
    let (name, staff) = {
        let cache = app.cache.borrow();
        let name = cache.employer(id).map(|e| e.data.name.clone()).unwrap_or_default();
        (name, cache.employees.iter().filter(|e| e.data.employer_id == id).count())
    };
    if staff > 0 {
        app.notify_error(format!(
            "„{name}“ kann nicht gelöscht werden, solange {staff} Mitarbeiter zugeordnet sind. \
             Bitte die Mitarbeiter zuerst einem anderen Arbeitgeber zuordnen oder löschen."
        ));
        return;
    }
    app.confirm("Arbeitgeber löschen?", &format!("„{name}“ wird endgültig gelöscht."), "Löschen", move |app| {
        app.run_db(
            "Arbeitgeber wird gelöscht …",
            move |db| async move { db.delete_employer(id).await },
            move |app, ()| {
                app.notify_success(format!("„{name}“ wurde gelöscht."));
                discard_state();
                app.reload_cache(show);
            },
        );
    });
}
