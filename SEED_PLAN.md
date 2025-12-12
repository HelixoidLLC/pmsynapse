# PMSynapse Repository Seed Plan

## Overview

This plan outlines how to seed the PMSynapse repository with artifacts from:
- **HumanLayer** - UI architecture, React + Tauri setup, CI/CD patterns
- **BAML** - Rust workspace patterns, multi-platform builds, Turborepo configuration

---

## Target Directory Structure

```
pmsynapse/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                    # Main CI pipeline
│   │   ├── release.yml               # Release orchestration
│   │   ├── rust-tests.yml            # Rust testing
│   │   └── build-release.yml         # Multi-platform builds
│   └── actions/
│       └── setup-rust/
│           └── action.yml            # Reusable Rust setup
├── apps/
│   └── desktop/                      # Tauri + React desktop app
│       ├── src/
│       │   ├── components/
│       │   │   └── ui/               # shadcn/ui components
│       │   ├── hooks/                # Custom React hooks
│       │   ├── stores/               # Zustand state management
│       │   ├── lib/                  # Utilities
│       │   ├── pages/                # Page components
│       │   ├── styles/               # CSS/Tailwind
│       │   ├── main.tsx              # Entry point
│       │   └── router.tsx            # Routes
│       ├── src-tauri/
│       │   ├── src/
│       │   │   ├── main.rs           # Tauri main
│       │   │   └── lib.rs            # Tauri commands
│       │   ├── Cargo.toml            # Tauri dependencies
│       │   ├── tauri.conf.json       # Tauri config
│       │   └── build.rs
│       ├── package.json
│       ├── vite.config.ts
│       ├── tsconfig.json
│       ├── tailwind.config.js
│       ├── postcss.config.js
│       ├── components.json           # shadcn/ui config
│       └── index.html
├── crates/
│   ├── snps-core/                    # Core Rust library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── graph/                # Knowledge graph
│   │       ├── llm/                  # LLM integration
│   │       └── idlc/                 # IDLC workflow
│   ├── snps-cli/                     # CLI binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── snps-wasm/                    # WASM bindings (future)
│       └── Cargo.toml
├── packages/
│   └── types/                        # Shared TypeScript types
│       ├── package.json
│       └── src/
│           └── index.ts
├── docs/                             # Documentation (existing)
├── scripts/                          # Utility scripts
│   └── setup.sh
├── Cargo.toml                        # Workspace root
├── Cargo.lock
├── rust-toolchain.toml               # Rust version
├── turbo.json                        # Turborepo config
├── package.json                      # Root package
├── pnpm-workspace.yaml               # pnpm workspaces
├── tsconfig.json                     # Root TS config
├── biome.json                        # Linting/formatting
├── Makefile                          # Dev commands
├── .gitignore
├── LICENSE
└── README.md
```

---

## Phase 1: Core Configuration Files

### From BAML (Rust Patterns)

| Source | Destination | Purpose |
|--------|-------------|---------|
| `rust-toolchain.toml` pattern | `/rust-toolchain.toml` | Pin Rust version, add targets |
| `turbo.json` pattern | `/turbo.json` | Task orchestration |
| Workspace `Cargo.toml` | `/Cargo.toml` | Rust workspace setup |
| CI workflow patterns | `/.github/workflows/` | Multi-platform builds |

### From HumanLayer (UI Patterns)

| Source | Destination | Purpose |
|--------|-------------|---------|
| `vite.config.ts` | `/apps/desktop/vite.config.ts` | Build configuration |
| `tsconfig.json` | `/apps/desktop/tsconfig.json` | TypeScript config |
| `components.json` | `/apps/desktop/components.json` | shadcn/ui setup |
| `eslint.config.mjs` | `/apps/desktop/eslint.config.mjs` | Linting |
| `.prettierrc.js` | `/.prettierrc.js` | Formatting |
| `tauri.conf.json` | `/apps/desktop/src-tauri/tauri.conf.json` | Tauri config |
| Makefile patterns | `/Makefile` | Dev commands |

