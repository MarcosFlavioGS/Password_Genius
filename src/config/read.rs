use crate::path::config::get_config_path_str;
use std::fs;
use toml;

use super::Config;

///
/// Read config file into a config: Config
///
pub fn read_config() -> Config {
    read_config_result().unwrap_or_else(|e| panic!("Failed to read config: {e}"))
}

/// Loads config from disk without panicking on missing or invalid files.
///
/// Used by the TUI and any caller that must recover gracefully.
pub fn read_config_result() -> Result<Config, String> {
    let config_path: String = get_config_path_str();
    let config_data =
        fs::read_to_string(&config_path).map_err(|e| format!("Cannot read {config_path}: {e}"))?;
    toml::from_str(&config_data).map_err(|e| format!("Invalid TOML in config: {e}"))
}
