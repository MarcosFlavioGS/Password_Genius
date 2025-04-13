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

use clap::{Parser, Subcommand};
use clipboarding::clipboarder;
use config::{create::create_default_config, read::read_config, Config};
use generate::generate;
use get_directories::get_directories;
use get_path::get_path;
use getter::getter;
use insert::insert;
use utils::get_path::get_base_path;

/// A secure password manager and generator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all stored passwords
    List,
    
    /// Generate a new password
    Generate {
        /// The name/identifier for the password
        name: String,
    },
    
    /// Insert a new password
    Insert {
        /// The name/identifier for the password
        name: String,
    },
    
    /// Get a stored password
    Get {
        /// The name/identifier of the password to retrieve
        name: String,
    },
    
    /// Create a new configuration file
    Config,
    
    /// Export passwords (TODO)
    Export,
    
    /// Import passwords (TODO)
    Import,
}

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
