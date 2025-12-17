use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub version: String,
    pub repositories_root: PathBuf,
    pub defaults: Defaults,
    pub user: UserConfig,
    pub search: SearchConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    pub editor: String,
    pub matter_type: String,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub index_enabled: bool,
    pub index_db: PathBuf,
    pub watch_for_changes: bool,
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub auto_sync: bool,
    pub sync_interval_minutes: u32,
    pub remote_portal_url: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            version: "1.0".into(),
            repositories_root: home.join("repos"),
            defaults: Defaults {
                editor: "code".into(),
                matter_type: "document".into(),
                visibility: "private".into(),
            },
            user: UserConfig {
                id: whoami::username(),
                name: whoami::realname(),
                email: format!("{}@example.com", whoami::username()),
            },
            search: SearchConfig {
                index_enabled: true,
                index_db: home.join(".pmsynapse/index.db"),
                watch_for_changes: true,
                exclude_patterns: vec!["node_modules/".into(), ".git/".into(), "*.lock".into()],
            },
            sync: SyncConfig {
                auto_sync: false,
                sync_interval_minutes: 15,
                remote_portal_url: None,
            },
        }
    }
}

pub fn load_global_config() -> Result<GlobalConfig, crate::SynapseError> {
    let config_path = get_pmsynapse_global_dir().join("config.yaml");

    if !config_path.exists() {
        return Ok(GlobalConfig::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to read config: {}", e)))?;

    let config: GlobalConfig = serde_yaml::from_str(&content)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to parse config: {}", e)))?;

    Ok(config)
}

pub fn save_global_config(config: &GlobalConfig) -> Result<(), crate::SynapseError> {
    let config_dir = get_pmsynapse_global_dir();
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to create config dir: {}", e)))?;

    let config_path = config_dir.join("config.yaml");
    let yaml = serde_yaml::to_string(config)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to serialize config: {}", e)))?;

    std::fs::write(&config_path, yaml)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to write config: {}", e)))?;

    Ok(())
}

pub fn get_pmsynapse_global_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(".pmsynapse")
}

/// Merged configuration with source tracking
#[derive(Debug, Clone)]
pub struct MergedConfig {
    pub config: GlobalConfig,
    pub sources: HashMap<String, ConfigSource>,
}

/// Source of a configuration setting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    Personal,
    Team(String),
    Project(String),
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigSource::Default => write!(f, "default"),
            ConfigSource::Personal => write!(f, "personal"),
            ConfigSource::Team(id) => write!(f, "team:{}", id),
            ConfigSource::Project(id) => write!(f, "project:{}", id),
        }
    }
}

/// Load and merge configs for a project context
pub fn load_merged_config(
    team_id: Option<&str>,
    project_id: Option<&str>,
) -> Result<MergedConfig, crate::SynapseError> {
    let mut config = GlobalConfig::default();
    let mut sources = HashMap::new();

    // Track default values
    track_sources_for_default(&mut sources);

    // Load personal config
    let personal_path = get_pmsynapse_global_dir().join("config.yaml");
    if personal_path.exists() {
        let personal = load_yaml_config(&personal_path)?;
        merge_config(&mut config, &personal, &mut sources, ConfigSource::Personal);
    }

    // Load team config if specified
    if let Some(team) = team_id {
        let team_path = get_pmsynapse_global_dir()
            .join("teams")
            .join(team)
            .join("config.yaml");
        if team_path.exists() {
            let team_cfg = load_yaml_config(&team_path)?;
            merge_config(
                &mut config,
                &team_cfg,
                &mut sources,
                ConfigSource::Team(team.into()),
            );
        }
    }

    // Load project config if specified
    if let Some(project) = project_id {
        let project_path = get_pmsynapse_global_dir()
            .join("teams")
            .join(team_id.unwrap_or("default"))
            .join("projects")
            .join(project)
            .join("config.yaml");
        if project_path.exists() {
            let project_cfg = load_yaml_config(&project_path)?;
            merge_config(
                &mut config,
                &project_cfg,
                &mut sources,
                ConfigSource::Project(project.into()),
            );
        }
    }

    Ok(MergedConfig { config, sources })
}

