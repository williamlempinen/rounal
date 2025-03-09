use log::{error, info};
use rounal::app;
use rounal::core::{config::Config, error::Result};
use simple_logging::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let config = Config::load("app_config.toml")?;
    let _ = log_to_file("debug.log", config.options.to_level_filter());

    info!("CONFIG: {:?}", config);
    info!("Rounal STARTING");

    if let Err(err) = app::start_application(config).await {
        error!("Rounal application error: {}", err);
    }

    info!("Rounal ENDED");
    Ok(())
}
