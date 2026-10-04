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
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        // The GitHub API rejects requests without a user agent.
        .user_agent(concat!("CashFlow/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| e.to_string())?;
    let response =
        client.get(&url).header("Accept", "application/vnd.github+json").send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }
    let text = response.text().await.map_err(|e| e.to_string())?;
    latest_version_from(&text).ok_or_else(|| "keine Versionsnummer gefunden".into())
}

/// Extracts the version from a GitHub release (JSON `tag_name`) or plain text.
pub fn latest_version_from(body: &str) -> Option<String> {
    let body = body.trim();
    let version = if body.starts_with('{') {
        let tag = body.split("\"tag_name\"").nth(1)?;
        let value = tag.split('"').nth(1)?;
        value.to_string()
    } else {
        body.lines().next()?.trim().to_string()
    };
    parse_version(&version).map(|_| version.trim_start_matches(['v', 'V']).to_string())
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

    #[test]
    fn reads_github_releases_and_plain_text() {
        let json = r#"{"url":"…","tag_name":"v2.1.0","name":"CashFlow 2.1.0","draft":false}"#;
        assert_eq!(latest_version_from(json).as_deref(), Some("2.1.0"));
        assert_eq!(latest_version_from("2.0.1\n").as_deref(), Some("2.0.1"));
        assert_eq!(latest_version_from("<html>").as_deref(), None);
    }
}
