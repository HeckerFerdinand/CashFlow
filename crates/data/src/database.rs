//! The entry point of the data layer: one [`Database`] per connection,
//! holding the signed-in session and refreshing it transparently.

use crate::auth::{AuthClient, Session, User};
use crate::error::{DataError, Result};
use crate::rest::{Query, RestClient};
use crate::settings::ConnectionSettings;
use chrono::Duration;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Version of `supabase/schema.sql` this build expects (see `app_status()`).
pub const EXPECTED_SCHEMA_VERSION: i32 = 1;

/// Called whenever the session changes (sign-in, token refresh, sign-out),
/// e.g. to keep the stored refresh token of "angemeldet bleiben" current.
pub type SessionListener = Arc<dyn Fn(Option<&Session>) + Send + Sync>;

/// Result of the `app_status()` check right after signing in.
#[derive(Debug, Clone, Deserialize)]
pub struct AppStatus {
    pub schema_version: i32,
    pub is_app_user: bool,
}

pub struct Database {
    pub(crate) rest: RestClient,
    auth: AuthClient,
    session: Mutex<Option<Session>>,
    listener: Option<SessionListener>,
}

impl Database {
    /// Creates a client for the given project. Does not contact the server yet.
    pub fn new(settings: &ConnectionSettings) -> Result<Self> {
        let base = settings.validated_base_url()?;
        let http = http_client()?;
        Ok(Self {
            rest: RestClient::new(http.clone(), &format!("{base}/rest/v1"), settings.api_key()),
            auth: AuthClient::new(http, &format!("{base}/auth/v1"), settings.api_key()),
            session: Mutex::new(None),
            listener: None,
        })
    }

    /// Test support: talk to a plain PostgREST server (no `/rest/v1` prefix,
    /// no Auth server) as started by `scripts/dev-db.sh`.
    #[doc(hidden)]
    pub fn for_local_postgrest(rest_url: &str) -> Result<Self> {
        let http = http_client()?;
        let base = rest_url.trim_end_matches('/');
        Ok(Self {
            rest: RestClient::new(http.clone(), base, "local"),
            auth: AuthClient::new(http, &format!("{base}/auth-not-available"), "local"),
            session: Mutex::new(None),
            listener: None,
        })
    }

    pub fn with_session_listener(mut self, listener: SessionListener) -> Self {
        self.listener = Some(listener);
        self
    }

    /// Signs in with e-mail and password and checks that the database is set up.
    pub async fn sign_in(&self, email: &str, password: &str) -> Result<User> {
        let session = self.auth.sign_in(email, password).await?;
        self.start(session).await
    }

    /// Restores a session from a stored refresh token ("angemeldet bleiben").
    pub async fn resume(&self, refresh_token: &str) -> Result<User> {
        let session = self.auth.refresh(refresh_token).await?;
        self.start(session).await
    }

    async fn start(&self, session: Session) -> Result<User> {
        let user = session.user.clone();
        self.set_session(Some(session)).await;
        match self.check_status().await {
            Ok(()) => Ok(user),
            Err(error) => {
                self.sign_out().await;
                Err(error)
            }
        }
    }

    /// Verifies schema version and that the user is unlocked in `app_users`.
    async fn check_status(&self) -> Result<()> {
        let status = self.app_status().await?;
        if !status.is_app_user {
            return Err(DataError::Forbidden);
        }
        if status.schema_version < EXPECTED_SCHEMA_VERSION {
            return Err(DataError::SchemaMissing(format!(
                "Datenbank-Schema Version {} gefunden, benötigt wird Version {EXPECTED_SCHEMA_VERSION}",
                status.schema_version
            )));
        }
        Ok(())
    }

    pub async fn app_status(&self) -> Result<AppStatus> {
        let args = serde_json::json!({});
        let args = &args;
        self.with_token(|token| async move { self.rest.rpc(&token, "app_status", args).await }).await
    }

    pub async fn sign_out(&self) {
        let session = self.session.lock().await.take();
        if let Some(session) = session {
            self.auth.sign_out(&session.access_token).await;
        }
        self.notify(None);
    }

