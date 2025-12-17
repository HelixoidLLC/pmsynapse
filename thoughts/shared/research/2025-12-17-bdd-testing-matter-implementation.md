---
date: 2025-12-17T14:26:37Z
researcher: igor
git_commit: 9ece9f9010d83b1170b30cc4e6b4be0a566e3756
branch: matter-sync
repository: pmsynapse
topic: "Adding BDD Test Coverage to Matter Protocol Implementation"
tags: [research, bdd, testing, rstest-bdd, matter, snps-core, snps-cli, integration-tests]
status: complete
last_updated: 2025-12-17
last_updated_by: igor
last_updated_note: "Added CLI-based full-flow integration tests"
---

# Research: Adding BDD Test Coverage to Matter Protocol Implementation

**Date**: 2025-12-17T14:26:37Z
**Researcher**: igor
**Git Commit**: 9ece9f9
**Branch**: matter-sync
**Repository**: pmsynapse

## Research Question

How to properly add BDD test coverage using rstest-bdd framework to document behaviors in the Matter protocol implementation committed in 9ece9f9.

## Summary

This research documents how to integrate rstest-bdd (v0.2.0) for BDD-style testing of the Matter protocol implementation at two levels:

1. **Unit-level BDD (snps-core)**: Tests individual module behaviors (parsing, indexing, config management) with ~70+ scenarios
2. **CLI-level BDD (snps-cli)**: Tests full user workflows via CLI commands (create, search, edit, delete, repo management)

The Matter implementation includes 5 new modules (matter, parser, config, index, repository) and 7 CLI command groups. rstest-bdd provides Gherkin-compatible Given/When/Then syntax while maintaining cargo test compatibility. All BDD tests integrate with existing unit and integration tests.

## rstest-bdd Framework Overview

### Installation

Add to `engine/snps-core/Cargo.toml`:

```toml
[dev-dependencies]
rstest = "0.26.1"
rstest-bdd = "0.2.0"
```

For compile-time step validation:
```toml
[dev-dependencies]
rstest-bdd-macros = { version = "0.2.0", features = ["compile-time-validation"] }
```

### Core Concepts

- **Step Definitions**: `#[given(...)]`, `#[when(...)]`, `#[then(...)]` decorators
- **Typed Placeholders**: Pattern matching with `{name:type}` syntax (e.g., `{count:u32}`)
- **Feature Files**: Gherkin `.feature` files with scenarios
- **Scenario Binding**: `#[scenario(path = "...", name = "...")]` attribute
- **Fixture Integration**: Works with rstest `#[fixture]` for dependency injection

### Basic Structure

```rust
use rstest_bdd::{given, when, then};

#[given("a matter file with valid frontmatter")]
fn valid_frontmatter(item: &mut MatterItem) {
    // setup
}

#[when("I parse the file")]
fn parse_file(item: &mut MatterItem) {
    // action
}

#[then("the frontmatter fields are extracted correctly")]
fn verify_frontmatter(item: &MatterItem) {
    // assertions
}
```

## Current Test Infrastructure

### Test Locations

| Type | Location | Count |
|------|----------|-------|
| Integration | `engine/snps-core/tests/` | 3 files, ~30 tests |
| Unit (inline) | `engine/snps-core/src/*.rs` | 13 modules, ~46 tests |

### Existing Dependencies

```toml
[dev-dependencies]
insta = { workspace = true }    # Snapshot testing (unused)
tokio-test = "0.4"              # Async testing (unused)
tempfile = "3.23"               # Temporary files (heavily used)
```

### Existing Patterns

1. **Temporary Directory Setup**: `tempfile::tempdir()` for database isolation
2. **Helper Functions**: `create_test_node()`, `create_test_edge()` at file bottom
3. **Result-Based Tests**: Return `Result<(), Box<dyn std::error::Error>>`
4. **Raw String Literals**: `r#"..."#` for YAML/JSON test data

## Matter Implementation Behaviors to Document

### Module: `matter.rs`

#### Enums (MatterType, ContextType, Visibility)

