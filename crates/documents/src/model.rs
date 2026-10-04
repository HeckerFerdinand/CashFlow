//! Data handed to the Typst templates (as `data.json`). Every value is a
//! ready-to-print string, so the templates contain no number formatting.

use crate::{DocumentError, Parties};
use cashflow_core::employee::format_social_security_number;
use cashflow_core::money::{format_amount, format_eur, format_flexible, format_number};
use cashflow_core::validate::{format_date, format_optional_date};
use cashflow_core::{Contribution, PayrollRecord, Period, TimeRecord};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

/// Column headers of the journals.
const MONTH_HEADERS: [&str; 12] = ["JAN", "FEB", "MÄR", "APR", "MAI", "JUN", "JUL", "AUG", "SEP", "OKT", "NOV", "DEZ"];
const MISSING: &str = "–";

// -- Payslip --------------------------------------------------------------------

#[derive(Serialize)]
pub(crate) struct Payslip {
    title: &'static str,
    date: String,
    period: String,
    employee: PayslipEmployee,
    employer: PayslipEmployer,
    gross: Gross,
    social: Social,
    net: String,
    payout: String,
}

#[derive(Serialize)]
struct PayslipEmployee {
    personnel_number: String,
    birth_date: String,
    ssn: String,
    health_insurer: String,
    person_group: String,
    contribution_group: String,
    tax_id: String,
    employment_start: String,
    name: String,
    street_line: String,
    city_line: String,
}

#[derive(Serialize)]
struct PayslipEmployer {
    name_line: String,
    address_line: String,
}

#[derive(Serialize)]
struct Gross {
    base_label: String,
    base: String,
    extra_label: &'static str,
    extra: String,
    total: String,
}

#[derive(Serialize)]
struct Social {
    kv_gross: String,
    rv_gross: String,
    deductions: String,
}

pub(crate) fn payslip(parties: Parties<'_>, record: &PayrollRecord) -> Payslip {
    let employee = &parties.employee.data;
    let employer = &parties.employer.data;
    let entry = &record.entry;
    let gross_total = record.calculation.gross_total;
    let base_label = if employee.occupation.is_empty() {
        "Vergütung".to_string()
    } else {
        format!("Vergütung {}", employee.occupation)
    };
    Payslip {
        title: "Lohnabrechnung der Brutto/Netto-Bezüge",
        date: format_date(entry.statement_date),
        period: entry.period.to_string(),
        employee: PayslipEmployee {
            personnel_number: employee.personnel_number.clone(),
            birth_date: format_optional_date(employee.birth_date),
            ssn: format_social_security_number(&employee.social_security_number),
            health_insurer: employee.health_insurer.clone(),
            person_group: employee.person_group.clone(),
            contribution_group: employee.contribution_group_key.clone(),
            tax_id: employee.tax_id.clone(),
            employment_start: format_optional_date(employee.employment_start),
            name: employee.full_name(),
            street_line: employee.address.street_line(),
            city_line: employee.address.city_line(),
        },
        employer: PayslipEmployer { name_line: employer.full_name(), address_line: employer.address.single_line() },
        gross: Gross {
            base_label,
            base: format_eur(entry.base_pay),
            extra_label: "Vergütung Regiestunden",
            extra: format_eur(entry.extra_pay),
            total: format_eur(gross_total),
        },
        social: Social {
            kv_gross: format_eur(gross_total),
            rv_gross: format_eur(gross_total),
            deductions: format_eur(record.deductions()),
        },
        net: format_eur(entry.payout),
        payout: format_eur(entry.payout),
    }
}

// -- Journals ---------------------------------------------------------------------

