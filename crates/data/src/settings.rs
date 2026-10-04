//! Connection settings for a Supabase project.

use crate::error::{DataError, Result};
use serde::{Deserialize, Serialize};

/// Where the Supabase project lives and which public API key to use.
///
/// The key is the project's *publishable* key (`sb_publishable_…`, or the
/// legacy `anon` key). It is safe to ship with the app: every table is
/// protected by row level security and requires a signed-in user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConnectionSettings {
    /// Project URL, e.g. `https://abcdefghijklmnop.supabase.co`.
    pub url: String,
    /// Publishable (or legacy anon) API key.
    pub api_key: String,
}

impl ConnectionSettings {
    pub fn new(url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self { url: url.into(), api_key: api_key.into() }
    }

    /// Checks the settings and returns the normalized project URL (no trailing slash).
    pub fn validated_base_url(&self) -> Result<String> {
        let mut url = self.url.trim().trim_end_matches('/');
        // Accept API addresses copied from the dashboard (".../rest/v1/").
        for suffix in ["/rest/v1", "/auth/v1"] {
            if let Some(base) = url.strip_suffix(suffix) {
                url = base.trim_end_matches('/');
            }
        }
        if url.is_empty() {
            return Err(DataError::NotConfigured("Projekt-URL fehlt.".into()));
        }
        let parsed = reqwest::Url::parse(url)
            .map_err(|_| DataError::NotConfigured(format!("„{url}“ ist keine gültige URL.")))?;
        let local = matches!(parsed.host_str(), Some("localhost" | "127.0.0.1"));
        if parsed.scheme() != "https" && !(local && parsed.scheme() == "http") {
            return Err(DataError::NotConfigured("Die Projekt-URL muss mit https:// beginnen.".into()));
        }
        if self.api_key.trim().is_empty() {
            return Err(DataError::NotConfigured("API-Schlüssel fehlt.".into()));
        }
        if self.api_key.trim().starts_with("sb_secret_") {
            return Err(DataError::NotConfigured(
                "Das ist ein geheimer Schlüssel (sb_secret_…). Bitte den „Publishable key“ verwenden – \
                 der geheime Schlüssel darf nie in die App."
                    .into(),
            ));
        }
        Ok(url.to_string())
    }

    pub(crate) fn api_key(&self) -> &str {
        self.api_key.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_and_validates() {
        let ok = ConnectionSettings::new(" https://abc.supabase.co/ ", "sb_publishable_x");
        assert_eq!(ok.validated_base_url().unwrap(), "https://abc.supabase.co");
        let rest = ConnectionSettings::new("https://abc.supabase.co/rest/v1/", "sb_publishable_x");
        assert_eq!(rest.validated_base_url().unwrap(), "https://abc.supabase.co");
        assert!(ConnectionSettings::new("http://127.0.0.1:54330", "k").validated_base_url().is_ok());
        assert!(ConnectionSettings::new("http://abc.supabase.co", "k").validated_base_url().is_err());
        assert!(ConnectionSettings::new("", "k").validated_base_url().is_err());
        assert!(ConnectionSettings::new("https://abc.supabase.co", " ").validated_base_url().is_err());
        assert!(ConnectionSettings::new("https://abc.supabase.co", "sb_secret_123").validated_base_url().is_err());
        assert!(ConnectionSettings::new("not a url", "k").validated_base_url().is_err());
    }
}
