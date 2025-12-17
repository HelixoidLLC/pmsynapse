use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContextType {
    User,
    Team,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Shared,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMapping {
    pub id: String,
    pub path: PathBuf,
    pub context_type: ContextType,
    pub context_id: String,
    pub visibility: Visibility,
    pub role: Option<String>,
    pub auto_index: bool,
    pub sync: RepositorySyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySyncConfig {
    pub enabled: bool,
    pub remote: Option<String>,
    pub branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoriesConfig {
    pub repositories: Vec<RepositoryMapping>,
}

impl Default for RepositorySyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            remote: None,
            branch: "main".to_string(),
        }
    }
}

pub fn load_repositories() -> Result<RepositoriesConfig, crate::SynapseError> {
    let repo_path = crate::config::get_pmsynapse_global_dir().join("repositories.yaml");

    if !repo_path.exists() {
        return Ok(RepositoriesConfig {
            repositories: Vec::new(),
        });
    }

    let content = std::fs::read_to_string(&repo_path).map_err(|e| {
        crate::SynapseError::Config(format!("Failed to read repositories config: {}", e))
    })?;

    let config: RepositoriesConfig = serde_yaml::from_str(&content).map_err(|e| {
        crate::SynapseError::Config(format!("Failed to parse repositories config: {}", e))
    })?;

    Ok(config)
}

pub fn save_repositories(config: &RepositoriesConfig) -> Result<(), crate::SynapseError> {
    let config_dir = crate::config::get_pmsynapse_global_dir();
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to create config dir: {}", e)))?;

    let repo_path = config_dir.join("repositories.yaml");
    let yaml = serde_yaml::to_string(config).map_err(|e| {
        crate::SynapseError::Config(format!("Failed to serialize repositories: {}", e))
    })?;

    std::fs::write(&repo_path, yaml)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to write repositories: {}", e)))?;

    Ok(())
}

pub fn add_repository(mapping: RepositoryMapping) -> Result<(), crate::SynapseError> {
    let mut config = load_repositories()?;

    // Check if repository with same ID already exists
    if config.repositories.iter().any(|r| r.id == mapping.id) {
        return Err(crate::SynapseError::Config(format!(
            "Repository with ID '{}' already exists",
            mapping.id
        )));
    }

    config.repositories.push(mapping);
    save_repositories(&config)?;

    Ok(())
}

pub fn remove_repository(id: &str) -> Result<(), crate::SynapseError> {
    let mut config = load_repositories()?;

    let initial_len = config.repositories.len();
    config.repositories.retain(|r| r.id != id);

    if config.repositories.len() == initial_len {
        return Err(crate::SynapseError::Config(format!(
            "Repository with ID '{}' not found",
            id
        )));
    }

    save_repositories(&config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_type_serialization() {
        let context = ContextType::Team;
        let json = serde_json::to_string(&context).unwrap();
        assert_eq!(json, r#""team""#);

        let deserialized: ContextType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ContextType::Team);
    }

    #[test]
    fn test_visibility_serialization() {
        let vis = Visibility::Shared;
        let json = serde_json::to_string(&vis).unwrap();
        assert_eq!(json, r#""shared""#);
    }

    #[test]
    fn test_default_sync_config() {
        let sync = RepositorySyncConfig::default();
        assert!(!sync.enabled);
        assert_eq!(sync.branch, "main");
        assert!(sync.remote.is_none());
    }
}
