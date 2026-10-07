# ADR 0002 — Le hook est un confort, la CI est l'autorité

Statut : accepté — 7 octobre 2026

## Contexte

On veut empêcher un agent de code de modifier `features/`, `mandates/` ou `ledger/`. Un hook `PreToolUse` de Claude Code peut refuser une écriture, mais il s'exécute sur le poste du développeur : l'agent peut passer par le shell, un autre outil peut ne pas l'utiliser, et un développeur peut le désactiver. Une issue publique signale par ailleurs que le code de sortie 2 bloquait Bash mais pas toujours Write et Edit selon la version.

`CLAUDE.md` est encore plus faible : c'est une instruction, que l'agent peut ignorer.

## Décision

Cinq couches, de la plus faible à la plus forte : `CLAUDE.md` (indicatif), hook `PreToolUse` (confort local), scénarios BDD via `cargo test` (déterministe), CODEOWNERS avec protection de branche (revue humaine obligatoire), `mandat check` en CI (déterministe).

Le hook est traité comme un **confort** qui donne un retour immédiat. **L'autorité est côté serveur** : CI et revue des code owners. Pour le blocage, on privilégie la sortie JSON `permissionDecision: "deny"`, et on teste sur la version installée laquelle des deux formes bloque réellement.

## Conséquences

- Aucune démonstration ne repose sur le seul hook : s'il laisse passer, la CI rouge ou « Review required » attrape la dérive.
- Si le hook ne bloque pas sur la version du jour, le refus montré en CI et en revue est l'argument.
- La protection de `main` (PR obligatoire, check `gates`, revue des code owners) est réglée à la main dans GitHub et n'est pas versionnée.
- Un administrateur du dépôt peut contourner la protection : la limite est assumée et dite.
