use crate::app::App;

use crate::core::{
    error::Result,
    journal::JournalLog,
    system::{ServiceUnitFiles, ServiceUnits},
};

use crate::ui::{
    layouts::center,
    styles::{create_list_item, services_title, GLOBAL_MARGIN},
};
use crate::util::map_to_priority_str;

use log::info;

use ratatui::widgets::Wrap;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget},
    Frame,
};

// logs view could be added here
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    ServiceUnits,
    ServiceUnitFiles,
}

#[derive(Debug, Clone)]
pub enum CurrentLine {
    Log(JournalLog),
    ServiceUnit(ServiceUnits),
    ServiceUnitFile(ServiceUnitFiles),
}

#[derive(Debug)]
pub struct UI {
    pub view: View,
    pub is_showing_help: bool,
    pub is_showing_line_in_modal: bool,
    pub is_in_logs: bool,
    pub selected_priority: Option<u8>,
    pub current_line: usize,
}

impl UI {
    pub fn new() -> Self {
        Self {
            view: View::ServiceUnits,
            is_showing_help: false,
            is_showing_line_in_modal: false,
            is_in_logs: false,
            selected_priority: Some(5),
            current_line: 0,
        }
    }

    pub fn toggle_help(&mut self) {
        self.is_showing_help = !self.is_showing_help;
    }

    pub fn toggle_logs(&mut self) {
        self.is_in_logs = !self.is_in_logs;
    }

    pub fn set_view(&mut self, new_view: View) {
        self.view = new_view;
    }

    pub fn set_is_showing_help(&mut self, state: bool) {
        self.is_showing_help = state;
    }

    pub fn set_is_showing_line_in_modal(&mut self, state: bool) {
        self.is_showing_line_in_modal = state;
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.selected_priority = Some(priority);
        self.current_line = 0;
    }

    pub fn set_current_line(&mut self, position: usize) {
        self.current_line = position;
    }

