//! Parsing, rounding and formatting of amounts in German notation.
//!
//! All money is handled as [`Decimal`] (never `f64`) so that cents add up
//! exactly. Rounding is commercial rounding ("kaufmännisches Runden":
//! half away from zero), which is what payroll documents expect.

use rust_decimal::{Decimal, RoundingStrategy};
use std::str::FromStr;

/// Why a number typed by the user could not be read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NumberError {
    #[error("Bitte einen Wert eingeben.")]
    Empty,
    #[error("„{0}“ ist keine gültige Zahl.")]
    Invalid(String),
    #[error("Höchstens {0} Nachkommastellen erlaubt.")]
    TooManyDecimals(u32),
    #[error("Der Wert darf nicht negativ sein.")]
    Negative,
}

/// Rounds to whole cents (half away from zero).
pub fn round_cents(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
}

/// Parses a non-negative money amount as typed by a user.
///
/// Accepts German and plain notation: `1.234,56`, `1234,56`, `1234.56`,
/// `1 234,56 €`, `3200`. Because amounts have at most two decimals, a single
/// dot followed by exactly three digits (`1.234`) is a thousands separator.
pub fn parse_amount(input: &str) -> Result<Decimal, NumberError> {
    let value = parse_localized(input, Separators::Grouped)?;
    check_scale(value, 2)?;
    if value.is_sign_negative() && !value.is_zero() {
        return Err(NumberError::Negative);
    }
    Ok(value.normalize().max(Decimal::ZERO).round_dp(2))
}

/// Parses a non-negative decimal number with at most `max_decimals` places
/// (hours, days, percentages). Both `,` and `.` are accepted as decimal
/// separator; thousands separators are not allowed.
pub fn parse_decimal(input: &str, max_decimals: u32) -> Result<Decimal, NumberError> {
    let value = parse_localized(input, Separators::DecimalOnly)?;
    check_scale(value, max_decimals)?;
    if value.is_sign_negative() && !value.is_zero() {
        return Err(NumberError::Negative);
    }
    Ok(value.normalize())
}

/// Like [`parse_decimal`] but also allows negative values (e.g. a negative
/// carry-over of working hours).
pub fn parse_signed_decimal(input: &str, max_decimals: u32) -> Result<Decimal, NumberError> {
    let value = parse_localized(input, Separators::DecimalOnly)?;
    check_scale(value, max_decimals)?;
    Ok(value.normalize())
}

