#!/usr/bin/env sh

set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if the system is using Wayland
is_wayland() {
    # Check common Wayland environment variables
    if [ -n "$WAYLAND_DISPLAY" ] || [ "$(echo $XDG_SESSION_TYPE | tr '[:upper:]' '[:lower:]')" = "wayland" ]; then
        return 0  # true in shell scripting
    fi

    # Check if we're running on a known Wayland compositor
    if command_exists hyprctl || command_exists sway || ps aux | grep -q "[w]ayland"; then
        return 0  # true
    fi

    return 1  # false
}

# Function to install wl-clipboard based on the detected package manager
install_wl_clipboard() {
    if command_exists apt; then
        echo "Detected apt-based system. Installing wl-clipboard..."
        sudo apt update && sudo apt install -y wl-clipboard
    elif command_exists pacman; then
        echo "Detected Arch-based system. Installing wl-clipboard..."
        sudo pacman -S --noconfirm wl-clipboard
    elif command_exists dnf; then
        echo "Detected Fedora/RHEL-based system. Installing wl-clipboard..."
        sudo dnf install -y wl-clipboard
    elif command_exists zypper; then
        echo "Detected openSUSE-based system. Installing wl-clipboard..."
        sudo zypper install -y wl-clipboard
    elif command_exists apk; then
        echo "Detected Alpine Linux. Installing wl-clipboard..."
        sudo apk add wl-clipboard
    elif command_exists emerge; then
        echo "Detected Gentoo. Installing wl-clipboard..."
        sudo emerge -v x11-misc/wl-clipboard
    else
        echo "Unable to detect package manager. Please install wl-clipboard manually."
        echo "Visit https://github.com/bugaevc/wl-clipboard for installation instructions."
        return 1
    fi
    return 0
}

# Compile the project
echo "Compiling passgen..."
cargo build --release

# Get the target binary
BINARY_PATH="target/release/passgen"

# Define the installation path
INSTALL_PATH="/usr/local/bin/passgen"

# Copy the binary
echo "Installing to $INSTALL_PATH"
sudo cp "$BINARY_PATH" "$INSTALL_PATH"
sudo chmod +x "$INSTALL_PATH"

# Check if system is using Wayland
if is_wayland; then
    echo "Wayland detected (possibly Hyprland or another Wayland compositor)"

    # Check if wl-clipboard is already installed
    if ! command_exists wl-copy; then
        echo "wl-clipboard is required for clipboard functionality on Wayland"
        echo "Would you like to install wl-clipboard now? (y/n)"
        read -r install_choice

        if [ "$install_choice" = "y" ] || [ "$install_choice" = "Y" ]; then
            if install_wl_clipboard; then
                echo "wl-clipboard installed successfully"
            else
                echo "Please install wl-clipboard manually for full clipboard functionality"
            fi
        else
            echo "wl-clipboard was not installed. Clipboard functionality may be limited."
            echo "You can install it later using your package manager."
        fi
    else
        echo "wl-clipboard is already installed. Good to go!"
    fi
fi

echo "Installation complete. Run 'passgen config' to run configuration."
