# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**PMSynapse** - AI-enabled project management system with:
- Knowledge graph (SQLite/rusqlite, CozoDB planned)
- Multi-provider LLM integration
- IDLC (Idea Development Lifecycle) - configurable per-team workflows
- Desktop app (Tauri 2.0 + React + shadcn/ui)

## Repository Structure

BAML-style organization with Rust isolated in `engine/`:

```
pmsynapse/
├── engine/              # All Rust crates
│   ├── snps-core/       # Core library (graph, LLM, IDLC)
│   └── snps-cli/        # CLI tool
├── apps/
│   ├── desktop/         # Tauri app (React + Rust)
│   └── vscode-ext/      # VS Code extension (planned)
├── packages/rpc/        # Shared TypeScript types
├── integ-tests/         # Cross-component tests
└── docs/                # Architecture & planning
```

## Build Commands

### Quick Actions
- `pnpm dev` - Start Tauri app in development mode
- `pnpm build` - Build everything (monorepo)
- `pnpm test` - Run all tests
- `pnpm lint` - Lint all code
- `make check-test` - Run pre-push checks

### Rust Development
```bash
cargo build -p snps-core -p snps-cli    # Build engine crates
cargo nextest run --all-features        # Run tests
cargo fmt --check && cargo clippy       # Lint
cargo run -p snps-cli -- <command>      # Run CLI
```

### snps CLI
```bash
snps dev                     # Start daemon + UI
snps daemon start            # Daemon only
snps ui                      # Launch UI
snps thoughts <command>      # Manage thoughts
snps claude <command>        # Claude session management
```

### Claude Session Commands
```bash
snps claude list             # List Claude Code sessions
snps claude parse <id>       # Parse session (supports partial IDs)
snps claude convert <json>   # Convert JSON to HTML/Markdown
snps claude analyze <dir>    # Analyze session hierarchy
snps claude import           # Batch import sessions to thoughts

# Export formats: json, markdown, html
snps claude parse <id> --format html -o session.html
snps claude convert session.json --format html -o output.html
```

See `docs/CLAUDE_SESSION_PARSER.md` for detailed documentation.

### Desktop App
```bash
cd apps/desktop
pnpm tauri:dev               # Development mode
pnpm tauri:build             # Production build
```

## Architecture

### Core Components
- **snps-core** - Knowledge graph (SQLite), LLM integration, IDLC engine
- **snps-cli** - CLI tool with daemon, thoughts, and workflow management
- **desktop** - Tauri 2.0 + React 18 + shadcn/ui
- **rpc** - Shared TypeScript RPC types

### Tech Stack
- **Rust**: Tokio async, thiserror, rusqlite
- **Frontend**: React 18, Zustand, Radix UI, Tailwind, Vite 6
- **Desktop**: Tauri 2.0 plugins (fs, shell, notification, log, store)

### IDLC (Idea Development Lifecycle)
Configurable per-team workflows defined in `.pmsynapse/teams/<team-id>/idlc.yaml`:
- Default: triage → backlog → research → planning → development → validation → delivery → completed
- Integrates with Linear, GitHub, knowledge graph
- See `docs/IDLC_CONFIGURATION.md`

### Knowledge Graph
Tracks relationships: Issues/Tasks → Research/Findings → Plans/Implementations → Code/Docs
- Current: SQLite (rusqlite)
- Planned: CozoDB (graph + vector DB)

## Testing

BAML-style organization:
- Unit tests: Inline with source (`#[cfg(test)]`)
- Integration: `engine/*/tests/`
- Cross-component: `integ-tests/`

```bash
cargo test                    # All tests
cargo test -p snps-core       # Specific crate
cargo nextest run             # Faster parallel execution
cargo insta review            # Snapshot tests
```

## Development Workflow

### Thoughts System
```bash
snps thoughts init                  # Initialize
snps thoughts new research "Topic"  # Create document
snps thoughts search "query"        # Search
snps thoughts list --recent 10      # List recent
```
See `docs/THOUGHTS_SYSTEM.md` and `docs/THOUGHTS_WORKFLOW_TUTORIAL.md`

### Linear Integration
- MCP server configurable in Claude Code settings
- Status mapping in `docs/PROJECT_WORKFLOW.md`
- Fetch tickets: `linear get-issue ENG-XXXX > thoughts/shared/tickets/eng-XXXX.md`

### Logging
- Rust: `RUST_LOG=debug`
- CLI: `--verbose` or `-v` flag

## Important Notes

### BAML Pattern
- Rust code isolated in `engine/` directory
- Separate `engine/Cargo.toml` workspace
- Root `Cargo.toml` includes engine + apps

### Database
- Current: rusqlite (MVP)
- Planned: CozoDB (disabled due to dependency conflicts)

### Cross-Platform
- CI/CD: Linux, macOS, Windows
- Desktop requires WebKit on Linux

## Configuration Files

- `Cargo.toml` - Root workspace
- `engine/Cargo.toml` - Engine workspace
- `rust-toolchain.toml` - Rust edition 2021
- `turbo.json` - Turborepo orchestration
- `.pmsynapse/config.yaml` - Project config
- `.pmsynapse/teams/*/idlc.yaml` - Team workflows

## Claude Code Behavior

### WebFetch Fallback
When WebFetch fails, use `curl`:
```bash
curl -sL "https://raw.githubusercontent.com/owner/repo/branch/file.txt"
curl -sL -H "Accept: application/vnd.github.v3+json" "https://api.github.com/..."
```

### Pre-push Testing
Run `make check-test` before committing:
- `cargo fmt --all -- --check`
- `cargo clippy -p snps-core -p snps-cli --all-targets -- -D warnings`
- `cargo test -p snps-core -p snps-cli --all-features`

## Documentation

Key docs in `docs/`:
- `STARTUP_GUIDE.md` - Daemon and UI setup
- `THOUGHTS_SYSTEM.md` + `THOUGHTS_WORKFLOW_TUTORIAL.md` - Thoughts management
- `IDLC_CONFIGURATION.md` - Workflow configuration
- `PROJECT_WORKFLOW.md` - Default workflow + Linear
- `PMSYNAPSE_CORE_FEATURES.md` - Feature overview
- `12_FACTOR_AGENTS_DESIGN.md` - Agent architecture
- `AI_TEAM_COORDINATION_PATTERNS.md` - Multi-agent patterns
