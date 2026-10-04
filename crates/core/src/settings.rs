//! Shared defaults (stored once in the database for all users).

use crate::money;
use crate::payroll::{Contribution, Rates};
use crate::validate::{ValidationErrors, Validator};

/// Values new employees start with; editable under "Einstellungen".
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Defaults {
    pub rates: Rates,
    pub health_insurer: String,
}

/// Raw form input for [`Defaults`]. Keys: `rate_health`, …, `health_insurer`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DefaultsDraft {
    pub rates: [String; 6],
    pub health_insurer: String,
}

impl DefaultsDraft {
    pub fn from_defaults(defaults: &Defaults) -> Self {
        Self {
            rates: Contribution::ALL.map(|c| money::format_flexible(defaults.rates.get(c), 0, 4)),
            health_insurer: defaults.health_insurer.clone(),
        }
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) -> bool {
        if key == "health_insurer" {
            self.health_insurer = value.into();
            return true;
        }
        match Contribution::ALL.iter().position(|c| c.key() == key) {
            Some(index) => {
                self.rates[index] = value.into();
                true
            }
            None => false,
        }
    }

    pub fn validate(&self) -> Result<Defaults, ValidationErrors> {
        let mut v = Validator::new();
        let mut rates = Rates::default();
        for (index, contribution) in Contribution::ALL.into_iter().enumerate() {
            if let Some(value) = v.percent(contribution.key(), &self.rates[index]) {
                rates.set(contribution, value);
            }
        }
        let health_insurer = v.optional_text("health_insurer", &self.health_insurer, 100);
        v.finish(Defaults { rates, health_insurer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn roundtrip() {
        let mut draft = DefaultsDraft::default();
        for (key, value) in [
            ("rate_health", "13"),
            ("rate_pension", "15"),
            ("rate_u1", "0,8"),
            ("rate_u2", "0,22"),
            ("rate_insolvency", "0,15"),
            ("rate_flat_tax", "2"),
        ] {
            assert!(draft.set(key, value));
        }
        assert!(draft.set("health_insurer", "Knappschaft"));
        let defaults = draft.validate().unwrap();
        assert_eq!(defaults.rates.u2, dec!(0.22));
        assert_eq!(DefaultsDraft::from_defaults(&defaults), draft);
        assert!(!draft.set("nope", "1"));
        draft.rates[0] = "200".into();
        assert!(draft.validate().unwrap_err().get("rate_health").is_some());
    }
}
