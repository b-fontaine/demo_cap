//! Adaptateur `AgentRunner` : `claude -p <prompt> --output-format json`.
//!
//! L'exécutable se règle par `MANDAT_CLAUDE_BIN` (défaut `claude`), ce qui permet
//! de tester avec un faux script sans réseau.

use std::path::Path;
use std::process::{Command, Stdio};

use mandat_core::ports::{AgentRunner, PortError, RunOutcome};
use mandat_core::Mandate;

pub struct ClaudeRunner {
    bin: String,
    cwd: std::path::PathBuf,
    /// Budget restant du mandat, transmis à `--max-budget-usd`.
    max_budget_usd: f64,
}

impl ClaudeRunner {
    pub fn new(cwd: &Path, max_budget_usd: f64) -> Self {
        let bin = std::env::var("MANDAT_CLAUDE_BIN").unwrap_or_else(|_| "claude".into());
        Self { bin, cwd: cwd.to_path_buf(), max_budget_usd }
    }
}

impl AgentRunner for ClaudeRunner {
    fn run(&self, _mandate: &Mandate, prompt: &str) -> Result<RunOutcome, PortError> {
        let out = Command::new(&self.bin)
            .current_dir(&self.cwd)
            .args(["-p", prompt, "--output-format", "json", "--max-budget-usd"])
            .arg(format!("{:.4}", self.max_budget_usd))
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| PortError::Failed(format!("impossible de lancer {} : {e}", self.bin)))?;
        // Même en erreur (ex. budget atteint), claude rend un JSON avec le coût : on le lit d'abord.
        match parse_output(&out.stdout) {
            Some(outcome) => Ok(outcome),
            None if !out.status.success() => {
                Err(PortError::Failed(format!("{} a échoué ({})", self.bin, out.status)))
            }
            None => Err(PortError::Failed("sortie de l'agent illisible : total_cost_usd absent".into())),
        }
    }
}

fn parse_output(stdout: &[u8]) -> Option<RunOutcome> {
    let v: serde_json::Value = serde_json::from_slice(stdout).ok()?;
    let cost_usd = v.get("total_cost_usd")?.as_f64()?;
    let session_id = v.get("session_id").and_then(|s| s.as_str()).unwrap_or("").to_string();
    Some(RunOutcome { cost_usd, session_id })
}
