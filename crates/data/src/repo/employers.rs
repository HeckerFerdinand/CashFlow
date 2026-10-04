//! `public.employers`

use crate::database::Database;
use crate::error::{DataError, Result};
use crate::rest::Query;
use cashflow_core::{Address, Employer, EmployerData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TABLE: &str = "employers";
const COLUMNS: &str = "id,name,representative,company_number,tax_number,street,house_number,postal_code,city";

#[derive(Deserialize)]
struct Row {
    id: Uuid,
    name: String,
    representative: String,
    company_number: String,
    tax_number: String,
    street: String,
    house_number: String,
    postal_code: String,
    city: String,
}

impl From<Row> for Employer {
    fn from(row: Row) -> Self {
        Employer {
            id: row.id,
            data: EmployerData {
                name: row.name,
                representative: row.representative,
                company_number: row.company_number,
                tax_number: row.tax_number,
                address: Address {
                    street: row.street,
                    house_number: row.house_number,
                    postal_code: row.postal_code,
                    city: row.city,
                },
            },
        }
    }
}

#[derive(Serialize)]
struct Write<'a> {
    name: &'a str,
    representative: &'a str,
    company_number: &'a str,
    tax_number: &'a str,
    street: &'a str,
    house_number: &'a str,
    postal_code: &'a str,
    city: &'a str,
}

impl<'a> From<&'a EmployerData> for Write<'a> {
    fn from(data: &'a EmployerData) -> Self {
        Write {
            name: &data.name,
            representative: &data.representative,
            company_number: &data.company_number,
            tax_number: &data.tax_number,
            street: &data.address.street,
            house_number: &data.address.house_number,
            postal_code: &data.address.postal_code,
            city: &data.address.city,
        }
    }
}

fn friendly(error: DataError) -> DataError {
    match error {
        DataError::Conflict { constraint: Some(name), .. } if name == "employers_name_key" => DataError::Conflict {
            constraint: Some(name),
            message: "Ein Arbeitgeber mit diesem Namen existiert bereits.".into(),
        },
        DataError::Reference(_) => DataError::Reference(
            "Der Arbeitgeber kann nicht gelöscht werden, solange ihm Mitarbeiter zugeordnet sind.".into(),
        ),
        other => other,
    }
}

impl Database {
    /// All employers, sorted by name.
    pub async fn list_employers(&self) -> Result<Vec<Employer>> {
        let rows: Vec<Row> = self.select(TABLE, &Query::new().select(COLUMNS).order("name.asc")).await?;
        Ok(rows.into_iter().map(Employer::from).collect())
    }

    pub async fn create_employer(&self, data: &EmployerData) -> Result<Employer> {
        let row: Row = self.insert(TABLE, &Write::from(data)).await.map_err(friendly)?;
        Ok(row.into())
    }

    pub async fn update_employer(&self, id: Uuid, data: &EmployerData) -> Result<Employer> {
        let filter = Query::new().eq("id", id).select(COLUMNS);
        let mut rows: Vec<Row> = self.update(TABLE, &filter, &Write::from(data)).await.map_err(friendly)?;
        Ok(rows.remove(0).into())
    }

    pub async fn delete_employer(&self, id: Uuid) -> Result<()> {
        match self.delete(TABLE, &Query::new().eq("id", id)).await.map_err(friendly)? {
            0 => Err(DataError::NotFound),
            _ => Ok(()),
        }
    }
}
