//! Tests d'intégration : vrai binaire `mandat`, dépôt temporaire, faux `claude`.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command as Proc;

use assert_cmd::Command;
use serde_json::{json, Value};
use tempfile::TempDir;

const FEATURE: &str = "Feature: Frais\n\n  @LIV-001\n  Scenario: a\n    Given x\n\n  @LIV-002\n  Scenario: b\n    Given y\n";

fn mandate(id: &str, budget: &str, scenarios: &str, deadline: &str, status: &str) -> String {
    format!(
        "id = \"{id}\"\ntitle = \"t\"\nsponsor = \"@sponsor\"\nobjective = \"o\"\nmetric = \"m\"\n\
         scenarios = [{scenarios}]\nbudget_usd = {budget}\nstop_when = \"s\"\nowner = \"@owner\"\n\
         deadline = \"{deadline}\"\nstatus = \"{status}\"\n"
    )
}

fn ledger_line(feature: &str, cost: f64) -> String {
    format!(
        "{{\"feature\":\"{feature}\",\"at\":\"2026-10-06T10:00:00Z\",\"cost_usd\":{cost},\"session_id\":\"s\",\"commit\":\"abc1234\",\"tool\":\"claude-code\"}}\n"
    )
}

struct Repo {
    dir: TempDir,
}

impl Repo {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        let r = dir.path();
        fs::create_dir_all(r.join("crates/boutique/features")).unwrap();
        fs::create_dir_all(r.join("mandates")).unwrap();
        fs::create_dir_all(r.join("ledger")).unwrap();
        fs::write(r.join("crates/boutique/features/livraison.feature"), FEATURE).unwrap();
        fs::write(
            r.join("mandates/FEAT-042.toml"),
            mandate("FEAT-042", "2.00", "\"LIV-001\", \"LIV-002\"", "2026-10-15", "approved"),
        )
        .unwrap();
        fs::write(r.join("mandates/FEAT-041.toml"), mandate("FEAT-041", "1.00", "\"LIV-001\"", "2026-10-15", "approved"))
            .unwrap();
        fs::write(r.join("ledger/FEAT-041.jsonl"), ledger_line("FEAT-041", 0.6) + &ledger_line("FEAT-041", 0.5)).unwrap();
        fs::write(r.join("prompt.md"), "Implémente FEAT-042").unwrap();
        // Faux claude : note ses arguments, rend un JSON de coût.
        let fake = r.join("fake-claude.sh");
        fs::write(
            &fake,
            "#!/bin/sh\necho \"$@\" > \"$(dirname \"$0\")/fake-args\"\n\
             echo '{\"type\":\"result\",\"total_cost_usd\":0.41,\"session_id\":\"sess-1\"}'\n",
        )
        .unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        git(r, &["init", "-q"]);
        git(r, &["add", "."]);
        git(r, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "init"]);
        Repo { dir }
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn mandat(&self) -> Command {
        let mut c = Command::cargo_bin("mandat").unwrap();
        c.current_dir(self.path())
            .env("MANDAT_TODAY", "2026-10-07")
            .env("MANDAT_CLAUDE_BIN", self.path().join("fake-claude.sh"));
        c
    }

    fn agent_launched(&self) -> bool {
        self.path().join("fake-args").exists()
    }
}

fn git(dir: &Path, args: &[&str]) {
    assert!(Proc::new("git").args(args).current_dir(dir).status().unwrap().success());
}

#[test]
fn check_sans_mandat_refuse_code_2() {
    let r = Repo::new();
    r.mandat().args(["check", "FEAT-043"]).assert().code(2).stderr("aucun mandat pour FEAT-043\n");
}

#[test]
fn check_mandat_valide() {
    let r = Repo::new();
    r.mandat()
        .args(["check", "FEAT-042"])
        .assert()
        .code(0)
        .stdout("FEAT-042 : mandat valide, budget restant 2.00 $ sur 2.00 $\n");
}

