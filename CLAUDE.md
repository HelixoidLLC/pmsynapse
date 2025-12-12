# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**PMSynapse** is an AI-enabled end-to-end project management system built with Rust + Tauri + React. It features:
- Knowledge graph for tracking project artifacts (using SQLite/rusqlite, CozoDB planned for later)
- Multi-provider LLM integration for AI-assisted workflows
- IDLC (Idea Development Lifecycle) - configurable per-team workflow system
- Desktop app with Tauri 2.0 + React + shadcn/ui

## Repository Structure

Following BAML's pattern of isolating Rust code in an `engine/` directory:

```
pmsynapse/
├── engine/                 # All Rust crates (BAML-style organization)
│   ├── Cargo.toml          # Engine workspace config
│   ├── snps-core/          # Core Rust library (graph, LLM, IDLC)
│   │   ├── src/
│   │   └── tests/          # Integration tests
│   └── snps-cli/           # CLI tool (`snps` binary)
│       ├── src/
│       └── tests/          # CLI integration tests
├── apps/
│   ├── desktop/            # Tauri desktop app
│   │   ├── src/            # React frontend
│   │   └── src-tauri/      # Tauri backend (Rust)
│   └── vscode-ext/         # VS Code extension (planned)
├── packages/               # Shared TypeScript packages
│   └── rpc/                # Shared RPC types
├── integ-tests/            # Cross-component integration tests
│   ├── fixtures/           # Test data and configurations
│   └── README.md           # Test documentation
├── docs/                   # Architecture & planning docs
├── Cargo.toml              # Root workspace (includes engine + apps)
└── package.json            # Root monorepo config
```

## Build Commands

### Rust (Engine)
```bash
# Build all Rust crates
cargo build

# Build engine crates only
cargo build -p snps-core -p snps-cli

# Run tests with nextest
pnpm test:rust
# or directly:
cargo nextest run --all-features

# Run integration tests
cargo test --test '*'

# Lint and format
pnpm lint:rust
# or directly:
cargo fmt --check
cargo clippy --all-targets --all-features

# Run CLI
cargo run -p snps-cli -- <command>
# Example:
cargo run -p snps-cli -- init
cargo run -p snps-cli -- status
cargo run -p snps-cli -- daemon status
cargo run -p snps-cli -- thoughts list
```

### Desktop App (Tauri + React)
```bash
# Install dependencies (first time)
pnpm install

# Development mode (hot reload)
pnpm dev
# or from desktop dir:
cd apps/desktop && pnpm tauri:dev

# Build for production
pnpm build
# or from desktop dir:
cd apps/desktop && pnpm tauri:build

# Frontend only
cd apps/desktop
pnpm dev           # Vite dev server
pnpm build         # Build frontend
pnpm typecheck     # TypeScript type checking
pnpm lint          # ESLint
```

### Using snps CLI
```bash
# Start development environment (daemon + UI)
snps dev

# Start daemon only
snps daemon start --foreground

# Launch UI
snps ui

# Manage thoughts
snps thoughts init
snps thoughts new research "Topic"
snps thoughts search "query"
```

### Monorepo (Turbo)
```bash
# Build everything
pnpm build

# Run tests across workspace
pnpm test

# Lint all
pnpm lint

# Type check all
pnpm typecheck

# Clean all build artifacts
pnpm clean
```

## Architecture

### Engine (`engine/`)

All Rust code lives in the `engine/` directory following BAML's pattern:

#### Core Library (`engine/snps-core`)
- `src/lib.rs` - Main exports, error types, initialization
- `src/graph.rs` - Knowledge graph implementation (SQLite MVP, CozoDB planned)
- `src/llm.rs` - Multi-provider LLM integration (Anthropic, OpenAI, etc.)
- `src/idlc.rs` - Idea Development Lifecycle workflow engine
- `tests/` - Integration tests

**Key patterns:**
- Uses workspace dependencies defined in root `Cargo.toml`
- Error handling via `SynapseError` enum (thiserror)
- Async runtime: Tokio
- Storage: rusqlite (CozoDB temporarily disabled due to dependency conflicts)

#### CLI (`engine/snps-cli`)
- `src/main.rs` - CLI implementation with clap
- `tests/` - CLI integration tests using assert_cmd

**Commands:**
- `init`, `status` - Project management
- `daemon start|stop|status|restart|logs` - Daemon lifecycle
- `ui`, `dev` - Launch UI and development mode
- `thoughts init|new|search|list|sync` - Thoughts management
- `graph`, `analyze`, `sync` - Knowledge graph operations
- `proposals`, `templates`, `team` - Workflow management

### Desktop App (`apps/desktop/`)

**Frontend** (`src/`):
- React 18 + TypeScript
- Routing: react-router-dom
- State management: Zustand
- UI: shadcn/ui components (Radix UI + Tailwind)
- Build: Vite 6

**Backend** (`src-tauri/`):
- Tauri 2.0
- Plugins: fs, shell, notification, clipboard-manager, log, store
- IPC commands defined in `src/lib.rs`

### IDLC (Idea Development Lifecycle)

PMSynapse's core concept: configurable per-team workflows. See `docs/IDLC_CONFIGURATION.md` for details.
- Teams define custom stages/statuses in `.pmsynapse/teams/<team-id>/idlc.yaml`
- Default stages: triage → backlog → research → planning → development → validation → delivery → completed
- Integrates with Linear, GitHub, and knowledge graph

