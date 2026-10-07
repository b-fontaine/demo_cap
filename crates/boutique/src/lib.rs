//! Règle de livraison de la boutique : montants en centimes (`u64`), jamais de flottant.

/// Seuil de livraison offerte, en centimes (50,00 €).
pub const SEUIL_LIVRAISON_OFFERTE: u64 = 5000;

/// Frais de livraison standard sous le seuil, en centimes (4,90 €).
pub const FRAIS_STANDARD: u64 = 490;

/// Frais de livraison en centimes pour un panier d'un montant donné en centimes.
pub fn frais(panier: u64) -> u64 {
    if panier >= SEUIL_LIVRAISON_OFFERTE {
        0
    } else {
        FRAIS_STANDARD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sous_le_seuil_la_livraison_est_payante() {
        assert_eq!(frais(4999), 490);
    }

    #[test]
    fn au_seuil_la_livraison_est_offerte() {
        assert_eq!(frais(5000), 0);
    }
}
