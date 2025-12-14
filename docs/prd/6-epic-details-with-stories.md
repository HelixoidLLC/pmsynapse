# 6. Epic Details with Stories

**Note:** This section provides representative stories from both Epic 1a and Epic 1b to demonstrate story format, acceptance criteria, and integration verification requirements. Complete story breakdown (all 25-33 stories) would be detailed during sprint planning with the development team.

---

## 6.1 Epic 1a: PMSynapse MVP Core (Pilot-Ready)

**Epic Goal:** Enable first pilot team to use PMSynapse daily for basic thought-to-ticket workflow without interrupting their development flow.

---

### Story 1a.1: Define Knowledge Graph SQLite Schema

**Complexity:** Medium (M) - 3-5 days

As a **backend developer**,
I want a formally documented SQLite database schema for the knowledge graph,
so that all team members understand the data model and can query/modify the graph consistently.

**Acceptance Criteria:**

1. Schema includes tables: `thoughts`, `workflow_stages`, `metadata`, `integrations`, `queue`
2. `thoughts` table columns: `id` (primary key), `title`, `description`, `author`, `created_at`, `updated_at`, `workflow_stage_id` (foreign key), `tags` (JSON)
3. `workflow_stages` table: `id`, `name`, `description`, `sequence_order`
4. `metadata` table: `thought_id` (foreign key), `key`, `value`, `type`
5. `integrations` table: `thought_id`, `integration_type` (Linear, GitHub), `external_id`, `url`, `status`, `synced_at`
6. `queue` table: `id`, `operation_type`, `payload` (JSON), `status`, `retry_count`, `created_at`
7. Indexes created on: `thoughts.workflow_stage_id`, `thoughts.created_at`, `thoughts.author`, `integrations.thought_id`
8. Foreign key constraints enforced
9. Migration script creates schema from scratch
10. Documentation written in `docs/database-schema.md` with ER diagram

**Integration Verification:**

- **IV1:** Existing thought data (if any) migrates without loss to new schema
- **IV2:** CLI commands (existing `snps thoughts` commands) continue working after schema deployment
- **IV3:** Query performance <500ms for typical operations (list thoughts, filter by stage, search by author)

---

### Story 1a.2: Implement RPC Interface Between CLI and Daemon

**Complexity:** Large (L) - 1-2 weeks

As a **CLI user**,
I want the CLI to communicate with the daemon process via RPC,
so that I can execute commands while the daemon manages state, integrations, and LLM calls.

**Acceptance Criteria:**

1. Daemon process exposes RPC server on configurable port (default: 50051, env var: `PMSYNAPSE_DAEMON_PORT`)
2. RPC protocol: gRPC or similar (efficient binary protocol)
3. RPC methods defined: `CreateThought`, `GetThought`, `ListThoughts`, `UpdateThoughtStage`, `SuggestWorkflow`, `SyncLinear`
4. CLI sends RPC requests to daemon, receives responses with proper error handling
5. Daemon startup check: CLI verifies daemon is running, prints helpful error if not ("Daemon not running. Start with: snps daemon start")
6. RPC connection timeout: 5 seconds (fail fast if daemon unreachable)
7. Shared TypeScript/Rust types for RPC contracts (using ts-rs or similar code generation)
8. Unit tests for each RPC method (mock daemon, test CLI RPC calls)
9. Integration test: CLI → daemon RPC → database → response

**Integration Verification:**

- **IV1:** Existing CLI commands route through new RPC layer without breaking functionality
- **IV2:** Daemon crash/restart doesn't lose CLI connection (auto-reconnect with exponential backoff)
- **IV3:** Multiple CLI instances can connect to single daemon concurrently without conflicts

---

### Story 1a.4: CLI Thought Capture with Metadata

**Complexity:** Medium (M) - 3-5 days

As a **developer**,
I want to capture thoughts via CLI (`snps thoughts new`) with automatic metadata,
so that I can quickly record ideas without manual data entry.

**Acceptance Criteria:**

1. Command: `snps thoughts new "thought title" --description "optional description"`
2. Interactive mode if no args: prompts for title, description, tags (optional)
3. Automatic metadata captured: timestamp (ISO 8601), author (from git config or system user), project context (current git repo if available)
4. Thought assigned unique ID (UUID or incrementing integer)
5. Thought saved to SQLite database via RPC call to daemon
6. CLI prints confirmation: "Thought created: #123 'thought title'" with thought ID
7. Execution time <100ms from command invocation to confirmation (FR26 requires <30 sec total, but CLI responsiveness should be instant)
8. Offline mode: If daemon unreachable, queue locally and sync when daemon available (deferred to Story 1a.11)
9. Help text: `snps thoughts new --help` explains command usage

