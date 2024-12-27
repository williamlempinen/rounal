use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, View};

pub fn draw_ui<B>(frame: &mut Frame<B>, app: &App)
where
    B: ratatui::backend::Backend, // Specify the trait bound for the backend
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(frame.area()); // Use `frame.get_area()` for the terminal size

    // Display logs or services based on the current view
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

    let content_widget = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Content"))
        .style(Style::default().fg(Color::White));

    frame.render_widget(content_widget, chunks[0]);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit | 'j' for journalctl | 's' for systemctl")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[1]);
}
