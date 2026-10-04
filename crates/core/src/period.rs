//! Payroll periods (one calendar month of one year).

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt;

/// German month names, index 0 = January.
pub const MONTH_NAMES: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

/// Earliest and latest year the app accepts for payroll periods.
pub const MIN_YEAR: i32 = 2000;
pub const MAX_YEAR: i32 = 2099;

/// One accounting month, e.g. März 2026.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Period {
    year: i32,
    month: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PeriodError {
    #[error("Ungültiger Monat: {0}")]
    InvalidMonth(u32),
    #[error("Das Jahr muss zwischen {MIN_YEAR} und {MAX_YEAR} liegen.")]
    InvalidYear(i32),
}

impl Period {
    pub fn new(year: i32, month: u32) -> Result<Self, PeriodError> {
        if !(1..=12).contains(&month) {
            return Err(PeriodError::InvalidMonth(month));
        }
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(PeriodError::InvalidYear(year));
        }
        Ok(Self { year, month })
    }

    /// The period containing `date`.
    pub fn containing(date: NaiveDate) -> Self {
        Self { year: date.year(), month: date.month() }
    }

    pub fn year(self) -> i32 {
        self.year
    }

    /// Month number 1–12.
    pub fn month(self) -> u32 {
        self.month
    }

    /// German month name, e.g. "März".
    pub fn month_name(self) -> &'static str {
        MONTH_NAMES[(self.month - 1) as usize]
    }

    pub fn first_day(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, 1).expect("validated period")
    }

    /// Last day of the month (leap years included).
    pub fn last_day(self) -> NaiveDate {
        self.next().first_day().pred_opt().expect("validated period")
    }

    pub fn next(self) -> Self {
        if self.month == 12 {
            Self { year: self.year + 1, month: 1 }
        } else {
            Self { year: self.year, month: self.month + 1 }
        }
    }

    pub fn previous(self) -> Self {
        if self.month == 1 {
            Self { year: self.year - 1, month: 12 }
        } else {
            Self { year: self.year, month: self.month - 1 }
        }
    }

    /// All twelve periods of a year.
    pub fn months_of(year: i32) -> impl Iterator<Item = Period> {
        (1..=12).map(move |month| Period { year, month })
    }
}

impl fmt::Display for Period {
    /// "März 2026"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.month_name(), self.year)
    }
}

/// Returns the month number (1–12) for a German month name ("März" → 3).
pub fn month_from_name(name: &str) -> Option<u32> {
    let name = name.trim();
    MONTH_NAMES.iter().position(|candidate| candidate.eq_ignore_ascii_case(name)).map(|index| index as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_day_handles_leap_years() {
        assert_eq!(Period::new(2024, 2).unwrap().last_day(), NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
        assert_eq!(Period::new(2025, 2).unwrap().last_day(), NaiveDate::from_ymd_opt(2025, 2, 28).unwrap());
        assert_eq!(Period::new(2025, 12).unwrap().last_day(), NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
        assert_eq!(Period::new(2025, 4).unwrap().last_day(), NaiveDate::from_ymd_opt(2025, 4, 30).unwrap());
    }

    #[test]
    fn validates_ranges() {
        assert_eq!(Period::new(2025, 0), Err(PeriodError::InvalidMonth(0)));
        assert_eq!(Period::new(2025, 13), Err(PeriodError::InvalidMonth(13)));
        assert_eq!(Period::new(1999, 1), Err(PeriodError::InvalidYear(1999)));
    }

    #[test]
    fn names_and_navigation() {
        let march = Period::new(2026, 3).unwrap();
        assert_eq!(march.to_string(), "März 2026");
        assert_eq!(march.previous().month_name(), "Februar");
        assert_eq!(Period::new(2026, 12).unwrap().next(), Period::new(2027, 1).unwrap());
        assert_eq!(Period::new(2026, 1).unwrap().previous(), Period::new(2025, 12).unwrap());
        assert_eq!(month_from_name("märz"), Some(3));
        assert_eq!(month_from_name("Dezember"), Some(12));
        assert_eq!(month_from_name("Foo"), None);
        assert_eq!(Period::months_of(2026).count(), 12);
    }
}
