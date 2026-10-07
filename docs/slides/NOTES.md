# Notes de l'orateur

Vingt-cinq minutes, six slides et un terminal. Les slides portent les idées, le terminal porte les preuves : on ne lit jamais une slide.

Installation de l'écran : terminal en police 18 minimum, deux volets (agent à gauche, `mandat report` et `cargo test` à droite), fond clair si projection.

## Déroulé minuté

| Temps | Séquence | Action à l'écran | Phrase clé |
| --- | --- | --- | --- |
| 0:00 | Ouverture | Slide titre | « Un agent ne démarre rien sans mandat ni budget, ne livre rien que la spec n'autorise, et s'arrête quand son budget est consommé. » |
| 1:00 | Le mandat | Slide des six éléments | « C'est la règle que j'applique à tout chantier depuis mon P&L chez Micropole. Je l'applique maintenant aux agents. » |
| 2:00 | Démo 1 | `make demo-1`, `mandat run FEAT-043 …` puis `mandat check FEAT-042` | « Pas de mandat : l'agent n'est même pas lancé. Zéro token dépensé. » |
| 5:00 | Démo 2 | `make demo-2`, prompt 1 puis prompt 2 | « La dérive des specs est le premier risque d'un agent de code. Voici ma réponse. » Puis : « Je ne contrôle pas ce que fait l'agent. Je contrôle les états dans lesquels il peut finir. » |
| 10:00 | Démo 3 | `make demo-3`, prompt 3, puis `mandat report` | « La spec a changé par la seule voie légitime : le sponsor. Et voici ce que cette feature a coûté, relié au commit. » |
| 12:00 | Démo 4 | `make demo-4`, `mandat run FEAT-041 …` | « Budget consommé : arrêt net, nouveau mandat requis. Et j'ai appliqué la même règle à cette démo : le critère d'arrêt a été posé avant d'écrire la première ligne : ce qui n'était pas vert à l'heure dite sortait de la démo. » |
| 14:00 | Recul | Slide des limites | Le hook est contournable, l'autorité est en CI. Les scénarios ne couvrent que le comportement spécifié : mutation testing pour le reste. Le jugement sémantique d'une spec reste consultatif |
| 18:00 | Passage à l'échelle | `KATA.md` à l'écran, puis slide de la fiche du poste | « Ce repo est aussi un kata : chaque garde-fou est un exercice. C'est le format de mes bootcamps et de mes cours à l'IUT. » |
| 21:00 | OneCraft | Slide des 4 piliers | Une phrase par pilier, sur la ligne « First 6 months » |
| 24:00 | Clôture | Retour à la slide titre | « Je viens industrialiser ça pour vos clients, avec un mandat et un budget. » |

Règles de conduite pendant la démo :

- l'agent mouline plus de 60 secondes : passer au tag suivant, sans excuse ;
- l'agent prend un chemin imprévu : nommer le chemin, montrer le garde-fou qui l'attrape, continuer ;
- un juré interrompt : répondre, puis « je vous montre la suite, elle répond aussi à ça ».

## Notes par slide

### Slide 1 : Des agents sous mandat (0:00, 1 min)

- Dire la thèse dans les 30 premières secondes, sans lire la slide : un agent ne démarre rien sans mandat ni budget, ne livre rien que la spec n'autorise, et s'arrête quand son budget est consommé. C'est ce que je rends exécutable.
- Principe à poser : aucun chantier sans sponsor nommé, objectif mesurable, budget plafond et critère d'arrêt.
- Vocabulaire : parler de discipline budgétaire et de mandat. Ne pas revendiquer de titre.
- Ancre : « Aucun chantier sans mandat ni budget, y compris pour un agent. »

### Slide 2 : Les six éléments d'un mandat (1:00, 1 min)

