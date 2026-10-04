//! Sign-in, "stay signed in" and the connection settings.

use crate::context::App;
use crate::keychain;
use crate::pages;
use crate::ui::{AppState, LoginPage, Nav, Page};
use cashflow_data::{ConnectionSettings, DataError, Database};
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::Arc;

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<LoginPage>();
    {
        let config = app.config.borrow();
        page.set_email(config.last_email.as_str().into());
        page.set_remember(config.remember_login);
        let connection = config.connection();
        page.set_url(connection.url.as_str().into());
        page.set_api_key(connection.api_key.as_str().into());
        page.set_show_connection(!config.has_connection());
    }

    let weak = Rc::downgrade(app);
    page.on_sign_in(move || {
        if let Some(app) = weak.upgrade() {
            sign_in(&app);
        }
    });

    let weak = Rc::downgrade(app);
    page.on_save_connection(move || {
        if let Some(app) = weak.upgrade() {
            save_connection(&app);
        }
    });

    let weak = Rc::downgrade(app);
    ui.global::<AppState>().on_sign_out(move || {
        if let Some(app) = weak.upgrade() {
            sign_out(&app);
        }
    });
}

fn database(app: &Rc<App>, settings: &ConnectionSettings) -> Result<Arc<Database>, DataError> {
    let tokens = app.tokens.clone();
    Ok(Arc::new(
        Database::new(settings)?.with_session_listener(Arc::new(move |session| tokens.on_session_changed(session))),
    ))
}

fn sign_in(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<LoginPage>();
    let email = page.get_email().trim().to_string();
    let password = page.get_password().to_string();
    let remember = page.get_remember();
    let settings = app.config.borrow().connection();
    let db = match database(app, &settings) {
        Ok(db) => db,
        Err(error) => {
            page.set_error(error.to_string().into());
            page.set_show_connection(true);
            return;
        }
    };
    page.set_error("".into());
    page.set_working(true);
    app.tokens.remember(remember.then(|| email.clone()));
    if !remember {
        keychain::delete_token(&email);
    }
    let session_db = db.clone();
    let account = email.clone();
    app.run("Anmeldung …", async move { session_db.sign_in(&account, &password).await }, move |app, result| {
        let ui = app.ui();
        let page = ui.global::<LoginPage>();
        page.set_working(false);
        match result {
            Ok(user) => {
                page.set_password("".into());
                {
                    let mut config = app.config.borrow_mut();
                    config.last_email = email.clone();
                    config.remember_login = remember;
                }
                app.save_config();
                enter_app(app, db, user.email.unwrap_or(email));
            }
            Err(error) => page.set_error(error.to_string().into()),
        }
    });
}

/// Tries the stored refresh token at startup ("angemeldet bleiben").
pub fn try_resume(app: &Rc<App>) {
    let (remember, email, settings) = {
        let config = app.config.borrow();
        (config.remember_login, config.last_email.clone(), config.connection())
    };
    if !remember || email.is_empty() || settings.validated_base_url().is_err() {
        return;
    }
    let Some(token) = keychain::load_token(&email) else { return };
    let Ok(db) = database(app, &settings) else { return };
    app.tokens.remember(Some(email.clone()));
    let ui = app.ui();
    ui.global::<LoginPage>().set_working(true);
    let session_db = db.clone();
    app.run(
        "Anmeldung wird wiederhergestellt …",
        async move { session_db.resume(&token).await },
        move |app, result| {
            app.ui().global::<LoginPage>().set_working(false);
            match result {
                Ok(user) => enter_app(app, db, user.email.unwrap_or(email)),
                Err(DataError::SessionExpired) => {}
                Err(error) => app.ui().global::<LoginPage>().set_error(error.to_string().into()),
            }
        },
    );
}

pub(crate) fn enter_app(app: &Rc<App>, db: Arc<Database>, email: String) {
    app.set_db(Some(db));
    let ui = app.ui();
    ui.global::<AppState>().set_user_email(email.into());
    ui.global::<LoginPage>().set_error("".into());
    app.reload_cache(|app| {
        let ui = app.ui();
        ui.global::<AppState>().set_signed_in(true);
        ui.global::<Nav>().set_page(Page::Overview);
        pages::refresh(app, Page::Overview);
        crate::update::check_in_background(app);
    });
}

fn sign_out(app: &Rc<App>) {
    let Some(db) = app.db() else { return };
    app.set_db(None);
    *app.cache.borrow_mut() = Default::default();
    app.ui().global::<AppState>().set_signed_in(false);
    app.run("Abmelden …", async move { db.sign_out().await }, |_, _| {});
}

fn save_connection(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<LoginPage>();
    let settings = ConnectionSettings::new(page.get_url().trim(), page.get_api_key().trim());
    match settings.validated_base_url() {
        Ok(base_url) => {
            {
                let mut config = app.config.borrow_mut();
                config.supabase_url = base_url.clone();
                config.supabase_key = settings.api_key.clone();
            }
            page.set_url(base_url.into());
            app.save_config();
            page.set_error("".into());
            page.set_show_connection(false);
            app.notify_success("Verbindung gespeichert. Jetzt anmelden.");
        }
        Err(error) => page.set_error(error.to_string().into()),
    }
}
