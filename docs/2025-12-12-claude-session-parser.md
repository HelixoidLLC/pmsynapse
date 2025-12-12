# Claude Code Session Parser Implementation Plan

## Overview

Build a comprehensive system to parse, analyze, and export Claude Code session data from JSONL files. This enables capturing rich conversation history including tool calls, agent hierarchies, and file changes for use in project documentation, tickets, and web-based session viewers (similar to ampcode.com thread view).

## Current State Analysis

**What Exists:**
- Claude Code stores sessions as JSONL files in `~/.claude/projects/<project-slug>/`
- Each line is a JSON message object with uuid-based DAG structure
- Main sessions: `<session-uuid>.jsonl`
- Agent sub-sessions: `agent-<agent-id>.jsonl` with `isSidechain: true`
- Messages contain tool uses, results, and file operations
- No built-in parser or export functionality in PMSynapse

**What's Missing:**
- Parser to extract structured data from JSONL files
- Data model for representing sessions, messages, and tool calls
- Export capabilities (JSON, Markdown, HTML)
- CLI commands to interact with session data
- Web-renderable format for session visualization

**Key Discoveries:**
- JSONL structure uses `uuid` + `parentUuid` for message threading (DAG)
- `isSidechain: true` indicates agent sub-sessions spawned via Task tool
- `sessionId` field links agents to parent sessions
- Tool uses embedded in message content as `tool_use` blocks
- Current research session `6df416b8-9af7-4dc2-9c55-3179c4afb8a8` spawned agent `a9c6145` for web research
- Session `bc289ee8...` spawned 6 agents with 132 messages total

## Desired End State

A complete Claude Code session management system that:

1. **Parses** all JSONL session files into structured data
2. **Analyzes** session hierarchies, tool usage, and timelines
3. **Exports** to multiple formats:
   - JSON (web-renderable, ampcode-style structure)
   - Markdown (documentation, tickets, plans)
   - Statistics (tool counts, duration, message flow)
4. **Integrates** with PMSynapse CLI and thoughts system
5. **Stores** parsed sessions in `thoughts/shared/sessions/` for reference

### Verification:
- Parse existing session files and reconstruct message DAG
- Export session to JSON matching ampcode.com thread structure
- Generate markdown summary with all tool calls and results
- CLI commands work end-to-end: parse → export → view

## What We're NOT Doing

- **Session replay/execution** - Only parsing and exporting, not re-running
- **Real-time monitoring** - Not watching Claude Code sessions as they happen
- **Web UI in this phase** - Data structures support future web viewer, but not building React components yet
- **Session editing** - Read-only analysis, no modification of source JSONL files
- **Cross-project session linking** - Each session analyzed independently
- **AI summarization** - Using LLM to summarize sessions (future enhancement)

## Implementation Approach

**Strategy:** Build incrementally from core parser to rich exports, following PMSynapse's architecture pattern of separating business logic (snps-core) from CLI interface (snps-cli).

**Phase breakdown:**
1. **Core data model** - Rust structs matching JSONL schema
2. **Parser implementation** - JSONL → structured data with DAG reconstruction
3. **Export engine** - JSON/Markdown templates
4. **CLI integration** - Commands following existing patterns
5. **Testing & validation** - Verify against real session files

---

## Phase 1: Core Data Model & JSONL Parser

### Overview
Create the foundational data structures and parsing logic in `snps-core` to represent Claude Code sessions, messages, and tool calls.

### Changes Required

#### 1. New Module: `engine/snps-core/src/claude/mod.rs`

**File**: `engine/snps-core/src/claude/mod.rs` (new)
**Changes**: Create module with public exports

```rust
//! Claude Code session parsing and analysis

pub mod models;
pub mod parser;
pub mod analyzer;

pub use models::{Session, Message, ToolUse, SessionMetadata};
pub use parser::SessionParser;
pub use analyzer::SessionAnalyzer;
```

#### 2. Data Models: `engine/snps-core/src/claude/models.rs`

**File**: `engine/snps-core/src/claude/models.rs` (new)
**Changes**: Define complete data model matching JSONL structure

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    User,
    Assistant,
    System,
    Summary,
    #[serde(rename = "file-history-snapshot")]
    FileHistorySnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Message content (text, thinking, tool results)
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub content: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: Option<u64>,
}

/// Raw JSONL message record
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
```

#### 3. Parser Implementation: `engine/snps-core/src/claude/parser.rs`

**File**: `engine/snps-core/src/claude/parser.rs` (new)
**Changes**: JSONL parsing logic

```rust
use super::models::*;
use crate::{Result, SynapseError};
use chrono::DateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SessionParser {
    project_dir: String,
}

impl SessionParser {
    pub fn new(project_dir: String) -> Self {
        Self { project_dir }
    }