**Integration Verification:**

- **IV1:** Existing thought storage (if any) remains intact; new thoughts append to database
- **IV2:** `snps thoughts list` displays newly created thought with correct metadata
- **IV3:** Thought metadata queryable via `snps thoughts get <id>` with JSON output option

---

### Story 1a.6: Workflow Suggestion with User Override

**Complexity:** Large (L) - 1-2 weeks

As a **developer**,
I want the system to suggest a workflow path (research/plan/implement) when I capture a thought, with ability to override,
so that the system guides me but doesn't force rigid workflows.

**Acceptance Criteria:**

1. After thought creation, daemon analyzes thought content and suggests workflow stage (research, planning, implementation, delivery)
2. Suggestion logic (MVP simple rules-based, LLM-enhanced in future):
   - Keywords "research", "investigate", "explore" → suggest "research" stage
   - Keywords "plan", "design", "architecture" → suggest "planning" stage
   - Keywords "implement", "build", "code" → suggest "implementation" stage
   - Default: "triage" stage if unclear
3. CLI displays suggestion: "Suggested workflow: research. Accept? [Y/n/override]"
4. User can accept (press Y or Enter), decline (press n, stays in triage), or override (type stage name manually)
5. If accepted/overridden, thought's `workflow_stage_id` updated in database
6. Workflow stage change logged in metadata table for audit trail
7. LLM provider abstraction: Supports OpenAI or Anthropic for content analysis (future enhancement, MVP uses keyword matching)
8. Performance: Suggestion returned <2 seconds (keyword matching should be near-instant)

**Integration Verification:**

- **IV1:** Existing workflow stage configuration (IDLC) remains valid; new suggestion logic doesn't override manual stage assignments
- **IV2:** CLI `snps thoughts update <id> --stage <stage>` allows manual override after initial suggestion
- **IV3:** Workflow suggestion doesn't block thought creation; if LLM/analysis fails, defaults to "triage" with warning message

---

### Story 1a.7: Linear OAuth 2.0 Authentication

**Complexity:** Extra Large (XL) - 2-3 weeks

As a **developer**,
I want to authenticate PMSynapse with my Linear account via OAuth 2.0,
so that the system can create and sync tickets on my behalf securely.

**Acceptance Criteria:**

1. OAuth flow implemented following Linear API OAuth 2.0 specification
2. Command: `snps integrations auth linear` initiates OAuth flow
3. CLI opens browser to Linear OAuth consent page (or prints URL if browser unavailable)
4. User authorizes PMSynapse application
5. OAuth callback redirects to localhost callback server (temporary HTTP server on port 8765)
6. CLI receives authorization code, exchanges for access token via Linear GraphQL API
7. Access token encrypted and stored in system keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
8. Token refresh logic: Automatically refresh expired tokens using refresh token
9. Command: `snps integrations status linear` shows authentication status (authenticated, token expiry, user info)
10. Command: `snps integrations revoke linear` removes stored token and revokes authorization
11. Error handling: Network failures, user denies consent, invalid callback codes (clear error messages, NFR21)
12. Documentation: Setup guide for Linear integration in `docs/LINEAR_INTEGRATION.md`

**Integration Verification:**

- **IV1:** Existing thoughts remain accessible after Linear authentication; no data corruption
- **IV2:** Multiple users on same machine can authenticate separately (per-user keychain storage)
- **IV3:** Token expiration/refresh doesn't interrupt ongoing operations (graceful token refresh in background)

---

### Story 1a.9: Linear Ticket Creation from Thoughts

**Complexity:** Large (L) - 1-2 weeks

As a **developer**,
I want to create Linear tickets automatically from thoughts,
so that my ideas flow directly into my team's issue tracker without manual copying.

**Acceptance Criteria:**

1. Command: `snps thoughts ticket <thought_id>` creates Linear ticket from thought
2. Ticket title: Thought title
3. Ticket description: Thought description + metadata (author, created date, PMSynapse thought ID for traceability)
4. Ticket created in user's default Linear team/project (configurable via `~/.pmsynapse/config.yaml`)
5. GraphQL mutation to Linear API: `createIssue` with proper error handling
6. Linear ticket ID and URL stored in `integrations` table linked to thought
7. CLI prints confirmation: "Linear ticket created: ENG-123 https://linear.app/..."
8. Sync latency <5 seconds (NFR3)
9. Offline mode: If Linear API unreachable, queue ticket creation for later retry (Story 1a.11 dependency)
10. Rate limiting: Respect Linear API limits (max 5 req/sec), implement backoff if rate-limited

