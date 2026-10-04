// Release builds on Windows: no console window next to the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = cashflow_app::run() {
        eprintln!("CashFlow konnte nicht gestartet werden: {error}");
        std::process::exit(1);
    }
}
