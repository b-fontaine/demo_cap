//! `mandat hook` : décision PreToolUse. Refuse toute écriture sous
//! `crates/boutique/features/`, `mandates/` et `ledger/`.
//!
//! Confort poste développeur, pas une autorité : l'autorité est la CI et CODEOWNERS.
//! Refus en JSON sur stdout ; silence quand rien n'est protégé. Code de sortie 0.

use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde_json::{json, Value};

const PROTECTED: [&str; 3] = ["crates/boutique/features", "mandates", "ledger"];
const MSG: &str = "Écriture refusée sous crates/boutique/features/, mandates/ ou ledger/ : \
ces fichiers appartiennent au sponsor. Propose le diff de scénario dans ta réponse finale \
au lieu de le modifier.";

pub fn run(root_arg: &Path) -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let event: Value = serde_json::from_str(&input).unwrap_or(Value::Null);
    let out = match decide(&event, root_arg) {
        Some(reason) => json!({"hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": reason}}),
        // Rien à refuser : on n'émet rien, pour ne pas court-circuiter les invites de permission.
        None => return 0,
    };
    println!("{out}");
    0
}

/// `Some(raison)` si l'appel d'outil doit être refusé.
pub fn decide(event: &Value, root_arg: &Path) -> Option<String> {
    let cwd = event.get("cwd").and_then(Value::as_str).map(PathBuf::from);
    // Racine du projet : variable Claude Code, sinon cwd de l'événement, sinon --root.
    let root = std::env::var_os("CLAUDE_PROJECT_DIR")
        .map(PathBuf::from)
        .or_else(|| cwd.clone())
        .unwrap_or_else(|| absolute(root_arg));
    let base = cwd.unwrap_or_else(|| root.clone());
    let root = normalize(&root);
    let input = event.get("tool_input")?;
    match event.get("tool_name").and_then(Value::as_str)? {
        "Edit" | "Write" | "MultiEdit" | "NotebookEdit" => {
            let p = input.get("file_path").or_else(|| input.get("notebook_path"))?.as_str()?;
            is_protected(p, &base, &root).then(|| MSG.to_string())
        }
        "Bash" => {
            let cmd = input.get("command")?.as_str()?;
            bash_writes_protected(cmd, &base, &root).then(|| format!("{MSG} (commande shell détectée)"))
        }
        _ => None,
    }
}

fn absolute(p: &Path) -> PathBuf {
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(p)
    }
}

/// Normalisation lexicale (`.`, `..`), sans accès disque.
fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn is_protected(path: &str, base: &Path, root: &Path) -> bool {
    if path.is_empty() {
        return false;
    }
    let abs = normalize(&if Path::new(path).is_absolute() { PathBuf::from(path) } else { base.join(path) });
    match abs.strip_prefix(root) {
        Ok(rel) => PROTECTED.iter().any(|p| rel.starts_with(p)),
        Err(_) => false,
    }
}

const WRITE_VERBS: [&str; 17] = [
    "tee", "cp", "mv", "rm", "rmdir", "dd", "install", "truncate", "touch", "ln", "rsync", "chmod",
    "chown", "patch", "sed", "perl", "git",
];

fn clean(tok: &str) -> &str {
    tok.trim_matches(|c| matches!(c, '"' | '\'' | '(' | ')' | '{' | '}'))
}

fn bash_writes_protected(cmd: &str, base: &Path, root: &Path) -> bool {
    let normalized = cmd.replace("&&", ";").replace("||", ";").replace(['|', '\n', '&'], ";");
    normalized.split(';').any(|seg| segment_writes_protected(seg, base, root))
}

fn segment_writes_protected(seg: &str, base: &Path, root: &Path) -> bool {
    let toks: Vec<&str> = seg.split_whitespace().collect();
    let mut has_protected = false;
    for (i, raw) in toks.iter().enumerate() {
        // Redirection dont la cible est protégée (lire un fichier protégé reste permis).
        if let Some(pos) = raw.find('>') {
            let after = raw[pos..].trim_start_matches('>');
            let target = if after.is_empty() { toks.get(i + 1).copied().unwrap_or("") } else { after };
            if is_protected(clean(target), base, root) {
                return true;
            }
            if !raw[..pos].is_empty() && !raw[..pos].chars().all(|c| c.is_ascii_digit()) {
                has_protected |= is_protected(clean(&raw[..pos]), base, root);
            }
            continue;
        }
        if is_protected(clean(raw), base, root) {
            has_protected = true;
        }
    }
    if !has_protected {
        return false;
    }
    let verb = |t: &str| WRITE_VERBS.contains(&Path::new(clean(t)).file_name().and_then(|n| n.to_str()).unwrap_or(""));
    let in_place = |t: &&str| t.starts_with("-") && !t.starts_with("--") && t.contains('i') || t.starts_with("--in-place");
    toks.iter().enumerate().any(|(i, t)| {
        if !verb(t) {
            return false;
        }
        match clean(t) {
            "sed" | "perl" => toks[i + 1..].iter().any(in_place),
            "git" => toks[i + 1..].iter().any(|a| matches!(*a, "checkout" | "restore" | "apply" | "rm" | "mv")),
            _ => true,
        }
    })
}
