.PHONY: all build test clean dev lint fmt check wasm desktop cli install help

# Default target
all: build

# Help
help:
	@echo "PMSynapse Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build       Build all Rust crates"
	@echo "  test        Run all tests"
	@echo "  lint        Run clippy lints"
	@echo "  fmt         Format code"
	@echo "  check       Run cargo check"
	@echo "  clean       Clean build artifacts"
	@echo "  dev         Run development server"
	@echo "  desktop     Build desktop app"
	@echo "  cli         Build CLI tool"
	@echo "  wasm        Build WASM package"
	@echo "  install     Install dependencies"
	@echo "  setup       Initial project setup"

# Build all Rust crates
build:
	cargo build --workspace

# Build release
build-release:
	cargo build --workspace --release

# Run all tests
test:
	cargo test --workspace --all-features

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
	rm -rf crates/snps-wasm/pkg

# Development server
dev:
	cd apps/desktop && pnpm dev

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
	cargo install --path crates/snps-cli

# Build WASM
wasm:
	cd crates/snps-wasm && wasm-pack build --target web

# Build WASM release
wasm-release:
	cd crates/snps-wasm && wasm-pack build --target web --release

# Install all dependencies
install:
	pnpm install
	rustup target add wasm32-unknown-unknown

# Initial setup
setup: install
	@echo "PMSynapse setup complete!"
	@echo "Run 'make dev' to start development"

# Run all checks (for CI)
ci: fmt-check lint test

# Documentation
docs:
	cargo doc --workspace --no-deps --open
