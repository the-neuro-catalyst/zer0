# ZERO: Systemic Utility Makefile

# Config
BINARY_CLI = zero-cli
BINARY_TUI = zero-tui
PREFIX = /usr/local/bin

# --- BUILD ---
.PHONY: all build build-cli build-tui build-ui
all: build

build: build-cli build-tui build-ui

build-cli:
	@echo "Building CLI..."
	@cargo build --release -p cli

build-tui:
	@echo "Building TUI..."
	@cargo build --release -p tui

build-ui:
	@echo "Building GUI..."
	@cd crates/ui && pnpm tauri build

# --- INSTALL (Local Infiltration) ---
.PHONY: install
install: build-cli build-tui
	@echo "Installing binaries to $(PREFIX)..."
	@sudo cp target/release/$(BINARY_CLI) $(PREFIX)/$(BINARY_CLI)
	@sudo cp target/release/$(BINARY_TUI) $(PREFIX)/$(BINARY_TUI)
	@sudo chmod +x $(PREFIX)/$(BINARY_CLI) $(PREFIX)/$(BINARY_TUI)

# --- DEVELOPMENT ---
.PHONY: dev-ui dev-tui
dev-ui:
	cd crates/ui && pnpm tauri dev

dev-tui:
	cargo run -p tui

# --- CLEANING (The Janitor's Duty) ---
.PHONY: clean
clean:
	@echo "Cleaning artifacts..."
	@cargo clean
	@rm -rf crates/ui/dist
	@rm -rf artifacts/

# --- INTEGRITY ---
.PHONY: check test lint
check:
	cargo check --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings
	cargo fmt --all -- --check