| Behavior | BDD Scenario |
|----------|--------------|
| Serializes to snake_case | Given a MatterType::Spec, When serialized to JSON, Then output is "spec" |
| Deserializes from snake_case | Given JSON string "team", When deserialized, Then produces ContextType::Team |
| All variants serializable | Given each enum variant, When serialized, Then produces valid JSON |

#### MatterItem

| Behavior | BDD Scenario |
|----------|--------------|
| Parses file with frontmatter | Given a markdown file with YAML frontmatter, When parsed, Then frontmatter and content are extracted |
| Generates file content | Given a MatterItem, When to_file_content called, Then output has --- delimiters |
| Saves to filesystem | Given a MatterItem, When saved, Then file exists at path with correct content |

#### Path Generation

| Behavior | BDD Scenario |
|----------|--------------|
| Includes visibility directory | Given visibility Private, When generating path, Then path contains "/private/" |
| Includes type directory | Given MatterType::Spec, When generating path, Then path contains "/specs/" |
| Slugifies title | Given title "Test Document!", When generating path, Then slug is "test-document" |
| Includes date prefix | Given any inputs, When generating path, Then filename starts with YYYY-MM-DD |

### Module: `matter/parser.rs`

| Behavior | BDD Scenario |
|----------|--------------|
| Parses valid frontmatter | Given content starting with ---, When parsed, Then frontmatter struct populated |
| Extracts remaining content | Given file with body text, When parsed, Then content string returned |
| Rejects missing opening delimiter | Given content without ---, When parsed, Then error returned |
| Rejects missing closing delimiter | Given content with unclosed ---, When parsed, Then error returned |
| Round-trip preservation | Given MatterItem, When saved and reparsed, Then values match |

### Module: `config.rs`

| Behavior | BDD Scenario |
|----------|--------------|
| Default config has expected values | Given no config file, When default loaded, Then version is "1.0" |
| Loads from YAML file | Given config.yaml exists, When loaded, Then values from file used |
| Saves config to file | Given GlobalConfig, When saved, Then YAML file created |
| Merges with precedence | Given personal and team configs, When merged, Then team overrides personal |
| Tracks config sources | Given merged config, When source queried, Then correct source returned |

### Module: `index.rs`

| Behavior | BDD Scenario |
|----------|--------------|
| Creates index with database | Given valid db path, When MatterIndex::new called, Then index created |
| Indexes matter file | Given markdown file, When index_file called, Then record in database |
| Removes from index | Given indexed file, When remove_from_index called, Then record deleted |
| Rebuilds full index | Given directory with .md files, When rebuild_index called, Then all files indexed |
| Searches by title prefix | Given indexed files, When search called, Then matching results returned |
| Calculates content hash | Given file content, When indexed, Then deterministic hash stored |
| Extracts repository ID | Given file in repo, When indexed, Then repository_id field populated |

### Module: `repository.rs`

| Behavior | BDD Scenario |
|----------|--------------|
| Default sync config | Given no config, When default created, Then enabled=false, branch="main" |
| Loads repositories | Given repositories.yaml, When loaded, Then mappings available |
| Adds repository | Given new mapping, When add_repository called, Then saved to config |
| Rejects duplicate ID | Given existing repository, When adding same ID, Then error returned |
| Removes repository | Given existing repository, When remove_repository called, Then removed from config |
| Rejects missing ID on remove | Given non-existent ID, When remove_repository called, Then error returned |

## Proposed BDD Test Structure

### Directory Organization

```
engine/snps-core/
├── tests/
│   ├── features/                    # Gherkin feature files
│   │   ├── matter_parsing.feature
│   │   ├── matter_indexing.feature
│   │   ├── config_management.feature
│   │   └── repository_management.feature
│   ├── steps/                       # Step definitions
│   │   ├── mod.rs
│   │   ├── matter_steps.rs
│   │   ├── config_steps.rs
│   │   ├── index_steps.rs
│   │   └── repository_steps.rs
│   └── bdd_tests.rs                 # Test runner
```

### Example Feature File: `features/matter_parsing.feature`

