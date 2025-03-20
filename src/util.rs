use crate::core::system::{Active, Load, Preset, State, Sub};

pub trait PadStr {
    fn pad_with(&self, width: usize) -> String;
}

impl PadStr for str {
    fn pad_with(&self, width: usize) -> String {
        format!("{:<width$}", self, width = width)
    }
}

pub fn map_to_priority_str(priority: &u8) -> &'static str {
    match priority {
        1 => "emerg",
        2 => "alert",
        3 => "err",
        4 => "warn",
        5 => "notice",
        6 => "info",
        7 => "debug",
        _ => "unknown",
    }
}

pub fn get_state_color_str(state: &State) -> &'static str {
    match state {
        State::Enabled
        | State::EnabledRuntime
        | State::Static
        | State::Generated
        | State::Alias => "green",
        State::Indirect | State::Transient | State::Disabled => "blue",
        State::Masked => "red",
        _ => "white",
    }
}

pub fn get_load_color_str(load: &Load) -> &'static str {
    match load {
        Load::Loaded => "green",
        Load::NotFound => "red",
        _ => "white",
    }
}

pub fn get_active_color_str(active: &Active) -> &'static str {
    match active {
        Active::Active => "green",
        Active::InActive => "blue",
        _ => "white",
    }
}

pub fn get_sub_color_str(sub: &Sub) -> &'static str {
    match sub {
        Sub::Running | Sub::Activating => "green",
        Sub::Dead | Sub::Waiting | Sub::Inactive | Sub::Deactivating | Sub::Reloading => "blue",
        Sub::Failed => "red",
        _ => "white",
    }
}

pub fn get_preset_color_str(preset: &Preset) -> &'static str {
    match preset {
        Preset::Enabled => "green",
        _ => "white",
    }
}
