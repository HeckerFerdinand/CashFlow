//! Application context shared by all pages: the window, the database
//! connection, cached master data and helpers for background work,
//! notifications and confirmations.

use crate::config::Config;
use crate::keychain::TokenStore;
use crate::ui::{AppState, AppWindow, Confirm, LoginPage, Notice, NoticeKind, Notices};
use cashflow_core::{Defaults, Employee, Employer};
use cashflow_data::{DataError, Database};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Master data used by several pages; reloaded after every change.
#[derive(Default, Clone)]
pub struct Cache {
    pub employers: Vec<Employer>,
    pub employees: Vec<Employee>,
    pub defaults: Defaults,
}

impl Cache {
    pub fn employer(&self, id: Uuid) -> Option<&Employer> {
        self.employers.iter().find(|e| e.id == id)
    }

    pub fn employee(&self, id: Uuid) -> Option<&Employee> {
        self.employees.iter().find(|e| e.id == id)
    }
}

/// Action waiting for the confirmation dialog.
type ConfirmAction = Box<dyn FnOnce(&Rc<App>)>;

pub struct App {
    window: slint::Weak<AppWindow>,
    runtime: tokio::runtime::Handle,
    pub config: RefCell<Config>,
    pub tokens: Arc<TokenStore>,
    db: RefCell<Option<Arc<Database>>>,
    pub cache: RefCell<Cache>,
    busy: Cell<u32>,
    notices: RefCell<Vec<Notice>>,
    next_notice: Cell<i32>,
    confirm_action: RefCell<Option<ConfirmAction>>,
    cancel_action: RefCell<Option<ConfirmAction>>,
}

impl App {
    pub fn new(window: &AppWindow, runtime: tokio::runtime::Handle, config: Config) -> Rc<Self> {
        let app = Rc::new(Self {
            window: window.as_weak(),
            runtime,
            config: RefCell::new(config),
            tokens: Arc::new(TokenStore::default()),
            db: RefCell::new(None),
            cache: RefCell::new(Cache::default()),
            busy: Cell::new(0),
            notices: RefCell::new(Vec::new()),
            next_notice: Cell::new(1),
            confirm_action: RefCell::new(None),
            cancel_action: RefCell::new(None),
        });
        app.install_shared_callbacks();
        app
    }

    /// The main window. Only call while the UI is running.
    pub fn ui(&self) -> AppWindow {
        self.window.upgrade().expect("main window alive")
    }

    pub fn db(&self) -> Option<Arc<Database>> {
        self.db.borrow().clone()
    }

    pub fn set_db(&self, db: Option<Arc<Database>>) {
        *self.db.borrow_mut() = db;
    }

    // -- background work ----------------------------------------------------------

    /// Runs `work` on the background runtime, then `done` with its result on
    /// the UI thread. Shows the busy indicator meanwhile.
    pub fn run<T, Fut>(self: &Rc<Self>, label: &str, work: Fut, done: impl FnOnce(&Rc<App>, T) + 'static)
    where
        T: Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        self.change_busy(1, label);
        let handle = self.runtime.spawn(work);
        let app = self.clone();
        let spawned = slint::spawn_local(async move {
            let result = handle.await;
            app.change_busy(-1, "");
            match result {
                Ok(value) => done(&app, value),
                Err(error) => {
                    tracing::error!(%error, "background task failed");
                    app.notify_error(format!("Interner Fehler: {error}"));
                }
            }
        });
        if let Err(error) = spawned {
            tracing::error!(%error, "event loop not running");
        }
    }

    /// Like [`App::run`] for database work: does nothing (with a message) if not
    /// signed in, and handles errors centrally; `done` only sees successes.
    pub fn run_db<T, Fut>(
        self: &Rc<Self>,
        label: &str,
        work: impl FnOnce(Arc<Database>) -> Fut,
        done: impl FnOnce(&Rc<App>, T) + 'static,
    ) where
        T: Send + 'static,
        Fut: Future<Output = Result<T, DataError>> + Send + 'static,
    {
        let Some(db) = self.db() else {
            self.session_lost("Bitte anmelden.");
            return;
        };
        self.run(label, work(db), move |app, result| match result {
            Ok(value) => done(app, value),
            Err(error) => app.report(&error),
        });
    }

    fn change_busy(&self, delta: i32, label: &str) {
        let count = (self.busy.get() as i32 + delta).max(0) as u32;
        self.busy.set(count);
        let ui = self.ui();
        let state = ui.global::<AppState>();
        state.set_busy(count > 0);
        if delta > 0 {
            state.set_busy_text(label.into());
        }
    }

    /// Shows a data error; a lost session returns to the login screen.
    pub fn report(self: &Rc<Self>, error: &DataError) {
        match error {
            DataError::SessionExpired | DataError::NotSignedIn => self.session_lost(&error.to_string()),
            other => self.notify_error(other.to_string()),
        }
    }

