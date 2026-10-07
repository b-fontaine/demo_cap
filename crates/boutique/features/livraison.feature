Feature: Frais de livraison

  @LIV-001
  Scenario: Livraison payante sous le seuil
    Given un panier de 49,99 €
    When je calcule les frais de livraison
    Then les frais sont de 4,90 €

  @LIV-002
  Scenario: Livraison offerte au seuil
    Given un panier de 50,00 €
    When je calcule les frais de livraison
    Then les frais sont de 0,00 €

  @LIV-003
  Scenario: Livraison offerte au-dessus du seuil
    Given un panier de 120,00 €
    When je calcule les frais de livraison
    Then les frais sont de 0,00 €
