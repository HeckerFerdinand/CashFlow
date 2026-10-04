//! Integration tests against the local Supabase stand-in (Postgres + PostgREST
//! with the real `supabase/schema.sql`).
//!
//! Run with:
//!     scripts/dev-db.sh reset && eval "$(scripts/dev-db.sh env)" && cargo test -p cashflow-data
//!
//! Without `CASHFLOW_TEST_REST_URL` the tests print a note and pass.

use cashflow_core::payroll::calculate;
use cashflow_core::{
    Address, Defaults, EmployeeData, EmployerData, Gender, PayrollEntry, Period, Rates, TimeEntry, TransitionZone,
};
use cashflow_data::{DataError, Database, Session, User};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

const ALLOWED_USER: &str = "00000000-0000-0000-0000-00000000a11d";
const BLOCKED_USER: &str = "00000000-0000-0000-0000-0000000b10cd";

async fn connect(user: &str) -> Option<Database> {
    let (Ok(url), Ok(secret)) = (std::env::var("CASHFLOW_TEST_REST_URL"), std::env::var("CASHFLOW_TEST_JWT_SECRET"))
    else {
        eprintln!("CASHFLOW_TEST_REST_URL not set – skipping (see scripts/dev-db.sh)");
        return None;
    };
    #[derive(serde::Serialize)]
    struct Claims<'a> {
        sub: &'a str,
        role: &'a str,
        exp: i64,
    }
    let expires_at = Utc::now() + chrono::Duration::hours(1);
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims { sub: user, role: "authenticated", exp: expires_at.timestamp() },
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("sign test token");
    let database = Database::for_local_postgrest(&url).expect("client");
    database
        .set_session_for_tests(Session {
            access_token: token,
            refresh_token: "not-used".into(),
            expires_at,
            user: User { id: Uuid::parse_str(user).unwrap(), email: None },
        })
        .await;
    Some(database)
}

fn unique(prefix: &str) -> String {
    format!("{prefix}{}", &Uuid::new_v4().simple().to_string()[..8])
}

fn employer(name: &str) -> EmployerData {
    EmployerData {
        name: name.into(),
        representative: "vertreten durch Ferdinand Hecker".into(),
        company_number: "12345678".into(),
        tax_number: "216/5738/0291".into(),
        address: Address {
            street: "Schulstraße".into(),
            house_number: "34".into(),
            postal_code: "93336".into(),
            city: "Altmannstein".into(),
        },
    }
}

fn rates() -> Rates {
    Rates {
        health: dec!(13),
        pension: dec!(15),
        u1: dec!(1.1),
        u2: dec!(0.24),
        insolvency: dec!(0.06),
        flat_tax: dec!(2),
    }
}

fn employee(employer_id: Uuid, personnel_number: &str) -> EmployeeData {
    EmployeeData {
        employer_id,
        personnel_number: personnel_number.into(),
        first_name: "Łukasz".into(),
        last_name: "Weber-Ğül".into(),
        birth_name: "Schneider".into(),
        address: Address {
            street: "Hauptstraße".into(),
            house_number: "12a".into(),
            postal_code: "50667".into(),
            city: "Köln".into(),
        },
        birth_date: NaiveDate::from_ymd_opt(1988, 3, 15),
        gender: Some(Gender::Female),
        nationality: "deutsch".into(),
        social_security_number: "12150388W042".into(),
        tax_id: "86095742719".into(),
        activity_key: "123456789".into(),
        contribution_group_key: "6500".into(),
        person_group: "109".into(),
        transition_zone: Some(TransitionZone::No),
        occupation: "Reinigungskraft".into(),
        health_insurer: "Minijob-Zentrale".into(),
        employment_start: NaiveDate::from_ymd_opt(2022, 4, 1),
        employment_end: None,
        monthly_salary: Some(dec!(538.00)),
        hourly_rate: Some(dec!(13.50)),
        rates: rates(),
    }
}

