use crate::insert_pass::insert_pass;
use std::io;

pub fn insert(path: &str) {
    println!("Write your password:\n");
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read line");
    match insert_pass(path, &pass) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => println!("Failed to insert password.\nError: {err}"),
    }
}