    /// Parse a single JSONL file into a Session
    pub fn parse_file(&self, file_path: &Path) -> Result<Session> {
        let file = File::open(file_path)
            .map_err(|e| SynapseError::Io(e))?;
        let reader = BufReader::new(file);

        let mut records: Vec<JsonlRecord> = Vec::new();

        // Parse each line as JSONL
        for line in reader.lines() {
            let line = line.map_err(|e| SynapseError::Io(e))?;
            if line.trim().is_empty() {
                continue;
            }

            let record: JsonlRecord = serde_json::from_str(&line)
                .map_err(|e| SynapseError::Serialization(e))?;
            records.push(record);
        }

        // Convert to Session
        self.build_session(file_path, records)
    }

    /// Build Session from JSONL records
    fn build_session(&self, file_path: &Path, records: Vec<JsonlRecord>) -> Result<Session> {
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| SynapseError::Config("Invalid filename".to_string()))?;

        let is_agent = filename.starts_with("agent-");
        let session_id = if is_agent {
            // Extract from first record
            records.first()
                .and_then(|r| r.session_id.clone())
                .unwrap_or_default()
        } else {
            filename.trim_end_matches(".jsonl").to_string()
        };

        let agent_id = if is_agent {
            Some(filename.trim_start_matches("agent-")
                .trim_end_matches(".jsonl")
                .to_string())
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

    fn extract_metadata(&self, records: &[JsonlRecord], file_path: &Path) -> Result<SessionMetadata> {
        let file_size = std::fs::metadata(file_path)
            .map(|m| m.len())
            .unwrap_or(0);

        let timestamps: Vec<_> = records.iter()
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

        let tool_call_count = records.iter()
            .filter(|r| {
                r.message.as_ref()
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_array())
                    .map(|arr| arr.iter().any(|item|
                        item.get("type").and_then(|t| t.as_str()) == Some("tool_use")
                    ))
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
            if let Some(uuid) = &record.uuid {
                let message = self.build_message(record)?;
                messages.push(message);
            }
        }

        Ok(messages)
    }

    fn build_message(&self, record: &JsonlRecord) -> Result<Message> {
        let message_type = self.parse_message_type(record.message_type.as_deref());

        let role = record.message.as_ref()
            .and_then(|m| m.get("role"))
            .and_then(|r| r.as_str())
            .and_then(|r| match r {
                "user" => Some(MessageRole::User),
                "assistant" => Some(MessageRole::Assistant),
                "system" => Some(MessageRole::System),
                _ => None,
            });

        let timestamp = record.timestamp.as_ref()
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

    fn extract_content_and_tools(&self, record: &JsonlRecord) -> Result<(MessageContent, Vec<ToolUse>)> {
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
                            tool_id: item.get("id")
                                .and_then(|id| id.as_str())
                                .unwrap_or("")
                                .to_string(),
                            tool_name: item.get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            timestamp: record.timestamp.as_ref()
                                .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
                                .map(|dt| dt.with_timezone(&chrono::Utc)),
                            input: item.get("input").cloned().unwrap_or(serde_json::Value::Null),
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
```

#### 4. Update `engine/snps-core/src/lib.rs`

**File**: `engine/snps-core/src/lib.rs`
**Changes**: Export new claude module

```rust
pub mod graph;
pub mod idlc;
pub mod llm;
pub mod claude;  // Add this line

use thiserror::Error;
// ... rest of file unchanged
```

### Success Criteria

#### Automated Verification:
- [ ] Code compiles without errors: `cargo build -p snps-core`
- [ ] Unit tests pass: `cargo test -p snps-core`
- [ ] Clippy checks pass: `cargo clippy -p snps-core --all-targets -- -D warnings`
- [ ] Format check passes: `cargo fmt --all -- --check`

#### Manual Verification:
- [ ] Can parse sample JSONL file from `~/.claude/projects/` directory
- [ ] Session struct correctly populated with metadata
- [ ] Messages extracted with proper uuid chain
- [ ] Tool uses identified and structured correctly
- [ ] Agent sessions correctly identified with `is_agent: true`

**Implementation Note**: After completing this phase and all automated verification passes, manually test parsing at least 3 different session files (1 main session, 1 agent session, 1 with multiple tool calls) before proceeding to Phase 2.

---

## Phase 2: Session Analyzer & Hierarchy Builder

### Overview
Build the analyzer that reconstructs session hierarchies, maps parent-child agent relationships, and computes statistics.

### Changes Required

#### 1. Analyzer Module: `engine/snps-core/src/claude/analyzer.rs`

**File**: `engine/snps-core/src/claude/analyzer.rs` (new)
**Changes**: Implement session analysis and hierarchy building

```rust
use super::models::*;
use super::parser::SessionParser;
use crate::{Result, SynapseError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SessionAnalyzer {
    parser: SessionParser,
}

impl SessionAnalyzer {
    pub fn new(project_dir: String) -> Self {
        Self {
            parser: SessionParser::new(project_dir),
        }
    }

    /// Analyze all sessions in a directory and build hierarchy
    pub fn analyze_directory(&self, dir_path: &Path) -> Result<SessionHierarchy> {
        let mut sessions = HashMap::new();
        let mut agents = HashMap::new();

        // Read all JSONL files
        for entry in std::fs::read_dir(dir_path)
            .map_err(|e| SynapseError::Io(e))?
        {
            let entry = entry.map_err(|e| SynapseError::Io(e))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
                let session = self.parser.parse_file(&path)?;

                if session.is_agent {
                    if let Some(agent_id) = &session.agent_id {
                        agents.insert(agent_id.clone(), session);
                    }
                } else {
                    sessions.insert(session.session_id.clone(), session);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHierarchy {
    pub sessions: Vec<Session>,
    pub agents: Vec<Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    pub total_messages: usize,
    pub tool_calls: usize,
    pub tool_usage: HashMap<String, usize>,
    pub message_flow: Vec<MessageFlowNode>,
    pub duration_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFlowNode {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub role: Option<MessageRole>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub has_tools: bool,
}
```

#### 2. Update Models: `engine/snps-core/src/claude/models.rs`

**File**: `engine/snps-core/src/claude/models.rs`
**Changes**: Add Serialize derives and export analyzer types

```rust
// Add to imports
use serde::Serialize;

// Add Serialize to all structs (already have Deserialize)
// Example update to Session struct:
#[derive(Debug, Clone, Serialize, Deserialize)]  // Added Serialize
pub struct Session {
    // ... fields unchanged
}

// Apply Serialize to all: SessionMetadata, Message, MessageContent, ToolUse, ToolResult
```

#### 3. Update Module Exports: `engine/snps-core/src/claude/mod.rs`

**File**: `engine/snps-core/src/claude/mod.rs`
**Changes**: Export analyzer types

```rust
pub use analyzer::{SessionAnalyzer, SessionHierarchy, SessionStatistics};
```

### Success Criteria

#### Automated Verification:
- [ ] Code compiles: `cargo build -p snps-core`
- [ ] All tests pass: `cargo test -p snps-core`
- [ ] Clippy clean: `cargo clippy -p snps-core --all-targets -- -D warnings`
- [ ] Format check: `cargo fmt --all -- --check`

#### Manual Verification:
- [ ] Can analyze directory with multiple session files
- [ ] Parent-child relationships correctly built (verify `bc289ee8...` has 6 child agents)
- [ ] Tool usage statistics accurate (count matches manual inspection)
- [ ] Message flow preserves chronological order
- [ ] Duration calculations correct for sessions with start/end times

**Implementation Note**: After completing this phase, test analyzer on the research session data collected earlier. Verify the session hierarchy matches the expected structure from previous analysis.

---

## Phase 3: Export Engine (JSON & Markdown)

### Overview
Implement export functionality to convert parsed sessions into JSON (ampcode-style) and Markdown formats.

### Changes Required

#### 1. Export Module: `engine/snps-core/src/claude/export.rs`

**File**: `engine/snps-core/src/claude/export.rs` (new)
**Changes**: Implement exporters for different formats

```rust
use super::models::*;
use super::analyzer::{SessionHierarchy, SessionStatistics};
use crate::{Result, SynapseError};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct SessionExporter;

impl SessionExporter {
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
        }.map_err(|e| SynapseError::Serialization(e))?;

        let mut file = File::create(output_path)
            .map_err(|e| SynapseError::Io(e))?;
        file.write_all(json.as_bytes())
            .map_err(|e| SynapseError::Io(e))?;

        Ok(())
    }

    /// Export session to Markdown
    pub fn export_markdown(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        output_path: &Path,
    ) -> Result<()> {
        let markdown = self.build_markdown(session, stats);

        let mut file = File::create(output_path)
            .map_err(|e| SynapseError::Io(e))?;
        file.write_all(markdown.as_bytes())
            .map_err(|e| SynapseError::Io(e))?;

        Ok(())
    }

    /// Build ampcode-style thread structure
    fn build_thread_data(&self, session: &Session, stats: &SessionStatistics) -> ThreadData {
        let title = self.infer_title(session);

        let messages: Vec<_> = session.messages.iter()
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
        format!("Session {}", &session.session_id[..8])
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
        md.push_str("\n");

        // Tool usage statistics
        if !stats.tool_usage.is_empty() {
            md.push_str("### Tool Usage\n\n");
            let mut tools: Vec<_> = stats.tool_usage.iter().collect();
            tools.sort_by(|a, b| b.1.cmp(a.1));
            for (tool, count) in tools {
                md.push_str(&format!("- **{}**: {} calls\n", tool, count));
            }
            md.push_str("\n");
        }

        // Child agents
        if !session.child_agents.is_empty() {
            md.push_str("### Spawned Agents\n\n");
            for agent_id in &session.child_agents {
                md.push_str(&format!("- `{}`\n", agent_id));
            }
            md.push_str("\n");
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
        let role_str = message.role.as_ref()
            .map(|r| format!("{:?}", r))
            .unwrap_or_else(|| "System".to_string());

        let timestamp_str = message.timestamp
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

/// ampcode-style thread data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    pub thread_id: String,
    pub title: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub metadata: ThreadMetadata,
    pub messages: Vec<ThreadMessage>,
    pub child_agents: Vec<String>,
    pub statistics: SessionStatistics,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub role: Option<MessageRole>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub content: Option<String>,
    pub thinking: Option<String>,
    pub tool_uses: Vec<ToolUse>,
    pub message_type: MessageType,
}
```

#### 2. Update Module: `engine/snps-core/src/claude/mod.rs`

**File**: `engine/snps-core/src/claude/mod.rs`
**Changes**: Export exporter types

```rust
pub mod export;

pub use export::{SessionExporter, ThreadData};
```

### Success Criteria

#### Automated Verification:
- [ ] Code compiles: `cargo build -p snps-core`
- [ ] Tests pass: `cargo test -p snps-core`
- [ ] Clippy clean: `cargo clippy -p snps-core --all-targets -- -D warnings`
- [ ] Format check: `cargo fmt --all -- --check`

#### Manual Verification:
- [ ] JSON export creates valid, parseable JSON file
- [ ] JSON structure matches ampcode.com thread format (thread_id, messages, metadata)
- [ ] Markdown export is human-readable with proper formatting
- [ ] Thinking blocks collapsed in markdown (using `<details>` tags)
- [ ] Tool calls formatted with input/output clearly separated
- [ ] Title inference works correctly (uses first user message)
- [ ] Statistics section shows accurate tool usage counts

**Implementation Note**: After completing this phase, export the current session (`6df416b8-9af7-4dc2-9c55-3179c4afb8a8.jsonl`) to both JSON and Markdown. Manually inspect outputs to verify structure and completeness before proceeding to CLI integration.

---

## Phase 4: CLI Integration

### Overview
Add `snps claude` subcommand to the CLI with parse, export, list, and analyze commands.

### Changes Required

#### 1. Add Claude Commands: `engine/snps-cli/src/main.rs`

**File**: `engine/snps-cli/src/main.rs`
**Changes**: Add Claude command enum and handlers

```rust
// Add to Commands enum (after line 130)
/// Manage Claude Code sessions
Claude {
    #[command(subcommand)]
    action: ClaudeCommands,
},

// Add new enum after existing subcommand enums (after line 435)
#[derive(Subcommand)]
enum ClaudeCommands {
    /// Parse Claude Code JSONL file(s)
    Parse {
        /// Path to JSONL file or directory
        path: String,

        /// Output format
        #[arg(long, default_value = "json")]
        format: ClaudeExportFormat,

        /// Output file path (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },

    /// List all Claude Code sessions
    List {
        /// Claude projects directory
        #[arg(long, default_value = "~/.claude/projects")]
        claude_dir: String,

        /// Show only main sessions (exclude agents)
        #[arg(long)]
        main_only: bool,
    },

    /// Analyze session and show statistics
    Analyze {
        /// Session file or ID
        session: String,

        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,
    },

    /// Show session hierarchy tree
    Tree {
        /// Session file or ID
        session: String,
    },
}

#[derive(Clone, ValueEnum)]
enum ClaudeExportFormat {
    Json,
    Markdown,
    Md,
}

impl std::fmt::Display for ClaudeExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Markdown | Self::Md => write!(f, "markdown"),
        }
    }
}

// Add dispatch in main match (line 486)
Commands::Claude { action } => cmd_claude(action),

// Add handler functions at end of file

fn cmd_claude(action: ClaudeCommands) -> anyhow::Result<()> {
    match action {
        ClaudeCommands::Parse { path, format, output, pretty } => {
            cmd_claude_parse(path, format, output, pretty)
        }
        ClaudeCommands::List { claude_dir, main_only } => {
            cmd_claude_list(claude_dir, main_only)
        }
        ClaudeCommands::Analyze { session, detailed } => {
            cmd_claude_analyze(session, detailed)
        }
        ClaudeCommands::Tree { session } => {
            cmd_claude_tree(session)
        }
    }
}

fn cmd_claude_parse(
    path: String,
    format: ClaudeExportFormat,
    output: Option<String>,
    pretty: bool,
) -> anyhow::Result<()> {
    use snps_core::claude::{SessionParser, SessionAnalyzer, SessionExporter};
    use std::path::PathBuf;

    println!("{}", format!("Parsing Claude Code session: {}", path).bright_blue());

    let file_path = PathBuf::from(shellexpand::tilde(&path).to_string());

    // Parse session
    let parser = SessionParser::new(String::new());
    let session = parser.parse_file(&file_path)?;

    // Analyze
    let analyzer = SessionAnalyzer::new(String::new());
    let stats = analyzer.analyze_session(&session);

    println!("{}", format!("  Session ID: {}", session.session_id).green());
    println!("  Messages: {}", stats.total_messages);
    println!("  Tool calls: {}", stats.tool_calls);

    // Export
    let exporter = SessionExporter::new();

    match format {
        ClaudeExportFormat::Json => {
            if let Some(output_path) = output {
                let out_path = PathBuf::from(output_path);
                exporter.export_json(&session, &stats, &out_path, pretty)?;
                println!("{}", format!("  Exported to: {}", out_path.display()).green());
            } else {
                // Print to stdout
                let thread_data = ThreadData { /* build from session */ };
                let json = if pretty {
                    serde_json::to_string_pretty(&thread_data)?
                } else {
                    serde_json::to_string(&thread_data)?
                };
                println!("{}", json);
            }
        }
        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
            if let Some(output_path) = output {
                let out_path = PathBuf::from(output_path);
                exporter.export_markdown(&session, &stats, &out_path)?;
                println!("{}", format!("  Exported to: {}", out_path.display()).green());
            } else {
                // Print to stdout
                println!("{}", format!("Error: Markdown export requires --output option").red());
                return Err(anyhow::anyhow!("Markdown export requires output file"));
            }
        }
    }

    Ok(())
}

fn cmd_claude_list(claude_dir: String, main_only: bool) -> anyhow::Result<()> {
    use snps_core::claude::SessionAnalyzer;
    use std::path::PathBuf;

    println!("{}", "Claude Code Sessions".bright_blue());
    println!();

    let dir_path = PathBuf::from(shellexpand::tilde(&claude_dir).to_string());

    // Find all project directories
    for entry in std::fs::read_dir(&dir_path)? {
        let entry = entry?;
        let project_path = entry.path();

        if project_path.is_dir() {
            let project_name = project_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            println!("{}", format!("Project: {}", project_name).yellow());

            let analyzer = SessionAnalyzer::new(project_path.to_string_lossy().to_string());
            let hierarchy = analyzer.analyze_directory(&project_path)?;

            // Main sessions
            for session in &hierarchy.sessions {
                println!("  {} {}", "📄".dimmed(), session.session_id);
                println!("    {} messages, {} tool calls",
                    session.messages.len(),
                    session.metadata.tool_call_count);

                if !main_only && !session.child_agents.is_empty() {
                    for agent_id in &session.child_agents {
                        println!("      {} agent-{}", "└─".dimmed(), agent_id);
                    }
                }
            }

            println!();
        }
    }

    Ok(())
}

fn cmd_claude_analyze(session: String, detailed: bool) -> anyhow::Result<()> {
    use snps_core::claude::{SessionParser, SessionAnalyzer};
    use std::path::PathBuf;

    println!("{}", format!("Analyzing session: {}", session).bright_blue());
    println!();

    let file_path = PathBuf::from(shellexpand::tilde(&session).to_string());

    let parser = SessionParser::new(String::new());
    let session = parser.parse_file(&file_path)?;

    let analyzer = SessionAnalyzer::new(String::new());
    let stats = analyzer.analyze_session(&session);

    // Basic stats
    println!("{}", "Session Statistics".bright_green());
    println!("  Session ID: {}", session.session_id);
    println!("  Messages: {}", stats.total_messages);
    println!("  Tool Calls: {}", stats.tool_calls);
    if let Some(duration) = stats.duration_seconds {
        println!("  Duration: {} seconds", duration);
    }
    println!();

    // Tool usage
    if !stats.tool_usage.is_empty() {
        println!("{}", "Tool Usage".bright_green());
        let mut tools: Vec<_> = stats.tool_usage.iter().collect();
        tools.sort_by(|a, b| b.1.cmp(a.1));
        for (tool, count) in tools {
            println!("  {}: {}", tool, count);
        }
        println!();
    }

    if detailed {
        println!("{}", "Message Flow".bright_green());
        for node in &stats.message_flow {
            let role_str = node.role.as_ref()
                .map(|r| format!("{:?}", r))
                .unwrap_or_else(|| "System".to_string());
            let tools_str = if node.has_tools { " [tools]" } else { "" };
            println!("  {} {}{}", node.uuid, role_str, tools_str);
        }
    }

    Ok(())
}

fn cmd_claude_tree(session: String) -> anyhow::Result<()> {
    println!("{}", format!("Session tree for: {}", session).bright_blue());
    println!();
    println!("{}", "Tree view implementation coming in Phase 5".yellow());
    Ok(())
}
```

#### 2. Add Dependency: `engine/snps-cli/Cargo.toml`

**File**: `engine/snps-cli/Cargo.toml`
**Changes**: Add shellexpand dependency for tilde expansion

```toml
[dependencies]
# ... existing dependencies ...
shellexpand = "3.1"
```

### Success Criteria

#### Automated Verification:
- [ ] Code compiles: `cargo build -p snps-cli`
- [ ] CLI tests pass: `cargo test -p snps-cli`
- [ ] Help text works: `cargo run -p snps-cli -- claude --help`
- [ ] Clippy clean: `cargo clippy -p snps-cli --all-targets -- -D warnings`
- [ ] Format check: `cargo fmt --all -- --check`

#### Manual Verification:
- [ ] `snps claude parse <file.jsonl>` successfully parses and outputs JSON
- [ ] `snps claude parse <file.jsonl> --format markdown -o output.md` creates markdown file
- [ ] `snps claude list` shows all sessions in `~/.claude/projects/`
- [ ] `snps claude analyze <file>` shows statistics correctly
- [ ] `snps claude analyze <file> --detailed` shows message flow
- [ ] Command help texts are clear and accurate
- [ ] Error messages are helpful when files don't exist

**Implementation Note**: After completing this phase, run all commands on real session data. Verify outputs match expected format. Test error handling by providing invalid paths.

---

## Phase 5: Thoughts Integration & Session Storage

### Overview
Integrate with PMSynapse thoughts system to save parsed sessions and enable auto-sync.

### Changes Required

#### 1. Create Sessions Directory Structure

**Manual step**: Create directory structure
```bash
mkdir -p thoughts/shared/sessions
```

#### 2. Add Auto-Save Feature: `engine/snps-cli/src/main.rs`

**File**: `engine/snps-cli/src/main.rs`
**Changes**: Update parse command to optionally save to thoughts

```rust
// Update ClaudeCommands::Parse variant
Parse {
    /// Path to JSONL file or directory
    path: String,

    /// Output format
    #[arg(long, default_value = "json")]
    format: ClaudeExportFormat,

    /// Output file path (stdout if not specified)
    #[arg(short, long)]
    output: Option<String>,

    /// Pretty-print JSON output
    #[arg(long)]
    pretty: bool,

    /// Save to thoughts/shared/sessions/ directory
    #[arg(long)]
    save: bool,
},

// Update cmd_claude_parse signature and add save logic
fn cmd_claude_parse(
    path: String,
    format: ClaudeExportFormat,
    output: Option<String>,
    pretty: bool,
    save: bool,
) -> anyhow::Result<()> {
    // ... existing parsing code ...

    // Auto-save to thoughts if requested
    if save {
        let thoughts_dir = PathBuf::from("thoughts/shared/sessions");
        std::fs::create_dir_all(&thoughts_dir)?;

        let session_filename = format!("{}.{}",
            session.session_id,
            match format {
                ClaudeExportFormat::Json => "json",
                ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => "md",
            }
        );

        let save_path = thoughts_dir.join(session_filename);

        match format {
            ClaudeExportFormat::Json => {
                exporter.export_json(&session, &stats, &save_path, pretty)?;
            }
            ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
                exporter.export_markdown(&session, &stats, &save_path)?;
            }
        }

        println!("{}", format!("  Saved to: {}", save_path.display()).green());

        // Run thoughts sync
        println!("{}", "  Syncing thoughts...".dimmed());
        let sync_result = std::process::Command::new("snps")
            .args(&["thoughts", "sync"])
            .output();

        match sync_result {
            Ok(output) if output.status.success() => {
                println!("{}", "  ✓ Thoughts synced".green());
            }
            _ => {
                println!("{}", "  ⚠ Could not auto-sync thoughts (run 'snps thoughts sync' manually)".yellow());
            }
        }
    }

    // ... rest of function ...
}
```

#### 3. Add Batch Import Command

**File**: `engine/snps-cli/src/main.rs`
**Changes**: Add import command to batch-process sessions

```rust
// Add to ClaudeCommands enum
/// Import all sessions from Claude projects directory
Import {
    /// Claude projects directory
    #[arg(long, default_value = "~/.claude/projects")]
    claude_dir: String,

    /// Output format
    #[arg(long, default_value = "markdown")]
    format: ClaudeExportFormat,

    /// Only import main sessions (skip agents)
    #[arg(long)]
    main_only: bool,
},

// Add handler
ClaudeCommands::Import { claude_dir, format, main_only } => {
    cmd_claude_import(claude_dir, format, main_only)
}

fn cmd_claude_import(
    claude_dir: String,
    format: ClaudeExportFormat,
    main_only: bool,
) -> anyhow::Result<()> {
    use snps_core::claude::{SessionParser, SessionAnalyzer, SessionExporter};

    println!("{}", "Importing Claude Code sessions...".bright_blue());
    println!();

    let dir_path = PathBuf::from(shellexpand::tilde(&claude_dir).to_string());
    let thoughts_dir = PathBuf::from("thoughts/shared/sessions");
    std::fs::create_dir_all(&thoughts_dir)?;

    let exporter = SessionExporter::new();
    let mut imported_count = 0;

    for entry in std::fs::read_dir(&dir_path)? {
        let entry = entry?;
        let project_path = entry.path();

        if project_path.is_dir() {
            let analyzer = SessionAnalyzer::new(project_path.to_string_lossy().to_string());
            let hierarchy = analyzer.analyze_directory(&project_path)?;

            let sessions_to_import = if main_only {
                hierarchy.sessions
            } else {
                let mut all = hierarchy.sessions;
                all.extend(hierarchy.agents);
                all
            };

            for session in sessions_to_import {
                let parser = SessionParser::new(String::new());
                let stats = analyzer.analyze_session(&session);

                let filename = format!("{}.{}",
                    session.session_id,
                    match format {
                        ClaudeExportFormat::Json => "json",
                        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => "md",
                    }
                );

                let output_path = thoughts_dir.join(filename);

                match format {
                    ClaudeExportFormat::Json => {
                        exporter.export_json(&session, &stats, &output_path, true)?;
                    }
                    ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
                        exporter.export_markdown(&session, &stats, &output_path)?;
                    }
                }

                imported_count += 1;
                println!("  {} {}", "✓".green(), session.session_id);
            }
        }
    }

    println!();
    println!("{}", format!("Imported {} sessions", imported_count).bright_green());
    println!();
    println!("{}", "Running thoughts sync...".dimmed());

    std::process::Command::new("snps")
        .args(&["thoughts", "sync"])
        .status()?;

    Ok(())
}
```

### Success Criteria

#### Automated Verification:
- [ ] Code compiles: `cargo build -p snps-cli`
- [ ] Tests pass: `cargo test -p snps-cli`
- [ ] Clippy clean: `cargo clippy -p snps-cli --all-targets -- -D warnings`

#### Manual Verification:
- [ ] `snps claude parse <file> --save` creates file in `thoughts/shared/sessions/`
- [ ] Auto-sync runs after save (or shows warning if fails)
- [ ] `snps claude import` successfully imports all sessions
- [ ] `snps claude import --main-only` skips agent sessions
- [ ] Imported markdown files are readable and well-formatted
- [ ] `snps thoughts list` shows imported sessions
- [ ] Session files can be searched with `snps thoughts search`

**Implementation Note**: After completing this phase, run `snps claude import --main-only` to import all main sessions. Verify they appear in thoughts system and are searchable.

---

## Testing Strategy

### Unit Tests

**File**: `engine/snps-core/tests/claude_parser_tests.rs` (new)

```rust
use snps_core::claude::{SessionParser, SessionAnalyzer};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_parse_main_session() {
    let sample_jsonl = r#"{"uuid":"test-uuid","type":"user","sessionId":"test-session","timestamp":"2025-12-12T20:00:00Z","message":{"role":"user","content":"test"}}"#;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test-session.jsonl");
    std::fs::write(&file_path, sample_jsonl).unwrap();

    let parser = SessionParser::new(String::new());
    let session = parser.parse_file(&file_path).unwrap();

    assert_eq!(session.session_id, "test-session");
    assert!(!session.is_agent);
    assert_eq!(session.messages.len(), 1);
}

#[test]
fn test_parse_agent_session() {
    let sample_jsonl = r#"{"uuid":"agent-uuid","isSidechain":true,"type":"assistant","sessionId":"parent-session","agentId":"abc123","timestamp":"2025-12-12T20:00:00Z","message":{"role":"assistant","content":"test"}}"#;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("agent-abc123.jsonl");
    std::fs::write(&file_path, sample_jsonl).unwrap();

    let parser = SessionParser::new(String::new());
    let session = parser.parse_file(&file_path).unwrap();

    assert!(session.is_agent);
    assert_eq!(session.agent_id, Some("abc123".to_string()));
    assert_eq!(session.parent_session_id, Some("parent-session".to_string()));
}

#[test]
fn test_tool_use_extraction() {
    let sample_jsonl = r#"{"uuid":"tool-msg","type":"assistant","timestamp":"2025-12-12T20:00:00Z","message":{"role":"assistant","content":[{"type":"tool_use","id":"tool-1","name":"Read","input":{"file_path":"/test.txt"}}]}}"#;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("session.jsonl");
    std::fs::write(&file_path, sample_jsonl).unwrap();

    let parser = SessionParser::new(String::new());
    let session = parser.parse_file(&file_path).unwrap();

    assert_eq!(session.messages[0].tool_uses.len(), 1);
    assert_eq!(session.messages[0].tool_uses[0].tool_name, "Read");
}

#[test]
fn test_session_hierarchy() {
    let dir = tempdir().unwrap();

    // Create main session
    std::fs::write(
        dir.path().join("main.jsonl"),
        r#"{"uuid":"main-msg","type":"user","sessionId":"main","message":{"role":"user","content":"test"}}"#
    ).unwrap();

    // Create agent session
    std::fs::write(
        dir.path().join("agent-a1.jsonl"),
        r#"{"uuid":"agent-msg","isSidechain":true,"sessionId":"main","agentId":"a1","message":{"role":"assistant","content":"test"}}"#
    ).unwrap();

    let analyzer = SessionAnalyzer::new(String::new());
    let hierarchy = analyzer.analyze_directory(dir.path()).unwrap();

    assert_eq!(hierarchy.sessions.len(), 1);
    assert_eq!(hierarchy.agents.len(), 1);
    assert_eq!(hierarchy.sessions[0].child_agents.len(), 1);
}
```

### Integration Tests

**File**: `engine/snps-cli/tests/claude_integration_tests.rs` (new)

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_claude_parse_json() {
    let dir = tempdir().unwrap();
    let session_file = dir.path().join("test.jsonl");
    std::fs::write(
        &session_file,
        r#"{"uuid":"test","type":"user","message":{"role":"user","content":"hello"}}"#
    ).unwrap();

    Command::cargo_bin("snps")
        .unwrap()
        .args(&["claude", "parse", session_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsing Claude Code session"));
}

#[test]
fn test_claude_list() {
    Command::cargo_bin("snps")
        .unwrap()
        .args(&["claude", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Claude Code Sessions"));
}

#[test]
fn test_claude_help() {
    Command::cargo_bin("snps")
        .unwrap()
        .args(&["claude", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage Claude Code sessions"));
}
```

### Manual Testing Steps

1. **Parse single session**:
   ```bash
   snps claude parse ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/6df416b8-9af7-4dc2-9c55-3179c4afb8a8.jsonl
   ```

2. **Export to markdown**:
   ```bash
   snps claude parse ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/6df416b8-9af7-4dc2-9c55-3179c4afb8a8.jsonl \
     --format markdown \
     -o session.md
   ```

3. **List all sessions**:
   ```bash
   snps claude list
   ```

4. **Analyze with details**:
   ```bash
   snps claude analyze ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/bc289ee8-ddb8-4e66-a0c9-358fa860c171.jsonl --detailed
   ```

5. **Import all sessions**:
   ```bash
   snps claude import --main-only
   ```

6. **Verify thoughts integration**:
   ```bash
   snps thoughts search "session"
   ```

## Performance Considerations

### Memory Usage
- JSONL files parsed line-by-line to avoid loading entire file into memory
- Use streaming JSON parser for large sessions (>100MB)
- Consider pagination for `list` command with many sessions

### File I/O
- Batch reads when analyzing directories
- Cache parsed sessions for repeated operations
- Use async I/O for parallel session processing (future enhancement)

### Export Optimization
- Template-based markdown generation for speed
- JSON serialization with serde (already optimized)
- Parallel export when processing multiple sessions

## Migration Notes

N/A - This is a new feature with no existing data to migrate.

## References

- Research session analysis: `/Users/igor/Dev/Helixoid/pmsynapse/claude_session_flow_analysis.json`
- Detailed flow mapping: `/Users/igor/Dev/Helixoid/pmsynapse/claude_session_detailed_flow.json`
- ampcode.com thread structure: https://ampcode.com/threads/T-24dca6a8-1a0a-4377-bfb2-5c4b77f8d3a9
- Claude Code JSONL format: `~/.claude/projects/<project-slug>/*.jsonl`
- PMSynapse CLI patterns: `engine/snps-cli/src/main.rs:10-2282`
- Thoughts system integration: `engine/snps-cli/src/main.rs:1253-1302`

---

*Plan created: 2025-12-12*
*For: PMSynapse - Claude Code Session Parser*
*Status: Ready for Implementation*
