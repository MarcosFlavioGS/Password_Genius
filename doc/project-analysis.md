# Project analysis: `passgen`

**Version:** 1.11.0 (from `Cargo.toml`)  
**Edition:** Rust 2021  
**Binary name:** `passgen` (package name in manifest)

## Purpose

`passgen` is a **CLI password manager** for the local machine. It stores **named** passwords under the user’s home directory, encrypts them at rest with **ChaCha20-Poly1305**, and can copy retrieved passwords to the **clipboard** (X11 via the `clipboard` crate, Wayland via `wl-copy` / `wl-paste`). You can use **one-shot subcommands** or a **full-screen TUI** (`-t` / `--tui`) built with **Ratatui** for list/generate/insert/get/delete/config without stdin prompts for those flows.

## Architecture (high level)

| Area | Role |
|------|------|
| `src/main.rs` | Entry point: parses CLI with `clap`; if `--tui`, runs `tui::run`; else dispatches optional subcommand or prints help. |
| `src/cli.rs` | `clap` definitions: global `--tui`, optional subcommands `List`, `Generate`, `Insert`, `Get`, `Config`, `Delete`, stub `Export` / `Import`. |
| `src/tui/` | Ratatui main loop (`run`), `app` (screens, keys), `ui` (layout), `filter` (fzf-style subsequence match for the main list). Uses `read_config_result`, `generate_stored`, `insert_password`, `write_config`, etc. |
| `src/config/` | TOML: `options.show_pass`, `encryption.passgen_key`. Read under `~/.config/passgen/passgen.toml`; `write::write_config` persists edits; `default_config` for first-run defaults; `read_config_result` for non-panicking load (TUI). |
| `src/path/` | Resolves `~/passgen/` (list base) and config directory paths via `dirs`. |
| `src/utils/` | Builds storage path segments, e.g. `passgen/<name>/pass`. |
| `src/generator/` | CLI: interactive length via `new_password`; shared generation via `generate_password_at_length`; `generate_stored` for TUI (store + clipboard, returns plaintext for status). |
| `src/password/` | `insert_pass` (encrypt + write file), `getter` (read + decrypt), `new_pass` (generator). |
| `src/encrypter/` | `derive` (Argon2 key derivation from config string), `encrypt` / `decrypt` (ChaCha20-Poly1305). `hasher` (Argon2 password hash) is **declared but not referenced** elsewhere. |
| `src/inserter/` | CLI reads from stdin; `insert_password` trims line endings for TUI/programmatic insert. |
| `src/deleter/` | Deletes the on-disk file for a named entry. |
| `src/directories/` | Recursive directory listing under the passgen root (for `List` / TUI list). |
| `src/clipboard/` | Clipboard copy with Wayland vs X11 branching and optional verification. |

## Data layout

- **Config:** `~/.config/passgen/passgen.toml` — created by `passgen config` (interactive) or TUI settings (Ctrl+S).
- **Secrets:** One file per logical name: `~/passgen/<name>/pass` (binary: 12-byte nonce + ciphertext).

`Get` resolves `~/passgen/<name>/pass`; generate/insert/delete use the same layout via `get_base_path` + `home_dir.join(path)`.

## Dependencies (intent)

| Crate | Use |
|-------|-----|
| `clap` | CLI parsing and help. |
| `ratatui` | Full-screen TUI (with `crossterm` backend feature). |
| `crossterm` | Input events and terminal integration (used by Ratatui). |
| `rand` | Password character sampling. |
| `serde` / `toml` | Config (de)serialization. |
| `dirs` | Home and standard paths. |
| `chacha20poly1305` | AEAD for stored secrets. |
| `argon2` | Key derivation (`derive`) and unused `hasher` helper. |
| `clipboard` | X11 clipboard. |

## Security-relevant notes (for maintainers)

- **Key material** comes from the user-chosen `passgen_key` in config, passed through **Argon2** in `derive` with a **fixed salt** (`derive.rs`). Storing the “master” string in a file on disk is a design tradeoff; rotating or hardening this path would be a product decision.
- **Encryption errors** in `insert_pass` are logged but the function can still return `Ok(())` if encryption fails (worth aligning error handling with behavior).
- **`read_config`** still uses `unwrap` on file read and TOML parse — missing or invalid config will **panic** at runtime (CLI paths). The **TUI** uses **`read_config_result`** to avoid panicking on startup.

## User-facing commands (behavioral summary)

- **`--tui` / `-t`** — Full-screen UI: main screen lists entries with a **filter line** — typing filters names with **fzf-style subsequence** matching (case-insensitive); **↑/↓** move selection, **Enter** opens get, **Esc** clears the filter. Actions on the main screen use **Ctrl+G** (generate), **Ctrl+I** (insert), **Ctrl+D** (delete), **Ctrl+S** (settings), **Ctrl+Q** (quit). Other screens: insert (masked password field), get (copy with `c`), delete (confirm), settings (write TOML with Ctrl+S). If config is missing, opens first-run settings; Esc with no saved config quits.
- **`list`** — Prints directory names under `~/passgen/` (recursive).
- **`generate <name>`** — Prompts for length, generates password, writes encrypted file, copies to clipboard.
- **`insert <name>`** — Reads password from stdin, encrypts, writes file.
- **`get <name>`** — Decrypts and optionally prints; always attempts clipboard copy.
- **`config`** — Interactive creation of default TOML.
- **`delete <name>`** — Removes the file at `~/passgen/<name>/pass`.
- **`export` / `import`** — Placeholders only (“coming soon”).
- **No subcommand, no `--tui`** — Prints help (same as empty invocation).

## Development tasks (Just)

The repo includes a **`Justfile`** at the project root ([Just](https://github.com/casey/just) command runner). Common commands: `just build`, `just release`, `just run -- --tui`, `just test`, `just fmt`, `just clippy`, `just check` (fmt check + clippy + test), `just install` (runs `./install.sh` for system install and optional Wayland deps), `just install-binary` (release build + copy to `/usr/local/bin/passgen` only). Run `just` or `just --list` to list recipes.

## Quality and maintenance observations

- **Unused module:** `encrypter::hasher` is never imported outside `encrypter/mod.rs`.
- **Consistency:** Mix of `unwrap`/`expect` (especially `read_config` in CLI) vs `Result` in TUI and `read_config_result`; good candidate for gradual hardening.
- **Copy/paste UX:** Generator error message says “Failed to read line” on clipboard failure (cosmetic).
- **Typos / strings:** e.g. decrypt error string “Uncable to decrypt…”, config prompt “Chose a secret…”.

## Suggested directions (non-blocking)

- Implement or remove `Export` / `Import` and the unused `hasher` path.
- Return `Result` from `read_config` or validate before use in CLI paths.
- Add integration tests with a temporary `HOME` and fixed config for deterministic runs.

This document is the **canonical overview** for agents and humans working in this repository; keep it updated when behavior or layout changes materially.
