#!/bin/bash
set -euo pipefail

# Setup script for fastset development environment
# Installs Rust toolchain and fetches crate dependencies for offline work.

# Detect package manager
if command -v apt-get >/dev/null; then
    PM_UPDATE="apt-get update"
    PM_INSTALL="apt-get install -y"
else
    echo "Unsupported package manager. Please install build-essential, curl, and git manually." >&2
    exit 1
fi

SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    SUDO="sudo"
fi

# Install OS packages
$SUDO $PM_UPDATE
$SUDO $PM_INSTALL build-essential curl git pkg-config libssl-dev

# Install rustup if not already installed
if ! command -v rustup >/dev/null; then
    echo "Installing rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
else
    echo "rustup already installed"
fi

# Ensure cargo bin path is in PATH for future sessions
if ! grep -q cargo/bin ~/.bashrc; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
fi

# Install stable toolchain and common components
rustup install stable
rustup default stable
rustup component add clippy rustfmt

# Prefetch crate dependencies for offline work
cargo fetch

# Run tests to verify everything works
cargo test

echo "Development environment setup complete." 
