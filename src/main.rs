mod clipboard;
mod cli;
mod config;
mod directories;
mod encrypter;
mod generator;
mod path;
mod password;
mod utils;

use clap::Parser;
use clipboard::clipboarder;
use cli::{Cli, Commands};
use config::{create::create_default_config, read::read_config, Config};
use directories::get_directories;
use generator::generate;
use path::get_path;
use password::{getter::getter, insert::insert};
use utils::get_path::get_base_path;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let path = get_path();
            let directories = get_directories(&path);
            for directory in directories {
                println!("{}", directory);
            }
        }
        Commands::Generate { name } => {
            let config: Config = read_config();
            let base_path: String = get_base_path(&name, "passgen/");
            generate(&base_path, &config);
        }
        Commands::Insert { name } => {
            let config: Config = read_config();
            let base_path: String = get_base_path(&name, "passgen/");
            insert(&base_path, &config);
        }
        Commands::Get { name } => {
            let config: Config = read_config();
            if let Ok(password) = getter(&name, &config) {
                if config.options.show_pass {
                    println!("Password for {name} is: {password}");
                }
                match clipboarder(&password[..]) {
                    Ok(_) => println!("Password copied to clipboard !"),
                    Err(err) => {
                        eprintln!("Failed to copy password to clipboard.\nError: {err}")
                    }
                }
            } else {
                eprint!("Failed to get password from: {name}");
            }
        }
        Commands::Config => match create_default_config() {
            Ok(_) => println!("Config file created at ~/.config/passgen/"),
            Err(err) => eprintln!("Error creating config file: Error: {err}"),
        },
        Commands::Export => {
            println!("Export functionality coming soon!");
        },
        Commands::Import => {
            println!("Import functionality coming soon!");
        }
    }
}
