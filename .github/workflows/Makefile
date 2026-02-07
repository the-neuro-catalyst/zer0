.PHONY: help setup lint test security-audit build release clean

help:
	@echo "ZERO Development Commands"
	@echo "========================="
	@echo "setup              - Install all dependencies"
	@echo "lint               - Run all linters (cargo fmt + clippy + ESLint)"
	@echo "test               - Run all tests (unit + integration + doc)"
	@echo "coverage           - Generate code coverage report"
	@echo "security-audit     - Run security audits"
	@echo "build              - Build all binaries"
	@echo "release            - Create a release (requires tag)"
	@echo "benchmark          - Run performance benchmarks"
	@echo "clean              - Clean build artifacts"

setup:
	@echo "Setting up ZERO development environment..."
	bash scripts/setup_test_data.sh
	cargo build
	cd crates/ui && pnpm install

lint:
	@echo "Running code quality checks..."
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings
	cd crates/ui && pnpm lint

test:
	@echo "Running tests..."
	bash scripts/setup_test_data.sh
	cargo test --workspace
	cd crates/ui && pnpm test

coverage:
	@echo "Generating coverage report..."
	cargo install cargo-tarpaulin
	cargo tarpaulin --workspace --out Html

security-audit:
	@echo "Running security audits..."
	cargo install cargo-audit cargo-deny
	cargo audit
	cargo deny check

build: lint test
	@echo "Building release binaries..."
	bash scripts/release-ci.sh
	bash scripts/release-tui.sh
	bash scripts/release-ui.sh

benchmark:
	@echo "Running benchmarks..."
	python3 scripts/generate_benchmark_data.py
	cargo bench --workspace

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf test_data/
	cd crates/ui && pnpm clean