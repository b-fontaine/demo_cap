//! Commandes `check`, `run`, `report`.

use std::path::Path;
use std::process::Command;

use mandat_core::ledger::total_cost;
use mandat_core::ports::{AgentRunner, LedgerStore, MandateRepository, PortError};
use mandat_core::{Date, LedgerEntry, Mandate, MandateError};

use crate::claude_runner::ClaudeRunner;
use crate::fs_mandates::FsMandates;
use crate::gherkin_catalog::GherkinCatalog;
use crate::jsonl_ledger::JsonlLedger;

/// Date du jour ; `MANDAT_TODAY=AAAA-MM-JJ` permet de la figer (tests, démo).
fn today() -> Result<Date, String> {
    if let Ok(v) = std::env::var("MANDAT_TODAY") {
        return Date::parse(&v);
    }
    use chrono::Datelike;
    let n = chrono::Utc::now().date_naive();
    Ok(Date { year: n.year() as u16, month: n.month() as u8, day: n.day() as u8 })
}

fn technical(msg: impl std::fmt::Display) -> i32 {
    eprintln!("erreur : {msg}");
    1
}

fn refuse(err: &MandateError) -> i32 {
    match err {
        MandateError::BudgetExhausted { .. } => eprintln!("budget consommé, nouveau mandat requis"),
        other => eprintln!("mandat refusé : {other}"),
    }
    err.exit_code()
}

/// Résultat de l'autorisation, ou code de sortie à rendre.
struct Authorized {
    mandate: Mandate,
    remaining_usd: f64,
}

fn authorize(root: &Path, feature: &str) -> Result<Authorized, i32> {
    let repo = FsMandates::new(root);
    let mandate = match repo.load(feature) {
        Ok(m) => m,
        Err(PortError::NotFound(_)) => {
            eprintln!("aucun mandat pour {feature}");
            return Err(2);
        }
        Err(e) => return Err(technical(e)),
    };
    let catalog = GherkinCatalog::load(root).map_err(technical)?;
    let entries = JsonlLedger::new(root).entries(feature).map_err(technical)?;
    let today = today().map_err(technical)?;
    match mandate.authorize(today, &catalog, total_cost(&entries)) {
        Ok(auth) => Ok(Authorized { mandate, remaining_usd: auth.remaining_usd }),
        Err(e) => Err(refuse(&e)),
    }
}

pub fn check(root: &Path, feature: &str) -> i32 {
    match authorize(root, feature) {
        Ok(a) => {
            println!(
                "{feature} : mandat valide, budget restant {:.2} $ sur {:.2} $",
                a.remaining_usd, a.mandate.budget_usd
            );
            0
        }
        Err(code) => code,
    }
}

pub fn run(root: &Path, feature: &str, prompt_path: &Path) -> i32 {
    // Le mandat est validé avant tout le reste : sans mandat, l'agent n'est jamais lancé.
    let auth = match authorize(root, feature) {
        Ok(a) => a,
        Err(code) => return code,
    };
    let prompt = match std::fs::read_to_string(prompt_path) {
        Ok(p) => p,
        Err(e) => return technical(format!("{} : {e}", prompt_path.display())),
    };
    println!("{feature} : mandat valide, budget restant {:.2} $", auth.remaining_usd);
    let runner = ClaudeRunner::new(root, auth.remaining_usd);
    let outcome = match runner.run(&auth.mandate, &prompt) {
        Ok(o) => o,
        Err(e) => return technical(e),
    };
    let entry = LedgerEntry {
        feature: feature.to_string(),
        at: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        cost_usd: outcome.cost_usd,
        session_id: outcome.session_id,
        commit: git_head(root),
        tool: "claude-code".into(),
    };
    if let Err(e) = JsonlLedger::new(root).append(&entry) {
        return technical(e);
    }
    println!("{feature} : run terminé, coût {:.4} $ (commit {})", entry.cost_usd, entry.commit);
    0
}

/// Commit courant (`git rev-parse --short HEAD`), « inconnu » hors dépôt git.
fn git_head(root: &Path) -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(root)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "inconnu".into())
}

pub fn report(root: &Path) -> i32 {
    let repo = FsMandates::new(root);
    let ledger = JsonlLedger::new(root);
    let ids = match repo.list_ids() {
        Ok(i) => i,
        Err(e) => return technical(e),
    };
    let catalog = match GherkinCatalog::load(root) {
        Ok(c) => c,
        Err(e) => return technical(e),
    };
    let today = match today() {
        Ok(t) => t,
        Err(e) => return technical(e),
    };
    println!("{:<10} {:>10} {:>10} {:>10}  STATUT", "FEATURE", "BUDGET $", "DÉPENSÉ $", "RESTANT $");
    for id in ids {
        let mandate = match repo.load(&id) {
            Ok(m) => m,
            Err(e) => {
                println!("{id:<10} {:>10} {:>10} {:>10}  illisible ({e})", "-", "-", "-");
                continue;
            }
        };
        let spent = match ledger.entries(&id) {
            Ok(e) => total_cost(&e) + 0.0, // `+ 0.0` : la somme d'une liste vide vaut -0.0
            Err(e) => return technical(e),
        };
        let statut = match mandate.authorize(today, &catalog, spent) {
            Ok(_) => "actif".to_string(),
            Err(MandateError::BudgetExhausted { .. }) => "consommé".to_string(),
            Err(e) => format!("refusé ({e})"),
        };
        let restant = (mandate.budget_usd - spent).max(0.0);
        println!("{id:<10} {:>10.2} {:>10.2} {:>10.2}  {statut}", mandate.budget_usd, spent, restant);
    }
    0
}
