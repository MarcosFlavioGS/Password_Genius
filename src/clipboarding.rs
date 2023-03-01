use clipboard::{ClipboardContext,ClipboardProvider};

pub fn clipboarder(password: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(password)?;
    Ok(())
}
