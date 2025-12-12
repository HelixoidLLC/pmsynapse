# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**PMSynapse** is an AI-enabled end-to-end project management system built with Rust + Tauri + React. It features:
- Knowledge graph for tracking project artifacts (using SQLite/rusqlite, CozoDB planned for later)
- Multi-provider LLM integration for AI-assisted workflows
- IDLC (Idea Development Lifecycle) - configurable per-team workflow system
- Desktop app with Tauri 2.0 + React + shadcn/ui

## Repository Structure

```
pmsynapse/
├── crates/
│   ├── snps-core/      # Core Rust library (graph, LLM, IDLC)
│   └── snps-cli/       # CLI tool (`snps` binary)
├── apps/
│   └── desktop/        # Tauri desktop app
│       ├── src/        # React frontend
│       └── src-tauri/  # Tauri backend (Rust)
└── docs/               # Architecture & planning docs
```

## Build Commands

### Rust (Core + CLI)
```bash
# Build all Rust crates
cargo build

# Run tests with nextest
pnpm test:rust
# or directly:
cargo nextest run --all-features

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

### Core Library (`snps-core`)
Located in `crates/snps-core/src/`:
- `lib.rs` - Main exports, error types, initialization
- `graph/` - Knowledge graph implementation (SQLite MVP, CozoDB planned)
- `llm/` - Multi-provider LLM integration (Anthropic, OpenAI, etc.)
- `idlc/` - Idea Development Lifecycle workflow engine

**Key patterns:**
- Uses workspace dependencies defined in root `Cargo.toml`
- Error handling via `SynapseError` enum (thiserror)
- Async runtime: Tokio
- Storage: rusqlite (CozoDB temporarily disabled due to dependency conflicts)

### CLI (`snps-cli`)
Located in `crates/snps-cli/src/main.rs`:
- Built with clap (derive API)
- Subcommands: `init`, `status`, `sync`, `analyze`, `proposals`, `templates`, `team`, `graph`
- Uses colored for terminal output
- Tracing for logging (controlled by `RUST_LOG` env var)

### Desktop App
**Frontend** (`apps/desktop/src/`):
- React 18 + TypeScript
- Routing: react-router-dom
- State management: Zustand
- UI: shadcn/ui components (Radix UI + Tailwind)
- Build: Vite 6

**Backend** (`apps/desktop/src-tauri/`):
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

## Development Workflow

### Linear Integration
This project uses Linear for issue tracking with MCP integration:
- Linear MCP server can be configured in Claude Code settings
- Status mapping defined in `docs/PROJECT_WORKFLOW.md`
- CLI commands: `snps linear sync`, `snps linear push`, `snps linear pull`

### Testing
- Rust tests: `cargo test` or `pnpm test:rust` (uses nextest)
- Snapshot tests: Uses `insta` crate for snapshot testing
- Frontend tests: Vitest (`pnpm test` in desktop dir)

### Logging
- Rust: Set `RUST_LOG=debug` for verbose output
- CLI: Use `--verbose` or `-v` flag

## Important Notes

### WASM Support (Removed)
WASM support was removed to unblock Rust dependency issues (see commit 97a2b91). If needed in future, it can be re-added in `crates/snps-wasm/`.

### Database Migration
Currently using rusqlite for MVP. CozoDB (graph + vector database) is planned but temporarily disabled due to dependency conflicts in workspace. When ready:
1. Re-enable `cozo` in workspace dependencies
2. Update `snps-core/Cargo.toml`
3. Implement graph storage in `crates/snps-core/src/graph/`

### Module Naming Convention
Recent refactor (commit 0990fc9) moved to modern Rust naming:
- Use `mod.rs` for module definitions
- Separate `graph.rs` → `graph/mod.rs`
- Separate `llm.rs` → `llm/mod.rs`
- Separate `idlc.rs` → `idlc/mod.rs`

### Cross-Platform Builds
CI/CD configured for:
- Linux (ubuntu-latest)
- macOS (macos-latest)
- Windows (windows-latest)

Desktop app requires platform-specific dependencies (WebKit on Linux).

## Configuration Files

- `rust-toolchain.toml` - Rust version pinning (edition 2021)
- `turbo.json` - Turborepo task orchestration
- `pnpm-workspace.yaml` - pnpm workspace config
- `.pmsynapse/config.yaml` - PMSynapse project config (created by `snps init`)
- `.pmsynapse/teams/*/idlc.yaml` - Per-team workflow config

## Documentation

Key docs in `docs/`:
- `IDLC_CONFIGURATION.md` - Workflow configuration system
- `PROJECT_WORKFLOW.md` - Default workflow + Linear integration
- `PMSYNAPSE_CORE_FEATURES.md` - Feature overview
- `12_FACTOR_AGENTS_DESIGN.md` - Agent architecture patterns
- `AI_TEAM_COORDINATION_PATTERNS.md` - Multi-agent patterns
- `SEED_PLAN.md` - Original repository seeding plan
