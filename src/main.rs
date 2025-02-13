mod core;
mod ui;

use core::config::Config;
use std::env;

use rounal::app;

use log::{error, info, LevelFilter};

use simple_logging::*;

#[tokio::main]
async fn main() -> core::error::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");

    let _ = log_to_file("debug.log", LevelFilter::Info);

    info!("Rounal STARTING");

    let config = Config::load("src/app_config.toml")?;
    info!("CONFIG: {:?}", config);

    if let Err(err) = app::start_application().await {
        error!("Rounal application error: {}", err);
    }

    info!("Rounal ENDED");
    Ok(())
}
