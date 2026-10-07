//! Adaptateur `MandateRepository` : un fichier TOML par mandat dans `mandates/`.

use std::path::PathBuf;

use mandat_core::ports::{MandateRepository, PortError};
use mandat_core::Mandate;

pub struct FsMandates {
    dir: PathBuf,
}

impl FsMandates {
    pub fn new(root: &std::path::Path) -> Self {
        Self { dir: root.join("mandates") }
    }
}

impl MandateRepository for FsMandates {
    fn load(&self, id: &str) -> Result<Mandate, PortError> {
        // Un identifiant ne doit jamais sortir de `mandates/`.
        if id.is_empty() || id.contains(['/', '\\']) || id.contains("..") {
            return Err(PortError::NotFound(id.to_string()));
        }
        let path = self.dir.join(format!("{id}.toml"));
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(PortError::NotFound(id.to_string()))
            }
            Err(e) => return Err(PortError::Failed(format!("{} : {e}", path.display()))),
        };
        toml::from_str(&text).map_err(|e| PortError::Failed(format!("{} : {e}", path.display())))
    }

    fn list_ids(&self) -> Result<Vec<String>, PortError> {
        let rd = match std::fs::read_dir(&self.dir) {
            Ok(rd) => rd,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(PortError::Failed(format!("{} : {e}", self.dir.display()))),
        };
        let mut ids: Vec<String> = rd
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let p = e.path();
                (p.extension()? == "toml").then(|| p.file_stem()?.to_str().map(String::from))?
            })
            .collect();
        ids.sort();
        Ok(ids)
    }
}
