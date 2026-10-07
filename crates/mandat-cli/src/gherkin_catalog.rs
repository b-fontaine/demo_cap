//! Adaptateur `SpecCatalog` : les tags `@LIV-xxx` des scénarios de `crates/boutique/features`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use gherkin::{Feature, GherkinEnv};
use mandat_core::ports::{PortError, SpecCatalog};

pub struct GherkinCatalog {
    ids: BTreeSet<String>,
}

impl GherkinCatalog {
    pub fn load(root: &Path) -> Result<Self, PortError> {
        let dir = root.join("crates/boutique/features");
        let mut files = Vec::new();
        collect(&dir, &mut files)?;
        let mut ids = BTreeSet::new();
        for file in files {
            let feature = Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| PortError::Failed(format!("{} : {e}", file.display())))?;
            let rule_scenarios = feature.rules.iter().flat_map(|r| r.scenarios.iter());
            for sc in feature.scenarios.iter().chain(rule_scenarios) {
                ids.extend(sc.tags.iter().map(|t| t.trim_start_matches('@').to_string()));
            }
        }
        Ok(Self { ids })
    }
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), PortError> {
    let rd = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(PortError::Failed(format!("{} : {e}", dir.display()))),
    };
    for entry in rd.filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_dir() {
            collect(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "feature") {
            out.push(p);
        }
    }
    Ok(())
}

impl SpecCatalog for GherkinCatalog {
    fn scenario_exists(&self, id: &str) -> bool {
        self.ids.contains(id)
    }
    fn scenario_ids(&self) -> Vec<String> {
        self.ids.iter().cloned().collect()
    }
}
