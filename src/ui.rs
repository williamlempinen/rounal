use crate::app::{App, ServiceView};
use crate::Result;

use log::info;

use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::Widget;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

const GLOBAL_MARGIN: u16 = 1;

fn render_after_clear<T: Widget>(f: &mut Frame<'_>, c: Rect, w: T) {
    f.render_widget(Clear, c);
    f.render_widget(w, c);
}

fn get_priority_color(p: &u8) -> Style {
    match p {
        1 => Style::default()
            .fg(Color::Rgb(211, 10, 39))
            .add_modifier(Modifier::BOLD),
        2 => Style::default()
            .fg(Color::Rgb(198, 19, 22))
            .add_modifier(Modifier::BOLD),
        3 => Style::default().fg(Color::Rgb(206, 70, 6)),
        4 => Style::default().fg(Color::Rgb(235, 82, 5)),
        5 => Style::default().fg(Color::Yellow),
        6 => Style::default().fg(Color::Green),
        7 => Style::default().fg(Color::Blue),
        _ => Style::default().fg(Color::White),
    }
}

fn get_logs_title(p: &u8) -> String {
    let postfix = match p {
        1 => "emerg",
        2 => "alert",
        3 => "err",
        4 => "warning",
        5 => "notice",
        6 => "info",
        7 => "debug",
        _ => "unknown",
    };
    format!("  Logs with priority {}/{}  ", p, postfix)
}

// handle the result/error
pub fn draw_ui(frame: &mut Frame<'_>, app: &App) -> Result<()> {
    info!("ENTER DRAW_UI");
    let terminal_layout = Layout::default()
        .margin(GLOBAL_MARGIN)
        .direction(Direction::Vertical)
        .constraints(&[Constraint::Min(0)])
        .split(frame.area());
    let terminal = terminal_layout
        .get(0)
        .expect("Error getting terminal layout")
        .clone();

    let display_lines = frame.area().height as usize;
    let scroll_offset = if app.current_line >= display_lines - 2 {
        app.current_line - (display_lines - 3)
    } else {
        0
    };

    if app.is_modal {
        info!("DRAW_UI -> is modal");

        let priority = &app.selected_priority.unwrap_or_default();

        let logs_display = if let Some(logs_arc) = app.logs.as_ref() {
            let logs_map = logs_arc.lock().unwrap();
            logs_map
                .get(priority)
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

        let modal_content = Paragraph::new(logs_display)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(get_logs_title(priority))
                    .style(get_priority_color(priority)),
            )
            .style(get_priority_color(&0)); // use unknown to make white

        render_after_clear(frame, terminal, modal_content);
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

        render_after_clear(frame, terminal, list);
    }

    Ok(())
}
