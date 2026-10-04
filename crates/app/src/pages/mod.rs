//! One module per page. Each module wires the callbacks of its Slint global
//! (`install`) and reloads its content when the page is shown (`refresh`).

pub mod about;
pub mod documents;
pub mod employees;
pub mod employers;
pub mod login;
pub mod overview;
pub mod payroll;
pub mod settings;
pub mod time;

use crate::context::App;
use crate::ui::{Nav, Page};
use slint::ComponentHandle;
use std::rc::Rc;

pub fn install(app: &Rc<App>) {
    login::install(app);
    overview::install(app);
    payroll::install(app);
    time::install(app);
    documents::install(app);
    employees::install(app);
    employers::install(app);
    settings::install(app);
    about::install(app);

    let weak = Rc::downgrade(app);
    app.ui().global::<Nav>().on_navigate(move |page| {
        if let Some(app) = weak.upgrade() {
            navigate(&app, page);
        }
    });
}

/// Switches pages, asking first if the current page has unsaved changes.
pub fn navigate(app: &Rc<App>, page: Page) {
    let current = app.ui().global::<Nav>().get_page();
    if current == page || !has_unsaved_changes(current) {
        show(app, page);
        return;
    }
    app.confirm(
        "Ungespeicherte Änderungen",
        "Die Änderungen auf dieser Seite wurden noch nicht gespeichert. Trotzdem wechseln und die Änderungen verwerfen?",
        "Verwerfen",
        move |app| {
            discard_changes(current);
            show(app, page);
        },
    );
}

fn show(app: &Rc<App>, page: Page) {
    app.ui().global::<Nav>().set_page(page);
    refresh(app, page);
}

pub fn refresh(app: &Rc<App>, page: Page) {
    match page {
        Page::Overview => overview::refresh(app),
        Page::Payroll => payroll::refresh(app),
        Page::Time => time::refresh(app),
        Page::Documents => documents::refresh(app),
        Page::Employees => employees::refresh(app),
        Page::Employers => employers::refresh(app),
        Page::Settings => settings::refresh(app),
        Page::About => {}
    }
}

/// True if the page holds edits that were not saved.
pub fn has_unsaved_changes(page: Page) -> bool {
    match page {
        Page::Payroll => payroll::is_dirty(),
        Page::Time => time::is_dirty(),
        Page::Employees => employees::is_dirty(),
        Page::Employers => employers::is_dirty(),
        Page::Settings => settings::is_dirty(),
        Page::Overview | Page::Documents | Page::About => false,
    }
}

pub fn any_unsaved_changes() -> bool {
    payroll::is_dirty() || time::is_dirty() || employees::is_dirty() || employers::is_dirty() || settings::is_dirty()
}

fn discard_changes(page: Page) {
    match page {
        Page::Payroll => payroll::discard_state(),
        Page::Time => time::discard_state(),
        Page::Employees => employees::discard_state(),
        Page::Employers => employers::discard_state(),
        Page::Settings => settings::discard_state(),
        Page::Overview | Page::Documents | Page::About => {}
    }
}