```gherkin
Feature: Matter Document Parsing
  As a developer
  I want to parse Matter documents with YAML frontmatter
  So that I can manage knowledge artifacts programmatically

  Background:
    Given a temporary directory for test files

  Scenario: Parse valid frontmatter
    Given a markdown file with content:
      """
      ---
      matter_type: spec
      title: Test Specification
      context_type: user
      context_id: igor
      visibility: private
      tags:
        - test
        - bdd
      created_at: '2024-01-01T00:00:00Z'
      created_by: igor
      ---

      This is the document content.
      """
    When I parse the file
    Then the frontmatter title is "Test Specification"
    And the frontmatter matter_type is "spec"
    And the content is "This is the document content."

  Scenario: Reject file without frontmatter delimiter
    Given a markdown file with content:
      """
      No frontmatter here
      Just regular markdown
      """
    When I try to parse the file
    Then parsing fails with "Content does not start with frontmatter delimiter"

  Scenario Outline: Generate correct path for different types
    Given matter_type "<type>" and visibility "<visibility>" and title "<title>"
    When I generate the path
    Then the path contains "/<visibility_dir>/<type_dir>/"
    And the filename ends with "<expected_slug>.md"

    Examples:
      | type       | visibility | title           | visibility_dir | type_dir   | expected_slug      |
      | spec       | private    | My Spec         | private        | specs      | my-spec            |
      | document   | shared     | Team Doc        | shared         | documents  | team-doc           |
      | research   | public     | API Research!   | public         | research   | api-research       |
```

### Example Step Definitions: `steps/matter_steps.rs`

```rust
use rstest_bdd::{given, when, then};
use snps_core::matter::{MatterItem, MatterType, Visibility, generate_matter_path};
use std::path::PathBuf;
use tempfile::TempDir;

// World/Context struct for sharing state
pub struct MatterWorld {
    pub temp_dir: Option<TempDir>,
    pub file_path: Option<PathBuf>,
    pub parsed_item: Option<MatterItem>,
    pub parse_error: Option<String>,
    pub generated_path: Option<PathBuf>,
}

impl Default for MatterWorld {
    fn default() -> Self {
        Self {
            temp_dir: None,
            file_path: None,
            parsed_item: None,
            parse_error: None,
            generated_path: None,
        }
    }
}

#[given("a temporary directory for test files")]
fn setup_temp_dir(world: &mut MatterWorld) {
    world.temp_dir = Some(tempfile::tempdir().unwrap());
}

#[given("a markdown file with content:")]
fn create_markdown_file(world: &mut MatterWorld, content: String) {
    let temp_dir = world.temp_dir.as_ref().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, content).unwrap();
    world.file_path = Some(file_path);
}

#[when("I parse the file")]
fn parse_file(world: &mut MatterWorld) {
    let file_path = world.file_path.as_ref().unwrap();
    match MatterItem::parse_file(file_path) {
        Ok(item) => world.parsed_item = Some(item),
        Err(e) => world.parse_error = Some(e.to_string()),
    }
}

#[when("I try to parse the file")]
fn try_parse_file(world: &mut MatterWorld) {
    parse_file(world);
}

#[then("the frontmatter title is {title:String}")]
fn verify_title(world: &MatterWorld, title: String) {
    let item = world.parsed_item.as_ref().unwrap();
    assert_eq!(item.frontmatter.title, title);
}

#[then("the frontmatter matter_type is {type_str:String}")]
fn verify_matter_type(world: &MatterWorld, type_str: String) {
    let item = world.parsed_item.as_ref().unwrap();
    let expected = serde_json::from_str::<MatterType>(&format!("\"{}\"", type_str)).unwrap();
    assert_eq!(item.frontmatter.matter_type, expected);
}

#[then("the content is {expected:String}")]
fn verify_content(world: &MatterWorld, expected: String) {
    let item = world.parsed_item.as_ref().unwrap();
    assert_eq!(item.content.trim(), expected);
}

#[then("parsing fails with {message:String}")]
fn verify_parse_error(world: &MatterWorld, message: String) {
    let error = world.parse_error.as_ref().expect("Expected error but parsing succeeded");
    assert!(error.contains(&message), "Error '{}' does not contain '{}'", error, message);
}

// Path generation steps
#[given("matter_type {type_str:String} and visibility {vis_str:String} and title {title:String}")]
fn setup_path_params(world: &mut MatterWorld, type_str: String, vis_str: String, title: String) {
    world.temp_dir = Some(tempfile::tempdir().unwrap());
    // Store params for when step
}

#[when("I generate the path")]
fn generate_path(world: &mut MatterWorld) {
    let repo_path = world.temp_dir.as_ref().unwrap().path().to_path_buf();
    // Use stored params to generate path
    world.generated_path = Some(generate_matter_path(
        &repo_path,
        &MatterType::Spec,
        "Test",
        &Visibility::Private,
    ));
}

#[then("the path contains {substring:String}")]
fn verify_path_contains(world: &MatterWorld, substring: String) {
    let path = world.generated_path.as_ref().unwrap();
    let path_str = path.to_string_lossy();
    assert!(path_str.contains(&substring), "Path '{}' does not contain '{}'", path_str, substring);
}
```

