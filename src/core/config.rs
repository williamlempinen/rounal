use crate::core::error::{Result, RounalError};
use log::{error, LevelFilter};
use ratatui::style::Color;
use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Palette {
    pub red: [u8; 3],
    pub black: [u8; 3],
    pub blue: [u8; 3],
    pub white: [u8; 3],
    pub gray: [u8; 3],
    pub green: [u8; 3],
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Options {
    pub description: bool,
    pub yank: String,
    pub initial_priority: u8,
    pub debug_level: String,
    pub command_format: String,
}

impl Options {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self.debug_level.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Off,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub palette: Palette,
    pub priority: Priority,
    pub options: Options,
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

        Ok(config.clone())
    }

    pub fn get_palette_color(&self, color_name: &str) -> Color {
        match color_name {
            "red" => Color::Rgb(
                self.palette.red[0],
                self.palette.red[1],
                self.palette.red[2],
            ),
            "black" => Color::Rgb(
                self.palette.black[0],
                self.palette.black[1],
                self.palette.black[2],
            ),
            "blue" => Color::Rgb(
                self.palette.blue[0],
                self.palette.blue[1],
                self.palette.blue[2],
            ),
            "white" => Color::Rgb(
                self.palette.white[0],
                self.palette.white[1],
                self.palette.white[2],
            ),
            "gray" => Color::Rgb(
                self.palette.gray[0],
                self.palette.gray[1],
                self.palette.gray[2],
            ),
            "green" => Color::Rgb(
                self.palette.green[0],
                self.palette.green[1],
                self.palette.green[2],
            ),
            _ => Color::White,
        }
    }

    pub fn get_priority_color(&self, level: &str) -> Color {
        match level {
            "emerg" => Color::Rgb(
                self.priority.emerg[0],
                self.priority.emerg[1],
                self.priority.emerg[2],
            ),
            "alert" => Color::Rgb(
                self.priority.alert[0],
                self.priority.alert[1],
                self.priority.alert[2],
            ),
            "err" => Color::Rgb(
                self.priority.err[0],
                self.priority.err[1],
                self.priority.err[2],
            ),
            "warn" => Color::Rgb(
                self.priority.warn[0],
                self.priority.warn[1],
                self.priority.warn[2],
            ),
            "notice" => Color::Rgb(
                self.priority.notice[0],
                self.priority.notice[1],
                self.priority.notice[2],
            ),
            "info" => Color::Rgb(
                self.priority.info[0],
                self.priority.info[1],
                self.priority.info[2],
            ),
            "debug" => Color::Rgb(
                self.priority.debug[0],
                self.priority.debug[1],
                self.priority.debug[2],
            ),
            "unknown" => Color::Rgb(
                self.priority.unknown[0],
                self.priority.unknown[1],
                self.priority.unknown[2],
            ),
            _ => Color::White,
        }
    }
}
