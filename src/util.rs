pub fn map_to_priority_str(priority: &u8) -> String {
    let str = match priority {
        1 => "emerg",
        2 => "alert",
        3 => "err",
        4 => "warn",
        5 => "notice",
        6 => "info",
        7 => "debug",
        _ => "unknown",
    };

    str.to_string()
}
