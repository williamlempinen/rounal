use std::sync::{Arc, RwLock};

use crate::system::{get_services, ServiceUnits};
use crate::ui::draw_ui;
use crate::{AppError, Result};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

pub struct App {
    pub is_running: bool,
    pub logs: Arc<RwLock<Vec<String>>>,
    pub services: Arc<RwLock<Vec<ServiceUnits>>>,
    pub selected_service: Option<String>,
    pub selected_priority: Option<u8>,
}

pub enum KeyEvents {
    Quit,
    Select(String),
}

impl App {
    pub fn new() -> Self {
        Self {
            is_running: false,
            logs: Arc::new(RwLock::new(Vec::new())),
            services: Arc::new(RwLock::new(Vec::new())),
            selected_service: None,
            selected_priority: Some(3), // default priority to 3 / errors
        }
    }

    pub fn set_services(&mut self, services: Vec<ServiceUnits>) -> Result<()> {
        let mut write_guard = self
            .services
            .write()
            .map_err(|_| AppError::UnexpectedError)?;

        write_guard.clear();
        write_guard.extend(services);

        Ok(())
    }
}

pub async fn start_application() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;

    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).map_err(AppError::TerminalError)?;

        let mut app = App::new();
        let (service_units, _service_unit_files) = get_services().await?;

        App::set_services(&mut app, service_units)?;

        run(&mut terminal, app).await?;
    }

    disable_raw_mode()?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;

        match listen_key_events() {
            Some(KeyEvents::Quit) => app.is_running = false,
            _ => {}
        }
    }
    Ok(())
}

fn listen_key_events() -> Option<KeyEvents> {
    if let Event::Key(key) = event::read().expect("Error reading keys") {
        match key.code {
            KeyCode::Char('q') => Some(KeyEvents::Quit),
            _ => None,
        }
    } else {
        None
    }
}
