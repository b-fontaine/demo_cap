# Règles projet pour l'agent

- Si une règle métier change, n'édite pas `features/` : propose le diff de scénario dans ta réponse finale.
- N'édite pas `mandates/` ni `ledger/` : ils appartiennent au sponsor et au CLI `mandat`.
- Le code métier vit dans `crates/boutique/src/` ; les montants sont des centimes entiers (`u64`), jamais de flottant.
- Vérifie ton travail avec `cargo test --workspace` avant de conclure.
