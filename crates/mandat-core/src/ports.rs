//! Ports implémentés par les adaptateurs de `mandat-cli`.

use crate::{ledger::LedgerEntry, mandate::Mandate};

/// Erreur d'un adaptateur (fichier absent, processus en échec, etc.).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PortError {
    #[error("introuvable : {0}")]
    NotFound(String),
    #[error("échec de l'adaptateur : {0}")]
    Failed(String),
}

/// Charge les mandats (adaptateur : fichiers TOML).
pub trait MandateRepository {
    fn load(&self, id: &str) -> Result<Mandate, PortError>;
    fn list_ids(&self) -> Result<Vec<String>, PortError>;
}

/// Lit et complète le ledger (adaptateur : JSONL).
pub trait LedgerStore {
    fn entries(&self, feature: &str) -> Result<Vec<LedgerEntry>, PortError>;
    fn append(&self, entry: &LedgerEntry) -> Result<(), PortError>;
}

/// Scénarios existants et leurs identifiants (adaptateur : parseur Gherkin).
pub trait SpecCatalog {
    fn scenario_exists(&self, id: &str) -> bool;
    fn scenario_ids(&self) -> Vec<String>;
}

/// Résultat d'un run d'agent.
#[derive(Debug, Clone, PartialEq)]
pub struct RunOutcome {
    pub cost_usd: f64,
    pub session_id: String,
}

/// Lance l'agent (adaptateur : `claude -p`) et rend le coût.
pub trait AgentRunner {
    fn run(&self, mandate: &Mandate, prompt: &str) -> Result<RunOutcome, PortError>;
}