- Six éléments, s'il en manque un le chantier ne démarre pas : sponsor, objectif mesurable, périmètre et critères d'acceptation, budget plafond, critère d'arrêt, owner et échéance.
- Trois niveaux d'application : le groupe (assessment qui vaut mandat, budget IA explicite), la feature (mandat versionné dans le repo, coût réel lu face au budget), l'agent (refuse de démarrer sans mandat valide, s'arrête au plafond).
- Appliqué à moi-même : la démo a été construite avec un critère d'arrêt fixé avant de coder, la veille de la soutenance. C'est la preuve que le principe est une pratique.
- Preuve de la casquette CTO : P&L et équipe de consultants chez Micropole/Talan.
- Si question sur le chiffrage : l'évitement de coût résiste à l'examen, pas les promesses de vélocité. Seule la mesure par feature est juste.

### Slide 3 : Cinq couches de garde-fous (rappel pendant la démo 2 et au recul)

- Lire l'ordre, du plus faible au plus fort : CLAUDE.md (indicatif, ignorable), hook PreToolUse (bloquant pour cet outil, contournable par le shell), scénarios BDD (déterministe, sauf à modifier la spec), CODEOWNERS et protection de branche (revue humaine, sauf droits admin), `mandat check` en CI (déterministe : mandats complets, scénarios référencés existants ; contournable seulement en modifiant le workflow, ce que la revue des code owners attrape).
- Message : l'instruction est un souhait, le hook est un confort, la CI et la revue sont l'autorité.
- Phrase d'ancrage tech lead : « Je ne contrôle pas ce que fait l'agent. Je contrôle les états dans lesquels il peut finir. »
- Preuve (parcours déclaré, à détailler à l'oral) : craft non négociable (TDD, BDD, clean architecture) chez AXA France.
- Plan de secours si le hook ne bloque pas Write sur la version du poste : montrer le refus en CI et en revue, c'est l'argument « l'autorité est côté serveur ».

### Slide 4 : Les limites (14:00, 4 min)

- Contournable : le hook, par le shell ou un autre outil. L'autorité est côté serveur : CI, CODEOWNERS, protection de branche.
- Non couvert : les scénarios ne couvrent que le comportement spécifié. Le mutation testing pour le reste.
- Consultatif : le jugement sémantique d'une spec reste un avis, pas une décision.
- Poser aussi ce qui prouverait que j'ai tort, puis laisser le jury réagir.
- Le principe de la réponse en trois temps : la réponse, la preuve, la limite.

### Slide 5 : La fiche du poste (18:00, 3 min, après `KATA.md`)

- Intitulé : celui de la fiche de poste (Principal Architect, AI-native SDLC). Le vocabulaire de la slide vient de cette fiche : le présenter comme une lecture de la fiche, pas comme une connaissance interne de l'organisation. Variante si le jury préfère sa nomenclature : Staff Engineer, industrialisation de l'IA dans le delivery.
- Trois missions : garde-fous, mesure et gouvernance, montée en compétence.
- Livrables à proposer comme engagement : J+30 kit v1 et mandat type, utilisé sur un pilote ; J+60 coût par feature sur ce pilote, visible par le sponsor ; J+90 bootcamp champions, première promotion, un talk interne, une équipe autonome sans moi.
- Dire avant qu'on le demande : je ne prends pas le management hiérarchique. Je prends le leadership technique d'équipes, rightshore compris.
- `KATA.md` : trois niveaux de 20 minutes. Niveau 1, écrire un scénario `LIV-005` et le faire passer ; niveau 2, faire échouer la CI par une dérive volontaire et la diagnostiquer ; niveau 3, écrire une règle de mandat avec son test rouge.
- Phrase d'ancrage formateur : « Ce repo est aussi un support de formation : chaque garde-fou est un exercice. » Preuve : Master 1 à l'IUT de Nantes, ateliers et bootcamps.
- Phrase d'ancrage mentor : « Je mesure un mentorat au jour où la personne n'a plus besoin de moi. »

### Slide 6 : OneCraft (21:00, 3 min)

- Une phrase par pilier, en s'appuyant sur la ligne « First 6 months » (engagement daté).
- Software Pioneer : transformer la veille IA en offres vendables, gouvernance mandat-budget, coût par feature.
- Craft Master : garde-fous exécutables, kit v1 sur un projet pilote. Certification : la slide n'affiche aucune ligne certification. Si une certification récente existe, l'annoncer à l'oral avec sa date ; ne rien inventer. Un trou annoncé passe, un trou découvert en Q&A coûte cher.
- People Guru : leadership technique rightshore compris, cohorte de champions autonome.
- Community Evangelist : une intervention interne, un CFP externe. Le mot CIOs figure sur la slide : le garder seulement si l'anglais tient sur une scène exécutive.
- Clôture : retour à la slide 1, « Je viens industrialiser ça pour vos clients, avec un mandat et un budget. »

## Questions à poser au jury

1. Quelle part du temps est attendue sur missions client ?
2. Qui arbitre mes refus quand plusieurs projets me sollicitent en même temps ?
3. Quel budget et quel sponsor porteraient le premier pilote du kit de garde-fous ?
4. Comment OneCraft mesure-t-il aujourd'hui la valeur de ses interventions IA chez les clients ?
