//! Validation of user input, field by field, with German error messages.
//!
//! Forms collect raw strings; a [`Validator`] turns them into typed values and
//! records one message per invalid field, so the UI can show every problem
//! next to the field it belongs to.

use crate::money::{self, NumberError};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::fmt;

/// One problem with one input field. `field` is the form's field key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}

/// All problems found in a form.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationErrors {
    errors: Vec<FieldError>,
}

impl ValidationErrors {
    pub fn add(&mut self, field: &'static str, message: impl Into<String>) {
        // Keep only the first message per field.
        if self.get(field).is_none() {
            self.errors.push(FieldError { field, message: message.into() });
        }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        self.errors.iter().find(|e| e.field == field).map(|e| e.message.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = &FieldError> {
        self.errors.iter()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.errors.len() {
            0 => write!(f, "Keine Fehler."),
            1 => write!(f, "{}", self.errors[0].message),
            n => write!(f, "{n} Eingaben sind ungültig. Bitte die markierten Felder prüfen."),
        }
    }
}

impl std::error::Error for ValidationErrors {}

/// Collects typed values and errors while reading a form.
#[derive(Debug, Default)]
pub struct Validator {
    errors: ValidationErrors,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn error(&mut self, field: &'static str, message: impl Into<String>) {
        self.errors.add(field, message);
    }

    pub fn has_error(&self, field: &str) -> bool {
        self.errors.get(field).is_some()
    }

    /// Returns `value` if no error was recorded, otherwise all errors.
    pub fn finish<T>(self, value: T) -> Result<T, ValidationErrors> {
        if self.errors.is_empty() { Ok(value) } else { Err(self.errors) }
    }

    /// Trimmed text that must not be empty.
    pub fn required_text(&mut self, field: &'static str, input: &str, max_chars: usize) -> String {
        let text = clean_text(input);
        if text.is_empty() {
            self.error(field, "Pflichtfeld.");
        } else if text.chars().count() > max_chars {
            self.error(field, format!("Höchstens {max_chars} Zeichen."));
        }
        text
    }

    /// Trimmed optional text.
    pub fn optional_text(&mut self, field: &'static str, input: &str, max_chars: usize) -> String {
        let text = clean_text(input);
        if text.chars().count() > max_chars {
            self.error(field, format!("Höchstens {max_chars} Zeichen."));
        }
        text
    }

    /// Optional value checked by `check`, which returns the normalized value.
    pub fn optional_with(
        &mut self,
        field: &'static str,
        input: &str,
        check: impl FnOnce(&str) -> Result<String, String>,
    ) -> String {
        let text = clean_text(input);
        if text.is_empty() {
            return text;
        }
        match check(&text) {
            Ok(normalized) => normalized,
            Err(message) => {
                self.error(field, message);
                text
            }
        }
    }

    /// Required value checked by `check`.
    pub fn required_with(
        &mut self,
        field: &'static str,
        input: &str,
        check: impl FnOnce(&str) -> Result<String, String>,
    ) -> String {
        let text = clean_text(input);
        if text.is_empty() {
            self.error(field, "Pflichtfeld.");
            return text;
        }
        self.optional_with(field, &text, check)
    }

    pub fn optional_date(&mut self, field: &'static str, input: &str) -> Option<NaiveDate> {
        let text = input.trim();
        if text.is_empty() {
            return None;
        }
        match parse_date(text) {
            Ok(date) => Some(date),
            Err(message) => {
                self.error(field, message);
                None
            }
        }
    }

    pub fn required_date(&mut self, field: &'static str, input: &str) -> Option<NaiveDate> {
        if input.trim().is_empty() {
            self.error(field, "Pflichtfeld.");
            return None;
        }
        self.optional_date(field, input)
    }

    /// Money amount (≥ 0, at most two decimals).
    pub fn amount(&mut self, field: &'static str, input: &str) -> Option<Decimal> {
        match money::parse_amount(input) {
            Ok(value) if value > MAX_AMOUNT => {
                self.error(field, "Der Betrag ist unrealistisch hoch.");
                None
            }
            Ok(value) => Some(value),
            Err(error) => {
                self.error(field, number_message(error));
                None
            }
        }
    }

    /// Money amount that may be left empty.
    pub fn optional_amount(&mut self, field: &'static str, input: &str) -> Option<Decimal> {
        if input.trim().is_empty() { None } else { self.amount(field, input) }
    }

    /// Percentage between 0 and 100 with up to four decimals.
    pub fn percent(&mut self, field: &'static str, input: &str) -> Option<Decimal> {
        match money::parse_decimal(input, 4) {
            Ok(value) if value > Decimal::ONE_HUNDRED => {
                self.error(field, "Der Satz muss zwischen 0 und 100 % liegen.");
                None
            }
            Ok(value) => Some(value),
            Err(error) => {
                self.error(field, number_message(error));
                None
            }
        }
    }

    /// Non-negative decimal (hours, days) with `max_decimals` places and an upper bound.
    pub fn quantity(&mut self, field: &'static str, input: &str, max_decimals: u32, max: Decimal) -> Option<Decimal> {
        match money::parse_decimal(input, max_decimals) {
            Ok(value) if value > max => {
                self.error(field, format!("Höchstens {}.", money::format_flexible(max, 0, max_decimals)));
                None
            }
            Ok(value) => Some(value),
            Err(error) => {
                self.error(field, number_message(error));
                None
            }
        }
    }
}

/// Upper bound for single money amounts (sanity check against typos).
const MAX_AMOUNT: Decimal = Decimal::from_parts(99_999_999, 0, 0, false, 2); // 999.999,99

fn number_message(error: NumberError) -> String {
    match error {
        NumberError::Empty => "Pflichtfeld.".into(),
        other => other.to_string(),
    }
}

/// Trims and collapses internal whitespace; removes control characters.
pub fn clean_text(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ").chars().filter(|c| !c.is_control()).collect()
}

/// Parses `TT.MM.JJJJ` (also `T.M.JJJJ`) or ISO `JJJJ-MM-TT`.
pub fn parse_date(input: &str) -> Result<NaiveDate, String> {
    let text = input.trim();
    let message = || format!("„{text}“ ist kein gültiges Datum (TT.MM.JJJJ).");
    let parsed = if text.contains('-') {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").ok()
    } else {
        let parts: Vec<&str> = text.split('.').collect();
        if parts.len() != 3 || parts[2].len() != 4 {
            return Err(message());
        }
        let day = parts[0].parse::<u32>().ok();
        let month = parts[1].parse::<u32>().ok();
        let year = parts[2].parse::<i32>().ok();
        match (day, month, year) {
            (Some(d), Some(m), Some(y)) => NaiveDate::from_ymd_opt(y, m, d),
            _ => None,
        }
    };
    let date = parsed.ok_or_else(message)?;
    if !(1900..=2100).contains(&chrono::Datelike::year(&date)) {
        return Err("Das Jahr muss zwischen 1900 und 2100 liegen.".into());
    }
    Ok(date)
}

/// `15.03.1988`
pub fn format_date(date: NaiveDate) -> String {
    date.format("%d.%m.%Y").to_string()
}

pub fn format_optional_date(date: Option<NaiveDate>) -> String {
    date.map(format_date).unwrap_or_default()
}

fn only_digits(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace() && *c != '/' && *c != '-').collect()
}

/// German postal code: five digits.
pub fn postal_code(input: &str) -> Result<String, String> {
    let digits = input.trim();
    if digits.len() == 5 && digits.chars().all(|c| c.is_ascii_digit()) {
        Ok(digits.to_string())
    } else {
        Err("Die Postleitzahl muss aus 5 Ziffern bestehen.".into())
    }
}

/// Exactly `count` digits (spaces are ignored), e.g. Betriebsnummer (8).
pub fn digits(count: usize, what: &'static str) -> impl Fn(&str) -> Result<String, String> {
    move |input: &str| {
        let value: String = input.chars().filter(|c| !c.is_whitespace()).collect();
        if value.len() == count && value.chars().all(|c| c.is_ascii_digit()) {
            Ok(value)
        } else {
            Err(format!("{what} muss aus {count} Ziffern bestehen."))
        }
    }
}

/// Personalnummer: 1–20 characters, letters, digits, `.`, `_`, `-`;
/// it becomes part of folder and file names.
pub fn personnel_number(input: &str) -> Result<String, String> {
    let value = input.trim();
    let valid = !value.is_empty()
        && value.chars().count() <= 20
        && value.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
        && value.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if valid {
        Ok(value.to_string())
    } else {
        Err("Erlaubt sind 1–20 Zeichen: Buchstaben (ohne Umlaute), Ziffern, „.“, „_“ und „-“.".into())
    }
}

/// Sozialversicherungsnummer (Rentenversicherungsnummer), e.g. `12 150388 W 042`.
/// Checks format and check digit; returns the normalized form `12150388W042`.
pub fn social_security_number(input: &str) -> Result<String, String> {
    let value: String = input.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
    let chars: Vec<char> = value.chars().collect();
    let format_ok = chars.len() == 12
        && chars[..8].iter().all(|c| c.is_ascii_digit())
        && chars[8].is_ascii_uppercase()
        && chars[9..].iter().all(|c| c.is_ascii_digit());
    if !format_ok {
        return Err(
            "Format: 2 Ziffern, Geburtsdatum TTMMJJ, Anfangsbuchstabe, 3 Ziffern (z. B. 12 150388 W 042).".into()
        );
    }
    // The letter is replaced by its position in the alphabet (two digits).
    let letter_position = chars[8] as u32 - 'A' as u32 + 1;
    let mut digits: Vec<u32> = chars[..8].iter().map(|c| c.to_digit(10).expect("digit")).collect();
    digits.push(letter_position / 10);
    digits.push(letter_position % 10);
    digits.extend(chars[9..11].iter().map(|c| c.to_digit(10).expect("digit")));
    const WEIGHTS: [u32; 12] = [2, 1, 2, 5, 7, 1, 2, 1, 2, 1, 2, 1];
    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS)
        .map(|(digit, weight)| {
            let product = digit * weight;
            product / 10 + product % 10
        })
        .sum();
    let expected = sum % 10;
    let actual = chars[11].to_digit(10).expect("digit");
    if expected == actual {
        Ok(value)
    } else {
        Err("Die Prüfziffer der SV-Nummer stimmt nicht – bitte auf Tippfehler prüfen.".into())
    }
}

