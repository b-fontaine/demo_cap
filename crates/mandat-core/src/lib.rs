//! Domaine pur de `mandat` : « aucun chantier sans mandat ni budget ».
//!
//! Aucune I/O ici : pas de fichier, pas de processus, pas de réseau, pas
//! d'horloge système. La date courante est toujours un paramètre.
//!
//! # API publique pour `mandat-cli`
//!
//! - [`Mandate`] : le mandat (sponsor, objectif, métrique, scénarios, budget,
//!   owner, plus échéance et statut). Se désérialise avec serde (le format TOML
//!   est choisi par l'adaptateur).
//! - [`Mandate::validate`]`(today, &dyn SpecCatalog)` : valide le mandat et rend
//!   une [`MandateError`] typée : `MissingField(nom)`, `NotApproved`,
//!   `Expired(date)`, `NoBudget`, `UnknownScenario(id)`.
//! - [`Mandate::authorize`]`(today, catalog, spent)` : validation puis contrôle
//!   du budget, rend [`Authorization`] (budget restant) ou
//!   `BudgetExhausted { spent, budget }`.
//! - [`ledger::total_cost`] : somme des [`LedgerEntry`] d'une feature, à passer
//!   comme `spent`.
//! - [`MandateError::exit_code`] : 2 refusé, 3 budget consommé (codes de la CLI).
//! - [`ports`] : traits [`MandateRepository`], [`LedgerStore`], [`SpecCatalog`],
//!   [`AgentRunner`] que les adaptateurs de la CLI implémentent.
//! - [`trace::orphan_scenarios`] : scénarios qu'aucun mandat ne référence.
//! - [`Date`] : date civile `AAAA-MM-JJ`, construite par l'appelant.
//!
//! Flux type d'un `mandat check` : `repo.load(id)` → `ledger.entries(id)` →
//! `Mandate::authorize(today, &catalog, total_cost(&entries))`.

pub mod budget;
pub mod ledger;
pub mod mandate;
pub mod ports;
pub mod trace;

pub use ledger::LedgerEntry;
pub use mandate::{Authorization, Date, Mandate, MandateError, Status};
pub use ports::{AgentRunner, LedgerStore, MandateRepository, PortError, RunOutcome, SpecCatalog};
