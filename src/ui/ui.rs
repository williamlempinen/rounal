use crate::app::{App, ServiceView};

use crate::core::error::Result;

use log::info;

use crate::ui::styles::{
    create_list_item, get_logs_title, get_priority_color, services_title, GLOBAL_MARGIN,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Widget,
    },
    Frame,
};

#[derive(Debug)]
pub enum View {
    ServiceUnits,
    ServiceUnitFiles,
    Logs,
}

#[derive(Debug)]
pub struct UI {
    pub view: View,
    pub is_looking_help: bool,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub verical_scroll: usize,
    pub horizontal_scroll: usize,
}

fn render_after_clear<T: Widget>(f: &mut Frame<'_>, clearable: Rect, w: T) {
    f.render_widget(Clear, clearable);
    f.render_widget(w, clearable);
}

// handle the result/error
pub fn draw_ui(frame: &mut Frame<'_>, app: &App) -> Result<()> {
    info!("ENTER DRAW_UI");
    let terminal_layout = Layout::default()
        .margin(GLOBAL_MARGIN)
        .direction(Direction::Vertical)
        .constraints(&[Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.area());
    let content_area = terminal_layout
        .get(0)
        .expect("Error getting terminal layout")
        .clone();
    let action_area = terminal_layout
        .get(1)
        .expect("Error getting instructions")
        .clone();

    let display_lines = frame.area().height.saturating_sub(6) as usize;
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
                vec![
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                    ListItem::new("No logs available").style(get_priority_color(&0)),
                ]
            }
        } else {
            vec![
                ListItem::new("No logs available").style(get_priority_color(&0)),
                ListItem::new("No logs available").style(get_priority_color(&0)),
                ListItem::new("No logs available").style(get_priority_color(&0)),
                ListItem::new("No logs available").style(get_priority_color(&0)),
                ListItem::new("No logs available").style(get_priority_color(&0)),
                ListItem::new("No logs available").style(get_priority_color(&0)),
            ]
        };

        let logs_list = List::new(logs_items)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(get_logs_title(priority))
                    .style(get_priority_color(priority)),
            )
            .style(get_priority_color(&0));

        render_after_clear(frame, content_area, logs_list);
    } else {
        let services: Vec<String> = match &app.services {
            Some((units, unit_files)) => {
                if app.selected_service_view == ServiceView::Units {
                    units
                        .iter()
                        .map(|u| {
                            format!(
                                "{}: {} -- States [{:?} {:?} {:?}]",
                                u.name, u.description, u.active, u.sub, u.load
                            )
                        })
                        .collect()
                } else {
                    unit_files
                        .iter()
                        .map(|f| format!("{} {:?} {:?}", f.name, f.state, f.preset))
                        .collect()
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

        render_after_clear(frame, content_area, list);
    }

    let i_txt =
        "Move-[hjkl/arrows] | Select-[Enter] | Close logs-[c] | Change priority-[1-7] | Quit-[q/Esc]";
    let i = Paragraph::new(i_txt)
        .block(Block::bordered().title("Help"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    render_after_clear(frame, action_area, i);

    Ok(())
}
