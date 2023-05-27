use clipboard::{ClipboardContext,ClipboardProvider};

pub fn clipboarder(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(password.to_string())?;
    Ok(())
}
