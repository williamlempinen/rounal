use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::ui::ui::View;

pub const GLOBAL_MARGIN: u16 = 1;

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

pub fn get_logs_style() {}

pub fn get_service_units_style() {}

pub fn get_service_unit_files_style() {}
