use anyhow::Result;

use rounal::system::{get_list_unit_files, get_list_units};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rounal!");

    let service_units = get_list_units().await?;
    let service_files = get_list_unit_files().await?;

    for service in service_units {
        println!(
            "{:?},{:?},{:?},{:?},{:?}",
            service.name, service.load, service.active, service.sub, service.description
        );
    }

    println!("######################");

    for service in service_files {
        println!(
            "{:?}, {:?}, {:?}",
            service.name, service.state, service.preset
        );
    }

    Ok(())
}
