use crate::insert_pass::insert_pass;
use crate::new_pass::new_password;
use crate::clipboarding::clipboarder;

pub fn generate(path: &str) {
    let passwd = new_password();
    match insert_pass(path, &passwd) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => println!("Failed to insert password.\nError: {err}"),
    }
    match clipboarder(&passwd[..]) {
        Ok(_) => println!("Copied to clipboard"),
        Err(err) => println!(
            "Failed to read line.\nError: {err}"
        ),
    }
}
