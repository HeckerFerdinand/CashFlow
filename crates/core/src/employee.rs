//! Employees (Arbeitnehmer) and their master data.

use crate::employer::{Address, join_non_empty};
use crate::money;
use crate::payroll::{Contribution, Rates};
use crate::validate::{self, ValidationErrors, Validator};
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    Male,
    Female,
    Diverse,
    Unspecified,
}

impl Gender {
    pub const ALL: [Gender; 4] = [Gender::Male, Gender::Female, Gender::Diverse, Gender::Unspecified];

    /// Code stored in the database (as used in DEÜV reports).
    pub fn code(self) -> &'static str {
        match self {
            Gender::Male => "m",
            Gender::Female => "w",
            Gender::Diverse => "d",
            Gender::Unspecified => "x",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Gender::Male => "männlich",
            Gender::Female => "weiblich",
            Gender::Diverse => "divers",
            Gender::Unspecified => "unbestimmt",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|g| g.code() == code.trim())
    }
}

/// Kennzeichen Übergangsbereich (formerly "Gleitzone").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionZone {
    /// 0 – kein Entgelt im Übergangsbereich
    No,
    /// 1 – Entgelt im Übergangsbereich
    Yes,
    /// 2 – Entgelte sowohl im als auch oberhalb des Übergangsbereichs
    Partly,
}

impl TransitionZone {
    pub const ALL: [TransitionZone; 3] = [TransitionZone::No, TransitionZone::Yes, TransitionZone::Partly];

    pub fn code(self) -> i16 {
        match self {
            TransitionZone::No => 0,
            TransitionZone::Yes => 1,
            TransitionZone::Partly => 2,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            TransitionZone::No => "0 – nein",
            TransitionZone::Yes => "1 – im Übergangsbereich",
            TransitionZone::Partly => "2 – im und über dem Übergangsbereich",
        }
    }

    pub fn from_code(code: i16) -> Option<Self> {
        Self::ALL.into_iter().find(|z| z.code() == code)
    }
}

/// Validated master data of an employee.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmployeeData {
    pub employer_id: Uuid,
    /// Personalnummer: unique, shown in lists and used for PDF folder/file names.
    pub personnel_number: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_name: String,
    pub address: Address,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub nationality: String,
    /// SV-Nummer, normalized without spaces (e.g. `12150388W042`).
    pub social_security_number: String,
    /// Steuer-ID (11 digits).
    pub tax_id: String,
    /// Tätigkeitsschlüssel (9 digits).
    pub activity_key: String,
    /// Beitragsgruppenschlüssel, "BGRS" (4 digits).
    pub contribution_group_key: String,
    /// Personengruppenschlüssel, "PGRS" (3 digits).
    pub person_group: String,
    pub transition_zone: Option<TransitionZone>,
    /// Berufsbezeichnung
    pub occupation: String,
    /// Krankenkasse
    pub health_insurer: String,
    /// Beschäftigungsbeginn (Eintritt)
    pub employment_start: Option<NaiveDate>,
    /// Austritt
    pub employment_end: Option<NaiveDate>,
    /// mtl. Vergütung (default for "Brutto-Bezüge")
    pub monthly_salary: Option<Decimal>,
    /// Stundensatz for Regiestunden (suggests "Sonstige Brutto-Bezüge")
    pub hourly_rate: Option<Decimal>,
    pub rates: Rates,
}

impl EmployeeData {
    /// "Anna Weber"
    pub fn full_name(&self) -> String {
        join_non_empty(&[&self.first_name, &self.last_name], " ")
    }

    /// "Weber, Anna" (for sorted lists)
    pub fn sort_name(&self) -> String {
        join_non_empty(&[&self.last_name, &self.first_name], ", ")
    }

    /// Whether the employee was employed at any time during `year`.
    pub fn is_employed_in_year(&self, year: i32) -> bool {
        let starts_after = self.employment_start.is_some_and(|start| start.year() > year);
        let ended_before = self.employment_end.is_some_and(|end| end.year() < year);
        !starts_after && !ended_before
    }

