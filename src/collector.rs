// Logique des robots collecteurs : navigation vers ressources, collecte, retour base.

use crate::map::{Position, ResourceType};
use crate::robot::{Robot, RobotId, RobotKind};

/// Etat du robot collecteur
#[derive(Debug, Clone, PartialEq)]
pub enum CollectorState {
    /// Attend une ressource connue à aller chercher.
    WaitingForResource,
    /// Se déplace vers une ressource.
    MovingToResource(Position),
    /// Collecte une ressource
    Collecting(Position),
    /// Rentre à la base avec une ressource
    ReturningToBase,
}

/// Definition robot collecteur
#[derive(Debug, Clone)]
pub struct Collector {
    // base robot (id, kind = collecteur, position)
    pub robot: Robot,
    // etat  (attente, deplacement, collecte, retour)
    pub state: CollectorState,
    /// type de ressource (energy ou crystal) None si pas de ressource
    pub carrying: Option<ResourceType>,
}

impl Collector {
    // Instanciation du robot collecteur
    pub fn new(id: RobotId, position: Position) -> Self {
        Self {
            robot: Robot::new(id, RobotKind::Collector, position),
            state: CollectorState::WaitingForResource,
            carrying: None,
        }
    }

    // retourne position
    pub fn position(&self) -> Position {
        self.robot.position
    }
}
