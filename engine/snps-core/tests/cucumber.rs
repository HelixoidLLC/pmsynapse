// Cucumber BDD tests for snps-core

use cucumber::{given, then, when, World};
use snps_core::config::{load_merged_config, ConfigSource, GlobalConfig};
use snps_core::index::{MatterIndex, MatterSearchResult};
use snps_core::matter::{generate_matter_path, MatterItem, MatterType, Visibility};
use snps_core::repository::{ContextType, RepositorySyncConfig, Visibility as RepoVisibility};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

// Wrapper for MatterIndex that implements Debug
#[derive(World, Default)]
#[world(init = Self::new)]
pub struct MatterWorld {
    temp_dir: Option<TempDir>,
    file_path: Option<PathBuf>,
    matter_item: Option<MatterItem>,
    parse_error: Option<String>,
    generated_path: Option<PathBuf>,

    // For indexing tests
    index: Option<MatterIndex>,
    db_path: Option<PathBuf>,
    search_results: Option<Vec<MatterSearchResult>>,

    // For config tests
    config: Option<GlobalConfig>,
    serialized_config: Option<String>,
    deserialized_config: Option<GlobalConfig>,
    merged_config_sources: Option<HashMap<String, ConfigSource>>,

    // For repository tests
    sync_config: Option<RepositorySyncConfig>,
    context_type: Option<ContextType>,
    repo_visibility: Option<RepoVisibility>,
    serialized_json: Option<String>,

    // Path generation parameters
    matter_type: Option<MatterType>,
    visibility: Option<Visibility>,
    title: Option<String>,
}

impl std::fmt::Debug for MatterWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatterWorld").finish()
    }
}

impl MatterWorld {
    fn new() -> Self {
        Self::default()
    }
}

// ============================================================================
// MATTER PARSING STEPS
// ============================================================================

#[given("a markdown file with valid frontmatter")]
async fn given_valid_frontmatter(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");

    let content = r#"---
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

This is the document content."#;

    std::fs::write(&file_path, content).unwrap();

    world.file_path = Some(file_path);
    world.temp_dir = Some(temp_dir);
}

#[given("a markdown file without frontmatter delimiter")]
async fn given_no_frontmatter_delimiter(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    let content = "No frontmatter here\nJust regular markdown";
    std::fs::write(&file_path, content).unwrap();

    world.file_path = Some(file_path);
    world.temp_dir = Some(temp_dir);
}

#[given("a markdown file with unclosed frontmatter")]
async fn given_unclosed_frontmatter(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    let content = r#"---
matter_type: spec
title: Test"#;
    std::fs::write(&file_path, content).unwrap();

    world.file_path = Some(file_path);
    world.temp_dir = Some(temp_dir);
}

#[when("I parse the file")]
async fn when_parse_file(world: &mut MatterWorld) {
    if let Some(ref path) = world.file_path {
        match MatterItem::parse_file(path) {
            Ok(item) => world.matter_item = Some(item),
            Err(e) => world.parse_error = Some(e.to_string()),
        }
    }
}

#[when("I try to parse the file")]
async fn when_try_parse_file(world: &mut MatterWorld) {
    when_parse_file(world).await;
}

#[then("the frontmatter should be parsed correctly")]
async fn then_frontmatter_parsed(world: &mut MatterWorld) {
    let item = world.matter_item.as_ref().expect("MatterItem should exist");
    assert_eq!(item.frontmatter.title, "Test Specification");
    assert_eq!(item.frontmatter.matter_type, MatterType::Spec);
}

#[then(regex = r#"^the content should be "([^"]*)"$"#)]
async fn then_content_should_be(world: &mut MatterWorld, expected: String) {
    let item = world.matter_item.as_ref().expect("MatterItem should exist");
    assert_eq!(item.content.trim(), expected);
}

#[then("parsing should fail")]
async fn then_parsing_fails(world: &mut MatterWorld) {
    assert!(world.parse_error.is_some(), "Expected parsing to fail");
}

#[then(regex = r#"^the error should contain "([^"]*)"$"#)]
async fn then_error_contains(world: &mut MatterWorld, expected: String) {
    let error = world.parse_error.as_ref().expect("Error should exist");
    assert!(
        error.contains(&expected),
        "Error '{}' should contain '{}'",
        error,
        expected
    );
}

