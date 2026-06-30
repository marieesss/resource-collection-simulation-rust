// Affichage Ratatui : couleurs, layout, rendu terminal en temps réel.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::base::Base;
use crate::collector::Collector;
use crate::map::{Cell, Map, Position, ResourceType};
use crate::scout::Scout;

/// Dessine l'interface complète : carte + panneau de statistiques.
pub fn draw(frame: &mut Frame, map: &Map, scouts: &[Scout], collectors: &[Collector], base: &Base) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    draw_map(frame, areas[0], map, scouts, collectors);
    draw_stats(frame, areas[1], base);
}

/// Dessine la carte avec les robots superposés.
fn draw_map(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    map: &Map,
    scouts: &[Scout],
    collectors: &[Collector],
) {
    // On construit les lignes de la carte une par une.
    let mut lines: Vec<Line> = Vec::new();

    for y in 0..map.height {
        let mut spans: Vec<Span> = Vec::new();

        for x in 0..map.width {
            let pos = Position::new(x, y);

            // Vérifie si un scout est sur cette case.
            let scout_here = scouts.iter().any(|s| s.position() == pos);
            // Vérifie si un collector est sur cette case.
            let collector_here = collectors.iter().any(|c| c.position() == pos);

            // Priorité d'affichage : robots > contenu de la cellule.
            let span = if scout_here {
                // Scout : x en rouge
                Span::styled("x", Style::default().fg(Color::Red))
            } else if collector_here {
                // Collector : o en magenta
                Span::styled("o", Style::default().fg(Color::Magenta))
            } else {
                // Sinon, on affiche la cellule avec sa couleur.
                match &map.cells[y][x] {
                    Cell::Empty => Span::raw("."),
                    // Obstacle : O
                    Cell::Obstacle => Span::styled("O", Style::default().fg(Color::Cyan)),
                    // Base : #
                    Cell::Base => Span::styled("#", Style::default().fg(Color::LightGreen)),
                    Cell::Resource(r) => match r.kind {
                        // Energie : E
                        ResourceType::Energy => {
                            Span::styled("E", Style::default().fg(Color::Green))
                        }
                        // Cristal : C
                        ResourceType::Crystal => {
                            Span::styled("C", Style::default().fg(Color::LightMagenta))
                        }
                    },
                }
            };
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }

    let carte_widget =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Carte"));

    frame.render_widget(carte_widget, area);
}

/// Dessine le panneau de statistiques en bas.
fn draw_stats(frame: &mut Frame, area: ratatui::layout::Rect, base: &Base) {
    let energie = base.total(ResourceType::Energy);
    let cristaux = base.total(ResourceType::Crystal);

    let texte = Line::from(vec![
        Span::raw("  "),
        Span::styled("Energie : ", Style::default().fg(Color::Green)),
        Span::styled(energie.to_string(), Style::default().fg(Color::Green)),
        Span::raw("   "),
        Span::styled("Cristaux : ", Style::default().fg(Color::LightMagenta)),
        Span::styled(
            cristaux.to_string(),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::raw("   "),
        Span::styled(
            "[Toute touche pour quitter]",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let stats_widget = Paragraph::new(texte).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Ressources collectées"),
    );

    frame.render_widget(stats_widget, area);
}