### Test Runner: `tests/bdd_tests.rs`

```rust
//! BDD tests for Matter protocol implementation
//!
//! Uses rstest-bdd for Gherkin-style behavior documentation.

mod steps;

use rstest_bdd::scenario;
use steps::matter_steps::MatterWorld;

#[scenario(path = "tests/features/matter_parsing.feature", name = "Parse valid frontmatter")]
fn test_parse_valid_frontmatter() {
    let mut world = MatterWorld::default();
    // Steps executed automatically based on feature file
}

#[scenario(path = "tests/features/matter_parsing.feature", name = "Reject file without frontmatter delimiter")]
fn test_reject_missing_delimiter() {
    let mut world = MatterWorld::default();
}
```

## CLI-Based Integration Tests (Full User Flows)

### Scope

CLI integration tests cover complete user workflows from command execution through file operations, indexing, and search. These tests verify the full stack: CLI → snps-core → filesystem → database → search results.

**Commands in Scope:**
- `snps matter create/list/search/show/edit/delete/import`
- `snps repo init/add/list/sync/index`
- `snps config show/init`
- All commands that interact with Matter files and indexing

**Out of Scope:**
- Daemon operations (`snps daemon`)
- UI interactions (`snps ui`)
- Claude session management (separate from Matter)

### CLI Command Workflows

#### Matter Management Commands (`engine/snps-cli/src/main.rs:666-719`)

| Command | Behavior | Integration Points |
|---------|----------|-------------------|
| `snps matter create` | Create matter file → Index automatically | matter.rs, index.rs, config.rs |
| `snps matter list` | List matter files from repository | repository.rs, filesystem |
| `snps matter search <query>` | Search index → Return results | index.rs, config.rs |
| `snps matter show <id>` | Display matter file content | filesystem, matter.rs |
| `snps matter edit <id>` | Edit → Re-index | matter.rs, index.rs |
| `snps matter delete <id>` | Delete → Remove from index | filesystem, index.rs |
| `snps matter import <file>` | Import external file as matter | matter.rs, index.rs |

#### Repository Commands (`engine/snps-cli/src/main.rs:759-812`)

| Command | Behavior | Integration Points |
|---------|----------|-------------------|
| `snps repo init <path>` | Initialize repository → Add config | repository.rs, config.rs |
| `snps repo add <path>` | Add existing repository | repository.rs |
| `snps repo list` | List all repositories | repository.rs |
| `snps repo sync [id]` | Git pull/push repository | repository.rs, git |
| `snps repo index [id]` | Rebuild matter index | index.rs, walkdir |

### Full-Flow BDD Scenarios

#### Feature File: `features/cli_matter_workflow.feature`

