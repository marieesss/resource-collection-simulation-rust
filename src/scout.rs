// Logique des robots éclaireurs : exploration aléatoire, découverte, partage.

use crate::map::Position;
use crate::robot::{Robot, RobotId, RobotKind};

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
}
