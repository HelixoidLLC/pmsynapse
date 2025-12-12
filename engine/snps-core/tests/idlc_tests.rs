//! Integration tests for the IDLC (Idea Development Lifecycle) module.
//!
//! Tests workflow configuration, stage transitions, and validation.

use snps_core::idlc::{IdlcConfig, IdlcItem, Stage, Status, Transition};

#[test]
fn test_default_config() {
    let config = IdlcConfig::default_config();

    // Default config should have standard stages and statuses
    assert_eq!(config.team_id, "default");
    assert_eq!(config.team_name, "Default Team");
    assert_eq!(config.stages.len(), 6);
    assert_eq!(config.statuses.len(), 6);
}

#[test]
fn test_default_stages() {
    let config = IdlcConfig::default_config();

    let stage_ids: Vec<_> = config.stages.iter().map(|s| s.id.as_str()).collect();

    // Verify standard stages exist
    assert!(stage_ids.contains(&"triage"));
    assert!(stage_ids.contains(&"backlog"));
    assert!(stage_ids.contains(&"unstarted"));
    assert!(stage_ids.contains(&"started"));
    assert!(stage_ids.contains(&"completed"));
    assert!(stage_ids.contains(&"canceled"));
}

#[test]
fn test_terminal_stages() {
    let config = IdlcConfig::default_config();

    // Find terminal stages
    let terminal: Vec<_> = config.stages.iter().filter(|s| s.terminal).collect();

    // Should have completed and canceled as terminal
    assert_eq!(terminal.len(), 2);

    let terminal_ids: Vec<_> = terminal.iter().map(|s| s.id.as_str()).collect();
    assert!(terminal_ids.contains(&"completed"));
    assert!(terminal_ids.contains(&"canceled"));
}

#[test]
fn test_valid_transition() {
    let config = IdlcConfig::default_config();

    // Test valid transitions
    assert!(config.is_valid_transition("triage", "backlog"));
    assert!(config.is_valid_transition("triage", "canceled"));
    assert!(config.is_valid_transition("backlog", "todo"));
    assert!(config.is_valid_transition("todo", "in-dev"));
    assert!(config.is_valid_transition("in-dev", "done"));
}

#[test]
fn test_invalid_transition() {
    let config = IdlcConfig::default_config();

    // Test invalid transitions
    assert!(!config.is_valid_transition("triage", "done"));
    assert!(!config.is_valid_transition("backlog", "done"));
    assert!(!config.is_valid_transition("done", "triage"));
}

#[test]
fn test_available_transitions() {
    let config = IdlcConfig::default_config();

    let from_triage = config.get_available_transitions("triage");
    assert_eq!(from_triage.len(), 2);
    assert!(from_triage.contains(&"backlog".to_string()));
    assert!(from_triage.contains(&"canceled".to_string()));

    let from_in_dev = config.get_available_transitions("in-dev");
    assert_eq!(from_in_dev.len(), 3);
    assert!(from_in_dev.contains(&"done".to_string()));
    assert!(from_in_dev.contains(&"todo".to_string()));
    assert!(from_in_dev.contains(&"canceled".to_string()));
}

#[test]
fn test_available_transitions_unknown_status() {
    let config = IdlcConfig::default_config();

    // Unknown status should return empty transitions
    let transitions = config.get_available_transitions("unknown");
    assert!(transitions.is_empty());
}

#[test]
fn test_idlc_item_creation() {
    let item = IdlcItem::new("Test Item");

    assert_eq!(item.title, "Test Item");
    assert_eq!(item.status, "triage"); // Default status
    assert!(item.description.is_none());
    assert!(item.metadata.is_empty());
}

#[test]
fn test_idlc_item_transition() {
    let config = IdlcConfig::default_config();
    let mut item = IdlcItem::new("Test Item");

    assert_eq!(item.status, "triage");

    // Valid transition
    assert!(item.transition(&config, "backlog").is_ok());
    assert_eq!(item.status, "backlog");

    // Another valid transition
    assert!(item.transition(&config, "todo").is_ok());
    assert_eq!(item.status, "todo");
}

#[test]
fn test_idlc_item_invalid_transition() {
    let config = IdlcConfig::default_config();
    let mut item = IdlcItem::new("Test Item");

    // Invalid transition - can't go from triage to done
    let result = item.transition(&config, "done");
    assert!(result.is_err());
    assert_eq!(item.status, "triage"); // Status unchanged
}

#[test]
fn test_workflow_serialization() {
    let config = IdlcConfig::default_config();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize");
    assert!(json.contains("team_id"));
    assert!(json.contains("team_name"));
    assert!(json.contains("stages"));
    assert!(json.contains("statuses"));
    assert!(json.contains("transitions"));

    // Deserialize back
    let parsed: IdlcConfig = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(parsed.team_id, config.team_id);
    assert_eq!(parsed.stages.len(), config.stages.len());
}

#[test]
fn test_custom_workflow_creation() {
    let custom_config = IdlcConfig {
        team_id: "custom-team".to_string(),
        team_name: "Custom Team".to_string(),
        stages: vec![
            Stage {
                id: "idea".to_string(),
                name: "Idea".to_string(),
                description: Some("New idea".to_string()),
                required: true,
                terminal: false,
            },
            Stage {
                id: "done".to_string(),
                name: "Done".to_string(),
                description: Some("Completed".to_string()),
                required: true,
                terminal: true,
            },
        ],
        statuses: vec![
            Status {
                id: "new".to_string(),
                stage_id: "idea".to_string(),
                name: "New".to_string(),
                description: Some("Newly created".to_string()),
                color: Some("#3B82F6".to_string()),
            },
            Status {
                id: "finished".to_string(),
                stage_id: "done".to_string(),
                name: "Finished".to_string(),
                description: None,
                color: Some("#22C55E".to_string()),
            },
        ],
        transitions: vec![Transition {
            from: "new".to_string(),
            to: vec!["finished".to_string()],
        }],
    };

    assert_eq!(custom_config.team_id, "custom-team");
    assert_eq!(custom_config.stages.len(), 2);
    assert!(custom_config.is_valid_transition("new", "finished"));
}

#[test]
fn test_stage_properties() {
    let stage = Stage {
        id: "development".to_string(),
        name: "Development".to_string(),
        description: Some("Active development".to_string()),
        required: true,
        terminal: false,
    };

    assert_eq!(stage.id, "development");
    assert_eq!(stage.name, "Development");
    assert!(stage.description.is_some());
    assert!(stage.required);
    assert!(!stage.terminal);
}

#[test]
fn test_status_properties() {
    let status = Status {
        id: "in-progress".to_string(),
        stage_id: "development".to_string(),
        name: "In Progress".to_string(),
        description: Some("Work in progress".to_string()),
        color: Some("#8B5CF6".to_string()),
    };

    assert_eq!(status.id, "in-progress");
    assert_eq!(status.stage_id, "development");
    assert_eq!(status.name, "In Progress");
    assert!(status.color.is_some());
}

#[test]
fn test_transition_properties() {
    let transition = Transition {
        from: "todo".to_string(),
        to: vec!["in-progress".to_string(), "canceled".to_string()],
    };

    assert_eq!(transition.from, "todo");
    assert_eq!(transition.to.len(), 2);
    assert!(transition.to.contains(&"in-progress".to_string()));
}
