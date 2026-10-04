//! Contribution rates and the monthly wage calculation (Lohnerfassung).
//!
//! CashFlow handles Minijob payroll: the employer pays flat-rate
//! contributions (Krankenversicherung, Rentenversicherung, Umlagen U1/U2,
//! Insolvenzgeldumlage and the flat-rate tax), each a percentage of the
//! month's total gross pay. The employee's payout is entered by the user.
//!
//! The formula is mirrored by the generated columns of `payroll_records`
//! in `supabase/schema.sql` — both must stay identical.

use crate::money::round_cents;
use crate::period::Period;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

/// The six flat-rate contributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Contribution {
    Health,
    Pension,
    U1,
    U2,
    Insolvency,
    FlatTax,
}

impl Contribution {
    pub const ALL: [Contribution; 6] = [
        Contribution::Health,
        Contribution::Pension,
        Contribution::U1,
        Contribution::U2,
        Contribution::Insolvency,
        Contribution::FlatTax,
    ];

    /// Label of the rate input, e.g. "Krankenversicherung".
    pub fn label(self) -> &'static str {
        match self {
            Contribution::Health => "Krankenversicherung",
            Contribution::Pension => "Rentenversicherung",
            Contribution::U1 => "Umlage U1",
            Contribution::U2 => "Umlage U2",
            Contribution::Insolvency => "Insolvenzgeldumlage",
            Contribution::FlatTax => "Pauschalsteuer",
        }
    }

    /// Label of the amount row in the yearly wage journal.
    pub fn journal_label(self) -> &'static str {
        match self {
            Contribution::Health => "KV-Beitrag",
            Contribution::Pension => "RV-Beitrag",
            Contribution::U1 => "Umlage U1",
            Contribution::U2 => "Umlage U2",
            Contribution::Insolvency => "Umlage Insolvenz",
            Contribution::FlatTax => "Pauschalsteuer ST",
        }
    }

    /// Field key used by forms and the settings ("rate_health", …).
    pub fn key(self) -> &'static str {
        match self {
            Contribution::Health => "rate_health",
            Contribution::Pension => "rate_pension",
            Contribution::U1 => "rate_u1",
            Contribution::U2 => "rate_u2",
            Contribution::Insolvency => "rate_insolvency",
            Contribution::FlatTax => "rate_flat_tax",
        }
    }
}

/// Contribution rates in percent (13 = 13 %).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rates {
    pub health: Decimal,
    pub pension: Decimal,
    pub u1: Decimal,
    pub u2: Decimal,
    pub insolvency: Decimal,
    pub flat_tax: Decimal,
}

impl Rates {
    pub fn get(&self, contribution: Contribution) -> Decimal {
        match contribution {
            Contribution::Health => self.health,
            Contribution::Pension => self.pension,
            Contribution::U1 => self.u1,
            Contribution::U2 => self.u2,
            Contribution::Insolvency => self.insolvency,
            Contribution::FlatTax => self.flat_tax,
        }
    }

    pub fn set(&mut self, contribution: Contribution, value: Decimal) {
        match contribution {
            Contribution::Health => self.health = value,
            Contribution::Pension => self.pension = value,
            Contribution::U1 => self.u1 = value,
            Contribution::U2 => self.u2 = value,
            Contribution::Insolvency => self.insolvency = value,
            Contribution::FlatTax => self.flat_tax = value,
        }
    }

    pub fn total(&self) -> Decimal {
        Contribution::ALL.iter().map(|c| self.get(*c)).sum()
    }
}

/// Amounts per contribution for one month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Contributions {
    pub health: Decimal,
    pub pension: Decimal,
    pub u1: Decimal,
    pub u2: Decimal,
    pub insolvency: Decimal,
    pub flat_tax: Decimal,
}

impl Contributions {
    pub fn get(&self, contribution: Contribution) -> Decimal {
        match contribution {
            Contribution::Health => self.health,
            Contribution::Pension => self.pension,
            Contribution::U1 => self.u1,
            Contribution::U2 => self.u2,
            Contribution::Insolvency => self.insolvency,
            Contribution::FlatTax => self.flat_tax,
        }
    }

