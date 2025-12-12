//! JSONL session parser for Claude Code sessions

use super::models::*;
use crate::{Result, SynapseError};
use chrono::DateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parser for Claude Code JSONL session files
pub struct SessionParser {
    project_dir: String,
}

impl SessionParser {
    /// Create a new session parser
    pub fn new(project_dir: String) -> Self {
        Self { project_dir }
    }

    /// Get the project directory
    pub fn project_dir(&self) -> &str {
        &self.project_dir
    }

    /// Parse a single JSONL file into a Session
    pub fn parse_file(&self, file_path: &Path) -> Result<Session> {
        let file = File::open(file_path).map_err(SynapseError::Io)?;
        let reader = BufReader::new(file);

        let mut records: Vec<JsonlRecord> = Vec::new();

        // Parse each line as JSONL
        for line in reader.lines() {
            let line = line.map_err(SynapseError::Io)?;
            if line.trim().is_empty() {
                continue;
            }

            let record: JsonlRecord =
                serde_json::from_str(&line).map_err(SynapseError::Serialization)?;
            records.push(record);
        }

        // Convert to Session
        self.build_session(file_path, records)
    }

    /// Build Session from JSONL records
    fn build_session(&self, file_path: &Path, records: Vec<JsonlRecord>) -> Result<Session> {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| SynapseError::Config("Invalid filename".to_string()))?;

        let is_agent = filename.starts_with("agent-");
        let session_id = if is_agent {
            // Extract from first record
            records
                .first()
                .and_then(|r| r.session_id.clone())
                .unwrap_or_default()
        } else {
            filename.trim_end_matches(".jsonl").to_string()
        };

        let agent_id = if is_agent {
            Some(
                filename
                    .trim_start_matches("agent-")
                    .trim_end_matches(".jsonl")
                    .to_string(),
            )
        } else {
            None
        };

        // Extract metadata
        let metadata = self.extract_metadata(&records, file_path)?;

        // Convert records to messages
        let messages = self.build_messages(&records)?;

