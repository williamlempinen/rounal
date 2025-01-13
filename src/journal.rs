use tokio::process::Command;

use crate::{AppError, Result};

#[derive(Debug, Clone)]
pub struct Log {
    pub priority: u8,
    pub timestamp: String,
    pub log_message: String,
    pub hostname: String,
    pub service: String,
}

pub async fn get_logs(service: &String, priority: u8) -> Result<Vec<Log>> {
    let out = Command::new("sudo")
        .arg("journalctl")
        .arg("-u")
        .arg(service)
        .arg("-r")
        .arg("-p")
        .arg(priority.to_string())
        .output()
        .await?;

    if !out.status.success() {
        return Err(AppError::JournalCtlError(format!(
            "{}, {}",
            service.to_string(),
            String::from_utf8_lossy(&out.stderr).to_string()
        )));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let logs: Vec<Log> = stdout
        .lines()
        .skip(1)
        .filter_map(|line| parse_log(line, &priority))
        .collect();

    Ok(logs)
}

fn parse_log(log_line: &str, p: &u8) -> Option<Log> {
    let parts: Vec<&str> = log_line.split_whitespace().collect();

    match p {
        1..=7 => {
            println!("Priority is {}", p);
            let priority = p.clone();
            let timestamp = parts.get(..3)?.join(" ");
            let hostname = parts.get(3)?.to_string();
            let service = parts.get(4)?.trim_end_matches(":").to_string();
            let log_message = parts.get(5..)?.join(" ").to_string();
            return Some(Log {
                priority,
                timestamp,
                log_message,
                hostname,
                service,
            });
        }
        _ => None,
    }
}
