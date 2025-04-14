use crate::clipboard::clipboarder::clipboarder;
use crate::config::Config;
use crate::password::{insert_pass::insert_pass, new_pass::new_password};

///
/// Generates a new pass: String, stores into path: &str and copies to clipboard.
///
pub fn generate(path: &str, config: &Config) {
    let passwd = new_password();
    if config.options.show_pass {
        println!("This is your new password: {passwd}");
    }

    match insert_pass(path, &passwd, config) {
        Ok(_) => println!("Inserted at: {path}"),
        Err(err) => eprintln!("Failed to insert password.\nError: {err}"),
    }
    match clipboarder(&passwd[..]) {
        Ok(_) => println!("Copied to clipboard"),
        Err(err) => eprintln!("Failed to read line.\nError: {err}"),
    }
}