    /// Sum of the (already rounded) amounts.
    pub fn total(&self) -> Decimal {
        Contribution::ALL.iter().map(|c| self.get(*c)).sum()
    }
}

/// Gross pay and contributions of one month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Calculation {
    pub gross_total: Decimal,
    pub contributions: Contributions,
    pub total_contribution: Decimal,
}

/// Computes the contributions: each amount is `gross × rate / 100`, rounded
/// to cents (half away from zero); the total is the sum of the rounded amounts.
pub fn calculate(base_pay: Decimal, extra_pay: Decimal, rates: &Rates) -> Calculation {
    let gross_total = base_pay + extra_pay;
    let amount = |rate: Decimal| round_cents(gross_total * rate / Decimal::ONE_HUNDRED);
    let contributions = Contributions {
        health: amount(rates.health),
        pension: amount(rates.pension),
        u1: amount(rates.u1),
        u2: amount(rates.u2),
        insolvency: amount(rates.insolvency),
        flat_tax: amount(rates.flat_tax),
    };
    Calculation { gross_total, contributions, total_contribution: contributions.total() }
}

/// What is needed to store a month's wage record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayrollEntry {
    pub employee_id: Uuid,
    pub period: Period,
    /// Date printed on the payslip (Abrechnungsdatum).
    pub statement_date: NaiveDate,
    /// Brutto-Bezüge
    pub base_pay: Decimal,
    /// Sonstige Brutto-Bezüge (Vergütung Regiestunden)
    pub extra_pay: Decimal,
    /// Auszahlungsbetrag
    pub payout: Decimal,
    /// Snapshot of the employee's rates at the time of recording.
    pub rates: Rates,
}

impl PayrollEntry {
    pub fn calculation(&self) -> Calculation {
        calculate(self.base_pay, self.extra_pay, &self.rates)
    }
}

/// A stored wage record, including the amounts computed by the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayrollRecord {
    pub id: Uuid,
    pub entry: PayrollEntry,
    pub calculation: Calculation,
}

impl PayrollRecord {
    /// Deductions between gross and payout (0 for a typical Minijob).
    pub fn deductions(&self) -> Decimal {
        (self.calculation.gross_total - self.entry.payout).max(Decimal::ZERO)
    }
}

/// Raw form input of the wage entry. Keys: `statement_date`, `base_pay`,
/// `extra_pay`, `payout`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PayrollDraft {
    pub statement_date: String,
    pub base_pay: String,
    pub extra_pay: String,
    pub payout: String,
}

impl PayrollDraft {
    pub fn get(&self, key: &str) -> Option<&str> {
        Some(match key {
            "statement_date" => &self.statement_date,
            "base_pay" => &self.base_pay,
            "extra_pay" => &self.extra_pay,
            "payout" => &self.payout,
            _ => return None,
        })
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) -> bool {
        let slot = match key {
            "statement_date" => &mut self.statement_date,
            "base_pay" => &mut self.base_pay,
            "extra_pay" => &mut self.extra_pay,
            "payout" => &mut self.payout,
            _ => return false,
        };
        *slot = value.into();
        true
    }

    /// Fills the form from a stored entry.
    pub fn from_entry(entry: &PayrollEntry) -> Self {
        Self {
            statement_date: crate::validate::format_date(entry.statement_date),
            base_pay: crate::money::format_amount(entry.base_pay),
            extra_pay: crate::money::format_amount(entry.extra_pay),
            payout: if entry.payout == entry.base_pay + entry.extra_pay {
                String::new()
            } else {
                crate::money::format_amount(entry.payout)
            },
        }
    }

    /// Gross total of the current input, if both amounts are valid (for live previews).
    pub fn gross_preview(&self) -> Option<Decimal> {
        let base = crate::money::parse_amount(&self.base_pay).ok()?;
        let extra = if self.extra_pay.trim().is_empty() {
            Decimal::ZERO
        } else {
            crate::money::parse_amount(&self.extra_pay).ok()?
        };
        Some(base + extra)
    }

