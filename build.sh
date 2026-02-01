#!/bin/bash

# Build script for mytop

echo "Building mytop..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Build in release mode
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful!"
    echo "Binary location: target/release/mytop"
    echo "Binary size: $(du -h target/release/mytop | cut -f1)"
    echo ""
    echo "To install system-wide, run:"
    echo "  sudo cp target/release/mytop /usr/local/bin/"
    echo ""
    echo "To run mytop:"
    echo "  ./target/release/mytop"
else
    echo "Build failed!"
    exit 1
fi
