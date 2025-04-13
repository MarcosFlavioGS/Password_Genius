use std::path::PathBuf;

use dirs;

///
/// Gets the full path of the base dir.
///
pub fn get_path() -> String {
    let home_dir = dirs::home_dir()
        .ok_or("Failed to get home directory")
        .unwrap();
    let full_path = home_dir.join("passgen/");
    let path = full_path.to_str().unwrap();

    path.to_string()
}

pub fn get_config_path() -> PathBuf {
    let home_dir = dirs::home_dir()
        .ok_or("Failed to get home directory")
        .unwrap();
    let path = home_dir.join(".config/passgen/");

    path
}

pub fn get_config_path_str() -> String {
    let home_dir = dirs::home_dir()
        .ok_or("Failed to get home directory")
        .unwrap();
    let path = home_dir.join(".config/passgen/passgen.toml");
    let path = path.to_str().unwrap();

    path.to_string()
}