### Knowledge Graph

Tracks relationships between:
- Issues, Tasks, Features
- Research, Questions, Findings
- Plans, Implementations
- Code, Documents, Proposals

Currently uses SQLite (rusqlite). CozoDB (graph + vector DB) planned once dependency issues resolved.

## Testing Structure

Following BAML's test organization pattern:

```
# Unit tests - inline with source code
engine/snps-core/src/graph.rs
  └── #[cfg(test)] mod tests { ... }

# Integration tests - separate directory
engine/snps-core/tests/
  ├── graph_tests.rs
  └── idlc_tests.rs

# CLI integration tests
engine/snps-cli/tests/
  └── cli_tests.rs

# Cross-component tests
integ-tests/
  ├── fixtures/
  └── README.md
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p snps-core
cargo test -p snps-cli

# Integration tests only
cargo test --test '*'

# With nextest (faster)
cargo nextest run

# Snapshot tests
cargo insta review
```

## Development Workflow

### Thoughts System

PMSynapse includes a thoughts management system for research and planning:

```bash
# Initialize thoughts for project
snps thoughts init

# Create documents
snps thoughts new research "Topic Name"
snps thoughts new plan "Feature Plan"
snps thoughts new ticket "PROJ-123"

# Search and list
snps thoughts search "query" --paths-only
snps thoughts list --recent 10
```

See `docs/THOUGHTS_SYSTEM.md` and `docs/THOUGHTS_WORKFLOW_TUTORIAL.md` for details.

### Linear Integration

This project uses Linear for issue tracking with MCP integration:
- Linear MCP server can be configured in Claude Code settings
- Status mapping defined in `docs/PROJECT_WORKFLOW.md`
- CLI commands: `snps linear sync`, `snps linear push`, `snps linear pull`

### Logging

- Rust: Set `RUST_LOG=debug` for verbose output
- CLI: Use `--verbose` or `-v` flag

## Important Notes

### Engine Directory (BAML Pattern)

The `engine/` directory contains all Rust crates, following BAML's organizational pattern:
- Isolates Rust code from TypeScript/frontend code
- Has its own `Cargo.toml` workspace config
- Root `Cargo.toml` includes both engine crates and apps

### WASM Support (Removed)

WASM support was removed to unblock Rust dependency issues. If needed in future, it can be re-added in `engine/snps-wasm/`.

### Database Migration

Currently using rusqlite for MVP. CozoDB (graph + vector database) is planned but temporarily disabled due to dependency conflicts. When ready:
1. Re-enable `cozo` in workspace dependencies
2. Update `engine/snps-core/Cargo.toml`
3. Implement graph storage in `engine/snps-core/src/graph.rs`

### Cross-Platform Builds

CI/CD configured for:
- Linux (ubuntu-latest)
- macOS (macos-latest)
- Windows (windows-latest)

Desktop app requires platform-specific dependencies (WebKit on Linux).

## Configuration Files

- `Cargo.toml` - Root workspace config
- `engine/Cargo.toml` - Engine workspace config
- `rust-toolchain.toml` - Rust version pinning (edition 2021)
- `turbo.json` - Turborepo task orchestration
- `pnpm-workspace.yaml` - pnpm workspace config
- `.pmsynapse/config.yaml` - PMSynapse project config (created by `snps init`)
- `.pmsynapse/teams/*/idlc.yaml` - Per-team workflow config

## Claude Code Behavior Rules

### Web Fetching Fallback

When the `WebFetch` tool fails (404, timeout, or other errors), fallback to using `curl` via the Bash tool:

```bash
# Example fallback pattern
curl -sL "https://raw.githubusercontent.com/owner/repo/branch/file.txt"

# For JSON APIs
curl -sL "https://api.github.com/repos/owner/repo/contents" | jq '.'

# With headers (e.g., for GitHub API)
curl -sL -H "Accept: application/vnd.github.v3+json" "https://api.github.com/..."
```

**When to use curl fallback:**
- WebFetch returns 404 or other HTTP errors
- WebFetch times out
- Need to access raw file content from GitHub
- Need more control over HTTP headers

### Pre-commit Testing

All code changes must pass the pre-push hook before committing:

```bash
# Run manually before committing
make check-test

# This runs:
# 1. cargo fmt --all -- --check
# 2. cargo clippy -p snps-core -p snps-cli --all-targets -- -D warnings
# 3. cargo test -p snps-core -p snps-cli --all-features
```

## Documentation

Key docs in `docs/`:
- `STARTUP_GUIDE.md` - How to run daemon and UI
- `THOUGHTS_SYSTEM.md` - Thoughts management documentation
- `THOUGHTS_WORKFLOW_TUTORIAL.md` - When and how to use thoughts
- `IDLC_CONFIGURATION.md` - Workflow configuration system
- `PROJECT_WORKFLOW.md` - Default workflow + Linear integration
- `PMSYNAPSE_CORE_FEATURES.md` - Feature overview
- `12_FACTOR_AGENTS_DESIGN.md` - Agent architecture patterns
- `AI_TEAM_COORDINATION_PATTERNS.md` - Multi-agent patterns
