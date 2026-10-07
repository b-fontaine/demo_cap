# KATA — Le repo comme support de formation

Chaque garde-fou de ce dépôt est un exercice. Trois niveaux, **20 minutes chacun**, à faire dans l'ordre. On ne passe au niveau suivant que si le précédent est vert.

Prérequis : une toolchain Rust stable, un clone propre, `cargo test --workspace` vert sur `main`. Travaillez sur une branche (`git switch -c kata-N`) et jetez-la ensuite.

---

## Niveau 1 — Écrire un scénario `LIV-005` et le faire passer

**Durée : 20 min. Garde-fou exercé : les scénarios BDD (`cargo test`).**

### Consignes

1. Lisez `crates/boutique/features/livraison.feature` et `crates/boutique/src/lib.rs`. Les montants sont des centimes entiers (`u64`), jamais de flottant.
2. Ajoutez un scénario `@LIV-005` : **un panier vide (0,00 €) a des frais de livraison de 0,00 €** (rien à livrer, rien à facturer). Gardez le même style : mots-clés Gherkin en anglais, phrases en français, un steps déjà existant réutilisé tel quel.
3. Lancez `cargo test -p boutique`. Le scénario doit être **rouge** : constatez-le avant de toucher au code.
4. Corrigez `lib.rs` au plus petit changement possible.
5. Relancez `cargo test --workspace` : tout est vert, `LIV-001` à `LIV-003` n'ont pas bougé.

### Critères de réussite

- [ ] Le scénario `@LIV-005` est présent et porte son tag.
- [ ] Vous avez vu le scénario rouge **avant** le correctif (copiez le message d'échec).
- [ ] Le correctif tient en quelques lignes et n'utilise aucun flottant.
- [ ] `cargo test --workspace` est vert.

### Indices

- Aucun nouveau step n'est nécessaire si vous réutilisez « un panier de … € » et « les frais sont de … € ».
- Un montant de 0 centime est un cas limite : demandez-vous quelle branche de la fonction l'attrape aujourd'hui.
- Dans la vraie vie, cette modification de la spec passerait par une PR approuvée par le sponsor (CODEOWNERS). Ici, vous êtes le sponsor.

---

## Niveau 2 — Une dérive volontaire, diagnostiquée en CI

**Durée : 20 min. Garde-fous exercés : CI et revue (CODEOWNERS).**

### Consignes

1. Sur une nouvelle branche, changez le seuil de livraison offerte dans `crates/boutique/src/lib.rs` de 50 € à 40 €. **Ne touchez pas** à `features/`.
2. Lancez `cargo test --workspace` en local : repérez quel scénario casse et pourquoi.
3. Poussez la branche, ouvrez une PR, observez le check `gates` rouge.
4. Dans les logs du job, retrouvez : l'étape en échec, le scénario, la valeur attendue et la valeur obtenue.
5. Écrivez en deux phrases le diagnostic : quelle règle a dérivé, de quoi à quoi, et pourquoi la CI l'a vu.
6. Deuxième partie : tentez d'« arranger » en modifiant `features/livraison.feature` dans la même PR. Observez l'état **Review required** imposé par CODEOWNERS.

### Critères de réussite

- [ ] Vous nommez le scénario rouge (`LIV-001`) sans lire la solution.
- [ ] Vous expliquez pourquoi `LIV-002` et `LIV-003` restent verts.
- [ ] Vous distinguez ce qui bloque (CI rouge) de ce qui attend un humain (revue du code owner).
- [ ] Vous savez dire pourquoi modifier la spec pour faire passer le code est précisément la dérive à éviter.

### Indices

- Un panier de 49,99 € est le cas qui change de côté du seuil.
- La sortie de `cargo test` nomme la feature, le scénario et l'étape qui échoue.
- GitHub interdit d'approuver sa propre PR : si vous êtes seul, vous restez bloqué en « Review required », et c'est voulu.
- Sans accès GitHub, le niveau se fait en local : le diagnostic via `cargo test` suffit pour les points 1 à 5.

---

## Niveau 3 — Une règle de mandat avec son test rouge

**Durée : 20 min. Garde-fou exercé : le domaine `mandat-core` en TDD.**

### Consignes

1. Lisez `mandates/FEAT-042.toml` et les tests de validation de `crates/mandat-core`.
2. Ajoutez la règle : **un mandat doit référencer au moins un scénario**. Erreur typée attendue : `NoScenario`.
3. Écrivez **d'abord** le test : un mandat valide dont la liste `scenarios` est vide doit être refusé avec `NoScenario`. Lancez `cargo test -p mandat-core` et constatez le rouge.
4. Ajoutez la variante d'erreur et la règle dans la validation, puis constatez le vert.
5. Vérifiez qu'aucun mandat existant (`FEAT-041`, `FEAT-042`) n'est devenu invalide : `cargo run -q -p mandat-cli -- check FEAT-042`.
6. Vérifiez que `mandat-core` n'a gagné aucune dépendance d'I/O (`cargo tree -p mandat-core`).

### Critères de réussite

- [ ] Le test existait et était rouge avant l'implémentation.
- [ ] L'erreur est typée (une variante, pas une chaîne) et un seul test couvre une seule règle.
- [ ] Les tests existants restent verts, `mandat check FEAT-042` aussi.
- [ ] Aucune I/O n'est entrée dans `mandat-core`.

### Indices

- Copiez le test du mandat complet valide, puis videz un seul champ : un test par règle, comme pour `MissingField`.
- L'ordre des vérifications compte si plusieurs règles échouent : placez la nouvelle après les champs obligatoires.
- Si la règle vous semble discutable, c'est le bon réflexe : une règle de gouvernance est une décision de sponsor, pas un détail de code.
