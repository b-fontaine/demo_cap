# Des agents sous mandat

> Un agent de code ne démarre rien sans mandat ni budget, ne livre rien que la spec n'autorise, et s'arrête quand son budget est consommé. Ce dépôt le rend exécutable.

## Démarrage en 2 commandes

```bash
make ci        # tests unitaires + scénarios BDD
make demo-1    # l'agent refuse de démarrer sans mandat
```

## Les démos

Chaque démo part d'un tag git propre et se rejoue à froid avec `make demo-N`, qui remet aussi le ledger à son état versionné.

| Démo | Tag | Ce qu'elle montre |
| --- | --- | --- |
| 1. Pas de mandat | `demo-1-mandat` | `mandat run FEAT-043` est refusé (code 2), l'agent n'est jamais lancé ; `mandat check FEAT-042` est valide |
| 2. Dérive de spec | `demo-2-derive` | Changer la règle sans changer le scénario rend la CI rouge ; une écriture dans `features/` est refusée par le hook |
| 3. Chemin légitime | `demo-3-spec-approuvee` | La spec est amendée par le sponsor, l'agent implémente, une ligne de coût rejoint le ledger de FEAT-042 |
| 4. Budget | `demo-4-budget` | `mandat report` affiche FEAT-041 consommée ; un nouveau run est refusé (code 3) |

## Les cinq couches de garde-fous

| Couche | Force | Contournable ? |
| --- | --- | --- |
| `.claude/CLAUDE.md` | Indicative | Oui |
| Hook PreToolUse (`mandat hook`) | Bloquante pour cet outil | Oui : par le shell ou un autre outil |
| Scénarios BDD (`cargo test`) | Déterministe | Non, sauf en modifiant la spec |
| CODEOWNERS et protection de branche | Revue humaine obligatoire | Non, sauf droits admin |
| `mandat check` en CI | Déterministe | Non |

## Données synthétiques

`mandates/FEAT-041.toml` et `ledger/FEAT-041.jsonl` sont des **données synthétiques** (champ `tool = "synthetic"`), créées pour illustrer un budget presque consommé. Seul un run réel de la démo 3 ajoute une ligne mesurée au ledger de FEAT-042.

## Structure

- `crates/boutique` : la règle de livraison que l'agent modifie, avec ses scénarios BDD
- `crates/mandat-core` : domaine pur de gouvernance (aucune I/O)
- `crates/mandat-cli` : adaptateurs et binaire `mandat` (`check`, `run`, `report`, `hook`)
- `docs/adr` : trois décisions d'architecture courtes ; `KATA.md` : le dépôt comme support de formation

Licence MIT.
