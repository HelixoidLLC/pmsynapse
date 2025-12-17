//! Matter Index Module
//!
//! Provides indexing capabilities for file-based matter items using CozoDB.

use crate::graph::KnowledgeGraph;
use crate::matter::{MatterItem, MatterType};
use crate::{Result, SynapseError};
use cozo::{DataValue, ScriptMutability};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Matter index for searching and querying matter items
pub struct MatterIndex {
    graph: KnowledgeGraph,
}

impl MatterIndex {
    /// Create a new matter index
    pub fn new(db_path: &str) -> Result<Self> {
        let mut graph = KnowledgeGraph::new(db_path)?;
        graph.init()?;

        Ok(Self { graph })
    }

    /// Index a single matter file
    pub fn index_file(&self, file_path: &PathBuf) -> Result<()> {
        tracing::debug!("Indexing matter file: {:?}", file_path);

        // Parse the matter file
        let matter = MatterItem::parse_file(file_path)?;

        // Calculate content hash
        let content_hash = self.calculate_content_hash(&matter.content);

        // Generate unique ID for this matter item based on file path
        // Use a deterministic hash of the file path for consistent IDs
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        file_path.to_string_lossy().hash(&mut hasher);
        let hash = hasher.finish();
        let id = Uuid::from_u64_pair(hash, hash);

        // Serialize tags as JSON array
        let tags_json = serde_json::to_string(&matter.frontmatter.tags)
            .map_err(|e| SynapseError::Matter(format!("Failed to serialize tags: {}", e)))?;

        // Serialize matter type
        let matter_type_str = serde_json::to_string(&matter.frontmatter.matter_type)
            .map_err(|e| SynapseError::Matter(format!("Failed to serialize matter type: {}", e)))?
            .trim_matches('"')
            .to_string();

        // Serialize context type
        let context_type_str = serde_json::to_string(&matter.frontmatter.context_type)
            .map_err(|e| {
                SynapseError::Matter(format!("Failed to serialize context type: {}", e))
            })?
            .trim_matches('"')
            .to_string();

        // Serialize visibility
        let visibility_str = serde_json::to_string(&matter.frontmatter.visibility)
            .map_err(|e| SynapseError::Matter(format!("Failed to serialize visibility: {}", e)))?
            .trim_matches('"')
            .to_string();

        // Get repository ID from file path (assuming repository is the parent of .pmsynapse)
        let repository_id = self.extract_repository_id(file_path)?;

        let params = BTreeMap::from([
            ("id".to_string(), DataValue::Str(id.to_string().into())),
            (
                "repository_id".to_string(),
                DataValue::Str(repository_id.into()),
            ),
            (
                "file_path".to_string(),
                DataValue::Str(file_path.to_string_lossy().to_string().into()),
            ),
            (
                "matter_type".to_string(),
                DataValue::Str(matter_type_str.into()),
            ),
            (
                "title".to_string(),
                DataValue::Str(matter.frontmatter.title.clone().into()),
            ),
            (
                "context_type".to_string(),
                DataValue::Str(context_type_str.into()),
            ),
            (
                "context_id".to_string(),
                DataValue::Str(matter.frontmatter.context_id.clone().into()),
            ),
            (
                "visibility".to_string(),
                DataValue::Str(visibility_str.into()),
            ),
            ("tags".to_string(), DataValue::Str(tags_json.into())),
            (
                "created_at".to_string(),
                DataValue::Str(matter.frontmatter.created_at.to_rfc3339().into()),
            ),
            (
                "updated_at".to_string(),
                DataValue::Str(
                    matter
                        .frontmatter
                        .updated_at
                        .unwrap_or(matter.frontmatter.created_at)
                        .to_rfc3339()
                        .into(),
                ),
            ),
            (
                "created_by".to_string(),
                DataValue::Str(matter.frontmatter.created_by.clone().into()),
            ),
            (
                "content_hash".to_string(),
                DataValue::Str(content_hash.into()),
            ),
        ]);

        self.graph
            .db
            .run_script(
                r#"
                ?[id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash] <- [[
                    $id, $repository_id, $file_path, $matter_type, $title, $context_type, $context_id, $visibility, $tags, $created_at, $updated_at, $created_by, $content_hash
                ]]
                :put matter { id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash }
                "#,
                params,
                ScriptMutability::Mutable,
            )
            .map_err(|e| SynapseError::Matter(format!("Failed to index matter: {}", e)))?;

        tracing::info!("Indexed matter file: {:?}", file_path);
        Ok(())
    }

    /// Remove a matter file from the index
    pub fn remove_from_index(&self, file_path: &PathBuf) -> Result<()> {
        tracing::debug!("Removing matter file from index: {:?}", file_path);

        let params = BTreeMap::from([(
            "file_path".to_string(),
            DataValue::Str(file_path.to_string_lossy().to_string().into()),
        )]);

        self.graph
            .db
            .run_script(
                r#"
                ?[id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash] :=
                    *matter{id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash},
                    file_path != $file_path
                :replace matter { id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash }
                "#,
                params,
                ScriptMutability::Mutable,
            )
            .map_err(|e| SynapseError::Matter(format!("Failed to remove matter from index: {}", e)))?;

        tracing::info!("Removed matter file from index: {:?}", file_path);
        Ok(())
    }

