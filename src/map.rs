// Gestion de la carte : cellules, obstacles, ressources, génération procédurale.

use noise::{NoiseFn, Perlin};
use rand::Rng;

/// Coordonnée (colonne, ligne) sur la carte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// types de ressources possibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Energy,
    Crystal,
}

/// Ressource sur la carte
#[derive(Debug, Clone)]
pub struct Resource {
    pub kind: ResourceType,
    pub amount: u32,
}

impl Resource {
    pub fn new(kind: ResourceType, amount: u32) -> Self {
        Self { kind, amount }
    }

    /// Retire une unité (amount) et retourne true si la ressource est épuisée.
    pub fn take_one(&mut self) -> bool {
        if self.amount > 0 {
            self.amount -= 1;
        }
        self.amount == 0
    }
}

/// Contenu possible d'une cellule de la carte.
#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Obstacle,
    Resource(Resource),
    Base,
}

/// La carte du monde : grille de cellules indexée par [y][x].
pub struct Map {
    pub width: usize,
    pub height: usize,
    /// cells[y][x] = cellule à la ligne y, colonne x.
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    /// Crée une carte vide de dimensions données.
    pub fn new(width: usize, height: usize) -> Self {
        // Pour chaque ligne y, on crée un Vec de width cellules vides.
        let cells = (0..height)
            .map(|_| (0..width).map(|_| Cell::Empty).collect())
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    /// Lecture d'une cellule (retourne None si hors-carte).
    pub fn get(&self, pos: Position) -> Option<&Cell> {
        self.cells.get(pos.y)?.get(pos.x)
    }

    /// Écriture d'une cellule.
    pub fn set(&mut self, pos: Position, cell: Cell) {
        if pos.x < self.width && pos.y < self.height {
            self.cells[pos.y][pos.x] = cell;
        }
    }

    /// Retourne true si la position est dans les limites de la carte.
    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// Génère la carte : obstacles, ressources aléatoires, base centrale.
    /// obstacle_threshold : seuil Perlin au-dessus duquel une cellule devient obstacle
    /// resource_count : nombre de ressources Energy + Crystal à placer.
    pub fn generate(&mut self, obstacle_threshold: f64, resource_count: usize) {
        let mut rng = rand::thread_rng();

        // Seed aléatoire : chaque partie génère une carte différente.
        let seed: u32 = rng.gen_range(0..u32::MAX);
        let perlin = Perlin::new(seed);

        // Facteur d'échelle : valeur basse = grandes zones, valeur haute = bruit fin.
        let scale = 0.15;

        // — Placement des obstacles via Perlin —
        for y in 0..self.height {
            for x in 0..self.width {
                let is_center = x == self.width / 2 && y == self.height / 2;

                // On convertit (x, y) en coordonnées flottantes pour le bruit.
                let noise_val = perlin.get([x as f64 * scale, y as f64 * scale]);

                // Si le bruit dépasse le seuil → obstacle (zone naturellement groupée).
                if !is_center && noise_val > obstacle_threshold {
                    self.cells[y][x] = Cell::Obstacle;
                }
            }
        }

        // Placement des ressources
        let mut placed = 0;
        while placed < resource_count {
            // Choix aléatoire d'une position entre 0 et width/height
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            // Init position
            let pos = Position::new(x, y);

            // Verification que la celulle est vide
            if matches!(self.cells[y][x], Cell::Empty) {
                // Choix 50/50 entre les deux types de ressources
                let kind = if rng.gen_bool(0.5) {
                    ResourceType::Energy
                } else {
                    ResourceType::Crystal
                };
                // Quantité aléatoire entre 50 et 200
                let amount = rng.gen_range(50..=200);
                self.set(pos, Cell::Resource(Resource::new(kind, amount)));
                placed += 1;
            }
        }

        //Placement de la base au centre
        let base_pos = Position::new(self.width / 2, self.height / 2);
        self.set(base_pos, Cell::Base);
    }
}
