use crate::core::system::{Active, Load, Preset, State, Sub};

pub const DOCS: &str = r#"
Commands that the program will run.
systemctl list-units --type=service --all,
systemctl list-unit-files --type=service --all,
sudo journalctl --unit=<service> --reverse --priority=<1-7>

Short description of service fields:
LOAD:
    Indicates if a unit file was loaded correctly.
ACTIVE:
    Describes if a service is currently running or not.
SUB:
    Provides a detailed state like running, exited, dead.
STATE:
    Whether the service is enabled, disabled, static, etc.
PRESET:
    System default recommendation for enabling services.
"#;

pub const HELP: &str = "Rounal - Key Mappings\n\n\
        Move: [hjkl / arrow keys]\n\
        Select: [Enter]\n\
        Close logs: [c]\n\
        Change priority: [1-7] or [Move]\n\
        Toggle see line in a modal: [K]\n\
        Yank message: [y] \n\
        Begin search: [/] \n\
        Toddle read explanations: [E] \n\
        Exit search mode: [Esc] \n\
        Quit: [q / Esc]\n\
        Toggle Help: [?]\n";

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
