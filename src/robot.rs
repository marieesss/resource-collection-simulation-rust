use crate::map::Position;

/// Identifiant unique d'un robot.
pub type RobotId = usize;

/// Type de robot : scout ou collecteur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotKind {
    Scout,
    Collector,
}

/// Champs communs à tous les robots.
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: RobotId,
    pub kind: RobotKind,
    pub position: Position,
}

impl Robot {
    /// Instanciation d'un robot avec son id, son type et sa position.
    pub fn new(id: RobotId, kind: RobotKind, position: Position) -> Self {
        Self { id, kind, position }
    }

    /// Retourne le symbole du robot selon son type.
    pub fn symbol(&self) -> char {
        match self.kind {
            RobotKind::Scout => 'x',
            RobotKind::Collector => 'o',
        }
    }
}
