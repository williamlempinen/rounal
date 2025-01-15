use crate::journal::{get_logs, Log};
use crate::system::{get_services, ServiceUnits};
use crate::ui::draw_ui;
use crate::{AppError, Result};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;

use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use std::io::stdout;
use std::sync::{Arc, RwLock};

pub struct App {
    pub is_running: bool,
    pub current_line: usize,
    pub is_modal: bool,
    pub logs: Option<Vec<Log>>,
    pub services: Arc<RwLock<Vec<ServiceUnits>>>,
    pub selected_service: Option<String>,
    pub selected_priority: Option<u8>,
}

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
            services: Arc::new(RwLock::new(Vec::new())),
            selected_service: None,
            selected_priority: Some(4), // default priority to 3 / errors
        }
    }

    pub fn set_services(&mut self, services: Vec<ServiceUnits>) -> Result<()> {
        let mut write_guard = self.services.write().map_err(|e| {
            AppError::UnexpectedError(format!("Error setting services for app: {}", e))
        })?;

        write_guard.clear();
        write_guard.extend(services);

        Ok(())
    }

    pub fn set_logs(&mut self, logs: Vec<Log>) {
        self.logs = Some(logs);
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
        let (service_units, _service_unit_files) = get_services().await?;

        App::set_services(&mut app, service_units)?;

        run(&mut terminal, app).await?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            let _ = draw_ui(frame, &app);
        })?;

        match listen_key_events(&mut app) {
            Some(KeyEvents::Quit) => app.is_running = false,
            Some(KeyEvents::EnterFor("get_logs")) => {
                if let Some(service) = &app.selected_service {
                    let service_logs =
                        get_logs(service, app.selected_priority.unwrap_or_default()).await?;
                    app.set_logs(service_logs);
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
                let services_len = app.services.read().unwrap().len();

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
                let services = app.services.read().expect("Could not read services");

                if let Some(service) = services.get(app.current_line) {
                    app.selected_service = Some(service.name.clone());
                    app.is_modal = true;
                    app.logs = None;
                }

                Some(KeyEvents::EnterFor("get_logs"))
            }
            KeyCode::Char('c') => {
                if app.is_modal {
                    app.is_modal = false;
                    app.selected_service = None;
                }

                None
            }
            _ => None,
        }
    } else {
        None
    }
}
