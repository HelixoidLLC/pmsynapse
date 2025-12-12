//! Session analyzer and hierarchy builder for Claude Code sessions

use super::models::*;
use super::parser::SessionParser;
use crate::{Result, SynapseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Analyzer for Claude Code sessions
pub struct SessionAnalyzer {
    parser: SessionParser,
}

impl SessionAnalyzer {
    /// Create a new session analyzer
    pub fn new(project_dir: String) -> Self {
        Self {
            parser: SessionParser::new(project_dir),
        }
    }

    /// Get reference to the internal parser
    pub fn parser(&self) -> &SessionParser {
        &self.parser
    }

    /// Analyze all sessions in a directory and build hierarchy
    pub fn analyze_directory(&self, dir_path: &Path) -> Result<SessionHierarchy> {
        let mut sessions = HashMap::new();
        let mut agents = HashMap::new();

        // Read all JSONL files
        for entry in std::fs::read_dir(dir_path).map_err(SynapseError::Io)? {
            let entry = entry.map_err(SynapseError::Io)?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
                match self.parser.parse_file(&path) {
                    Ok(session) => {
                        if session.is_agent {
                            if let Some(agent_id) = &session.agent_id {
                                agents.insert(agent_id.clone(), session);
                            }
                        } else {
                            sessions.insert(session.session_id.clone(), session);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse session file {:?}: {}", path, e);
                    }
                }
            }
        }

        // Build parent-child relationships
        self.build_hierarchy(&mut sessions, &agents)?;

        Ok(SessionHierarchy {
            sessions: sessions.into_values().collect(),
            agents: agents.into_values().collect(),
        })
    }

    /// Build parent-child relationships between sessions and agents
    fn build_hierarchy(
        &self,
        sessions: &mut HashMap<String, Session>,
        agents: &HashMap<String, Session>,
    ) -> Result<()> {
        // Map agents to their parent sessions
        for (agent_id, agent) in agents {
            if let Some(parent_id) = &agent.parent_session_id {
                if let Some(parent_session) = sessions.get_mut(parent_id) {
                    parent_session.child_agents.push(agent_id.clone());
                }
            }
        }

        Ok(())
    }

    /// Analyze a single session and compute statistics
    pub fn analyze_session(&self, session: &Session) -> SessionStatistics {
        let tool_usage = self.compute_tool_usage(session);
        let message_flow = self.build_message_flow(session);

        SessionStatistics {
            total_messages: session.messages.len(),
            tool_calls: session.metadata.tool_call_count,
            tool_usage,
            message_flow,
            duration_seconds: session.metadata.duration_seconds,
        }
    }

    fn compute_tool_usage(&self, session: &Session) -> HashMap<String, usize> {
        let mut usage = HashMap::new();

        for message in &session.messages {
            for tool_use in &message.tool_uses {
                *usage.entry(tool_use.tool_name.clone()).or_insert(0) += 1;
            }
        }

        usage
    }

    fn build_message_flow(&self, session: &Session) -> Vec<MessageFlowNode> {
        let mut flow = Vec::new();

        for message in &session.messages {
            flow.push(MessageFlowNode {
                uuid: message.uuid.clone(),
                parent_uuid: message.parent_uuid.clone(),
                role: message.role.clone(),
                timestamp: message.timestamp,
                has_tools: !message.tool_uses.is_empty(),
            });
        }

        flow
    }
}

/// Session hierarchy containing main sessions and agent sub-sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHierarchy {
    pub sessions: Vec<Session>,
    pub agents: Vec<Session>,
}

/// Statistics computed from session analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    pub total_messages: usize,
    pub tool_calls: usize,
    pub tool_usage: HashMap<String, usize>,
    pub message_flow: Vec<MessageFlowNode>,
    pub duration_seconds: Option<i64>,
}

/// Node in the message flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFlowNode {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub role: Option<MessageRole>,
    pub timestamp: Option<DateTime<Utc>>,
    pub has_tools: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_session_hierarchy() {
        let dir = tempdir().unwrap();

        // Create main session
        let main_session = r#"{"uuid":"main-msg","type":"user","sessionId":"main","message":{"role":"user","content":[{"type":"text","text":"test"}]}}"#;
        let mut main_file = std::fs::File::create(dir.path().join("main.jsonl")).unwrap();
        writeln!(main_file, "{}", main_session).unwrap();

        // Create agent session
        let agent_session = r#"{"uuid":"agent-msg","isSidechain":true,"sessionId":"main","agentId":"a1","message":{"role":"assistant","content":[{"type":"text","text":"test"}]}}"#;
        let mut agent_file = std::fs::File::create(dir.path().join("agent-a1.jsonl")).unwrap();
        writeln!(agent_file, "{}", agent_session).unwrap();

        let analyzer = SessionAnalyzer::new(String::new());
        let hierarchy = analyzer.analyze_directory(dir.path()).unwrap();

        assert_eq!(hierarchy.sessions.len(), 1);
        assert_eq!(hierarchy.agents.len(), 1);
        assert_eq!(hierarchy.sessions[0].child_agents.len(), 1);
    }

    #[test]
    fn test_tool_usage_statistics() {
        let session = Session {
            session_id: "test".to_string(),
            is_agent: false,
            agent_id: None,
            parent_session_id: None,
            metadata: SessionMetadata::default(),
            messages: vec![
                Message {
                    uuid: "1".to_string(),
                    parent_uuid: None,
                    is_sidechain: false,
                    message_type: MessageType::Assistant,
                    role: Some(MessageRole::Assistant),
                    timestamp: None,
                    content: MessageContent::default(),
                    tool_uses: vec![
                        ToolUse {
                            tool_id: "t1".to_string(),
                            tool_name: "Read".to_string(),
                            timestamp: None,
                            input: serde_json::Value::Null,
                            output: None,
                        },
                        ToolUse {
                            tool_id: "t2".to_string(),
                            tool_name: "Read".to_string(),
                            timestamp: None,
                            input: serde_json::Value::Null,
                            output: None,
                        },
                    ],
                },
                Message {
                    uuid: "2".to_string(),
                    parent_uuid: None,
                    is_sidechain: false,
                    message_type: MessageType::Assistant,
                    role: Some(MessageRole::Assistant),
                    timestamp: None,
                    content: MessageContent::default(),
                    tool_uses: vec![ToolUse {
                        tool_id: "t3".to_string(),
                        tool_name: "Write".to_string(),
                        timestamp: None,
                        input: serde_json::Value::Null,
                        output: None,
                    }],
                },
            ],
            child_agents: Vec::new(),
        };

        let analyzer = SessionAnalyzer::new(String::new());
        let stats = analyzer.analyze_session(&session);

        assert_eq!(stats.tool_usage.get("Read"), Some(&2));
        assert_eq!(stats.tool_usage.get("Write"), Some(&1));
    }
}