        Ok(Session {
            session_id,
            is_agent,
            agent_id,
            parent_session_id: records.first().and_then(|r| r.session_id.clone()),
            metadata,
            messages,
            child_agents: Vec::new(), // Will be populated by analyzer
        })
    }

    fn extract_metadata(
        &self,
        records: &[JsonlRecord],
        file_path: &Path,
    ) -> Result<SessionMetadata> {
        let file_size = std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);

        let timestamps: Vec<_> = records
            .iter()
            .filter_map(|r| r.timestamp.as_ref())
            .filter_map(|t| DateTime::parse_from_rfc3339(t).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .collect();

        let start_time = timestamps.first().cloned();
        let end_time = timestamps.last().cloned();

        let duration_seconds = match (start_time, end_time) {
            (Some(start), Some(end)) => Some((end - start).num_seconds()),
            _ => None,
        };

        let tool_call_count = records
            .iter()
            .filter(|r| {
                r.message
                    .as_ref()
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter().any(|item| {
                            item.get("type").and_then(|t| t.as_str()) == Some("tool_use")
                        })
                    })
                    .unwrap_or(false)
            })
            .count();

        Ok(SessionMetadata {
            cwd: records.first().and_then(|r| r.cwd.clone()),
            version: records.first().and_then(|r| r.version.clone()),
            git_branch: records.first().and_then(|r| r.git_branch.clone()),
            start_time,
            end_time,
            duration_seconds,
            message_count: records.len(),
            tool_call_count,
            file_size_bytes: file_size,
        })
    }

    fn build_messages(&self, records: &[JsonlRecord]) -> Result<Vec<Message>> {
        let mut messages = Vec::new();

        for record in records {
            if record.uuid.is_some() {
                let message = self.build_message(record)?;
                messages.push(message);
            }
        }

        Ok(messages)
    }

    fn build_message(&self, record: &JsonlRecord) -> Result<Message> {
        let message_type = self.parse_message_type(record.message_type.as_deref());

        let role = record
            .message
            .as_ref()
            .and_then(|m| m.get("role"))
            .and_then(|r| r.as_str())
            .and_then(|r| match r {
                "user" => Some(MessageRole::User),
                "assistant" => Some(MessageRole::Assistant),
                "system" => Some(MessageRole::System),
                _ => None,
            });

        let timestamp = record
            .timestamp
            .as_ref()
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));

        let (content, tool_uses) = self.extract_content_and_tools(record)?;

        Ok(Message {
            uuid: record.uuid.clone().unwrap_or_default(),
            parent_uuid: record.parent_uuid.clone(),
            is_sidechain: record.is_sidechain.unwrap_or(false),
            message_type,
            role,
            timestamp,
            content,
            tool_uses,
        })
    }

    fn parse_message_type(&self, type_str: Option<&str>) -> MessageType {
        match type_str {
            Some("user") => MessageType::User,
            Some("assistant") => MessageType::Assistant,
            Some("system") => MessageType::System,
            Some("summary") => MessageType::Summary,
            Some("file-history-snapshot") => MessageType::FileHistorySnapshot,
            _ => MessageType::System,
        }
    }

    fn extract_content_and_tools(
        &self,
        record: &JsonlRecord,
    ) -> Result<(MessageContent, Vec<ToolUse>)> {
        let raw_content = record.message.clone().unwrap_or(serde_json::Value::Null);

        let mut text_parts = Vec::new();
        let mut thinking = None;
        let mut tool_uses = Vec::new();

        if let Some(content_array) = raw_content.get("content").and_then(|c| c.as_array()) {
            for item in content_array {
                match item.get("type").and_then(|t| t.as_str()) {
                    Some("text") => {
                        if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                            text_parts.push(text.to_string());
                        }
                    }
                    Some("thinking") => {
                        if let Some(think) = item.get("thinking").and_then(|t| t.as_str()) {
                            thinking = Some(think.to_string());
                        }
                    }
                    Some("tool_use") => {
                        let tool_use = ToolUse {
                            tool_id: item
                                .get("id")
                                .and_then(|id| id.as_str())
                                .unwrap_or("")
                                .to_string(),
                            tool_name: item
                                .get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            timestamp: record
                                .timestamp
                                .as_ref()
                                .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
                                .map(|dt| dt.with_timezone(&chrono::Utc)),
                            input: item
                                .get("input")
                                .cloned()
                                .unwrap_or(serde_json::Value::Null),
                            output: None, // Will be populated from tool_result messages
                        };
                        tool_uses.push(tool_use);
                    }
                    _ => {}
                }
            }
        }

        let text = if text_parts.is_empty() {
            None
        } else {
            Some(text_parts.join("\n"))
        };

        let content = MessageContent {
            text,
            thinking,
            raw_content,
        };

        Ok((content, tool_uses))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_main_session() {
        let sample_jsonl = r#"{"uuid":"test-uuid","type":"user","sessionId":"test-session","timestamp":"2025-12-12T20:00:00Z","message":{"role":"user","content":[{"type":"text","text":"test"}]}}"#;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test-session.jsonl");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", sample_jsonl).unwrap();

        let parser = SessionParser::new(String::new());
        let session = parser.parse_file(&file_path).unwrap();

        assert_eq!(session.session_id, "test-session");
        assert!(!session.is_agent);
        assert_eq!(session.messages.len(), 1);
    }

    #[test]
    fn test_parse_agent_session() {
        let sample_jsonl = r#"{"uuid":"agent-uuid","isSidechain":true,"type":"assistant","sessionId":"parent-session","agentId":"abc123","timestamp":"2025-12-12T20:00:00Z","message":{"role":"assistant","content":[{"type":"text","text":"test"}]}}"#;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("agent-abc123.jsonl");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", sample_jsonl).unwrap();

        let parser = SessionParser::new(String::new());
        let session = parser.parse_file(&file_path).unwrap();

        assert!(session.is_agent);
        assert_eq!(session.agent_id, Some("abc123".to_string()));
        assert_eq!(
            session.parent_session_id,
            Some("parent-session".to_string())
        );
    }

    #[test]
    fn test_tool_use_extraction() {
        let sample_jsonl = r#"{"uuid":"tool-msg","type":"assistant","timestamp":"2025-12-12T20:00:00Z","message":{"role":"assistant","content":[{"type":"tool_use","id":"tool-1","name":"Read","input":{"file_path":"/test.txt"}}]}}"#;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("session.jsonl");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", sample_jsonl).unwrap();

        let parser = SessionParser::new(String::new());
        let session = parser.parse_file(&file_path).unwrap();

        assert_eq!(session.messages[0].tool_uses.len(), 1);
        assert_eq!(session.messages[0].tool_uses[0].tool_name, "Read");
    }
}
