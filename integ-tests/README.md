# PMSynapse Integration Tests

Cross-component integration tests for the PMSynapse project.

Following BAML's test organization pattern where integration tests live
separately from unit tests and can span multiple components.

## Directory Structure

```
integ-tests/
├── README.md              # This file
├── fixtures/              # Shared test fixtures and data
│   ├── projects/          # Sample project configurations
│   ├── workflows/         # Sample IDLC configurations
│   └── graphs/            # Sample knowledge graph data
├── rust/                  # Rust integration tests
│   ├── Cargo.toml
│   └── src/
├── typescript/            # TypeScript tests (future)
└── python/                # Python tests (future)
```

## Test Categories

### Component Integration Tests

Located in `engine/<crate>/tests/`:
- `snps-core/tests/` - Core library integration tests
- `snps-cli/tests/` - CLI integration tests

### Cross-Component Tests

Located in `integ-tests/`:
- Full workflow tests (CLI → Core → Graph)
- UI ↔ Daemon communication tests
- End-to-end feature tests

## Running Tests

### All Tests

```bash
# From project root
pnpm test:rust
# or
cargo nextest run --all-features
```

### Engine Tests Only

```bash
cd engine
cargo test
```

### Specific Crate Tests

```bash
cargo test -p snps-core
cargo test -p snps-cli
```

### Integration Tests Only

```bash
cargo test --test '*'
```

## Writing Tests

### Unit Tests

Place in the same file as the code being tested using `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }
}
```

### Integration Tests

Place in `<crate>/tests/` directory:

```rust
// engine/snps-core/tests/graph_tests.rs
use snps_core::graph::KnowledgeGraph;

#[test]
fn test_graph_operations() {
    // ...
}
```

### CLI Tests

Use `assert_cmd` for CLI testing:

```rust
// engine/snps-cli/tests/cli_tests.rs
use assert_cmd::Command;

#[test]
fn test_cli_command() {
    Command::cargo_bin("snps")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}
```

## Snapshot Testing

We use `insta` for snapshot testing:

```rust
use insta::assert_yaml_snapshot;

#[test]
fn test_config_snapshot() {
    let config = IdlcConfig::default_config();
    assert_yaml_snapshot!(config);
}
```

Update snapshots with:

```bash
cargo insta review
```

## Test Fixtures

Shared test data lives in `integ-tests/fixtures/`:

- `projects/` - Sample `.pmsynapse/` configurations
- `workflows/` - Sample IDLC YAML files
- `graphs/` - Pre-populated graph databases for testing

## CI/CD

Tests run automatically on:
- Every pull request
- Every push to main branch
- Scheduled nightly runs for extended tests

See `.github/workflows/rust.yml` for CI configuration.
