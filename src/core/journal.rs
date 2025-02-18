use log::{error, info};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use tokio::{process::Command, sync::mpsc};

use crate::core::error::{Result, RounalError};

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
    info!("get_journal_logs called");

    for p in 1..=7 {
        info!("get_journal_logs called, in for loop");
        let thread_logs = logs_for_service.clone();
        let thread_service = service.to_string();
        let thread_sender = sender.clone();

        tokio::spawn(async move {
            info!("Spawned with {}", p);

            let logs = get_logs(thread_service, p)
                .await
                .expect("Error getting logs for: {service} with priority: {p}");

            thread_logs
                .lock()
                .map_err(|e| RounalError::JournalCtlError(format!("{:?}", e)))
                .ok()
                .map(|mut logs_map| logs_map.insert(p, logs));
            info!("Done");

            if let Err(e) = thread_sender.send(()).await {
                error!("Error in thread sender: {:?}", e);
            }
        });
    }

    for x in 1..=7 {
        if receiver.recv().await.is_none() {
            return Err(RounalError::UnexpectedError(format!(
                "Error receiving logs for priority {}",
                x
            )));
        }
    }

    info!("LOGS: {:?}", logs_for_service);
    Ok(logs_for_service)
}

async fn get_logs(service: String, priority: u8) -> Result<Vec<JournalLog>> {
    //info!("get_logs with {} {}", service, priority);
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
        return Err(RounalError::JournalCtlError(format!(
            "{}, {}",
            service.to_string(),
            String::from_utf8_lossy(&out.stderr).to_string()
        )));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    //info!(
    //    "RUN THIS: journalctl -u {:?} -r -p {:?}",
    //    &service, &priority
    //);
    //info!("STDOUT: {:?}", stdout);

    let logs: Vec<JournalLog> = stdout
        .lines()
        .skip(1)
        .filter_map(|line| parse_log(line, &priority))
        .collect();

    //info!("LOHS THEN: {:?}", logs);

    Ok(logs)
}

fn parse_log(log_line: &str, p: &u8) -> Option<JournalLog> {
    //info!("LOG LINE: {:?}", log_line);
    let parts: Vec<&str> = log_line.split_whitespace().collect();

    match p {
        1..=7 => {
            //info!("Found entry with priority of {}", p);
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
