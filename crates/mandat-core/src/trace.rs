//! Traçabilité scénarios et mandats (P2). Seule la partie pure est ici.
//! TODO (P2, documenté) : lecture des tags `@LIV-004` dans `features/` et
//! commande `mandat trace` vivent côté CLI (I/O), pas dans ce crate.

use crate::mandate::Mandate;

/// Scénarios existants qu'aucun mandat ne référence.
pub fn orphan_scenarios(scenarios: &[String], mandates: &[Mandate]) -> Vec<String> {
    scenarios
        .iter()
        .filter(|id| !mandates.iter().any(|m| m.scenarios.contains(id)))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mandate::{Date, Status};

    #[test]
    fn signale_le_scenario_non_reference() {
        let m = Mandate {
            id: "FEAT-042".into(),
            title: "t".into(),
            sponsor: "s".into(),
            objective: "o".into(),
            metric: "m".into(),
            scenarios: vec!["LIV-001".into()],
            budget_usd: 1.0,
            stop_when: "x".into(),
            owner: "o".into(),
            deadline: Date { year: 2026, month: 10, day: 15 },
            status: Status::Approved,
        };
        let tous = vec!["LIV-001".to_string(), "LIV-004".to_string()];
        assert_eq!(orphan_scenarios(&tous, &[m]), vec!["LIV-004".to_string()]);
    }
}