fn check_scale(value: Decimal, max_decimals: u32) -> Result<(), NumberError> {
    if value.normalize().scale() > max_decimals { Err(NumberError::TooManyDecimals(max_decimals)) } else { Ok(()) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Separators {
    /// Thousands separators (`.` or space) are allowed, as in `1.234,56`.
    Grouped,
    /// Only a single decimal separator (`,` or `.`) is allowed.
    DecimalOnly,
}

fn parse_localized(input: &str, separators: Separators) -> Result<Decimal, NumberError> {
    let cleaned: String = input
        .trim()
        .trim_end_matches('€')
        .trim_end_matches('%')
        .chars()
        .filter(|c| !matches!(c, ' ' | '\u{a0}' | '\u{202f}' | '\''))
        .collect();
    if cleaned.is_empty() {
        return Err(NumberError::Empty);
    }
    let invalid = || NumberError::Invalid(input.trim().to_string());
    if !cleaned
        .chars()
        .enumerate()
        .all(|(i, c)| c.is_ascii_digit() || c == ',' || c == '.' || (i == 0 && (c == '-' || c == '+')))
    {
        return Err(invalid());
    }

    let commas = cleaned.matches(',').count();
    let dots = cleaned.matches('.').count();
    let normalized = match separators {
        Separators::DecimalOnly => match (commas, dots) {
            (0, 0) | (0, 1) => cleaned.clone(),
            (1, 0) => cleaned.replace(',', "."),
            _ => return Err(invalid()),
        },
        Separators::Grouped => match (cleaned.rfind(','), cleaned.rfind('.')) {
            (None, None) => cleaned.clone(),
            (Some(_), None) if commas == 1 => cleaned.replace(',', "."),
            (Some(_), None) => return Err(invalid()),
            (None, Some(dot)) if dots == 1 => {
                if cleaned.len() - dot - 1 == 3 {
                    // "1.234" can only be a thousands separator for amounts.
                    cleaned.replace('.', "")
                } else {
                    cleaned.clone()
                }
            }
            (None, Some(_)) => {
                if !valid_grouping(&cleaned, '.') {
                    return Err(invalid());
                }
                cleaned.replace('.', "")
            }
            // German "1.234,56": dots group thousands, the last comma separates decimals.
            (Some(comma), Some(dot)) if comma > dot && commas == 1 => {
                if !valid_grouping(&cleaned[..comma], '.') {
                    return Err(invalid());
                }
                cleaned.replace('.', "").replace(',', ".")
            }
            // English "1,234.56": commas group thousands, the last dot separates decimals.
            (Some(comma), Some(dot)) if dot > comma && dots == 1 => {
                if !valid_grouping(&cleaned[..dot], ',') {
                    return Err(invalid());
                }
                cleaned.replace(',', "")
            }
            _ => return Err(invalid()),
        },
    };
    if normalized.starts_with('.') || normalized.ends_with('.') || normalized == "-" || normalized == "+" {
        return Err(invalid());
    }
    Decimal::from_str(&normalized).map_err(|_| invalid())
}

/// Checks that thousands separators sit every three digits ("1.234.567").
fn valid_grouping(int_part: &str, separator: char) -> bool {
    let digits = int_part.trim_start_matches(['-', '+']);
    if !digits.contains(separator) {
        return !digits.is_empty();
    }
    let mut groups = digits.split(separator);
    let Some(first) = groups.next() else { return false };
    if first.is_empty() || first.len() > 3 {
        return false;
    }
    groups.all(|group| group.len() == 3)
}

/// Formats a number with German separators and a fixed number of decimals:
/// `format_number(1234.5, 2)` → `"1.234,50"`.
pub fn format_number(value: Decimal, decimals: u32) -> String {
    let rounded = value.round_dp_with_strategy(decimals, RoundingStrategy::MidpointAwayFromZero);
    let negative = rounded.is_sign_negative() && !rounded.is_zero();
    let plain = format!("{:.*}", decimals as usize, rounded.abs());
    let (int_part, frac_part) = match plain.split_once('.') {
        Some((i, f)) => (i.to_string(), Some(f.to_string())),
        None => (plain.clone(), None),
    };
    let mut grouped = String::with_capacity(int_part.len() + int_part.len() / 3);
    for (i, c) in int_part.chars().enumerate() {
        if i > 0 && (int_part.len() - i) % 3 == 0 {
            grouped.push('.');
        }
        grouped.push(c);
    }
    let mut out = String::new();
    if negative {
        out.push('-');
    }
    out.push_str(&grouped);
    if let Some(frac) = frac_part {
        out.push(',');
        out.push_str(&frac);
    }
    out
}

/// `1234.5` → `"1.234,50"`.
pub fn format_amount(value: Decimal) -> String {
    format_number(value, 2)
}

/// `1234.5` → `"1.234,50 €"`.
pub fn format_eur(value: Decimal) -> String {
    format!("{} €", format_amount(value))
}

/// Formats a decimal with as many decimals as needed (at least `min`, at most `max`):
/// `format_flexible(13, 0, 4)` → `"13"`, `format_flexible(0.06, 0, 4)` → `"0,06"`.
pub fn format_flexible(value: Decimal, min_decimals: u32, max_decimals: u32) -> String {
    let rounded = value.round_dp_with_strategy(max_decimals, RoundingStrategy::MidpointAwayFromZero);
    let scale = rounded.normalize().scale().clamp(min_decimals, max_decimals);
    format_number(rounded, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn parses_common_amount_notations() {
        assert_eq!(parse_amount("3200"), Ok(dec!(3200)));
        assert_eq!(parse_amount("3200,5"), Ok(dec!(3200.5)));
        assert_eq!(parse_amount("3200.50"), Ok(dec!(3200.50)));
        assert_eq!(parse_amount("3.200,50"), Ok(dec!(3200.50)));
        assert_eq!(parse_amount("1.234"), Ok(dec!(1234)));
        assert_eq!(parse_amount("1.234.567,89"), Ok(dec!(1234567.89)));
        assert_eq!(parse_amount("1,234.56"), Ok(dec!(1234.56)));
        assert_eq!(parse_amount(" 538,00 € "), Ok(dec!(538)));
        assert_eq!(parse_amount("1 234,56"), Ok(dec!(1234.56)));
        assert_eq!(parse_amount("0"), Ok(dec!(0)));
    }

    #[test]
    fn rejects_invalid_amounts() {
        assert_eq!(parse_amount(""), Err(NumberError::Empty));
        assert_eq!(parse_amount("   "), Err(NumberError::Empty));
        assert!(matches!(parse_amount("abc"), Err(NumberError::Invalid(_))));
        assert!(matches!(parse_amount("12,3,4"), Err(NumberError::Invalid(_))));
        assert!(matches!(parse_amount("1.23,45"), Err(NumberError::Invalid(_))));
        assert!(matches!(parse_amount(","), Err(NumberError::Invalid(_))));
        assert!(matches!(parse_amount("12a"), Err(NumberError::Invalid(_))));
        assert_eq!(parse_amount("12,345"), Err(NumberError::TooManyDecimals(2)));
        assert_eq!(parse_amount("-5"), Err(NumberError::Negative));
    }

    #[test]
    fn parses_decimals_with_either_separator() {
        assert_eq!(parse_decimal("13", 4), Ok(dec!(13)));
        assert_eq!(parse_decimal("0,06", 4), Ok(dec!(0.06)));
        assert_eq!(parse_decimal("1.1", 4), Ok(dec!(1.1)));
        assert_eq!(parse_decimal("1.234", 4), Ok(dec!(1.234)));
        assert_eq!(parse_decimal("13 %", 4), Ok(dec!(13)));
        assert!(matches!(parse_decimal("1.234,5", 4), Err(NumberError::Invalid(_))));
        assert_eq!(parse_decimal("0,12345", 4), Err(NumberError::TooManyDecimals(4)));
        assert_eq!(parse_decimal("-1", 2), Err(NumberError::Negative));
        assert_eq!(parse_signed_decimal("-1,5", 2), Ok(dec!(-1.5)));
    }

    #[test]
    fn rounds_commercially() {
        assert_eq!(round_cents(dec!(0.125)), dec!(0.13));
        assert_eq!(round_cents(dec!(0.124)), dec!(0.12));
        assert_eq!(round_cents(dec!(-0.125)), dec!(-0.13));
        assert_eq!(round_cents(dec!(69.94)), dec!(69.94));
    }

    #[test]
    fn formats_german_numbers() {
        assert_eq!(format_amount(dec!(1234.5)), "1.234,50");
        assert_eq!(format_amount(dec!(0)), "0,00");
        assert_eq!(format_amount(dec!(999)), "999,00");
        assert_eq!(format_amount(dec!(1000)), "1.000,00");
        assert_eq!(format_amount(dec!(1234567.891)), "1.234.567,89");
        assert_eq!(format_amount(dec!(-1234.5)), "-1.234,50");
        assert_eq!(format_eur(dec!(538)), "538,00 €");
        assert_eq!(format_number(dec!(7.5), 1), "7,5");
        assert_eq!(format_flexible(dec!(13), 0, 4), "13");
        assert_eq!(format_flexible(dec!(0.06), 0, 4), "0,06");
        assert_eq!(format_flexible(dec!(1.10), 2, 4), "1,10");
    }
}
