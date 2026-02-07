#!/bin/bash
# ZERO TUI: Build and package for operational use
set -e # Exit on error

echo "Building TUI (Release)..."
cargo build --release -p tui

if command -v cargo-deb &> /dev/null; then
    echo "Generating Debian package..."
    cargo deb -p tui --no-build
else
    echo "Notice: 'cargo-deb' not found. Skipping .deb package generation."
    echo "To generate .deb, install with: cargo install cargo-deb"
fi

echo "Artifact: target/release/zero-tui"