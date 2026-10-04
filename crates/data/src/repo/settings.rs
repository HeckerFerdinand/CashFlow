//! `public.settings`: the single row of shared defaults.

use crate::database::Database;
use crate::error::{DataError, Result};
use crate::rest::Query;
use cashflow_core::{Defaults, Rates};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

const TABLE: &str = "settings";

#[derive(Deserialize, Serialize)]
struct Row {
    default_rate_health: Decimal,
    default_rate_pension: Decimal,
    default_rate_u1: Decimal,
    default_rate_u2: Decimal,
    default_rate_insolvency: Decimal,
    default_rate_flat_tax: Decimal,
    default_health_insurer: String,
}

impl From<Row> for Defaults {
    fn from(row: Row) -> Self {
        Defaults {
            rates: Rates {
                health: row.default_rate_health,
                pension: row.default_rate_pension,
                u1: row.default_rate_u1,
                u2: row.default_rate_u2,
                insolvency: row.default_rate_insolvency,
                flat_tax: row.default_rate_flat_tax,
            },
            health_insurer: row.default_health_insurer,
        }
    }
}

impl From<&Defaults> for Row {
    fn from(defaults: &Defaults) -> Self {
        Row {
            default_rate_health: defaults.rates.health,
            default_rate_pension: defaults.rates.pension,
            default_rate_u1: defaults.rates.u1,
            default_rate_u2: defaults.rates.u2,
            default_rate_insolvency: defaults.rates.insolvency,
            default_rate_flat_tax: defaults.rates.flat_tax,
            default_health_insurer: defaults.health_insurer.clone(),
        }
    }
}

const COLUMNS: &str = "default_rate_health,default_rate_pension,default_rate_u1,default_rate_u2,\
default_rate_insolvency,default_rate_flat_tax,default_health_insurer";

impl Database {
    pub async fn defaults(&self) -> Result<Defaults> {
        let row: Option<Row> = self.select_optional(TABLE, &Query::new().select(COLUMNS).eq("id", 1)).await?;
        row.map(Defaults::from).ok_or_else(|| DataError::SchemaMissing("Die Zeile in „settings“ fehlt.".into()))
    }

    pub async fn save_defaults(&self, defaults: &Defaults) -> Result<Defaults> {
        let filter = Query::new().eq("id", 1).select(COLUMNS);
        let mut rows: Vec<Row> = self.update(TABLE, &filter, &Row::from(defaults)).await?;
        Ok(rows.remove(0).into())
    }
}