fn load_yaml_config(path: &PathBuf) -> Result<GlobalConfig, crate::SynapseError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to read config: {}", e)))?;

    let config: GlobalConfig = serde_yaml::from_str(&content)
        .map_err(|e| crate::SynapseError::Config(format!("Failed to parse config: {}", e)))?;

    Ok(config)
}

fn track_sources_for_default(sources: &mut HashMap<String, ConfigSource>) {
    sources.insert("version".to_string(), ConfigSource::Default);
    sources.insert("repositories_root".to_string(), ConfigSource::Default);
    sources.insert("defaults.editor".to_string(), ConfigSource::Default);
    sources.insert("defaults.matter_type".to_string(), ConfigSource::Default);
    sources.insert("defaults.visibility".to_string(), ConfigSource::Default);
    sources.insert("user.id".to_string(), ConfigSource::Default);
    sources.insert("user.name".to_string(), ConfigSource::Default);
    sources.insert("user.email".to_string(), ConfigSource::Default);
    sources.insert("search.index_enabled".to_string(), ConfigSource::Default);
    sources.insert("search.index_db".to_string(), ConfigSource::Default);
    sources.insert(
        "search.watch_for_changes".to_string(),
        ConfigSource::Default,
    );
    sources.insert("search.exclude_patterns".to_string(), ConfigSource::Default);
    sources.insert("sync.auto_sync".to_string(), ConfigSource::Default);
    sources.insert(
        "sync.sync_interval_minutes".to_string(),
        ConfigSource::Default,
    );
    sources.insert("sync.remote_portal_url".to_string(), ConfigSource::Default);
}

