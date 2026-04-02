use std::fs;
use std::path::Path;

use crate::path::config::get_config_path_str;

use super::Config;

/// Writes the full config to `~/.config/passgen/passgen.toml`, creating parent directories as needed.
pub fn write_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = get_config_path_str();
    let path = Path::new(&path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}