```gherkin
Feature: Matter Knowledge Management Workflow
  As a knowledge worker
  I want to create, search, and manage matter documents via CLI
  So that I can organize my research and documentation

  Background:
    Given a temporary workspace directory
    And a test repository is initialized
    And the index database is initialized

  Scenario: Create and search matter document
    When I run "snps matter create spec 'API Design' --context user --id testuser --tags api,design"
    Then the command succeeds
    And a matter file is created with title "API Design"
    And the file is automatically indexed

    When I run "snps matter search 'API Design'"
    Then the command succeeds
    And the search results contain 1 item
    And the result title is "API Design"
    And the result matter_type is "spec"
    And the result tags include "api" and "design"

  Scenario: Edit matter document and verify re-indexing
    Given a matter file exists with title "Draft Proposal"
    And the file is indexed

    When I edit the matter file to change title to "Final Proposal"
    And I run "snps matter edit <id>"
    Then the command succeeds
    And the file is re-indexed

    When I run "snps matter search 'Final Proposal'"
    Then the search results contain the updated title
    And the search for "Draft Proposal" returns no results

  Scenario: Delete matter document and verify index cleanup
    Given a matter file exists with title "Obsolete Spec"
    And the file is indexed

    When I run "snps matter delete <id> --force"
    Then the command succeeds
    And the matter file is deleted from filesystem
    And the file is removed from search index

    When I run "snps matter search 'Obsolete Spec'"
    Then the search results are empty

  Scenario: Import external markdown as matter
    Given an external markdown file "external-doc.md" with content:
      """
      # Research Notes

      This is imported research.
      """

    When I run "snps matter import external-doc.md --matter-type research --context user"
    Then the command succeeds
    And a matter file is created with frontmatter
    And the frontmatter matter_type is "research"
    And the content is preserved
    And the file is automatically indexed

  Scenario Outline: Create different matter types and verify organization
    When I run "snps matter create <type> '<title>' --context user --visibility <visibility>"
    Then the command succeeds
    And the file path contains "/<visibility_dir>/<type_dir>/"
    And the file is indexed with matter_type "<type>"

    Examples:
      | type         | title           | visibility | visibility_dir | type_dir      |
      | spec         | User Stories    | private    | private        | specs         |
      | research     | Market Analysis | shared     | shared         | research      |
      | plan         | Q1 Roadmap      | shared     | shared         | plans         |
      | insight      | User Feedback   | private    | private        | insights      |
```

#### Feature File: `features/cli_repo_management.feature`

```gherkin
Feature: Matter Repository Management
  As a knowledge worker
  I want to manage multiple matter repositories
  So that I can organize documents by project and team

  Background:
    Given a temporary workspace directory
    And global config is initialized

  Scenario: Initialize new matter repository
    When I run "snps repo init test-repo --context team --id eng-team"
    Then the command succeeds
    And a directory "test-repo" is created
    And a ".pmsynapse" directory exists in the repository
    And a repository config file exists
    And the repository is added to global repositories list

    When I run "snps repo list"
    Then the output contains repository "test-repo"
    And the context is "team (eng-team)"

  Scenario: Add existing repository and rebuild index
    Given a directory "existing-repo" with matter files:
      | Title          | Type     | Visibility |
      | Spec A         | spec     | shared     |
      | Research B     | research | private    |
      | Plan C         | plan     | shared     |
    And a ".pmsynapse" directory exists in "existing-repo"

    When I run "snps repo add existing-repo"
    Then the command succeeds
    And the repository is added to config

    When I run "snps repo index existing-repo"
    Then the command succeeds
    And 3 files are indexed
    And all files are searchable

  Scenario: Cross-repository search
    Given repository "repo-a" contains:
      | Title       | Type | Tags        |
      | API Spec    | spec | api,backend |
    And repository "repo-b" contains:
      | Title       | Type     | Tags           |
      | API Research| research | api,market     |
    And both repositories are indexed

    When I run "snps matter search 'API'"
    Then the search results contain 2 items
    And results include files from both repositories
    And one result is from "repo-a" with type "spec"
    And one result is from "repo-b" with type "research"

  Scenario: Repository sync with git remote
    Given a repository "sync-test" with remote configured
    And local changes exist in the repository

    When I run "snps repo sync sync-test"
    Then the command succeeds
    And git pull is executed
    And git push is executed
    And the sync status is displayed

  Scenario: Disable and re-enable repository indexing
    Given a repository "test-repo" with auto_index enabled
    And the repository contains 5 matter files

    When I disable auto_index for "test-repo" in config
    And I create a new matter file in "test-repo"
    Then the new file is not automatically indexed

    When I run "snps repo index test-repo"
    Then the command succeeds
    And all 6 files are now indexed
```

