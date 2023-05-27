mod clipboarding;
mod new_pass;
mod insert_pass;
mod insert;
mod generate;
mod getter;

use generate::generate;
use insert::insert;
use getter::getter;
use std::env;
use clipboarding::clipboarder;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("TODO: Show filetree of all created passwords."),
        (2..=3) => {
            match &args[1][..] {
                "generate" => {
                    let filepath = &args[2][..];
                    let mut base_path = String::from("passwords/");
                    base_path.push_str(filepath);
                    base_path.push_str("/pass");
                    generate(&base_path);
                },
                "insert" => {
                    let filepath = &args[2][..];
                    let mut base_path = String::from("passwords/");
                    base_path.push_str(filepath);
                    base_path.push_str("/pass");
                    insert(&base_path);
                },
                "get" => {
                    let source = &args[2][..];
                    let password = getter(source);
                    println!("Password for {source} is: {password}");
                    match clipboarder(&password[..]) {
                        Ok(_) => println!("Copied to clipboard"),
                        Err(err) => println!(
                            "Failed to read line.\nError: {err}"
                        ),
                    }
                },
                _ => println!("Do nothing !"),
            }
        },
        _ => panic!("Too many arguments !"),
    }
}
