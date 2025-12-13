//! Export functionality for Claude Code sessions
//!
//! Supports exporting to JSON (ampcode-style), Markdown, and HTML formats.

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

    /// Export session to HTML file
    pub fn export_html(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        output_path: &Path,
        author: Option<&str>,
    ) -> Result<()> {
        let html = self.build_html(session, stats, author);

        let mut file = File::create(output_path).map_err(SynapseError::Io)?;
        file.write_all(html.as_bytes())
            .map_err(SynapseError::Io)?;

        Ok(())
    }

    /// Export session to HTML string
    pub fn export_html_string(
        &self,
        session: &Session,
        stats: &SessionStatistics,
        author: Option<&str>,
    ) -> String {
        self.build_html(session, stats, author)
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

    /// Build standalone HTML representation (ampcode-style)
    fn build_html(&self, session: &Session, stats: &SessionStatistics, author: Option<&str>) -> String {
        let title = self.infer_title(session);
        let author_name = author.unwrap_or("User");
        let author_initials: String = author_name
            .split_whitespace()
            .take(2)
            .filter_map(|w| w.chars().next())
            .collect::<String>()
            .to_uppercase();

        let date = session
            .metadata
            .start_time
            .map(|t| t.format("%b %d, %Y").to_string())
            .unwrap_or_else(|| "Unknown date".to_string());

        let duration = session
            .metadata
            .duration_seconds
            .map(|s| self.format_duration(s))
            .unwrap_or_default();

        let git_branch = session.metadata.git_branch.clone().unwrap_or_default();

        // Build session JSON for embedding
        let thread_data = self.build_thread_data(session, stats);
        let session_json =
            serde_json::to_string(&thread_data).unwrap_or_else(|_| "null".to_string());

        // Include JavaScript from separate file (uses vanilla JS, no template literals)
        const JS_TEMPLATE: &str = include_str!("html_template.js");

        format!(
            r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} - Claude Session</title>
  <style>
    :root {{
      --background: 0 0% 100%;
      --foreground: 240 10% 3.9%;
      --card: 0 0% 100%;
      --card-foreground: 240 10% 3.9%;
      --muted: 240 4.8% 95.9%;
      --muted-foreground: 240 3.8% 46.1%;
      --accent: 240 4.8% 95.9%;
      --accent-foreground: 240 5.9% 10%;
      --border: 240 5.9% 90%;
      --input: 240 5.9% 90%;
      --radius: 0.5rem;
      --editor-background: 240 10% 4%;
      --hljs-function: #61afef;
      --hljs-keyword: #c678dd;
      --hljs-class: #e5c07b;
      --hljs-string: #98c379;
      --hljs-comment: #5c6370;
      --hljs-number: #d19a66;
    }}
    @media (prefers-color-scheme: dark) {{
      :root {{
        --background: 240 10% 3.9%;
        --foreground: 0 0% 98%;
        --card: 240 10% 3.9%;
        --card-foreground: 0 0% 98%;
        --muted: 240 3.7% 15.9%;
        --muted-foreground: 240 5% 64.9%;
        --accent: 240 3.7% 15.9%;
        --accent-foreground: 0 0% 98%;
        --border: 240 3.7% 15.9%;
        --input: 240 3.7% 15.9%;
      }}
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
      background: hsl(var(--background));
      color: hsl(var(--foreground));
      line-height: 1.6;
      -webkit-font-smoothing: antialiased;
    }}
    .session-viewer {{ max-width: 42rem; margin: 0 auto; padding: 0 1rem; }}
    .session-header {{ display: flex; flex-direction: column; gap: 0.5rem; padding: 2rem 0; text-align: center; }}
    .session-title {{ font-size: 1.5rem; font-weight: 500; letter-spacing: -0.025em; }}
    .session-meta {{ display: flex; align-items: center; justify-content: center; gap: 0.5rem; color: hsl(var(--muted-foreground)); font-size: 0.875rem; }}
    .session-meta .separator {{ opacity: 0.5; }}
    .avatar {{ width: 1.25rem; height: 1.25rem; border-radius: 50%; background: hsl(var(--muted)); display: flex; align-items: center; justify-content: center; font-size: 0.625rem; font-weight: 500; flex-shrink: 0; }}
    .thread-container {{ display: flex; flex-direction: column; gap: 1rem; }}
    .message-block {{ position: relative; }}
    .block-link {{ position: absolute; left: -1.5rem; top: 0.25rem; opacity: 0; transition: opacity 0.2s; color: hsl(var(--muted-foreground)); text-decoration: none; }}
    .message-block:hover .block-link {{ opacity: 1; }}
    .user-message {{ display: flex; align-items: flex-start; gap: 0.5rem; }}
    .user-message .content {{ flex: 1; padding: 0.5rem 0.75rem; border: 1px solid hsl(var(--border)); border-radius: 0.25rem; white-space: pre-wrap; word-break: break-word; }}
    .file-chips {{ display: flex; flex-wrap: wrap; gap: 0.25rem; margin-bottom: 0.5rem; }}
    .file-chip {{ display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.125rem 0.5rem; background: hsl(var(--muted)); border: 1px solid hsl(var(--border)); border-radius: 0.25rem; font-size: 0.75rem; color: hsl(var(--muted-foreground)); }}
    .file-chip svg {{ width: 0.75rem; height: 0.75rem; }}
    .assistant-message {{ display: flex; flex-direction: column; gap: 0.5rem; }}
    .thinking-block {{ border-radius: 0.25rem; }}
    .thinking-trigger {{ display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.25rem 0.5rem; background: transparent; border: none; color: hsl(var(--muted-foreground)); cursor: pointer; font-size: 0.875rem; }}
    .thinking-trigger:hover {{ background: hsl(var(--muted)); }}
    .thinking-trigger svg {{ width: 1rem; height: 1rem; transition: transform 0.2s; }}
    .thinking-trigger[aria-expanded="true"] svg:first-child {{ transform: rotate(90deg); }}
    .thinking-content {{ display: none; margin-top: 0.25rem; padding: 0.75rem; background: hsl(var(--card)); border: 1px solid hsl(var(--border)); border-radius: 0.25rem; color: hsl(var(--muted-foreground)); font-size: 0.875rem; }}
    .thinking-content.expanded {{ display: block; }}
    .tool-chip {{ display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.375rem 0.5rem; background: hsl(var(--card) / 0.6); border: 1px solid hsl(var(--border) / 0.8); border-radius: 0.375rem; font-size: 0.875rem; cursor: pointer; }}
    .tool-chip:hover {{ background: hsl(var(--muted)); }}
    .tool-chip svg {{ width: 1rem; height: 1rem; color: hsl(var(--muted-foreground)); }}
    .tool-chip .tool-name {{ color: hsl(var(--muted-foreground)); }}
    .tool-chip .file-path {{ display: flex; align-items: baseline; }}
    .tool-chip .path-prefix {{ opacity: 0.6; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
    .tool-chip .path-separator {{ opacity: 0.6; }}
    .tool-status {{ padding: 0.125rem 0.375rem; border-radius: 0.25rem; font-size: 0.75rem; font-weight: 500; }}
    .tool-status.success {{ background: hsl(142 76% 36% / 0.15); color: hsl(142 76% 45%); }}
    .tool-status.error {{ background: hsl(0 84% 60% / 0.15); color: hsl(0 84% 60%); }}
    .tool-details {{ display: none; margin-top: 0.5rem; padding: 0.75rem; border-top: 1px solid hsl(var(--border) / 0.5); }}
    .tool-details.expanded {{ display: block; }}
    .tool-details-label {{ font-size: 0.75rem; color: hsl(var(--muted-foreground)); margin-bottom: 0.5rem; }}
    .markdown h1 {{ font-size: 1.75em; font-weight: 700; border-bottom: 2px solid hsl(var(--border)); margin: 1.8em 0 0.7em; padding-bottom: 0.4em; }}
    .markdown h1:first-child {{ margin-top: 0; }}
    .markdown h2 {{ font-size: 1.5em; font-weight: 700; margin: 1.5em 0 0.6em; }}
    .markdown h2:first-child {{ margin-top: 0; }}
    .markdown h3 {{ font-size: 1.3em; font-weight: 600; margin: 1.4em 0 0.5em; }}
    .markdown h3:first-child {{ margin-top: 0; }}
    .markdown p {{ margin: 0.75em 0; }}
    .markdown p:first-child {{ margin-top: 0; }}
    .markdown p:last-child {{ margin-bottom: 0; }}
    .markdown ul, .markdown ol {{ padding-left: 1.5rem; margin: 0.5em 0 0.75em; }}
    .markdown ul {{ list-style-type: disc; }}
    .markdown ol {{ list-style-type: decimal; }}
    .markdown li {{ margin: 0.4em 0; }}
    .markdown code {{ font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace; }}
    .markdown :not(pre) > code {{ background: hsl(var(--muted)); padding: 0.125rem 0.375rem; border-radius: 0.25rem; font-size: 0.875em; }}
    .markdown pre {{ background: hsl(var(--editor-background)); border-radius: calc(var(--radius) - 6px); padding: 0.75rem; overflow-x: auto; margin: 0.75em 0; }}
    .markdown pre code {{ background: transparent; padding: 0; font-size: 0.875rem; color: #e5e7eb; }}
    .markdown a {{ color: hsl(217 91% 60%); text-decoration: underline; text-underline-offset: 2px; }}
    .markdown a:hover {{ text-decoration: none; }}
    .markdown strong {{ font-weight: 700; }}
    .markdown em {{ font-style: italic; }}
    .code-block {{ background: hsl(240 10% 4%); border-radius: 0.5rem; overflow: hidden; margin: 0.75em 0; }}
    .code-block-header {{ display: flex; align-items: center; justify-content: space-between; padding: 0.5rem 1rem; border-bottom: 1px solid hsl(240 5% 20%); }}
    .code-block-lang {{ font-size: 0.75rem; color: hsl(240 5% 60%); }}
    .code-block-copy {{ padding: 0.25rem; background: transparent; border: none; color: hsl(240 5% 60%); cursor: pointer; }}
    .code-block-copy:hover {{ color: #fff; }}
    .code-block pre {{ margin: 0; padding: 1rem; overflow-x: auto; }}
    .code-block code {{ font-size: 0.875rem; color: #e5e7eb; }}
    .hljs-keyword {{ color: var(--hljs-keyword); }}
    .hljs-built_in, .hljs-function {{ color: var(--hljs-function); }}
    .hljs-string {{ color: var(--hljs-string); }}
    .hljs-number {{ color: var(--hljs-number); }}
    .hljs-comment {{ color: var(--hljs-comment); font-style: italic; }}
    .hljs-title, .hljs-class {{ color: var(--hljs-class); font-style: italic; }}
    .session-stats {{ margin-top: 2rem; padding: 1.5rem; background: hsl(var(--card)); border: 1px solid hsl(var(--border)); border-radius: 0.5rem; }}
    .session-stats h2 {{ font-size: 1rem; font-weight: 600; margin-bottom: 1rem; }}
    .stats-grid {{ display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin-bottom: 1.5rem; }}
    @media (min-width: 640px) {{ .stats-grid {{ grid-template-columns: repeat(4, 1fr); }} }}
    .stat-card {{ padding: 1rem; background: hsl(var(--muted) / 0.5); border-radius: 0.5rem; }}
    .stat-card .label {{ display: flex; align-items: center; gap: 0.5rem; font-size: 0.75rem; color: hsl(var(--muted-foreground)); margin-bottom: 0.25rem; }}
    .stat-card .value {{ font-size: 1.25rem; font-weight: 700; }}
    .tool-usage {{ display: flex; flex-wrap: wrap; gap: 0.5rem; }}
    .tool-usage-item {{ display: flex; align-items: center; gap: 0.5rem; padding: 0.25rem 0.75rem; background: hsl(var(--muted)); border-radius: 9999px; }}
    .tool-usage-item .name {{ font-size: 0.875rem; font-weight: 500; }}
    .tool-usage-item .count {{ padding: 0.125rem 0.5rem; background: hsl(var(--background)); border-radius: 9999px; font-size: 0.75rem; }}
    .message-nav {{ position: fixed; top: 50%; left: 1rem; transform: translateY(-50%); display: none; flex-direction: column; padding: 0.25rem 0; }}
    @media (min-width: 1024px) {{ .message-nav {{ display: flex; }} }}
    .nav-dot {{ display: flex; align-items: center; gap: 0.125rem; padding: 0.1875rem 0; cursor: pointer; background: none; border: none; }}
    .nav-dot .dot {{ width: 0.125rem; height: 0.125rem; background: currentColor; border-radius: 50%; opacity: 0.3; }}
    .nav-dot .line {{ width: 1.5rem; height: 1px; background: currentColor; border-radius: 1px; opacity: 0.3; transform: scaleX(0.8); transform-origin: left; }}
    .nav-dot:hover .dot, .nav-dot:hover .line {{ opacity: 1; }}
  </style>
</head>
<body>
  <div class="session-viewer">
    <nav class="message-nav" id="messageNav"></nav>
    <header class="session-header">
      <h1 class="session-title">{title}</h1>
      <div class="session-meta">
        <span class="avatar">{author_initials}</span>
        <span>{author_name}</span>
        <span class="separator">•</span>
        <span>{date}</span>
        {git_branch_html}
      </div>
    </header>
    <div class="thread-container" id="threadContainer"></div>
    <div class="session-stats">
      <h2>Session Statistics</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="label">Messages</div>
          <div class="value">{message_count}</div>
        </div>
        <div class="stat-card">
          <div class="label">Tool Calls</div>
          <div class="value">{tool_call_count}</div>
        </div>
        {duration_html}
      </div>
      <h3 style="font-size: 0.875rem; color: hsl(var(--muted-foreground)); margin-bottom: 0.75rem;">Tool Usage</h3>
      <div class="tool-usage" id="toolUsage"></div>
    </div>
  </div>
  <script>
    var sessionData = {session_json};
    {js_template}
  </script>
</body>
</html>"##,
            title = self.escape_html(&title),
            author_initials = author_initials,
            author_name = self.escape_html(author_name),
            date = date,
            git_branch_html = if !git_branch.is_empty() {
                format!(
                    r#"<span class="separator">•</span><span>{}</span>"#,
                    self.escape_html(&git_branch)
                )
            } else {
                String::new()
            },
            message_count = stats.total_messages,
            tool_call_count = stats.tool_calls,
            duration_html = if !duration.is_empty() {
                format!(
                    r#"<div class="stat-card"><div class="label">Duration</div><div class="value">{}</div></div>"#,
                    duration
                )
            } else {
                String::new()
            },
            session_json = session_json,
            js_template = JS_TEMPLATE,
        )
    }

    fn format_duration(&self, seconds: i64) -> String {
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            format!("{}m {}s", seconds / 60, seconds % 60)
        } else {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        }
    }

    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
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
