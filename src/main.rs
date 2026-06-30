// Point d'entrée de l'application : déclaration des modules et lancement.

mod base;
mod collector;
mod map;
mod messages;
mod robot;
mod scout;
mod simulation;
mod ui;

use std::time::Duration;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use base::Base;
use collector::Collector;
use map::{Map, Position, ResourceType};
use scout::Scout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // — Initialisation de la carte et des entités —
    let mut carte = Map::new(60, 30);
    carte.generate(0.2, 15);

    // Placement de la base sur la carte
    let base_pos = Position::new(carte.width / 2, carte.height / 2);
    // Instanciation de la base avec la position
    let mut base = Base::new(base_pos);

    // Deux scouts et deux collectors placés à la base à l'instanciation.
    let mut scouts = vec![Scout::new(0, base_pos), Scout::new(1, base_pos)];
    let mut collectors = vec![Collector::new(2, base_pos), Collector::new(3, base_pos)];

    // Initialisation du terminal Ratatui —
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Vider les événements bufférisés avant le démarrage (évite une sortie immédiate).
    while event::poll(Duration::from_millis(0))? {
        event::read()?;
    }

    // Boucle de simulation
    loop {
        // Rendu de l'interface.
        terminal.draw(|frame| {
            ui::draw(frame, &carte, &scouts, &collectors, &base);
        })?;

        // Toute touche clavier quitte la simulation.
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        // Pour chaque scout : déplacement + observation
        for scout in &mut scouts {
            // Déplacement aléatoire du scout
            scout.move_randomly(&carte);
            // Observe les cases voisines après le déplacement.
            scout.observe(&carte);
        }

        // Les scouts convertissent leurs découvertes en messages et les envoient à la base.
        for scout in &mut scouts {
            for msg in scout.flush_discoveries() {
                // La base agrège les informations reçues.
                base.receive_message(msg);
            }
        }

        // Les collectors avancent, collectent et déchargent à la base
        for collector in &mut collectors {
            let msgs = collector.state_change(&mut carte, &base.known_resources, base_pos);
            // Les messages de collecte et dépôt sont transmis à la base.
            for msg in msgs {
                base.receive_message(msg);
            }
        }
    }

    // — Restauration du terminal —
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Bilan final affiché dans le terminal normal.
    println!("=== Simulation terminée ===");
    println!("Energie collectée  : {}", base.total(ResourceType::Energy));
    println!("Cristaux collectés : {}", base.total(ResourceType::Crystal));

    Ok(())
}
