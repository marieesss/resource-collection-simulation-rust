// Point d'entrée de l'application : déclaration des modules et lancement.

mod base;
mod collector;
mod map;
mod messages;
mod robot;
mod scout;
mod simulation;
mod ui;

use base::Base;
use collector::Collector;
use map::{Cell, Map, Position, ResourceType};
use scout::Scout;

fn afficher_carte(carte: &Map, scouts: &[Scout], collectors: &[Collector]) {
    for y in 0..carte.height {
        for x in 0..carte.width {
            let pos = Position::new(x, y);

            // Les robots ont la priorité d'affichage sur les cellules, renvoie x ou o
            let robot_char = scouts
                // pour chaque scout
                .iter()
                // on cherche si sa position correspond à la cellule en cours
                .find(|s| s.position() == pos)
                .map(|_| 'x')
                // si aucun scout n'est trouvé, on cherche un collector
                .or_else(|| collectors.iter().find(|c| c.position() == pos).map(|_| 'o'));

            // Afficher caractère (x ou o) correspondant à la cellule ou au robot
            let symbole = if let Some(c) = robot_char {
                c
            } else {
                // Sinon, on affiche le symbole de la cellule (., O, #, E, C)
                match &carte.cells[y][x] {
                    Cell::Empty => '.',
                    Cell::Obstacle => 'O',
                    Cell::Base => '#',
                    Cell::Resource(r) => match r.kind {
                        ResourceType::Energy => 'E',
                        ResourceType::Crystal => 'C',
                    },
                }
            };
            print!("{}", symbole);
        }
        println!();
    }
}

fn main() {
    let mut carte = Map::new(40, 20);
    carte.generate(0.2, 10);

    // Placement de la base sur la carte
    let base_pos = Position::new(carte.width / 2, carte.height / 2);
    // Instanciation de la base avec la position
    let _base = Base::new(base_pos);

    // Deux scouts et deux collectors placés à la base à l'instanciation.
    let mut scouts = vec![Scout::new(0, base_pos), Scout::new(1, base_pos)];
    let mut collectors = vec![Collector::new(2, base_pos), Collector::new(3, base_pos)];

    // 5 ticks pour vérifier que les scouts se déplacent.
    for tick in 0..5 {
        for scout in &mut scouts {
            scout.move_randomly(&carte);
        }
        afficher_carte(&carte, &scouts, &collectors);
        println!("Scout 0 : {:?}", scouts[0].position());
        println!("Scout 1 : {:?}", scouts[1].position());
    }
}
