.PHONY: all build test clean dev lint fmt check desktop cli install help engine
.PHONY: check-test lint-engine githooks setup-ci check-local

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

# Run clippy on engine crates only (no GTK dependencies)
lint-engine:
	cargo clippy -p snps-core -p snps-cli --all-targets --all-features -- -D warnings

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

# Build desktop app (requires GTK on Linux)
desktop:
	@if pkg-config --exists gtk+-3.0 2>/dev/null; then \
		cd apps/desktop && pnpm tauri build; \
	else \
		echo "⚠️  GTK not found. Skipping desktop build."; \
		echo "   Install GTK: sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev"; \
	fi

# Build desktop app for development (requires GTK on Linux)
desktop-dev:
	@if pkg-config --exists gtk+-3.0 2>/dev/null; then \
		cd apps/desktop && pnpm tauri dev; \
	else \
		echo "⚠️  GTK not found. Skipping desktop dev."; \
		echo "   Install GTK: sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev"; \
	fi

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
setup: install githooks
	@echo ""
	@echo "✅ PMSynapse setup complete!"
	@echo ""
	@echo "Quick start:"
	@echo "  make engine      - Build Rust engine"
	@echo "  make test-engine - Run tests"
	@echo "  make dev         - Start full development environment"
	@echo "  make daemon      - Start daemon in foreground"
	@echo "  make desktop-dev - Start desktop app in dev mode (requires GTK)"

# Run all checks and tests (engine-safe, for pre-push)
check-test: fmt-check lint-engine test-engine
	@echo "✅ All checks and tests passed!"

# Run all checks (for CI - engine only to avoid GTK issues)
ci: fmt-check lint-engine test-engine
	@echo "✅ CI checks passed!"

# CI setup
setup-ci:
	@echo "Setting up CI environment..."
	rustup component add clippy rustfmt

# Install git hooks
githooks:
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@cp scripts/pre-push .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push
	@echo "✅ Git hooks installed!"
	@echo "   Pre-push hook will run: make check-test"

# Check for local branches (block pushing)
check-local:
	@if git rev-parse --abbrev-ref HEAD | grep -q "^local/"; then \
		echo "❌ Cannot push local/* branches"; \
		exit 1; \
	fi

# Documentation
docs:
	cargo doc --workspace --no-deps --open

# Snapshot testing
snapshots:
	cargo insta review

# Accept all pending snapshots
snapshots-accept:
	cargo insta accept
