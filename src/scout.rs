// Logique des robots éclaireurs : exploration aléatoire, découverte, partage.

use crate::map::{Cell, Map, Position};
use crate::robot::{Robot, RobotId, RobotKind};
use rand::Rng;

/// Etat du robot éclaireur
#[derive(Debug, Clone)]
pub struct Scout {
    pub robot: Robot,
}

impl Scout {
    // Instanciation du robot
    pub fn new(id: RobotId, position: Position) -> Self {
        Self {
            robot: Robot::new(id, RobotKind::Scout, position),
        }
    }

    /// Retourne la position actuelle du robot
    pub fn position(&self) -> Position {
        self.robot.position
    }

    /// Déplace le scout vers une celulle à à côté
    /// Reçoit une référence immuable sur la carte pour lire les cellules.
    pub fn move_randomly(&mut self, map: &Map) {
        let mut rng = rand::thread_rng();
        // Position actuelle du robot
        let pos = self.robot.position;

        // Directions possibles (haut, bas, gauche, droite)
        let directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        // Mélange aléatoire des directions
        // copie de directions
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

            // On ne se déplace que sur une case vide ou la base (pas sur un obstacle).
            if let Some(cell) = map.get(new_pos) {
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
}
