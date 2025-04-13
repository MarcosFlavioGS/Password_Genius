use std::fs;
use toml;
use crate::path::get_config_path_str;

use super::Config;

///
/// Read config file into a config: Config
///
pub fn read_config() -> Config {
    let config_path: String = get_config_path_str();
    let config_data = fs::read_to_string(&config_path).unwrap();
    let config = toml::from_str(&config_data).unwrap();

    config
}
