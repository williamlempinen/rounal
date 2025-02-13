use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::ui::ui::View;

pub const GLOBAL_MARGIN: u16 = 1;

pub fn get_logs_title(priority: &u8) -> String {
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

pub fn create_list_item(index: usize, current_line: usize, service: String) -> ListItem<'static> {
    let style = if index == current_line.clone() {
        Style::default()
            .fg(Color::Rgb(5, 94, 207))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    ListItem::new(service).style(style)
}

pub fn get_priority_color(priority: &u8) -> Style {
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
