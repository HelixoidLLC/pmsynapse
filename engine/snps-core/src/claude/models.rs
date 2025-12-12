//! Data models for Claude Code session parsing
//!
//! These structures represent the parsed JSONL session data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Root session structure (main or agent)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub is_agent: bool,
    pub agent_id: Option<String>,
    pub parent_session_id: Option<String>,
    pub metadata: SessionMetadata,
    pub messages: Vec<Message>,
    pub child_agents: Vec<String>,
}

/// Session-level metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionMetadata {
    pub cwd: Option<String>,
    pub version: Option<String>,
    pub git_branch: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub file_size_bytes: u64,
}

/// Individual message in session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub is_sidechain: bool,
    pub message_type: MessageType,
    pub role: Option<MessageRole>,
    pub timestamp: Option<DateTime<Utc>>,
    pub content: MessageContent,
    pub tool_uses: Vec<ToolUse>,
}

/// Message type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    User,
    Assistant,
    System,
    Summary,
    #[serde(rename = "file-history-snapshot")]
    FileHistorySnapshot,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::System
    }
}

/// Message role enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Message content (text, thinking, tool results)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageContent {
    pub text: Option<String>,
    pub thinking: Option<String>,
    pub raw_content: serde_json::Value,
}

/// Tool use block with input/output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    pub tool_id: String,
    pub tool_name: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub input: serde_json::Value,
    pub output: Option<ToolResult>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub content: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: Option<u64>,
}

/// Raw JSONL message record for parsing
#[derive(Debug, Deserialize)]
pub struct JsonlRecord {
    pub uuid: Option<String>,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    #[serde(rename = "isSidechain")]
    pub is_sidechain: Option<bool>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    pub timestamp: Option<String>,
    pub message: Option<serde_json::Value>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_default() {
        let mt = MessageType::default();
        assert_eq!(mt, MessageType::System);
    }

    #[test]
    fn test_session_metadata_default() {
        let meta = SessionMetadata::default();
        assert!(meta.cwd.is_none());
        assert_eq!(meta.message_count, 0);
    }

    #[test]
    fn test_jsonl_record_deserialize() {
        let json = r#"{"uuid":"test-uuid","type":"user","sessionId":"test-session"}"#;
        let record: JsonlRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.uuid, Some("test-uuid".to_string()));
        assert_eq!(record.message_type, Some("user".to_string()));
    }
}
