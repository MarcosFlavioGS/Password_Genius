use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use toml::map::Map;
use toml::Value;

use crate::get_path::get_config_path;

///
/// Create a configuration file into ".config/passgen"
///
pub fn create_default_config() -> std::io::Result<()> {
    let mut input = String::new();
    println!("Show password on console ? [Y/n]:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let show_password: bool;

    match input.trim() {
        "Y" | "y" => show_password = true,
        "N" | "n" => show_password = false,
        _ => show_password = false,
    }

    // Get standard project directories
    let config_dir: PathBuf = get_config_path();
    let config_file_path = config_dir.join("passgen.toml");

    fs::create_dir_all(config_dir)?;

    let mut options = Map::new();
    // options.insert("theme".to_string(), Value::String("dark".to_string())); Ex on String value
    options.insert("show_pass".to_string(), Value::Boolean(show_password));

    let mut encryption = Map::new();
    encryption.insert(
        "passgen_key".to_string(),
        Value::String("randomkey123".to_string()),
    );

    let mut config = Map::new();
    config.insert("options".to_string(), Value::Table(options));
    config.insert("encryption".to_string(), Value::Table(encryption));

    // Serialize the configuration to TOML
    let toml_str =
        toml::to_string(&Value::Table(config)).expect("Failed to serialize configuration to TOML");

    // Write to the configuration file
    let mut file = File::create(config_file_path)?;
    file.write_all(toml_str.as_bytes())?;

    println!("Default configuration created successfully.");
    Ok(())
}