fn merge_config(
    target: &mut GlobalConfig,
    source: &GlobalConfig,
    sources: &mut HashMap<String, ConfigSource>,
    source_type: ConfigSource,
) {
    // Merge version if different from default
    if source.version != "1.0" {
        target.version = source.version.clone();
        sources.insert("version".to_string(), source_type.clone());
    }

    // Merge repositories_root
    let default_repos_root = dirs::home_dir().unwrap_or_default().join("repos");
    if source.repositories_root != default_repos_root {
        target.repositories_root = source.repositories_root.clone();
        sources.insert("repositories_root".to_string(), source_type.clone());
    }

    // Merge defaults
    if source.defaults.editor != "code" {
        target.defaults.editor = source.defaults.editor.clone();
        sources.insert("defaults.editor".to_string(), source_type.clone());
    }
    if source.defaults.matter_type != "document" {
        target.defaults.matter_type = source.defaults.matter_type.clone();
        sources.insert("defaults.matter_type".to_string(), source_type.clone());
    }
    if source.defaults.visibility != "private" {
        target.defaults.visibility = source.defaults.visibility.clone();
        sources.insert("defaults.visibility".to_string(), source_type.clone());
    }

    // Merge user config
    let default_user = whoami::username();
    if source.user.id != default_user {
        target.user.id = source.user.id.clone();
        sources.insert("user.id".to_string(), source_type.clone());
    }
    let default_name = whoami::realname();
    if source.user.name != default_name {
        target.user.name = source.user.name.clone();
        sources.insert("user.name".to_string(), source_type.clone());
    }
    let default_email = format!("{}@example.com", whoami::username());
    if source.user.email != default_email {
        target.user.email = source.user.email.clone();
        sources.insert("user.email".to_string(), source_type.clone());
    }

    // Merge search config
    if !source.search.index_enabled {
        target.search.index_enabled = source.search.index_enabled;
        sources.insert("search.index_enabled".to_string(), source_type.clone());
    }
    let default_index_db = dirs::home_dir()
        .unwrap_or_default()
        .join(".pmsynapse/index.db");
    if source.search.index_db != default_index_db {
        target.search.index_db = source.search.index_db.clone();
        sources.insert("search.index_db".to_string(), source_type.clone());
    }
    if !source.search.watch_for_changes {
        target.search.watch_for_changes = source.search.watch_for_changes;
        sources.insert("search.watch_for_changes".to_string(), source_type.clone());
    }
    let default_exclude = vec![
        "node_modules/".to_string(),
        ".git/".to_string(),
        "*.lock".to_string(),
    ];
    if source.search.exclude_patterns != default_exclude {
        target.search.exclude_patterns = source.search.exclude_patterns.clone();
        sources.insert("search.exclude_patterns".to_string(), source_type.clone());
    }

    // Merge sync config
    if source.sync.auto_sync {
        target.sync.auto_sync = source.sync.auto_sync;
        sources.insert("sync.auto_sync".to_string(), source_type.clone());
    }
    if source.sync.sync_interval_minutes != 15 {
        target.sync.sync_interval_minutes = source.sync.sync_interval_minutes;
        sources.insert(
            "sync.sync_interval_minutes".to_string(),
            source_type.clone(),
        );
    }
    if source.sync.remote_portal_url.is_some() {
        target.sync.remote_portal_url = source.sync.remote_portal_url.clone();
        sources.insert("sync.remote_portal_url".to_string(), source_type.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GlobalConfig::default();
        assert_eq!(config.version, "1.0");
        assert_eq!(config.defaults.matter_type, "document");
        assert_eq!(config.defaults.visibility, "private");
        assert!(!config.sync.auto_sync);
        assert!(config.search.index_enabled);
    }

    #[test]
    fn test_get_global_dir() {
        let dir = get_pmsynapse_global_dir();
        assert!(dir.ends_with(".pmsynapse"));
    }

    #[test]
    fn test_config_serialization() {
        let config = GlobalConfig::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: GlobalConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    fn test_config_source_display() {
        assert_eq!(ConfigSource::Default.to_string(), "default");
        assert_eq!(ConfigSource::Personal.to_string(), "personal");
        assert_eq!(
            ConfigSource::Team("eng".to_string()).to_string(),
            "team:eng"
        );
        assert_eq!(
            ConfigSource::Project("web-app".to_string()).to_string(),
            "project:web-app"
        );
    }

    #[test]
    fn test_merged_config_default_only() {
        let merged = load_merged_config(None, None).unwrap();
        assert_eq!(merged.config.version, "1.0");
        assert_eq!(merged.sources.get("version"), Some(&ConfigSource::Default));
        assert_eq!(
            merged.sources.get("defaults.editor"),
            Some(&ConfigSource::Default)
        );
    }

    #[test]
    fn test_merge_config_precedence() {
        let mut target = GlobalConfig::default();
        let mut sources = HashMap::new();
        track_sources_for_default(&mut sources);

        // Create a custom config with different editor
        let mut source = GlobalConfig::default();
        source.defaults.editor = "vim".to_string();
        source.user.name = "Test User".to_string();

        merge_config(&mut target, &source, &mut sources, ConfigSource::Personal);

        assert_eq!(target.defaults.editor, "vim");
        assert_eq!(target.user.name, "Test User");
        assert_eq!(
            sources.get("defaults.editor"),
            Some(&ConfigSource::Personal)
        );
        assert_eq!(sources.get("user.name"), Some(&ConfigSource::Personal));
    }

    #[test]
    fn test_merge_config_team_overrides_personal() {
        let mut target = GlobalConfig::default();
        let mut sources = HashMap::new();
        track_sources_for_default(&mut sources);

        // Personal config sets editor to vim
        let mut personal = GlobalConfig::default();
        personal.defaults.editor = "vim".to_string();
        merge_config(&mut target, &personal, &mut sources, ConfigSource::Personal);

        // Team config sets editor to emacs
        let mut team = GlobalConfig::default();
        team.defaults.editor = "emacs".to_string();
        merge_config(
            &mut target,
            &team,
            &mut sources,
            ConfigSource::Team("eng".to_string()),
        );

        assert_eq!(target.defaults.editor, "emacs");
        assert_eq!(
            sources.get("defaults.editor"),
            Some(&ConfigSource::Team("eng".to_string()))
        );
    }
}
