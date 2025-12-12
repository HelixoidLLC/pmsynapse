//! Export functionality for Claude Code sessions
//!
//! Supports exporting to JSON (ampcode-style) and Markdown formats.

use super::analyzer::SessionStatistics;
use super::models::*;
use crate::{Result, SynapseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Exporter for Claude Code sessions
pub struct SessionExporter;

impl SessionExporter {
    /// Create a new session exporter
    pub fn new() -> Self {
        Self
    }

    /// Export session to JSON (ampcode-style structure)
    pub fn export_json(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        output_path: &Path,
        pretty: bool,
    ) -> Result<()> {
        let thread_data = self.build_thread_data(session, stats);

        let json = if pretty {
            serde_json::to_string_pretty(&thread_data)
        } else {
            serde_json::to_string(&thread_data)
        }
        .map_err(SynapseError::Serialization)?;

        let mut file = File::create(output_path).map_err(SynapseError::Io)?;
        file.write_all(json.as_bytes()).map_err(SynapseError::Io)?;

        Ok(())
    }

    /// Export session to JSON string
    pub fn export_json_string(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        pretty: bool,
    ) -> Result<String> {
        let thread_data = self.build_thread_data(session, stats);

        let json = if pretty {
            serde_json::to_string_pretty(&thread_data)
        } else {
            serde_json::to_string(&thread_data)
        }
        .map_err(SynapseError::Serialization)?;

        Ok(json)
    }

    /// Export session to Markdown
    pub fn export_markdown(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        output_path: &Path,
    ) -> Result<()> {
        let markdown = self.build_markdown(session, stats);

        let mut file = File::create(output_path).map_err(SynapseError::Io)?;
        file.write_all(markdown.as_bytes())
            .map_err(SynapseError::Io)?;

        Ok(())
    }

    /// Export session to Markdown string
    pub fn export_markdown_string(&self, session: &Session, stats: &SessionStatistics) -> String {
        self.build_markdown(session, stats)
    }

    /// Build ampcode-style thread structure
    pub fn build_thread_data(&self, session: &Session, stats: &SessionStatistics) -> ThreadData {
        let title = self.infer_title(session);

        let messages: Vec<_> = session
            .messages
            .iter()
            .map(|m| self.convert_message(m))
            .collect();

        ThreadData {
            thread_id: session.session_id.clone(),
            title,
            created_at: session.metadata.start_time,
            updated_at: session.metadata.end_time,
            status: if session.metadata.end_time.is_some() {
                "complete".to_string()
            } else {
                "in-progress".to_string()
            },
            metadata: ThreadMetadata {
                cwd: session.metadata.cwd.clone(),
                git_branch: session.metadata.git_branch.clone(),
                version: session.metadata.version.clone(),
                duration_seconds: session.metadata.duration_seconds,
                message_count: stats.total_messages,
                tool_call_count: stats.tool_calls,
                is_agent: session.is_agent,
                parent_session_id: session.parent_session_id.clone(),
            },
            messages,
            child_agents: session.child_agents.clone(),
            statistics: stats.clone(),
        }
    }

    fn infer_title(&self, session: &Session) -> String {
        // Try to get title from first user message
        for message in &session.messages {
            if matches!(message.role, Some(MessageRole::User)) {
                if let Some(text) = &message.content.text {
                    let first_line = text.lines().next().unwrap_or("");
                    if !first_line.is_empty() {
                        return first_line.chars().take(100).collect();
                    }
                }
            }
        }

        // Fallback to session ID
        if session.session_id.len() >= 8 {
            format!("Session {}", &session.session_id[..8])
        } else {
            format!("Session {}", session.session_id)
        }
    }

    fn convert_message(&self, message: &Message) -> ThreadMessage {
        ThreadMessage {
            uuid: message.uuid.clone(),
            parent_uuid: message.parent_uuid.clone(),
            role: message.role.clone(),
            timestamp: message.timestamp,
            content: message.content.text.clone(),
            thinking: message.content.thinking.clone(),
            tool_uses: message.tool_uses.clone(),
            message_type: message.message_type.clone(),
        }
    }

    /// Build Markdown representation
    fn build_markdown(&self, session: &Session, stats: &SessionStatistics) -> String {
        let mut md = String::new();

        // Header
        let title = self.infer_title(session);
        md.push_str(&format!("# {}\n\n", title));

        // Metadata section
        md.push_str("## Session Metadata\n\n");
        md.push_str(&format!("- **Session ID**: `{}`\n", session.session_id));
        if let Some(agent_id) = &session.agent_id {
            md.push_str(&format!("- **Agent ID**: `{}`\n", agent_id));
        }
        if let Some(cwd) = &session.metadata.cwd {
            md.push_str(&format!("- **Working Directory**: `{}`\n", cwd));
        }
        if let Some(branch) = &session.metadata.git_branch {
            md.push_str(&format!("- **Git Branch**: `{}`\n", branch));
        }
        if let Some(start) = session.metadata.start_time {
            md.push_str(&format!("- **Started**: {}\n", start.to_rfc3339()));
        }
        if let Some(end) = session.metadata.end_time {
            md.push_str(&format!("- **Ended**: {}\n", end.to_rfc3339()));
        }
        if let Some(duration) = session.metadata.duration_seconds {
            md.push_str(&format!("- **Duration**: {} seconds\n", duration));
        }
        md.push_str(&format!("- **Messages**: {}\n", stats.total_messages));
        md.push_str(&format!("- **Tool Calls**: {}\n", stats.tool_calls));
        md.push('\n');

        // Tool usage statistics
        if !stats.tool_usage.is_empty() {
            md.push_str("### Tool Usage\n\n");
            let mut tools: Vec<_> = stats.tool_usage.iter().collect();
            tools.sort_by(|a, b| b.1.cmp(a.1));
            for (tool, count) in tools {
                md.push_str(&format!("- **{}**: {} calls\n", tool, count));
            }
            md.push('\n');
        }

        // Child agents
        if !session.child_agents.is_empty() {
            md.push_str("### Spawned Agents\n\n");
            for agent_id in &session.child_agents {
                md.push_str(&format!("- `{}`\n", agent_id));
            }
            md.push('\n');
        }

        // Message flow
        md.push_str("## Conversation Flow\n\n");
        for message in &session.messages {
            self.write_message_markdown(&mut md, message);
        }

        md
    }

    fn write_message_markdown(&self, md: &mut String, message: &Message) {
        // Message header
        let role_str = message
            .role
            .as_ref()
            .map(|r| format!("{:?}", r))
            .unwrap_or_else(|| "System".to_string());

        let timestamp_str = message
            .timestamp
            .map(|t| t.to_rfc3339())
            .unwrap_or_else(|| "unknown".to_string());

        md.push_str(&format!("### {} - {}\n\n", role_str, timestamp_str));

        // Thinking block (collapsed)
        if let Some(thinking) = &message.content.thinking {
            md.push_str("<details>\n<summary>Thinking</summary>\n\n");
            md.push_str("```\n");
            md.push_str(thinking);
            md.push_str("\n```\n</details>\n\n");
        }

        // Content
        if let Some(text) = &message.content.text {
            md.push_str(text);
            md.push_str("\n\n");
        }

        // Tool uses
        for tool_use in &message.tool_uses {
            md.push_str(&format!("#### Tool: `{}`\n\n", tool_use.tool_name));
            md.push_str("**Input:**\n```json\n");
            md.push_str(&serde_json::to_string_pretty(&tool_use.input).unwrap_or_default());
            md.push_str("\n```\n\n");

            if let Some(output) = &tool_use.output {
                md.push_str("**Output:**\n```json\n");
                md.push_str(&serde_json::to_string_pretty(&output.content).unwrap_or_default());
                md.push_str("\n```\n\n");
            }
        }

        md.push_str("---\n\n");
    }
}

impl Default for SessionExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// ampcode-style thread data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    pub thread_id: String,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: String,
    pub metadata: ThreadMetadata,
    pub messages: Vec<ThreadMessage>,
    pub child_agents: Vec<String>,
    pub statistics: SessionStatistics,
}

