// Messages échangés entre robots et base : découvertes, collectes, synchronisation.

use crate::map::{Position, ResourceType};

/// Messages envoyés par les scouts et collectors vers la base.
#[derive(Debug, Clone)]
pub enum RobotMessage {
    /// Un robot scout a découvert une ressource à cette position.
    ResourceDiscovered { position: Position, kind: ResourceType },
    /// Un robot scout a découvert un obstacle à cette position.
    ObstacleDiscovered { position: Position },
    /// Un robot collector a déposé une unité de ressource à la base.
    ResourceDeposited { kind: ResourceType },
    /// Un robot collector a prélevé une unité — la base retire la ressource de ses connaissances.
    ResourceCollected { position: Position },
}
