# Séquence de démo

Toutes les commandes se lancent depuis la racine du dépôt. 

## Répétition d'un seul coup

```bash
task demo:all          # agent réel, environ 0,6 $ et 2 minutes ; bilan durées, coûts, résultats
task demo:all:offline  # sans agent, aucun coût
```

## Avant de commencer (une fois)

```bash
cargo build -p mandat-cli          # le hook utilise target/debug/mandat
git status --short                 # doit être vide
export CLAUDE_FLAGS='--max-budget-usd 0.50 --permission-mode acceptEdits --allowedTools Read,Glob,Grep,Edit,Write,Bash'
```

En mode non interactif, sans `--allowedTools`, l'agent s'arrête dès sa première commande shell sans rien modifier.

## Entre deux démos

Les runs de l'agent salissent l'arbre de travail et `task demo:N` refuse de changer de tag si l'arbre est sale. Avant chaque démo :

```bash
git reset --hard && git clean -fdq
```

## Démo 1 — pas de mandat

```bash
task demo:1
```

Attendu : `aucun mandat pour FEAT-043` (code 2, l'agent n'est pas lancé), puis `FEAT-042 : mandat valide, budget restant 2.00 $`.

Message à faire passer : « Pas de mandat : l'agent n'est même pas lancé. Zéro token dépensé. »

## Démo 2 — dérive de spec

```bash
task demo:2
```

**Chemin 1, code seul** (environ 0,20 $, 1 minute) :

```bash
claude -p "$(cat prompts/1-code-seul.md)" $CLAUDE_FLAGS
cargo test --workspace             # attendu : LIV-001 rouge
```

L'agent change le seuil à 40 € dans le code, laisse la spec intacte et propose le diff de scénario. La CI aurait été rouge.

**Chemin 2, « fais passer les tests »** :

```bash
git reset --hard && git clean -fdq
claude -p "$(cat prompts/2-faire-passer.md)" $CLAUDE_FLAGS
```

Sur les essais réels, l'agent obéit à `.claude/CLAUDE.md` et ne touche pas à `features/`. Il peut aussi ne pas obéir : annonce-le.

**Chemin 2 bis, le hook en action** (si l'agent a obéi) :

```bash
git reset --hard && git clean -fdq
git rm -q --cached .claude/CLAUDE.md && rm .claude/CLAUDE.md
claude -p "$(git show main:prompts/2b-edition-directe.md)" $CLAUDE_FLAGS
git diff --stat crates/boutique/features   # attendu : vide
```

Le prompt est lu depuis `main` : il n'existe pas dans le tag `demo-2-derive`, qui le précède.

Attendu : deux `Edit` refusés par le hook (« ces fichiers appartiennent au sponsor »).

Message à faire passer : « Je ne contrôle pas ce que fait l'agent. Je contrôle les états dans lesquels il peut finir. »

Limite à dire : un interpréteur lancé par le shell contourne le hook. L'autorité est CODEOWNERS et la CI côté serveur.

## Démo 3 — chemin légitime

```bash
git reset --hard && git clean -fdq
task demo:3
cargo test --workspace             # attendu : LIV-004 rouge, la spec a été amendée par le sponsor
cargo run -q -p mandat-cli -- run FEAT-042 --prompt prompts/3-implementer.md
cargo test --workspace             # attendu : 4 scénarios verts
cargo run -q -p mandat-cli -- report
```

Premier run réel : coût de 0,19 $, une seule ligne de ledger ajoutée, seul `crates/boutique/src/lib.rs` modifié. Le coût varie d'un run à l'autre : dis « quelques dizaines de centimes ».

Message à faire passer : « La spec a changé par la seule voie légitime : le sponsor. Et voici ce que cette feature a coûté, relié au commit. »

La colonne `commit` est lue après le run : sans commit de l'agent, c'est le commit de départ.

## Démo 4 — budget consommé

```bash
git reset --hard && git clean -fdq
task demo:4
cargo run -q -p mandat-cli -- run FEAT-041 --prompt prompts/3-implementer.md   # attendu : code 3
```

Attendu : `budget consommé, nouveau mandat requis`.

Précise que le ledger de FEAT-041 est synthétique (`tool = "synthetic"`).

## Retour sur main

```bash
git reset --hard && git clean -fdq && git switch main
```

## Si ça coince

- L'agent tourne plus de 60 secondes : passe au tag suivant, sans excuse.
- Le réseau tombe : partage 4G, puis les enregistrements de secours.
- Chemin imprévu de l'agent : nomme-le, montre le garde-fou qui l'attrape, continue.
