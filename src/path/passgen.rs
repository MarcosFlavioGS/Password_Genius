use std::path::PathBuf;

use dirs;

/// Returns the absolute path to `~/passgen` (the password store root).
pub fn passgen_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join("passgen")
}
