use clap::{Parser, Subcommand};

/// A secure password manager and generator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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
