use crate::core::error::{Result, RounalError};
use log::info;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub enum State {
    Enabled,
    Disabled,
    Static,
    Masked,
    Alias,
    Indirect,
    Generated,
    EnabledRuntime,
    Transient,
    Unknown,
}

impl State {
    fn get_state(state_as_str: &str) -> Self {
        match state_as_str {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "static" => Self::Static,
            "masked" => Self::Masked,
            "alias" => Self::Alias,
            "transient" => Self::Transient,
            "indirect" => Self::Indirect,
            "generated" => Self::Generated,
            "enabled-runtime" => Self::EnabledRuntime,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Preset {
    Enabled,
    Disabled,
    Empty,
    Unknown,
}

impl Preset {
    fn get_preset_state(state_as_str: &str) -> Self {
        match state_as_str {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "-" => Self::Empty,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Load {
    Loaded,
    NotFound,
    Unknown,
}

impl Load {
    fn get_load_state(state_as_str: &str) -> Self {
        match state_as_str {
            "loaded" => Self::Loaded,
            "not-found" => Self::NotFound,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Active {
    Active,
    InActive,
    Unknown,
}

impl Active {
    fn get_active_state(state_as_str: &str) -> Self {
        match state_as_str {
            "active" => Self::Active,
            "inactive" => Self::InActive,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sub {
    Running,
    Exited,
    Dead,
    Waiting,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Reloading,
    Unknown,
}

impl Sub {
    fn get_sub_state(state_as_str: &str) -> Self {
        match state_as_str {
            "running" => Self::Running,
            "exited" => Self::Exited,
            "dead" => Self::Dead,
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
pub struct ServiceUnits {
    pub name: String,
    pub load: Load,
    pub active: Active,
    pub sub: Sub,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ServiceUnitFiles {
    pub name: String,
    pub state: State,
    pub preset: Preset,
}

pub async fn get_system_services() -> Result<(Vec<ServiceUnits>, Vec<ServiceUnitFiles>)> {
    Ok((
        tokio::spawn(get_list_units())
            .await
            .map_err(|e| RounalError::SystemCtlError(format!("{:?}", e)))??,
        tokio::spawn(get_list_unit_files())
            .await
            .map_err(|e| RounalError::SystemCtlError(format!("{:?}", e)))??,
    ))
}

pub async fn get_list_units() -> Result<Vec<ServiceUnits>> {
    let out = Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service")
        .arg("--all")
        .output()
        .await?;

    if !out.status.success() {
        return Err(RounalError::SystemCtlError(format!(
            "{}",
            String::from_utf8_lossy(&out.stderr).to_string()
        )));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let services: Vec<ServiceUnits> = stdout
        .lines()
        .skip(1) // first is column headers
        .filter_map(|line| parse_service_units(line))
        .collect();

    Ok(services)
}

fn parse_service_units(service_line: &str) -> Option<ServiceUnits> {
    // unit can be also flagged, i.e. not-found
    let idx = if service_line.starts_with('●') {
        1
    } else {
        0
    };

    let parts: Vec<&str> = service_line.split_whitespace().collect();
    info!("units parts: {:?}", parts);

    if parts.len() < 4 {
        info!("Service is missing parts");
        info!("{:?}", parts);
        return None;
    }

    let name = parts.get(idx)?.to_owned();

    if !name.ends_with(".service") {
        info!("Not a valid service");
        return None;
    }

    let load = Load::get_load_state(parts.get(idx + 1)?);
    let active = Active::get_active_state(parts.get(idx + 2)?);
    let sub = Sub::get_sub_state(parts.get(idx + 3)?);
    let description = parts.get(idx + 4..)?.join(" ");

    Some(ServiceUnits {
        name: name.to_string(),
        load,
        active,
        sub,
        description,
    })
}

pub async fn get_list_unit_files() -> Result<Vec<ServiceUnitFiles>> {
    let out = Command::new("systemctl")
        .arg("list-unit-files")
        .arg("--type=service")
        .arg("--all")
        .output()
        .await?;

    if !out.status.success() {
        return Err(RounalError::SystemCtlError(format!(
            "{}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let services: Vec<ServiceUnitFiles> = stdout
        .lines()
        .skip(1) // first is column headers
        .filter_map(parse_service_unit_files)
        .collect();

    Ok(services)
}

fn parse_service_unit_files(service_line: &str) -> Option<ServiceUnitFiles> {
    let parts: Vec<&str> = service_line.split_whitespace().collect();
    info!("files {:?}", parts);

    if parts.len() < 3 {
        info!("Service is missing parts");
        info!("{:?}", parts);
        return None;
    }

    let name = parts.first()?.to_owned();

    if !name.ends_with(".service") {
        info!("Not a valid service");
        return None;
    }

    let state = State::get_state(parts.get(1)?);
    let preset = Preset::get_preset_state(parts.get(2)?);

    Some(ServiceUnitFiles {
        name: name.to_string(),
        state,
        preset,
    })
}
