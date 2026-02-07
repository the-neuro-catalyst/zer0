#!/bin/bash
set -e

echo "🔧 COMPREHENSIVE ZERO SETUP FIX"
echo "=================================="

# 1. Fix Cargo.toml
echo "📝 Fixing Cargo.toml..."
sed -i 's/edition = "2024"/edition = "2021"/' Cargo.toml
sed -i 's/rust-version = "1.85.0"/rust-version = "1.85"/' Cargo.toml

# 2. Fix book.toml  
echo "📝 Fixing book.toml..."
sed -i 's/edition = "2024"/edition = "2021"/' book.toml

# 3. Fix .rustfmt.toml
echo "📝 Fixing .rustfmt.toml..."
sed -i 's/edition = "2024"/edition = "2021"/' .rustfmt.toml

# 4. Fix Dockerfiles
echo "📦 Fixing Dockerfiles..."
for dockerfile in docker/Dockerfile.*; do
  if [ -f "$dockerfile" ]; then
    service=$(basename "$dockerfile" | sed 's/Dockerfile\.//')
    cat > "$dockerfile" << EOF
FROM rust:1.85-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p $service

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/$service /usr/local/bin/
ENV APP_VERSION=\${VERSION:-0.1.0}
ENTRYPOINT ["$service"]
EOF
    echo "  ✓ Fixed docker/Dockerfile.$service"
  fi
done

# 5. Add workflow_dispatch to all CI/CD workflows
echo "⚙️ Adding workflow_dispatch triggers..."
add_workflow_dispatch() {
  local file=$1
  
  if grep -q "workflow_dispatch:" "$file"; then
    return  # Already has it
  fi
  
  # Create temp file with workflow_dispatch added
  awk '
  /^on:/ { 
    print $0
    getline
    print "  workflow_dispatch:"
    print $0
    next
  }
  { print }
  ' "$file" > "${file}.tmp"
  
  mv "${file}.tmp" "$file"
  echo "  ✓ $(basename $file)"
}

cd .github/workflows
for yml in ci.yml audit.yml e2e-tests.yml documentation.yml deploy.yml gemini-extension.yml; do
  if [ -f "$yml" ]; then
    add_workflow_dispatch "$yml"
  fi
done
cd ../..

# 6. Validate all YAML workflows
echo "✅ Validating workflows..."
for yml in .github/workflows/*.yml; do
  if command -v yamllint &> /dev/null; then
    yamllint "$yml" 2>/dev/null || echo "  ⚠️ $(basename $yml) has YAML issues"
  fi
done

# 7. Create .envrc for direnv (optional)
echo "🔧 Creating .envrc for development..."
cat > .envrc << 'EOF'
export ZERO_ENV=development
export ZERO_LOG_LEVEL=info
export RUST_BACKTRACE=1
export CARGO_TERM_COLOR=always

# Optional: Set these for local testing
# export SLACK_CI_WEBHOOK=https://hooks.slack.com/...
# export SLACK_DEPLOYMENTS_WEBHOOK=https://hooks.slack.com/...

use nix
EOF

chmod 644 .envrc

# 8. Create .gitignore additions if missing
echo "🚫 Updating .gitignore..."
if ! grep -q "\.envrc" .gitignore; then
  cat >> .gitignore << 'EOF'

# Development
.envrc
.direnv/
.env.local
.env.*.local

# Build artifacts
dist/
build/
target/

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Test artifacts
test_data/
playwright-report/
coverage/
.nyc_output/

# GitHub Actions
.act/
EOF
fi

# 9. Validate project structure
echo "📊 Validating project structure..."
required_files=(
  "Cargo.toml"
  "Cargo.lock"
  "README.md"
  "LICENSE"
  "Makefile"
  "zero.config.toml"
  ".github/workflows/ci.yml"
  ".github/workflows/audit.yml"
)

for file in "${required_files[@]}"; do
  if [ -f "$file" ]; then
    echo "  ✓ $file"
  else
    echo "  ❌ MISSING: $file"
  fi
done

# 10. Test Rust build
echo ""
echo "🧪 Testing Rust build..."
cargo check --quiet 2>/dev/null && echo "  ✓ Rust check passed" || echo "  ❌ Rust check failed"

# 11. Create CI test script
echo "📋 Creating local test script..."
cat > test-ci-locally.sh << 'EOF'
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
EOF

chmod +x test-ci-locally.sh
echo "  ✓ Created test-ci-locally.sh"

# 12. Summary
echo ""
echo "=================================="
echo "✅ SETUP COMPLETE!"
echo "=================================="
echo ""
echo "📋 Next steps:"
echo "1. Run: cargo check"
echo "2. Run: git add -A && git commit -m 'fix: correct Rust edition and CI/CD setup'"
echo "3. Run: git push"
echo "4. Check GitHub Actions: gh run list"
echo ""
echo "🧪 Test locally with:"
echo "  ./test-ci-locally.sh"
echo ""
echo "📚 Documentation:"
echo "  - Workflows: .github/WORKFLOWS.md"
echo "  - Secrets: .github/SECRETS_TEMPLATE.md"
echo ""