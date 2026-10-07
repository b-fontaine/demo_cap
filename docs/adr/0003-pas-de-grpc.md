# ADR 0003 — Pas de gRPC (ni Kong, Temporal ou Flutter)

Statut : accepté — 7 octobre 2026

## Contexte

La stack habituelle de l'auteur est Rust hexagonal avec gRPC, Kong comme passerelle, Temporal pour l'orchestration et Flutter pour l'interface. Ce dépôt démontre qu'on ne lance aucun chantier sans mandat ni budget. Or il n'expose aucune API, n'a pas d'orchestration longue et n'a pas d'interface.

## Décision

Le dépôt reste en **Rust hexagonal, sous forme de CLI, sans gRPC**. Kong, Temporal et Flutter sont absents : pas d'API exposée, la CI suffit comme orchestration, et le BDD est porté par `cucumber` côté Rust. Le TDD/BDD est conservé et sert de cœur à la démo.

Les ports de `mandat-core` (`MandateRepository`, `LedgerStore`, `SpecCatalog`, `AgentRunner`) laissent la possibilité d'ajouter plus tard un adaptateur réseau sans toucher au domaine.

## Conséquences

- Moins de code, moins de dépendances, une démo qui tient en 25 minutes.
- La frontière hexagonale est imposée par les dépendances Cargo : `mandat-core` n'a ni I/O ni réseau.
- Défense en une phrase : « J'ai appliqué ma propre règle : rien sans mandat. Rien dans cette démo ne justifiait un gRPC. »
- Si un client réseau devient nécessaire, il fera l'objet de son propre mandat et d'un nouvel ADR.
