// Boucle de simulation, état global, orchestration des robots et de la carte.

use std::collections::HashSet;
use std::sync::mpsc;

use crate::base::Base;
use crate::collector::{Collector, CollectorState};
use crate::map::{Map, Position};
use crate::messages::RobotMessage;
use crate::scout::Scout;

/// État global de la simulation : carte, robots, base, canal de messages.
pub struct Simulation {
    pub map: Map,
    pub scouts: Vec<Scout>,
    pub collectors: Vec<Collector>,
    pub base: Base,
    /// les robots envoient leurs messages ici.
    tx: mpsc::Sender<RobotMessage>,
    /// la base lit les messages ici.
    rx: mpsc::Receiver<RobotMessage>,
}

impl Simulation {
    /// Crée et initialise la simulation complète.
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map::new(width, height);
        map.generate(0.2, 15);

        let base_pos = Position::new(width / 2, height / 2);
        let base = Base::new(base_pos);

        // Création du canal mpsc : tx pour envoyer, rx pour recevoir.
        let (tx, rx) = mpsc::channel();

        // Deux scouts et deux collectors démarrent tous à la base.
        let scouts = vec![Scout::new(0, base_pos), Scout::new(1, base_pos)];
        let collectors = vec![Collector::new(2, base_pos), Collector::new(3, base_pos)];

        Self {
            map,
            scouts,
            collectors,
            base,
            tx,
            rx,
        }
    }

    /// Exécute un tick complet de la simulation.
    /// Appelé depuis le thread de simulation
    pub fn tick(&mut self) {
        // On clone le tx pour que chaque robot puisse envoyer sans s'approprier l'original.
        let tx = self.tx.clone();

        // Scouts : déplacement + observation + envoi des découvertes
        for scout in &mut self.scouts {
            scout.move_randomly(&self.map);
            scout.observe(&self.map);
            // flush_discoveries() vide le Vec et retourne les messages à envoyer.
            for msg in scout.flush_discoveries() {
                tx.send(msg).ok();
            }
        }

        // Collectors : avance d'un pas + collecte + retour base
        // On calcule d'abord les positions déjà réservées
        let mut targeted: HashSet<Position> = self
            .collectors
            .iter()
            .filter_map(|c| match c.state {
                CollectorState::MovingToResource(pos) => Some(pos),
                CollectorState::Collecting(pos) => Some(pos),
                _ => None,
            })
            .collect();

        for collector in &mut self.collectors {
            let msgs = collector.state_change(
                &mut self.map,
                &self.base.known_resources,
                self.base.position,
                &targeted,
            );
            // Si ce collector vient de choisir une cible, on l'ajoute immédiatement
            // pour que le prochain collector dans la boucle ne la prenne pas.
            if let CollectorState::MovingToResource(pos) = collector.state {
                targeted.insert(pos);
            }
            for msg in msgs {
                tx.send(msg).ok();
            }
        }

        // Base : traitement asynchrone des messages reçus —
        while let Ok(msg) = self.rx.try_recv() {
            self.base.receive_message(msg);
        }
    }
}
