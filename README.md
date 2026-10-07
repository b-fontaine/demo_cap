# Des agents sous mandat

> Un agent de code ne démarre rien sans mandat ni budget, ne livre rien que la spec n'autorise, et s'arrête quand son budget est consommé. Ce dépôt le rend exécutable.

## Démarrage en 2 commandes

```bash
make ci        # tests unitaires + scénarios BDD
make demo-1    # l'agent refuse de démarrer sans mandat
```

Détail des démos, des garde-fous et du kata : voir `KATA.md` et `docs/adr/`.
