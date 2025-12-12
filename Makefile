.PHONY: all build test clean dev lint fmt check desktop cli install help engine

# Default target
all: build

# Help
help:
	@echo "PMSynapse Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Build Targets:"
	@echo "  build        Build all Rust crates"
	@echo "  build-release Build all crates in release mode"
	@echo "  engine       Build engine crates only"
	@echo "  cli          Build CLI tool"
	@echo "  desktop      Build desktop app"
	@echo ""
	@echo "Test Targets:"
	@echo "  test         Run all tests"
	@echo "  test-engine  Run engine tests only"
	@echo "  test-integ   Run integration tests only"
	@echo ""
	@echo "Development Targets:"
	@echo "  dev          Run development server (via snps CLI)"
	@echo "  desktop-dev  Run desktop app in dev mode"
	@echo "  daemon       Start daemon in foreground"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint         Run clippy lints"
	@echo "  fmt          Format code"
	@echo "  fmt-check    Check formatting"
	@echo "  check        Run cargo check"
	@echo ""
	@echo "Other:"
	@echo "  clean        Clean build artifacts"
	@echo "  install      Install dependencies"
	@echo "  setup        Initial project setup"
	@echo "  docs         Generate documentation"

# Build all Rust crates
build:
	cargo build --workspace

# Build release
build-release:
	cargo build --workspace --release

# Build engine crates only
engine:
	cargo build -p snps-core -p snps-cli

# Build engine crates in release mode
engine-release:
	cargo build -p snps-core -p snps-cli --release

# Run all tests
test:
	cargo test --workspace --all-features

# Run engine tests only
test-engine:
	cargo test -p snps-core -p snps-cli --all-features

# Run integration tests only
test-integ:
	cargo test --test '*' --all-features

# Run clippy lints
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Cargo check
check:
	cargo check --workspace --all-features

# Clean build artifacts
clean:
	cargo clean
	rm -rf apps/desktop/dist
	rm -rf apps/desktop/src-tauri/target

# Development server (using snps CLI)
dev:
	cargo run -p snps-cli -- dev

# Start daemon in foreground
daemon:
	cargo run -p snps-cli -- daemon start --foreground

# Build desktop app
desktop:
	cd apps/desktop && pnpm tauri build

# Build desktop app for development
desktop-dev:
	cd apps/desktop && pnpm tauri dev

# Build CLI
cli:
	cargo build -p snps-cli --release

# Install CLI locally
cli-install:
	cargo install --path engine/snps-cli

# Install all dependencies
install:
	pnpm install

# Initial setup
setup: install
	@echo "PMSynapse setup complete!"
	@echo ""
	@echo "Quick start:"
	@echo "  make dev         - Start full development environment"
	@echo "  make daemon      - Start daemon in foreground"
	@echo "  make desktop-dev - Start desktop app in dev mode"
	@echo "  make test        - Run all tests"

# Run all checks (for CI)
ci: fmt-check lint test

# Documentation
docs:
	cargo doc --workspace --no-deps --open

# Snapshot testing
snapshots:
	cargo insta review

# Accept all pending snapshots
snapshots-accept:
	cargo insta accept