    /// Rebuild the index for an entire repository
    pub fn rebuild_index(&self, repo_path: &PathBuf) -> Result<u32> {
        tracing::info!("Rebuilding index for repository: {:?}", repo_path);

        let mut count = 0;

        // Walk the repository and find all markdown files
        for entry in walkdir::WalkDir::new(repo_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Skip .git directory
                !e.path()
                    .components()
                    .any(|c| c.as_os_str() == ".git" || c.as_os_str() == "node_modules")
            })
        {
            let entry = entry.map_err(|e| {
                SynapseError::Matter(format!("Failed to walk directory: {}", e))
            })?;

            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "md" {
                        // Try to parse and index the file
                        // Skip if it doesn't have proper frontmatter
                        if let Ok(()) = self.index_file(&entry.path().to_path_buf()) {
                            count += 1;
                        } else {
                            tracing::debug!(
                                "Skipping file without valid frontmatter: {:?}",
                                entry.path()
                            );
                        }
                    }
                }
            }
        }

        tracing::info!("Indexed {} matter files in repository", count);
        Ok(count)
    }

    /// Search for matter items
    pub fn search(&self, query: &str) -> Result<Vec<MatterSearchResult>> {
        tracing::debug!("Searching matter index for: {}", query);

        // Simple search: find matter items where title contains query (case-insensitive)
        // Using starts_with as a simple filter (can be enhanced later with full-text search)
        let params = BTreeMap::from([("query".to_string(), DataValue::Str(query.to_lowercase().into()))]);

        let result = self.graph
            .db
            .run_script(
                r#"
                ?[id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash] :=
                    *matter{id, repository_id, file_path, matter_type, title, context_type, context_id, visibility, tags, created_at, updated_at, created_by, content_hash},
                    lower = lowercase(title),
                    starts_with(lower, $query)
                "#,
                params,
                ScriptMutability::Immutable,
            )
            .map_err(|e| SynapseError::Matter(format!("Failed to search matter: {}", e)))?;

        self.rows_to_search_results(result.rows)
    }

    /// Calculate a simple hash of content for change detection
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Extract repository ID from file path
    fn extract_repository_id(&self, file_path: &Path) -> Result<String> {
        // Look for .pmsynapse directory in parent directories
        let mut current = file_path.to_path_buf();
        while let Some(parent) = current.parent() {
            let pmsynapse_dir = parent.join(".pmsynapse");
            if pmsynapse_dir.exists() {
                // Use the parent directory name as repository ID
                return Ok(parent
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string());
            }
            current = parent.to_path_buf();
        }

        // Fallback to "default" if no .pmsynapse found
        Ok("default".to_string())
    }

    /// Convert CozoDB rows to MatterSearchResult structs
    fn rows_to_search_results(&self, rows: Vec<Vec<DataValue>>) -> Result<Vec<MatterSearchResult>> {
        rows.into_iter()
            .map(|row| {
                if row.len() != 13 {
                    return Err(SynapseError::Matter(format!(
                        "Invalid row length: expected 13, got {}",
                        row.len()
                    )));
                }

                let id = row[0]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid id type".into()))?
                    .parse::<Uuid>()
                    .map_err(|e| SynapseError::Matter(format!("Failed to parse UUID: {}", e)))?;

                let repository_id = row[1]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid repository_id type".into()))?
                    .to_string();

                let file_path = PathBuf::from(
                    row[2]
                        .get_str()
                        .ok_or_else(|| SynapseError::Matter("Invalid file_path type".into()))?,
                );

                let matter_type_str = row[3]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid matter_type type".into()))?;
                let matter_type: MatterType =
                    serde_json::from_str(&format!("\"{}\"", matter_type_str)).map_err(|e| {
                        SynapseError::Matter(format!("Failed to parse matter type: {}", e))
                    })?;

                let title = row[4]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid title type".into()))?
                    .to_string();

                let context_type = row[5]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid context_type type".into()))?
                    .to_string();

                let context_id = row[6]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid context_id type".into()))?
                    .to_string();

                let visibility = row[7]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid visibility type".into()))?
                    .to_string();

                let tags_str = row[8]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid tags type".into()))?;
                let tags: Vec<String> = serde_json::from_str(tags_str)
                    .map_err(|e| SynapseError::Matter(format!("Failed to parse tags: {}", e)))?;

                let created_at_str = row[9]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid created_at type".into()))?;
                let created_at = chrono::DateTime::parse_from_rfc3339(created_at_str)
                    .map_err(|e| {
                        SynapseError::Matter(format!("Failed to parse created_at: {}", e))
                    })?
                    .with_timezone(&chrono::Utc);

                let updated_at_str = row[10]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid updated_at type".into()))?;
                let updated_at = chrono::DateTime::parse_from_rfc3339(updated_at_str)
                    .map_err(|e| {
                        SynapseError::Matter(format!("Failed to parse updated_at: {}", e))
                    })?
                    .with_timezone(&chrono::Utc);

                let created_by = row[11]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid created_by type".into()))?
                    .to_string();

                let content_hash = row[12]
                    .get_str()
                    .ok_or_else(|| SynapseError::Matter("Invalid content_hash type".into()))?
                    .to_string();

                Ok(MatterSearchResult {
                    id,
                    repository_id,
                    file_path,
                    matter_type,
                    title,
                    context_type,
                    context_id,
                    visibility,
                    tags,
                    created_at,
                    updated_at,
                    created_by,
                    content_hash,
                })
            })
            .collect()
    }
}

/// Search result for matter queries
#[derive(Debug, Clone)]
pub struct MatterSearchResult {
    pub id: Uuid,
    pub repository_id: String,
    pub file_path: PathBuf,
    pub matter_type: MatterType,
    pub title: String,
    pub context_type: String,
    pub context_id: String,
    pub visibility: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub content_hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_matter_index_creation() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let index = MatterIndex::new(db_path.to_str().unwrap());
        assert!(index.is_ok());
    }

    #[test]
    fn test_content_hash() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let index = MatterIndex::new(db_path.to_str().unwrap()).unwrap();

        let hash1 = index.calculate_content_hash("test content");
        let hash2 = index.calculate_content_hash("test content");
        let hash3 = index.calculate_content_hash("different content");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
