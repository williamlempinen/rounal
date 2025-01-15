use std::env;

use rounal::{app, Result};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    println!("Rounal!");

    if let Err(err) = app::start_application().await {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
