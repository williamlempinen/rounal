use crate::app::{App, ServiceView};
use crate::layouts::center;
use crate::Result;
use log::{info, warn};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

// handle the result/error
pub fn draw_ui(frame: &mut Frame<'_>, app: &App) -> Result<()> {
    info!("ENTER DRAW_UI");
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(0)].as_ref())
        .split(frame.area());

    let display_lines = chunks.get(0).unwrap().height as usize;
    let scroll_offset = if app.current_line >= display_lines - 2 {
        app.current_line - (display_lines - 3)
    } else {
        0
    };

    warn!("-- APP-- {:?}", app);

    if app.is_modal {
        info!("DRAW_UI -> is modal");
        let logs_display = if let Some(logs_arc) = app.logs.as_ref() {
            let logs_map = logs_arc.lock().unwrap();
            logs_map
                .get(&app.selected_priority.unwrap_or_default())
                .map(|logs| {
                    logs.iter()
                        .map(|log| {
                            format!("[{}] {} - {}", log.timestamp, log.hostname, log.log_message)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_else(|| "No logs available".to_string())
        } else {
            "No logs available".to_string()
        };

        let modal_content = Paragraph::new(logs_display).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "Logs with priority {}",
                    app.selected_priority.unwrap_or_default()
                ))
                .style(Style::default().fg(Color::White)),
        );

        let modal = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(frame.area())[1];

        frame.render_widget(modal_content, modal);
    } else {
        info!("DRAW_UI -> no modal");
        let services: Vec<String> = match &app.services {
            Some((units, unit_files)) => {
                if app.selected_service_view == ServiceView::Units {
                    units.iter().map(|u| u.name.clone()).collect()
                } else {
                    unit_files.iter().map(|f| f.name.clone()).collect()
                }
            }
            None => vec![],
        };

        let items: Vec<ListItem> = services
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(display_lines)
            .map(|(i, service_name)| {
                let style = if i == app.current_line {
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(service_name.clone()).style(style)
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
    }

    Ok(())
}

pub fn render_modal(frame: &mut Frame) {
    info!("render modal fn");
    let area = center(
        frame.area(),
        Constraint::Percentage(20),
        Constraint::Length(3),
    );

    let modal =
        Paragraph::new("Content").block(Block::default().borders(Borders::ALL).title("Modal"));

    frame.render_widget(Clear, area);
    frame.render_widget(modal, area);
}
