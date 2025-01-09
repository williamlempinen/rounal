use rounal::{app::start_application, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    if let Err(err) = start_application().await {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
