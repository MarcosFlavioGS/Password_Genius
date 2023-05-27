use cli_clipboard::{ClipboardContext,ClipboardProvider};

pub fn clipboarder(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), password);
    Ok(())
}
