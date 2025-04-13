<div align="center">
  <img width=100% src="https://github.com/MarcosFlavioGS/Password_Genius/assets/95108526/8939958b-8dd5-46e8-8a9c-12e175157604" alt="PassGen Logo">
</div>

# PassGen

A secure and efficient password manager and generator written in Rust. PassGen helps you generate, store, and manage your passwords securely while providing a simple command-line interface.

## Features

- 🔒 Generate secure, random passwords
- 📋 Copy passwords to clipboard automatically
- 🔐 Encrypted password storage
- 📁 Organized password storage structure
- ⚙️ Configurable settings
- 🔄 Import/Export functionality (coming soon)

## Requirements

- Rust programming language (latest stable version recommended)
- Linux/Unix-like system (for clipboard support)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/MarcosFlavioGS/Password_Genius.git
   ```

2. Navigate to the project directory and run the installation script:
   ```bash
   cd Password_Genius
   ./install.sh
   ```

## Configuration

After installation, you need to create your initial configuration:

```bash
passgen config
```

This will create a configuration file at `~/.config/passgen/passgen.toml` with default settings. You can customize:

- Password display preferences
- Encryption settings
- Default password length
- And more...

### Security Features

- **Encryption**: All passwords are encrypted using a configurable secret key
- **Clipboard Management**: Passwords are automatically copied to clipboard and cleared after use
- **Secure Storage**: Passwords are stored in encrypted files with proper permissions

## Usage

PassGen provides a simple command-line interface with the following commands:

```bash
passgen [COMMAND] [OPTIONS]
```

### Commands

- `list` - List all stored passwords
- `generate <name>` - Generate a new password for the specified identifier
- `insert <name>` - Insert a new password for the specified identifier
- `get <name>` - Retrieve a stored password
- `config` - Create or update configuration
- `export` - Export passwords (coming soon)
- `import` - Import passwords (coming soon)

### Examples

Generate a new password for GitHub:
```bash
passgen generate github
```

Insert a custom password:
```bash
passgen insert github
```

Retrieve a stored password:
```bash
passgen get github
```

List all stored passwords:
```bash
passgen list
```

## Project Structure

The codebase is organized into logical modules:

```
src/
├── main.rs
├── cli.rs
├── password/      # Password management
├── config/        # Configuration handling
├── path/          # Path utilities
├── clipboard/     # Clipboard operations
├── generator/     # Password generation
├── directories/   # Directory management
├── utils/         # Utility functions
└── encrypter/     # Encryption functionality
```

## Password Storage

Passwords are stored in an organized directory structure:

```
~/.local/share/passgen/
├── github/
│   └── pass
├── twitter/
│   └── pass
└── work/
    └── pass
```

Each password is stored in its own encrypted file, making it easy to manage and retrieve.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
