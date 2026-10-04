//! "Angemeldet bleiben": the refresh token is kept in the operating system's
//! credential store (Windows Credential Manager, macOS Keychain), never in a file.

use cashflow_data::Session;
use std::sync::Mutex;

const SERVICE: &str = "CashFlow";

fn entry(account: &str) -> Option<keyring::Entry> {
    keyring::Entry::new(SERVICE, account).map_err(|error| tracing::warn!(%error, "credential store unavailable")).ok()
}

/// Stored refresh token for `account`, if any.
pub fn load_token(account: &str) -> Option<String> {
    entry(account)?.get_password().ok().filter(|token| !token.is_empty())
}

pub fn delete_token(account: &str) {
    if let Some(entry) = entry(account) {
        let _ = entry.delete_credential();
    }
}

/// Keeps the stored token in sync with the session (tokens rotate on refresh).
#[derive(Default)]
pub struct TokenStore {
    /// The account whose session should be remembered; `None` = don't remember.
    account: Mutex<Option<String>>,
}

impl TokenStore {
    pub fn remember(&self, account: Option<String>) {
        *self.account.lock().expect("token store lock") = account;
    }

    pub fn on_session_changed(&self, session: Option<&Session>) {
        let Some(account) = self.account.lock().expect("token store lock").clone() else { return };
        let Some(entry) = entry(&account) else { return };
        let result = match session {
            Some(session) => entry.set_password(&session.refresh_token),
            None => entry.delete_credential(),
        };
        if let Err(error) = result {
            tracing::debug!(%error, "credential store update");
        }
    }
}
