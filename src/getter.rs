use crate::encrypter::encrypt::decrypt;
use dirs;
use std::fs;

pub fn getter(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;
    let file_path = home_dir.join("passgen").join(source).join("pass");

    // Read the binary content of the file
    let cipher_text_with_nonce = fs::read(file_path).expect("Failed to read from file");

    match decrypt(cipher_text_with_nonce) {
        Ok(decrypted) => Ok(decrypted),
        Err(err) => {
            eprintln!("Error: {err}");
            return Err(err);
        }
    }
}
