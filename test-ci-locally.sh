#!/bin/bash
set -e

echo "🧪 Running local CI tests..."

echo "1️⃣ Format check..."
cargo fmt --all -- --check

echo "2️⃣ Lint check..."
cargo clippy --workspace --all-targets -- -D warnings

echo "3️⃣ Unit tests..."
cargo test --lib --workspace --quiet

echo "4️⃣ Build release..."
cargo build --release --workspace

echo ""
echo "✅ All checks passed!"
