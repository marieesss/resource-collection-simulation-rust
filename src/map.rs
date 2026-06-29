// Gestion de la carte : cellules, obstacles, ressources, génération procédurale.

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
        Self { width, height, cells }
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
}
