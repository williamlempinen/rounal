use std::env;

use log::{error, info, LevelFilter};
use rounal::{app, Result};
use simple_logging::*;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");

    let _ = log_to_file("debug.log", LevelFilter::Info);

    info!("Rounal STARTING");

    if let Err(err) = app::start_application().await {
        error!("Rounal application error: {}", err);
    }

    info!("Rounal ENDED");
    Ok(())
}
