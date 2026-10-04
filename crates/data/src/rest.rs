//! Thin, typed wrapper around Supabase's Data API (PostgREST).

use crate::error::{DataError, Result, map_rest_error};
use reqwest::{Client, Method, RequestBuilder};
use serde::{Serialize, de::DeserializeOwned};

/// Filter, ordering and column selection of a request, encoded as PostgREST
/// query parameters (`?select=*&employee_id=eq.…&order=month.asc`).
#[derive(Debug, Clone, Default)]
pub struct Query {
    params: Vec<(String, String)>,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    /// Columns (and embedded resources) to return, e.g. `*,employer:employers(*)`.
    pub fn select(mut self, columns: &str) -> Self {
        self.params.push(("select".into(), columns.into()));
        self
    }

    pub fn eq(self, column: &str, value: impl ToString) -> Self {
        self.filter(column, "eq", value)
    }

    pub fn gte(self, column: &str, value: impl ToString) -> Self {
        self.filter(column, "gte", value)
    }

    pub fn lte(self, column: &str, value: impl ToString) -> Self {
        self.filter(column, "lte", value)
    }

    pub fn is_null(self, column: &str) -> Self {
        self.filter(column, "is", "null")
    }

    /// `column=in.(a,b,c)`; values are quoted so commas inside values are safe.
    pub fn in_list<I, T>(mut self, column: &str, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        let list = values
            .into_iter()
            .map(|v| format!("\"{}\"", v.to_string().replace('\\', "\\\\").replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",");
        self.params.push((column.into(), format!("in.({list})")));
        self
    }

    /// `order=last_name.asc,first_name.asc`
    pub fn order(mut self, spec: &str) -> Self {
        self.params.push(("order".into(), spec.into()));
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.params.push(("limit".into(), limit.to_string()));
        self
    }

    /// Upsert target: `on_conflict=employee_id,year,month`.
    pub fn on_conflict(mut self, columns: &str) -> Self {
        self.params.push(("on_conflict".into(), columns.into()));
        self
    }

    fn filter(mut self, column: &str, op: &str, value: impl ToString) -> Self {
        self.params.push((column.into(), format!("{op}.{}", value.to_string())));
        self
    }

    pub(crate) fn has_filter(&self) -> bool {
        self.params.iter().any(|(key, _)| !matches!(key.as_str(), "select" | "order" | "limit" | "on_conflict"))
    }
}

/// Client for `/rest/v1`. Every call needs the signed-in user's access token.
#[derive(Clone)]
pub struct RestClient {
    http: Client,
    base: String,
    api_key: String,
}

impl RestClient {
    pub(crate) fn new(http: Client, base_url: &str, api_key: &str) -> Self {
        Self { http, base: base_url.to_string(), api_key: api_key.to_string() }
    }

    fn request(&self, method: Method, path: &str, token: &str, query: &Query) -> RequestBuilder {
        self.http
            .request(method, format!("{}/{path}", self.base))
            .header("apikey", &self.api_key)
            .bearer_auth(token)
            .query(&query.params)
    }

    /// `GET /table?…` → all matching rows.
    pub async fn select<T: DeserializeOwned>(&self, token: &str, table: &str, query: &Query) -> Result<Vec<T>> {
        let request = self.request(Method::GET, table, token, query);
        send_json(request).await
    }

    /// `GET /table?…` expecting at most one row.
    pub async fn select_optional<T: DeserializeOwned>(
        &self,
        token: &str,
        table: &str,
        query: &Query,
    ) -> Result<Option<T>> {
        let mut rows: Vec<T> = self.select(token, table, &query.clone().limit(2)).await?;
        match rows.len() {
            0 => Ok(None),
            1 => Ok(rows.pop()),
            _ => Err(DataError::Decode(format!("mehr als ein Datensatz in {table} gefunden"))),
        }
    }

