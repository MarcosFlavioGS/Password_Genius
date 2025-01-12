use std::fs;

///
/// Returns a vector containing all created folders.
/// Gets all dirs with full paths and returns just the file_name.
/// 
pub fn get_directories(path: &str) -> Vec<String> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Push only the directory name
                    if let Some(dir_name) = entry_path.file_name().and_then(|name| name.to_str()) {
                        directories.push(dir_name.to_string());
                    }

                    // Recur to process subdirectories
                    directories.extend(get_directories(entry_path.to_str().unwrap()));
                }
            }
        }
    }

    directories
}