#[test]
fn check_budget_consomme_code_3() {
    let r = Repo::new();
    r.mandat().args(["check", "FEAT-041"]).assert().code(3).stderr("budget consommé, nouveau mandat requis\n");
}

#[test]
fn check_scenario_inconnu_expire_et_brouillon_code_2() {
    let r = Repo::new();
    let w = |name: &str, body: String| fs::write(r.path().join(format!("mandates/{name}.toml")), body).unwrap();
    w("FEAT-050", mandate("FEAT-050", "2.0", "\"LIV-999\"", "2026-10-15", "approved"));
    w("FEAT-051", mandate("FEAT-051", "2.0", "\"LIV-001\"", "2026-10-01", "approved"));
    w("FEAT-052", mandate("FEAT-052", "2.0", "\"LIV-001\"", "2026-10-15", "draft"));
    for (id, msg) in [("FEAT-050", "scénario inconnu : LIV-999"), ("FEAT-051", "expiré"), ("FEAT-052", "non approuvé")] {
        let out = r.mandat().args(["check", id]).assert().code(2).get_output().stderr.clone();
        assert!(String::from_utf8(out).unwrap().contains(msg), "{id}");
    }
}

#[test]
fn run_sans_mandat_ne_lance_pas_l_agent() {
    let r = Repo::new();
    r.mandat()
        .args(["run", "FEAT-043", "--prompt", "prompt.md"])
        .assert()
        .code(2)
        .stderr("aucun mandat pour FEAT-043\n");
    assert!(!r.agent_launched());
    assert!(!r.path().join("ledger/FEAT-043.jsonl").exists());
}

#[test]
fn run_budget_consomme_ne_lance_pas_l_agent() {
    let r = Repo::new();
    r.mandat()
        .args(["run", "FEAT-041", "--prompt", "prompt.md"])
        .assert()
        .code(3)
        .stderr("budget consommé, nouveau mandat requis\n");
    assert!(!r.agent_launched());
}

#[test]
fn run_valide_ecrit_le_ledger_avec_le_commit() {
    let r = Repo::new();
    r.mandat().args(["run", "FEAT-042", "--prompt", "prompt.md"]).assert().code(0);
    let head = Proc::new("git").args(["rev-parse", "--short", "HEAD"]).current_dir(r.path()).output().unwrap();
    let head = String::from_utf8(head.stdout).unwrap().trim().to_string();
    let ledger = fs::read_to_string(r.path().join("ledger/FEAT-042.jsonl")).unwrap();
    let line: Value = serde_json::from_str(ledger.lines().next().unwrap()).unwrap();
    assert_eq!(line["feature"], "FEAT-042");
    assert_eq!(line["cost_usd"], 0.41);
    assert_eq!(line["session_id"], "sess-1");
    assert_eq!(line["commit"], head.as_str());
    assert_eq!(line["tool"], "claude-code");
    assert_eq!(ledger.lines().count(), 1);
    let args = fs::read_to_string(r.path().join("fake-args")).unwrap();
    assert!(args.starts_with("-p Implémente FEAT-042 --output-format json --max-budget-usd 2.0000"), "{args}");
    // Le coût s'ajoute : le restant baisse.
    r.mandat()
        .args(["check", "FEAT-042"])
        .assert()
        .stdout("FEAT-042 : mandat valide, budget restant 1.59 $ sur 2.00 $\n");
}

#[test]
fn run_echec_de_l_agent_code_1_sans_ledger() {
    let r = Repo::new();
    let mut c = r.mandat();
    c.env("MANDAT_CLAUDE_BIN", "/nonexistent/claude");
    c.args(["run", "FEAT-042", "--prompt", "prompt.md"]).assert().code(1);
    assert!(!r.path().join("ledger/FEAT-042.jsonl").exists());
}

