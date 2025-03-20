use crate::core::error::{Result, RounalError};
use arboard::Clipboard;
use log::info;
use std::thread::sleep;
use std::time::Duration;

pub fn yank_to_clipboard(content: String) -> Result<()> {
    let mut clipboard =
        Clipboard::new().map_err(|e| RounalError::ClipboardError(format!("{:?}", e)))?;

    info!("Got clipboard: {}", content);
    yank(&mut clipboard, content.as_str());
    sleep(Duration::from_secs(1));
    Ok(())
}

fn yank(clipboard: &mut Clipboard, content: &str) {
    let _ = clipboard.set_text(content);
}
