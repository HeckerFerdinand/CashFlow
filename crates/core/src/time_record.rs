//! Monthly working time (Zeiterfassung).

use crate::period::Period;
use chrono::Datelike;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Upper bound for hours in one month (31 × 24).
pub const MAX_HOURS_PER_MONTH: Decimal = Decimal::from_parts(744, 0, 0, false, 0);
/// Upper bound for vacation or sick days in one month.
pub const MAX_DAYS_PER_MONTH: Decimal = Decimal::from_parts(31, 0, 0, false, 0);

/// What is needed to store a month's working time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeEntry {
    pub employee_id: Uuid,
    pub period: Period,
    /// Arbeitszeit (hours)
    pub hours_worked: Decimal,
    /// Arbeitszeit Regie (hours)
    pub extra_hours: Decimal,
    /// Urlaubstage
    pub vacation_days: Decimal,
    /// Krankheitstage
    pub sick_days: Decimal,
}

impl TimeEntry {
    /// Gesamtarbeitszeit
    pub fn total_hours(&self) -> Decimal {
        self.hours_worked + self.extra_hours
    }
}

/// Raw form input of the time entry. Keys: `hours_worked`, `extra_hours`,
/// `vacation_days`, `sick_days`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimeDraft {
    pub hours_worked: String,
    pub extra_hours: String,
    pub vacation_days: String,
    pub sick_days: String,
}

impl TimeDraft {
    pub fn get(&self, key: &str) -> Option<&str> {
        Some(match key {
            "hours_worked" => &self.hours_worked,
            "extra_hours" => &self.extra_hours,
            "vacation_days" => &self.vacation_days,
            "sick_days" => &self.sick_days,
            _ => return None,
        })
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) -> bool {
        let slot = match key {
            "hours_worked" => &mut self.hours_worked,
            "extra_hours" => &mut self.extra_hours,
            "vacation_days" => &mut self.vacation_days,
            "sick_days" => &mut self.sick_days,
            _ => return false,
        };
        *slot = value.into();
        true
    }

    pub fn from_entry(entry: &TimeEntry) -> Self {
        use crate::money::format_flexible;
        Self {
            hours_worked: format_flexible(entry.hours_worked, 0, 2),
            extra_hours: format_flexible(entry.extra_hours, 0, 2),
            vacation_days: format_flexible(entry.vacation_days, 0, 1),
            sick_days: format_flexible(entry.sick_days, 0, 1),
        }
    }

    pub fn validate(&self, employee_id: Uuid, period: Period) -> Result<TimeEntry, crate::ValidationErrors> {
        let mut v = crate::validate::Validator::new();
        let optional = |v: &mut crate::validate::Validator, key, input: &str, decimals, max| {
            if input.trim().is_empty() { Some(Decimal::ZERO) } else { v.quantity(key, input, decimals, max) }
        };
        let hours_worked = v.quantity("hours_worked", &self.hours_worked, 2, MAX_HOURS_PER_MONTH);
        let extra_hours = optional(&mut v, "extra_hours", &self.extra_hours, 2, MAX_HOURS_PER_MONTH);
        let vacation_days = optional(&mut v, "vacation_days", &self.vacation_days, 1, MAX_DAYS_PER_MONTH);
        let sick_days = optional(&mut v, "sick_days", &self.sick_days, 1, MAX_DAYS_PER_MONTH);
        let days_in_month = Decimal::from(period.last_day().day());
        if let (Some(vacation), Some(sick)) = (vacation_days, sick_days)
            && vacation + sick > days_in_month
        {
            v.error("sick_days", format!("Urlaub und Krankheit zusammen: höchstens {days_in_month} Tage."));
        }
        v.finish(TimeEntry {
            employee_id,
            period,
            hours_worked: hours_worked.unwrap_or_default(),
            extra_hours: extra_hours.unwrap_or_default(),
            vacation_days: vacation_days.unwrap_or_default(),
            sick_days: sick_days.unwrap_or_default(),
        })
    }
}

/// A stored time record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeRecord {
    pub id: Uuid,
    pub entry: TimeEntry,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn validates_time_input() {
        let feb = Period::new(2026, 2).unwrap();
        let draft = TimeDraft {
            hours_worked: "41,5".into(),
            extra_hours: "".into(),
            vacation_days: "1,5".into(),
            sick_days: "".into(),
        };
        let entry = draft.validate(Uuid::nil(), feb).unwrap();
        assert_eq!((entry.hours_worked, entry.extra_hours, entry.vacation_days), (dec!(41.5), dec!(0), dec!(1.5)));
        assert_eq!(entry.total_hours(), dec!(41.5));
        assert_eq!(TimeDraft::from_entry(&entry).validate(Uuid::nil(), feb).unwrap(), entry);

        let errors = TimeDraft { hours_worked: "".into(), vacation_days: "20".into(), sick_days: "10".into(), ..draft }
            .validate(Uuid::nil(), feb)
            .unwrap_err();
        assert!(errors.get("hours_worked").is_some());
        assert!(errors.get("sick_days").unwrap().contains("28"), "February has 28 days");
    }
}
