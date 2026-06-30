// Logique des robots éclaireurs : exploration aléatoire, découverte, partage.

use crate::map::{Cell, Map, Position, ResourceType};
use crate::robot::{Robot, RobotId, RobotKind};
use rand::Rng;

/// Ce qu'un scout peut découvrir sur une case voisine.
#[derive(Debug, Clone)]
pub enum Discovery {
    /// Une ressource découverte à cette position.
    Resource(Position, ResourceType),
    /// Un obstacle découvert à cette position.
    Obstacle(Position),
}

/// Etat du robot éclaireur
#[derive(Debug, Clone)]
pub struct Scout {
    pub robot: Robot,
    /// Découvertes faites depuis le dernier tick.
    pub discoveries: Vec<Discovery>,
}

impl Scout {
    // Instanciation du robot
    pub fn new(id: RobotId, position: Position) -> Self {
        Self {
            robot: Robot::new(id, RobotKind::Scout, position),
            discoveries: Vec::new(),
        }
    }

    /// Retourne la position actuelle du robot
    pub fn position(&self) -> Position {
        self.robot.position
    }

    /// Déplace le scout vers une celulle à à côté
    /// Reçoit une référence immuable sur la carte pour lire les cellules.
    /// Déplace le scout vers une case voisine libre (pas d'obstacle, pas hors-carte).
    pub fn move_randomly(&mut self, map: &Map) {
        let mut rng = rand::thread_rng();
        // Position actuelle du robot
        let pos = self.robot.position;

        // Directions possibles (haut, bas, gauche, droite)
        let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        // copie de directions
        // Mélange aléatoire des directions
        let mut dirs = directions;
        // pour chaque élement, on change aletatoirement avec un autre élement
        // revert pour eviter de mélanger les éléments déjà mélangés
        for i in (1..dirs.len()).rev() {
            let j = rng.gen_range(0..=i);
            dirs.swap(i, j);
        }

        // On essaie chaque direction et on prend la première case valide
        for (dx, dy) in dirs {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;

            // Verifie que la nouvelle position est dans la carte
            if nx < 0 || ny < 0 {
                continue;
            }
            // nouvelle position
            let new_pos = Position::new(nx as usize, ny as usize);
            // On lit la celulle pour recupérer son contenu
            let cell = map.get(new_pos);

            // On ne se déplace que sur une case vide ou la base (pas sur un obstacle).
            // On verifie si cell est pas None (hors carte)
            if let Some(cell) = cell {
                if *cell == Cell::Obstacle {
                    continue;
                }
                self.robot.position = new_pos;
                return;
            }
            // Si la case n'est pas valide, on essaie la direction suivante
        }
        // Si toutes les directions sont bloquées, le scout reste sur place.
    }

    /// Observe les cases adjacentes et enregistre les découvertes (ressources + obstacles).
    pub fn observe(&mut self, map: &Map) {
        let pos = self.robot.position;
        // Directions possibles (haut, bas, gauche, droite)
        let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in directions {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;

            if nx < 0 || ny < 0 {
                continue;
            }
            let neighbor = Position::new(nx as usize, ny as usize);
            let neighbor_cell = map.get(neighbor);

            // On lit la cellule voisine et on enregistre ce qu'on y trouve.
            if let Some(cell) = neighbor_cell {
                let discovery = match cell {
                    // Si c'est une ressource, on enregistre le type et la position
                    // Si c'est un obstacle, on enregistre la position
                    Cell::Obstacle => Some(Discovery::Obstacle(neighbor)),
                    Cell::Resource(r) => Some(Discovery::Resource(neighbor, r.kind)),
                    _ => None,
                };
                // On ajoute la découverte à la liste du scout
                if let Some(d) = discovery {
                    self.discoveries.push(d);
                }
            }
        }
    }
}