---

## Phase 2: UI Components & Hooks

### Copy from HumanLayer WUI

| Source | Destination | Priority |
|--------|-------------|----------|
| `src/components/ui/*` | `/apps/desktop/src/components/ui/` | ⭐⭐⭐ |
| `src/hooks/useLocalStorage.ts` | `/apps/desktop/src/hooks/` | ⭐⭐⭐ |
| `src/hooks/useDebounce.ts` | `/apps/desktop/src/hooks/` | ⭐⭐ |
| `src/stores/appStore.ts` pattern | `/apps/desktop/src/stores/` | ⭐⭐⭐ |
| `src/lib/utils.ts` | `/apps/desktop/src/lib/` | ⭐⭐⭐ |
| `src/styles/App.css` | `/apps/desktop/src/styles/` | ⭐⭐ |

---

## Phase 3: Rust Workspace Setup

### Core Crates

```toml
# /Cargo.toml
[workspace]
resolver = "2"
members = [
  "crates/snps-core",
  "crates/snps-cli",
  "crates/snps-wasm",
  "apps/desktop/src-tauri",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "Apache-2.0"
repository = "https://github.com/HelixoidLLC/pmsynapse"

[workspace.dependencies]
# Core
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"

# Graph & Vector (CozoDB)
cozo = { version = "0.7", features = ["storage-rocksdb"] }

# LLM
reqwest = { version = "0.12", features = ["json"] }
async-trait = "0.1"

# CLI
clap = { version = "4", features = ["derive"] }

# Tauri
tauri = "2"
tauri-build = "2"

# WASM (conditional)
wasm-bindgen = "0.2"
```

---

## Phase 4: CI/CD Pipelines

### Workflow Files to Create

1. **ci.yml** - Main CI (lint, test, typecheck)
2. **rust-tests.yml** - Cargo tests, clippy, fmt
3. **release.yml** - Multi-platform release builds
4. **build-desktop.yml** - Tauri builds for macOS/Linux/Windows

### Matrix Strategy (from BAML)

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
    - os: ubuntu-latest
      target: aarch64-unknown-linux-gnu
    - os: macos-latest
      target: x86_64-apple-darwin
    - os: macos-latest
      target: aarch64-apple-darwin
    - os: windows-latest
      target: x86_64-pc-windows-msvc
```

---

## Phase 5: Implementation Tasks

### 5.1 Landing Page UI

Create a simple landing page with:
- PMSynapse branding
- Navigation sidebar placeholder
- Main content area
- Status bar

### 5.2 Tauri Client

- Configure Tauri with plugins (fs, clipboard, shell, notifications)
- Create basic Rust commands for IPC
- Window configuration (size, title, decorations)

### 5.3 Rust Backend Service

Core library features:
- Basic CozoDB initialization
- Simple graph operations
- CLI with `init`, `status` commands

---

## Execution Order

1. ✅ Clone reference repositories
2. 🔄 Create directory structure
3. 🔄 Copy/adapt configuration files
4. 🔄 Set up Rust workspace
5. 🔄 Set up pnpm workspace
6. 🔄 Copy UI components
7. 🔄 Implement landing page
8. 🔄 Configure Tauri
9. 🔄 Create CI/CD workflows
10. 🔄 Run validation tests

---

## Commands to Execute

```bash
# Create directories
mkdir -p apps/desktop/{src/{components/ui,hooks,stores,lib,pages,styles},src-tauri/{src,icons}}
mkdir -p crates/{snps-core/src,snps-cli/src,snps-wasm/src}
mkdir -p packages/types/src
mkdir -p .github/{workflows,actions/setup-rust}
mkdir -p scripts

# Install dependencies
pnpm init
pnpm add -D turbo typescript @types/node

# Rust setup
rustup target add wasm32-unknown-unknown
cargo init --lib crates/snps-core
cargo init crates/snps-cli

# Tauri setup
cd apps/desktop && pnpm create tauri-app --yes
```

---

*Plan version: 1.0*
*Created: December 2025*