#### Feature File: `features/cli_config_integration.feature`

```gherkin
Feature: Configuration Management Integration
  As a knowledge worker
  I want configuration to affect matter operations
  So that my workflow preferences are respected

  Background:
    Given a clean configuration state

  Scenario: Default config enables search indexing
    When I run "snps config show"
    Then the output shows "index_enabled: true"

    When I create a matter file
    Then the file is automatically indexed

  Scenario: Disabled search index prevents auto-indexing
    Given search.index_enabled is set to false in config

    When I create a matter file
    Then the file is created successfully
    And the file is not indexed
    And a warning is displayed about disabled indexing

  Scenario: Config precedence affects matter creation
    Given global config has default visibility "private"
    And team config overrides visibility to "shared"

    When I create a matter file without specifying visibility
    Then the file is created in "/shared/" directory
    And the frontmatter visibility is "shared"

  Scenario: Custom repositories root affects file paths
    Given config.repositories_root is set to "/custom/path"

    When I run "snps repo init my-repo --context user --id testuser"
    Then the repository is created at "/custom/path/my-repo"
```

### CLI Test Structure

```
engine/snps-cli/
├── tests/
│   ├── features/                        # Gherkin feature files
│   │   ├── cli_matter_workflow.feature
│   │   ├── cli_repo_management.feature
│   │   └── cli_config_integration.feature
│   ├── steps/                           # Step definitions
│   │   ├── mod.rs
│   │   ├── cli_steps.rs                # Common CLI execution steps
│   │   ├── matter_cli_steps.rs         # Matter command steps
│   │   ├── repo_cli_steps.rs           # Repo command steps
│   │   └── config_cli_steps.rs         # Config steps
│   └── cli_bdd_tests.rs                # Test runner
```

### CLI Step Definitions Example: `steps/cli_steps.rs`

```rust
use assert_cmd::Command;
use rstest_bdd::{given, when, then};
use std::path::PathBuf;
use tempfile::TempDir;
use snps_core::index::MatterIndex;
use snps_core::config::GlobalConfig;

/// Shared test context for CLI scenarios
pub struct CliWorld {
    pub workspace: Option<TempDir>,
    pub last_command: Option<Command>,
    pub last_output: Option<std::process::Output>,
    pub index_db: Option<PathBuf>,
    pub created_files: Vec<PathBuf>,
    pub repositories: Vec<String>,
}

impl Default for CliWorld {
    fn default() -> Self {
        Self {
            workspace: None,
            last_command: None,
            last_output: None,
            index_db: None,
            created_files: Vec::new(),
            repositories: Vec::new(),
        }
    }
}

#[given("a temporary workspace directory")]
fn setup_workspace(world: &mut CliWorld) {
    let temp_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    world.workspace = Some(temp_dir);
}

#[given("a test repository is initialized")]
fn init_test_repository(world: &mut CliWorld) {
    let repo_path = world.workspace.as_ref().unwrap().path().join("test-repo");
    std::fs::create_dir_all(&repo_path).unwrap();
    std::fs::create_dir(repo_path.join(".pmsynapse")).unwrap();
    world.repositories.push("test-repo".to_string());
}

#[given("the index database is initialized")]
fn init_index_database(world: &mut CliWorld) {
    let db_path = world.workspace.as_ref().unwrap().path().join("index.db");
    let mut index = MatterIndex::new(db_path.to_str().unwrap()).unwrap();
    world.index_db = Some(db_path);
}

#[when("I run {command:String}")]
fn run_cli_command(world: &mut CliWorld, command: String) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let mut cmd = Command::cargo_bin("snps").unwrap();

    // Skip "snps" prefix if present
    let args = if parts[0] == "snps" {
        &parts[1..]
    } else {
        &parts[..]
    };

    cmd.args(args);
    world.last_output = Some(cmd.output().unwrap());
}

#[then("the command succeeds")]
fn verify_success(world: &CliWorld) {
    let output = world.last_output.as_ref().unwrap();
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then("the output contains {text:String}")]
fn verify_output_contains(world: &CliWorld, text: String) {
    let output = world.last_output.as_ref().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&text),
        "Output does not contain '{}'\nActual output:\n{}",
        text, stdout
    );
}
```

