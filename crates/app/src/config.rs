//! Per-computer settings, stored as TOML in the user's config folder
//! (Windows: `%APPDATA%\CashFlow\config\config.toml`,
//! macOS: `~/Library/Application Support/de.CashFlow.CashFlow/config.toml`).
//!
//! Shared settings (default rates, …) live in the database instead.

use cashflow_data::ConnectionSettings;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Release builds can be preconfigured for a Supabase project:
/// `CASHFLOW_SUPABASE_URL=… CASHFLOW_SUPABASE_KEY=… cargo build --release`.
const BUILT_IN_URL: Option<&str> = option_env!("CASHFLOW_SUPABASE_URL");
const BUILT_IN_KEY: Option<&str> = option_env!("CASHFLOW_SUPABASE_KEY");

/// Where the update check looks for the latest version number (plain text).
const DEFAULT_UPDATE_URL: &str =
    "https://gist.githubusercontent.com/HeckerFerdinand/f0e799558d487d925598c8cf560d8cf5/raw/version.txt";
const DEFAULT_DOWNLOAD_URL: &str = "https://github.com/HeckerFerdinand/CashFlow/releases";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    /// Supabase project URL (empty: use the built-in default, if any).
    pub supabase_url: String,
    /// Publishable key of the project.
    pub supabase_key: String,
    /// Base folder for generated PDFs (one sub-folder per employee).
    pub pdf_dir: Option<PathBuf>,
    pub open_pdf_after_create: bool,
    pub remember_login: bool,
    pub last_email: String,
    pub check_for_updates: bool,
    pub update_url: String,
    pub download_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            supabase_url: String::new(),
            supabase_key: String::new(),
            pdf_dir: None,
            open_pdf_after_create: true,
            remember_login: true,
            last_email: String::new(),
            check_for_updates: true,
            update_url: DEFAULT_UPDATE_URL.into(),
            download_url: DEFAULT_DOWNLOAD_URL.into(),
        }
    }
}

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("de", "CashFlow", "CashFlow")
}

pub fn config_file() -> Option<PathBuf> {
    project_dirs().map(|dirs| dirs.config_dir().join("config.toml"))
}

/// Folder for the log file.
pub fn data_dir() -> Option<PathBuf> {
    project_dirs().map(|dirs| dirs.data_local_dir().to_path_buf())
}

impl Config {
    pub fn load() -> Self {
        let Some(path) = config_file() else { return Self::default() };
        match std::fs::read_to_string(&path) {
            Ok(text) => toml::from_str(&text).unwrap_or_else(|error| {
                tracing::warn!(%error, ?path, "invalid config file, using defaults");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_file().ok_or("Kein Konfigurationsordner gefunden.")?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("Ordner {} nicht anlegbar: {e}", dir.display()))?;
        }
        let text = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, text).map_err(|e| format!("Einstellungen nicht speicherbar: {e}"))
    }

    /// Connection settings, falling back to the values built into the app.
    pub fn connection(&self) -> ConnectionSettings {
        let url =
            if self.supabase_url.trim().is_empty() { BUILT_IN_URL.unwrap_or_default() } else { &self.supabase_url };
        let key =
            if self.supabase_key.trim().is_empty() { BUILT_IN_KEY.unwrap_or_default() } else { &self.supabase_key };
        ConnectionSettings::new(url.trim(), key.trim())
    }

    pub fn has_connection(&self) -> bool {
        self.connection().validated_base_url().is_ok()
    }

    /// PDF folder: the configured one or `Dokumente/CashFlow`.
    pub fn pdf_dir(&self) -> PathBuf {
        if let Some(dir) = &self.pdf_dir {
            return dir.clone();
        }
        UserDirs::new()
            .and_then(|dirs| dirs.document_dir().map(|d| d.to_path_buf()))
            .or_else(|| UserDirs::new().map(|dirs| dirs.home_dir().to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("CashFlow")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_roundtrip_and_partial_files() {
        let config = Config {
            supabase_url: "https://x.supabase.co".into(),
            pdf_dir: Some("/tmp/pdf".into()),
            ..Config::default()
        };
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(toml::from_str::<Config>(&text).unwrap(), config);
        // Missing keys fall back to defaults (older config files keep working).
        let partial: Config = toml::from_str("last_email = \"a@b.de\"").unwrap();
        assert_eq!(partial.last_email, "a@b.de");
        assert!(partial.open_pdf_after_create);
    }
}
