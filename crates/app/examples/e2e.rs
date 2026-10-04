//! End-to-end check of the real app against the local Supabase stand-in:
//! drives the UI through the same callbacks a click would trigger and checks
//! the results (database rows, error messages, PDF files).
//!
//!     scripts/dev-db.sh start && eval "$(scripts/dev-db.sh env)"
//!     cargo run -p cashflow-app --example e2e

use cashflow_app::testing::{self, App, Page};
use cashflow_app::ui::*;
use cashflow_data::{Database, Session, User};
use chrono::Utc;
use slint::{ComponentHandle, Model, ModelRc, SharedString};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

type Step = Box<dyn Fn(&AppWindow, &Rc<App>, &mut Ctx) -> Result<(), String>>;

/// Values remembered between steps.
#[derive(Default)]
struct Ctx {
    employer_name: String,
    personnel_number: String,
    employer_id: String,
}

fn main() {
    let url = std::env::var("CASHFLOW_TEST_REST_URL").expect("run: eval \"$(scripts/dev-db.sh env)\"");
    let secret = std::env::var("CASHFLOW_TEST_JWT_SECRET").expect("CASHFLOW_TEST_JWT_SECRET");
    slint::platform::set_platform(Box::new(i_slint_backend_testing::TestingBackend::new(
        i_slint_backend_testing::TestingBackendOptions { mock_time: false, threading: true, renderer_name: None },
    )))
    .unwrap();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = Arc::new(Database::for_local_postgrest(&url).unwrap());
    runtime.block_on(db.set_session_for_tests(session(&secret)));
    let _ = std::fs::remove_dir_all(testing::pdf_dir());

    let window = AppWindow::new().unwrap();
    window.show().unwrap();
    let app = testing::start(&window, runtime.handle().clone(), db);
    let suffix = &uuid::Uuid::new_v4().simple().to_string()[..6];
    let ctx = Rc::new(RefCell::new(Ctx {
        employer_name: format!("E2E Arbeitgeber {suffix}"),
        personnel_number: format!("E2E-{suffix}"),
        ..Ctx::default()
    }));

    let steps: Vec<(&str, Step)> = vec![
        (
            "Arbeitgeber anlegen",
            Box::new(|w, app, c| {
                testing::navigate(app, Page::Employers);
                let page = w.global::<EmployersPage>();
                page.invoke_create();
                page.invoke_edited("name".into(), c.employer_name.as_str().into());
                page.invoke_edited("postal_code".into(), "1234".into()); // invalid on purpose
                page.invoke_save();
                expect(
                    page.get_error().contains("Postleitzahl") || form_error(&page.get_form(), "postal_code").is_some(),
                    "PLZ-Fehler wird angezeigt",
                )?;
                page.invoke_edited("postal_code".into(), "93336".into());
                page.invoke_save();
                Ok(())
            }),
        ),
        (
            "Arbeitgeber gespeichert",
            Box::new(|w, _, c| {
                let page = w.global::<EmployersPage>();
                expect(page.get_error().is_empty(), &format!("kein Fehler (war: {})", page.get_error()))?;
                let item = find(&page.get_items(), &c.employer_name).ok_or("Arbeitgeber fehlt in der Liste")?;
                expect(page.get_selected_id() == item.id, "neuer Arbeitgeber ist ausgewählt")?;
                c.employer_id = item.id.to_string();
                Ok(())
            }),
        ),
        (
            "Mitarbeiter mit falscher SV-Nummer",
            Box::new(|w, app, c| {
                testing::navigate(app, Page::Employees);
                let page = w.global::<EmployeesPage>();
                page.invoke_create();
                for (key, value) in [
                    ("employer_id", c.employer_id.as_str()),
                    ("personnel_number", c.personnel_number.as_str()),
                    ("first_name", "Erika"),
                    ("last_name", "Mustermann"),
                    ("social_security_number", "12 150388 W 043"),
                    ("monthly_salary", "538,00"),
                    ("hourly_rate", "13,50"),
                ] {
                    page.invoke_edited(key.into(), value.into());
                }
                page.invoke_save();
                let error = form_error(&page.get_form(), "social_security_number").unwrap_or_default();
                expect(error.contains("Prüfziffer"), &format!("SV-Prüfziffer-Fehler (war: „{error}“)"))?;
                page.invoke_edited("social_security_number".into(), "12 150388 W 042".into());
                page.invoke_save();
                Ok(())
            }),
        ),
        (
            "Mitarbeiter gespeichert",
            Box::new(|w, _, c| {
                let page = w.global::<EmployeesPage>();
                expect(page.get_error().is_empty(), &format!("kein Fehler (war: {})", page.get_error()))?;
                expect(
                    page.get_items().iter().any(|i| i.subtitle.contains(c.personnel_number.as_str())),
                    "Mitarbeiter in der Liste",
                )?;
                expect(page.get_title() == "Erika Mustermann", "Titel zeigt den Namen")?;
                Ok(())
            }),
        ),
        (
            "Lohnerfassung öffnen",
            Box::new(|_, app, _| {
                testing::navigate(app, Page::Payroll);
                Ok(())
            }),
        ),
        (
            "März 2025 wählen",
            Box::new(|w, _, c| {
                let page = w.global::<PayrollPage>();
                let years = page.get_years();
                let year =
                    (0..years.row_count()).find(|i| years.row_data(*i).unwrap() == "2025").ok_or("Jahr 2025 fehlt")?;
                page.invoke_period_changed(2, year as i32); // März 2025
                let _ = c;
                Ok(())
            }),
        ),
        (
            "Mitarbeiter wählen",
            Box::new(|w, _, c| {
                let page = w.global::<PayrollPage>();
                let index = index_of(&page.get_employees(), &c.personnel_number)
                    .ok_or("Mitarbeiter fehlt in der Lohnerfassung")?;
                page.invoke_employee_changed(index);
                Ok(())
            }),
        ),
        (
            "Lohn speichern",
            Box::new(|w, _, _| {
                let page = w.global::<PayrollPage>();
                expect(page.get_ready(), "Monat geladen")?;
                expect(!page.get_exists(), "März 2025 ist noch leer")?;
                expect(
                    field_value(&page.get_form(), "base_pay").as_deref() == Some("538,00"),
                    "Brutto aus Stammdaten vorgeschlagen",
                )?;
                page.invoke_edited("extra_pay".into(), "27".into());
                page.invoke_edited("payout".into(), "600".into()); // higher than gross: must be refused
                page.invoke_save();
                expect(
                    form_error(&page.get_form(), "payout").is_some_and(|e| e.contains("höher")),
                    "Auszahlung > Brutto wird abgelehnt",
                )?;
                page.invoke_edited("payout".into(), "".into());
                page.invoke_save_and_print();
                Ok(())
            }),
        ),
        (
            "Lohn gespeichert und PDF erstellt",
            Box::new(|w, _, c| {
                let page = w.global::<PayrollPage>();
                expect(page.get_exists(), "Lohn ist gespeichert")?;
                let total =
                    page.get_calculation().iter().find(|l| l.key == "Abgaben Arbeitgeber").map(|l| l.value.to_string());
                // 565,00 € × 31,17 %, each contribution rounded: 73,45 + 84,75 + 4,52 + 1,24 + 0,85 + 11,30 = 176,11 €
                expect(total.as_deref() == Some("176,11 €"), &format!("Abgaben 176,11 € (war {total:?})"))?;
                let pdf = testing::pdf_dir()
                    .join(format!("{} Mustermann", c.personnel_number))
                    .join(format!("2025-03-31 {} Mustermann Lohnabrechnung.pdf", c.personnel_number));
                expect(pdf.exists(), &format!("PDF vorhanden: {}", pdf.display()))?;
                Ok(())
            }),
        ),
        (
            "Zeiterfassung öffnen",
            Box::new(|_, app, _| {
                testing::navigate(app, Page::Time);
                Ok(())
            }),
        ),
        (
            "Zeit erfassen (gleicher Mitarbeiter und Monat)",
            Box::new(|w, _, c| {
                let page = w.global::<TimePage>();
                expect(page.get_ready(), "Zeitmonat geladen")?;
                let selected = page.get_employees().row_data(page.get_employee_index() as usize).unwrap_or_default();
                expect(selected.contains(&c.personnel_number), &format!("Mitarbeiter übernommen (war {selected})"))?;
                expect(page.get_month_index() == 2, "März übernommen")?;
                page.invoke_edited("hours_worked".into(), "39,5".into());
                page.invoke_edited("extra_hours".into(), "2".into());
                page.invoke_save();
                Ok(())
            }),
        ),
        (
            "Zeit gespeichert, Lohnjournal erstellen",
            Box::new(|w, app, c| {
                let page = w.global::<TimePage>();
                expect(page.get_exists(), "Zeiten gespeichert")?;
                testing::navigate(app, Page::Documents);
                let docs = w.global::<DocumentsPage>();
                let index = index_of(&docs.get_journal_employees(), &c.personnel_number)
                    .ok_or("Mitarbeiter fehlt bei den Journalen")?;
                docs.set_journal_employee(index);
                let years = docs.get_years();
                let year = (0..years.row_count()).find(|i| years.row_data(*i).unwrap() == "2025").unwrap();
                docs.set_journal_year(year as i32);
                docs.invoke_create_payroll_journal();
                Ok(())
            }),
        ),
        (
            "Journal vorhanden, Mitarbeiter löschen",
            Box::new(|w, app, c| {
                let pdf = testing::pdf_dir()
                    .join(format!("{} Mustermann", c.personnel_number))
                    .join(format!("2025-12-31 {} Mustermann Lohnjournal.pdf", c.personnel_number));
                expect(pdf.exists(), "Lohnjournal vorhanden")?;
                testing::navigate(app, Page::Employees);
                w.global::<EmployeesPage>().invoke_delete();
                Ok(())
            }),
        ),
        (
            "Löschen bestätigen",
            Box::new(|w, _, _| {
                let confirm = w.global::<Confirm>();
                expect(confirm.get_open(), "Bestätigung wird angezeigt")?;
                expect(
                    confirm.get_message().contains("1 Lohn- und 1 Zeitmonaten"),
                    &format!("Zählt die Monate: {}", confirm.get_message()),
                )?;
                confirm.invoke_accepted();
                Ok(())
            }),
        ),
        (
            "Mitarbeiter weg, Arbeitgeber löschen",
            Box::new(|w, app, c| {
                let still_listed = w
                    .global::<EmployeesPage>()
                    .get_items()
                    .iter()
                    .any(|i| i.subtitle.contains(c.personnel_number.as_str()));
                expect(!still_listed, "Mitarbeiter gelöscht")?;
                testing::navigate(app, Page::Employers);
                let page = w.global::<EmployersPage>();
                page.invoke_select(c.employer_id.as_str().into());
                page.invoke_delete();
                w.global::<Confirm>().invoke_accepted();
                Ok(())
            }),
        ),
        (
            "Arbeitgeber weg",
            Box::new(|w, _, c| {
                expect(
                    find(&w.global::<EmployersPage>().get_items(), &c.employer_name).is_none(),
                    "Arbeitgeber gelöscht",
                )?;
                Ok(())
            }),
        ),
    ];

    let index = Rc::new(Cell::new(0));
    let failed = Rc::new(Cell::new(false));
    let timer = slint::Timer::default();
    let weak = window.as_weak();
    let app_ref = app.clone();
    let failed_ref = failed.clone();
    timer.start(slint::TimerMode::Repeated, Duration::from_millis(800), move || {
        let window = weak.upgrade().unwrap();
        let Some((name, step)) = steps.get(index.get()) else {
            let _ = slint::quit_event_loop();
            return;
        };
        index.set(index.get() + 1);
        match step(&window, &app_ref, &mut ctx.borrow_mut()) {
            Ok(()) => println!("✓ {name}"),
            Err(error) => {
                println!("✗ {name}: {error}");
                let notices: Vec<String> =
                    window.global::<Notices>().get_items().iter().map(|n| n.text.to_string()).collect();
                println!("  Meldungen: {notices:?}");
                failed_ref.set(true);
                let _ = slint::quit_event_loop();
            }
        }
    });
    slint::run_event_loop().unwrap();
    drop(app);
    if failed.get() {
        std::process::exit(1);
    }
    println!("E2E OK");
}

