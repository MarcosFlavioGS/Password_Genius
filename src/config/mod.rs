pub mod create;
pub mod read;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub options: Options,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub show_pass: bool,
}