#[test]
fn report_affiche_budget_depense_restant_statut() {
    let r = Repo::new();
    let out = r.mandat().arg("report").assert().code(0).get_output().stdout.clone();
    let out = String::from_utf8(out).unwrap();
    let cols = |id: &str| -> Vec<String> {
        out.lines().find(|l| l.starts_with(id)).unwrap().split_whitespace().map(String::from).collect()
    };
    assert_eq!(cols("FEAT-041"), ["FEAT-041", "1.00", "1.10", "0.00", "consommé"]);
    assert_eq!(cols("FEAT-042"), ["FEAT-042", "2.00", "0.00", "2.00", "actif"]);
}

fn hook(r: &Repo, event: Value) -> Value {
    let root = r.path().to_str().unwrap();
    let out = r
        .mandat()
        .env("CLAUDE_PROJECT_DIR", root)
        .arg("hook")
        .write_stdin(event.to_string())
        .assert()
        .code(0)
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&out).unwrap()
}

fn decision(v: &Value) -> &str {
    v["hookSpecificOutput"]["permissionDecision"].as_str().unwrap()
}

#[test]
fn hook_refuse_les_ecritures_protegees() {
    let r = Repo::new();
    let root = r.path().to_str().unwrap().to_string();
    for p in [
        format!("{root}/crates/boutique/features/livraison.feature"),
        "mandates/FEAT-042.toml".to_string(),
        "./ledger/FEAT-042.jsonl".to_string(),
        "src/../mandates/x.toml".to_string(),
    ] {
        let v = hook(&r, json!({"tool_name":"Edit","cwd":root,"tool_input":{"file_path":p}}));
        assert_eq!(decision(&v), "deny", "{p}");
        assert_eq!(v["hookSpecificOutput"]["hookEventName"], "PreToolUse");
        assert!(v["hookSpecificOutput"]["permissionDecisionReason"].as_str().unwrap().contains("sponsor"));
    }
    for tool in ["Write", "MultiEdit"] {
        let v = hook(&r, json!({"tool_name":tool,"cwd":root,"tool_input":{"file_path":"mandates/a.toml"}}));
        assert_eq!(decision(&v), "deny", "{tool}");
    }
}

#[test]
fn hook_autorise_le_reste() {
    let r = Repo::new();
    let root = r.path().to_str().unwrap().to_string();
    for p in ["crates/boutique/src/lib.rs", "mandates-notes.md", "crates/boutique/features-old/x"] {
        let v = hook(&r, json!({"tool_name":"Write","cwd":root,"tool_input":{"file_path":p}}));
        assert_eq!(decision(&v), "allow", "{p}");
    }
    let lecture = hook(&r, json!({"tool_name":"Read","cwd":root,"tool_input":{"file_path":"mandates/FEAT-042.toml"}}));
    assert_eq!(decision(&lecture), "allow");
    // Entrée illisible : on n'invente pas de refus.
    let out = r.mandat().arg("hook").write_stdin("pas du json").assert().code(0).get_output().stdout.clone();
    assert_eq!(decision(&serde_json::from_slice(&out).unwrap()), "allow");
}

#[test]
fn hook_bash_refuse_ecritures_et_autorise_lectures() {
    let r = Repo::new();
    let root = r.path().to_str().unwrap().to_string();
    let bash = |c: &str| decision(&hook(&r, json!({"tool_name":"Bash","cwd":root,"tool_input":{"command":c}}))).to_string();
    for c in [
        "echo x > mandates/FEAT-042.toml",
        "echo x >> ledger/FEAT-042.jsonl",
        "sed -i s/a/b/ crates/boutique/features/livraison.feature",
        "rm -rf ./mandates",
        "cargo test && cp x ledger/y",
        "git checkout -- crates/boutique/features",
    ] {
        assert_eq!(bash(c), "deny", "{c}");
    }
    for c in [
        "cat mandates/FEAT-042.toml",
        "ls ledger",
        "cargo test",
        "grep LIV crates/boutique/features/livraison.feature > out.txt",
        "echo x > notes.md",
    ] {
        assert_eq!(bash(c), "allow", "{c}");
    }
}