fn expect(condition: bool, what: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(what.to_string()) }
}

fn find(items: &ModelRc<ListItem>, title: &str) -> Option<ListItem> {
    items.iter().find(|item| item.title == title)
}

fn index_of(labels: &ModelRc<SharedString>, needle: &str) -> Option<i32> {
    labels.iter().position(|label| label.contains(needle)).map(|i| i as i32)
}

fn field(form: &ModelRc<FormSection>, key: &str) -> Option<FormField> {
    form.iter().flat_map(|section| section.fields.iter().collect::<Vec<_>>()).find(|f| f.key == key)
}

fn field_value(form: &ModelRc<FormSection>, key: &str) -> Option<String> {
    field(form, key).map(|f| f.value.to_string())
}

fn form_error(form: &ModelRc<FormSection>, key: &str) -> Option<String> {
    field(form, key).map(|f| f.error.to_string()).filter(|e| !e.is_empty())
}

fn session(secret: &str) -> Session {
    #[derive(serde::Serialize)]
    struct Claims<'a> {
        sub: &'a str,
        role: &'a str,
        exp: i64,
    }
    let user = "00000000-0000-0000-0000-00000000a11d";
    let expires_at = Utc::now() + chrono::Duration::hours(1);
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims { sub: user, role: "authenticated", exp: expires_at.timestamp() },
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Session {
        access_token: token,
        refresh_token: "-".into(),
        expires_at,
        user: User { id: user.parse().unwrap(), email: None },
    }
}
