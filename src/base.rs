// Base centrale : stockage des ressources collectées, hub de connaissances partagées.

use crate::map::{Position, ResourceType};
use crate::messages::RobotMessage;
use std::collections::HashMap;

/// La base centrale : point de départ, stockage, et futur centre de communication.
pub struct Base {
    /// Position de la base sur la carte (centre).
    pub position: Position,
    /// Ressources collectées, indexées par type.
    pub stored: HashMap<ResourceType, u32>,
    /// Ressources découvertes par les scouts : position → type.
    pub known_resources: HashMap<Position, ResourceType>,
}

impl Base {
    /// Instanciation de la base
    pub fn new(position: Position) -> Self {
        let mut stored = HashMap::new();
        // On initialise les deux compteurs à zéro dès la création.
        stored.insert(ResourceType::Energy, 0);
        stored.insert(ResourceType::Crystal, 0);

        Self {
            position,
            stored,
            known_resources: HashMap::new(),
        }
    }

    /// Ajoute une unité de ressource au stock de la base.
    pub fn deposit(&mut self, kind: ResourceType) {
        let count = self.stored.entry(kind).or_insert(0);
        *count += 1;
    }

    /// Retourne le total collecté pour un type de ressource.
    pub fn total(&self, kind: ResourceType) -> u32 {
        *self.stored.get(&kind).unwrap_or(&0)
    }

    /// Traite un message reçu d'un robot et met à jour l'état de la base.
    pub fn receive_message(&mut self, message: RobotMessage) {
        match message {
            // Un scout a découvert une ressource : on l'ajoute aux connaissances.
            RobotMessage::ResourceDiscovered { position, kind, .. } => {
                self.known_resources.entry(position).or_insert(kind);
            }
            // Un scout a découvert un obstacle : on le note (futur pathfinding).
            RobotMessage::ObstacleDiscovered { .. } => {
                // Stockage des obstacles prévu au commit 11 si nécessaire.
            }
            // Un collector a déposé une ressource : on incrémente le stock.
            RobotMessage::ResourceDeposited { kind, .. } => {
                self.deposit(kind);
            }
            // Un collector a prélevé : on retire la ressource des connaissances si épuisée.
            RobotMessage::ResourceCollected { position, .. } => {
                // Mise à jour fine de la carte prévue au commit 14.
                let _ = position;
            }
        }
    }
}
