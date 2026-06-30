// Logique des robots collecteurs : navigation vers ressources, collecte, retour base.

use crate::map::{Cell, Map, Position, ResourceType};
use crate::messages::RobotMessage;
use crate::robot::{Robot, RobotId, RobotKind};
use std::collections::{HashMap, VecDeque};

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
    /// Chemin calculé par BFS vers la cible (None si pas de chemin actif).
    pub path: Vec<Position>,
}

impl Collector {
    // Instanciation du robot collecteur
    pub fn new(id: RobotId, position: Position) -> Self {
        Self {
            robot: Robot::new(id, RobotKind::Collector, position),
            state: CollectorState::WaitingForResource,
            carrying: None,
            path: Vec::new(),
        }
    }

    // retourne position
    pub fn position(&self) -> Position {
        self.robot.position
    }

    /// Calcule un chemin simple entre start et goal (BFS).
    /// Retourne les positions à suivre, sans inclure start.
    pub fn find_path(start: Position, goal: Position, map: &Map) -> Vec<Position> {
        // VecDeque pour la file d'attente des positions à explorer
        let mut queue = VecDeque::new();
        // HashMap pour stocker les positions visitées et leur parent
        let mut visited = HashMap::new();

        // On commence par la position de départ
        queue.push_back(start);
        // On marque la position de départ comme visitée avec elle-même
        visited.insert(start, start);

        // pop_front pour traiter les positions dans l'ordre
        // Tant que la file n'est pas vide, on explore les voisins
        while let Some(current) = queue.pop_front() {
            // arrete si on arrive à la position goal
            if current == goal {
                break;
            }

            // On explore les 4 celulles autour (haut, bas, gauche, droite)
            let neighbors = [
                Position::new(current.x, current.y.saturating_sub(1)), // evite les -1 pour usize
                Position::new(current.x, current.y + 1),
                Position::new(current.x.saturating_sub(1), current.y), // evite les -1 pour usize
                Position::new(current.x + 1, current.y),
            ];

            for neighbor in neighbors {
                // Si la celulle a déjà été visitée, on l'ignore
                if visited.contains_key(&neighbor) {
                    continue;
                }

                // Si on peut se déplacer sur la celulle (pas hors carte)
                let Some(cell) = map.get(neighbor) else {
                    continue;
                };

                // Si la celulle est un obstacle, on l'ignore
                match cell {
                    Cell::Obstacle => continue,
                    // Sinon on l'ajoute à la file d'attente et on marque comme visitée
                    _ => {
                        visited.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Si la position goal n'a pas été visitée, on retourne un chemin vide
        if !visited.contains_key(&goal) {
            return Vec::new();
        }

        // On reconstruit le chemin en partant de la position goal et en remontant vers start
        let mut path = Vec::new();
        let mut current = goal;

        // Boucle pour remonter le chemin jusqu'à la position de départ
        while current != start {
            path.push(current);
            current = visited[&current];
        }

        // On inverse le chemin pour qu'il aille de start à goal
        path.reverse();
        path
    }

    /// Avance d'un pas sur le chemin pré-calculé.
    pub fn step_along_path(&mut self) {
        if !self.path.is_empty() {
            // retire et retourne le premier élément (le prochain pas)
            self.robot.position = self.path.remove(0);
        }
    }

    /// Logique complète pour un collector.
    /// Retourne les messages à envoyer à la base (collecte, dépôt).
    pub fn state_change(
        &mut self, // robot collecteur
        map: &mut Map,
        known_resources: &HashMap<Position, ResourceType>,
        base_pos: Position,
    ) -> Vec<RobotMessage> {
        // Messages générés ce tick (collecte ou dépôt).
        let mut messages = Vec::new();

        // Clone pour ne pas modifier la référence
        // On verifie l'état du robot
        match self.state.clone() {
            // Si le robot attend une ressource
            CollectorState::WaitingForResource => {
                // Cherche la première ressource connue par la base.
                if let Some((&target_pos, _)) = known_resources.iter().next() {
                    let path = Self::find_path(self.robot.position, target_pos, map);
                    if !path.is_empty() {
                        // attribution du chemin et changement d'état vers MovingToResource
                        self.path = path;
                        self.state = CollectorState::MovingToResource(target_pos);
                    }
                }
            }

            // Si le robot est en déplacement vers une ressource
            CollectorState::MovingToResource(target) => {
                // Si le robot est arrivé à la ressource, on change d'état vers Collecting
                if self.robot.position == target {
                    // Arrivé sur la ressource → passage à l'état Collecting.
                    self.state = CollectorState::Collecting(target);
                } else {
                    // Avance d'un pas vers la cible.
                    self.step_along_path();
                }
            }
            // Si le robot est en train de collecter une ressource
            CollectorState::Collecting(target) => {
                // On accède à la cellule cible et on prélève une unité.
                if let Cell::Resource(resource) = &mut map.cells[target.y][target.x] {
                    let kind = resource.kind;
                    // Retire une unité, retourne true si la ressource est épuisée.
                    let exhausted = resource.take_one();

                    // Le collector transporte maintenant la ressource.
                    self.carrying = Some(kind);

                    // On envoie un message à la base pour indiquer la collecte
                    messages.push(RobotMessage::ResourceCollected {
                        from: self.robot.id,
                        position: target,
                        kind,
                    });

                    // Si la ressource est épuisée, on vide la case sur la carte.
                    if exhausted {
                        map.cells[target.y][target.x] = Cell::Empty;
                    }
                }
                // Retour à la base après la collecte
                // goal = position de la base
                let path = Self::find_path(self.robot.position, base_pos, map);
                self.path = path;
                self.state = CollectorState::ReturningToBase;
            }

            // Si le robot est en train de retourner à la base
            CollectorState::ReturningToBase => {
                // Si le robot est arrivé à la base
                if self.robot.position == base_pos {
                    if let Some(kind) = self.carrying.take() {
                        // On envoie un message à la base pour indiquer le dépôt
                        messages.push(RobotMessage::ResourceDeposited {
                            from: self.robot.id,
                            kind,
                        });
                    }
                    // Retour à l'état d'attente pour le prochain cycle.
                    self.state = CollectorState::WaitingForResource;
                } else {
                    // Avance d'un pas vers la base.
                    self.step_along_path();
                }
            }
        }

        messages
    }
}