    /// Back to the login screen with a message.
    pub fn session_lost(self: &Rc<Self>, message: &str) {
        self.set_db(None);
        let ui = self.ui();
        ui.global::<AppState>().set_signed_in(false);
        ui.global::<LoginPage>().set_error(message.into());
        ui.global::<LoginPage>().set_working(false);
    }

    // -- notifications ----------------------------------------------------------

    pub fn notify_success(self: &Rc<Self>, text: impl Into<String>) {
        self.notify(NoticeKind::Success, text.into(), Duration::from_secs(4));
    }

    pub fn notify_info(self: &Rc<Self>, text: impl Into<String>) {
        self.notify(NoticeKind::Info, text.into(), Duration::from_secs(6));
    }

    pub fn notify_error(self: &Rc<Self>, text: impl Into<String>) {
        self.notify(NoticeKind::Error, text.into(), Duration::from_secs(12));
    }

    fn notify(self: &Rc<Self>, kind: NoticeKind, text: String, visible_for: Duration) {
        let id = self.next_notice.get();
        self.next_notice.set(id + 1);
        {
            let mut notices = self.notices.borrow_mut();
            notices.push(Notice { id, kind, text: text.into() });
            // Keep the stack short.
            let excess = notices.len().saturating_sub(4);
            notices.drain(..excess);
        }
        self.sync_notices();
        let app = Rc::downgrade(self);
        slint::Timer::single_shot(visible_for, move || {
            if let Some(app) = app.upgrade() {
                app.dismiss_notice(id);
            }
        });
    }

    fn dismiss_notice(&self, id: i32) {
        self.notices.borrow_mut().retain(|n| n.id != id);
        self.sync_notices();
    }

    fn sync_notices(&self) {
        let items = self.notices.borrow().clone();
        self.ui().global::<Notices>().set_items(ModelRc::new(VecModel::from(items)));
    }

    // -- confirmation dialog --------------------------------------------------------

    /// Asks before doing something irreversible; `action` runs on "confirm".
    pub fn confirm(
        self: &Rc<Self>,
        title: &str,
        message: &str,
        confirm_label: &str,
        action: impl FnOnce(&Rc<App>) + 'static,
    ) {
        *self.confirm_action.borrow_mut() = Some(Box::new(action));
        *self.cancel_action.borrow_mut() = None;
        let ui = self.ui();
        let confirm = ui.global::<Confirm>();
        confirm.set_title(title.into());
        confirm.set_message(message.into());
        confirm.set_confirm_label(confirm_label.into());
        // Deleting and discarding are shown in red.
        let lower = confirm_label.to_lowercase();
        confirm.set_destructive(lower.contains("lösch") || lower.contains("verwerfen") || lower.contains("beenden"));
        confirm.set_open(true);
    }

    /// Like [`App::confirm`], with an action for "Abbrechen".
    pub fn confirm_or(
        self: &Rc<Self>,
        title: &str,
        message: &str,
        confirm_label: &str,
        action: impl FnOnce(&Rc<App>) + 'static,
        on_cancel: impl FnOnce(&Rc<App>) + 'static,
    ) {
        self.confirm(title, message, confirm_label, action);
        *self.cancel_action.borrow_mut() = Some(Box::new(on_cancel));
    }

    fn install_shared_callbacks(self: &Rc<Self>) {
        let ui = self.ui();
        let weak = Rc::downgrade(self);
        ui.global::<Notices>().on_dismiss(move |id| {
            if let Some(app) = weak.upgrade() {
                app.dismiss_notice(id);
            }
        });

        let weak = Rc::downgrade(self);
        ui.global::<Confirm>().on_accepted(move || {
            let Some(app) = weak.upgrade() else { return };
            app.ui().global::<Confirm>().set_open(false);
            app.cancel_action.borrow_mut().take();
            let action = app.confirm_action.borrow_mut().take();
            if let Some(action) = action {
                action(&app);
            }
        });

        let weak = Rc::downgrade(self);
        ui.global::<Confirm>().on_rejected(move || {
            let Some(app) = weak.upgrade() else { return };
            app.ui().global::<Confirm>().set_open(false);
            app.confirm_action.borrow_mut().take();
            let cancel = app.cancel_action.borrow_mut().take();
            if let Some(cancel) = cancel {
                cancel(&app);
            }
        });
    }

    // -- master data ------------------------------------------------------------------

    /// Reloads employers, employees and defaults, then calls `done`.
    pub fn reload_cache(self: &Rc<Self>, done: impl FnOnce(&Rc<App>) + 'static) {
        self.run_db(
            "Daten werden geladen …",
            |db| async move {
                let (employers, employees, defaults) =
                    tokio::try_join!(db.list_employers(), db.list_employees(), db.defaults())?;
                Ok(Cache { employers, employees, defaults })
            },
            move |app, cache| {
                *app.cache.borrow_mut() = cache;
                done(app);
            },
        );
    }

    pub fn save_config(self: &Rc<Self>) {
        if let Err(error) = self.config.borrow().save() {
            self.notify_error(error);
        }
    }
}
