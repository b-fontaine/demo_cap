//! Les sept tests du plan, dans l'ordre, via l'API publique uniquement.

use mandat_core::ledger::total_cost;
use mandat_core::{Date, LedgerEntry, Mandate, MandateError, SpecCatalog, Status};

struct Catalog(Vec<&'static str>);
impl SpecCatalog for Catalog {
    fn scenario_exists(&self, id: &str) -> bool {
        self.0.contains(&id)
    }
    fn scenario_ids(&self) -> Vec<String> {
        self.0.iter().map(|s| s.to_string()).collect()
    }
}

const TODAY: Date = Date { year: 2026, month: 10, day: 7 };

fn catalog() -> Catalog {
    Catalog(vec!["LIV-001", "LIV-002", "LIV-003"])
}

fn mandate() -> Mandate {
    Mandate {
        id: "FEAT-042".into(),
        title: "Livraison offerte dès 40 €".into(),
        sponsor: "@b-fontaine".into(),
        objective: "Augmenter la conversion des paniers entre 40 et 50 €".into(),
        metric: "taux de conversion des paniers 40-50 €".into(),
        scenarios: vec!["LIV-001".into(), "LIV-002".into(), "LIV-003".into()],
        budget_usd: 2.00,
        stop_when: "budget consommé ou 3 runs sans scénario vert".into(),
        owner: "@b-fontaine".into(),
        deadline: Date { year: 2026, month: 10, day: 15 },
        status: Status::Approved,
    }
}

fn entry(cost: f64) -> LedgerEntry {
    LedgerEntry {
        feature: "FEAT-042".into(),
        at: "2026-10-07T15:12:03Z".into(),
        cost_usd: cost,
        session_id: "s".into(),
        commit: "a1b2c3d".into(),
        tool: "claude-code".into(),
    }
}

#[test]
fn t1_mandat_complet_valide() {
    assert_eq!(mandate().validate(TODAY, &catalog()), Ok(()));
}

#[test]
fn t2_chaque_champ_manquant() {
    let cas: Vec<(&str, fn(&mut Mandate))> = vec![
        ("sponsor", |m| m.sponsor = "  ".into()),
        ("objective", |m| m.objective = String::new()),
        ("metric", |m| m.metric = String::new()),
        ("owner", |m| m.owner = String::new()),
    ];
    for (nom, casse) in cas {
        let mut m = mandate();
        casse(&mut m);
        assert_eq!(m.validate(TODAY, &catalog()), Err(MandateError::MissingField(nom)));
    }
}

#[test]
fn t3_statut_non_approuve() {
    let mut m = mandate();
    m.status = Status::Draft;
    assert_eq!(m.validate(TODAY, &catalog()), Err(MandateError::NotApproved));
}

#[test]
fn t4_echeance_depassee() {
    let m = mandate();
    let apres = Date { year: 2026, month: 10, day: 16 };
    assert_eq!(m.validate(apres, &catalog()), Err(MandateError::Expired(m.deadline)));
    // Le jour même de l'échéance, le mandat reste valide.
    assert_eq!(m.validate(m.deadline, &catalog()), Ok(()));
}

#[test]
fn t5_scenario_inconnu() {
    let mut m = mandate();
    m.scenarios.push("LIV-999".into());
    assert_eq!(
        m.validate(TODAY, &catalog()),
        Err(MandateError::UnknownScenario("LIV-999".into()))
    );
}

#[test]
fn t6_budget_consomme() {
    let m = mandate();
    assert_eq!(
        m.authorize(TODAY, &catalog(), 2.00),
        Err(MandateError::BudgetExhausted { spent: 2.00, budget: 2.00 })
    );
    assert!(m.authorize(TODAY, &catalog(), 1.99).is_ok());
}

#[test]
fn t7_somme_du_ledger() {
    let m = mandate();
    let entries = [entry(0.5), entry(0.75), entry(0.25)];
    let spent = total_cost(&entries);
    assert_eq!(spent, 1.5);
    assert_eq!(m.authorize(TODAY, &catalog(), spent).unwrap().remaining_usd, 0.5);
    let entries = [entry(1.25), entry(0.75)];
    assert_eq!(
        m.authorize(TODAY, &catalog(), total_cost(&entries)),
        Err(MandateError::BudgetExhausted { spent: 2.0, budget: 2.0 })
    );
}
