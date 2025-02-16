use thiserror::Error;

pub type Result<T> = std::result::Result<T, RounalError>;

#[derive(Error, Debug)]
pub enum RounalError {
    #[error("Failed to initialize terminal")]
    TerminalError(#[from] std::io::Error),

    #[error("Failed to execute system command: {0}")]
    CommandError(String),

    #[error("Journalctl logs fetch failed: {0}")]
    JournalCtlError(String),

    #[error("Systemctl services fetch failed: {0}")]
    SystemCtlError(String),

    #[error("Unexpected application error")]
    UnexpectedError(String),

    #[error("Config file error")]
    ConfigurationFileError,

    //no impl yet
    #[error("Copying to clipboard error")]
    ClipboardError,
}
