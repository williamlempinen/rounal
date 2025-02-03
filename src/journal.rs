use log::info;
use std::{collections::HashMap, sync::Arc};

use tokio::{
    process::Command,
    sync::{mpsc, Mutex},
};

use crate::{AppError, Result};

#[derive(Debug, Clone)]
pub struct JournalLog {
    pub priority: u8,
    pub timestamp: String,
    pub log_message: String,
    pub hostname: String,
    pub service: String,
}

pub type Priority = u8;
pub type JournalLogMap = HashMap<Priority, Vec<JournalLog>>;
pub type SharedJournalLogs = Arc<Mutex<JournalLogMap>>;

pub async fn get_journal_logs(service: &str) -> Result<SharedJournalLogs> {
    let logs_for_service = Arc::new(Mutex::new(HashMap::new()));
    let (sender, mut receiver) = mpsc::channel(7);

    for p in 1..=7 {
        let cloned_logs = logs_for_service.clone();
        let cloned_service = service.to_string();
        let cloned_sender = sender.clone();

        tokio::task::spawn(async move {
            log::info!("Spawned");

            let logs = get_logs(cloned_service, p)
                .await
                .expect("Error getting logs for: {service} with priority: {p}");

            cloned_logs.lock().await.insert(p, logs);
            log::info!("Done");
            cloned_sender.send(()).await.unwrap();
        });
    }

    for _ in 1..=7 {
        receiver.recv().await.ok_or(AppError::UnexpectedError(
            "Error receiving logs".to_string(),
        ))?;
    }

    log::info!("Logs: {:?}", logs_for_service);

    Ok(logs_for_service)
}

async fn get_logs(service: String, priority: u8) -> Result<Vec<JournalLog>> {
    let out = Command::new("sudo")
        .arg("journalctl")
        .arg("-u")
        .arg(&service)
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

    let logs: Vec<JournalLog> = stdout
        .lines()
        .skip(1)
        .filter_map(|line| parse_log(line, &priority))
        .collect();

    Ok(logs)
}

fn parse_log(log_line: &str, p: &u8) -> Option<JournalLog> {
    let parts: Vec<&str> = log_line.split_whitespace().collect();

    match p {
        1..=7 => {
            info!("Priority is {}", p);
            let priority = p.clone();
            let timestamp = parts.get(..3)?.join(" ");
            let hostname = parts.get(3)?.to_string();
            let service = parts.get(4)?.trim_end_matches(":").to_string();
            let log_message = parts.get(5..)?.join(" ").to_string();
            return Some(JournalLog {
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
