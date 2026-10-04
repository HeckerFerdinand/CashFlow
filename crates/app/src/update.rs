//! Update check: compares the running version with the version published at
//! `Config::update_url` (a plain text file such as "2.0.1").

use crate::context::App;
use std::rc::Rc;

/// Parses "2.0.1" / "v2.0.1" into comparable numbers.
pub fn parse_version(text: &str) -> Option<Vec<u64>> {
    let text = text.trim().trim_start_matches(['v', 'V']);
    let parts: Option<Vec<u64>> = text.split('.').map(|p| p.trim().parse().ok()).collect();
    parts.filter(|p| !p.is_empty())
}

pub fn is_newer(candidate: &str, current: &str) -> bool {
    match (parse_version(candidate), parse_version(current)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

async fn fetch_latest(url: String) -> Result<String, String> {
    let client =
        reqwest::Client::builder().timeout(std::time::Duration::from_secs(10)).build().map_err(|e| e.to_string())?;
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }
    let text = response.text().await.map_err(|e| e.to_string())?;
    Ok(text.lines().next().unwrap_or_default().trim().to_string())
}

/// Checks once after sign-in; only reports when a newer version exists.
pub fn check_in_background(app: &Rc<App>) {
    let (enabled, url) = {
        let config = app.config.borrow();
        (config.check_for_updates, config.update_url.clone())
    };
    if !enabled || url.trim().is_empty() {
        return;
    }
    app.run("", fetch_latest(url), |app, result| match result {
        Ok(latest) if is_newer(&latest, CURRENT_VERSION) => {
            app.notify_info(format!(
                "Version {latest} ist verfügbar (installiert: {CURRENT_VERSION}). Download unter „Info“."
            ));
            crate::pages::about::set_available_update(app, Some(latest));
        }
        Ok(_) => crate::pages::about::set_available_update(app, None),
        Err(error) => tracing::info!(%error, "update check failed"),
    });
}

/// Manual check from the Info page, always reports the result.
pub fn check_now(app: &Rc<App>) {
    let url = app.config.borrow().update_url.clone();
    app.run("Suche nach Updates …", fetch_latest(url), |app, result| match result {
        Ok(latest) if is_newer(&latest, CURRENT_VERSION) => {
            app.notify_info(format!("Version {latest} ist verfügbar."));
            crate::pages::about::set_available_update(app, Some(latest));
        }
        Ok(_) => {
            app.notify_success(format!("CashFlow ist aktuell (Version {CURRENT_VERSION})."));
            crate::pages::about::set_available_update(app, None);
        }
        Err(error) => app.notify_error(format!("Update-Prüfung fehlgeschlagen: {error}")),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_versions() {
        assert!(is_newer("2.0.1", "2.0.0"));
        assert!(is_newer("v2.1", "2.0.9"));
        assert!(!is_newer("2.0.0", "2.0.0"));
        assert!(!is_newer("1.1.1", "2.0.0"), "older remote versions must not trigger a notice");
        assert!(!is_newer("garbage", "2.0.0"));
    }
}
