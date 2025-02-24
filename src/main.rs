mod clipboarding;
mod config;
mod encrypter;
mod generate;
mod get_directories;
mod get_path;
mod getter;
mod insert;
mod insert_pass;
mod new_pass;
mod utils;

use clipboarding::clipboarder;
use config::{create::create_default_config, read::read_config, Config};
use generate::generate;
use get_directories::get_directories;
use get_path::get_path;
use getter::getter;
use insert::insert;
use std::env;
use utils::get_path::get_base_path;

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
                let config: Config = read_config();
                if args.len() < 3 {
                    eprintln!("Arguments must be passed to generate.");
                } else {
                    let base_path: String = get_base_path(&args[2][..], "passgen/");

                    generate(&base_path, &config);
                }
            }
            "insert" | "i" => {
                let config: Config = read_config();
                if args.len() < 3 {
                    eprintln!("Arguments must be passed to insert.");
                } else {
                    let base_path: String = get_base_path(&args[2][..], "passgen/");

                    insert(&base_path, &config);
                }
            }
            "get" => {
                let config: Config = read_config();
                if args.len() < 3 {
                    eprintln!("Arguments must be passed to get.");
                } else {
                    let source = &args[2][..];

                    if let Ok(password) = getter(source, &config) {
                        if config.options.show_pass {
                            println!("Password for {source} is: {password}");
                        }
                        match clipboarder(&password[..]) {
                            Ok(_) => println!("Password copied to clipboard !"),
                            Err(err) => {
                                eprintln!("Failed to copy password to clipboard.\nError: {err}")
                            }
                        }
                    } else {
                        eprint!("Failed to get password from: {source}");
                    }
                }
            }
            "config" => match create_default_config() {
                Ok(_) => println!("Config file created at ~/.config/passgen/"),
                Err(err) => eprintln!("Error creating config file: Error: {err}"),
            },
            "export" => {
                // TODO: Export /passgen/ files into .zip
            },
            "import" => {
                // TODO: Unzip files into /passgen/ directory
            }
            _ => eprintln!("Command not found..."),
        },
        _ => panic!("Too many arguments !"),
    }
}
