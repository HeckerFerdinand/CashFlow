//! Data access for CashFlow: Supabase Auth + Data API (PostgREST).
//!
//! The app talks to Supabase only through [`Database`]. All tables are
//! protected by row level security, so the public API key alone gives no
//! access; every request carries the signed-in user's token.

pub mod auth;
pub mod database;
pub mod error;
pub mod repo;
pub mod rest;
pub mod settings;

pub use auth::{Session, User};
pub use database::{AppStatus, Database, EXPECTED_SCHEMA_VERSION, SessionListener};
pub use error::{DataError, Result};
pub use repo::employees::EmployeeRecordCounts;
pub use repo::records::RecordedMonths;
pub use settings::ConnectionSettings;
