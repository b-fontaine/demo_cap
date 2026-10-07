//! Contrôle du budget.
//!
//! Montants en dollars `f64` : l'outil rapporte `total_cost_usd` en dollars, on
//! évite toute conversion ; l'erreur d'arrondi est négligeable à cette échelle.

use crate::mandate::MandateError;

/// Rend le budget restant, ou `BudgetExhausted` si la dépense atteint le budget.
pub fn remaining(budget: f64, spent: f64) -> Result<f64, MandateError> {
    if spent >= budget {
        Err(MandateError::BudgetExhausted { spent, budget })
    } else {
        Ok(budget - spent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reste_positif_sous_le_budget() {
        assert_eq!(remaining(2.0, 0.5), Ok(1.5));
    }

    #[test]
    fn epuise_au_budget() {
        assert_eq!(remaining(2.0, 2.0), Err(MandateError::BudgetExhausted { spent: 2.0, budget: 2.0 }));
    }
}
