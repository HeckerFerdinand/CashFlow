//! PDF documents of CashFlow, rendered with Typst:
//! the monthly payslip (Lohnabrechnung) and the yearly wage and time journals.
//!
//! The layout lives in `templates/*.typ`; this crate prepares the data
//! (formatted German numbers and dates) and compiles the template to PDF.

mod model;
mod world;

use cashflow_core::{Employee, Employer, PayrollRecord, TimeRecord};
use chrono::NaiveDate;
use serde::Serialize;
use typst_layout::PagedDocument;

const PAYSLIP: &str = include_str!("../templates/payslip.typ");
const JOURNAL: &str = include_str!("../templates/journal.typ");

#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("{0}")]
    NoData(String),
    #[error("Das Dokument konnte nicht erstellt werden: {0}")]
    Render(String),
}

/// Employer and employee a document is about.
#[derive(Clone, Copy)]
pub struct Parties<'a> {
    pub employer: &'a Employer,
    pub employee: &'a Employee,
}

/// Monthly payslip. Dated with the record's statement date.
pub fn payslip(parties: Parties<'_>, record: &PayrollRecord) -> Result<Vec<u8>, DocumentError> {
    let data = model::payslip(parties, record);
    render(PAYSLIP, &data, record.entry.statement_date)
}

/// Yearly wage journal ("Lohnjournal") of one employee. Months without a
/// record are shown as "–"; at least one month must exist.
pub fn payroll_journal(
    parties: Parties<'_>,
    year: i32,
    records: &[PayrollRecord],
    created: NaiveDate,
) -> Result<Vec<u8>, DocumentError> {
    let data = model::payroll_journal(parties, year, records, created)?;
    render(JOURNAL, &data, created)
}

/// Yearly time journal ("Zeitjournal") of one employee.
pub fn time_journal(
    parties: Parties<'_>,
    year: i32,
    records: &[TimeRecord],
    created: NaiveDate,
) -> Result<Vec<u8>, DocumentError> {
    let data = model::time_journal(parties, year, records, created)?;
    render(JOURNAL, &data, created)
}

fn render(template: &str, data: &impl Serialize, today: NaiveDate) -> Result<Vec<u8>, DocumentError> {
    let json = serde_json::to_vec(data).map_err(|error| DocumentError::Render(error.to_string()))?;
    let world = world::DocumentWorld::new(template, json, today);
    let document: PagedDocument = typst::compile(&world)
        .output
        .map_err(|errors| DocumentError::Render(join(errors.iter().map(|e| e.message.as_str()))))?;
    typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default())
        .map_err(|errors| DocumentError::Render(join(errors.iter().map(|e| e.message.as_str()))))
}

fn join<'a>(messages: impl Iterator<Item = &'a str>) -> String {
    messages.collect::<Vec<_>>().join("; ")
}
