pub mod create;
pub mod read;
pub mod write;

use serde::{Deserialize, Serialize};

/// Default values used when creating a new config (CLI wizard or TUI).
pub fn default_config() -> Config {
    Config {
        options: Options { show_pass: false },
        encryption: Encryption {
            passgen_key: "randomkey123".to_string(),
        },
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub options: Options,
    pub encryption: Encryption,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub show_pass: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Encryption {
    pub passgen_key: String,
}
