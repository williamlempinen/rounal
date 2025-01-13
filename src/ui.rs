use crate::journal::get_logs;
use crate::Result;
use crate::{app::App, AppError};

use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// handle the result/error
pub fn draw_ui(frame: &mut Frame<'_>, app: &App) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
        .split(frame.area());

    if app.is_modal {
        let logs = app
            .logs
            .as_ref()
            .map(|logs| {
                logs.iter()
                    .map(|log| format!("{:?}", log))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_else(|| "No logs available".to_string());

        let logs_modal = Paragraph::new(logs).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Logs with {} priority",
                    app.selected_priority.unwrap_or_default()
                ))
                .style(Style::default().fg(Color::White)),
        );

        let modal = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(frame.area())[1];

        frame.render_widget(logs_modal, modal);
    } else {
        let services = app.services.read().map_err(|e| {
            AppError::UnexpectedError(format!("Error getting lock for services: {}", e))
        })?;

        let items: Vec<ListItem> = services
            .iter()
            .enumerate()
            .map(|(i, service)| {
                let style = if i == app.current_line {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(service.name.clone()).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Services"))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(list, chunks[0]);

        // Render the footer
        let footer = Paragraph::new("Press 'q' to quit | '↑/↓' to navigate | 'Enter' to view logs")
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[1]);
    }

    Ok(())
}
