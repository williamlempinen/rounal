use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.area());

    // Convert services to a displayable string
    let services = app.services.read().unwrap();
    let content: String = services
        .iter()
        .map(|service| format!("{}\n", service.name)) // Assuming ServiceUnits has a `name` field
        .collect();

    // Render the services list
    let content_widget =
        Paragraph::new(content).block(Block::default().borders(Borders::ALL).title("Content"));
    frame.render_widget(content_widget, chunks[0]);

    // Render the footer
    let footer = Paragraph::new("Press 'q' to quit | 'j' for journalctl | 's' for systemctl")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[1]);
}
