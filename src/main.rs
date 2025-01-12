mod clipboarding;
mod encrypter;
mod generate;
mod get_directories;
mod get_path;
mod getter;
mod insert;
mod insert_pass;
mod new_pass;

use clipboarding::clipboarder;
use generate::generate;
use get_directories::get_directories;
use get_path::get_path;
use getter::getter;
use insert::insert;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let path = get_path();
            let directories = get_directories(&path);
            for directory in directories {
                println!("{}", directory);
            }
        }
        (2..=3) => match &args[1][..] {
            "version" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("v{version}");
            }
            "generate" | "g" => {
                let mut base_path = String::from("passgen/");
                base_path.push_str(&args[2][..]);
                base_path.push_str("/pass");
                generate(&base_path);
            }
            "insert" | "i" => {
                let mut base_path = String::from("passgen/");
                base_path.push_str(&args[2][..]);
                base_path.push_str("/pass");
                insert(&base_path);
            }
            "get" => {
                let source = &args[2][..];
                if let Ok(password) = getter(source) {
                    println!("Password for {source} is: {password}");
                    match clipboarder(&password[..]) {
                        Ok(_) => println!("Copied to clipboard"),
                        Err(err) => eprintln!("Failed to read line.\nError: {err}"),
                    }
                } else {
                    eprint!("Failed to get password from: {source}");
                }
            }
            _ => eprintln!("Command not found..."),
        },
        _ => panic!("Too many arguments !"),
    }
}
