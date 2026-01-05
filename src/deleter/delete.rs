use dirs;
use std::fs;

///
/// Deletes a pass using the path
///
pub fn delete(path: &str) {
    match delete_pass(path) {
        Ok(_) => println!("Pass folder at: {path} deleted."),
        Err(err) => println!("Failed to delete password.\nError: {err}"),
    }
}

fn delete_pass(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let full_path = home_dir.join(path);
    fs::remove_file(full_path)?;
	Ok(())
}
