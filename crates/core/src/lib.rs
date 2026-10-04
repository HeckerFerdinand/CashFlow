//! Domain model and business rules of CashFlow.
//!
//! This crate is pure: no network, no files, no UI. Everything here can be
//! unit-tested in isolation and is shared by the data layer, the PDF
//! documents and the desktop app.

pub mod employee;
pub mod employer;
pub mod files;
pub mod money;
pub mod payroll;
pub mod period;
pub mod settings;
pub mod time_record;
pub mod validate;

pub use employee::{Employee, EmployeeData, EmployeeDraft, Gender, TransitionZone};
pub use employer::{Address, Employer, EmployerData, EmployerDraft};
pub use payroll::{Calculation, Contribution, Contributions, PayrollDraft, PayrollEntry, PayrollRecord, Rates};
pub use period::Period;
pub use settings::{Defaults, DefaultsDraft};
pub use time_record::{TimeDraft, TimeEntry, TimeRecord};
pub use validate::{FieldError, ValidationErrors};
