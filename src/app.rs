use crate::journal::{get_journal_logs, JournalLogMap, SharedJournalLogs};
use crate::system::{get_system_services, ServiceUnitFiles, ServiceUnits};
use crate::ui::draw_ui;
use crate::{AppError, Result};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;

use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use tokio::sync::Mutex;

use std::io::stdout;
use std::sync::Arc;

#[derive(PartialEq)]
pub enum ServiceView {
    Units,
    UnitFiles,
}

pub struct App {
    pub is_running: bool,
    pub current_line: usize,
    pub is_modal: bool,
    pub logs: Option<SharedJournalLogs>,
    pub services: Option<(Vec<ServiceUnits>, Vec<ServiceUnitFiles>)>,
    pub selected_service: Option<String>,
    pub selected_priority: Option<u8>,
    pub selected_service_view: ServiceView,
}

#[derive(PartialEq)]
pub enum KeyEvents<'a> {
    Quit,
    EnterFor(&'a str),
}

impl App {
    pub fn new() -> Self {
        Self {
            is_running: true,
            current_line: 0,
            is_modal: false,
            logs: None,
            services: None,
            selected_service: None,
            selected_priority: Some(4), // default priority level
            selected_service_view: ServiceView::Units,
        }
    }

    fn set_services(&mut self, services: (Vec<ServiceUnits>, Vec<ServiceUnitFiles>)) -> Result<()> {
        self.services = Some(services);
        Ok(())
    }

    fn set_logs(&mut self, logs: Arc<Mutex<JournalLogMap>>) {
        self.logs = Some(logs);
    }

    fn clear_logs(&mut self) {
        self.logs = None;
    }
}

pub async fn start_application() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).map_err(AppError::TerminalError)?;

        let mut app = App::new();
        let services = get_system_services().await?;
        app.set_services(services);

        run(&mut terminal, app).await?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;

        match listen_key_events(&mut app) {
            Some(KeyEvents::Quit) => app.is_running = false,
            Some(KeyEvents::EnterFor("get_logs")) => {
                if let Some(service) = &app.selected_service {
                    let all_logs_for_service = get_journal_logs(service).await?;
                    app.set_logs(all_logs_for_service);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn listen_key_events(app: &mut App) -> Option<KeyEvents> {
    if let Event::Key(key) = event::read().expect("Error keyboard input") {
        match key.code {
            KeyCode::Char('q') => Some(KeyEvents::Quit),
            KeyCode::Down => {
                let services_len = match &app.services {
                    Some((u, f)) => {
                        if app.selected_service_view == ServiceView::Units {
                            u.len()
                        } else {
                            f.len()
                        }
                    }
                    None => 0,
                };

                if app.current_line < services_len - 1 {
                    app.current_line += 1;
                }

                None
            }
            KeyCode::Up => {
                if app.current_line > 0 {
                    app.current_line -= 1;
                }

                None
            }
            KeyCode::Enter => {
                if let Some((u, f)) = &app.services {
                    match app.selected_service_view {
                        ServiceView::Units => {
                            if let Some(service) = u.get(app.current_line) {
                                app.selected_service = Some(service.name.clone());
                                app.is_modal = true;
                            }
                        }
                        ServiceView::UnitFiles => {
                            if let Some(service) = f.get(app.current_line) {
                                app.selected_service = Some(service.name.clone());
                                app.is_modal = true;
                            }
                        }
                    }
                    Some(KeyEvents::EnterFor("get_logs"))
                } else {
                    None
                }
            }
            KeyCode::Char(c) if ('1'..='7').contains(&c) => {
                app.selected_priority = Some(c.to_digit(10).unwrap() as u8);
                None
            }
            KeyCode::Char('c') => {
                if app.is_modal {
                    app.is_modal = false;
                    app.selected_service = None;
                    app.clear_logs();
                }
                None
            }
            _ => None,
        }
    } else {
        None
    }
}
