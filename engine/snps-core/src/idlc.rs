//! IDLC (Idea Development Lifecycle) Module
//!
//! Provides configurable workflow management for guiding ideas
//! from inception to implementation.

use crate::{Result, SynapseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// IDLC Transition rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: String,
    pub to: Vec<String>,
}

/// IDLC Configuration for a team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlcConfig {
    pub team_id: String,
    pub team_name: String,
    pub stages: Vec<Stage>,
    pub statuses: Vec<Status>,
    pub transitions: Vec<Transition>,
}

impl IdlcConfig {
    /// Load default IDLC configuration
    pub fn default_config() -> Self {
        Self {
            team_id: "default".into(),
            team_name: "Default Team".into(),
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
                },
                Transition {
                    from: "backlog".into(),
                    to: vec!["todo".into(), "canceled".into()],
                },
                Transition {
                    from: "todo".into(),
                    to: vec!["in-dev".into(), "canceled".into()],
                },
                Transition {
                    from: "in-dev".into(),
                    to: vec!["done".into(), "todo".into(), "canceled".into()],
                },
            ],
        }
    }

    /// Check if a transition is valid
    pub fn is_valid_transition(&self, from: &str, to: &str) -> bool {
        self.transitions
            .iter()
            .find(|t| t.from == from)
            .map(|t| t.to.contains(&to.to_string()))
            .unwrap_or(false)
    }

    /// Get available transitions from a status
    pub fn get_available_transitions(&self, from: &str) -> Vec<String> {
        self.transitions
            .iter()
            .find(|t| t.from == from)
            .map(|t| t.to.clone())
            .unwrap_or_default()
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
}
