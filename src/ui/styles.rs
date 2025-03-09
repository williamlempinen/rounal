use crate::{
    core::{
        config::Config,
        journal::JournalLog,
        system::{ServiceUnitFiles, ServiceUnits},
    },
    ui::ui::View,
};
use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{ListItem, Paragraph},
};

use super::ui::UI;

pub(crate) const GLOBAL_MARGIN: u16 = 1;
pub(crate) const CURSOR_LEFT: &str = "▶";
pub(crate) const CURSOR_RIGHT: &str = "◀";

#[derive(Debug)]
pub struct Styler {
    pub config: Config,
}

impl Styler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub(crate) fn get_bottom_info(&self, ui: &UI) -> Paragraph<'static> {
        if ui.is_in_search_mode {
            return Paragraph::new(format!(" -- SEARCH MODE: {}", ui.search_query))
                .alignment(Alignment::Left)
                .style(Style::default().fg(self.config.get_palette_color("blue")));
        } else {
            return Paragraph::new(" -- Press [?] for help -- ")
                .alignment(Alignment::Center)
                .style(Style::default().fg(self.config.get_palette_color("white")));
        }
    }

    pub(crate) fn get_services_title(&self, view: View) -> Line<'static> {
        let active = Style::default()
            .fg(self.config.get_palette_color("green"))
            .add_modifier(Modifier::BOLD);
        let inactive = Style::default().fg(self.config.get_palette_color("gray"));

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

    pub(crate) fn create_log_list_item(
        &self,
        index: usize,
        current_line: usize,
        log: &JournalLog,
    ) -> ListItem<'static> {
        let is_on_cursor = index == current_line;

        ListItem::from(Text::from(Line::from(vec![
            Span::styled(
                if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("[{}] ", log.timestamp),
                Style::default()
                    .fg(self.config.get_palette_color("gray"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} ", log.hostname),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("{} ", log.service),
                Style::default()
                    .fg(self.config.get_palette_color("green"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("[{}] ", log.priority),
                Style::default().fg(self.config.get_priority_color(&log.priority.to_string())),
            ),
            Span::styled(
                format!("Message: {}", log.log_message),
                Style::default().fg(self.config.get_palette_color("red")),
            ),
            Span::styled(
                if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
        ])))
    }

    pub(crate) fn create_files_list_item(
        &self,
        index: usize,
        current_line: usize,
        file: &ServiceUnitFiles,
    ) -> ListItem<'static> {
        let is_on_cursor = index == current_line;

        ListItem::from(Text::from(Line::from(vec![
            Span::styled(
                if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("[{}] ", file.name),
                Style::default()
                    .fg(self.config.get_palette_color("gray"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:?} ", file.state),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("{:?} ", file.preset),
                Style::default()
                    .fg(self.config.get_palette_color("green"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
        ])))
    }

    pub fn create_units_list_item(
        &self,
        index: usize,
        current_line: usize,
        unit: &ServiceUnits,
    ) -> ListItem<'static> {
        let is_on_cursor = index == current_line;

        ListItem::from(Text::from(Line::from(vec![
            Span::styled(
                if is_on_cursor { CURSOR_LEFT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("[{}] ", unit.name),
                Style::default()
                    .fg(self.config.get_palette_color("gray"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:?} ", unit.load),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
            Span::styled(
                format!("{:?} ", unit.active),
                Style::default()
                    .fg(self.config.get_palette_color("green"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("[{:?}] ", unit.sub),
                Style::default().fg(self.config.get_palette_color("red")),
            ),
            Span::styled(
                format!("Message: {}", unit.description),
                Style::default().fg(self.config.get_palette_color("red")),
            ),
            Span::styled(
                if is_on_cursor { CURSOR_RIGHT } else { " " }.to_string(),
                Style::default().fg(self.config.get_palette_color("blue")),
            ),
        ])))
    }
}

//pub fn highlighted_line(line: &CurrentLine) -> ListItem<'static> {}
