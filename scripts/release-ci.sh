#!/bin/bash
# ZERO CLI: Build and package for infiltration
set -e # Exit on error

echo "Building CLI (Release)..."
cargo build --release -p cli

if command -v cargo-deb &> /dev/null; then
    echo "Generating Debian package..."
    cargo deb -p cli --no-build
else
    echo "Notice: 'cargo-deb' not found. Skipping .deb package generation."
    echo "To generate .deb, install with: cargo install cargo-deb"
fi

echo "Artifact: target/release/zero-cli"