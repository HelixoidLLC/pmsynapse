---
date: 2025-12-17
status: completed
type: implementation
related_research: thoughts/shared/research/2025-12-17-rstest-bdd-nightly-issue.md
---

# Cucumber BDD Migration - Complete

## Summary

Successfully migrated all BDD tests from manual Given/When/Then comments to proper cucumber-rs framework with Gherkin feature files and step definitions.

## Problem Solved

- rstest-bdd v0.2.0 requires nightly Rust despite documentation claiming otherwise
- Needed BDD testing framework that works on stable Rust 1.87.0

## Solution

Switched to **cucumber-rs v0.21** which:
- ✅ Works perfectly on stable Rust
- ✅ Provides full Gherkin syntax support
- ✅ More mature and widely adopted
- ✅ Better documentation and ecosystem

## Migration Results

### snps-core: 5 features, 15 scenarios, 60 steps

**Feature files created:**
1. `features/matter_parsing.feature` - 3 scenarios
   - Parse valid frontmatter
   - Reject file without frontmatter delimiter
   - Reject file with unclosed frontmatter

2. `features/matter_paths.feature` - 3 scenarios
   - Generate path for spec in shared directory
   - Generate path sanitizes title with special characters
   - Slugify title correctly

3. `features/indexing.feature` - 3 scenarios
   - Create index with database
   - Index matter file
   - Search indexed files

4. `features/config.feature` - 3 scenarios
   - Default config has expected values
   - Config serialization round trip
   - Merged config tracks sources

5. `features/repository.feature` - 3 scenarios
   - Default sync config has expected values
   - Repository context type serialization
   - Repository visibility serialization

**Step definitions:** `tests/cucumber.rs` (650+ lines)

### snps-cli: 3 features, 14 scenarios, 55 steps

**Feature files created:**
1. `features/cli_basic.feature` - 5 scenarios
   - CLI help command
   - CLI version command
   - Matter subcommand exists
   - Repo subcommand exists
   - Config show command help

2. `features/matter_commands.feature` - 4 scenarios
   - Matter list command help
   - Matter search command help
   - Matter create requires arguments
   - Matter create command with full arguments

3. `features/repo_commands.feature` - 5 scenarios
   - Repo init command help
   - Repo list command help
   - Repo add command help
   - Repo index command help
   - Repo init requires arguments

**Step definitions:** `tests/cucumber.rs` (140+ lines)

## Test Results

**All 115 steps passing:**
- ✅ 60 steps in snps-core
- ✅ 55 steps in snps-cli
- ✅ All other tests continue to pass (46 unit + 15 IDLC + 12 CLI + 9 graph + 2 cozo)

## Files Changed

**Added:**
- `engine/snps-core/features/*.feature` (5 files)
- `engine/snps-core/tests/cucumber.rs`
- `engine/snps-cli/features/*.feature` (3 files)
- `engine/snps-cli/tests/cucumber.rs`

**Modified:**
- `engine/snps-core/Cargo.toml` - Added cucumber dependency and test harness config
- `engine/snps-cli/Cargo.toml` - Added cucumber dependency and test harness config

**Removed:**
- `engine/snps-core/tests/bdd_tests.rs` (old BDD-style tests)
- `engine/snps-cli/tests/cli_bdd_tests.rs` (old BDD-style tests)

## Configuration

Both `Cargo.toml` files configured with:
```toml
[dev-dependencies]
cucumber = "0.21"

[[test]]
name = "cucumber"
harness = false
```

## Pre-commit Checks

✅ All checks pass:
- `cargo fmt --all -- --check`
- `cargo clippy -p snps-core -p snps-cli --all-targets -- -D warnings`
- `cargo test -p snps-core -p snps-cli --all-features`

## Benefits

1. **Proper Gherkin Syntax**: Feature files are now readable business specifications
2. **Reusable Steps**: Step definitions can be shared across scenarios
3. **Better Documentation**: Features serve as living documentation
4. **IDE Support**: Better tooling support for Gherkin files
5. **Standard Format**: Follows industry-standard BDD practices
6. **Stable Rust**: No nightly dependencies required

## Running Tests

```bash
# Run all cucumber tests
cargo test --test cucumber -p snps-core
cargo test --test cucumber -p snps-cli

# Run all tests including cucumber
cargo test -p snps-core -p snps-cli

# Pre-push checks
make check-test
```

## Next Steps

- Consider adding more scenarios for edge cases
- Could add data tables for parameterized tests
- Potential for scenario outlines for test variations
