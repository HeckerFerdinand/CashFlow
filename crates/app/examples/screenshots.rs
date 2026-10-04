//! Renders every page of the real app headlessly, at several window sizes,
//! into `target/screenshots/`. Uses the local Supabase stand-in and fills it
//! with demo data (only if it is empty).
//!
//!     scripts/dev-db.sh reset && eval "$(scripts/dev-db.sh env)"
//!     cargo run -p cashflow-app --example screenshots [page …]

use cashflow_app::testing::{self, Page};
use cashflow_app::ui::AppWindow;
use cashflow_core::payroll::calculate;
use cashflow_core::*;
use cashflow_data::{Database, Session, User};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

const SIZES: [(u32, u32, &str); 3] = [(1280, 800, "desktop"), (900, 620, "small"), (1920, 1080, "large")];

fn main() {
    let url = std::env::var("CASHFLOW_TEST_REST_URL").expect("run: eval \"$(scripts/dev-db.sh env)\"");
    let secret = std::env::var("CASHFLOW_TEST_JWT_SECRET").expect("CASHFLOW_TEST_JWT_SECRET");
    let filter: Vec<String> = std::env::args().skip(1).collect();

    slint::platform::set_platform(Box::new(i_slint_backend_testing::TestingBackend::new(
        i_slint_backend_testing::TestingBackendOptions {
            mock_time: false,
            threading: true,
            renderer_name: Some("software".into()),
        },
    )))
    .expect("platform");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let db = Arc::new(Database::for_local_postgrest(&url).unwrap());
    runtime.block_on(async {
        db.set_session_for_tests(test_session(&secret)).await;
        seed_demo_data(&db).await;
    });

    let window = AppWindow::new().unwrap();
    window.window().set_size(slint::LogicalSize::new(1280.0, 800.0));
    window.show().unwrap();

    if filter.iter().any(|f| f == "login") {
        // The login screen needs no data: render it directly.
        let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/screenshots");
        std::fs::create_dir_all(&out_dir).unwrap();
        window.global::<cashflow_app::ui::LoginPage>().set_show_connection(true);
        window.global::<cashflow_app::ui::LoginPage>().set_error("E-Mail-Adresse oder Passwort ist falsch.".into());
        for (w, h, size) in SIZES {
            window.window().set_size(slint::LogicalSize::new(w as f32, h as f32));
            let shot = window.window().take_snapshot().unwrap();
            let file = out_dir.join(format!("login-{size}.png"));
            image::save_buffer(&file, shot.as_bytes(), shot.width(), shot.height(), image::ExtendedColorType::Rgba8)
                .unwrap();
            println!("{}", file.display());
        }
        return;
    }
    let app = testing::start(&window, runtime.handle().clone(), db);

    // (file name, page, action run after navigating)
    let pages: Vec<(&str, Page, Action)> = vec![
        ("overview", Page::Overview, None),
        ("payroll", Page::Payroll, None),
        ("time", Page::Time, None),
        ("documents", Page::Documents, None),
        ("employees", Page::Employees, None),
        ("employees-edit", Page::Employees, Some(open_first_employee)),
        ("employers", Page::Employers, None),
        ("employers-edit", Page::Employers, Some(open_first_employer)),
        ("settings", Page::Settings, None),
        ("about", Page::About, None),
    ];
    let steps: Vec<(Page, &str, Action)> = pages
        .into_iter()
        .filter(|(name, _, _)| filter.is_empty() || filter.iter().any(|f| name.starts_with(f.as_str())))
        .map(|(name, page, action)| (page, name, action))
        .collect();

    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/screenshots");
    std::fs::create_dir_all(&out_dir).unwrap();

    // Walk through the pages with a timer so that the background requests
    // (real HTTP calls to the local database) can finish between steps.
    let queue = Rc::new(RefCell::new(
        steps
            .into_iter()
            .flat_map(|(page, name, action)| SIZES.map(|size| (page, name, action, size)))
            .collect::<Vec<_>>(),
    ));
    let weak = window.as_weak();
    let app_ref = app.clone();
    let timer = slint::Timer::default();
    let pending: Rc<RefCell<Option<Step>>> = Rc::new(RefCell::new(None));
    timer.start(slint::TimerMode::Repeated, Duration::from_millis(700), move || {
        let window = weak.upgrade().unwrap();
        // 1. Take the screenshot of the previously prepared step.
        if let Some((_, name, _, (_, _, size_name))) = pending.borrow_mut().take() {
            let shot = window.window().take_snapshot().expect("snapshot");
            let file = out_dir.join(format!("{name}-{size_name}.png"));
            image::save_buffer(&file, shot.as_bytes(), shot.width(), shot.height(), image::ExtendedColorType::Rgba8)
                .unwrap();
            println!("{}", file.display());
        }
        // 2. Prepare the next step.
        let next = queue.borrow_mut().first().copied();
        match next {
            Some(step) => {
                queue.borrow_mut().remove(0);
                let (page, _, action, (w, h, _)) = step;
                window.window().set_size(slint::LogicalSize::new(w as f32, h as f32));
                testing::navigate(&app_ref, page);
                if let Some(action) = action {
                    action(&window);
                }
                *pending.borrow_mut() = Some(step);
            }
            None => {
                let _ = slint::quit_event_loop();
            }
        }
    });
    slint::run_event_loop().unwrap();
    drop(app);
}

