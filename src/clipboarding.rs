use clipboard::{ClipboardContext, ClipboardProvider};

///
/// Copies a pass to the clipboard
///
pub fn clipboarder(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), password);
    Ok(())
}
