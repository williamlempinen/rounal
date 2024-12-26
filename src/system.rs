use std::sync::{Arc, RwLock};

use anyhow::{bail, Context, Ok, Result};
use tokio::process::Command;

#[derive(Debug, Clone)]
pub enum ActiveStates {
    Running,
    Exited,
    Waiting,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Reloading,
    Unknown,
}

impl ActiveStates {
    fn get_state(state_as_str: &str) -> Self {
        match state_as_str {
            "running" => Self::Running,
            "exited" => Self::Exited,
            "waiting" => Self::Waiting,
            "inactive" => Self::Inactive,
            "failed" => Self::Failed,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            "reloading" => Self::Reloading,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnabledStates {
    Enabled,
    Disabled,
    Static,
    Masked,
    Alias,
    Indirect,
    Generated,
    EnabledRuntime,
    Unknown,
}

impl EnabledStates {
    fn get_state(state_as_str: &str) -> Self {
        match state_as_str {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "static" => Self::Static,
            "masked" => Self::Masked,
            "alias" => Self::Alias,
            "indirect" => Self::Indirect,
            "generated" => Self::Generated,
            "enabled-runtime" => Self::EnabledRuntime,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub active: ActiveStates,
    pub enabled: EnabledStates,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub state: ServiceState,
    pub description: String,
}

pub type SharedService = Arc<RwLock<Service>>;

pub async fn get_list_units() -> Result<Vec<Service>> {
    let out = Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service")
        .arg("--all")
        .output()
        .await
        .context("Failed running systemctl")?;

    if !out.status.success() {
        bail!("Systemctl failed with: {:?}", out.status);
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let services: Vec<Service> = stdout
        .lines()
        .skip(1) // first is column headers
        .filter_map(|line| parse_service_units(line))
        .collect();

    Ok(services)
}

fn parse_service_units(service_line: &str) -> Option<Service> {
    let parts: Vec<&str> = service_line.split_whitespace().collect();

    if parts.len() < 4 {
        println!("Service is missing parts with length of {}", parts.len());
        println!("{:?}", parts);
        return None;
    }

    let name = parts
        .get(0)
        .expect("Failed to get service name")
        .to_owned()
        .to_string();
    let state = ActiveStates::get_state(parts.get(2)?);
    let unknown = EnabledStates::Unknown;
    let description = parts.get(3..)?.join(" ");

    Some(Service {
        name,
        state: ServiceState {
            active: state,
            enabled: unknown,
        },
        description,
    })
}

//pub async fn get_list_unit_files() -> Result<Vec<Service>> {
//    let out = Command::new("systemctl")
//        .arg("list-unit-files")
//        .arg("--type=service")
//        .arg("--all")
//        .output()
//        .await
//        .context("Failed running systemctl")?;
//
//    if !out.status.success() {
//        bail!("Systemctl failed with: {:?}", out.status);
//    }
//
//    let stdout = String::from_utf8_lossy(&out.stdout);
//
//    let services: Vec<Service> = stdout
//        .lines()
//        .skip(1) // first is column headers
//        .filter_map(|line| parse_service_unit_files(line))
//        .collect();
//
//    Ok(services)
//}

// todo return other systemctl call
//fn parse_service_unit_files(service_line: &str) -> Option<Service> {
//    let parts: Vec<&str> = service_line.split_whitespace().collect();
//
//    if parts.len() < 3 {
//        println!("Service is missing parts with length of {}", parts.len());
//        println!("{:?}", parts);
//        return None;
//    }
//
//    let name = parts
//        .get(0)
//        .expect("Failed to get service name")
//        .to_owned()
//        .to_string();
//    let state = EnabledStates::get_state(parts.get(1)?);
//    let preset = EnabledStates::Unknown;
//    let description = parts.get(3..)?.join(" ");
//
//    Some(Service {
//        name,
//        state: ServiceState {
//            active: state,
//            enabled: unknown,
//        },
//        description,
//    })
//}
