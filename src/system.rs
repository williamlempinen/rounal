use std::sync::{Arc, RwLock};

use anyhow::{bail, Context, Result};
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
}

#[derive(Debug, Clone)]
pub enum EnabledStates {
    Enabled,
    Disabled,
    Static,
    Masked,
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

pub async fn get_services() -> Result<Vec<Service>> {
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
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                return None;
            }

            let active_state = match parts[2] {
                "running" => ActiveStates::Running,
                "exited" => ActiveStates::Exited,
                "waiting" => ActiveStates::Waiting,
                "inactive" => ActiveStates::Inactive,
                "failed" => ActiveStates::Failed,
                "activating" => ActiveStates::Activating,
                "deactivating" => ActiveStates::Deactivating,
                "reloading" => ActiveStates::Reloading,
                _ => ActiveStates::Inactive,
            };

            Some(Service {
                name: parts[0].to_string(),
                state: ServiceState {
                    active: active_state,
                    enabled: EnabledStates::Enabled, // Modify as needed
                },
                description: parts[3..].join(" "),
            })
        })
        .collect();

    Ok(services)
}
