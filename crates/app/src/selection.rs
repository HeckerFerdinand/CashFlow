//! Employee, month and year choices shared by the entry and document pages.

use crate::context::App;
use cashflow_core::Period;
use cashflow_core::period::MONTH_NAMES;
use chrono::{Datelike, Local};
use slint::{ModelRc, SharedString, VecModel};
use std::cell::Cell;
use std::rc::Rc;
use uuid::Uuid;

thread_local! {
    /// Employee and month shared by wage and time entry, so switching between
    /// the two pages keeps the same person and month.
    static EMPLOYEE: Cell<Option<Uuid>> = const { Cell::new(None) };
    static PERIOD: Cell<Option<Period>> = const { Cell::new(None) };
}

pub fn current_employee() -> Option<Uuid> {
    EMPLOYEE.get()
}

pub fn set_employee(employee: Option<Uuid>) {
    EMPLOYEE.set(employee);
}

/// The selected month (initially [`default_period`]).
pub fn current_period() -> Period {
    PERIOD.get().unwrap_or_else(default_period)
}

pub fn set_period(period: Period) {
    PERIOD.set(Some(period));
}

/// Years offered in the pickers: eight years back (retention period) up to next year.
pub fn years() -> Vec<i32> {
    let current = Local::now().year();
    (current - 8..=current + 1).collect()
}

pub fn year_labels(years: &[i32]) -> ModelRc<SharedString> {
    ModelRc::new(VecModel::from(years.iter().map(|y| SharedString::from(y.to_string())).collect::<Vec<_>>()))
}

pub fn month_labels() -> ModelRc<SharedString> {
    ModelRc::new(VecModel::from(MONTH_NAMES.iter().map(|m| SharedString::from(*m)).collect::<Vec<_>>()))
}

/// The month that is usually being processed: the current month, or the
/// previous one during the first ten days (payroll is often done afterwards).
pub fn default_period() -> Period {
    let today = Local::now().date_naive();
    let current = Period::containing(today);
    if today.day() <= 10 { current.previous() } else { current }
}

/// Employees to choose from: everyone employed in `year` (plus `keep`, so a
/// current selection never disappears), sorted by name.
pub fn employees_for_year(app: &Rc<App>, year: i32, keep: Option<Uuid>) -> (Vec<Uuid>, ModelRc<SharedString>) {
    let cache = app.cache.borrow();
    let mut employees: Vec<_> =
        cache.employees.iter().filter(|e| e.data.is_employed_in_year(year) || Some(e.id) == keep).collect();
    employees.sort_by_key(|e| e.data.sort_name().to_lowercase());
    let ids = employees.iter().map(|e| e.id).collect();
    let labels: Vec<SharedString> = employees.iter().map(|e| e.list_label().into()).collect();
    (ids, ModelRc::new(VecModel::from(labels)))
}

pub fn index_of<T: PartialEq>(items: &[T], item: &T) -> i32 {
    items.iter().position(|x| x == item).map(|i| i as i32).unwrap_or(-1)
}

/// The first of `ids` who is employed during `period`.
pub fn first_employed(app: &Rc<App>, ids: &[Uuid], period: Period) -> Option<Uuid> {
    let cache = app.cache.borrow();
    ids.iter().copied().find(|id| {
        cache.employee(*id).is_some_and(|e| {
            e.data.employment_start.is_none_or(|start| start <= period.last_day())
                && e.data.employment_end.is_none_or(|end| end >= period.first_day())
        })
    })
}

/// Up to two upper-case initials ("Weber, Anna" → "AW" when given first/last name).
pub fn initials(first: &str, second: &str) -> String {
    [first, second].iter().filter_map(|part| part.trim().chars().next()).flat_map(char::to_uppercase).collect()
}

/// Initials of a company name: the first letters of the first two words.
pub fn name_initials(name: &str) -> String {
    let mut words = name.split_whitespace().filter(|w| w.chars().next().is_some_and(char::is_alphanumeric));
    initials(words.next().unwrap_or_default(), words.next().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_initials() {
        assert_eq!(initials("anna", "Weber"), "AW");
        assert_eq!(initials("Łukasz", ""), "Ł");
        assert_eq!(name_initials("Hausverwaltung Hecker GmbH"), "HH");
        assert_eq!(name_initials("WEG – Lindenallee 12"), "WL");
        assert_eq!(name_initials(""), "");
    }
}