### CLI-Specific Step Definitions: `steps/matter_cli_steps.rs`

```rust
use rstest_bdd::{given, when, then};
use super::cli_steps::CliWorld;
use snps_core::matter::MatterItem;
use snps_core::index::MatterIndex;

#[then("a matter file is created with title {title:String}")]
fn verify_file_created(world: &CliWorld, title: String) {
    // Parse output to get file path
    let output = String::from_utf8_lossy(
        &world.last_output.as_ref().unwrap().stdout
    );

    // Extract path from output (assumes format: "✓ Created matter file at: /path/to/file.md")
    let path_line = output.lines()
        .find(|l| l.contains("Created matter file at:"))
        .expect("Could not find file path in output");

    let path_str = path_line.split("at:").nth(1).unwrap().trim();
    let path = std::path::PathBuf::from(path_str);

    assert!(path.exists(), "Matter file was not created at {:?}", path);

    // Parse and verify title
    let item = MatterItem::parse_file(&path).unwrap();
    assert_eq!(item.frontmatter.title, title);
}

#[then("the file is automatically indexed")]
fn verify_auto_indexed(world: &CliWorld) {
    // Wait briefly for async indexing (if applicable)
    std::thread::sleep(std::time::Duration::from_millis(100));

    let db_path = world.index_db.as_ref().expect("Index DB not initialized");
    let index = MatterIndex::new(db_path.to_str().unwrap()).unwrap();

    // Verify last created file is in index
    let last_file = world.created_files.last().expect("No files created");

    // Search for file in index (implementation depends on index API)
    // This is a placeholder - actual implementation would query by file_path
    let results = index.search("").unwrap();
    assert!(
        results.iter().any(|r| r.file_path == last_file.to_string_lossy()),
        "File not found in search index"
    );
}

#[then("the search results contain {count:usize} item(s)")]
fn verify_search_count(world: &CliWorld, count: usize) {
    let output = String::from_utf8_lossy(&world.last_output.as_ref().unwrap().stdout);

    // Parse "Found X results:" line
    let results_line = output.lines()
        .find(|l| l.contains("Found") && l.contains("results"))
        .expect("Could not find results count in output");

    let actual_count: usize = results_line
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    assert_eq!(actual_count, count, "Expected {} results, found {}", count, actual_count);
}

#[then("the result title is {title:String}")]
fn verify_result_title(world: &CliWorld, title: String) {
    let output = String::from_utf8_lossy(&world.last_output.as_ref().unwrap().stdout);
    assert!(output.contains(&title), "Results do not contain title '{}'", title);
}

#[then("the result matter_type is {matter_type:String}")]
fn verify_result_type(world: &CliWorld, matter_type: String) {
    let output = String::from_utf8_lossy(&world.last_output.as_ref().unwrap().stdout);
    assert!(output.contains(&matter_type), "Results do not show matter_type '{}'", matter_type);
}
```

### Test Runner Integration: `tests/cli_bdd_tests.rs`

```rust
//! CLI integration BDD tests
//!
//! Tests full user workflows via CLI commands.

mod steps;

use rstest_bdd::scenario;
use steps::cli_steps::CliWorld;

#[scenario(
    path = "tests/features/cli_matter_workflow.feature",
    name = "Create and search matter document"
)]
fn test_create_and_search() {
    let mut world = CliWorld::default();
    // Steps execute automatically
}

#[scenario(
    path = "tests/features/cli_matter_workflow.feature",
    name = "Edit matter document and verify re-indexing"
)]
fn test_edit_and_reindex() {
    let mut world = CliWorld::default();
}

#[scenario(
    path = "tests/features/cli_matter_workflow.feature",
    name = "Delete matter document and verify index cleanup"
)]
fn test_delete_and_cleanup() {
    let mut world = CliWorld::default();
}

// Additional scenario bindings...
```

