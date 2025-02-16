use clipboard::{ClipboardContext, ClipboardProvider};

use crate::core::error::{Result, RounalError};

pub fn copy_to_clipboard(content: String) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| RounalError::ClipboardError(format!("Clipboard init failed: {}", e)))?;

    ctx.set_contents(content)
        .map_err(|e| RounalError::ClipboardError(format!("Clipboard set failed: {}", e)))?;

    Ok(())
}
