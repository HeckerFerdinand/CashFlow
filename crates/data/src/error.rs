//! Errors of the data layer.
//!
//! Every variant's `Display` text is a German message that can be shown to
//! the user as-is; technical details stay available through `Debug` and the
//! log file.

use serde::Deserialize;

pub type Result<T, E = DataError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// No connection settings yet, or they are malformed.
    #[error("Die Datenbankverbindung ist nicht eingerichtet: {0}")]
    NotConfigured(String),

    /// DNS failure, refused connection, TLS problem, …
    #[error("Keine Verbindung zum Server. Bitte die Internetverbindung prüfen. ({0})")]
    Network(String),

    #[error("Der Server hat nicht rechtzeitig geantwortet. Bitte erneut versuchen.")]
    Timeout,

    /// The Supabase project is paused (free projects pause after a week
    /// without activity) or temporarily down.
    #[error(
        "Die Datenbank ist gerade nicht erreichbar (HTTP {status}). Kostenlose Supabase-Projekte \
         werden nach 7 Tagen ohne Nutzung pausiert – bitte im Supabase-Dashboard „Restore project“ \
         wählen und es in ein paar Minuten erneut versuchen."
    )]
    Unavailable { status: u16 },

    #[error("E-Mail-Adresse oder Passwort ist falsch.")]
    InvalidCredentials,

    #[error("Die Anmeldung ist abgelaufen. Bitte erneut anmelden.")]
    SessionExpired,

    #[error("Nicht angemeldet.")]
    NotSignedIn,

    /// Signed in, but row level security refused access (user not in `app_users`).
    #[error("Keine Berechtigung. Ist dieses Benutzerkonto in der Tabelle „app_users“ freigeschaltet?")]
    Forbidden,

    /// The tables/functions don't exist: `schema.sql` was not run yet.
    #[error(
        "Die Datenbank ist noch nicht eingerichtet (fehlende Tabelle oder Funktion). Bitte \
         supabase/schema.sql im SQL-Editor von Supabase ausführen. ({0})"
    )]
    SchemaMissing(String),

    /// Unique constraint violated, e.g. duplicate Personalnummer.
    #[error("{message}")]
    Conflict { constraint: Option<String>, message: String },

    /// A referenced record is missing or still referenced (foreign key).
    #[error("{0}")]
    Reference(String),

    #[error("Der Datensatz wurde nicht gefunden. Wurde er inzwischen gelöscht?")]
    NotFound,

    /// The server rejected the values (check constraint, wrong type, …).
    #[error("Ungültige Daten: {0}")]
    Invalid(String),

    #[error("{0}")]
    Auth(String),

    #[error("Serverfehler (HTTP {status}): {message}")]
    Server { status: u16, message: String },

    #[error("Unerwartete Antwort vom Server: {0}")]
    Decode(String),
}

impl DataError {
    /// True when retrying with a fresh access token may help.
    pub(crate) fn is_token_expired(&self) -> bool {
        matches!(self, DataError::SessionExpired)
    }

    pub(crate) fn from_reqwest(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            DataError::Timeout
        } else if error.is_decode() {
            DataError::Decode(error.to_string())
        } else {
            DataError::Network(root_cause(&error))
        }
    }
}

fn root_cause(error: &(dyn std::error::Error + 'static)) -> String {
    let mut current = error;
    while let Some(source) = current.source() {
        current = source;
    }
    current.to_string()
}

/// Error body returned by PostgREST.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct PostgrestError {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
}

/// Maps a failed PostgREST response to a [`DataError`].
pub(crate) fn map_rest_error(status: u16, body: &str) -> DataError {
    let parsed: PostgrestError = serde_json::from_str(body).unwrap_or_default();
    let code = parsed.code.as_deref().unwrap_or("");
    let message = parsed
        .message
        .clone()
        .unwrap_or_else(|| if body.trim().is_empty() { format!("HTTP {status}") } else { body.trim().to_string() });
    let detail = || {
        let mut text = message.clone();
        if let Some(details) = parsed.details.as_deref().filter(|d| !d.is_empty()) {
            text.push_str(" – ");
            text.push_str(details);
        }
        text
    };

    match code {
        // JWT expired / invalid
        "PGRST301" | "PGRST302" | "PGRST303" => return DataError::SessionExpired,
        // Table, view or function not found in the schema cache.
        "PGRST202" | "PGRST205" | "42P01" | "42883" => return DataError::SchemaMissing(message),
        "42501" => return DataError::Forbidden,
        "23505" => {
            return DataError::Conflict { constraint: constraint_name(&message), message: detail() };
        }
        "23503" => return DataError::Reference(detail()),
        "23502" | "23514" | "22P02" | "22003" | "22007" | "22008" | "22023" | "P0001" => {
            // P0001 = RAISE EXCEPTION from our own triggers/functions: message is meant for users.
            return DataError::Invalid(if code == "P0001" { message } else { detail() });
        }
        _ => {}
    }

    match status {
        401 => DataError::SessionExpired,
        403 => DataError::Forbidden,
        404 => DataError::SchemaMissing(message),
        406 => DataError::NotFound,
        409 => DataError::Conflict { constraint: constraint_name(&message), message: detail() },
        400 | 422 => DataError::Invalid(detail()),
        502..=504 | 520..=599 => DataError::Unavailable { status },
        _ => DataError::Server { status, message: detail() },
    }
}

/// Extracts the constraint from `duplicate key value violates unique constraint "x"`.
fn constraint_name(message: &str) -> Option<String> {
    let start = message.find('"')? + 1;
    let end = message[start..].find('"')? + start;
    Some(message[start..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_unique_violation_with_constraint_name() {
        let body = r#"{"code":"23505","details":"Key (personnel_number)=(7) already exists.","hint":null,"message":"duplicate key value violates unique constraint \"employees_employer_personnel_number_key\""}"#;
        match map_rest_error(409, body) {
            DataError::Conflict { constraint, .. } => {
                assert_eq!(constraint.as_deref(), Some("employees_employer_personnel_number_key"))
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn maps_common_failures() {
        assert!(matches!(
            map_rest_error(401, r#"{"code":"PGRST303","message":"JWT expired"}"#),
            DataError::SessionExpired
        ));
        assert!(matches!(
            map_rest_error(404, r#"{"code":"PGRST205","message":"Could not find the table"}"#),
            DataError::SchemaMissing(_)
        ));
        assert!(matches!(
            map_rest_error(403, r#"{"code":"42501","message":"permission denied"}"#),
            DataError::Forbidden
        ));
        assert!(matches!(map_rest_error(400, r#"{"code":"23514","message":"violates check"}"#), DataError::Invalid(_)));
        assert!(matches!(map_rest_error(503, ""), DataError::Unavailable { status: 503 }));
        assert!(matches!(map_rest_error(540, "Project paused"), DataError::Unavailable { status: 540 }));
        assert!(matches!(map_rest_error(500, "boom"), DataError::Server { status: 500, .. }));
    }

    #[test]
    fn raised_exceptions_keep_their_message() {
        let body = r#"{"code":"P0001","message":"Monat bereits abgerechnet","details":null}"#;
        match map_rest_error(400, body) {
            DataError::Invalid(text) => assert_eq!(text, "Monat bereits abgerechnet"),
            other => panic!("unexpected {other:?}"),
        }
    }
}
