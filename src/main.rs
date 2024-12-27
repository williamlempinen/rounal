use anyhow::Result;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use rounal::{
    app::{run, App},
    journal::get_logs,
    system::get_services,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    let services = get_services().await?;
    let logs = get_logs(4).await?;

    //for unit in services.0 {
    //    println!(
    //        "{:?},{:?},{:?},{:?},{:?}",
    //        unit.name, unit.load, unit.active, unit.sub, unit.description
    //    );
    //}
    //
    //println!("######################");
    //
    //for unit in services.1 {
    //    println!("{:?}, {:?}, {:?}", unit.name, unit.state, unit.preset);
    //}

    //for log in logs {
    //    println!(
    //        "{:?},{:?},{:?},{:?},{:?}",
    //        log.priority, log.timestamp, log.hostname, log.service, log.log_message
    //    );
    //}

    // Setup terminal for TUI
    enable_raw_mode()?;

    // Prepare terminal
    let mut stdout = std::io::stdout(); // Import `stdout` from `std::io`
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?; // Enter alternate screen

    // Pass mutable reference to avoid moving ownership
    let backend = CrosstermBackend::new(&mut stdout);
    let terminal = Terminal::new(backend)?;

    // Initialize app
    let app = App::new();

    // Run the application
    let res = run(terminal, app);

    // Restore terminal state
    disable_raw_mode()?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?; // Leave alternate screen

    // Handle errors if any
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