/// Thread metadata for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetadata {
    pub cwd: Option<String>,
    pub git_branch: Option<String>,
    pub version: Option<String>,
    pub duration_seconds: Option<i64>,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub is_agent: bool,
    pub parent_session_id: Option<String>,
}

/// Thread message for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub role: Option<MessageRole>,
    pub timestamp: Option<DateTime<Utc>>,
    pub content: Option<String>,
    pub thinking: Option<String>,
    pub tool_uses: Vec<ToolUse>,
    pub message_type: MessageType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_session() -> Session {
        Session {
            session_id: "test-session-123".to_string(),
            is_agent: false,
            agent_id: None,
            parent_session_id: None,
            metadata: SessionMetadata {
                cwd: Some("/test/dir".to_string()),
                version: Some("1.0.0".to_string()),
                git_branch: Some("main".to_string()),
                start_time: None,
                end_time: None,
                duration_seconds: Some(120),
                message_count: 2,
                tool_call_count: 1,
                file_size_bytes: 1024,
            },
            messages: vec![
                Message {
                    uuid: "msg-1".to_string(),
                    parent_uuid: None,
                    is_sidechain: false,
                    message_type: MessageType::User,
                    role: Some(MessageRole::User),
                    timestamp: None,
                    content: MessageContent {
                        text: Some("Hello, this is a test message".to_string()),
                        thinking: None,
                        raw_content: serde_json::Value::Null,
                    },
                    tool_uses: Vec::new(),
                },
                Message {
                    uuid: "msg-2".to_string(),
                    parent_uuid: Some("msg-1".to_string()),
                    is_sidechain: false,
                    message_type: MessageType::Assistant,
                    role: Some(MessageRole::Assistant),
                    timestamp: None,
                    content: MessageContent {
                        text: Some("Response from assistant".to_string()),
                        thinking: Some("Let me think about this...".to_string()),
                        raw_content: serde_json::Value::Null,
                    },
                    tool_uses: vec![ToolUse {
                        tool_id: "tool-1".to_string(),
                        tool_name: "Read".to_string(),
                        timestamp: None,
                        input: serde_json::json!({"file_path": "/test.txt"}),
                        output: None,
                    }],
                },
            ],
            child_agents: vec!["agent-1".to_string()],
        }
    }

    fn create_test_stats() -> SessionStatistics {
        let mut tool_usage = HashMap::new();
        tool_usage.insert("Read".to_string(), 1);

        SessionStatistics {
            total_messages: 2,
            tool_calls: 1,
            tool_usage,
            message_flow: Vec::new(),
            duration_seconds: Some(120),
        }
    }

    #[test]
    fn test_infer_title() {
        let session = create_test_session();
        let exporter = SessionExporter::new();

        let title = exporter.infer_title(&session);
        assert_eq!(title, "Hello, this is a test message");
    }

    #[test]
    fn test_export_json_string() {
        let session = create_test_session();
        let stats = create_test_stats();
        let exporter = SessionExporter::new();

        let json = exporter.export_json_string(&session, &stats, true).unwrap();

        assert!(json.contains("test-session-123"));
        assert!(json.contains("Hello, this is a test message"));
    }

    #[test]
    fn test_export_markdown_string() {
        let session = create_test_session();
        let stats = create_test_stats();
        let exporter = SessionExporter::new();

        let md = exporter.export_markdown_string(&session, &stats);

        assert!(md.contains("# Hello, this is a test message"));
        assert!(md.contains("**Session ID**: `test-session-123`"));
        assert!(md.contains("### Spawned Agents"));
        assert!(md.contains("`agent-1`"));
        assert!(md.contains("Tool: `Read`"));
    }

    #[test]
    fn test_build_thread_data() {
        let session = create_test_session();
        let stats = create_test_stats();
        let exporter = SessionExporter::new();

        let thread_data = exporter.build_thread_data(&session, &stats);

        assert_eq!(thread_data.thread_id, "test-session-123");
        assert_eq!(thread_data.messages.len(), 2);
        assert_eq!(thread_data.child_agents.len(), 1);
        assert!(!thread_data.metadata.is_agent);
    }
}
