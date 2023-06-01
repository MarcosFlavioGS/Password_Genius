use std::fs;

pub fn get_directories(path: &str) -> Vec<String> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    directories.push(entry_path.display().to_string());
                    directories.extend(get_directories(entry_path.to_str().unwrap()));
                }
            }
        }
    }

    directories
}
