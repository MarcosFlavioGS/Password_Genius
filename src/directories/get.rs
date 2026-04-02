use std::fs;

///
/// Returns directory names under `path` that contain a `pass` file (encrypted secret).
///
/// Only entries with `…/<name>/pass` as a regular file are listed, so empty folders left after
/// deleting the `pass` file do not appear.
///
pub fn get_directories(path: &str) -> Vec<String> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let pass_file = entry_path.join("pass");
                if pass_file.is_file() {
                    if let Some(dir_name) = entry_path.file_name().and_then(|name| name.to_str()) {
                        directories.push(dir_name.to_string());
                    }
                }

                directories.extend(get_directories(entry_path.to_str().unwrap()));
            }
        }
    }

    directories
}