    pub fn move_cursor_down(&mut self, max: usize) {
        if self.current_line < max.saturating_sub(1) {
            self.current_line += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.current_line > 0 {
            self.current_line -= 1;
        }
    }

    pub fn get_current_line(&self, app: &App) -> Option<CurrentLine> {
        if self.is_in_logs {
            return app
                .logs
                .as_ref()?
                .lock()
                .ok()?
                .get(self.selected_priority.as_ref()?)?
                .get(self.current_line)
                .map(|log| CurrentLine::Log(log.clone()));
        } else {
            if let Some((u, f)) = app.services.as_ref() {
                let service_line = match self.view {
                    View::ServiceUnits => u
                        .get(self.current_line)
                        .map(|unit| CurrentLine::ServiceUnit(unit.clone())),
                    View::ServiceUnitFiles => f
                        .get(self.current_line)
                        .map(|file| CurrentLine::ServiceUnitFile(file.clone())),
                };

                return service_line;
            }
        };
        None
    }

    pub fn yank_log_message(&self, app: &App) -> Option<String> {
        match self.get_current_line(app) {
            line => match line {
                Some(CurrentLine::Log(l)) => Some(format!("{:?}", l.log_message)),
                _ => None,
            },
        }
    }
}

fn render_after_clear<T: Widget>(f: &mut Frame<'_>, clearable: Rect, w: T) {
    f.render_widget(Clear, clearable);
    f.render_widget(w, clearable);
}

// handle the result/error
pub fn draw_ui(frame: &mut Frame<'_>, app: &mut App) -> Result<()> {
    info!("ENTER DRAW_UI");

    let config = app.config.clone();

    let terminal_layout = Layout::default()
        .margin(GLOBAL_MARGIN)
        .direction(Direction::Vertical)
        .constraints(&[Constraint::Percentage(97), Constraint::Percentage(3)])
        .split(frame.area());
    let content_area = terminal_layout
        .get(0)
        .expect("Error getting terminal layout")
        .clone();
    let action_area = terminal_layout
        .get(1)
        .expect("Error getting instructions")
        .clone();
    let width = frame.size().width as usize;

    let display_lines = frame.area().height.saturating_sub(6) as usize;
    let scroll_offset = if app.ui.current_line >= display_lines - 2 {
        app.ui.current_line - (display_lines - 3)
    } else {
        0
    };

    if app.ui.is_in_logs {
        let priority = &app.ui.selected_priority.unwrap_or_default();
        let priority_str = map_to_priority_str(priority);
        let priority_style = Style::default().fg(config.get_priority_color(&priority_str));

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
                            app.ui.current_line,
                            format!("[{}] {} - {}", log.timestamp, log.hostname, log.log_message),
                        )
                    })
                    .collect()
            } else {
                vec![ListItem::new("No logs available").style(priority_style)]
            }
        } else {
            vec![ListItem::new("No logs available").style(priority_style)]
        };

        let logs_list = List::new(logs_items)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(format!(
                        "  Logs with priority {}/{}  ",
                        priority, priority_str
                    ))
                    .style(priority_style),
            )
            .style(priority_style);

        render_after_clear(frame, content_area, logs_list);
    } else {
        let services: Vec<String> = match &app.services {
            Some((units, unit_files)) => {
                if app.ui.view == View::ServiceUnits {
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

        let service_items: Vec<ListItem> = services
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(display_lines)
            .map(|(idx, service_name)| {
                create_list_item(idx, app.ui.current_line, service_name.clone())
            })
            .collect();

        let list = List::new(service_items)
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(services_title(app.ui.view.clone())),
            )
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        render_after_clear(frame, content_area, list);
    }

    let i_txt = " -- Press [?] for help -- ";
    let i = Paragraph::new(i_txt)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    render_after_clear(frame, action_area, i);

    Ok(())
}

pub fn draw_help_modal(frame: &mut Frame<'_>) -> Result<()> {
    let area = center(frame.area(), Constraint::Max(40), Constraint::Max(20));

    let help_text = "Rounal - Key Mappings\n\n\
        Move: [hjkl / arrow keys]\n\
        Select: [Enter]\n\
        Close logs: [c]\n\
        Change priority: [1-7] or [Move]\n\
        See line in a modal: [K]\n\
        Yank message: [y] \n\
        Begin search: [/] \n\
        Quit: [q / Esc]\n\
        Toggle Help: [?]\n";

    let help_modal = Paragraph::new(help_text)
        .block(
            Block::bordered().title(" Help ").style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .alignment(Alignment::Center);

    render_after_clear(frame, area, help_modal);

    Ok(())
}

pub fn draw_whole_line(frame: &mut Frame<'_>, app: &App) -> Result<()> {
    let area = center(frame.area(), Constraint::Max(70), Constraint::Max(50));

    let content: Vec<Line> = if let Some(line) = app.ui.get_current_line(&app) {
        match line {
            CurrentLine::Log(log) => {
                info!("Log: {:?}", log);
                vec![
                    Line::from(Span::styled(
                        format!("\t[{:?}]\n\n", log.timestamp),
                        Style::default().fg(app.config.get_palette_color("red")),
                    )),
                    Line::from(Span::styled(
                        format!("Host: {:?}\n\n", log.hostname),
                        Style::default().fg(app.config.get_palette_color("red")),
                    )),
                    Line::from(Span::styled(
                        format!("Service: {:?}\n\n", log.service),
                        Style::default().fg(app.config.get_palette_color("red")),
                    )),
                    Line::from(Span::styled(
                        format!("Message: {:?}", log.log_message),
                        Style::default().fg(app.config.get_palette_color("red")),
                    )),
                ]
            }
            CurrentLine::ServiceUnit(unit) => {
                info!("Unit: {:?}", unit);
                vec![
                    Line::from(Span::styled(
                        format!("\t{:?}", unit.name),
                        Style::default().fg(app.config.get_palette_color("green")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", unit.sub),
                        Style::default().fg(app.config.get_palette_color("green")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", unit.load),
                        Style::default().fg(app.config.get_palette_color("green")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", unit.active),
                        Style::default().fg(app.config.get_palette_color("green")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", unit.description),
                        Style::default().fg(app.config.get_palette_color("green")),
                    )),
                ]
            }
            CurrentLine::ServiceUnitFile(file) => {
                info!("File: {:?}", file);
                vec![
                    Line::from(Span::styled(
                        format!("\t{:?}", file.name),
                        Style::default().fg(app.config.get_palette_color("blue")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", file.state),
                        Style::default().fg(app.config.get_palette_color("blue")),
                    )),
                    Line::from(Span::styled(
                        format!("\t{:?}", file.preset),
                        Style::default().fg(app.config.get_palette_color("blue")),
                    )),
                ]
            }
        }
    } else {
        vec![Line::from(Span::styled(
            " No line to present ",
            Style::default().fg(app.config.get_palette_color("blue")),
        ))]
    };

    let line_fully_modal = Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .block(
            Block::bordered().title(" Entry as a whole ").style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .alignment(Alignment::Left);

    render_after_clear(frame, area, line_fully_modal);
    Ok(())
}
