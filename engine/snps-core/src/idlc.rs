//! IDLC (Idea Development Lifecycle) Module
//!
//! Provides configurable workflow management for guiding ideas
//! from inception to implementation.

pub mod templates;

use crate::{Result, SynapseError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use uuid::Uuid;

/// IDLC Stage definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub terminal: bool,
}

/// IDLC Status within a stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub stage_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

/// IDLC Transition rule with wildcard support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: String, // Can be "*" for wildcard
    pub to: Vec<String>,
    #[serde(default)]
    pub except: Vec<String>, // States excluded from wildcard
}

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// IDLC Configuration for a team (YAML format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcConfig {
    #[serde(default = "default_version")]
    pub version: String,
    pub team: TeamInfo,
    #[serde(default)]
    pub extends: Option<String>, // Base template to extend
    pub stages: Vec<Stage>,
    pub statuses: Vec<Status>,
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub automation: Vec<AutomationRule>,
    #[serde(default)]
    pub complexity: Option<ComplexityConfig>,
    #[serde(default)]
    pub integrations: Option<IntegrationsConfig>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Automation rule (structure only, no runtime execution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub trigger: String,
    #[serde(default)]
    pub from: Option<Vec<String>>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub conditions: Vec<serde_json::Value>,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
}

/// Complexity configuration (structure only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityConfig {
    pub levels: Vec<ComplexityLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityLevel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub points: Option<String>,
    #[serde(default)]
    pub skip_stages: Vec<String>,
    #[serde(default)]
    pub required_stages: Vec<String>,
}

