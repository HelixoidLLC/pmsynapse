# Claude Code Session Parser Documentation

**Created**: 2025-12-12
**Last Updated**: 2025-12-12
**Status**: Production Ready

## Table of Contents

1. [Overview](#overview)
2. [Problem Discovery](#problem-discovery)
3. [Research & Root Cause Analysis](#research--root-cause-analysis)
4. [Implementation Plan](#implementation-plan)
5. [Fixes Applied](#fixes-applied)
6. [Session ID Lookup Enhancement](#session-id-lookup-enhancement)
7. [Usage Guide](#usage-guide)
8. [Architecture](#architecture)
9. [File Format Reference](#file-format-reference)

---

## Overview

The Claude Code Session Parser is a comprehensive system for parsing, analyzing, and exporting Claude Code session data from JSONL files stored in `~/.claude/projects/`. It enables capturing rich conversation history including tool calls, agent hierarchies, and file changes for use in project documentation, tickets, and analysis.

### Key Features

- **Session Parsing**: Parse JSONL session files into structured data
- **Title Extraction**: Automatic title extraction from summary records
- **Message Counting**: Accurate message counting excluding tool results
- **Agent Hierarchy**: Track parent-child relationships between main sessions and agents
- **Statistics**: Tool usage statistics and message flow analysis
- **Multiple Export Formats**: JSON (ampcode-style), Markdown, and HTML
- **HTML Export**: Beautiful, ampcode.com-inspired HTML rendering with light/dark mode
- **Session ID Lookup**: Flexible session resolution using full/partial IDs or paths
- **Thoughts Integration**: Save parsed sessions to thoughts directory
- **Format Conversion**: Convert between JSON, Markdown, and HTML formats

### Commands

```bash
snps claude list        # List sessions
snps claude parse       # Parse and export session
snps claude convert     # Convert between formats (JSON → HTML/Markdown)
snps claude analyze     # Analyze session hierarchy
snps claude tree        # Display message tree (planned)
snps claude import      # Batch import to thoughts
```

---

## Problem Discovery

### Initial Symptom

When running `snps claude list --all`, the output differed significantly from Claude Code's native `/resume` command:

**snps claude list output:**
```
❯ (No title)
  16 minutes ago · 4 messages · claude/setup-ai-research-tools...
⚡ (Agent session)
  18 minutes ago · 2 messages · claude/setup-ai-research-tools...
❯ (No title)
  24 minutes ago · 13 messages · claude/setup-ai-research-tools...
```

**claude /resume output:**
```
❯ CLI rebuild for new snps command options
  1 minute ago · 3 messages · claude/setup-ai-research-tools...

  UI Location and Tri-Platform Architecture
  5 hours ago · 106 messages · claude/setup-ai-research-tools...

  Claude session parser implementation plan
  6 hours ago · 88 messages · claude/setup-ai-research-tools...
```

### Key Discrepancies

1. **Titles**: Most sessions showed "(No title)" instead of actual titles
2. **Message Counts**: Different counts (e.g., 4 vs 3, 13 vs 12, 38 vs 34)
3. **Session Display**: Different ordering and formatting

---

## Research & Root Cause Analysis

### Research Process

We conducted comprehensive research by:

1. **Reading the implementation plan** (`thoughts/shared/plans/2025-12-12-claude-session-parser.md`)
2. **Analyzing the claude module** in `snps-core` (models, parser, analyzer, export)
3. **Examining the CLI implementation** in `snps-cli`
4. **Inspecting actual JSONL files** to understand the data format

### Key Discovery: Summary Records

Claude Code JSONL files contain `type: "summary"` records that explicitly store session titles:

```json
{
  "type": "summary",
  "summary": "CLI rebuild for new snps command options",
  "leafUuid": "b9cf28d7-8eac-4c60-9e0c-69fde59a84d1"
}
```

**Location**: The summary record is typically the **first line** in the JSONL file.

### Root Causes Identified

#### 1. Incorrect Title Source

**Problem**: The `extract_session_info()` function extracted titles from the first user message text instead of the summary record.

**Original logic** (`main.rs:2639-2669`):
- Looked for first non-meta user message
- Extracted text from `message.content` array
- Skipped messages starting with `<` (system messages)
- Skipped messages containing `<command` (command invocations)
- Many sessions start with `/resume` or other commands → "(No title)"

**Why it failed**:
- Sessions often start with command invocations like `/resume`
- These get filtered out as system messages
- Result: No title found → "(No title)"

#### 2. Incorrect Message Counting

**Problem**: Message counting included tool result records and meta messages.

**Original logic** (`main.rs:2633-2637`):
```rust
if msg_type == Some("user") || msg_type == Some("assistant") {
    message_count += 1;
}
```

**What got incorrectly counted**:
- Tool result records (`type: "user"` with `tool_result` content)
- System-generated user messages
- Meta messages (`isMeta: true`)

**Claude's method**: Likely counts only human-initiated user/assistant exchanges, excluding tool infrastructure.

### JSONL Structure Analysis

We analyzed the actual JSONL structure and found these record types:

| Record Type | Contains Title | Should Count as Message |
|-------------|---------------|------------------------|
| `summary` | **Yes** - in `summary` field | No |
| `user` (human) | First line can be title | Yes |
| `user` (tool_result) | No | **No** (was incorrectly Yes) |
| `assistant` | No | Yes |
| `system` | No | No |
| `file-history-snapshot` | No | No |

---

## Implementation Plan

The implementation followed a phased approach defined in `thoughts/shared/plans/2025-12-12-claude-session-parser.md`:

### Phase 1: Core Data Model & Parser
- Created data structures in `snps-core/src/claude/models.rs`
- Implemented JSONL parser in `snps-core/src/claude/parser.rs`
- Built session hierarchy analyzer in `snps-core/src/claude/analyzer.rs`

### Phase 2: Export Engine
- Implemented JSON/Markdown exporters in `snps-core/src/claude/export.rs`
- Created ampcode-style thread data structures
- Added title inference from first user message

### Phase 3: CLI Integration
- Added `snps claude` subcommand in `snps-cli/src/main.rs`
- Implemented list, parse, analyze, tree, import commands
- Added project auto-detection from CWD

### Phase 4: Fixes & Enhancements
- Fixed title extraction to use summary records
- Fixed message counting to exclude tool results
- Added session ID lookup for convenience

---

## Fixes Applied

### Fix 1: Summary Record Title Extraction

**File**: `engine/snps-cli/src/main.rs:2635-2641`

**Implementation**:
```rust
// Extract title from summary record (preferred method)
if title.is_none() && msg_type == Some("summary") {
    title = record
        .get("summary")
        .and_then(|v| v.as_str())
        .map(String::from);
}
```

**Strategy**:
1. Check for `type: "summary"` records first
2. Extract `summary` field value as title
3. Fall back to first user message if no summary found
4. Maintain backwards compatibility with sessions lacking summary records

### Fix 2: Accurate Message Counting

**File**: `engine/snps-cli/src/main.rs:2643-2663`

**Implementation**:
```rust
// Count messages (exclude tool results and meta messages)
if msg_type == Some("user") || msg_type == Some("assistant") {
    // Check if it's a tool result (user message with tool_result content)
    let is_tool_result = msg_type == Some("user")
        && record
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .any(|item| item.get("type").and_then(|t| t.as_str()) == Some("tool_result"))
            })
            .unwrap_or(false);

    // Skip meta messages and tool results
    let is_meta = record.get("isMeta").and_then(|v| v.as_bool()).unwrap_or(false);

    if !is_tool_result && !is_meta {
        message_count += 1;
    }
}
```

**What's excluded**:
- Tool result records (user messages containing tool_result content)
- Meta messages (records with `isMeta: true`)
- System messages (already excluded by type check)

### Fix 3: Fallback Behavior

**File**: `engine/snps-cli/src/main.rs:2665-2696`

Sessions without summary records (very new or interrupted sessions) still get title extraction:
1. Looks for first non-meta user message
2. Extracts text content
3. Skips command messages (starting with `<` or containing `<command`)
4. Truncates to 60 characters with "..." suffix
5. Falls back to "(No title)" if nothing found

### Results After Fixes

**Before:**
```
❯ (No title)
  16 minutes ago · 4 messages
```

**After:**
```
❯ CLI rebuild for new snps command options
  16 minutes ago · 3 messages
```

**Verified matches** with `claude /resume`:
- ✓ "CLI rebuild for new snps command options"
- ✓ "UI Location and Tri-Platform Architecture"
- ✓ "Claude session parser implementation plan"
- ✓ "Build process and installation guide"

---

## Session ID Lookup Enhancement

After fixing the core issues, we added a convenience feature to allow using session IDs instead of full file paths.

### Problem

Users had to provide full paths to session files:
```bash
snps claude parse ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl
```

This was cumbersome and not user-friendly.

### Solution: Flexible Session Resolution

**File**: `engine/snps-cli/src/main.rs:2447-2505`

Added two helper functions:

#### 1. `resolve_session_path(id_or_path: &str)`

**Algorithm**:
1. Expand tilde in input path
2. If path exists, return it (already a valid path)
3. Otherwise, treat as session ID:
   - Search current project directory first
   - Then search all projects in `~/.claude/projects/`
4. Return first match or error

**Code**:
```rust
fn resolve_session_path(id_or_path: &str) -> anyhow::Result<PathBuf> {
    let path = expand_path(id_or_path);

    // If it's already a valid path, use it
    if path.exists() {
        return Ok(path);
    }

    // Otherwise, treat it as a session ID and search for it
    let sessions_dir = get_claude_sessions_dir();

    // Try current project first
    if let Some(project_dir) = get_claude_project_dir() {
        if let Some(found) = search_session_in_dir(&project_dir, id_or_path)? {
            return Ok(found);
        }
    }

    // Search all projects
    for entry in std::fs::read_dir(&sessions_dir)? {
        let entry = entry?;
        let project_path = entry.path();

        if project_path.is_dir() {
            if let Some(found) = search_session_in_dir(&project_path, id_or_path)? {
                return Ok(found);
            }
        }
    }

    Err(anyhow::anyhow!(
        "Session not found: {}. Searched in {} projects.",
        id_or_path,
        sessions_dir.display()
    ))
}
```

#### 2. `search_session_in_dir(dir: &Path, id: &str)`

**Matching logic**:
1. Exact prefix match: `id` matches start of filename
2. Partial match: `id` appears anywhere in filename
3. Must end with `.jsonl`

**Example matches**:
- Input: `"0c721d51"`
- Matches: `"0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl"`
- Input: `"2e90adc4"`
- Matches: `"2e90adc4-4a84-4086-858d-f7af134de790.jsonl"`

### Integration

Updated `cmd_claude_parse()` to use the resolver:

**Before**:
```rust
let file_path = expand_path(&file);

if !file_path.exists() {
    println!("Session file not found: {}", file_path.display());
    return Ok(());
}
```

**After**:
```rust
let file_path = resolve_session_path(&file)?;
```

Now returns clear error messages if session not found:
```
Session not found: xyz123. Searched in /Users/igor/.claude/projects projects.
```

---

## HTML Export Feature

### Overview

The HTML export feature provides a beautiful, standalone HTML rendering of Claude Code sessions inspired by [ampcode.com's session viewer](https://ampcode.com/threads/T-24dca6a8-1a0a-4377-bfb2-5c4b77f8d3a9).

### Design Features

**Visual Design**:
- ampcode.com-inspired aesthetic with clean, modern styling
- Automatic light/dark mode support via CSS `prefers-color-scheme`
- Responsive layout optimized for reading (max-width: 42rem)
- Message navigation dots on desktop (left sidebar on wide screens)

**Interactive Elements**:
- Collapsible thinking blocks with expand/collapse buttons
- Expandable tool use details showing input/output
- Clickable message anchors for direct linking
- Smooth scrolling navigation

**Content Rendering**:
- Markdown rendering (headers, code blocks, lists, bold, inline code)
- Syntax-highlighted code blocks with dark theme
- File path chips for file operations (Read, Write, Edit)
- Tool use icons with visual status indicators (✓ success, ✗ error)

**Session Statistics Panel**:
- Message count, tool call count, duration
- Tool usage breakdown with counts
- Session metadata (author, date, git branch)

### Implementation Details

**File**: `engine/snps-core/src/claude/export.rs`

**Key Components**:
1. `export_html()` - Export session to HTML file
2. `export_html_string()` - Get HTML as string
3. `build_html()` - Construct complete HTML document
4. `html_template.js` - Vanilla JavaScript rendering logic (separate file)

**Why Separate JS File**:
- Rust string escaping issues with JavaScript template literals
- Uses `include_str!()` macro to embed at compile time
- Vanilla JS (no template literals) for compatibility

**HTML Structure**:
```html
<!DOCTYPE html>
<html>
  <head>
    <style>/* CSS custom properties for theming */</style>
  </head>
  <body>
    <div class="session-viewer">
      <nav class="message-nav"><!-- Navigation dots --></nav>
      <header class="session-header"><!-- Title, author, metadata --></header>
      <div class="thread-container"><!-- Messages rendered here --></div>
      <div class="session-stats"><!-- Statistics panel --></div>
    </div>
    <script>
      var sessionData = {...}; // Embedded JSON
      // JavaScript from html_template.js
    </script>
  </body>
</html>
```

### Workflow

**Direct Export from JSONL**:
```bash
# Export session directly to HTML
snps claude parse <session-id> --format html -o session.html
```

**Two-Step Workflow (Recommended)**:
```bash
# Step 1: Parse session to JSON
snps claude parse <session-id> --format json -o session.json

# Step 2: Convert JSON to HTML
snps claude convert session.json --format html -o session.html

# Or convert to Markdown
snps claude convert session.json --format markdown -o session.md
```

### Why Two-Step Workflow?

The two-step workflow is recommended because:
1. **JSON as source of truth**: Store the structured session data once
2. **Multiple formats**: Generate both HTML and Markdown from same source
3. **Regeneration**: Easily regenerate HTML if template is updated
4. **Portability**: JSON can be used by other tools/scripts

### CSS Custom Properties

The HTML uses CSS custom properties for theming:
```css
:root {
  --background: 0 0% 100%;
  --foreground: 240 10% 3.9%;
  --muted: 240 4.8% 95.9%;
  --border: 240 5.9% 90%;
  --editor-background: 240 10% 4%;
}

@media (prefers-color-scheme: dark) {
  :root {
    --background: 240 10% 3.9%;
    --foreground: 0 0% 98%;
    /* ... dark mode colors */
  }
}
```

### JavaScript Rendering Functions

**File**: `engine/snps-core/src/claude/html_template.js`

**Key Functions**:
- `renderSession(session)` - Main entry point, renders entire session
- `renderMessage(message, index)` - Renders user/assistant messages
- `renderThinking(content, blockIndex)` - Collapsible thinking blocks
- `renderToolUse(tool, toolId)` - Tool chips with input/output
- `renderMarkdown(text)` - Simple markdown→HTML conversion
- `getToolIcon(name)` - SVG icons for different tools
- `toggleThinking(id)` - Toggle thinking block visibility
- `toggleToolDetails(id)` - Toggle tool details panel

---

## Usage Guide

### Quick Start

**1. List available sessions:**
```bash
snps claude list
```

**2. List all sessions (including agents):**
```bash
snps claude list --all
```

**3. Show recent 20 sessions:**
```bash
snps claude list --recent 20
```

### Parse & Export

**Parse with session ID:**
```bash
# Using first 8 characters of session ID
snps claude parse 0c721d51

# Using full session ID
snps claude parse 0c721d51-3a4f-4db0-b5c9-e364c9c55de4
```

**Export to Markdown:**
```bash
snps claude parse 2e90adc4 --format markdown -o session.md
```

**Export to JSON:**
```bash
snps claude parse 0c721d51 --format json -o session.json --pretty
```

**Export to HTML:**
```bash
snps claude parse 2e90adc4 --format html -o session.html
```

**Save to thoughts directory:**
```bash
snps claude parse 2e90adc4 --format markdown --save
# Saves to: thoughts/shared/sessions/2e90adc4-4a84-4086-858d-f7af134de790.md
```

### List Output Formats

**Table format (default):**
```bash
snps claude list
```
Output:
```
❯ CLI rebuild for new snps command options
  just now · 31 messages · claude/setup-ai-research-tools...

❯ UI Location and Tri-Platform Architecture
  4 hours ago · 24 messages · claude/setup-ai-research-tools...
```

**JSON format:**
```bash
snps claude list --format json
```
Output:
```json
[
  {
    "session_id": "0c721d51-3a4f-4db0-b5c9-e364c9c55de4",
    "path": "/Users/igor/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl",
    "title": "CLI rebuild for new snps command options",
    "message_count": 31,
    "git_branch": "claude/setup-ai-research-tools-01HBS3NWx65LJaJxW9Bmg6hJ",
    "is_agent": false
  }
]
```

**Paths format (for scripting):**
```bash
snps claude list --format paths
```
Output:
```
/Users/igor/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl
/Users/igor/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse/2e90adc4-4a84-4086-858d-f7af134de790.jsonl
```

### Analyze Session Hierarchy

**Analyze directory of sessions:**
```bash
snps claude analyze ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse
```

**With tree view:**
```bash
snps claude analyze ~/.claude/projects/-Users-igor-Dev-Helixoid-pmsynapse --tree
```

### Import to Thoughts

**Import all main sessions:**
```bash
snps claude import --main-only
```

**Import all sessions including agents:**
```bash
snps claude import
```

**Import specific project:**
```bash
snps claude import --project /Users/igor/Dev/Helixoid/pmsynapse
```

### Format Conversion

**Convert JSON to HTML:**
```bash
snps claude convert session.json --format html -o session.html
```

**Convert JSON to Markdown:**
```bash
snps claude convert session.json --format markdown -o session.md
```

**Output to stdout:**
```bash
# View HTML in terminal (or pipe to file)
snps claude convert session.json --format html
```

### Common Workflows

#### Workflow 1: Quick Session Analysis

```bash
# 1. List recent sessions
snps claude list

# 2. Copy first 8 chars of interesting session
# Example: 2e90adc4

# 3. Export to markdown for review
snps claude parse 2e90adc4 --format markdown -o /tmp/session.md

# 4. Open in editor
open /tmp/session.md
```

#### Workflow 2: HTML Export for Sharing

```bash
# 1. Parse session to JSON (source of truth)
snps claude parse 2e90adc4 --format json -o session.json

# 2. Generate HTML for viewing in browser
snps claude convert session.json --format html -o session.html

# 3. Open in browser
open session.html

# 4. Optionally generate Markdown too
snps claude convert session.json --format markdown -o session.md
```

#### Workflow 3: Archive Sessions to Thoughts

```bash
# Import all main sessions to thoughts
snps claude import --main-only --format markdown

# Verify import
snps thoughts search "session"
```

#### Workflow 4: Session Statistics

```bash
# Parse to JSON and extract statistics
snps claude parse 0c721d51 --format json | jq '.statistics'
```

Output:
```json
{
  "total_messages": 31,
  "tool_calls": 15,
  "tool_usage": {
    "Bash": 8,
    "Read": 4,
    "Edit": 3
  },
  "duration_seconds": 1200
}
```

---

## Architecture

### Module Structure

```
engine/
├── snps-core/
│   └── src/
│       └── claude/
│           ├── mod.rs           # Public exports
│           ├── models.rs        # Data structures
│           ├── parser.rs        # JSONL parsing
│           ├── analyzer.rs      # Hierarchy & stats
│           ├── export.rs        # JSON/Markdown/HTML export
│           └── html_template.js # HTML rendering JavaScript
└── snps-cli/
    └── src/
        └── main.rs              # CLI commands (lines 445-3500+)
```

### Data Flow

#### Parse Workflow
```
JSONL File
    ↓
SessionParser::parse_file()
    ↓
Session struct (with metadata, messages)
    ↓
SessionAnalyzer::analyze_session()
    ↓
SessionStatistics
    ↓
SessionExporter::export_*()
    ↓
JSON/Markdown/HTML output
```

#### Convert Workflow
```
JSON File (ThreadData)
    ↓
serde_json::from_str()
    ↓
ThreadData struct
    ↓
thread_data_to_session()
    ↓
Session struct
    ↓
SessionAnalyzer::analyze_session()
    ↓
SessionStatistics
    ↓
SessionExporter::export_html() / export_markdown()
    ↓
HTML/Markdown output
```

### Key Components

#### SessionParser (`snps-core/src/claude/parser.rs`)

**Responsibilities**:
- Read JSONL files line-by-line
- Deserialize JSON records
- Build Session structs with messages
- Extract metadata (timestamps, tool counts, etc.)
- Detect main vs agent sessions

**Key methods**:
- `parse_file(path) -> Result<Session>` - Parse single JSONL file
- `build_session(records) -> Result<Session>` - Convert records to Session
- `extract_metadata(records) -> SessionMetadata` - Aggregate metadata
- `build_messages(records) -> Vec<Message>` - Convert to message structs

#### SessionAnalyzer (`snps-core/src/claude/analyzer.rs`)

**Responsibilities**:
- Build session hierarchies (parent-child relationships)
- Compute statistics (tool usage, message flow)
- Analyze directories of sessions

**Key methods**:
- `analyze_directory(dir) -> SessionHierarchy` - Parse all sessions in dir
- `analyze_session(session) -> SessionStatistics` - Compute stats
- `build_hierarchy()` - Link agents to parent sessions

#### SessionExporter (`snps-core/src/claude/export.rs`)

**Responsibilities**:
- Export to JSON (ampcode-style format)
- Export to Markdown (human-readable)
- Export to HTML (standalone, browser-ready)
- Infer titles from sessions
- Build thread data structures for export

**Key methods**:
- `export_json(session, stats, output_path, pretty)` - JSON export
- `export_json_string(session, stats, pretty)` - JSON as string
- `export_markdown(session, stats, output_path)` - Markdown export
- `export_markdown_string(session, stats)` - Markdown as string
- `export_html(session, stats, output_path, author)` - HTML export
- `export_html_string(session, stats, author)` - HTML as string
- `build_html(session, stats, author)` - Construct HTML document
- `build_thread_data(session, stats)` - Build ThreadData structure
- `infer_title(session) -> String` - Extract session title
- `format_duration(seconds)` - Format duration (e.g., "1h 23m")
- `escape_html(text)` - HTML entity escaping

#### CLI Commands (`snps-cli/src/main.rs`)

**Command handlers**:
- `cmd_claude_list()` - List sessions
- `cmd_claude_parse()` - Parse JSONL and export to JSON/Markdown/HTML
- `cmd_claude_convert()` - Convert JSON to HTML/Markdown
- `cmd_claude_analyze()` - Analyze session hierarchy
- `cmd_claude_import()` - Batch import to thoughts

**Helper functions**:
- `resolve_session_path()` - Session ID lookup
- `search_session_in_dir()` - Search for session files
- `extract_session_info()` - Extract metadata quickly
- `get_claude_project_dir()` - Auto-detect current project
- `thread_data_to_session()` - Convert ThreadData back to Session

### Data Models

#### Session (`models.rs:10-18`)
```rust
pub struct Session {
    pub session_id: String,              // UUID or extracted ID
    pub is_agent: bool,                  // Agent vs main session
    pub agent_id: Option<String>,        // Agent identifier
    pub parent_session_id: Option<String>, // Parent session
    pub metadata: SessionMetadata,       // Aggregated metadata
    pub messages: Vec<Message>,          // All messages
    pub child_agents: Vec<String>,       // Spawned agents
}
```

#### SessionMetadata (`models.rs:21-32`)
```rust
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
```

#### Message (`models.rs:35-45`)
```rust
pub struct Message {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub is_sidechain: bool,
    pub message_type: MessageType,       // User/Assistant/System/Summary
    pub role: Option<MessageRole>,       // User/Assistant/System
    pub timestamp: Option<DateTime<Utc>>,
    pub content: MessageContent,
    pub tool_uses: Vec<ToolUse>,
}
```

---

## File Format Reference

### JSONL Structure

Claude Code stores sessions as JSON Lines format (`.jsonl`). Each line is a complete JSON record.

### Record Types

#### 1. Summary Record
```json
{
  "type": "summary",
  "summary": "Session title here",
  "leafUuid": "b9cf28d7-8eac-4c60-9e0c-69fde59a84d1"
}
```
**Location**: First line of file
**Purpose**: Stores session title

#### 2. User Message Record
```json
{
  "type": "user",
  "parentUuid": null,
  "sessionId": "0c721d51-3a4f-4db0-b5c9-e364c9c55de4",
  "isSidechain": false,
  "userType": "external",
  "cwd": "/Users/igor/Dev/Helixoid/pmsynapse",
  "version": "2.0.69",
  "gitBranch": "claude/setup-ai-research-tools-01HBS3NWx65LJaJxW9Bmg6hJ",
  "message": {
    "role": "user",
    "content": "check the latest commit"
  },
  "uuid": "98cab19a-d5ef-401d-88ff-4ad3acf5e932",
  "timestamp": "2025-12-13T03:10:20.623Z",
  "isMeta": false
}
```

#### 3. Assistant Message Record
```json
{
  "type": "assistant",
  "parentUuid": "98cab19a-d5ef-401d-88ff-4ad3acf5e932",
  "sessionId": "0c721d51-3a4f-4db0-b5c9-e364c9c55de4",
  "message": {
    "model": "claude-sonnet-4-5-20250929",
    "role": "assistant",
    "content": [
      {
        "type": "text",
        "text": "Let me check the latest commit..."
      }
    ],
    "usage": {
      "input_tokens": 10,
      "output_tokens": 50
    }
  },
  "uuid": "b9dc5fd6-b42e-4046-92d4-e3a5bcc732c5",
  "timestamp": "2025-12-13T03:10:24.409Z"
}
```

#### 4. Tool Use Record
```json
{
  "type": "assistant",
  "message": {
    "content": [
      {
        "type": "tool_use",
        "id": "toolu_01PWwA2TmRCynCxPnnYpiWLG",
        "name": "Bash",
        "input": {
          "command": "git log -1 --stat",
          "description": "Show latest commit details"
        }
      }
    ]
  }
}
```

#### 5. Tool Result Record
```json
{
  "type": "user",
  "message": {
    "role": "user",
    "content": [
      {
        "tool_use_id": "toolu_01PWwA2TmRCynCxPnnYpiWLG",
        "type": "tool_result",
        "content": "commit c89ab8d...\n..."
      }
    ]
  },
  "toolUseResult": {
    "stdout": "...",
    "stderr": "",
    "interrupted": false
  }
}
```

#### 6. System Command Record
```json
{
  "type": "system",
  "subtype": "local_command",
  "content": "<command-name>/resume</command-name>",
  "level": "info",
  "timestamp": "2025-12-13T03:19:02.566Z",
  "uuid": "2785114a-3fbf-43c9-9989-6926db8e05fe",
  "isMeta": false
}
```

#### 7. File History Snapshot
```json
{
  "type": "file-history-snapshot",
  "messageId": "98cab19a-d5ef-401d-88ff-4ad3acf5e932",
  "snapshot": {
    "messageId": "98cab19a-d5ef-401d-88ff-4ad3acf5e932",
    "trackedFileBackups": {},
    "timestamp": "2025-12-13T03:10:20.637Z"
  },
  "isSnapshotUpdate": false
}
```

### Common Fields

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | String | Unique message identifier |
| `parentUuid` | String? | Parent message for threading |
| `sessionId` | String | Session this belongs to |
| `agentId` | String? | Agent ID if sidechain |
| `isSidechain` | Boolean | True for agent sessions |
| `timestamp` | String | ISO 8601 timestamp |
| `type` | String | Record type |
| `isMeta` | Boolean | System-generated message |
| `cwd` | String? | Working directory |
| `version` | String? | Claude Code version |
| `gitBranch` | String? | Git branch name |

### File Naming Conventions

**Main sessions**: `{uuid}.jsonl`
- Example: `0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl`
- Length: 36 chars (UUID) + 6 chars (`.jsonl`) = 42 chars

**Agent sessions**: `agent-{8-char-hex}.jsonl`
- Example: `agent-a5b0da3.jsonl`
- Prefix: `agent-`
- ID: 7-8 hexadecimal characters

### Directory Structure

```
~/.claude/projects/
├── -Users-igor-Dev-Helixoid-pmsynapse/         # Project directory
│   ├── 0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl  # Main session
│   ├── 2e90adc4-4a84-4086-858d-f7af134de790.jsonl  # Main session
│   ├── agent-a5b0da3.jsonl                          # Agent session
│   └── agent-a9c6145.jsonl                          # Agent session
└── -Users-igor-Dev-Other-project/
    └── ...
```

**Project path transformation**:
- Original: `/Users/igor/Dev/Helixoid/pmsynapse`
- Transformed: `-Users-igor-Dev-Helixoid-pmsynapse`
- Rule: Replace `/` with `-`

---

## Development Notes

### Building

```bash
# Build core library
cargo build -p snps-core

# Build CLI tool
cargo build -p snps-cli --release

# Run tests
cargo test -p snps-core
cargo test -p snps-cli

# Lint
cargo clippy -p snps-core -p snps-cli --all-targets -- -D warnings
```

### Adding New Features

**To add a new export format**:
1. Add variant to `ClaudeExportFormat` enum in `main.rs`
2. Implement export methods in `SessionExporter`:
   - `export_<format>(session, stats, output_path)`
   - `export_<format>_string(session, stats)`
3. Update `cmd_claude_parse()` match statement to handle new format
4. Update `cmd_claude_convert()` match statement if applicable
5. Add tests in `export.rs`

**To add a new command**:
1. Add variant to `ClaudeCommands` enum
2. Implement `cmd_claude_<name>()` function
3. Add dispatch in `cmd_claude()` match statement
4. Update documentation

**To modify HTML template**:
1. Edit `engine/snps-core/src/claude/html_template.js`
2. Use vanilla JavaScript (no template literals for object keys)
3. Rebuild with `cargo build -p snps-core`
4. Test with `snps claude parse <id> --format html`

### Testing

**Manual testing checklist**:
- [ ] `snps claude list` shows titles correctly
- [ ] `snps claude list --all` includes agent sessions
- [ ] `snps claude parse <id>` resolves session ID
- [ ] `snps claude parse <id> --format markdown` creates readable output
- [ ] `snps claude parse <id> --format html` creates valid HTML
- [ ] `snps claude parse <id> --format json` creates valid JSON
- [ ] `snps claude convert <json> --format html` works correctly
- [ ] `snps claude convert <json> --format markdown` works correctly
- [ ] HTML output renders in browser with light/dark mode
- [ ] HTML thinking blocks are collapsible
- [ ] HTML tool details are expandable
- [ ] `snps claude parse <id> --save` stores in thoughts directory
- [ ] Message counts match Claude Code's `/resume`
- [ ] Partial session IDs work (first 8 chars)

---

## Troubleshooting

### Session not found

**Error**: `Session not found: xyz123. Searched in ~/.claude/projects projects.`

**Solutions**:
1. Verify session ID exists: `snps claude list`
2. Use full path instead of ID
3. Check project directory exists: `ls ~/.claude/projects/`

### No title shown

**Cause**: Session doesn't have a summary record (very new or interrupted).

**Solutions**:
- This is expected for incomplete sessions
- Titles will appear once Claude Code creates the summary record
- Use `--format json` to see raw session data

### Message count mismatch

**Cause**: Different counting methods (we exclude tool results, Claude might count differently).

**Explanation**: This is normal - our implementation excludes:
- Tool result records
- Meta messages
- System infrastructure messages

---

## References

- Implementation plan: `thoughts/shared/plans/2025-12-12-claude-session-parser.md`
- Research document: `thoughts/shared/research/2025-12-12-session-list-discrepancy.md`
- Core module: `engine/snps-core/src/claude/`
- CLI implementation: `engine/snps-cli/src/main.rs:445-2929`
- JSONL format: `~/.claude/projects/<project-slug>/*.jsonl`

---

**Document Status**: Complete and production-ready
**Last Verified**: 2025-12-12
**Next Review**: As needed for feature additions

---

## Changelog

### 2025-12-12 - HTML Export Feature
- Added HTML export functionality with ampcode.com-inspired design
- Implemented `snps claude convert` command for format conversion
- Added standalone JavaScript rendering with `html_template.js`
- Updated documentation with HTML export workflows and examples
- Added CSS custom properties for light/dark mode support
