use crate::ui::ui::{draw_docs_modal, draw_entry_line, draw_help_modal, draw_ui, View, UI};
use crate::{
    core::{
        config::Config,
        error::{Result, RounalError},
        input_handler::handle_key_events,
        journal::{get_journal_logs, JournalLogMap, SharedJournalLogs},
        system::{get_system_services, ServiceUnitFiles, ServiceUnits},
    },
    ui::styles::Styler,
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::info;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io::stdout,
    sync::{Arc, Mutex},
};

// TODO
#[derive(PartialEq)]
pub enum Events {
    Quit,
    GetLogs,
    GetHelp,
    GetLineInModal,
    Search,
    Docs,
}

// TODO:
//      - yanking -> clipboard content destroyed after exiting application
//      - explanations modal
//      - align items vertically and title columns
//      - responsive layout
//      - error handling -> no reason to panic every time
//      - filtering based on status (failed | running | exited)
//      - actions for: start | stop | status

#[derive(Debug)]
pub struct App {
    pub ui: UI,
    pub config: Config,
    pub is_running: bool,
    pub logs: Option<SharedJournalLogs>,
    pub services: Option<(Vec<ServiceUnits>, Vec<ServiceUnitFiles>)>,
    pub selected_service: Option<String>,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            ui: UI::new(),
            config,
            is_running: true,
            logs: None,
            services: None,
            selected_service: None,
        }
    }

    pub fn set_is_running(&mut self, state: bool) {
        self.is_running = state;
    }

    pub fn set_services(
        &mut self,
        services: (Vec<ServiceUnits>, Vec<ServiceUnitFiles>),
    ) -> Result<()> {
        self.services = Some(services);
        Ok(())
    }

    pub fn set_logs(&mut self, logs: Arc<Mutex<JournalLogMap>>) {
        self.logs = Some(logs);
    }

    pub fn clear_logs(&mut self) {
        self.logs = None;
    }

    pub fn reorder_lines(&mut self) {
        if self.ui.search_query.trim().is_empty() {
            return;
        }

        let q = self.ui.search_query.to_lowercase();

        if self.ui.is_in_logs {
            if let Some(logs_arc) = &self.logs {
                if let Ok(mut logs_map) = logs_arc.lock() {
                    let priority = self
                        .ui
                        .selected_priority
                        .unwrap_or(self.config.options.initial_priority);

                    if let Some(entries) = logs_map.get_mut(&priority) {
                        entries.sort_by_key(|entry| {
                            let time_service = format!(
                                "{} {}",
                                entry.timestamp.to_lowercase(),
                                entry.service.to_lowercase(),
                            );

                            if time_service.contains(&q) {
                                0
                            } else {
                                1
                            }
                        });
                    }
                }
            }
        } else if let Some((units, files)) = &mut self.services {
            match self.ui.view {
                View::ServiceUnits => {
                    units.sort_by_key(|u| {
                        let name_desc =
                            format!("{} {}", u.name.to_lowercase(), u.description.to_lowercase());
                        if name_desc.contains(&q) {
                            0
                        } else {
                            1
                        }
                    });
                }
                View::ServiceUnitFiles => {
                    files.sort_by_key(|f| {
                        if f.name.to_lowercase().contains(&q) {
                            0
                        } else {
                            1
                        }
                    });
                }
            }
        }
        self.ui.set_current_line(0);
    }
}

pub async fn start_application(config: Config) -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).map_err(RounalError::TerminalError)?;

        let styler = Styler::new(&config);
        let mut app = App::new(config);
        let services = get_system_services().await?;
        app.set_services(services)?;

        run(&mut terminal, app, styler).await?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App, styler: Styler) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            draw_ui(frame, &app, &styler).ok();

            if app.ui.is_showing_help {
                draw_help_modal(frame, &styler).ok();
            }

            if app.ui.is_showing_line_in_modal {
                draw_entry_line(frame, &app, &styler).ok();
            }

            if app.ui.is_showing_docs {
                draw_docs_modal(frame, &styler).ok();
            }
        })?;

        if let Some(event) = handle_key_events(&mut app) {
            match event {
                Events::Quit => app.set_is_running(false),
                Events::Search => app.ui.set_is_in_search_mode(true),
                Events::GetHelp => app.ui.set_is_showing_help(!app.ui.is_showing_help),
                Events::Docs => app.ui.set_is_showing_docs(!app.ui.is_showing_docs),
                Events::GetLineInModal => app
                    .ui
                    .set_is_showing_line_in_modal(!app.ui.is_showing_line_in_modal),
                Events::GetLogs => {
                    if let Some(service) = &app.selected_service {
                        info!("start getting journals");
                        let all_logs_for_service = get_journal_logs(service).await?;
                        app.set_logs(all_logs_for_service);
                        info!("journals set to app");
                    }
                }
            }
        }
    }
    Ok(())
}