    pub fn validate(
        &self,
        employee_id: Uuid,
        period: Period,
        rates: Rates,
    ) -> Result<PayrollEntry, crate::ValidationErrors> {
        let mut v = crate::validate::Validator::new();
        let statement_date = v.required_date("statement_date", &self.statement_date);
        let base_pay = v.amount("base_pay", &self.base_pay);
        let extra_pay =
            if self.extra_pay.trim().is_empty() { Some(Decimal::ZERO) } else { v.amount("extra_pay", &self.extra_pay) };
        // An empty payout means "same as gross" (the usual Minijob case).
        let payout = if self.payout.trim().is_empty() {
            base_pay.zip(extra_pay).map(|(base, extra)| base + extra)
        } else {
            v.amount("payout", &self.payout)
        };
        if let (Some(base), Some(extra), Some(payout)) = (base_pay, extra_pay, payout)
            && payout > base + extra
        {
            v.error("payout", "Die Auszahlung kann nicht höher sein als das Gesamt-Brutto.");
        }
        v.finish(PayrollEntry {
            employee_id,
            period,
            statement_date: statement_date.unwrap_or_else(|| period.last_day()),
            base_pay: base_pay.unwrap_or_default(),
            extra_pay: extra_pay.unwrap_or_default(),
            payout: payout.unwrap_or_default(),
            rates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn validates_wage_input() {
        let period = Period::new(2026, 3).unwrap();
        let draft = PayrollDraft {
            statement_date: "31.03.2026".into(),
            base_pay: "538,00".into(),
            extra_pay: "".into(),
            payout: "538".into(),
        };
        let entry = draft.validate(Uuid::nil(), period, rates()).unwrap();
        assert_eq!(entry.extra_pay, dec!(0));
        let follows_gross = PayrollDraft { payout: " ".into(), extra_pay: "27".into(), ..draft.clone() };
        assert_eq!(follows_gross.validate(Uuid::nil(), period, rates()).unwrap().payout, dec!(565));
        assert_eq!(draft.gross_preview(), Some(dec!(538)));
        assert_eq!(PayrollDraft::from_entry(&entry).validate(Uuid::nil(), period, rates()).unwrap(), entry);

        let too_much = PayrollDraft { payout: "600".into(), ..draft.clone() };
        assert!(too_much.validate(Uuid::nil(), period, rates()).unwrap_err().get("payout").unwrap().contains("höher"));
        let broken = PayrollDraft { statement_date: "".into(), base_pay: "x".into(), ..draft };
        let errors = broken.validate(Uuid::nil(), period, rates()).unwrap_err();
        assert!(errors.get("statement_date").is_some() && errors.get("base_pay").is_some());
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

    /// Worked example from the old application (and the database).
    #[test]
    fn matches_reference_example() {
        let result = calculate(dec!(520.00), dec!(37.50), &rates());
        assert_eq!(result.gross_total, dec!(557.50));
        assert_eq!(result.contributions.health, dec!(72.48));
        assert_eq!(result.contributions.pension, dec!(83.63));
        assert_eq!(result.contributions.u1, dec!(6.13));
        assert_eq!(result.contributions.u2, dec!(1.34));
        assert_eq!(result.contributions.insolvency, dec!(0.33));
        assert_eq!(result.contributions.flat_tax, dec!(11.15));
        assert_eq!(result.total_contribution, dec!(175.06));
    }

    #[test]
    fn rounds_half_away_from_zero() {
        // 538.50 × 13 % = 70.005 → 70.01 (the old journal showed 70,00 by mistake)
        let result = calculate(dec!(538.50), dec!(0), &rates());
        assert_eq!(result.contributions.health, dec!(70.01));
    }

    #[test]
    fn zero_rates_and_amounts() {
        let result = calculate(dec!(0), dec!(0), &Rates::default());
        assert_eq!(result.total_contribution, dec!(0));
        assert_eq!(rates().total(), dec!(31.40));
    }
}