#[tokio::test]
async fn access_requires_app_user() {
    let Some(blocked) = connect(BLOCKED_USER).await else { return };
    let status = blocked.app_status().await.unwrap();
    assert!(!status.is_app_user);
    assert!(blocked.list_employers().await.unwrap().is_empty(), "RLS must hide all rows");
    let error = blocked.create_employer(&employer(&unique("Blocked "))).await.unwrap_err();
    assert!(matches!(error, DataError::Forbidden), "{error:?}");

    let allowed = connect(ALLOWED_USER).await.unwrap();
    let status = allowed.app_status().await.unwrap();
    assert!(status.is_app_user);
    assert_eq!(status.schema_version, cashflow_data::EXPECTED_SCHEMA_VERSION);
}

#[tokio::test]
async fn employer_crud_and_friendly_errors() {
    let Some(db) = connect(ALLOWED_USER).await else { return };
    let name = unique("Hausverwaltung ");
    let created = db.create_employer(&employer(&name)).await.unwrap();
    assert_eq!(created.data, employer(&name));
    assert!(db.list_employers().await.unwrap().iter().any(|e| e.id == created.id));

    let duplicate = db.create_employer(&employer(&name.to_uppercase())).await.unwrap_err();
    assert_eq!(duplicate.to_string(), "Ein Arbeitgeber mit diesem Namen existiert bereits.");

    let mut changed = created.data.clone();
    changed.address.city = "Ingolstadt".into();
    let updated = db.update_employer(created.id, &changed).await.unwrap();
    assert_eq!(updated.data.address.city, "Ingolstadt");

    let staff = db.create_employee(&employee(created.id, &unique("P-"))).await.unwrap();
    let blocked = db.delete_employer(created.id).await.unwrap_err();
    assert!(blocked.to_string().contains("Mitarbeiter zugeordnet"), "{blocked}");

    db.delete_employee(staff.id).await.unwrap();
    db.delete_employer(created.id).await.unwrap();
    assert!(matches!(db.delete_employer(created.id).await, Err(DataError::NotFound)));
}

#[tokio::test]
async fn employee_roundtrip_keeps_every_field() {
    let Some(db) = connect(ALLOWED_USER).await else { return };
    let boss = db.create_employer(&employer(&unique("AG "))).await.unwrap();
    let number = unique("P-");
    let data = employee(boss.id, &number);
    let created = db.create_employee(&data).await.unwrap();
    assert_eq!(created.data, data);
    let listed = db.list_employees().await.unwrap().into_iter().find(|e| e.id == created.id).unwrap();
    assert_eq!(listed.data, data);

    let duplicate = db.create_employee(&employee(boss.id, &number)).await.unwrap_err();
    assert_eq!(duplicate.to_string(), "Diese Personalnummer ist bereits vergeben.");

    let mut changed = data.clone();
    changed.employment_end = NaiveDate::from_ymd_opt(2026, 6, 30);
    changed.gender = None;
    changed.transition_zone = None;
    changed.monthly_salary = None;
    let updated = db.update_employee(created.id, &changed).await.unwrap();
    assert_eq!(updated.data, changed);

    // The database rejects what the form would reject, too.
    let mut invalid = changed.clone();
    invalid.social_security_number = "12 150388 W 042".into();
    assert!(matches!(db.update_employee(created.id, &invalid).await, Err(DataError::Invalid(_))));

    db.delete_employee(created.id).await.unwrap();
    db.delete_employer(boss.id).await.unwrap();
}