**Integration Verification:**

- **IV1:** Existing Linear tickets (created manually) remain unaffected; no duplicate tickets created
- **IV2:** Linear ticket status changes (completed, cancelled) don't retroactively affect thought data in PMSynapse
- **IV3:** Multiple thoughts can create tickets in parallel without race conditions or duplicate tickets

---

## 6.2 Epic 1b: PMSynapse MVP Complete (Feature Complete)

**Epic Goal:** Deliver full MVP feature set enabling three pilot teams to adopt PMSynapse with rich UI experiences and comprehensive integrations.

---

### Story 1b.1: VS Code Extension Infrastructure and Activation

**Complexity:** Extra Large (XL) - 2-3 weeks

As a **developer using VS Code**,
I want PMSynapse installed as a VS Code extension that activates when I need it,
so that I can capture thoughts without leaving my IDE.

**Acceptance Criteria:**

1. Extension scaffold created using `yo code` generator (TypeScript extension template)
2. Extension manifest (`package.json`) defines:
   - Activation events: `onCommand:pmsynapse.captureThought`, `onView:pmsynapse.graphSidebar`
   - Contributes: Commands, views (sidebar), keybindings
3. Extension activates on first command invocation or sidebar open (lazy activation for performance)
4. Extension connects to daemon via TCP socket (localhost:50051) or Unix socket
5. Connection status indicator in VS Code status bar: "PMSynapse: Connected" (green) or "PMSynapse: Disconnected" (red)
6. If daemon not running, extension prompts: "Start PMSynapse daemon? [Yes/No]" and executes `snps daemon start` if user confirms
7. Extension settings in VS Code preferences: Daemon port, auto-start daemon, keyboard shortcuts
8. Webview infrastructure for thought capture panel (HTML/CSS/JS loaded from extension)
9. Extension icon and branding (logo, colors matching PMSynapse design system)
10. Published to VS Code Marketplace (or manual install via VSIX for testing)
11. Cross-platform compatibility: macOS, Linux, Windows
12. Documentation: Extension setup guide in `apps/vscode-ext/README.md`

**Integration Verification:**

- **IV1:** Extension installation doesn't conflict with existing VS Code extensions (no namespace collisions)
- **IV2:** Extension activation doesn't slow down VS Code startup (lazy activation ensures <200ms overhead)
- **IV3:** Daemon connection loss handled gracefully; extension reconnects automatically when daemon restarts

---

### Story 1b.3: VS Code Thought Capture Panel

**Complexity:** Large (L) - 1-2 weeks

As a **developer in VS Code**,
I want a keyboard shortcut (`Cmd/Ctrl+Shift+T`) to open a thought capture panel,
so that I can quickly record ideas without context switching.

**Acceptance Criteria:**

1. Keyboard shortcut: `Cmd+Shift+T` (macOS) / `Ctrl+Shift+T` (Windows/Linux) opens thought capture panel
2. Panel appears as webview panel (not sidebar) for focused input
3. Panel UI: Title field (required), description textarea (optional), tags field (comma-separated, optional)
4. "Capture" button sends thought to daemon via RPC
5. Workflow suggestion displayed after RPC response: "Suggested: research" with accept/override buttons
6. Panel closes automatically after successful capture (configurable: auto-close or stay open)
7. Keyboard shortcuts within panel: `Cmd/Ctrl+Enter` to submit, `Esc` to close
8. Error handling: Network errors, validation errors (empty title) displayed inline with helpful messages
9. Form state preserved if panel accidentally closed (draft recovery)
10. Accessibility: Proper ARIA labels, keyboard navigation, screen reader support

**Integration Verification:**

- **IV1:** Existing VS Code keyboard shortcuts don't conflict; `Cmd/Ctrl+Shift+T` remappable if user has conflict
- **IV2:** Thought captured via VS Code extension syncs to CLI immediately (queryable via `snps thoughts list`)
- **IV3:** Concurrent thought capture from CLI and VS Code extension doesn't create race conditions or data loss

---

### Story 1b.7: Desktop App Knowledge Graph Visualization

**Complexity:** Extra Large (XL) - 2-3 weeks

As a **team lead or architect**,
I want to view the knowledge graph in a desktop app with interactive visualization,
so that I can understand how thoughts, research, plans, and implementations are connected.

**Acceptance Criteria:**

