//! CLI `mandat` : adaptateurs (TOML, JSONL, Gherkin, `claude -p`, hook) autour de `mandat-core`.
//!
//! Codes de sortie : 0 succès, 2 refus (mandat absent ou invalide), 3 budget consommé,
//! 1 erreur technique (fichier illisible, agent en échec).

pub mod claude_runner;
pub mod commands;
pub mod fs_mandates;
pub mod gherkin_catalog;
pub mod hook;
pub mod jsonl_ledger;

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mandat", about = "Aucun chantier sans mandat ni budget")]
struct Cli {
    /// Racine du dépôt (contient `mandates/`, `ledger/`, `crates/boutique/features/`).
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Valide le mandat et le budget restant d'une feature.
    Check { feature: String },
    /// Valide, lance l'agent, écrit le coût dans le ledger.
    Run {
        feature: String,
        #[arg(long)]
        prompt: PathBuf,
    },
    /// Tableau budget / dépensé / restant par feature.
    Report,
    /// Décision PreToolUse (JSON sur stdin, JSON sur stdout).
    Hook,
}

/// Point d'entrée testable : rend le code de sortie.
pub fn run_cli<I, T>(args: I) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(c) => c,
        Err(e) => {
            let _ = e.print();
            return if e.use_stderr() { 1 } else { 0 };
        }
    };
    match cli.command {
        Command::Check { feature } => commands::check(&cli.root, &feature),
        Command::Run { feature, prompt } => commands::run(&cli.root, &feature, &prompt),
        Command::Report => commands::report(&cli.root),
        Command::Hook => hook::run(&cli.root),
    }
}