// ============================================================================
// PATH GENERATION STEPS
// ============================================================================

#[given(regex = r#"^matter_type "([^"]*)" and visibility "([^"]*)"$"#)]
async fn given_matter_type_and_visibility(
    world: &mut MatterWorld,
    matter_type: String,
    visibility: String,
) {
    world.matter_type = Some(match matter_type.as_str() {
        "spec" => MatterType::Spec,
        "document" => MatterType::Document,
        "research" => MatterType::Research,
        _ => panic!("Unknown matter_type: {}", matter_type),
    });

    world.visibility = Some(match visibility.as_str() {
        "shared" => Visibility::Shared,
        "private" => Visibility::Private,
        "public" => Visibility::Public,
        _ => panic!("Unknown visibility: {}", visibility),
    });

    world.temp_dir = Some(TempDir::new().unwrap());
}

#[given(regex = r#"^title "([^"]*)"$"#)]
async fn given_title(world: &mut MatterWorld, title: String) {
    world.title = Some(title);
}

#[when("I generate the path")]
async fn when_generate_path(world: &mut MatterWorld) {
    let temp_dir = world.temp_dir.as_ref().unwrap();
    let repo_path = temp_dir.path();
    let matter_type = world.matter_type.as_ref().unwrap();
    let visibility = world.visibility.as_ref().unwrap();
    let title = world.title.as_ref().unwrap();

    let path = generate_matter_path(repo_path, matter_type, title, visibility);
    world.generated_path = Some(path);
}

#[then(regex = r#"^the path should contain "([^"]*)"$"#)]
async fn then_path_contains(world: &mut MatterWorld, expected: String) {
    let path = world.generated_path.as_ref().unwrap();
    let path_str = path.to_string_lossy();
    assert!(
        path_str.contains(&expected),
        "Path '{}' should contain '{}'",
        path_str,
        expected
    );
}

#[then(regex = r#"^the filename should end with "([^"]*)"$"#)]
async fn then_filename_ends_with(world: &mut MatterWorld, expected: String) {
    let path = world.generated_path.as_ref().unwrap();
    let filename = path.file_name().unwrap().to_string_lossy();
    assert!(
        filename.ends_with(&expected),
        "Filename '{}' should end with '{}'",
        filename,
        expected
    );
}

#[then(regex = r#"^the filename should contain "([^"]*)"$"#)]
async fn then_filename_contains(world: &mut MatterWorld, expected: String) {
    let path = world.generated_path.as_ref().unwrap();
    let filename = path.file_name().unwrap().to_string_lossy();
    assert!(
        filename.contains(&expected),
        "Filename '{}' should contain '{}'",
        filename,
        expected
    );
}

// ============================================================================
// INDEXING STEPS
// ============================================================================

#[given("a valid database path")]
async fn given_valid_db_path(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    world.db_path = Some(db_path);
    world.temp_dir = Some(temp_dir);
}

#[given("a MatterIndex with a database")]
async fn given_matter_index(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let index = MatterIndex::new(db_path.to_str().unwrap()).unwrap();
    world.index = Some(index);
    world.temp_dir = Some(temp_dir);
}

#[given(regex = r#"^a matter file with title "([^"]*)"$"#)]
async fn given_matter_file(world: &mut MatterWorld, title: String) {
    let temp_dir = world.temp_dir.as_ref().unwrap();
    let repo_dir = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_dir).unwrap();
    std::fs::create_dir(repo_dir.join(".pmsynapse")).unwrap();

    let file_path = repo_dir.join("test.md");
    let content = format!(
        r#"---
matter_type: document
title: {}
context_type: user
context_id: testuser
visibility: private
tags:
  - test
created_at: '2024-01-01T00:00:00Z'
created_by: testuser
---

Test content"#,
        title
    );
    std::fs::write(&file_path, content).unwrap();
    world.file_path = Some(file_path);
}