    /// Whether the employee has left before `date`.
    pub fn has_left_before(&self, date: NaiveDate) -> bool {
        self.employment_end.is_some_and(|end| end < date)
    }

    /// Name of the employee's PDF folder: "<Personalnummer> <Nachname>".
    pub fn document_folder_name(&self) -> String {
        crate::files::sanitize_file_name(&format!("{} {}", self.personnel_number, self.last_name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub id: Uuid,
    pub data: EmployeeData,
}

impl Employee {
    /// "1001 · Weber, Anna"
    pub fn list_label(&self) -> String {
        format!("{} · {}", self.data.personnel_number, self.data.sort_name())
    }
}

/// Raw form input for an employee. Field keys equal the database columns.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmployeeDraft {
    pub employer_id: String,
    pub personnel_number: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_name: String,
    pub street: String,
    pub house_number: String,
    pub postal_code: String,
    pub city: String,
    pub birth_date: String,
    /// Gender code ("m", "w", "d", "x") or empty.
    pub gender: String,
    pub nationality: String,
    pub social_security_number: String,
    pub tax_id: String,
    pub activity_key: String,
    pub contribution_group_key: String,
    pub person_group: String,
    /// "0", "1", "2" or empty.
    pub transition_zone: String,
    pub occupation: String,
    pub health_insurer: String,
    pub employment_start: String,
    pub employment_end: String,
    pub monthly_salary: String,
    pub hourly_rate: String,
    pub rate_health: String,
    pub rate_pension: String,
    pub rate_u1: String,
    pub rate_u2: String,
    pub rate_insolvency: String,
    pub rate_flat_tax: String,
}

impl EmployeeDraft {
    pub const KEYS: [&'static str; 30] = [
        "employer_id",
        "personnel_number",
        "first_name",
        "last_name",
        "birth_name",
        "street",
        "house_number",
        "postal_code",
        "city",
        "birth_date",
        "gender",
        "nationality",
        "social_security_number",
        "tax_id",
        "activity_key",
        "contribution_group_key",
        "person_group",
        "transition_zone",
        "occupation",
        "health_insurer",
        "employment_start",
        "employment_end",
        "monthly_salary",
        "hourly_rate",
        "rate_health",
        "rate_pension",
        "rate_u1",
        "rate_u2",
        "rate_insolvency",
        "rate_flat_tax",
    ];

    /// An empty form for a new employee, prefilled with the shared defaults.
    pub fn new_with_defaults(defaults: &crate::settings::Defaults) -> Self {
        let mut draft = Self { health_insurer: defaults.health_insurer.clone(), ..Self::default() };
        for contribution in Contribution::ALL {
            draft.set(contribution.key(), money::format_flexible(defaults.rates.get(contribution), 0, 4));
        }
        draft
    }

    pub fn from_data(data: &EmployeeData) -> Self {
        let amount = |value: Option<Decimal>| value.map(money::format_amount).unwrap_or_default();
        let rate = |value: Decimal| money::format_flexible(value, 0, 4);
        Self {
            employer_id: data.employer_id.to_string(),
            personnel_number: data.personnel_number.clone(),
            first_name: data.first_name.clone(),
            last_name: data.last_name.clone(),
            birth_name: data.birth_name.clone(),
            street: data.address.street.clone(),
            house_number: data.address.house_number.clone(),
            postal_code: data.address.postal_code.clone(),
            city: data.address.city.clone(),
            birth_date: validate::format_optional_date(data.birth_date),
            gender: data.gender.map(|g| g.code().to_string()).unwrap_or_default(),
            nationality: data.nationality.clone(),
            social_security_number: format_social_security_number(&data.social_security_number),
            tax_id: data.tax_id.clone(),
            activity_key: data.activity_key.clone(),
            contribution_group_key: data.contribution_group_key.clone(),
            person_group: data.person_group.clone(),
            transition_zone: data.transition_zone.map(|z| z.code().to_string()).unwrap_or_default(),
            occupation: data.occupation.clone(),
            health_insurer: data.health_insurer.clone(),
            employment_start: validate::format_optional_date(data.employment_start),
            employment_end: validate::format_optional_date(data.employment_end),
            monthly_salary: amount(data.monthly_salary),
            hourly_rate: amount(data.hourly_rate),
            rate_health: rate(data.rates.health),
            rate_pension: rate(data.rates.pension),
            rate_u1: rate(data.rates.u1),
            rate_u2: rate(data.rates.u2),
            rate_insolvency: rate(data.rates.insolvency),
            rate_flat_tax: rate(data.rates.flat_tax),
        }
    }

    fn slot(&mut self, key: &str) -> Option<&mut String> {
        Some(match key {
            "employer_id" => &mut self.employer_id,
            "personnel_number" => &mut self.personnel_number,
            "first_name" => &mut self.first_name,
            "last_name" => &mut self.last_name,
            "birth_name" => &mut self.birth_name,
            "street" => &mut self.street,
            "house_number" => &mut self.house_number,
            "postal_code" => &mut self.postal_code,
            "city" => &mut self.city,
            "birth_date" => &mut self.birth_date,
            "gender" => &mut self.gender,
            "nationality" => &mut self.nationality,
            "social_security_number" => &mut self.social_security_number,
            "tax_id" => &mut self.tax_id,
            "activity_key" => &mut self.activity_key,
            "contribution_group_key" => &mut self.contribution_group_key,
            "person_group" => &mut self.person_group,
            "transition_zone" => &mut self.transition_zone,
            "occupation" => &mut self.occupation,
            "health_insurer" => &mut self.health_insurer,
            "employment_start" => &mut self.employment_start,
            "employment_end" => &mut self.employment_end,
            "monthly_salary" => &mut self.monthly_salary,
            "hourly_rate" => &mut self.hourly_rate,
            "rate_health" => &mut self.rate_health,
            "rate_pension" => &mut self.rate_pension,
            "rate_u1" => &mut self.rate_u1,
            "rate_u2" => &mut self.rate_u2,
            "rate_insolvency" => &mut self.rate_insolvency,
            "rate_flat_tax" => &mut self.rate_flat_tax,
            _ => return None,
        })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match key {
            "employer_id" => Some(&self.employer_id),
            "personnel_number" => Some(&self.personnel_number),
            "first_name" => Some(&self.first_name),
            "last_name" => Some(&self.last_name),
            "birth_name" => Some(&self.birth_name),
            "street" => Some(&self.street),
            "house_number" => Some(&self.house_number),
            "postal_code" => Some(&self.postal_code),
            "city" => Some(&self.city),
            "birth_date" => Some(&self.birth_date),
            "gender" => Some(&self.gender),
            "nationality" => Some(&self.nationality),
            "social_security_number" => Some(&self.social_security_number),
            "tax_id" => Some(&self.tax_id),
            "activity_key" => Some(&self.activity_key),
            "contribution_group_key" => Some(&self.contribution_group_key),
            "person_group" => Some(&self.person_group),
            "transition_zone" => Some(&self.transition_zone),
            "occupation" => Some(&self.occupation),
            "health_insurer" => Some(&self.health_insurer),
            "employment_start" => Some(&self.employment_start),
            "employment_end" => Some(&self.employment_end),
            "monthly_salary" => Some(&self.monthly_salary),
            "hourly_rate" => Some(&self.hourly_rate),
            "rate_health" => Some(&self.rate_health),
            "rate_pension" => Some(&self.rate_pension),
            "rate_u1" => Some(&self.rate_u1),
            "rate_u2" => Some(&self.rate_u2),
            "rate_insolvency" => Some(&self.rate_insolvency),
            "rate_flat_tax" => Some(&self.rate_flat_tax),
            _ => None,
        }
        .map(String::as_str)
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) -> bool {
        match self.slot(key) {
            Some(slot) => {
                *slot = value.into();
                true
            }
            None => false,
        }
    }

    pub fn validate(&self) -> Result<EmployeeData, ValidationErrors> {
        let mut v = Validator::new();

        let employer_id = match Uuid::parse_str(self.employer_id.trim()) {
            Ok(id) => Some(id),
            Err(_) => {
                v.error("employer_id", "Bitte einen Arbeitgeber auswählen.");
                None
            }
        };
        let personnel_number = v.required_with("personnel_number", &self.personnel_number, validate::personnel_number);
        let first_name = v.required_text("first_name", &self.first_name, 100);
        let last_name = v.required_text("last_name", &self.last_name, 100);
        let birth_name = v.optional_text("birth_name", &self.birth_name, 100);
        let address = Address {
            street: v.optional_text("street", &self.street, 100),
            house_number: v.optional_text("house_number", &self.house_number, 20),
            postal_code: v.optional_with("postal_code", &self.postal_code, validate::postal_code),
            city: v.optional_text("city", &self.city, 100),
        };
        let birth_date = v.optional_date("birth_date", &self.birth_date);
        let gender = match self.gender.trim() {
            "" => None,
            code => {
                let gender = Gender::from_code(code);
                if gender.is_none() {
                    v.error("gender", "Ungültige Auswahl.");
                }
                gender
            }
        };
        let nationality = v.optional_text("nationality", &self.nationality, 60);
        let social_security_number =
            v.optional_with("social_security_number", &self.social_security_number, validate::social_security_number);
        let tax_id = v.optional_with("tax_id", &self.tax_id, validate::tax_id);
        let activity_key =
            v.optional_with("activity_key", &self.activity_key, validate::digits(9, "Der Tätigkeitsschlüssel"));
        let contribution_group_key = v.optional_with(
            "contribution_group_key",
            &self.contribution_group_key,
            validate::digits(4, "Der Beitragsgruppenschlüssel"),
        );
        let person_group =
            v.optional_with("person_group", &self.person_group, validate::digits(3, "Der Personengruppenschlüssel"));
        let transition_zone = match self.transition_zone.trim() {
            "" => None,
            code => {
                let zone = code.parse::<i16>().ok().and_then(TransitionZone::from_code);
                if zone.is_none() {
                    v.error("transition_zone", "Ungültige Auswahl.");
                }
                zone
            }
        };
        let occupation = v.optional_text("occupation", &self.occupation, 100);
        let health_insurer = v.optional_text("health_insurer", &self.health_insurer, 100);
        let employment_start = v.optional_date("employment_start", &self.employment_start);
        let employment_end = v.optional_date("employment_end", &self.employment_end);
        if let (Some(start), Some(end)) = (employment_start, employment_end)
            && end < start
        {
            v.error("employment_end", "Der Austritt liegt vor dem Beschäftigungsbeginn.");
        }
        let monthly_salary = v.optional_amount("monthly_salary", &self.monthly_salary);
        let hourly_rate = v.optional_amount("hourly_rate", &self.hourly_rate);

        let mut rates = Rates::default();
        for contribution in Contribution::ALL {
            let key = contribution.key();
            let input = self.get(key).unwrap_or_default();
            if let Some(value) = v.percent(key, input) {
                rates.set(contribution, value);
            }
        }

        let data = EmployeeData {
            employer_id: employer_id.unwrap_or_default(),
            personnel_number,
            first_name,
            last_name,
            birth_name,
            address,
            birth_date,
            gender,
            nationality,
            social_security_number,
            tax_id,
            activity_key,
            contribution_group_key,
            person_group,
            transition_zone,
            occupation,
            health_insurer,
            employment_start,
            employment_end,
            monthly_salary,
            hourly_rate,
            rates,
        };
        v.finish(data)
    }
}

/// `12150388W042` → `12 150388 W 042` (display form).
pub fn format_social_security_number(normalized: &str) -> String {
    let chars: Vec<char> = normalized.chars().collect();
    if chars.len() != 12 {
        return normalized.to_string();
    }
    let part = |range: std::ops::Range<usize>| chars[range].iter().collect::<String>();
    format!("{} {} {} {}", part(0..2), part(2..8), part(8..9), part(9..12))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Defaults;
    use rust_decimal_macros::dec;

    fn valid_draft() -> EmployeeDraft {
        EmployeeDraft {
            employer_id: "6d49f165-69e9-4271-b6e0-1f844580993c".into(),
            personnel_number: "1001".into(),
            first_name: "Anna".into(),
            last_name: "Weber".into(),
            postal_code: "50667".into(),
            birth_date: "15.03.1988".into(),
            gender: "w".into(),
            social_security_number: "12 150388 W 042".into(),
            tax_id: "86095742719".into(),
            transition_zone: "0".into(),
            employment_start: "01.04.2022".into(),
            monthly_salary: "538,00".into(),
            hourly_rate: "13.50".into(),
            rate_health: "13".into(),
            rate_pension: "15".into(),
            rate_u1: "0,8".into(),
            rate_u2: "0,22".into(),
            rate_insolvency: "0,15".into(),
            rate_flat_tax: "2".into(),
            ..Default::default()
        }
    }

    #[test]
    fn validates_a_complete_employee() {
        let data = valid_draft().validate().unwrap();
        assert_eq!(data.full_name(), "Anna Weber");
        assert_eq!(data.social_security_number, "12150388W042");
        assert_eq!(data.gender, Some(Gender::Female));
        assert_eq!(data.transition_zone, Some(TransitionZone::No));
        assert_eq!(data.monthly_salary, Some(dec!(538)));
        assert_eq!(data.rates.u2, dec!(0.22));
        assert_eq!(data.document_folder_name(), "1001 Weber");
    }

    #[test]
    fn roundtrips_through_the_form() {
        let data = valid_draft().validate().unwrap();
        let draft = EmployeeDraft::from_data(&data);
        assert_eq!(draft.social_security_number, "12 150388 W 042");
        assert_eq!(draft.rate_u1, "0,8");
        assert_eq!(draft.monthly_salary, "538,00");
        assert_eq!(draft.validate().unwrap(), data);
    }

    #[test]
    fn reports_every_invalid_field() {
        let draft = EmployeeDraft {
            employer_id: "".into(),
            personnel_number: "Mü/1".into(),
            first_name: " ".into(),
            last_name: "Weber".into(),
            social_security_number: "12150388W043".into(),
            employment_start: "01.04.2022".into(),
            employment_end: "31.03.2022".into(),
            rate_health: "abc".into(),
            rate_pension: "".into(),
            ..valid_draft()
        };
        let errors = draft.validate().unwrap_err();
        for key in [
            "employer_id",
            "personnel_number",
            "first_name",
            "social_security_number",
            "employment_end",
            "rate_health",
            "rate_pension",
        ] {
            assert!(errors.get(key).is_some(), "missing error for {key}");
        }
        assert!(errors.get("last_name").is_none());
    }

    #[test]
    fn keys_are_complete() {
        let mut draft = EmployeeDraft::default();
        for key in EmployeeDraft::KEYS {
            assert!(draft.set(key, key), "{key}");
            assert_eq!(draft.get(key), Some(key));
        }
    }

    #[test]
    fn defaults_prefill_rates() {
        let defaults = Defaults {
            rates: Rates {
                health: dec!(13),
                pension: dec!(15),
                u1: dec!(0.8),
                u2: dec!(0.22),
                insolvency: dec!(0.15),
                flat_tax: dec!(2),
            },
            health_insurer: "Minijob-Zentrale".into(),
        };
        let draft = EmployeeDraft::new_with_defaults(&defaults);
        assert_eq!(draft.rate_u2, "0,22");
        assert_eq!(draft.rate_health, "13");
        assert_eq!(draft.health_insurer, "Minijob-Zentrale");
    }

    #[test]
    fn employment_in_year() {
        let mut data = valid_draft().validate().unwrap();
        assert!(data.is_employed_in_year(2022));
        assert!(!data.is_employed_in_year(2021));
        data.employment_end = NaiveDate::from_ymd_opt(2024, 6, 30);
        assert!(data.is_employed_in_year(2024));
        assert!(!data.is_employed_in_year(2025));
    }
}
