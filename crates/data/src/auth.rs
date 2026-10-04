//! Supabase Auth (GoTrue): sign in with e-mail + password, refresh, sign out.

use crate::error::{DataError, Result};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The signed-in user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    #[serde(default)]
    pub email: Option<String>,
}

/// Tokens of a signed-in user. The access token is valid for about an hour;
/// the refresh token (single use, rotated on every refresh) gets a new one.
#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub user: User,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never log tokens.
        f.debug_struct("Session").field("user", &self.user).field("expires_at", &self.expires_at).finish()
    }
}

impl Session {
    /// True if the access token expires within `margin`.
    pub fn expires_within(&self, margin: Duration) -> bool {
        self.expires_at - margin <= Utc::now()
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    #[serde(default)]
    expires_in: Option<i64>,
    #[serde(default)]
    expires_at: Option<i64>,
    user: User,
}

impl TokenResponse {
    fn into_session(self) -> Session {
        let expires_at = self
            .expires_at
            .and_then(|ts| DateTime::from_timestamp(ts, 0))
            .unwrap_or_else(|| Utc::now() + Duration::seconds(self.expires_in.unwrap_or(3600)));
        Session { access_token: self.access_token, refresh_token: self.refresh_token, expires_at, user: self.user }
    }
}

/// Error body of the Auth API (both the current and the legacy shape).
#[derive(Debug, Default, Deserialize)]
struct AuthErrorBody {
    #[serde(default)]
    error_code: Option<String>,
    #[serde(default)]
    msg: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

fn map_auth_error(status: u16, body: &str) -> DataError {
    let parsed: AuthErrorBody = serde_json::from_str(body).unwrap_or_default();
    let code = parsed.error_code.as_deref().or(parsed.error.as_deref()).unwrap_or("");
    let text = parsed.msg.or(parsed.message).or(parsed.error_description).unwrap_or_else(|| body.trim().to_string());
    match code {
        "invalid_credentials" | "invalid_grant" if text.to_lowercase().contains("refresh") => DataError::SessionExpired,
        "invalid_credentials" | "invalid_grant" => DataError::InvalidCredentials,
        "refresh_token_not_found"
        | "refresh_token_already_used"
        | "session_not_found"
        | "session_expired"
        | "bad_jwt" => DataError::SessionExpired,
        "email_not_confirmed" => {
            DataError::Auth("Die E-Mail-Adresse ist noch nicht bestätigt (Supabase → Authentication → Users).".into())
        }
        "user_banned" => DataError::Auth("Dieses Benutzerkonto ist gesperrt.".into()),
        "same_password" => DataError::Auth("Das neue Passwort muss sich vom alten unterscheiden.".into()),
        "weak_password" => DataError::Auth(format!("Das Passwort ist zu schwach. {text}")),
        "over_request_rate_limit" | "over_email_send_rate_limit" => {
            DataError::Auth("Zu viele Versuche. Bitte einen Moment warten.".into())
        }
        _ => match status {
            401 | 403 => DataError::SessionExpired,
            429 => DataError::Auth("Zu viele Versuche. Bitte einen Moment warten.".into()),
            502..=504 | 520..=599 => DataError::Unavailable { status },
            404 => DataError::NotConfigured("Unter dieser Projekt-URL wurde kein Supabase-Projekt gefunden.".into()),
            _ => DataError::Auth(if text.is_empty() { format!("Anmeldefehler (HTTP {status})") } else { text }),
        },
    }
}

/// Client for `/auth/v1`.
#[derive(Clone)]
pub struct AuthClient {
    http: Client,
    base: String,
    api_key: String,
}

impl AuthClient {
    pub(crate) fn new(http: Client, base_url: &str, api_key: &str) -> Self {
        Self { http, base: base_url.to_string(), api_key: api_key.to_string() }
    }

    fn post(&self, path: &str) -> RequestBuilder {
        self.http.post(format!("{}/{path}", self.base)).header("apikey", &self.api_key)
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<Session> {
        let email = email.trim();
        if email.is_empty() || password.is_empty() {
            return Err(DataError::Auth("Bitte E-Mail-Adresse und Passwort eingeben.".into()));
        }
        let request = self
            .post("token")
            .query(&[("grant_type", "password")])
            .json(&serde_json::json!({ "email": email, "password": password }));
        Ok(send(request).await?.into_session())
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<Session> {
        let request = self
            .post("token")
            .query(&[("grant_type", "refresh_token")])
            .json(&serde_json::json!({ "refresh_token": refresh_token }));
        Ok(send(request).await?.into_session())
    }

    /// Revokes the session on the server. Errors are irrelevant for the caller
    /// (the local session is dropped anyway), so they are only logged.
    pub async fn sign_out(&self, access_token: &str) {
        let result = self.post("logout").bearer_auth(access_token).send().await;
        if let Err(error) = result {
            tracing::warn!(%error, "sign out request failed");
        }
    }

    pub async fn change_password(&self, access_token: &str, new_password: &str) -> Result<()> {
        let request = self
            .http
            .put(format!("{}/user", self.base))
            .header("apikey", &self.api_key)
            .bearer_auth(access_token)
            .json(&serde_json::json!({ "password": new_password }));
        let response = request.send().await.map_err(DataError::from_reqwest)?;
        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        let body = response.text().await.unwrap_or_default();
        Err(map_auth_error(status.as_u16(), &body))
    }
}

async fn send(request: RequestBuilder) -> Result<TokenResponse> {
    let response = request.send().await.map_err(DataError::from_reqwest)?;
    let status = response.status();
    let body = response.text().await.map_err(DataError::from_reqwest)?;
    if !status.is_success() {
        let error = map_auth_error(status.as_u16(), &body);
        tracing::warn!(status = status.as_u16(), "auth request failed: {error}");
        return Err(error);
    }
    serde_json::from_str(&body).map_err(|error| DataError::Decode(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_auth_errors() {
        let wrong = r#"{"code":400,"error_code":"invalid_credentials","msg":"Invalid login credentials"}"#;
        assert!(matches!(map_auth_error(400, wrong), DataError::InvalidCredentials));
        let legacy = r#"{"error":"invalid_grant","error_description":"Invalid login credentials"}"#;
        assert!(matches!(map_auth_error(400, legacy), DataError::InvalidCredentials));
        let used =
            r#"{"code":400,"error_code":"refresh_token_already_used","msg":"Invalid Refresh Token: Already Used"}"#;
        assert!(matches!(map_auth_error(400, used), DataError::SessionExpired));
        let legacy_refresh =
            r#"{"error":"invalid_grant","error_description":"Invalid Refresh Token: Refresh Token Not Found"}"#;
        assert!(matches!(map_auth_error(400, legacy_refresh), DataError::SessionExpired));
        assert!(matches!(map_auth_error(503, ""), DataError::Unavailable { status: 503 }));
        assert!(matches!(map_auth_error(429, "{}"), DataError::Auth(_)));
    }

    #[test]
    fn token_response_computes_expiry() {
        let json = r#"{"access_token":"a","token_type":"bearer","expires_in":3600,"refresh_token":"r","user":{"id":"8d0fd2b3-9ca5-4b6e-9f0e-1c3a7f0c2b11","email":"x@y.de"}}"#;
        let session = serde_json::from_str::<TokenResponse>(json).unwrap().into_session();
        assert!(!session.expires_within(Duration::minutes(5)));
        assert!(session.expires_within(Duration::minutes(61)));
        assert_eq!(session.user.email.as_deref(), Some("x@y.de"));
        assert!(!format!("{session:?}").contains("\"a\""));
    }
}
