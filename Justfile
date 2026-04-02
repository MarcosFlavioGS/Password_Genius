# Passgen — task runner (https://github.com/casey/just)
# Run `just` or `just --list` to see recipes. Run `just <recipe>` to execute.

# Fail fast on errors in any recipe line (`set -e`); `-u` catches unset variables.
set shell := ["sh", "-eu", "-c"]

# Where `install-binary` copies the release artifact (matches install.sh).
install_path := "/usr/local/bin/passgen"
binary := "target/release/passgen"

# Show recipes (default when you run `just` with no arguments)
default:
	@just --list

# Debug build
build:
	cargo build

# Optimized binary under target/release/
release:
	cargo build --release

# Run the crate from source; forward args to the binary (e.g. `just run -- --tui`)
run *args:
	cargo run -- {{args}}

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features

# Format check + clippy + tests (handy before a commit)
check: fmt-check clippy test

fmt-check:
	cargo fmt --all -- --check

# Full installer: release build, sudo install to {{ install_path }}, Wayland / wl-clipboard prompts
install:
	sh ./install.sh

# Only build release and copy the binary to {{ install_path }} (no wl-clipboard / Wayland prompts)
install-binary: release
	sudo cp "{{binary}}" "{{install_path}}"
	sudo chmod +x "{{install_path}}"

clean:
	cargo clean
