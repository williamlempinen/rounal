use anyhow::Result;

use rounal::system::get_list_units;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    let services = get_list_units().await?;

    for service in services {
        println!(
            "{},{:?},{}",
            service.name, service.state, service.description
        );
    }
    Ok(())
}
