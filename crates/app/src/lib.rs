//! The CashFlow desktop application: Slint UI (`ui/`) plus the Rust side that
//! loads and saves data (`pages/`), generates PDFs and stores settings.

pub mod ui {
    slint::include_modules!();
}

mod config;
mod context;
mod forms;
mod keychain;
#[cfg(target_os = "macos")]
mod macos;
mod output;
mod pages;
mod selection;
mod update;

use slint::{CloseRequestResponse, ComponentHandle};
use std::rc::Rc;

/// Starts the application and blocks until the window is closed.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    tracing::info!(version = update::CURRENT_VERSION, "CashFlow starting");

    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build()?;
    let window = ui::AppWindow::new()?;
    window.global::<ui::AppState>().set_version(update::CURRENT_VERSION.into());

    let app = context::App::new(&window, runtime.handle().clone(), config::Config::load());
    pages::install(&app);
    install_quit_handling(&app, &window);
    pages::login::try_resume(&app);

    window.run()?;
    tracing::info!("CashFlow closed");
    Ok(())
}

/// Closing the window (and on macOS also ⌘Q, the app menu and the Dock)
/// asks first while there are unsaved changes.
fn install_quit_handling(app: &Rc<context::App>, window: &ui::AppWindow) {
    let weak = Rc::downgrade(app);
    window.window().on_close_requested(move || match weak.upgrade() {
        Some(app) if !may_quit(&app) => CloseRequestResponse::KeepWindowShown,
        _ => CloseRequestResponse::HideWindow,
    });
    #[cfg(target_os = "macos")]
    {
        let weak = Rc::downgrade(app);
        macos::intercept_quit(move || weak.upgrade().is_none_or(|app| may_quit(&app)));
    }
}

/// Whether CashFlow may close right away. With unsaved changes it asks
/// instead and quits the event loop itself if the user agrees.
fn may_quit(app: &Rc<context::App>) -> bool {
    if !pages::any_unsaved_changes() {
        return true;
    }
    app.confirm(
        "CashFlow beenden?",
        "Es gibt ungespeicherte Änderungen. Trotzdem beenden und die Änderungen verwerfen?",
        "Beenden",
        |_| {
            let _ = slint::quit_event_loop();
        },
    );
    false
}

/// Logs to `cashflow.log` in the app's data folder (shown on the Info page).
fn init_logging() {
    use std::sync::Mutex;
    let file = config::data_dir().and_then(|dir| {
        std::fs::create_dir_all(&dir).ok()?;
        let path = dir.join("cashflow.log");
        // Start fresh once the log grows beyond 2 MB.
        if std::fs::metadata(&path).map(|m| m.len() > 2_000_000).unwrap_or(false) {
            let _ = std::fs::remove_file(&path);
        }
        std::fs::OpenOptions::new().create(true).append(true).open(path).ok()
    });
    let builder = tracing_subscriber::fmt().with_ansi(false).with_target(false);
    let _ = match file {
        Some(file) => builder.with_writer(Mutex::new(file)).try_init(),
        None => builder.try_init(),
    };
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("panic: {info}");
        default_hook(info);
    }));
}

/// Hooks for the screenshot tool (`examples/screenshots.rs`): run the real
/// app against a pre-signed-in database. Not part of the public API.
#[doc(hidden)]
pub mod testing {
    use super::*;
    use std::sync::Arc;

    pub use crate::context::App;
    pub use crate::ui::Page;

    pub fn start(window: &ui::AppWindow, runtime: tokio::runtime::Handle, db: Arc<cashflow_data::Database>) -> Rc<App> {
        window.global::<ui::AppState>().set_version(update::CURRENT_VERSION.into());
        let config = config::Config {
            check_for_updates: false,
            open_pdf_after_create: false,
            pdf_dir: Some(pdf_dir()),
            ..config::Config::default()
        };
        let app = context::App::new(window, runtime, config);
        pages::install(&app);
        install_quit_handling(&app, window);
        pages::login::enter_app(&app, db, "demo@cashflow.local".into());
        app
    }

    pub fn navigate(app: &Rc<App>, page: Page) {
        pages::navigate(app, page);
    }

    /// Folder the test runs write their PDFs to.
    pub fn pdf_dir() -> std::path::PathBuf {
        std::env::temp_dir().join("cashflow-dev-pdfs")
    }
}

pub fn log_file() -> Option<std::path::PathBuf> {
    config::data_dir().map(|dir| dir.join("cashflow.log"))
}
