# Turborepo Parallel Release Process

This document describes PMSynapse's release process, which follows patterns established by the [BAML](https://github.com/BoundaryML/baml) project for parallel builds and releases using Turborepo.

## Overview

PMSynapse uses **Turborepo** to orchestrate parallel builds and releases across its hybrid monorepo structure:

- **TypeScript packages**: Apps and libraries managed by pnpm workspace
- **Rust engine**: Core engine crates managed by Cargo workspace

The key benefits of this approach:

1. **Parallel execution**: Independent tasks run concurrently
2. **Dependency-aware builds**: Tasks respect `dependsOn` relationships
3. **Remote caching**: CI builds leverage cached artifacts
4. **Unified workflow**: Single command orchestrates Rust + TypeScript

## Architecture

```
pmsynapse/
├── turbo.json              # Turborepo task configuration
├── package.json            # Root scripts orchestrating both systems
├── pnpm-workspace.yaml     # TypeScript workspace definition
├── Cargo.toml              # Rust workspace definition
├── apps/
│   ├── desktop/            # Tauri desktop app
│   └── vscode-ext/         # VS Code extension
├── packages/
│   └── rpc/                # Shared RPC types
└── engine/
    ├── snps-core/          # Core Rust library
    └── snps-cli/           # CLI tool
```

## Task Pipeline

### turbo.json Tasks

| Task | Description | Dependencies | Outputs |
|------|-------------|--------------|---------|
| `build` | Build TypeScript packages | `^build` | `dist/**` |
| `build:engine` | Build Rust engine (release) | none | `target/release/**` |
| `build:debug` | Build Rust engine (debug) | none | `target/debug/**` |
| `build:tauri` | Build Tauri app | `build:engine`, `build` | `src-tauri/target/**` |
| `lint` | Lint TypeScript | `^lint` | none |
| `lint:engine` | Lint Rust (clippy) | none | none |
| `test` | Test TypeScript | `build` | `coverage/**` |
| `test:engine` | Test Rust | `build:debug` | none |
| `release` | Release TypeScript | `^release`, `build`, `test` | none |
| `release:engine` | Release Rust | `lint:engine`, `test:engine` | `target/release/**` |
| `release:vscode` | Package VS Code ext | `^build`, `build`, `test` | `*.vsix` |
| `release:desktop` | Build desktop bundles | `build:tauri` | bundles |

### Dependency Graph

```
                    ┌─────────────────┐
                    │   build:engine  │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
        ┌───────────┐ ┌───────────┐ ┌───────────────┐
        │lint:engine│ │test:engine│ │  build:tauri  │
        └─────┬─────┘ └─────┬─────┘ └───────┬───────┘
              │             │               │
              └──────┬──────┘               │
                     ▼                      ▼
             ┌──────────────┐       ┌──────────────┐
             │release:engine│       │release:desktop│
             └──────────────┘       └──────────────┘
```

## Commands

### Development

```bash
# Build all packages (TypeScript only, parallel)
pnpm build

# Build everything including Rust engine
pnpm build:all

# Development mode (watch)
pnpm dev                  # All packages
pnpm dev:desktop          # Desktop app
pnpm dev:vscode           # VS Code extension
```

### Testing

```bash
# Test TypeScript packages
pnpm test

# Test Rust engine
pnpm test:engine

# Test everything
pnpm test:all
```

### Linting

```bash
# Lint TypeScript packages
pnpm lint

# Lint Rust engine (clippy)
pnpm lint:engine

# Lint everything
pnpm lint:all
```

### Release

```bash
# Release TypeScript packages
pnpm release

# Release Rust engine (builds release binary)
pnpm release:engine

# Release VS Code extension
pnpm release:vscode

# Release desktop app
pnpm release:desktop

# Release everything
pnpm release:all
```

### CI

```bash
# Full CI pipeline (format check + lint + test)
pnpm ci
```

## Configuration Details

