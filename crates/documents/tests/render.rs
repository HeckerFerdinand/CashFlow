//! Renders every document with sample data. The PDFs are written to
//! `target/document-samples/` so the layout can be checked by eye.

use cashflow_core::payroll::calculate;
use cashflow_core::{
    Address, Employee, EmployeeData, Employer, EmployerData, Gender, PayrollEntry, PayrollRecord, Period, Rates,
    TimeEntry, TimeRecord, TransitionZone,
};
use cashflow_documents::{DocumentError, Parties, payroll_journal, payslip, time_journal};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::path::PathBuf;
use uuid::Uuid;

fn employer() -> Employer {
    Employer {
        id: Uuid::new_v4(),
        data: EmployerData {
            name: "Hausverwaltung Muster".into(),
            representative: "vertreten durch Max Muster".into(),
            company_number: "12345678".into(),
            tax_number: "216/5738/0291".into(),
            address: Address {
                street: "Industrieweg".into(),
                house_number: "5".into(),
                postal_code: "50668".into(),
                city: "Köln".into(),
            },
        },
    }
}

fn employee(employer_id: Uuid) -> Employee {
    Employee {
        id: Uuid::new_v4(),
        data: EmployeeData {
            employer_id,
            personnel_number: "1001".into(),
            first_name: "Łukasz".into(),
            last_name: "Weber-Ğül".into(),
            birth_name: String::new(),
            address: Address {
                street: "Hauptstraße".into(),
                house_number: "12".into(),
                postal_code: "50667".into(),
                city: "Köln".into(),
            },
            birth_date: NaiveDate::from_ymd_opt(1988, 3, 15),
            gender: Some(Gender::Male),
            nationality: "deutsch".into(),
            social_security_number: "12150388W042".into(),
            tax_id: "86095742719".into(),
            activity_key: "123456789".into(),
            contribution_group_key: "6500".into(),
            person_group: "109".into(),
            transition_zone: Some(TransitionZone::No),
            occupation: "Hausmeister".into(),
            health_insurer: "Minijob-Zentrale (Knappschaft-Bahn-See)".into(),
            employment_start: NaiveDate::from_ymd_opt(2022, 4, 1),
            employment_end: None,
            monthly_salary: Some(dec!(538)),
            hourly_rate: Some(dec!(13.50)),
            rates: rates(),
        },
    }
}

fn rates() -> Rates {
    Rates {
        health: dec!(13),
        pension: dec!(15),
        u1: dec!(0.8),
        u2: dec!(0.22),
        insolvency: dec!(0.15),
        flat_tax: dec!(2),
    }
}

fn payroll(employee_id: Uuid, month: u32, base_pay: Decimal, extra_pay: Decimal) -> PayrollRecord {
    let period = Period::new(2025, month).unwrap();
    let entry = PayrollEntry {
        employee_id,
        period,
        statement_date: period.last_day(),
        base_pay,
        extra_pay,
        payout: base_pay + extra_pay - dec!(1.00),
        rates: rates(),
    };
    PayrollRecord { id: Uuid::new_v4(), calculation: calculate(base_pay, extra_pay, &rates()), entry }
}

fn save(name: &str, pdf: &[u8]) {
    assert!(pdf.starts_with(b"%PDF"), "{name} is not a PDF");
    assert!(pdf.len() > 5_000, "{name} is suspiciously small");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/document-samples");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(name), pdf).unwrap();
}

#[test]
fn renders_payslip() {
    let boss = employer();
    let person = employee(boss.id);
    let record = payroll(person.id, 2, dec!(538.00), dec!(40.50));
    let pdf = payslip(Parties { employer: &boss, employee: &person }, &record).unwrap();
    save("payslip.pdf", &pdf);
}

#[test]
fn renders_payroll_journal_with_gaps_and_large_amounts() {
    let boss = employer();
    let person = employee(boss.id);
    // March and August missing; large amounts must not wrap.
    let records: Vec<PayrollRecord> = (1..=12)
        .filter(|m| *m != 3 && *m != 8)
        .map(|m| payroll(person.id, m, if m == 12 { dec!(12345.67) } else { dec!(538.00) }, dec!(18.50)))
        .collect();
    let created = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
    let pdf = payroll_journal(Parties { employer: &boss, employee: &person }, 2025, &records, created).unwrap();
    save("payroll-journal.pdf", &pdf);
}

#[test]
fn renders_time_journal() {
    let boss = employer();
    let person = employee(boss.id);
    let records: Vec<TimeRecord> = (1..=12)
        .map(|m| TimeRecord {
            id: Uuid::new_v4(),
            entry: TimeEntry {
                employee_id: person.id,
                period: Period::new(2025, m).unwrap(),
                hours_worked: dec!(41.5),
                extra_hours: if m % 3 == 0 { dec!(3.25) } else { Decimal::ZERO },
                vacation_days: if m == 8 { dec!(5) } else { Decimal::ZERO },
                sick_days: if m == 2 { dec!(1.5) } else { Decimal::ZERO },
            },
        })
        .collect();
    let created = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
    let pdf = time_journal(Parties { employer: &boss, employee: &person }, 2025, &records, created).unwrap();
    save("time-journal.pdf", &pdf);
}

#[test]
fn journal_without_data_is_refused() {
    let boss = employer();
    let person = employee(boss.id);
    let created = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
    let error = payroll_journal(Parties { employer: &boss, employee: &person }, 2025, &[], created).unwrap_err();
    assert!(matches!(error, DocumentError::NoData(_)));
    assert!(error.to_string().contains("keine Löhne"));
}