1. Desktop app (Tauri 2.0) launches via command: `snps ui` or desktop icon
2. Graph visualization library: D3.js, Cytoscape.js, or vis.js (interactive node-link diagram)
3. Nodes represent entities: Thoughts (blue circles), Research (green squares), Plans (yellow diamonds), Implementations (purple hexagons)
4. Edges represent relationships: originated-from, informs, implements (labeled, directional arrows)
5. Graph layout algorithm: Force-directed layout for automatic positioning
6. Interactions:
   - Pan: Click and drag background
   - Zoom: Scroll wheel or pinch gesture
   - Select node: Click to highlight and show detail panel
   - Filter: Dropdown to show/hide node types or workflow stages
7. Detail panel (sidebar): Selected node's metadata, linked entities, quick actions (edit, delete, create ticket)
8. Performance: Render <1 second for graphs with 100 nodes, <5 seconds for 1000 nodes
9. Empty state: "No thoughts yet. Capture your first idea!" with CTA button to open thought capture
10. Export: Button to export graph as PNG, SVG, or JSON

**Integration Verification:**

- **IV1:** Graph visualization reflects real-time state from daemon (WebSocket connection for live updates)
- **IV2:** Changes made in CLI or VS Code extension appear in desktop app within 2 seconds
- **IV3:** Desktop app doesn't lock database; concurrent access with CLI/VS Code extension supported

---

### Story 1b.12: Research Document Auto-Generation

**Complexity:** Large (L) - 1-2 weeks

As a **developer**,
I want the system to automatically generate research document templates when I select "research" workflow stage,
so that I can start documenting findings without manual setup.

**Acceptance Criteria:**

1. When thought moves to "research" stage, daemon auto-generates research document in `~/.pmsynapse/research/<thought-id>.md`
2. Template structure:
   ```markdown
   # Research: [Thought Title]

   **Created:** [Timestamp]
   **Author:** [Author]
   **Originating Thought:** #[Thought ID]

   ## Objective
   [Auto-populated from thought description]

   ## Findings
   - [ ] Finding 1
   - [ ] Finding 2

   ## Conclusions
   [To be filled]

   ## Next Steps
   - [ ] Action 1
   ```
3. Research document linked to thought in knowledge graph (`metadata` table: `thought_id`, `research_doc_path`)
4. CLI command: `snps research open <thought_id>` opens document in default editor (via `$EDITOR` or `xdg-open`)
5. VS Code extension: Research document automatically opened in editor when created
6. Research markdown files searchable via `snps search research <query>`
7. Template customizable via `~/.pmsynapse/templates/research.md`

**Integration Verification:**

- **IV1:** Existing research documents (if manually created) not overwritten by auto-generation
- **IV2:** Research document changes (editing in external editor) reflected in knowledge graph (file watcher updates metadata)
- **IV3:** Deleting thought deletes associated research document (cascade delete with user confirmation)

---

## 6.3 Remaining Stories

**Epic 1a Remaining Stories (6 additional stories):**

- Story 1a.3: Daemon Startup and Configuration
- Story 1a.5: CLI Thought Listing and Filtering
- Story 1a.8: Linear Status Sync (Bidirectional)
- Story 1a.10: Queue-and-Retry for Offline Operations
- Story 1a.11: LLM Response Caching for Offline Mode
- Story 1a.12: Error Handling and User Documentation

**Epic 1b Remaining Stories (10 additional stories):**

- Story 1b.2: VS Code Extension Settings and Configuration
- Story 1b.4: VS Code Knowledge Graph Sidebar
- Story 1b.5: VS Code Workflow Status View
- Story 1b.6: Desktop App Foundation and Routing
- Story 1b.8: Desktop App Dashboard with Metrics
- Story 1b.9: Desktop App Settings Panel
- Story 1b.10: Plan Automation with Structured Format
- Story 1b.11: Document Template System
- Story 1b.13: GitHub Commit Linking
- Story 1b.14: GitHub PR Status Display
- Story 1b.15: Multi-User Authentication
- Story 1b.16: Role-Based Access Control
- Story 1b.17: Web UI Read-Only Visualization
- Story 1b.18: Database Schema Documentation and Migration

**Note:** These remaining stories follow similar format with acceptance criteria and integration verification. Detailed story breakdown would occur during sprint planning sessions with the development team, where stories can be refined, split, or reprioritized based on team capacity and emerging requirements.

---

**End of PRD**

---

**Document Status:** Draft Complete - Ready for Review

**Next Steps:**
1. Review PRD with stakeholders (engineering team, product leadership, pilot team candidates)
2. Validate technical assumptions (Tauri 2.0 stability timeline, CozoDB migration feasibility)
3. Refine story estimates with development team during sprint planning
4. Prioritize Epic 1a stories into first sprint backlog
5. Begin Epic 1a development with Story 1a.1 (Database Schema)

---
