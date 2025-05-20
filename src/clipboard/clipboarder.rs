use clipboard::{ClipboardContext, ClipboardProvider};
use std::process::Command;
use std::env;

///
/// Copies a pass to the clipboard
///
// pub fn clipboarder(password: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut ctx: ClipboardContext = ClipboardProvider::new()?;
//     ctx.set_contents(password.to_owned()).unwrap();
//     assert_eq!(ctx.get_contents().unwrap(), password);
//     Ok(())
// }

fn is_wayland() -> bool {
    env::var("WAYLAND_DISPLAY").is_ok() || env::var("XDG_SESSION_TYPE").map(|v| v == "wayland").unwrap_or(false)
}

/// Copies a password to the clipboard, supporting both X11 and Wayland environments
pub fn clipboarder(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    if is_wayland() {
        // Use wl-copy for Wayland
        let status = Command::new("wl-copy")
            .arg("--trim-newline")
            .arg("--type").arg("text/plain")
            .arg(password)
            .status()?;

        if !status.success() {
            return Err("Failed to copy to clipboard using wl-copy".into());
        }

        // Optionally verify (requires wl-paste)
        let output = Command::new("wl-paste")
            .arg("--no-newline")
            .output()?;

        let clipboard_content = String::from_utf8(output.stdout)?;
        if clipboard_content != password {
            return Err("Clipboard verification failed".into());
        }
    } else {
        // X11 approach using clipboard crate
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(password.to_owned())?;

        // Verify the content was set correctly
        let clipboard_content = ctx.get_contents()?;
        if clipboard_content != password {
            return Err("Clipboard verification failed".into());
        }
    }

    Ok(())
}
