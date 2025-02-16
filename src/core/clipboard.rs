use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content).ok();
    Ok(())
}