### Concurrency

```json
{
  "concurrency": 10
}
```

Up to 10 tasks run in parallel, maximizing build throughput.

### Environment Variables

Environment variables are passed through to tasks:

```json
{
  "globalPassThroughEnv": [
    "NODE_ENV",
    "CI",
    "GITHUB_ACTIONS",
    "RUST_LOG",
    "RUST_BACKTRACE"
  ]
}
```

Release-specific variables:
- `GITHUB_TOKEN` - GitHub releases
- `NPM_TOKEN` - npm publishing
- `CARGO_REGISTRY_TOKEN` - crates.io publishing
- `VSCE_PAT` - VS Code marketplace
- `TAURI_SIGNING_PRIVATE_KEY` - Desktop app signing

### Caching

Most build tasks are cached:

```json
{
  "build:engine": {
    "cache": true,
    "inputs": ["../../engine/**/*.rs", "../../Cargo.toml", "../../Cargo.lock"],
    "outputs": ["../../target/release/**"]
  }
}
```

- `inputs` - Files that affect cache validity
- `outputs` - Files to cache
- Tests are not cached (`cache: false`) to ensure fresh runs

### Global Dependencies

Changes to these files invalidate all caches:

```json
{
  "globalDependencies": [
    ".env",
    ".env.local",
    "Cargo.toml",
    "Cargo.lock"
  ]
}
```

## Comparison: BAML vs PMSynapse

| Feature | BAML | PMSynapse |
|---------|------|-----------|
| Task runner | Turborepo | Turborepo |
| TypeScript manager | pnpm | pnpm |
| Rust manager | Cargo | Cargo |
| Concurrency | 40 | 10 |
| Remote caching | Yes | Optional |
| Engine integration | Via tasks | Via tasks |
| Release cascade | `^release` | `^release` |

## Best Practices

### 1. Keep Tasks Independent

Tasks without dependencies run in parallel automatically:

```json
{
  "lint": { "dependsOn": [] },        // Can run immediately
  "lint:engine": { "dependsOn": [] }  // Can run immediately
}
```

### 2. Use `^` for Cross-Package Dependencies

The `^` prefix means "run this task in dependencies first":

```json
{
  "build": { "dependsOn": ["^build"] }  // Build deps before this package
}
```

### 3. Separate Build Profiles

Use different tasks for debug vs release builds:

```json
{
  "build:debug": { ... },   // Fast, for development
  "build:engine": { ... }   // Optimized, for release
}
```

### 4. Define Clear Inputs/Outputs

Accurate inputs/outputs enable effective caching:

```json
{
  "build": {
    "inputs": ["src/**", "package.json", "tsconfig.json"],
    "outputs": ["dist/**"]
  }
}
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: pnpm/action-setup@v2
        with:
          version: 9

      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'pnpm'

      - uses: dtolnay/rust-toolchain@stable

      - uses: Swatinem/rust-cache@v2

      - run: pnpm install

      - run: pnpm ci
```

### Remote Caching (Optional)

Enable Turborepo remote caching for faster CI:

```bash
# Login to Vercel (Turborepo creator)
npx turbo login

# Link project
npx turbo link

# CI builds will now use remote cache
```

## Troubleshooting

### Cache Issues

```bash
# Clear local cache
rm -rf node_modules/.cache/turbo

# Force fresh build
turbo run build --force
```

### Task Not Running

Check dependencies in `turbo.json`. Use `--graph` to visualize:

```bash
turbo run build --graph
```

### Rust Build Failures

Ensure Cargo.toml workspace is correct:

```toml
[workspace]
members = [
    "engine/snps-core",
    "engine/snps-cli",
]
```

## References

- [Turborepo Documentation](https://turbo.build/repo/docs)
- [BAML Repository](https://github.com/BoundaryML/baml) - Reference implementation
- [pnpm Workspaces](https://pnpm.io/workspaces)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
