#!/usr/bin/env sh

set -e

# Compile the project
cargo build --release

# Get the target binary
BINARY_PATH="target/release/passgen"

# Define the installation path
INSTALL_PATH="/usr/local/bin/passgen"

# Copy the binary
echo "Installing to $INSTALL_PATH"
sudo cp "$BINARY_PATH" "$INSTALL_PATH"
sudo chmod +x "$INSTALL_PATH"

echo "Installation complete. Run 'passgen config' to run configuration."
