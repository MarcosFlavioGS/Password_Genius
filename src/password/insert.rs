use crate::{config::Config, password::insert_pass::insert_pass};
use std::io;

///
/// Gets a pass from the user and inserts it to the path file
///
pub fn insert(path: &str, config: &Config) {
    println!("Write your password:\n");
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read line");
    match insert_pass(path, &pass, config) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => println!("Failed to insert password.\nError: {err}"),
    }
}