#[given("a MatterIndex with indexed files")]
async fn given_index_with_files(world: &mut MatterWorld) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let index = MatterIndex::new(db_path.to_str().unwrap()).unwrap();

    let repo_dir = temp_dir.path().join("repo");
    std::fs::create_dir_all(&repo_dir).unwrap();
    std::fs::create_dir(repo_dir.join(".pmsynapse")).unwrap();

    for (i, title) in ["API Specification", "API Design Document", "User Guide"]
        .iter()
        .enumerate()
    {
        let file_path = repo_dir.join(format!("test{}.md", i));
        let content = format!(
            r#"---
matter_type: document
title: {}
context_type: user
context_id: testuser
visibility: private
tags:
  - test
created_at: '2024-01-01T00:00:00Z'
created_by: testuser
---

Content for {}"#,
            title, title
        );
        std::fs::write(&file_path, content).unwrap();
        index.index_file(&file_path).unwrap();
    }

    world.index = Some(index);
    world.temp_dir = Some(temp_dir);
}

#[when("I create a MatterIndex")]
async fn when_create_index(world: &mut MatterWorld) {
    let db_path = world.db_path.as_ref().unwrap();
    let result = MatterIndex::new(db_path.to_str().unwrap());
    assert!(result.is_ok());
    world.index = Some(result.unwrap());
}

#[when("I index the file")]
async fn when_index_file(world: &mut MatterWorld) {
    let index = world.index.as_ref().unwrap();
    let file_path = world.file_path.as_ref().unwrap();
    let result = index.index_file(file_path);
    assert!(result.is_ok(), "Failed to index file: {:?}", result.err());
}

#[when(regex = r#"^I search for "([^"]*)"$"#)]
async fn when_search(world: &mut MatterWorld, query: String) {
    let index = world.index.as_ref().unwrap();
    let results = index.search(&query).unwrap();
    world.search_results = Some(results);
}

#[then("the index should be created successfully")]
async fn then_index_created(_world: &mut MatterWorld) {
    // Already verified in when_create_index
}

#[then("the file should be indexed successfully")]
async fn then_file_indexed(_world: &mut MatterWorld) {
    // Already verified in when_index_file
}

#[then(regex = r"^I should get (\d+) results?$")]
async fn then_should_get_results(world: &mut MatterWorld, count: usize) {
    let results = world
        .search_results
        .as_ref()
        .expect("Search results should exist");
    assert_eq!(
        results.len(),
        count,
        "Expected {} results, got {}",
        count,
        results.len()
    );
}

// ============================================================================
// CONFIG STEPS
// ============================================================================

#[when("I create a default config")]
async fn when_create_default_config(world: &mut MatterWorld) {
    world.config = Some(GlobalConfig::default());
}

#[given("a default config")]
async fn given_default_config(world: &mut MatterWorld) {
    world.config = Some(GlobalConfig::default());
}

#[when("I serialize and deserialize the config")]
async fn when_serialize_deserialize_config(world: &mut MatterWorld) {
    let config = world.config.as_ref().unwrap();
    let yaml = serde_yaml::to_string(config).unwrap();
    let deserialized: GlobalConfig = serde_yaml::from_str(&yaml).unwrap();
    world.serialized_config = Some(yaml);
    world.deserialized_config = Some(deserialized);
}

#[when("I load merged config with no overrides")]
async fn when_load_merged_config(world: &mut MatterWorld) {
    let merged = load_merged_config(None, None).unwrap();
    world.merged_config_sources = Some(merged.sources);
}

#[then(regex = r#"^the version should be "([^"]*)"$"#)]
async fn then_version_should_be(world: &mut MatterWorld, expected: String) {
    let config = world.config.as_ref().unwrap();
    assert_eq!(config.version, expected);
}

#[then(regex = r#"^the default matter_type should be "([^"]*)"$"#)]
async fn then_default_matter_type(world: &mut MatterWorld, expected: String) {
    let config = world.config.as_ref().unwrap();
    assert_eq!(config.defaults.matter_type, expected);
}

#[then(regex = r#"^the default visibility should be "([^"]*)"$"#)]
async fn then_default_visibility(world: &mut MatterWorld, expected: String) {
    let config = world.config.as_ref().unwrap();
    assert_eq!(config.defaults.visibility, expected);
}

#[then("index_enabled should be true")]
async fn then_index_enabled(world: &mut MatterWorld) {
    let config = world.config.as_ref().unwrap();
    assert!(config.search.index_enabled);
}

