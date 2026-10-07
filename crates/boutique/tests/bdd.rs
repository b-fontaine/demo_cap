use boutique::frais;
use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
struct BoutiqueWorld {
    panier: u64,
    frais: u64,
}

/// Convertit un montant français ("49,99", "4,90", "120,00" ou "120") en centimes,
/// sans passer par un flottant.
fn en_centimes(montant: &str) -> u64 {
    let (euros, centimes) = match montant.split_once(',') {
        Some((e, c)) => (e, c),
        None => (montant, "00"),
    };
    assert_eq!(centimes.len(), 2, "montant invalide : {montant}");
    let euros: u64 = euros.parse().expect("euros invalides");
    let centimes: u64 = centimes.parse().expect("centimes invalides");
    euros * 100 + centimes
}

#[given(regex = r"^un panier de (\d+(?:,\d{2})?) €$")]
fn un_panier(world: &mut BoutiqueWorld, montant: String) {
    world.panier = en_centimes(&montant);
}

#[when("je calcule les frais de livraison")]
fn calcul(world: &mut BoutiqueWorld) {
    world.frais = frais(world.panier);
}

#[then(regex = r"^les frais sont de (\d+(?:,\d{2})?) €$")]
fn frais_attendus(world: &mut BoutiqueWorld, montant: String) {
    assert_eq!(world.frais, en_centimes(&montant));
}

#[tokio::main]
async fn main() {
    BoutiqueWorld::cucumber().run_and_exit("features").await;
}