### Required Dependencies for CLI Testing

Add to `engine/snps-cli/Cargo.toml`:

```toml
[dev-dependencies]
rstest = "0.26.1"
rstest-bdd = "0.2.0"
assert_cmd = "2"           # CLI testing utilities
predicates = "3"           # Assertion helpers
tempfile = { workspace = true }
```

### Integration with Cargo Test

All BDD tests run via standard `cargo test`:

```bash
# Run all tests (unit + BDD)
cargo test

# Run only CLI BDD tests
cargo test --test cli_bdd_tests

# Run specific feature
cargo test --test cli_bdd_tests test_create_and_search

# Run only snps-core BDD tests
cd engine/snps-core && cargo test --test bdd_tests

# Run only snps-cli BDD tests
cd engine/snps-cli && cargo test --test cli_bdd_tests

# Run all tests with output
cargo test -- --nocapture
```

### Test Organization Summary

| Test Level | Location | Tools | Scope |
|------------|----------|-------|-------|
| Unit BDD | `engine/snps-core/tests/bdd_tests.rs` | rstest-bdd | Module behaviors (parsing, config, index) |
| Integration BDD | `engine/snps-cli/tests/cli_bdd_tests.rs` | rstest-bdd + assert_cmd | Full CLI workflows |
| Existing Unit | `engine/snps-core/src/**/*.rs` | Standard Rust | Quick validation |
| Existing Integration | `engine/snps-core/tests/*.rs` | Standard Rust | Component integration |

## Integration with Existing Tests

### Coexistence Strategy

- Keep existing unit tests for quick validation
- Add unit-level BDD tests (snps-core) for behavior documentation
- Add CLI-level BDD tests (snps-cli) for user workflow verification
- BDD tests serve as executable specifications
- All tests run with `cargo test`

### Migration Path

1. **Phase 1**: Add rstest-bdd dependencies
2. **Phase 2**: Create feature files for Matter modules
3. **Phase 3**: Implement step definitions
4. **Phase 4**: Add scenario bindings
5. **Phase 5**: Run alongside existing tests

## Code References

- `engine/snps-core/src/matter.rs` - MatterType, ContextType, Visibility, MatterItem, generate_matter_path
- `engine/snps-core/src/matter/parser.rs:9-33` - parse_frontmatter function
- `engine/snps-core/src/config.rs:44-77` - GlobalConfig::default()
- `engine/snps-core/src/config.rs:143-188` - load_merged_config()
- `engine/snps-core/src/index.rs:20-145` - MatterIndex indexing operations
- `engine/snps-core/src/repository.rs:54-120` - Repository CRUD operations
- `engine/snps-core/tests/graph_tests.rs:167-191` - Existing helper function pattern

## Architecture Documentation

### rstest-bdd Integration Points

1. **Cargo.toml**: Add dev-dependencies for rstest and rstest-bdd
2. **Feature Files**: Store in `tests/features/` directory
3. **Step Definitions**: Rust modules with `#[given]`, `#[when]`, `#[then]` attributes
4. **World Struct**: Shared state container passed to step functions
5. **Scenario Binding**: Connect test functions to feature file scenarios

### Test Execution Flow

```
cargo test
  └── tests/bdd_tests.rs
       └── #[scenario(...)]
            └── Feature file parsed
                 └── Steps matched to definitions
                      └── MatterWorld state shared
                           └── Assertions executed
```

## Related Resources

- [rstest-bdd on lib.rs](https://lib.rs/crates/rstest-bdd) - Crate documentation
- [rstest](https://docs.rs/rstest) - Underlying testing framework
- [Gherkin syntax](https://cucumber.io/docs/gherkin/) - Feature file format

## Open Questions

1. **Async Support**: Should BDD tests use `#[tokio::test]` for async operations? (rstest-bdd supports tokio feature flag)
2. **Snapshot Integration**: Can insta snapshots be used within BDD step definitions for complex output verification?
3. **CI Integration**: How to report BDD test results in CI pipeline?
4. **Feature File Location**: Should features live in `tests/features/` or separate `features/` at crate root?
