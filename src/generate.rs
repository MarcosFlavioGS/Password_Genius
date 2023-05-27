use crate::insert_pass::insert_pass;
use crate::clipboarding::clipboarder;
use crate::new_pass::new_password;

pub fn generate(path: &str) {
    let passwd = new_password();
    match clipboarder(&passwd) {
        Ok(_) => println!("Password copied to clipboard !"),
        Err(err) => println!(
            "Failed to copy to clipboard.\nError: {err}"
        ),
    }
    match insert_pass(path, &passwd) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => println!("Failed to insert password.\nError: {err}"),
    }
}