type Action = Option<fn(&AppWindow)>;
/// One screenshot: page, file name, preparation and window size.
type Step = (Page, &'static str, Action, (u32, u32, &'static str));

fn open_first_employer(window: &AppWindow) {
    use cashflow_app::ui::EmployersPage;
    use slint::Model;
    let page = window.global::<EmployersPage>();
    if let Some(item) = page.get_items().row_data(0) {
        page.invoke_select(item.id);
    }
}

fn open_first_employee(window: &AppWindow) {
    use cashflow_app::ui::EmployeesPage;
    use slint::Model;
    let page = window.global::<EmployeesPage>();
    if let Some(item) = page.get_items().row_data(0) {
        page.invoke_select(item.id);
    }
}

fn test_session(secret: &str) -> Session {
    #[derive(serde::Serialize)]
    struct Claims<'a> {
        sub: &'a str,
        role: &'a str,
        exp: i64,
    }
    let user = "00000000-0000-0000-0000-00000000a11d";
    let expires_at = Utc::now() + chrono::Duration::hours(2);
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

/// Fills an empty database with two employers, four employees and records.
async fn seed_demo_data(db: &Database) {
    if !db.list_employers().await.unwrap().is_empty() {
        return;
    }
    let defaults = db.defaults().await.unwrap();
    let main = db
        .create_employer(&EmployerData {
            name: "Hausverwaltung Hecker".into(),
            representative: "vertreten durch Ferdinand Hecker".into(),
            company_number: "12345678".into(),
            tax_number: "124/116/40017".into(),
            address: Address {
                street: "Schulstraße".into(),
                house_number: "34".into(),
                postal_code: "93336".into(),
                city: "Altmannstein".into(),
            },
        })
        .await
        .unwrap();
    let second = db
        .create_employer(&EmployerData {
            name: "WEG Lindenallee 12".into(),
            address: Address {
                street: "Lindenallee".into(),
                house_number: "12".into(),
                postal_code: "85049".into(),
                city: "Ingolstadt".into(),
            },
            ..Default::default()
        })
        .await
        .unwrap();
    let people = [
        ("1001", "Anna", "Weber", main.id, "Reinigungskraft", dec!(538), Some(dec!(13.50)), None),
        ("1002", "Tobias", "Richter", main.id, "Hausmeister", dec!(520), Some(dec!(15)), None),
        ("1003", "Lena", "Fischer", second.id, "Gartenpflege", dec!(450), None, None),
        ("1004", "Max", "Bauer", second.id, "Winterdienst", dec!(300), None, NaiveDate::from_ymd_opt(2026, 3, 31)),
    ];
    for (number, first, last, employer_id, occupation, salary, rate, end) in people {
        let employee = db
            .create_employee(&EmployeeData {
                employer_id,
                personnel_number: number.into(),
                first_name: first.into(),
                last_name: last.into(),
                birth_name: String::new(),
                address: Address {
                    street: "Hauptstraße".into(),
                    house_number: "5".into(),
                    postal_code: "93336".into(),
                    city: "Altmannstein".into(),
                },
                birth_date: NaiveDate::from_ymd_opt(1980, 5, 17),
                gender: None,
                nationality: "deutsch".into(),
                social_security_number: String::new(),
                tax_id: "86095742719".into(),
                activity_key: String::new(),
                contribution_group_key: "6500".into(),
                person_group: "109".into(),
                transition_zone: Some(TransitionZone::No),
                occupation: occupation.into(),
                health_insurer: defaults.health_insurer.clone(),
                employment_start: NaiveDate::from_ymd_opt(2024, 1, 1),
                employment_end: end,
                monthly_salary: Some(salary),
                hourly_rate: rate,
                rates: defaults.rates,
            })
            .await
            .unwrap();
        // Wage and time records for most months of 2025 and the first months of 2026.
        for (year, months) in [(2025, 1..=12u32), (2026, 1..=8u32)] {
            for month in months {
                if (month + number.len() as u32).is_multiple_of(5) {
                    continue; // leave some gaps for the overview
                }
                let period = Period::new(year, month).unwrap();
                let extra = if rate.is_some() { dec!(27) } else { Decimal::ZERO };
                let gross = salary + extra;
                let _ = calculate(salary, extra, &defaults.rates);
                db.save_payroll(&PayrollEntry {
                    employee_id: employee.id,
                    period,
                    statement_date: period.last_day(),
                    base_pay: salary,
                    extra_pay: extra,
                    payout: gross,
                    rates: defaults.rates,
                })
                .await
                .unwrap();
                if month % 4 != 0 {
                    db.save_time(&TimeEntry {
                        employee_id: employee.id,
                        period,
                        hours_worked: dec!(39.5),
                        extra_hours: if rate.is_some() { dec!(2) } else { Decimal::ZERO },
                        vacation_days: if month == 8 { dec!(5) } else { Decimal::ZERO },
                        sick_days: Decimal::ZERO,
                    })
                    .await
                    .unwrap();
                }
            }
        }
    }
}
