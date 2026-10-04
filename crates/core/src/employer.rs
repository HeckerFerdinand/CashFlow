//! Employers (Arbeitgeber). One employer has many employees.

use crate::validate::{self, ValidationErrors, Validator};
use uuid::Uuid;

/// Postal address.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Address {
    pub street: String,
    pub house_number: String,
    pub postal_code: String,
    pub city: String,
}

impl Address {
    /// "Hauptstraße 12"
    pub fn street_line(&self) -> String {
        join_non_empty(&[&self.street, &self.house_number], " ")
    }

    /// "50667 Köln"
    pub fn city_line(&self) -> String {
        join_non_empty(&[&self.postal_code, &self.city], " ")
    }

    /// "Hauptstraße 12, 50667 Köln"
    pub fn single_line(&self) -> String {
        join_non_empty(&[&self.street_line(), &self.city_line()], ", ")
    }
}

pub(crate) fn join_non_empty(parts: &[&str], separator: &str) -> String {
    parts.iter().map(|p| p.trim()).filter(|p| !p.is_empty()).collect::<Vec<_>>().join(separator)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmployerData {
    pub name: String,
    /// "Vertretung": representative or second name line, printed after the name.
    pub representative: String,
    /// Betriebsnummer (8 digits)
    pub company_number: String,
    /// Steuernummer
    pub tax_number: String,
    pub address: Address,
}

impl EmployerData {
    /// Name plus representative, as printed on documents.
    pub fn full_name(&self) -> String {
        join_non_empty(&[&self.name, &self.representative], " ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employer {
    pub id: Uuid,
    pub data: EmployerData,
}

/// Raw form input for an employer. Field keys equal the database columns.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmployerDraft {
    pub name: String,
    pub representative: String,
    pub company_number: String,
    pub tax_number: String,
    pub street: String,
    pub house_number: String,
    pub postal_code: String,
    pub city: String,
}

impl EmployerDraft {
    pub const KEYS: [&'static str; 8] =
        ["name", "representative", "company_number", "tax_number", "street", "house_number", "postal_code", "city"];

    pub fn from_data(data: &EmployerData) -> Self {
        Self {
            name: data.name.clone(),
            representative: data.representative.clone(),
            company_number: data.company_number.clone(),
            tax_number: data.tax_number.clone(),
            street: data.address.street.clone(),
            house_number: data.address.house_number.clone(),
            postal_code: data.address.postal_code.clone(),
            city: data.address.city.clone(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        Some(match key {
            "name" => &self.name,
            "representative" => &self.representative,
            "company_number" => &self.company_number,
            "tax_number" => &self.tax_number,
            "street" => &self.street,
            "house_number" => &self.house_number,
            "postal_code" => &self.postal_code,
            "city" => &self.city,
            _ => return None,
        })
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) -> bool {
        let slot = match key {
            "name" => &mut self.name,
            "representative" => &mut self.representative,
            "company_number" => &mut self.company_number,
            "tax_number" => &mut self.tax_number,
            "street" => &mut self.street,
            "house_number" => &mut self.house_number,
            "postal_code" => &mut self.postal_code,
            "city" => &mut self.city,
            _ => return false,
        };
        *slot = value.into();
        true
    }

    pub fn validate(&self) -> Result<EmployerData, ValidationErrors> {
        let mut v = Validator::new();
        let data = EmployerData {
            name: v.required_text("name", &self.name, 120),
            representative: v.optional_text("representative", &self.representative, 120),
            company_number: v.optional_with(
                "company_number",
                &self.company_number,
                validate::digits(8, "Die Betriebsnummer"),
            ),
            tax_number: v.optional_text("tax_number", &self.tax_number, 30),
            address: Address {
                street: v.optional_text("street", &self.street, 100),
                house_number: v.optional_text("house_number", &self.house_number, 20),
                postal_code: v.optional_with("postal_code", &self.postal_code, validate::postal_code),
                city: v.optional_text("city", &self.city, 100),
            },
        };
        v.finish(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_normalizes() {
        let draft = EmployerDraft {
            name: "  Müller &  Söhne GmbH ".into(),
            company_number: "1234 5678".into(),
            postal_code: "50668".into(),
            street: "Industrieweg".into(),
            house_number: "5".into(),
            city: "Köln".into(),
            ..Default::default()
        };
        let data = draft.validate().unwrap();
        assert_eq!(data.name, "Müller & Söhne GmbH");
        assert_eq!(data.company_number, "12345678");
        assert_eq!(data.address.single_line(), "Industrieweg 5, 50668 Köln");
        assert_eq!(EmployerDraft::from_data(&data).validate().unwrap(), data);
    }

    #[test]
    fn reports_field_errors() {
        let draft = EmployerDraft { company_number: "123".into(), postal_code: "abc".into(), ..Default::default() };
        let errors = draft.validate().unwrap_err();
        assert_eq!(errors.get("name"), Some("Pflichtfeld."));
        assert!(errors.get("company_number").is_some());
        assert!(errors.get("postal_code").is_some());
    }

    #[test]
    fn keys_roundtrip() {
        let mut draft = EmployerDraft::default();
        for key in EmployerDraft::KEYS {
            assert!(draft.set(key, key));
            assert_eq!(draft.get(key), Some(key));
        }
        assert!(!draft.set("unknown", "x"));
    }
}