#[tokio::test]
async fn payroll_amounts_match_core_calculation() {
    let Some(db) = connect(ALLOWED_USER).await else { return };
    let boss = db.create_employer(&employer(&unique("AG "))).await.unwrap();
    let person = db.create_employee(&employee(boss.id, &unique("P-"))).await.unwrap();

    // Includes tie cases (x.xx5) where rounding direction matters.
    let cases = [
        (dec!(520.00), dec!(37.50)),
        (dec!(538.50), dec!(0)),
        (dec!(0.05), dec!(0)),
        (dec!(603.00), dec!(129.99)),
        (dec!(1234.56), dec!(0.01)),
        (dec!(0), dec!(0)),
    ];
    for (month, (base_pay, extra_pay)) in (1..).zip(cases) {
        let entry = PayrollEntry {
            employee_id: person.id,
            period: Period::new(2025, month).unwrap(),
            statement_date: Period::new(2025, month).unwrap().last_day(),
            base_pay,
            extra_pay,
            payout: base_pay + extra_pay,
            rates: rates(),
        };
        let stored = db.save_payroll(&entry).await.unwrap();
        assert_eq!(stored.entry, entry);
        assert_eq!(stored.calculation, calculate(base_pay, extra_pay, &rates()), "month {month}");
    }

    // Saving the same month again replaces it instead of creating a duplicate.
    let march = Period::new(2025, 3).unwrap();
    let mut entry = db.payroll_record(person.id, march).await.unwrap().unwrap().entry;
    entry.base_pay = dec!(100);
    entry.payout = dec!(96.40);
    let replaced = db.save_payroll(&entry).await.unwrap();
    assert_eq!(replaced.calculation.gross_total, dec!(100));
    assert_eq!(replaced.deductions(), dec!(3.60));
    let year = db.payroll_records_of_year(person.id, 2025).await.unwrap();
    assert_eq!(year.len(), cases.len());
    assert!(year.windows(2).all(|w| w[0].entry.period < w[1].entry.period));

    // Payout above gross is refused by the database.
    entry.payout = dec!(100.01);
    assert!(matches!(db.save_payroll(&entry).await, Err(DataError::Invalid(_))));

    assert!(db.payroll_record(person.id, Period::new(2024, 1).unwrap()).await.unwrap().is_none());
    db.delete_payroll_record(replaced.id).await.unwrap();
    assert!(db.payroll_record(person.id, march).await.unwrap().is_none());

    db.delete_employee(person.id).await.unwrap();
    db.delete_employer(boss.id).await.unwrap();
}

#[tokio::test]
async fn time_records_overview_and_cascade() {
    let Some(db) = connect(ALLOWED_USER).await else { return };
    let boss = db.create_employer(&employer(&unique("AG "))).await.unwrap();
    let person = db.create_employee(&employee(boss.id, &unique("P-"))).await.unwrap();

    let entry = TimeEntry {
        employee_id: person.id,
        period: Period::new(2026, 2).unwrap(),
        hours_worked: dec!(41.5),
        extra_hours: dec!(3.25),
        vacation_days: dec!(1.5),
        sick_days: Decimal::ZERO,
    };
    let stored = db.save_time(&entry).await.unwrap();
    assert_eq!(stored.entry, entry);
    let mut changed = entry.clone();
    changed.sick_days = dec!(2);
    assert_eq!(db.save_time(&changed).await.unwrap().id, stored.id, "same month must be updated in place");
    assert_eq!(db.time_records_of_year(person.id, 2026).await.unwrap().len(), 1);

    let payroll = PayrollEntry {
        employee_id: person.id,
        period: Period::new(2026, 3).unwrap(),
        statement_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        base_pay: dec!(538),
        extra_pay: dec!(0),
        payout: dec!(538),
        rates: rates(),
    };
    db.save_payroll(&payroll).await.unwrap();

    let months = db.recorded_months(2026).await.unwrap();
    assert!(months.has_time(person.id, 2) && !months.has_time(person.id, 3));
    assert!(months.has_payroll(person.id, 3) && !months.has_payroll(person.id, 2));

    let counts = db.count_employee_records(person.id).await.unwrap();
    assert_eq!((counts.payroll_months, counts.time_months), (1, 1));

    db.delete_employee(person.id).await.unwrap();
    let months = db.recorded_months(2026).await.unwrap();
    assert!(
        !months.has_time(person.id, 2) && !months.has_payroll(person.id, 3),
        "records must be deleted with the employee"
    );
    db.delete_employer(boss.id).await.unwrap();
}

#[tokio::test]
async fn shared_defaults() {
    let Some(db) = connect(ALLOWED_USER).await else { return };
    let original = db.defaults().await.unwrap();
    let changed = Defaults { rates: rates(), health_insurer: "Knappschaft".into() };
    assert_eq!(db.save_defaults(&changed).await.unwrap(), changed);
    assert_eq!(db.defaults().await.unwrap(), changed);
    db.save_defaults(&original).await.unwrap();
}
