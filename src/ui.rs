use crate::app::{App, ServiceView};
use crate::Result;

use log::info;

use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::Widget;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Clear, List, ListItem},
    Frame,
};

const GLOBAL_MARGIN: u16 = 1;

fn render_after_clear<T: Widget>(f: &mut Frame<'_>, clearable: Rect, w: T) {
    f.render_widget(Clear, clearable);
    f.render_widget(w, clearable);
}

fn get_priority_color(priority: &u8) -> Style {
    match priority {
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

fn get_logs_title(priority: &u8) -> String {
    let postfix = match priority {
        1 => "emerg",
        2 => "alert",
        3 => "err",
        4 => "warning",
        5 => "notice",
        6 => "info",
        7 => "debug",
        _ => "unknown",
    };
    format!("  Logs with priority {}/{}  ", priority, postfix)
}

fn services_title(view: ServiceView) -> String {
    match view {
        ServiceView::Units => "  Service units  ".to_string(),
        ServiceView::UnitFiles => "  Service unit files  ".to_string(),
    }
}

fn create_list_item(index: usize, current_line: usize, service: String) -> ListItem<'static> {
    let style = if index == current_line.clone() {
        Style::default()
            .fg(Color::Rgb(5, 94, 207))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    ListItem::new(service).style(style)
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

    let display_lines = frame.area().height.saturating_sub(2) as usize;
    let scroll_offset = if app.current_line >= display_lines - 2 {
        app.current_line - (display_lines - 3)
    } else {
        0
    };

    if app.is_in_logs {
        let priority = &app.selected_priority.unwrap_or_default();

        let logs_items: Vec<ListItem> = if let Some(logs_arc) = &app.logs {
            let logs_map = logs_arc.lock().unwrap();
            if let Some(log_entries) = logs_map.get(priority) {
                log_entries
                    .iter()
                    .enumerate()
                    .skip(scroll_offset)
                    .take(display_lines)
                    .map(|(idx, log)| {
                        create_list_item(
                            idx,
                            app.current_line.clone(),
                            format!("[{}] {} - {}", log.timestamp, log.hostname, log.log_message),
                        )
                    })
                    .collect()
            } else {
                vec![ListItem::new("No logs available").style(get_priority_color(&0))]
            }
        } else {
            vec![ListItem::new("No logs available").style(get_priority_color(&0))]
        };

        let logs_list = List::new(logs_items)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(get_logs_title(priority))
                    .style(get_priority_color(priority)),
            )
            .style(get_priority_color(&0));

        render_after_clear(frame, terminal, logs_list);
    } else {
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
            .map(|(idx, service_name)| {
                create_list_item(idx, app.current_line, service_name.clone())
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(services_title(app.selected_service_view.clone())),
            )
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        render_after_clear(frame, terminal, list);
    }

    Ok(())
}
