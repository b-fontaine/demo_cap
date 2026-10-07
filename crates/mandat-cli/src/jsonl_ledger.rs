//! Adaptateur `LedgerStore` : `ledger/<FEAT>.jsonl`, une ligne JSON par dépense.

use std::io::Write;
use std::path::{Path, PathBuf};

use mandat_core::ports::{LedgerStore, PortError};
use mandat_core::LedgerEntry;

pub struct JsonlLedger {
    dir: PathBuf,
}

impl JsonlLedger {
    pub fn new(root: &Path) -> Self {
        Self { dir: root.join("ledger") }
    }

    fn path(&self, feature: &str) -> Result<PathBuf, PortError> {
        if feature.is_empty() || feature.contains(['/', '\\']) || feature.contains("..") {
            return Err(PortError::Failed(format!("identifiant invalide : {feature}")));
        }
        Ok(self.dir.join(format!("{feature}.jsonl")))
    }
}

impl LedgerStore for JsonlLedger {
    fn entries(&self, feature: &str) -> Result<Vec<LedgerEntry>, PortError> {
        let path = self.path(feature)?;
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(PortError::Failed(format!("{} : {e}", path.display()))),
        };
        text.lines()
            .enumerate()
            .filter(|(_, l)| !l.trim().is_empty())
            .map(|(i, l)| {
                serde_json::from_str(l)
                    .map_err(|e| PortError::Failed(format!("{} ligne {} : {e}", path.display(), i + 1)))
            })
            .collect()
    }

    fn append(&self, entry: &LedgerEntry) -> Result<(), PortError> {
        let path = self.path(&entry.feature)?;
        let fail = |e: std::io::Error| PortError::Failed(format!("{} : {e}", path.display()));
        std::fs::create_dir_all(&self.dir).map_err(fail)?;
        let mut line = serde_json::to_string(entry).map_err(|e| PortError::Failed(e.to_string()))?;
        line.push('\n');
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path).map_err(fail)?;
        f.write_all(line.as_bytes()).map_err(fail)
    }
}
