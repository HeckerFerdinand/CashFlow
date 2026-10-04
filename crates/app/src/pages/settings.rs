//! Einstellungen: local settings of this computer, shared defaults for new
//! employees, password and database connection.

use crate::context::App;
use crate::forms::{self, FieldSpec, SectionSpec};
use crate::output;
use crate::ui::{AppState, LoginPage, SettingsPage};
use cashflow_core::{Contribution, DefaultsDraft, ValidationErrors};
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct State {
    defaults: DefaultsDraft,
    errors: Option<ValidationErrors>,
    dirty: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn defaults_section() -> SectionSpec {
    let mut fields: Vec<FieldSpec> =
        Contribution::ALL.iter().map(|c| FieldSpec::percent(c.key(), c.label()).required()).collect();
    fields.push(FieldSpec::text("health_insurer", "Krankenkasse").wide());
    SectionSpec::described(
        "Standardwerte für neue Mitarbeiter",
        "Gilt für alle Benutzer. Bestehende Mitarbeiter und bereits erfasste Monate werden nicht verändert.",
        fields,
    )
}

fn defaults_value<'a>(draft: &'a DefaultsDraft, key: &str) -> &'a str {
    if key == "health_insurer" {
        return &draft.health_insurer;
    }
    Contribution::ALL.iter().position(|c| c.key() == key).map(|i| draft.rates[i].as_str()).unwrap_or_default()
}

pub fn is_dirty() -> bool {
    STATE.with_borrow(|s| s.dirty)
}

pub fn discard_state() {
    STATE.with_borrow_mut(|s| {
        s.dirty = false;
        s.errors = None;
    });
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<SettingsPage>();

    let weak = Rc::downgrade(app);
    page.on_change_folder(move || {
        if let Some(app) = weak.upgrade()
            && output::choose_pdf_folder(&app)
        {
            show(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_open_folder(move || {
        if let Some(app) = weak.upgrade() {
            output::open_pdf_folder(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_local_changed(move || {
        let Some(app) = weak.upgrade() else { return };
        let ui = app.ui();
        let page = ui.global::<SettingsPage>();
        {
            let mut config = app.config.borrow_mut();
            config.open_pdf_after_create = page.get_open_after_create();
            config.check_for_updates = page.get_check_updates();
        }
        app.save_config();
    });
    page.on_defaults_edited(|key, value| {
        STATE.with_borrow_mut(|s| {
            if defaults_value(&s.defaults, &key) != value.as_str() {
                s.defaults.set(&key, value.as_str());
                s.dirty = true;
            }
        });
    });
    let weak = Rc::downgrade(app);
    page.on_save_defaults(move || {
        if let Some(app) = weak.upgrade() {
            save_defaults(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_change_password(move || {
        if let Some(app) = weak.upgrade() {
            change_password(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_change_connection(move || {
        let Some(app) = weak.upgrade() else { return };
        app.confirm(
            "Verbindung ändern?",
            "Dafür wirst du abgemeldet. Auf der Anmeldeseite kannst du dann Projekt-URL und Schlüssel ändern.",
            "Abmelden",
            |app| {
                app.ui().global::<LoginPage>().set_show_connection(true);
                app.ui().global::<AppState>().invoke_sign_out();
            },
        );
    });
    let weak = Rc::downgrade(app);
    page.on_open_log(move || {
        if let (Some(app), Some(path)) = (weak.upgrade(), crate::log_file()) {
            output::open_path(&app, &path);
        }
    });
}

pub fn refresh(app: &Rc<App>) {
    if !is_dirty() {
        let draft = DefaultsDraft::from_defaults(&app.cache.borrow().defaults);
        STATE.with_borrow_mut(|s| {
            s.defaults = draft;
            s.errors = None;
        });
    }
    show(app);
}

fn show(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<SettingsPage>();
    let config = app.config.borrow();
    page.set_pdf_folder(config.pdf_dir().display().to_string().into());
    page.set_open_after_create(config.open_pdf_after_create);
    page.set_check_updates(config.check_for_updates);
    page.set_connection_url(config.connection().url.into());
    page.set_email(ui.global::<AppState>().get_user_email());
    page.set_log_file(crate::log_file().map(|p| p.display().to_string()).unwrap_or_default().into());
    STATE.with_borrow(|s| {
        let section = defaults_section();
        let model =
            forms::build(std::slice::from_ref(&section), |key| defaults_value(&s.defaults, key), s.errors.as_ref());
        use slint::Model;
        if let Some(section) = model.row_data(0) {
            page.set_defaults(section);
        }
        page.set_defaults_error(s.errors.as_ref().map(|e| e.to_string()).unwrap_or_default().into());
    });
}

fn save_defaults(app: &Rc<App>) {
    let validated = STATE.with_borrow(|s| s.defaults.validate());
    let defaults = match validated {
        Ok(defaults) => defaults,
        Err(errors) => {
            STATE.with_borrow_mut(|s| s.errors = Some(errors));
            show(app);
            return;
        }
    };
    app.run_db(
        "Standardwerte werden gespeichert …",
        move |db| async move { db.save_defaults(&defaults).await },
        |app, saved| {
            app.cache.borrow_mut().defaults = saved;
            STATE.with_borrow_mut(|s| {
                s.dirty = false;
                s.errors = None;
            });
            app.notify_success("Standardwerte gespeichert.");
            refresh(app);
        },
    );
}

fn change_password(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<SettingsPage>();
    let password = page.get_new_password().to_string();
    if password != page.get_repeat_password().as_str() {
        page.set_password_error("Die beiden Eingaben stimmen nicht überein.".into());
        return;
    }
    if password.chars().count() < 8 {
        page.set_password_error("Das Passwort muss mindestens 8 Zeichen lang sein.".into());
        return;
    }
    page.set_password_error("".into());
    app.run_db(
        "Passwort wird geändert …",
        move |db| async move { db.change_password(&password).await },
        |app, ()| {
            let ui = app.ui();
            let page = ui.global::<SettingsPage>();
            page.set_new_password("".into());
            page.set_repeat_password("".into());
            app.notify_success("Das Passwort wurde geändert.");
        },
    );
}
