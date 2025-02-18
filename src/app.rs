use crate::core::{
    config::Config,
    error::{Result, RounalError},
    input_handler::handle_key_events,
    journal::{get_journal_logs, JournalLogMap, SharedJournalLogs},
    system::{get_system_services, ServiceUnitFiles, ServiceUnits},
};

use crate::ui::ui::{draw_help_modal, draw_ui, draw_whole_line, UI};

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

// TODO
#[derive(PartialEq)]
pub enum Events {
    Quit,
    GetLogs,
    GetHelp,
    GetLineInModal,
    Search,
}

// TODO:
//      - yanking -> clipboard content destroyed after exiting application
//      - services/logs highlighted accordingly
//      - action mode
//      - responsive layout
//      - error handling -> no reason to panic every where, i.e. unwrap
//      - vim-like search
//      - read custom configs
//      - upgrade package xcb to > 1.0 -> use arbroad crate for clipboard management
//      - filtering based on status (failed | running | exited)
//      - catch sudo
//
//

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
}

pub async fn start_application(config: Config) -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    {
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend).map_err(RounalError::TerminalError)?;

        let mut app = App::new(config);
        let services = get_system_services().await?;
        //info!("GOT SERVICES");
        app.set_services(services)?;
        //info!("SET SERVICES");

        //info!("CALL RUN");
        run(&mut terminal, app).await?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

async fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    while app.is_running {
        terminal.draw(|frame| {
            draw_ui(frame, &mut app).ok();

            if app.ui.is_showing_help {
                draw_help_modal(frame).ok();
            }

            if app.ui.is_showing_line_in_modal {
                draw_whole_line(frame, &app).ok();
            }
        })?;

        if let Some(event) = handle_key_events(&mut app) {
            match event {
                Events::Quit => app.set_is_running(false),
                Events::GetHelp => app.ui.set_is_showing_help(!app.ui.is_showing_help),
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
                _ => {}
            }
        }
    }
    Ok(())
}
