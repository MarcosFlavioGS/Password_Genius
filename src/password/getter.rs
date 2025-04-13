use crate::{config::Config, encrypter::encrypt::decrypt};
use dirs;
use std::fs;

///
/// Retrieves a pass from the path, decrypts it and returns the plain text
///
pub fn getter(source: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;
    let file_path = home_dir.join("passgen").join(source).join("pass");

    // Read the binary content of the file
    let cipher_text_with_nonce = fs::read(file_path).expect("Failed to read from file");

    match decrypt(cipher_text_with_nonce, config) {
        Ok(decrypted) => Ok(decrypted),
        Err(err) => {
            eprintln!("Error: {err}");
            return Err(err);
        }
    }
}
