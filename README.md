<div align="center">
  <img width=100% src="https://github.com/MarcosFlavioGS/Password_Genius/assets/95108526/8939958b-8dd5-46e8-8a9c-12e175157604" alt="PassGen Logo">
</div>

# PassGen

A secure and efficient password manager and generator written in Rust. PassGen helps you generate, store, and manage your passwords securely. You can use either a traditional CLI (subcommands) or an optional full-screen terminal UI built with [Ratatui](https://github.com/ratatui/ratatui).

**Version:** 1.11.0

## Features

- Full-screen **TUI** (`--tui` / `-t`) for browsing, generating, inserting, retrieving, deleting, and configuring without leaving the terminal
- Generate secure, random passwords (CLI or TUI)
- Copy passwords to clipboard automatically
- Encrypted password storage (ChaCha20-Poly1305)
- Organized password storage under your home directory
- Configurable settings (`~/.config/passgen/passgen.toml`)
- Import/Export functionality (coming soon)

## Requirements

- Rust programming language (latest stable version recommended)
- Linux/Unix-like system (for clipboard support: X11 via the `clipboard` crate, Wayland via `wl-copy` / `wl-paste`)
- A terminal that supports alternate-screen and raw mode (typical modern terminals)

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

   Or build from source with Cargo:
   ```bash
   cargo build --release
   ```

## Configuration

After installation, create your initial configuration (either subcommand or TUI):

```bash
passgen config
```

Or launch the TUI and use **Settings** (`s` on the main screen), then **Ctrl+S** to save.

This creates `~/.config/passgen/passgen.toml`. You can customize:

- Whether retrieved/generated passwords are shown on screen (`show_pass`)
- The encryption key used with Argon2 key derivation (`passgen_key`)

### Security Features

- **Encryption**: Passwords at rest are encrypted; key material is derived via Argon2 from `passgen_key` in config (see project docs for design tradeoffs).
- **Clipboard**: Retrieved/generated passwords can be copied to the clipboard from the CLI or TUI.
- **Storage**: One encrypted file per entry under `~/passgen/<name>/pass`.

## Usage

### Global options

```bash
passgen [OPTIONS] [COMMAND]
```

- `-t`, `--tui` — Run the full-screen terminal UI instead of a one-shot subcommand.

With no command and without `--tui`, `passgen` prints help.

### TUI mode

```bash
passgen --tui
# or
passgen -t
```

| Screen | Keys (summary) |
|--------|----------------|
| Main list | `j` / `k` or arrows — move; `Enter` — open entry; `g` — generate; `i` — insert; `d` — delete; `s` — settings; `q` — quit |
| Generate / Insert | `Tab` — switch fields; `Enter` — submit; `Esc` — back |
| Retrieve | `c` — copy to clipboard; `Esc` — back |
| Delete confirm | `y` / `n` or `Esc` |
| Settings | `Tab` — switch fields; `Space` — toggle “show password”; `Ctrl+S` — save; `Esc` — back (or quit if no config yet) |

### CLI subcommands

```bash
passgen list
passgen generate <name>
passgen insert <name>
passgen get <name>
passgen config
passgen delete <name>
```

### Examples

Generate a new password for GitHub (CLI):

```bash
passgen generate github
```

Open the TUI:

```bash
passgen -t
```

List all stored password entry names:

```bash
passgen list
```

## Project structure

```
src/
├── main.rs
├── cli.rs
├── tui/           # Ratatui full-screen UI (entry: tui::run)
├── password/      # Password management
├── config/        # Configuration (read / write / interactive create)
├── path/          # Path utilities
├── clipboard/     # Clipboard operations
├── generator/     # Password generation
├── directories/   # Directory listing for stored names
├── utils/         # Utility functions
└── encrypter/     # Encryption and key derivation
```

## Password storage layout

Entries are stored as:

```
~/passgen/
├── github/
│   └── pass
└── work/
    └── pass
```

Each `pass` file holds encrypted data (nonce + ciphertext).

## Contributing

Contributions are welcome. Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
