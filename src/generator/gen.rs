use crate::clipboard::clipboarder::clipboarder;
use crate::config::Config;
use crate::password::{insert_pass::insert_pass, new_pass::new_password};

///
/// Generates a new pass: String, stores into path: &str and copies to clipboard.
///
pub fn generate(path: &str, config: &Config) {
    let passwd = new_password();
    if config.options.show_pass {
        println!("This is your new password: {passwd}");
    }

    match insert_pass(path, &passwd, config) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => eprintln!("Failed to insert password.\nError: {err}"),
    }
    match clipboarder(&passwd[..]) {
        Ok(_) => println!("Copied to clipboard"),
        Err(err) => eprintln!("Failed to read line.\nError: {err}"),
    }
}

/// Generates a password of `length`, stores it at `path`, copies it to the clipboard, and returns
/// the plaintext (for callers that need to display it without writing to stdout).
pub fn generate_stored(path: &str, config: &Config, length: u8) -> Result<String, String> {
    use crate::password::new_pass::generate_password_at_length;

    let passwd = generate_password_at_length(length);
    insert_pass(path, &passwd, config).map_err(|e| format!("Failed to insert password: {e}"))?;
    clipboarder(&passwd[..]).map_err(|e| format!("Failed to copy to clipboard: {e}"))?;
    Ok(passwd)
}
