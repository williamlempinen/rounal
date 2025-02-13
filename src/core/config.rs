use crate::core::error::{Result, RounalError};

use ratatui::style::Color;

use serde::Deserialize;

use std::{fs, path::Path};

use toml;

use log::error;

#[derive(Debug, Deserialize, Default)]
pub struct Palette {
    pub red: [u8; 3],
    pub black: [u8; 3],
    pub blue: [u8; 3],
    pub white: [u8; 3],
    pub gray: [u8; 3],
}

#[derive(Debug, Deserialize, Default)]
pub struct Priority {
    pub emerg: [u8; 3],
    pub alert: [u8; 3],
    pub err: [u8; 3],
    pub warn: [u8; 3],
    pub notice: [u8; 3],
    pub info: [u8; 3],
    pub debug: [u8; 3],
    pub unknown: [u8; 3],
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub palette: Palette,
    pub priority: Priority,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let path = Path::new(path);

        if !path.exists() {
            error!("Config file not found, using defaults.");
            return Err(RounalError::ConfigurationFileError);
        }

        let contents = fs::read_to_string(path).map_err(|_| {
            error!("Failed to read config file: {}", path.display());
            RounalError::ConfigurationFileError
        })?;

        let config: Config = toml::from_str(&contents).map_err(|e| {
            error!("Failed to parse config: {}", e);
            RounalError::ConfigurationFileError
        })?;

        Ok(config)
    }
}
