use dirs;

pub fn get_path() -> String {
    let home_dir = dirs::home_dir()
        .ok_or("Failed to get home directory")
        .unwrap();
    let full_path = home_dir.join("passgen/");
    let path = full_path.to_str().unwrap();
    path.to_string()
}
