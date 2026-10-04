//! Writing generated PDFs to the user's folder and opening files/folders.

use crate::context::App;
use cashflow_core::{Employee, Employer};
use cashflow_documents::{DocumentError, Parties};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

thread_local! {
    static LAST_FILE: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

pub fn last_file() -> Option<PathBuf> {
    LAST_FILE.with_borrow(|f| f.clone())
}

/// Writes `bytes` to `<base>/<folder>/<file_name>`. The file is written to a
/// temporary name first and then renamed, so a failure never leaves a broken PDF.
pub fn save_pdf(base: &Path, folder: &str, file_name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    let dir = base.join(folder);
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Der Ordner „{}“ kann nicht angelegt werden: {e}", dir.display()))?;
    let target = dir.join(file_name);
    let temporary = dir.join(format!(".{file_name}.tmp"));
    std::fs::write(&temporary, bytes)
        .map_err(|e| format!("„{}“ kann nicht geschrieben werden: {e}", temporary.display()))?;
    if let Err(error) = std::fs::rename(&temporary, &target) {
        let _ = std::fs::remove_file(&temporary);
        return Err(format!(
            "„{}“ kann nicht gespeichert werden ({error}). Ist die Datei noch in einem PDF-Programm geöffnet?",
            target.display()
        ));
    }
    Ok(target)
}

pub fn open_path(app: &Rc<App>, path: &Path) {
    if let Err(error) = opener::open(path) {
        app.notify_error(format!("„{}“ kann nicht geöffnet werden: {error}", path.display()));
    }
}

pub fn open_pdf_folder(app: &Rc<App>) {
    let dir = app.config.borrow().pdf_dir();
    if let Err(error) = std::fs::create_dir_all(&dir) {
        app.notify_error(format!("Der Ordner „{}“ kann nicht angelegt werden: {error}", dir.display()));
        return;
    }
    open_path(app, &dir);
}

/// Lets the user pick the PDF folder; returns true if it changed.
pub fn choose_pdf_folder(app: &Rc<App>) -> bool {
    let current = app.config.borrow().pdf_dir();
    let picked = rfd::FileDialog::new()
        .set_title("Ablageordner für PDFs wählen")
        .set_directory(if current.exists() { current } else { PathBuf::from(".") })
        .pick_folder();
    let Some(dir) = picked else { return false };
    app.config.borrow_mut().pdf_dir = Some(dir);
    app.save_config();
    true
}

/// What to render; the data is cloned so rendering can run in the background.
pub enum Job {
    Payslip(Box<cashflow_core::PayrollRecord>),
    PayrollJournal(i32, Vec<cashflow_core::PayrollRecord>),
    TimeJournal(i32, Vec<cashflow_core::TimeRecord>),
}

/// Renders one document for one employee and stores it. Runs on a worker thread.
pub fn render_and_save(base: &Path, employer: &Employer, employee: &Employee, job: &Job) -> Result<PathBuf, String> {
    use cashflow_core::files;
    let parties = Parties { employer, employee };
    let folder = employee.data.document_folder_name();
    let today = chrono::Local::now().date_naive();
    let (bytes, file_name) = match job {
        Job::Payslip(record) => {
            (cashflow_documents::payslip(parties, record), files::payslip_file_name(record.entry.period, &folder))
        }
        Job::PayrollJournal(year, records) => (
            cashflow_documents::payroll_journal(parties, *year, records, today),
            files::payroll_journal_file_name(*year, &folder),
        ),
        Job::TimeJournal(year, records) => (
            cashflow_documents::time_journal(parties, *year, records, today),
            files::time_journal_file_name(*year, &folder),
        ),
    };
    let bytes = bytes.map_err(|error: DocumentError| error.to_string())?;
    save_pdf(base, &folder, &file_name, &bytes)
}

/// Creates a document in the background, then reports and (optionally) opens it.
pub fn create(app: &Rc<App>, employee_id: uuid::Uuid, job: Job) {
    let (employer, employee) = {
        let cache = app.cache.borrow();
        let Some(employee) = cache.employee(employee_id).cloned() else {
            app.notify_error("Der Mitarbeiter wurde nicht gefunden.");
            return;
        };
        let Some(employer) = cache.employer(employee.data.employer_id).cloned() else {
            app.notify_error("Der Arbeitgeber des Mitarbeiters wurde nicht gefunden.");
            return;
        };
        (employer, employee)
    };
    let base = app.config.borrow().pdf_dir();
    app.run(
        "PDF wird erstellt …",
        async move {
            tokio::task::spawn_blocking(move || render_and_save(&base, &employer, &employee, &job))
                .await
                .unwrap_or_else(|error| Err(error.to_string()))
        },
        |app, result| match result {
            Ok(path) => finished(app, &[path]),
            Err(error) => app.notify_error(error),
        },
    );
}

/// Reports created files and opens a single one if configured.
pub fn finished(app: &Rc<App>, paths: &[PathBuf]) {
    let Some(last) = paths.last() else { return };
    LAST_FILE.with_borrow_mut(|f| *f = Some(last.clone()));
    let name = last.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    if paths.len() == 1 {
        app.notify_success(format!("„{name}“ wurde gespeichert."));
        if app.config.borrow().open_pdf_after_create {
            open_path(app, last);
        }
    } else {
        app.notify_success(format!("{} PDFs wurden gespeichert.", paths.len()));
    }
    crate::pages::documents::refresh_folder(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_atomically_into_employee_folder() {
        let base = std::env::temp_dir().join(format!("cashflow-test-{}", uuid::Uuid::new_v4()));
        let path = save_pdf(&base, "1001 Weber", "a.pdf", b"%PDF-1.7").unwrap();
        assert_eq!(path, base.join("1001 Weber").join("a.pdf"));
        assert_eq!(std::fs::read(&path).unwrap(), b"%PDF-1.7");
        // Overwriting works and leaves no temporary file behind.
        save_pdf(&base, "1001 Weber", "a.pdf", b"%PDF-2").unwrap();
        let names: Vec<_> =
            std::fs::read_dir(base.join("1001 Weber")).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(names.len(), 1);
        std::fs::remove_dir_all(base).unwrap();
    }
}