/// Integrations configuration (structure only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    #[serde(default)]
    pub linear: Option<LinearIntegration>,
    #[serde(default)]
    pub github: Option<GithubIntegration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIntegration {
    pub enabled: bool,
    #[serde(default)]
    pub team_id: Option<String>,
    #[serde(default)]
    pub status_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubIntegration {
    pub enabled: bool,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub label_prefix: Option<String>,
}

impl IdlcConfig {
    /// Load default IDLC configuration
    pub fn default_config() -> Self {
        Self {
            version: "1.0".into(),
            team: TeamInfo {
                id: "default".into(),
                name: "Default Team".into(),
                description: None,
            },
            extends: None,
            stages: vec![
                Stage {
                    id: "triage".into(),
                    name: "Triage".into(),
                    description: Some("Initial intake and categorization".into()),
                    required: true,
                    terminal: false,
                },
                Stage {
                    id: "backlog".into(),
                    name: "Backlog".into(),
                    description: Some("Prioritized queue".into()),
                    required: true,
                    terminal: false,
                },
                Stage {
                    id: "unstarted".into(),
                    name: "Unstarted".into(),
                    description: Some("Research and planning".into()),
                    required: true,
                    terminal: false,
                },
                Stage {
                    id: "started".into(),
                    name: "Started".into(),
                    description: Some("Active development".into()),
                    required: true,
                    terminal: false,
                },
                Stage {
                    id: "completed".into(),
                    name: "Completed".into(),
                    description: Some("Successfully finished".into()),
                    required: true,
                    terminal: true,
                },
                Stage {
                    id: "canceled".into(),
                    name: "Canceled".into(),
                    description: Some("Work stopped".into()),
                    required: false,
                    terminal: true,
                },
            ],
            statuses: vec![
                Status {
                    id: "triage".into(),
                    stage_id: "triage".into(),
                    name: "Triage".into(),
                    description: None,
                    color: Some("#6B7280".into()),
                },
                Status {
                    id: "backlog".into(),
                    stage_id: "backlog".into(),
                    name: "Backlog".into(),
                    description: None,
                    color: Some("#9CA3AF".into()),
                },
                Status {
                    id: "todo".into(),
                    stage_id: "unstarted".into(),
                    name: "Todo".into(),
                    description: None,
                    color: Some("#3B82F6".into()),
                },
                Status {
                    id: "in-dev".into(),
                    stage_id: "started".into(),
                    name: "In Development".into(),
                    description: None,
                    color: Some("#8B5CF6".into()),
                },
                Status {
                    id: "done".into(),
                    stage_id: "completed".into(),
                    name: "Done".into(),
                    description: None,
                    color: Some("#22C55E".into()),
                },
                Status {
                    id: "canceled".into(),
                    stage_id: "canceled".into(),
                    name: "Canceled".into(),
                    description: None,
                    color: Some("#EF4444".into()),
                },
            ],
            transitions: vec![
                Transition {
                    from: "triage".into(),
                    to: vec!["backlog".into(), "canceled".into()],
                    except: vec![],
                },
                Transition {
                    from: "backlog".into(),
                    to: vec!["todo".into(), "canceled".into()],
                    except: vec![],
                },
                Transition {
                    from: "todo".into(),
                    to: vec!["in-dev".into(), "canceled".into()],
                    except: vec![],
                },
                Transition {
                    from: "in-dev".into(),
                    to: vec!["done".into(), "todo".into(), "canceled".into()],
                    except: vec![],
                },
            ],
            automation: vec![],
            complexity: None,
            integrations: None,
        }
    }

    /// Load IDLC config from YAML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SynapseError::Idlc(format!("Failed to read {}: {}", path.display(), e)))?;
        Self::from_yaml(&content)
    }

    /// Parse IDLC config from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let config: IdlcConfig = serde_yaml::from_str(yaml)
            .map_err(|e| SynapseError::Idlc(format!("Invalid YAML: {}", e)))?;
        config.validate()?;
        Ok(config)
    }

    /// Save IDLC config to YAML file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| SynapseError::Idlc(format!("Failed to serialize: {}", e)))?;
        std::fs::write(path, yaml).map_err(|e| {
            SynapseError::Idlc(format!("Failed to write {}: {}", path.display(), e))
        })?;
        Ok(())
    }

    /// Validate configuration integrity
    pub fn validate(&self) -> Result<()> {
        // Check all status stage_ids reference valid stages
        let stage_ids: HashSet<_> = self.stages.iter().map(|s| &s.id).collect();
        for status in &self.statuses {
            if !stage_ids.contains(&status.stage_id) {
                return Err(SynapseError::Idlc(format!(
                    "Status '{}' references unknown stage '{}'",
                    status.id, status.stage_id
                )));
            }
        }

        // Check all transition targets exist
        let status_ids: HashSet<_> = self.statuses.iter().map(|s| &s.id).collect();
        for transition in &self.transitions {
            if transition.from != "*" && !status_ids.contains(&transition.from) {
                return Err(SynapseError::Idlc(format!(
                    "Transition from unknown status '{}'",
                    transition.from
                )));
            }
            for to in &transition.to {
                if !status_ids.contains(to) {
                    return Err(SynapseError::Idlc(format!(
                        "Transition to unknown status '{}'",
                        to
                    )));
                }
            }
            // Validate except list references valid statuses
            for except in &transition.except {
                if !status_ids.contains(except) {
                    return Err(SynapseError::Idlc(format!(
                        "Transition except clause references unknown status '{}'",
                        except
                    )));
                }
            }
        }

        // Check for duplicate status IDs
        let mut seen = HashSet::new();
        for status in &self.statuses {
            if !seen.insert(&status.id) {
                return Err(SynapseError::Idlc(format!(
                    "Duplicate status ID '{}'",
                    status.id
                )));
            }
        }

        Ok(())
    }

    /// Check if a transition is valid (with wildcard support)
    pub fn is_valid_transition(&self, from: &str, to: &str) -> bool {
        // Check explicit transitions first
        for t in &self.transitions {
            if t.from == from && t.to.contains(&to.to_string()) {
                return true;
            }
        }

        // Check wildcard transitions
        for t in &self.transitions {
            if t.from == "*" && t.to.contains(&to.to_string()) {
                // Check if current state is excluded
                if !t.except.contains(&from.to_string()) {
                    return true;
                }
            }
        }

        false
    }

    /// Get available transitions from a status (including wildcards)
    pub fn get_available_transitions(&self, from: &str) -> Vec<String> {
        let mut available = Vec::new();

        for t in &self.transitions {
            if t.from == from || (t.from == "*" && !t.except.contains(&from.to_string())) {
                available.extend(t.to.clone());
            }
        }

        // Deduplicate
        available.sort();
        available.dedup();
        available
    }
}

