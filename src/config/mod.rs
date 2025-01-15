pub mod create;
pub mod read;

use serde::{Deserialize, Serialize};

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
