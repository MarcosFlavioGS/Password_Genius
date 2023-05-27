mod clipboarding;
mod new_pass;
mod insert_pass;

use clipboarding::clipboarder;
use new_pass::new_password;
use insert_pass::insert_pass;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("TODO: Show filetree of all created passwords."),
        (2..=3) => {
            match &args[1][..] {
                "generate" => {
                    let _passwd = new_password();
                    match clipboarder(_passwd) {
                        Ok(_) => println!("Password copied to clipboard !"),
                        Err(err) => println!(
                            "Failed to copy to clipboard.\nError: {err}"
                        ),
                    }
                },
                "insert" => {
                    let filepath = &args[2][..];
                    match insert_pass(filepath) {
                        Ok(_) => println!("Iserted at: {filepath}"),
                        Err(err) => println!("Failed to insert password.\nError: {err}"),
                    }
                },
                _ => println!("Do nothing !"),
            }
        },
        _ => panic!("Too many arguments !"),
    }
}
