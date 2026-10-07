# ADR 0001 — TOML plutôt que YAML pour les mandats

Statut : accepté — 7 octobre 2026

## Contexte

Les mandats (`mandates/FEAT-042.toml`) sont des fichiers de configuration lus par `mandat-cli` et validés par `mandat-core`. Ils sont versionnés avec le code et relus par un humain via CODEOWNERS. Le format doit être lisible, sans ambiguïté de typage, et pris en charge par une bibliothèque maintenue.

YAML est le choix réflexe, mais `serde_yaml` est archivé et n'est plus maintenu. Bâtir la gouvernance d'un projet sur une dépendance abandonnée serait incohérent avec le principe présenté.

## Décision

Les mandats sont écrits en **TOML**, lus par le crate `toml` (maintenu) avec `serde`. La version est vérifiée au moment du `cargo add`, pas de mémoire. Le parsing reste dans l'adaptateur `fs_mandates` de `mandat-cli` : `mandat-core` ne connaît que des types du domaine.

## Conséquences

- Le typage est explicite (chaînes entre guillemets, nombres, dates) : moins de surprises que le YAML (« no », indentation).
- Un mandat TOML plat est facile à relire dans une PR.
- Le TOML est moins répandu que le YAML dans l'outillage CI ; ici, personne d'autre que `mandat` ne lit ces fichiers.
- Changer de format plus tard ne touche que l'adaptateur, grâce au port `MandateRepository`.