    /// `POST /table` with one row; returns the stored row (with generated columns).
    pub async fn insert<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        token: &str,
        table: &str,
        row: &B,
    ) -> Result<T> {
        let request =
            self.request(Method::POST, table, token, &Query::new()).header("Prefer", "return=representation").json(row);
        single(send_json(request).await?)
    }

    /// Insert or update on the unique key `on_conflict` (`POST` + merge-duplicates).
    pub async fn upsert<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        token: &str,
        table: &str,
        on_conflict: &str,
        row: &B,
    ) -> Result<T> {
        let request = self
            .request(Method::POST, table, token, &Query::new().on_conflict(on_conflict))
            .header("Prefer", "resolution=merge-duplicates,return=representation")
            .json(row);
        single(send_json(request).await?)
    }

    /// `PATCH /table?filters` → updated rows. Fails with `NotFound` if nothing matched.
    pub async fn update<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        token: &str,
        table: &str,
        filter: &Query,
        changes: &B,
    ) -> Result<Vec<T>> {
        guard_filter(filter)?;
        let request =
            self.request(Method::PATCH, table, token, filter).header("Prefer", "return=representation").json(changes);
        let rows: Vec<T> = send_json(request).await?;
        if rows.is_empty() { Err(DataError::NotFound) } else { Ok(rows) }
    }

    /// `DELETE /table?filters` → number of deleted rows.
    pub async fn delete(&self, token: &str, table: &str, filter: &Query) -> Result<usize> {
        guard_filter(filter)?;
        let request = self
            .request(Method::DELETE, table, token, &filter.clone().select("*"))
            .header("Prefer", "return=representation");
        let rows: Vec<serde_json::Value> = send_json(request).await?;
        Ok(rows.len())
    }

    /// `POST /rpc/function` with named arguments.
    pub async fn rpc<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        token: &str,
        function: &str,
        args: &B,
    ) -> Result<T> {
        let request = self.request(Method::POST, &format!("rpc/{function}"), token, &Query::new()).json(args);
        send_json(request).await
    }
}

/// Refuses unfiltered PATCH/DELETE: a missing filter would hit every row.
fn guard_filter(filter: &Query) -> Result<()> {
    if filter.has_filter() { Ok(()) } else { Err(DataError::Invalid("interner Fehler: Änderung ohne Filter".into())) }
}

fn single<T>(mut rows: Vec<T>) -> Result<T> {
    match rows.len() {
        1 => Ok(rows.pop().expect("one row")),
        0 => Err(DataError::NotFound),
        n => Err(DataError::Decode(format!("{n} Datensätze statt einem erhalten"))),
    }
}

pub(crate) async fn send_json<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send().await.map_err(DataError::from_reqwest)?;
    let status = response.status();
    let body = response.text().await.map_err(DataError::from_reqwest)?;
    if !status.is_success() {
        let error = map_rest_error(status.as_u16(), &body);
        tracing::warn!(status = status.as_u16(), body = %body, "request failed: {error}");
        return Err(error);
    }
    let body = if body.trim().is_empty() { "null" } else { body.as_str() };
    serde_json::from_str(body).map_err(|error| {
        tracing::error!(%error, body, "could not decode response");
        DataError::Decode(error.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_postgrest_parameters() {
        let query = Query::new()
            .select("*")
            .eq("employee_id", "abc")
            .gte("year", 2025)
            .in_list("month", [1, 2])
            .order("month.asc");
        assert_eq!(
            query.params,
            vec![
                ("select".to_string(), "*".to_string()),
                ("employee_id".to_string(), "eq.abc".to_string()),
                ("year".to_string(), "gte.2025".to_string()),
                ("month".to_string(), "in.(\"1\",\"2\")".to_string()),
                ("order".to_string(), "month.asc".to_string()),
            ]
        );
        assert!(query.has_filter());
        assert!(!Query::new().select("*").order("x").has_filter());
    }
}
