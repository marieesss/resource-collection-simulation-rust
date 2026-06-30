// Messages échangés entre robots et base : découvertes, collectes, synchronisation.

use crate::map::{Position, ResourceType};
use crate::robot::RobotId;

/// Messages envoyés par les scouts et collectors vers la base.
#[derive(Debug, Clone)]
pub enum RobotMessage {
    /// Un robot scout a découvert une ressource à cette position.
    ResourceDiscovered {
        from: RobotId,
        position: Position,
        kind: ResourceType,
    },
    /// Un robot scout a découvert un obstacle à cette position.
    ObstacleDiscovered { from: RobotId, position: Position },
    /// Un robot collector a déposé une unité de ressource à la base.
    ResourceDeposited { from: RobotId, kind: ResourceType },
    /// Un robot collector a prélevé une unité sur une ressource (pour mise à jour de la carte).
    ResourceCollected {
        from: RobotId,
        position: Position,
        kind: ResourceType,
    },
}
