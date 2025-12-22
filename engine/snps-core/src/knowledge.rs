use crate::SynapseError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Context type for knowledge scoping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeContext {
    User,
    Team,
    Project,
}

/// Shadow repository configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowRepository {
    pub path: PathBuf,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type", default = "default_repo_type")]
    pub repo_type: String,
    pub context: KnowledgeContext,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_repo_type() -> String {
    "folder".to_string()
}
fn default_enabled() -> bool {
    true
}

/// Git exclude pattern configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitExcludeConfig {
    #[serde(default)]
    pub patterns: Vec<String>,
}

/// Knowledge repositories configuration
/// Stored in `.pmsynapse/repositories.yaml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConfig {
    pub version: String,
    #[serde(default)]
    pub git_exclude_patterns: Vec<String>,
    pub repositories: Vec<ShadowRepository>,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            git_exclude_patterns: vec![r"^\.pmsynapse/".to_string(), r"^knowledge/".to_string()],
            repositories: Vec::new(),
        }
    }
}

/// Load knowledge config from project's .pmsynapse/repositories.yaml
pub fn load_knowledge_config(project_root: &Path) -> Result<KnowledgeConfig, SynapseError> {
    let config_path = project_root.join(".pmsynapse/repositories.yaml");

    if !config_path.exists() {
        return Err(SynapseError::Config(
            "Knowledge not initialized. Run 'snps know init' first.".to_string(),
        ));
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: KnowledgeConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// Save knowledge config to project's .pmsynapse/repositories.yaml
pub fn save_knowledge_config(
    project_root: &Path,
    config: &KnowledgeConfig,
) -> Result<(), SynapseError> {
    let config_dir = project_root.join(".pmsynapse");
    std::fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join("repositories.yaml");
    let yaml = serde_yaml::to_string(config)?;
    std::fs::write(&config_path, yaml)?;
    Ok(())
}

/// Get repositories sorted by precedence (user first, project last)
pub fn get_repos_by_precedence(config: &KnowledgeConfig) -> Vec<&ShadowRepository> {
    let mut repos: Vec<_> = config.repositories.iter().filter(|r| r.enabled).collect();

    repos.sort_by_key(|r| match r.context {
        KnowledgeContext::User => 0,
        KnowledgeContext::Team => 1,
        KnowledgeContext::Project => 2,
    });

    repos
}

/// Generate unique ID for repository if not provided
pub fn generate_repo_id(context: &KnowledgeContext, path: &Path) -> String {
    let context_str = match context {
        KnowledgeContext::User => "user",
        KnowledgeContext::Team => "team",
        KnowledgeContext::Project => "project",
    };
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".to_string());
    format!("{}-{}", context_str, name)
}

// Phase 2: Smart Sync

use std::collections::HashMap;
use std::time::SystemTime;

/// File sync entry with metadata
#[derive(Debug, Clone)]
pub struct SyncEntry {
    pub relative_path: PathBuf,
    pub source_path: PathBuf,
    pub source_repo_id: String,
    pub context: KnowledgeContext,
    pub hash: String,
    pub modified: SystemTime,
}

/// Sync operation type
#[derive(Debug)]
pub enum SyncOperation {
    Copy {
        from: PathBuf,
        to: PathBuf,
        repo_id: String,
    },
    Override {
        from: PathBuf,
        to: PathBuf,
        repo_id: String,
        overridden_repo: String,
    },
    Skip {
        path: PathBuf,
        reason: String,
    },
    Push {
        from: PathBuf,
        to: PathBuf,
    },
}

/// Scan shadow repository for files
pub fn scan_shadow_repo(repo: &ShadowRepository) -> Result<Vec<SyncEntry>, SynapseError> {
    let mut entries = Vec::new();
    let repo_id = repo.id.clone().unwrap_or_else(|| "unknown".to_string());

    // Scan entire repository for all files
    if repo.path.exists() {
        scan_directory(
            &repo.path,
            &repo.path,
            &repo_id,
            &repo.context,
            &mut entries,
        )?;
    }

    Ok(entries)
}

fn scan_directory(
    dir: &Path,
    base: &Path,
    repo_id: &str,
    context: &KnowledgeContext,
    entries: &mut Vec<SyncEntry>,
) -> Result<(), SynapseError> {
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative = path
            .strip_prefix(base)
            .map_err(|e| SynapseError::Knowledge(e.to_string()))?;

        let metadata = std::fs::metadata(path)?;
        let modified = metadata.modified()?;
        let hash = compute_file_hash(path)?;

        entries.push(SyncEntry {
            relative_path: relative.to_path_buf(),
            source_path: path.to_path_buf(),
            source_repo_id: repo_id.to_string(),
            context: context.clone(),
            hash,
            modified,
        });
    }
    Ok(())
}

/// Compute file hash for change detection
pub fn compute_file_hash(path: &Path) -> Result<String, SynapseError> {
    use sha2::{Digest, Sha256};
    let content = std::fs::read(path)?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}

/// Build sync plan with precedence resolution
pub fn build_sync_plan(
    config: &KnowledgeConfig,
    project_root: &Path,
    force: bool,
) -> Result<Vec<SyncOperation>, SynapseError> {
    let mut operations = Vec::new();
    let mut file_sources: HashMap<PathBuf, SyncEntry> = HashMap::new();

    // Get repos in precedence order (user → team → project)
    let repos = get_repos_by_precedence(config);

    // Scan all repos, later repos override earlier ones
    for repo in repos {
        let entries = scan_shadow_repo(repo)?;

        for entry in entries {
            if let Some(existing) = file_sources.get(&entry.relative_path) {
                // Higher precedence repo overrides
                operations.push(SyncOperation::Override {
                    from: entry.source_path.clone(),
                    to: project_root.join(&entry.relative_path),
                    repo_id: entry.source_repo_id.clone(),
                    overridden_repo: existing.source_repo_id.clone(),
                });
            }
            file_sources.insert(entry.relative_path.clone(), entry);
        }
    }

    // Generate copy operations for winning entries
    for (relative_path, entry) in &file_sources {
        let dest = project_root.join(relative_path);

        // Check if destination exists and compare
        if dest.exists() && !force {
            let dest_hash = compute_file_hash(&dest)?;
            if dest_hash == entry.hash {
                operations.push(SyncOperation::Skip {
                    path: dest.clone(),
                    reason: "unchanged".to_string(),
                });
                continue;
            }
        }

        operations.push(SyncOperation::Copy {
            from: entry.source_path.clone(),
            to: dest,
            repo_id: entry.source_repo_id.clone(),
        });
    }

    Ok(operations)
}

// Phase 3: Git Integration

/// Update .git/info/exclude with synced paths
pub fn update_git_exclude(
    project_root: &Path,
    config: &KnowledgeConfig,
    synced_paths: &[PathBuf],
) -> Result<(), SynapseError> {
    let exclude_path = project_root.join(".git/info/exclude");

    // Check if .git exists
    if !project_root.join(".git").exists() {
        return Ok(()); // Not a git repo, skip
    }

    // Compile exclusion patterns
    let exclude_patterns: Vec<regex::Regex> = config
        .git_exclude_patterns
        .iter()
        .filter_map(|p| regex::Regex::new(p).ok())
        .collect();

    // Determine which paths should be excluded
    let mut paths_to_exclude: Vec<String> = Vec::new();

    for path in synced_paths {
        let path_str = path.to_string_lossy().to_string();

        // Check if any exclusion pattern matches (skip if matches)
        let should_skip = exclude_patterns.iter().any(|re| re.is_match(&path_str));

        if !should_skip {
            // Add the path itself (git will handle both files and directories)
            if !paths_to_exclude.contains(&path_str) {
                paths_to_exclude.push(path_str);
            }
        }
    }

    // Always exclude .pmsynapse/ and knowledge/
    if !paths_to_exclude.contains(&".pmsynapse/".to_string()) {
        paths_to_exclude.push(".pmsynapse/".to_string());
    }
    if !paths_to_exclude.contains(&"knowledge/".to_string()) {
        paths_to_exclude.push("knowledge/".to_string());
    }

    // Read existing exclude file
    let existing_content = if exclude_path.exists() {
        std::fs::read_to_string(&exclude_path)?
    } else {
        String::new()
    };

    // Find and preserve user section
    const MARKER_START: &str = "# BEGIN snps-know auto-generated";
    const MARKER_END: &str = "# END snps-know auto-generated";

    let user_section = if let Some(start_idx) = existing_content.find(MARKER_START) {
        if let Some(end_idx) = existing_content.find(MARKER_END) {
            let before = &existing_content[..start_idx];
            let after = &existing_content[end_idx + MARKER_END.len()..];
            format!("{}{}", before.trim_end(), after.trim_start())
        } else {
            existing_content.clone()
        }
    } else {
        existing_content.clone()
    };

    // Build new content
    let auto_section = format!(
        "\n{}\n# DO NOT EDIT - Changes will be overwritten by 'snps know sync'\n{}\n{}\n",
        MARKER_START,
        paths_to_exclude.join("\n"),
        MARKER_END
    );

    let new_content = if user_section.is_empty() {
        auto_section
    } else {
        format!("{}\n{}", user_section.trim_end(), auto_section)
    };

    // Write exclude file
    if let Some(parent) = exclude_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&exclude_path, new_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_knowledge_context_serialization() {
        let context = KnowledgeContext::Team;
        let json = serde_json::to_string(&context).unwrap();
        assert_eq!(json, r#""team""#);
    }

    #[test]
    fn test_default_config() {
        let config = KnowledgeConfig::default();
        assert_eq!(config.version, "1.0");
        assert!(config.repositories.is_empty());
        assert!(!config.git_exclude_patterns.is_empty());
    }

    #[test]
    fn test_generate_repo_id() {
        let path = PathBuf::from("/Users/test/my-knowledge");
        let id = generate_repo_id(&KnowledgeContext::User, &path);
        assert_eq!(id, "user-my-knowledge");
    }

    #[test]
    fn test_get_repos_by_precedence() {
        let config = KnowledgeConfig {
            version: "1.0".to_string(),
            git_exclude_patterns: vec![],
            repositories: vec![
                ShadowRepository {
                    path: PathBuf::from("/project"),
                    id: Some("project-main".to_string()),
                    description: None,
                    repo_type: "folder".to_string(),
                    context: KnowledgeContext::Project,
                    enabled: true,
                },
                ShadowRepository {
                    path: PathBuf::from("/user"),
                    id: Some("user-personal".to_string()),
                    description: None,
                    repo_type: "folder".to_string(),
                    context: KnowledgeContext::User,
                    enabled: true,
                },
            ],
        };

        let repos = get_repos_by_precedence(&config);
        assert_eq!(repos.len(), 2);
        assert!(matches!(repos[0].context, KnowledgeContext::User));
        assert!(matches!(repos[1].context, KnowledgeContext::Project));
    }

    #[test]
    fn test_compute_file_hash() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("test.txt");
        std::fs::write(&file, "hello world").unwrap();

        let hash = compute_file_hash(&file).unwrap();
        assert!(!hash.is_empty());

        // Same content = same hash
        let file2 = dir.path().join("test2.txt");
        std::fs::write(&file2, "hello world").unwrap();
        let hash2 = compute_file_hash(&file2).unwrap();
        assert_eq!(hash, hash2);
    }
}
