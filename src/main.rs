mod clipboarding;
mod generate;
mod getter;
mod insert;
mod insert_pass;
mod new_pass;

use clipboarding::clipboarder;
use generate::generate;
use getter::getter;
use insert::insert;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("TODO: Show filetree of all created passwords."),
        (2..=3) => match &args[1][..] {
            "generate" => {
                let filepath = &args[2][..];
                let mut base_path = String::from("passwords/");
                base_path.push_str(filepath);
                base_path.push_str("/pass");
                generate(&base_path);
            }
            "insert" => {
                let filepath = &args[2][..];
                let mut base_path = String::from("passwords/");
                base_path.push_str(filepath);
                base_path.push_str("/pass");
                insert(&base_path);
            }
            "get" => {
                let source = &args[2][..];
                let password = getter(source);
                println!("Password for {source} is: {password}");
                match clipboarder(&password[..]) {
                    Ok(_) => println!("Copied to clipboard"),
                    Err(err) => println!("Failed to read line.\nError: {err}"),
                }
            }
            _ => println!("Command not found..."),
        },
        _ => panic!("Too many arguments !"),
    }
}
