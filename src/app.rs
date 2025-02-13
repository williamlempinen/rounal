use crate::core::{
    config::Config,
    error::{Result, RounalError},
    input_handler::handle_key_events,
    journal::{get_journal_logs, JournalLogMap, SharedJournalLogs},
    system::{get_system_services, ServiceUnitFiles, ServiceUnits},
};

use crate::ui::ui::draw_ui;

use std::{
    io::stdout,
    sync::{Arc, Mutex},
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

#[derive(PartialEq, Debug, Default, Clone)]
pub enum ServiceView {
    #[default]
    Units,
    UnitFiles,
}

// TODO
#[derive(PartialEq)]
pub enum Events {
    Quit,
    GetLogs,
    GetHelp,
    Search,
}

// TODO:
//      - scrollbar
//      - yanking
//      - action mode
//      - vim-like search
//      - read custom configs
//      - error handling
//      - filtering based on status (failed | running | exited)
//      - catch sudo

#[derive(Debug)]
pub struct App {
    pub is_running: bool,
    pub is_in_logs: bool,
    pub current_line: usize,
    pub logs: Option<SharedJournalLogs>,
    pub services: Option<(Vec<ServiceUnits>, Vec<ServiceUnitFiles>)>,
    pub selected_service: Option<String>,
    pub selected_priority: Option<u8>,
    pub selected_service_view: ServiceView,
}

impl App {
    pub fn new() -> Self {
        Self {
            is_running: true,
            is_in_logs: false,
            current_line: 0,
            logs: None,
            services: None,
            selected_service: None,
            selected_priority: Some(4), // default priority level
            selected_service_view: ServiceView::Units,
        }
    }

    pub fn set_init(&mut self) {
        self.selected_priority = Some(4);
        self.is_in_logs = false;
        self.clear_logs();
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.selected_priority = Some(priority);
    }

    pub fn set_services(
        &mut self,
        services: (Vec<ServiceUnits>, Vec<ServiceUnitFiles>),
    ) -> Result<()> {
        self.services = Some(services);
        Ok(())
    }

    pub fn set_view(&mut self, new_view: ServiceView) {
        self.selected_service_view = new_view;
    }

    pub fn set_current_line(&mut self, position: usize) {
        self.current_line = position;
    }

    pub fn set_logs(&mut self, logs: Arc<Mutex<JournalLogMap>>) {
        self.logs = Some(logs);
    }

    pub fn clear_logs(&mut self) {
        self.logs = None;
    }
}

pub async fn start_application() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).map_err(RounalError::TerminalError)?;

        let mut app = App::new();
        let services = get_system_services().await?;
        info!("GOT SERVICES");
        app.set_services(services)?;
        info!("SET SERVICES");

        info!("CALL RUN");
        run(&mut terminal, app).await?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            draw_ui(frame, &app).ok();

            // if is looking help -> draw help modal
        })?;

        if let Some(event) = handle_key_events(&mut app) {
            match event {
                Events::Quit => app.is_running = false,
                Events::GetLogs => {
                    if let Some(service) = &app.selected_service {
                        info!("start getting journals");
                        let all_logs_for_service = get_journal_logs(service).await?;
                        app.set_logs(all_logs_for_service);
                        info!("journals set to app");
                    }
                }
                _ => info!("nothing for key events"),
            }
        }
    }
    Ok(())
}
