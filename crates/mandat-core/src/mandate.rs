//! Le mandat et sa validation.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ports::SpecCatalog;

/// Date civile `AAAA-MM-JJ`. L'ordre dérivé est chronologique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Approved,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Mandate {
    pub id: String,
    pub title: String,
    pub sponsor: String,
    pub objective: String,
    pub metric: String,
    pub scenarios: Vec<String>,
    pub budget_usd: f64,
    pub stop_when: String,
    pub owner: String,
    pub deadline: Date,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Authorization {
    pub remaining_usd: f64,
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum MandateError {
    #[error("champ manquant : {0}")]
    MissingField(&'static str),
    #[error("mandat non approuvé")]
    NotApproved,
    #[error("mandat expiré le {0}")]
    Expired(Date),
    #[error("budget absent ou nul")]
    NoBudget,
    #[error("scénario inconnu : {0}")]
    UnknownScenario(String),
    #[error("budget consommé : {spent} $ dépensés sur {budget} $")]
    BudgetExhausted { spent: f64, budget: f64 },
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Date {
    /// Analyse `AAAA-MM-JJ` (mois 1-12, jour 1-31).
    pub fn parse(s: &str) -> Result<Self, String> {
        let err = || format!("date invalide : {s:?} (attendu AAAA-MM-JJ)");
        let mut it = s.split('-');
        let (y, m, d) = (it.next(), it.next(), it.next());
        if it.next().is_some() || s.len() != 10 {
            return Err(err());
        }
        let year = y.and_then(|v| v.parse::<u16>().ok()).ok_or_else(err)?;
        let month = m.and_then(|v| v.parse::<u8>().ok()).filter(|v| (1..=12).contains(v)).ok_or_else(err)?;
        let day = d.and_then(|v| v.parse::<u8>().ok()).filter(|v| (1..=31).contains(v)).ok_or_else(err)?;
        Ok(Date { year, month, day })
    }
}

impl TryFrom<String> for Date {
    type Error = String;
    fn try_from(s: String) -> Result<Self, String> {
        Date::parse(&s)
    }
}

impl From<Date> for String {
    fn from(d: Date) -> String {
        d.to_string()
    }
}

impl MandateError {
    /// Code de sortie de la CLI : 3 budget consommé, 2 tout autre refus.
    pub fn exit_code(&self) -> i32 {
        match self {
            MandateError::BudgetExhausted { .. } => 3,
            _ => 2,
        }
    }
}

impl Mandate {
    /// Valide le mandat. Ordre : champs, statut, échéance, budget, scénarios.
    pub fn validate(&self, today: Date, catalog: &dyn SpecCatalog) -> Result<(), MandateError> {
        let required = [
            ("sponsor", &self.sponsor),
            ("objective", &self.objective),
            ("metric", &self.metric),
            ("owner", &self.owner),
        ];
        for (name, value) in required {
            if value.trim().is_empty() {
                return Err(MandateError::MissingField(name));
            }
        }
        if self.status != Status::Approved {
            return Err(MandateError::NotApproved);
        }
        if self.deadline < today {
            return Err(MandateError::Expired(self.deadline));
        }
        // `is_nan` explicite : un budget NaN est aussi refusé.
        if self.budget_usd.is_nan() || self.budget_usd <= 0.0 {
            return Err(MandateError::NoBudget);
        }
        if let Some(unknown) = self.scenarios.iter().find(|id| !catalog.scenario_exists(id)) {
            return Err(MandateError::UnknownScenario(unknown.clone()));
        }
        Ok(())
    }

    /// Validation puis contrôle du budget face à la dépense cumulée `spent`.
    pub fn authorize(
        &self,
        today: Date,
        catalog: &dyn SpecCatalog,
        spent: f64,
    ) -> Result<Authorization, MandateError> {
        self.validate(today, catalog)?;
        let remaining_usd = crate::budget::remaining(self.budget_usd, spent)?;
        Ok(Authorization { remaining_usd })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_aller_retour() {
        let d = Date::parse("2026-10-15").unwrap();
        assert_eq!(d, Date { year: 2026, month: 10, day: 15 });
        assert_eq!(d.to_string(), "2026-10-15");
    }

    #[test]
    fn date_invalide() {
        for s in ["2026-13-01", "2026-10", "26-10-15", "abcd-10-15", "2026-10-15-1"] {
            assert!(Date::parse(s).is_err(), "{s}");
        }
    }

    #[test]
    fn budget_nul_ou_negatif_refuse() {
        struct Tous;
        impl SpecCatalog for Tous {
            fn scenario_exists(&self, _: &str) -> bool {
                true
            }
            fn scenario_ids(&self) -> Vec<String> {
                vec![]
            }
        }
        let m = Mandate {
            id: "F".into(),
            title: "t".into(),
            sponsor: "s".into(),
            objective: "o".into(),
            metric: "m".into(),
            scenarios: vec![],
            budget_usd: 0.0,
            stop_when: "x".into(),
            owner: "o".into(),
            deadline: Date { year: 2030, month: 1, day: 1 },
            status: Status::Approved,
        };
        let today = Date { year: 2026, month: 10, day: 7 };
        assert_eq!(m.validate(today, &Tous), Err(MandateError::NoBudget));
    }

    #[test]
    fn codes_de_sortie() {
        assert_eq!(MandateError::NotApproved.exit_code(), 2);
        assert_eq!(MandateError::BudgetExhausted { spent: 1.0, budget: 1.0 }.exit_code(), 3);
    }
}