#[then("auto_sync should be false")]
async fn then_auto_sync_false(world: &mut MatterWorld) {
    let config = world.config.as_ref().unwrap();
    assert!(!config.sync.auto_sync);
}

#[then("the deserialized config should match the original")]
async fn then_configs_match(world: &mut MatterWorld) {
    let original = world.config.as_ref().unwrap();
    let deserialized = world.deserialized_config.as_ref().unwrap();
    assert_eq!(original.version, deserialized.version);
    assert_eq!(
        original.defaults.matter_type,
        deserialized.defaults.matter_type
    );
    assert_eq!(
        original.defaults.visibility,
        deserialized.defaults.visibility
    );
}

#[then("config sources should be tracked")]
async fn then_sources_tracked(world: &mut MatterWorld) {
    let sources = world.merged_config_sources.as_ref().unwrap();
    assert!(!sources.is_empty());
}

#[then(regex = r#"^"([^"]*)" should have source "([^"]*)"$"#)]
async fn then_source_should_be(world: &mut MatterWorld, key: String, expected: String) {
    let sources = world.merged_config_sources.as_ref().unwrap();
    let source = sources
        .get(&key)
        .unwrap_or_else(|| panic!("Key '{}' should exist", key));
    let expected_source = match expected.as_str() {
        "Default" => ConfigSource::Default,
        _ => panic!("Unknown source: {}", expected),
    };
    assert_eq!(*source, expected_source);
}

// ============================================================================
// REPOSITORY STEPS
// ============================================================================

#[when("I create a default sync config")]
async fn when_create_sync_config(world: &mut MatterWorld) {
    world.sync_config = Some(RepositorySyncConfig::default());
}

#[then("sync enabled should be false")]
async fn then_sync_disabled(world: &mut MatterWorld) {
    let sync = world.sync_config.as_ref().unwrap();
    assert!(!sync.enabled);
}

#[then(regex = r#"^the branch should be "([^"]*)"$"#)]
async fn then_branch_should_be(world: &mut MatterWorld, expected: String) {
    let sync = world.sync_config.as_ref().unwrap();
    assert_eq!(sync.branch, expected);
}

#[then("remote should be none")]
async fn then_remote_none(world: &mut MatterWorld) {
    let sync = world.sync_config.as_ref().unwrap();
    assert!(sync.remote.is_none());
}

#[given(regex = r#"^a ContextType "([^"]*)"$"#)]
async fn given_context_type(world: &mut MatterWorld, context_type: String) {
    world.context_type = Some(match context_type.as_str() {
        "team" => ContextType::Team,
        "user" => ContextType::User,
        "project" => ContextType::Project,
        _ => panic!("Unknown context type: {}", context_type),
    });
}

#[given(regex = r#"^a Visibility "([^"]*)"$"#)]
async fn given_visibility(world: &mut MatterWorld, visibility: String) {
    world.repo_visibility = Some(match visibility.as_str() {
        "shared" => RepoVisibility::Shared,
        "private" => RepoVisibility::Private,
        _ => panic!("Unknown visibility: {}", visibility),
    });
}

#[when("I serialize it to JSON")]
async fn when_serialize_to_json(world: &mut MatterWorld) {
    if let Some(ref context_type) = world.context_type {
        world.serialized_json = Some(serde_json::to_string(context_type).unwrap());
    } else if let Some(ref visibility) = world.repo_visibility {
        world.serialized_json = Some(serde_json::to_string(visibility).unwrap());
    }
}

#[then(regex = r#"^it should serialize to "([^"]*)"$"#)]
async fn then_serializes_to(world: &mut MatterWorld, expected: String) {
    let json = world.serialized_json.as_ref().unwrap();
    assert_eq!(json, &format!(r#""{}""#, expected));
}

#[then("deserialize correctly")]
async fn then_deserializes_correctly(world: &mut MatterWorld) {
    let json = world.serialized_json.as_ref().unwrap();
    if world.context_type.is_some() {
        let deserialized: ContextType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, world.context_type.as_ref().unwrap().clone());
    }
}

#[tokio::main]
async fn main() {
    MatterWorld::run("features").await;
}
