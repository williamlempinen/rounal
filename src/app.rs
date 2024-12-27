use std::sync::{Arc, RwLock};

use crate::ui::draw_ui;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

pub enum View {
    Journalctl,
    Systemctl,
}

pub struct App {
    pub quit: bool,
    pub logs: Arc<RwLock<Vec<String>>>,     // Logs from journalctl
    pub services: Arc<RwLock<Vec<String>>>, // Services from systemctl
    pub current_view: View,                 // Current view in the TUI
    pub modal_visible: bool,                // If a modal is visible
}

impl App {
    pub fn new() -> Self {
        Self {
            quit: false,
            logs: Arc::new(RwLock::new(Vec::new())),
            services: Arc::new(RwLock::new(Vec::new())),
            current_view: View::Journalctl,
            modal_visible: false,
        }
    }
}

pub fn start() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?; // Enable raw mode for the terminal
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?; // Use an alternate screen
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run(terminal, app);

    // Restore terminal state
    disable_raw_mode()?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?;

    res
}

fn run<B: ratatui::backend::Backend>(mut terminal: Terminal<B>, mut app: App) -> Result<()> {
    while !app.quit {
        // Draw the TUI
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.quit = true, // Quit the app
                KeyCode::Char('j') => app.current_view = View::Journalctl, // Switch to journalctl view
                KeyCode::Char('s') => app.current_view = View::Systemctl, // Switch to systemctl view
                _ => {}
            }
        }
    }
    Ok(())
}
