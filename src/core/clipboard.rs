use log::info;

use crate::core::error::{Result, RounalError};

use arboard::Clipboard;

use std::thread::sleep;
use std::time::Duration;

pub fn copy_to_clipboard(content: String) -> Result<()> {
    let mut clipboard =
        Clipboard::new().map_err(|e| RounalError::ClipboardError(format!("{:?}", e)))?;

    info!("Got clipboard: {}", content);

    copy(&mut clipboard, content.as_str());
    sleep(Duration::from_secs(1));
    Ok(())
}

fn copy(clipboard: &mut Clipboard, content: &str) {
    let _ = clipboard.set_text(content);
}
