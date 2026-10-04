//! Naming of generated files (PDFs) and their folders.

use crate::period::Period;

/// Replaces characters that are not allowed in file names on Windows or
/// macOS and trims trailing dots/spaces (invalid on Windows).
pub fn sanitize_file_name(name: &str) -> String {
    let replaced: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let trimmed = replaced.trim().trim_end_matches(['.', ' ']).to_string();
    if trimmed.is_empty() { "_".into() } else { trimmed }
}

/// "2026-03-31 1001 Weber Lohnabrechnung.pdf"
pub fn payslip_file_name(period: Period, folder_name: &str) -> String {
    sanitize_file_name(&format!("{} {folder_name} Lohnabrechnung.pdf", period.last_day().format("%Y-%m-%d")))
}

/// "2025-12-31 1001 Weber Lohnjournal.pdf"
pub fn payroll_journal_file_name(year: i32, folder_name: &str) -> String {
    sanitize_file_name(&format!("{year}-12-31 {folder_name} Lohnjournal.pdf"))
}

/// "2025-12-31 1001 Weber Zeitjournal.pdf"
pub fn time_journal_file_name(year: i32, folder_name: &str) -> String {
    sanitize_file_name(&format!("{year}-12-31 {folder_name} Zeitjournal.pdf"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_files_like_the_old_app() {
        let feb = Period::new(2024, 2).unwrap();
        assert_eq!(payslip_file_name(feb, "1001 Weber"), "2024-02-29 1001 Weber Lohnabrechnung.pdf");
        assert_eq!(payroll_journal_file_name(2025, "1001 Weber"), "2025-12-31 1001 Weber Lohnjournal.pdf");
        assert_eq!(time_journal_file_name(2025, "1001 Weber"), "2025-12-31 1001 Weber Zeitjournal.pdf");
    }

    #[test]
    fn sanitizes() {
        assert_eq!(sanitize_file_name("a/b:c*?.  "), "a_b_c__");
        assert_eq!(sanitize_file_name("Łukasz Ğül"), "Łukasz Ğül");
        assert_eq!(sanitize_file_name(" .. "), "_");
    }
}
