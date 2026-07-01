# Simulation de collecte de ressources — Rust / Ratatui

Simulation multi-robots en terminal : des éclaireurs explorent une carte procédurale pendant que des collecteurs rapatrient les ressources découvertes à la base.

## Lancer le projet

```bash
cargo run
```

Appuyer sur n'importe quelle touche pour quitter. Les statistiques finales s'affichent dans le terminal.

## Carte des symboles

| Symbole | Couleur | Signification |
|---------|---------|---------------|
| `x` | Rouge | Scout (éclaireur) |
| `o` | Magenta | Collector (collecteur) |
| `#` | Vert clair | Base centrale |
| `E` | Vert | Ressource Énergie |
| `C` | Magenta clair | Ressource Cristal |
| `O` | Cyan | Obstacle |
| `.` | — | Case vide |

## Architecture

```
src/
├── main.rs         — point d'entrée, deux threads (simulation + UI)
├── simulation.rs   — état global, boucle tick(), canal mpsc
├── map.rs          — carte Vec<Vec<Cell>>, génération Perlin
├── robot.rs        — struct Robot commun (id, kind, position)
├── scout.rs        — déplacement aléatoire, observation, découvertes
├── collector.rs    — BFS, cycle WaitingForResource → Collecting → ReturningToBase
├── base.rs         — agrégation ressources, known_resources, known_obstacles
├── messages.rs     — enum RobotMessage (canal robots → base)
└── ui.rs           — rendu Ratatui, panneau Statistiques
```

## Dépendances

```toml
ratatui  = "0.29"   # UI terminal
crossterm = "0.28"  # événements clavier, mode raw
noise    = "0.9"    # bruit de Perlin (génération de carte)
rand     = "0.8"    # placement aléatoire, déplacement des scouts
```

## Contributors

- Marie Espinosa
- Thomas Stecinski
- Mattheo Naegellen
- Boris Prince
- Imane Zazar
