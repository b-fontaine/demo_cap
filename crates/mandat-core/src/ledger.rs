//! Lignes de ledger (`ledger/FEAT-042.jsonl`) et agrégation.

use serde::{Deserialize, Serialize};

/// Une dépense d'agent, reliée au commit produit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub feature: String,
    /// Horodatage RFC 3339, fourni par l'appelant.
    pub at: String,
    pub cost_usd: f64,
    pub session_id: String,
    pub commit: String,
    pub tool: String,
}

/// Somme des coûts, en dollars.
pub fn total_cost(entries: &[LedgerEntry]) -> f64 {
    entries.iter().map(|e| e.cost_usd).sum()
}