/// An item being tracked through the IDLC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcItem {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl IdlcItem {
    /// Create a new IDLC item
    pub fn new(title: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: None,
            status: "triage".into(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    /// Transition to a new status
    pub fn transition(&mut self, config: &IdlcConfig, to: &str) -> Result<()> {
        if !config.is_valid_transition(&self.status, to) {
            return Err(SynapseError::Idlc(format!(
                "Invalid transition from {} to {}",
                self.status, to
            )));
        }

        self.status = to.to_string();
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = IdlcConfig::default_config();
        assert_eq!(config.stages.len(), 6);
        assert_eq!(config.statuses.len(), 6);
    }

    #[test]
    fn test_valid_transition() {
        let config = IdlcConfig::default_config();
        assert!(config.is_valid_transition("triage", "backlog"));
        assert!(!config.is_valid_transition("triage", "done"));
    }

    #[test]
    fn test_item_transition() {
        let config = IdlcConfig::default_config();
        let mut item = IdlcItem::new("Test Item");

        assert_eq!(item.status, "triage");
        assert!(item.transition(&config, "backlog").is_ok());
        assert_eq!(item.status, "backlog");
    }

    #[test]
    fn test_yaml_parsing() {
        let yaml = r##"
version: "1.0"
team:
  id: "test-team"
  name: "Test Team"
  description: "A test team"
stages:
  - id: "start"
    name: "Start"
    description: "Starting stage"
    required: true
    terminal: false
  - id: "end"
    name: "End"
    description: "Ending stage"
    required: true
    terminal: true
statuses:
  - id: "new"
    stage_id: "start"
    name: "New"
    color: "#3B82F6"
  - id: "done"
    stage_id: "end"
    name: "Done"
    color: "#22C55E"
transitions:
  - from: "new"
    to: ["done"]
"##;

        let config = IdlcConfig::from_yaml(yaml);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.team.id, "test-team");
        assert_eq!(config.team.name, "Test Team");
        assert_eq!(config.stages.len(), 2);
        assert_eq!(config.statuses.len(), 2);
    }

    #[test]
    fn test_validation_catches_invalid_stage_reference() {
        let yaml = r##"
version: "1.0"
team:
  id: "test-team"
  name: "Test Team"
stages:
  - id: "start"
    name: "Start"
    required: true
    terminal: false
statuses:
  - id: "new"
    stage_id: "nonexistent"
    name: "New"
transitions:
  - from: "new"
    to: ["done"]
"##;

        let result = IdlcConfig::from_yaml(yaml);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("unknown stage"));
    }

    #[test]
    fn test_validation_catches_invalid_transition_target() {
        let yaml = r##"
version: "1.0"
team:
  id: "test-team"
  name: "Test Team"
stages:
  - id: "start"
    name: "Start"
    required: true
    terminal: false
statuses:
  - id: "new"
    stage_id: "start"
    name: "New"
transitions:
  - from: "new"
    to: ["nonexistent"]
"##;

        let result = IdlcConfig::from_yaml(yaml);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("unknown status"));
    }

    #[test]
    fn test_wildcard_transitions() {
        let yaml = r##"
version: "1.0"
team:
  id: "test-team"
  name: "Test Team"
stages:
  - id: "active"
    name: "Active"
    required: true
    terminal: false
  - id: "done"
    name: "Done"
    required: true
    terminal: true
statuses:
  - id: "new"
    stage_id: "active"
    name: "New"
  - id: "in-progress"
    stage_id: "active"
    name: "In Progress"
  - id: "done"
    stage_id: "done"
    name: "Done"
  - id: "canceled"
    stage_id: "done"
    name: "Canceled"
transitions:
  - from: "new"
    to: ["in-progress"]
  - from: "in-progress"
    to: ["done"]
  - from: "*"
    to: ["canceled"]
    except: ["done", "canceled"]
"##;

        let config = IdlcConfig::from_yaml(yaml).unwrap();

        // Test wildcard transition from "new" to "canceled"
        assert!(config.is_valid_transition("new", "canceled"));

        // Test wildcard transition from "in-progress" to "canceled"
        assert!(config.is_valid_transition("in-progress", "canceled"));

        // Test that "done" cannot transition to "canceled" (in except list)
        assert!(!config.is_valid_transition("done", "canceled"));

        // Test that "canceled" cannot transition to itself (in except list)
        assert!(!config.is_valid_transition("canceled", "canceled"));

        // Test available transitions include wildcards
        let from_new = config.get_available_transitions("new");
        assert!(from_new.contains(&"in-progress".to_string()));
        assert!(from_new.contains(&"canceled".to_string()));
    }

    #[test]
    fn test_wildcard_except_validation() {
        let yaml = r##"
version: "1.0"
team:
  id: "test-team"
  name: "Test Team"
stages:
  - id: "active"
    name: "Active"
    required: true
    terminal: false
statuses:
  - id: "new"
    stage_id: "active"
    name: "New"
transitions:
  - from: "*"
    to: ["new"]
    except: ["nonexistent"]
"##;

        let result = IdlcConfig::from_yaml(yaml);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("except clause references unknown status"));
    }

    #[test]
    fn test_template_loading() {
        // Test that default template can be loaded
        let default_template = super::templates::get_template("default");
        assert!(default_template.is_some());

        // Test unknown template
        let unknown = super::templates::get_template("unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_default_template_parsing() {
        let template = super::templates::get_template("default").unwrap();

        // Replace placeholders
        let yaml = template
            .replace("{{team_id}}", "test-team")
            .replace("{{team_name}}", "Test Team");

        // Parse template
        let config = IdlcConfig::from_yaml(&yaml);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.team.id, "test-team");
        assert_eq!(config.team.name, "Test Team");
        assert!(!config.stages.is_empty());
        assert!(!config.statuses.is_empty());
        assert!(!config.transitions.is_empty());
    }

}
