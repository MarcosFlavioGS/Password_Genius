use dirs;
use std::fs::{self};

pub fn getter(source: &str) -> String {
    let home_dir = dirs::home_dir().expect("Failed to find dir");
    let file_path = home_dir.join("passwords").join(source).join("pass");
    let content = fs::read_to_string(file_path).expect("Failed to read from file");
    content
}
