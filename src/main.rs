// Point d'entrée de l'application : déclaration des modules et lancement.

mod base;
mod collector;
mod map;
mod messages;
mod robot;
mod scout;
mod simulation;
mod ui;

use map::{Cell, Map, ResourceType};

fn main() {
    let mut carte = Map::new(40, 20);
    carte.generate(0.2, 10);

    for y in 0..carte.height {
        for x in 0..carte.width {
            let symbole = match &carte.cells[y][x] {
                Cell::Empty => '.',
                Cell::Obstacle => 'O',
                Cell::Base => '#',
                Cell::Resource(r) => match r.kind {
                    ResourceType::Energy => 'E',
                    ResourceType::Crystal => 'C',
                },
            };
            print!("{}", symbole);
        }
        println!();
    }
}
