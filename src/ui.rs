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
        // Length(4) = 2 bordures + 2 lignes de contenu
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(frame.area());

    draw_map(frame, areas[0], map, scouts, collectors);
    draw_stats(frame, areas[1], map, scouts, collectors, base);
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

            // Priorité d'affichage : robots > contenu de la cellule.
            // find() retourne le robot à cette position, symbol() donne son caractère.
            let span = if let Some(s) = scouts.iter().find(|s| s.position() == pos) {
                // Scout : symbol() retourne 'x', affiché en rouge.
                Span::styled(
                    s.robot.symbol().to_string(),
                    Style::default().fg(Color::Red),
                )
            } else if let Some(c) = collectors.iter().find(|c| c.position() == pos) {
                // Collector : symbol() retourne 'o', affiché en magenta.
                Span::styled(
                    c.robot.symbol().to_string(),
                    Style::default().fg(Color::Magenta),
                )
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

/// Dessine le panneau de statistiques en bas (2 lignes de contenu).
fn draw_stats(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    map: &Map,
    scouts: &[Scout],
    collectors: &[Collector],
    base: &Base,
) {
    // Somme des unités restantes sur toutes les ressources de la carte.
    let restantes: u32 = map
        .cells
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|cell| {
            if let Cell::Resource(r) = cell {
                Some(r.amount)
            } else {
                None
            }
        })
        .sum();

    // robots actifs + ressources restantes + obstacles découverts.
    let ligne1 = Line::from(vec![
        Span::raw("  "),
        Span::styled("Scouts : ", Style::default().fg(Color::Red)),
        Span::styled(scouts.len().to_string(), Style::default().fg(Color::Red)),
        Span::raw("   "),
        Span::styled("Collectors : ", Style::default().fg(Color::Magenta)),
        Span::styled(
            collectors.len().to_string(),
            Style::default().fg(Color::Magenta),
        ),
        Span::raw("   "),
        Span::styled(
            "Ressources restantes : ",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(restantes.to_string(), Style::default().fg(Color::Yellow)),
        Span::raw("   "),
        Span::styled("Obstacles découverts : ", Style::default().fg(Color::Cyan)),
        Span::styled(
            base.known_obstacles.len().to_string(),
            Style::default().fg(Color::Cyan),
        ),
    ]);

    // ressources collectées + aide clavier.
    let ligne2 = Line::from(vec![
        Span::raw("  "),
        Span::styled("Energie : ", Style::default().fg(Color::Green)),
        Span::styled(
            base.total(ResourceType::Energy).to_string(),
            Style::default().fg(Color::Green),
        ),
        Span::raw("   "),
        Span::styled("Cristaux : ", Style::default().fg(Color::LightMagenta)),
        Span::styled(
            base.total(ResourceType::Crystal).to_string(),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::raw("   "),
        Span::styled(
            "[Toute touche pour quitter]",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let stats_widget = Paragraph::new(vec![ligne1, ligne2])
        .block(Block::default().borders(Borders::ALL).title("Statistiques"));

    frame.render_widget(stats_widget, area);
}
