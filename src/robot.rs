use crate::map::Position;

/// Type de robot : scout ou collecteur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotKind {
    Scout,
    Collector,
}

/// Champs communs à tous les robots.
#[derive(Debug, Clone)]
pub struct Robot {
    pub kind: RobotKind,
    pub position: Position,
}

impl Robot {
    /// Instanciation d'un robot avec son type et sa position.
    pub fn new(kind: RobotKind, position: Position) -> Self {
        Self { kind, position }
    }

    /// Retourne le symbole du robot selon son type.
    pub fn symbol(&self) -> char {
        match self.kind {
            RobotKind::Scout => 'x',
            RobotKind::Collector => 'o',
        }
    }
}