/// Steuerliche Identifikationsnummer (11 digits, ISO 7064 MOD 11,10 check digit).
pub fn tax_id(input: &str) -> Result<String, String> {
    let value = only_digits(input);
    if value.len() != 11 || !value.chars().all(|c| c.is_ascii_digit()) || value.starts_with('0') {
        return Err("Die Steuer-ID besteht aus 11 Ziffern und beginnt nicht mit 0.".into());
    }
    let digits: Vec<u32> = value.chars().map(|c| c.to_digit(10).expect("digit")).collect();
    let mut product = 10;
    for digit in &digits[..10] {
        let mut sum = (digit + product) % 10;
        if sum == 0 {
            sum = 10;
        }
        product = (sum * 2) % 11;
    }
    let check = match 11 - product {
        10 => 0,
        other => other,
    };
    if check == digits[10] {
        Ok(value)
    } else {
        Err("Die Prüfziffer der Steuer-ID stimmt nicht – bitte auf Tippfehler prüfen.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_social_security_numbers() {
        assert_eq!(social_security_number("12 150388 W 042"), Ok("12150388W042".into()));
        assert_eq!(social_security_number("65170839J003"), Ok("65170839J003".into()));
        assert_eq!(social_security_number("65 170839 j 003"), Ok("65170839J003".into()));
        assert!(social_security_number("65170839J004").unwrap_err().contains("Prüfziffer"));
        assert!(social_security_number("1215038W042").unwrap_err().contains("Format"));
        assert!(social_security_number("ABCDEFGHIJKL").is_err());
    }

    #[test]
    fn validates_tax_ids() {
        assert_eq!(tax_id("86095742719"), Ok("86095742719".into()));
        assert_eq!(tax_id("86 095 742 719"), Ok("86095742719".into()));
        assert!(tax_id("86095742718").unwrap_err().contains("Prüfziffer"));
        assert!(tax_id("06095742719").is_err());
        assert!(tax_id("8609574271").is_err());
    }

    #[test]
    fn parses_dates() {
        let date = NaiveDate::from_ymd_opt(1988, 3, 15).unwrap();
        assert_eq!(parse_date("15.03.1988"), Ok(date));
        assert_eq!(parse_date("15.3.1988"), Ok(date));
        assert_eq!(parse_date(" 1988-03-15 "), Ok(date));
        assert!(parse_date("31.02.2024").is_err());
        assert!(parse_date("15.03.88").is_err());
        assert!(parse_date("heute").is_err());
        assert!(parse_date("01.01.1850").is_err());
        assert_eq!(format_date(date), "15.03.1988");
    }

    #[test]
    fn simple_formats() {
        assert_eq!(postal_code("93336"), Ok("93336".into()));
        assert!(postal_code("9333").is_err());
        assert!(postal_code("9333a").is_err());
        assert_eq!(digits(8, "Die Betriebsnummer")("1234 5678"), Ok("12345678".into()));
        assert!(digits(8, "Die Betriebsnummer")("1234567").is_err());
        assert_eq!(personnel_number("P-1001"), Ok("P-1001".into()));
        assert!(personnel_number("Müller").is_err());
        assert!(personnel_number("a/b").is_err());
        assert!(personnel_number("-1").is_err());
        assert_eq!(clean_text("  Anna   Maria\t "), "Anna Maria");
    }

    #[test]
    fn validator_collects_errors() {
        let mut v = Validator::new();
        let name = v.required_text("name", "  ", 50);
        let amount = v.amount("amount", "12,345");
        let percent = v.percent("rate", "120");
        let ok = v.percent("rate2", "13,5");
        assert!(name.is_empty() && amount.is_none() && percent.is_none());
        assert_eq!(ok, Some(Decimal::new(135, 1)));
        let errors = v.finish(()).unwrap_err();
        assert_eq!(errors.len(), 3);
        assert_eq!(errors.get("name"), Some("Pflichtfeld."));
        assert!(errors.get("rate").unwrap().contains("100"));
    }
}