    pub async fn change_password(&self, new_password: &str) -> Result<()> {
        if new_password.chars().count() < 8 {
            return Err(DataError::Auth("Das Passwort muss mindestens 8 Zeichen lang sein.".into()));
        }
        let token = self.access_token().await?;
        self.auth.change_password(&token, new_password).await
    }

    pub async fn current_user(&self) -> Option<User> {
        self.session.lock().await.as_ref().map(|session| session.user.clone())
    }

    /// A valid access token, refreshed if it expires within the next minute.
    pub(crate) async fn access_token(&self) -> Result<String> {
        let mut guard = self.session.lock().await;
        let session = guard.as_ref().ok_or(DataError::NotSignedIn)?;
        if !session.expires_within(Duration::seconds(60)) {
            return Ok(session.access_token.clone());
        }
        let refreshed = self.auth.refresh(&session.refresh_token).await;
        match refreshed {
            Ok(new_session) => {
                let token = new_session.access_token.clone();
                *guard = Some(new_session);
                self.notify(guard.as_ref());
                Ok(token)
            }
            Err(error) => {
                if matches!(error, DataError::SessionExpired) {
                    *guard = None;
                    self.notify(None);
                }
                Err(error)
            }
        }
    }

    /// Forces a refresh after the server rejected the token.
    async fn force_refresh(&self) -> Result<()> {
        let mut guard = self.session.lock().await;
        if let Some(session) = guard.as_mut() {
            // Mark as expired; the next access_token() call refreshes.
            session.expires_at = chrono::Utc::now() - Duration::seconds(1);
        }
        drop(guard);
        self.access_token().await.map(|_| ())
    }

    /// Runs a request with the current access token and retries once with a
    /// refreshed token if the server says the token expired.
    pub(crate) async fn with_token<T, F, Fut>(&self, op: F) -> Result<T>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let token = self.access_token().await?;
        match op(token).await {
            Err(error) if error.is_token_expired() => {
                self.force_refresh().await?;
                let token = self.access_token().await?;
                op(token).await
            }
            other => other,
        }
    }

    // -- Request helpers used by the repositories -------------------------------

    pub(crate) async fn select<T: DeserializeOwned>(&self, table: &str, query: &Query) -> Result<Vec<T>> {
        self.with_token(|token| async move { self.rest.select(&token, table, query).await }).await
    }

    pub(crate) async fn select_optional<T: DeserializeOwned>(&self, table: &str, query: &Query) -> Result<Option<T>> {
        self.with_token(|token| async move { self.rest.select_optional(&token, table, query).await }).await
    }

    pub(crate) async fn insert<B, T>(&self, table: &str, body: &B) -> Result<T>
    where
        B: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        self.with_token(|token| async move { self.rest.insert(&token, table, body).await }).await
    }

    pub(crate) async fn upsert<B, T>(&self, table: &str, on_conflict: &str, body: &B) -> Result<T>
    where
        B: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        self.with_token(|token| async move { self.rest.upsert(&token, table, on_conflict, body).await }).await
    }

    pub(crate) async fn update<B, T>(&self, table: &str, filter: &Query, body: &B) -> Result<Vec<T>>
    where
        B: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        self.with_token(|token| async move { self.rest.update(&token, table, filter, body).await }).await
    }

    pub(crate) async fn delete(&self, table: &str, filter: &Query) -> Result<usize> {
        self.with_token(|token| async move { self.rest.delete(&token, table, filter).await }).await
    }

    /// Test support: use a pre-made session (e.g. a locally signed JWT)
    /// instead of signing in through Supabase Auth.
    #[doc(hidden)]
    pub async fn set_session_for_tests(&self, session: Session) {
        self.set_session(Some(session)).await;
    }

    async fn set_session(&self, session: Option<Session>) {
        let mut guard = self.session.lock().await;
        *guard = session;
        self.notify(guard.as_ref());
    }

    fn notify(&self, session: Option<&Session>) {
        if let Some(listener) = &self.listener {
            listener(session);
        }
    }
}

fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent(concat!("CashFlow/", env!("CARGO_PKG_VERSION")))
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|error| DataError::NotConfigured(error.to_string()))
}
