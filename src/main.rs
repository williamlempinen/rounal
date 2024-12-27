use anyhow::Result;

use rounal::{journal::get_logs, system::get_services};

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
    enable_raw_mode()?; // Enable raw mode
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?; // Enter alternate screen
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // Initialize the app state
    let mut app = App::new();
    app.logs
        .write()?
        .extend(logs.into_iter().map(|log| format!("{:?}", log)));
    app.services
        .write()?
        .extend(services.0.into_iter().map(|unit| format!("{:?}", unit)));

    // Run the TUI application
    let res = rounal::app::run(terminal, app).await;

    // Restore terminal state
    disable_raw_mode()?; // Disable raw mode
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?; // Leave alternate screen

    // Handle errors, if any
    if let Err(err) = res {
        eprintln!("Application error: {:?}", err);
    }

    Ok(())
}
