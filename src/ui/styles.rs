use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::{
    core::{
        config::Config,
        journal::JournalLog,
        system::{ServiceUnitFiles, ServiceUnits},
    },
    ui::ui::View,
};

pub const GLOBAL_MARGIN: u16 = 1;
pub const CURSOR_LEFT: &str = "▶";
pub const CURSOR_RIGHT: &str = "◀";

pub fn services_title(view: View) -> Line<'static> {
    let active = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(Color::DarkGray);

    let styles = match view {
        View::ServiceUnits => (active, inactive),
        View::ServiceUnitFiles => (inactive, active),
    };

    Line::from(vec![
        Span::styled(" Service units ", styles.0),
        Span::raw(" / "),
        Span::styled(" Service unit files ", styles.1),
    ])
}

pub fn create_log_list_item(
    index: usize,
    current_line: usize,
    log: &JournalLog,
    config: &Config,
) -> ListItem<'static> {
    let is_on_cursor = index == current_line;

    ListItem::from(Text::from(Line::from(vec![
        Span::styled(
            if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("[{}] ", log.timestamp),
            Style::default()
                .fg(config.get_palette_color("gray"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{} ", log.hostname),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("{} ", log.service),
            Style::default()
                .fg(config.get_palette_color("green"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("[{}] ", log.priority),
            Style::default().fg(config.get_priority_color(&log.priority.to_string())),
        ),
        Span::styled(
            format!("Message: {}", log.log_message),
            Style::default().fg(config.get_palette_color("red")),
        ),
        Span::styled(
            if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
    ])))
}

pub fn create_files_list_item(
    index: usize,
    current_line: usize,
    file: &ServiceUnitFiles,
    config: &Config,
) -> ListItem<'static> {
    let is_on_cursor = index == current_line;

    ListItem::from(Text::from(Line::from(vec![
        Span::styled(
            if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("[{}] ", file.name),
            Style::default()
                .fg(config.get_palette_color("gray"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{:?} ", file.state),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("{:?} ", file.preset),
            Style::default()
                .fg(config.get_palette_color("green"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
    ])))
}

pub fn create_units_list_item(
    index: usize,
    current_line: usize,
    unit: &ServiceUnits,
    config: &Config,
) -> ListItem<'static> {
    let is_on_cursor = index == current_line;

    ListItem::from(Text::from(Line::from(vec![
        Span::styled(
            if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("[{}] ", unit.name),
            Style::default()
                .fg(config.get_palette_color("gray"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{:?} ", unit.load),
            Style::default().fg(config.get_palette_color("blue")),
        ),
        Span::styled(
            format!("{:?} ", unit.active),
            Style::default()
                .fg(config.get_palette_color("green"))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("[{:?}] ", unit.sub),
            Style::default().fg(config.get_palette_color("red")),
        ),
        Span::styled(
            format!("Message: {}", unit.description),
            Style::default().fg(config.get_palette_color("red")),
        ),
        Span::styled(
            if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
            Style::default().fg(config.get_palette_color("blue")),
        ),
    ])))
}
