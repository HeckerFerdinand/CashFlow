//! Info: version, imprint, changelog, licenses, update check.

use crate::context::App;
use crate::ui::AboutPage;
use slint::ComponentHandle;
use std::rc::Rc;

const IMPRINT: &str = include_str!("../../../../assets/impressum.txt");
const DISCLAIMER: &str = include_str!("../../../../assets/disclaimer.txt");
const CHANGELOG: &str = include_str!("../../../../CHANGELOG.md");
const LICENSES: &str = include_str!("../../../../assets/licenses.txt");

/// The changelog without Markdown markup, for display.
fn plain_changelog() -> String {
    let mut out: Vec<String> = Vec::new();
    for line in CHANGELOG.lines().filter(|line| !line.starts_with("# ")) {
        if let Some(title) = line.strip_prefix("## ") {
            if !out.is_empty() {
                out.push(String::new());
            }
            out.push(title.to_string());
        } else if let Some(item) = line.strip_prefix("- ") {
            out.push(format!("• {item}"));
        } else if !line.trim().is_empty() {
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

pub fn install(app: &Rc<App>) {
    let ui = app.ui();
    let page = ui.global::<AboutPage>();
    page.set_imprint(IMPRINT.trim().into());
    page.set_disclaimer(DISCLAIMER.trim().into());
    page.set_changelog(plain_changelog().into());
    page.set_licenses(LICENSES.trim().into());

    let weak = Rc::downgrade(app);
    page.on_check_updates(move || {
        if let Some(app) = weak.upgrade() {
            crate::update::check_now(&app);
        }
    });
    let weak = Rc::downgrade(app);
    page.on_open_download(move || {
        if let Some(app) = weak.upgrade() {
            let url = app.config.borrow().download_url.clone();
            if let Err(error) = opener::open(&url) {
                app.notify_error(format!("{url} kann nicht geöffnet werden: {error}"));
            }
        }
    });
}

pub fn set_available_update(app: &Rc<App>, version: Option<String>) {
    let ui = app.ui();
    let page = ui.global::<AboutPage>();
    match version {
        Some(version) => {
            page.set_update_available(true);
            page.set_update_text(format!("Version {version} ist verfügbar.").into());
        }
        None => {
            page.set_update_available(false);
            page.set_update_text("CashFlow ist auf dem neuesten Stand.".into());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn changelog_is_readable() {
        let text = super::plain_changelog();
        assert!(text.starts_with(env!("CARGO_PKG_VERSION")), "newest changelog entry must match the version");
        assert!(text.contains("• Beta-Release"));
        assert!(!text.contains("##"));
        assert!(text.contains("Lohn- und Zeiterfassung"), "hyphens inside text stay");
        assert!(!text.contains("\n\n\n"));
    }
}
