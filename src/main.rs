use anyhow::Result;

use rounal::system::get_services;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    let services = get_services().await?;

    for unit in services.0 {
        println!(
            "{:?},{:?},{:?},{:?},{:?}",
            unit.name, unit.load, unit.active, unit.sub, unit.description
        );
    }

    println!("######################");

    for unit in services.1 {
        println!("{:?}, {:?}, {:?}", unit.name, unit.state, unit.preset);
    }

    Ok(())
}
