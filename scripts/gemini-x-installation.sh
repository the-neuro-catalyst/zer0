#!/bin/bash
# ZERO GEMINI EXTENSION: Build and package for infiltration

echo "Building GEMINI Extension (Release)..."
cd extensions/zero-gemini-extension

if ! command -v pnpm &> /dev/null; then
    echo "Error: pnpm is not installed. Please install pnpm first."
    exit 1
fi

pnpm install 
pnpm run build

if command -v gemini &> /dev/null; then
    echo "Linking extension to Gemini CLI..."
    gemini extensions link .
else
    echo "Warning: 'gemini' CLI not found in PATH."
    echo "Extension built successfully, but automatic linking skipped."
    echo "To link manually, run: gemini extensions link $(pwd)"
fi