#[derive(Serialize)]
pub(crate) struct Journal {
    title: String,
    date: String,
    /// Shown in the header cell of the label column (unit of the values).
    unit: &'static str,
    employee: JournalEmployee,
    employer: JournalEmployer,
    months: [&'static str; 12],
    groups: Vec<Vec<Row>>,
    note: String,
}

#[derive(Serialize)]
struct JournalEmployee {
    name: String,
    street_line: String,
    city_line: String,
    ssn: String,
    personnel_number: String,
}

#[derive(Serialize)]
struct JournalEmployer {
    name: String,
    representative: String,
    street_line: String,
    city_line: String,
    company_number: String,
    tax_number: String,
}

#[derive(Serialize)]
struct Row {
    label: String,
    bold: bool,
    values: Vec<String>,
    total: String,
}

/// Builds one row: a value per month (or "–" if the month has no record)
/// and the sum of the existing months.
fn row<T>(
    label: &str,
    bold: bool,
    by_month: &[Option<&T>; 12],
    value: impl Fn(&T) -> Decimal,
    format: impl Fn(Decimal) -> String,
) -> Row {
    let values =
        by_month.iter().map(|record| record.map(|r| format(value(r))).unwrap_or_else(|| MISSING.into())).collect();
    let total: Decimal = by_month.iter().flatten().map(|r| value(r)).sum();
    Row { label: label.into(), bold, values, total: format(total) }
}

fn header_parts(parties: Parties<'_>) -> (JournalEmployee, JournalEmployer) {
    let employee = &parties.employee.data;
    let employer = &parties.employer.data;
    (
        JournalEmployee {
            name: employee.full_name(),
            street_line: employee.address.street_line(),
            city_line: employee.address.city_line(),
            ssn: format_social_security_number(&employee.social_security_number),
            personnel_number: employee.personnel_number.clone(),
        },
        JournalEmployer {
            name: employer.name.clone(),
            representative: employer.representative.clone(),
            street_line: employer.address.street_line(),
            city_line: employer.address.city_line(),
            company_number: employer.company_number.clone(),
            tax_number: employer.tax_number.clone(),
        },
    )
}

/// Note listing the months without a record ("" if all twelve exist).
fn missing_note(year: i32, present: &[bool; 12]) -> String {
    let missing: Vec<&str> = Period::months_of(year)
        .zip(present)
        .filter(|(_, present)| !**present)
        .map(|(period, _)| period.month_name())
        .collect();
    if missing.is_empty() { String::new() } else { format!("„–“: keine Erfassung für {}.", missing.join(", ")) }
}

fn by_month<T>(records: &[T], year: i32, period: impl Fn(&T) -> Period) -> [Option<&T>; 12] {
    let mut months: [Option<&T>; 12] = [None; 12];
    for record in records {
        let p = period(record);
        if p.year() == year {
            months[(p.month() - 1) as usize] = Some(record);
        }
    }
    months
}

pub(crate) fn payroll_journal(
    parties: Parties<'_>,
    year: i32,
    records: &[PayrollRecord],
    created: NaiveDate,
) -> Result<Journal, DocumentError> {
    let months = by_month(records, year, |r| r.entry.period);
    if months.iter().all(Option::is_none) {
        return Err(DocumentError::NoData(format!(
            "Für {} sind {year} keine Löhne erfasst.",
            parties.employee.data.full_name()
        )));
    }
    let (employee, employer) = header_parts(parties);
    let amount = |v: Decimal| format_amount(v);
    let contributions: Vec<Row> = Contribution::ALL
        .iter()
        .map(|c| {
            row(c.journal_label(), false, &months, |r: &PayrollRecord| r.calculation.contributions.get(*c), amount)
        })
        .collect();
    Ok(Journal {
        title: format!("Lohnjournal der Brutto/Netto-Bezüge {year}"),
        date: format_date(created),
        unit: "in €",
        employee,
        employer,
        months: MONTH_HEADERS,
        groups: vec![
            vec![
                row("Vergütung brutto", false, &months, |r: &PayrollRecord| r.entry.base_pay, amount),
                row("Sonstige Vergütung", false, &months, |r: &PayrollRecord| r.entry.extra_pay, amount),
                row("Gesamtbrutto", true, &months, |r: &PayrollRecord| r.calculation.gross_total, amount),
                row("Auszahlung netto", true, &months, |r: &PayrollRecord| r.entry.payout, amount),
            ],
            contributions,
            vec![row("Gesamtbeitrag", true, &months, |r: &PayrollRecord| r.calculation.total_contribution, amount)],
        ],
        note: missing_note(year, &months.map(|m| m.is_some())),
    })
}

pub(crate) fn time_journal(
    parties: Parties<'_>,
    year: i32,
    records: &[TimeRecord],
    created: NaiveDate,
) -> Result<Journal, DocumentError> {
    let months = by_month(records, year, |r| r.entry.period);
    if months.iter().all(Option::is_none) {
        return Err(DocumentError::NoData(format!(
            "Für {} sind {year} keine Arbeitszeiten erfasst.",
            parties.employee.data.full_name()
        )));
    }
    let (employee, employer) = header_parts(parties);
    let hours = |v: Decimal| format_number(v, 2);
    let days = |v: Decimal| format_flexible(v, 0, 1);
    Ok(Journal {
        title: format!("Zeitjournal {year}"),
        date: format_date(created),
        unit: "Std. / Tage",
        employee,
        employer,
        months: MONTH_HEADERS,
        groups: vec![
            vec![
                row("Arbeitszeit (Std.)", false, &months, |r: &TimeRecord| r.entry.hours_worked, hours),
                row("Arbeitszeit Regie (Std.)", false, &months, |r: &TimeRecord| r.entry.extra_hours, hours),
                row("Gesamtarbeitszeit (Std.)", true, &months, |r: &TimeRecord| r.entry.total_hours(), hours),
            ],
            vec![
                row("Urlaubstage", false, &months, |r: &TimeRecord| r.entry.vacation_days, days),
                row("Krankheitstage", false, &months, |r: &TimeRecord| r.entry.sick_days, days),
            ],
        ],
        note: missing_note(year, &months.map(|m| m.is_some())),
    })
}
