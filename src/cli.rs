use clap::{Parser, Subcommand};

/// A secure password manager and generator.
///
/// Without `-t`/`--tui`, pass a subcommand (or run with no arguments to print help).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Run the full-screen terminal UI (Ratatui) instead of a one-shot subcommand.
    #[arg(short = 't', long = "tui")]
    pub tui: bool,

    /// Subcommand to run (omit when using `--tui`).
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all stored passwords
    List,

    /// Generate a new password
    Generate {
        /// The name/identifier for the password
        name: String,
    },

    /// Insert a new custom password
    Insert {
        /// The name/identifier for the password
        name: String,
    },

    /// Get a stored password by name
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

    /// Removes a password
    Delete {
        ///the name/identifier for the password to be deleted
        name: String,
    },
}
