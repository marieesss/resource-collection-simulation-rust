// Point d'entrée de l'application : déclaration des modules et lancement.

mod base;
mod collector;
mod map;
mod messages;
mod robot;
mod scout;
mod simulation;
mod ui;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use map::ResourceType;
use simulation::Simulation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Création de la simulation
    let simulation = Arc::new(Mutex::new(Simulation::new(60, 30)));

    // Arc::clone crée un nouveau pointeur vers le même Mutex<Simulation>.
    let sim_thread = Arc::clone(&simulation);
    thread::spawn(move || {
        loop {
            // On verrouille le Mutex le temps d'un tick, puis on le relâche.
            {
                let mut sim = sim_thread.lock().unwrap();
                sim.tick();
            }

            // Pause entre deux ticks : vitesse de simulation.
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Initialisation du terminal Ratatui
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Vider les événements bufférisés avant le démarrage (évite une sortie immédiate).
    while event::poll(Duration::from_millis(0))? {
        event::read()?;
    }

    loop {
        // Verrouille brièvement pour lire l'état et dessiner.
        {
            let sim = simulation.lock().unwrap();
            terminal.draw(|frame| {
                ui::draw(frame, &sim.map, &sim.scouts, &sim.collectors, &sim.base);
            })?;
        } // Verrou relâché → thread simulation peut reprendre.

        // Toute touche clavier quitte la simulation.
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        // Courte pause pour ne pas saturer le CPU du thread UI.
        thread::sleep(Duration::from_millis(50));
    }

    // Restauration du terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Bilan final dans le terminal normal.
    let sim = simulation.lock().unwrap();
    println!("=== Simulation terminée ===");
    println!(
        "Energie collectée  : {}",
        sim.base.total(ResourceType::Energy)
    );
    println!(
        "Cristaux collectés : {}",
        sim.base.total(ResourceType::Crystal)
    );

    Ok(())
}
