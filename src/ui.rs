use crate::app::{App, View};
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

    let content = match app.current_view {
        View::Journalctl => {
            let logs = app.logs.read().unwrap();
            logs.join("\n")
        }
        View::Systemctl => {
            let services = app.services.read().unwrap();
            services.join("\n")
        }
    };

    let content_widget =
        Paragraph::new(content).block(Block::default().borders(Borders::ALL).title("Content"));
    frame.render_widget(content_widget, chunks[0]);

    let footer = Paragraph::new("Press 'q' to quit | 'j' for journalctl | 's' for systemctl")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[1]);
}
