use anyhow::Result;

use rounal::system::get_services;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    let services = get_services().await?;

    for service in services {
        println!(
            "Name: {}, State: {:?}, Description: {}",
            service.name, service.state, service.description
        );
    }
    Ok(())
}
