mod cli;
mod clipboard;
mod config;
mod deleter;
mod directories;
mod encrypter;
mod generator;
mod import_export;
mod inserter;
mod password;
mod path;
mod tui;
mod utils;

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::CommandFactory;
use clap::Parser;
use cli::{Cli, Commands};
use clipboard::clipboarder::clipboarder;
use config::{create::create_default_config, read::read_config, Config};
use deleter::delete::delete;
use directories::get::get_directories;
use generator::gen::generate;
use inserter::insert::insert;
use password::getter::getter;
use path::config::get_path;
use utils::get_path::get_base_path;

fn default_export_path() -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(format!("passgen-export-{ts}.zip"))
}

fn main() {
    let cli = Cli::parse();

    if cli.tui {
        if let Err(e) = tui::run() {
            eprintln!("TUI error: {e}");
            std::process::exit(1);
        }
        return;
    }

    match cli.command {
        Some(Commands::List) => {
            let path = get_path();
            let directories = get_directories(&path);
            for directory in directories {
                println!("{}", directory);
            }
        }
        Some(Commands::Generate { name }) => {
            let config: Config = read_config();
            let base_path: String = get_base_path(&name, "passgen/");
            generate(&base_path, &config);
        }
        Some(Commands::Insert { name }) => {
            let config: Config = read_config();
            let base_path: String = get_base_path(&name, "passgen/");
            insert(&base_path, &config);
        }
        Some(Commands::Get { name }) => {
            let config: Config = read_config();
            match getter(&name, &config) {
                Ok(password) => {
                    if config.options.show_pass {
                        println!("Password for {name} is: {password}");
                    }
                    match clipboarder(&password[..]) {
                        Ok(_) => println!("Password copied to clipboard !"),
                        Err(err) => {
                            eprintln!("Failed to copy password to clipboard.\nError: {err}")
                        }
                    }
                }
                Err(err) => {
                    eprint!("Failed to get password from: {name}; Error: {err}");
                }
            }
        }
        Some(Commands::Config) => match create_default_config() {
            Ok(_) => println!("Config file created at ~/.config/passgen/"),
            Err(err) => eprintln!("Error creating config file: Error: {err}"),
        },
        Some(Commands::Delete { name }) => {
            let base_path: String = get_base_path(&name, "passgen/");
            delete(&base_path);
        }
        Some(Commands::Export { output }) => {
            let out = output.unwrap_or_else(default_export_path);
            match import_export::export_passgen(&out) {
                Ok(()) => println!("Exported password store to {}", out.display()),
                Err(e) => {
                    eprintln!("Export failed: {e}");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Import { path }) => match import_export::import_passgen(&path) {
            Ok(stats) => println!(
                "Import finished: {} file(s) added, {} skipped (already present).",
                stats.files_added, stats.files_skipped_existing
            ),
            Err(e) => {
                eprintln!("Import failed: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
        }
    }
}
