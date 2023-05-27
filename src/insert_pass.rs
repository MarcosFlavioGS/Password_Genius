use dirs;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn insert_pass(path: &str, pass: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let full_path = home_dir.join(path);
    if let Some(parent_dir) = std::path::Path::new(&full_path).parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file = File::create(full_path).expect("Failed creating file");
    file.write_all(pass.as_bytes()).expect("Failed to write to file !");
    Ok(())
}
