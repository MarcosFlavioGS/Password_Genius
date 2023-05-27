use std::fs::File;
use std::io::prelude::*;

pub fn insert_pass(path: &str, pass: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path).expect("Failed creating file");
    file.write_all(pass.as_bytes()).expect("Failed to write to file !");
    Ok(())
}
