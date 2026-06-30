// Base centrale : stockage des ressources collectées, hub de connaissances partagées.

use std::collections::HashMap;
use crate::map::{Position, ResourceType};

/// La base centrale : point de départ, stockage, et futur centre de communication.
pub struct Base {
    /// Position de la base sur la carte (centre de la carte).
    pub position: Position,
    /// Ressources collectées, indexées par type.
    pub stored: HashMap<ResourceType, u32>,
    /// Ressources découvertes par les scouts : position → type.
    /// Sera rempli via le système de messages (commit 10-11).
    pub known_resources: HashMap<Position, ResourceType>,
}

impl Base {
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
}
