# PMSynapse Brownfield Enhancement PRD

**Document Version:** v1.0
**Last Updated:** 2025-12-13
**Status:** Draft

---

## 1. Intro Project Analysis and Context

### 1.1 Existing Project Overview

#### 1.1.1 Analysis Source

**Source:** IDE-based fresh analysis combined with existing project documentation

The PMSynapse project has extensive documentation in the `docs/` folder (25+ documents) including:
- Comprehensive project brief (`docs/brief.md`)
- Brainstorming session results documenting strategic direction
- Architecture and design documents (12-Factor Agents, AI Team Coordination, UI patterns)
- System-specific documentation (Thoughts System, IDLC Configuration, Project Workflow)
- Technical research (BAML best practices, database options, VS Code extension architecture)

**Note:** No formal `document-project` analysis was previously run. This PRD leverages existing organic documentation created during project development.

#### 1.1.2 Current Project State

**What PMSynapse Currently Is:**

PMSynapse is an AI-enabled project management system in early development, designed to automate thought management through workflow steps aligned to human project delivery. The platform addresses knowledge fragmentation in software development by providing non-intrusive knowledge capture that serves both executives (strategic visibility) and developers (workflow automation).

**Current Implementation Status:**

- **Engine Layer (Rust):**
  - `snps-core` - Core library with knowledge graph (SQLite/rusqlite), LLM integration foundation, IDLC engine structure
  - `snps-cli` - CLI tool with daemon process, thoughts commands, Claude session management
  - ~16 Rust source files with foundational implementation

- **Application Layer:**
  - `desktop` - Tauri 2.0 + React 18 + shadcn/ui (structure exists, limited implementation)
  - `vscode-ext` - VS Code extension (planned, minimal implementation)

- **Working Features:**
  - Thoughts system (CLI-based knowledge capture with local storage)
  - Claude session parser and management (`snps claude` commands)
  - Daemon process architecture
  - Basic Linear integration foundation
  - IDLC (Idea Development Lifecycle) configuration system

- **Repository Structure:**
  - BAML-style monorepo organization
  - Rust isolated in `engine/` workspace
  - Turborepo orchestration for multi-package builds
  - Comprehensive documentation in `docs/`

**Primary Purpose:** Enable software teams to capture, organize, and progress ideas from conception through implementation without interrupting developer flow, while uniquely addressing brownfield projects with years of lost knowledge.

### 1.2 Documentation Analysis

#### 1.2.1 Available Documentation

**Comprehensive documentation exists** - No `document-project` analysis was run, but extensive organic documentation is available:

✓ **Tech Stack Documentation**
- `CLAUDE.md` - Project overview, build commands, tech stack summary
- `PMSYNAPSE_CORE_FEATURES.md` - Feature overview and architecture
- `RUST_GRAPH_VECTOR_DATABASE_OPTIONS.md` - Database research

✓ **Source Tree/Architecture**
- `UI_ARCHITECTURE_PATTERNS.md` - UI design patterns
- `VSCODE_EXTENSION_ARCHITECTURE.md` - VS Code extension design
- `12_FACTOR_AGENTS_DESIGN.md` - Agent architecture principles
- `AI_TEAM_COORDINATION_PATTERNS.md` - Multi-agent coordination

✓ **Coding Standards**
- `BAML_BEST_PRACTICES_ANALYSIS.md` - Repository organization patterns
- Rust conventions documented in `CLAUDE.md`

✓ **System Documentation**
- `THOUGHTS_SYSTEM.md` - Thoughts management system
- `THOUGHTS_WORKFLOW_TUTORIAL.md` - Workflow guidance with stories
- `THOUGHTS_REFERENCE.md` - Reference documentation
- `THOUGHTS_TEAM_TUTORIAL.md` - Team-focused tutorial
- `IDLC_CONFIGURATION.md` - Workflow configuration
- `PROJECT_WORKFLOW.md` - Default workflow + Linear integration
- `STARTUP_GUIDE.md` - Daemon and UI setup

✓ **Technical Research**
- `AI_ENABLED_PROJECT_MANAGEMENT_RESEARCH.md` - Market research
- `HUMANLAYER_THOUGHTS_ANALYSIS.md` - Thought system analysis
- `TASK_MASTER_ANALYSIS.md` - Task management patterns

✓ **Strategic Documentation**
- `brief.md` - Comprehensive project brief (executive summary, problem statement, solution, MVP scope, post-MVP vision)
- `brainstorming-session-results.md` - Strategic brainstorming session output
- `PLANNED_FEATURES.md` - Feature roadmap

✓ **Process Documentation**
- `TURBOREPO_RELEASE_PROCESS.md` - Release process
- `CLAUDE_SESSION_PARSER.md` - Claude session management

**Assessment:** Documentation coverage is excellent for a project in early development. No critical documentation gaps exist. The project has strong strategic clarity (brief, brainstorming results) and solid technical foundation documentation.

### 1.3 Enhancement Scope Definition

#### 1.3.1 Enhancement Type

This PRD addresses **MVP feature development** for an existing brownfield project:

- ✓ **New Feature Addition** - Building out MVP features defined in project brief
- ✓ **Integration with New Systems** - Linear, GitHub, LLM providers
- ✓ **UI/UX Development** - Desktop app, VS Code extension, web UI
- ✓ **Major Feature Development** - Thought capture, workflow engine, knowledge graph visualization

**Complexity Assessment:**

This is **NOT** a simple feature addition suitable for `brownfield-create-story`. This is comprehensive MVP development requiring:
- Multiple coordinated features across CLI, desktop app, and VS Code extension
- Complex integrations (Linear API, GitHub API, LLM providers)
- Full-stack implementation (Rust backend, React frontend, VS Code extension)
- Architectural decisions about data models, API contracts, workflow logic
- Multiple stories and epics

**Recommendation:** Full brownfield PRD process is appropriate for this scope.

#### 1.3.2 Enhancement Description

**Enhancement Summary:**

Build out the PMSynapse MVP to deliver core value proposition: **non-intrusive knowledge capture that serves both executives and developers while enabling brownfield teams to restore years of lost project knowledge.**

The enhancement transforms the current foundational codebase into a working MVP with 8 core features:

1. **Thought Capture System** - CLI + VS Code extension for instant idea capture (<30 sec)
2. **Guided Workflow Engine** - Context-aware system suggesting workflow paths (research → plan → implement → deliver)
3. **Research Document Generation** - Automated creation of research documents linked to thoughts
4. **Planning Automation** - Convert research into implementation plans with ticket generation
5. **VS Code Extension** - Native IDE integration for thought capture and workflow visibility
6. **CLI Tool** - Command-line interface for all core functions
7. **Knowledge Graph Visualization** - Basic web UI showing relationships between thoughts, research, plans, code
8. **Linear Integration** - Two-way sync for ticket creation and status tracking

**Target Users:**
- **Primary:** Individual contributors (developers/engineers, 5-20 years experience, working on brownfield projects)
- **Secondary:** Engineering leadership (CTOs, VPs Engineering, Engineering Managers)

#### 1.3.3 Impact Assessment

**Scope of Impact on Existing Codebase:**

- ✓ **Major Impact (architectural changes required)**

**Rationale:**

While foundational architecture exists, the MVP requires:
- Significant expansion of `snps-core` library (workflow engine, integration adapters, knowledge graph queries)
- Complete implementation of desktop app (currently structural only)
- New VS Code extension from architectural foundation
- Data model evolution for knowledge graph (thoughts → research → plans → implementations)
- New RPC contracts between daemon, CLI, VS Code extension, desktop app
- Integration layer development (Linear GraphQL, GitHub REST/GraphQL, LLM APIs)
- Web UI development for knowledge graph visualization

**Existing Code Changes:**
- Moderate changes to existing thoughts system (expand from CLI-only to multi-interface)
- Evolution of daemon process (add workflow engine, integration sync, LLM orchestration)
- IDLC configuration system expansion (support default workflow, user overrides)

**New Code:**
- Substantial new implementation across all layers (engine, CLI, desktop, VS Code)
- Integration adapters for Linear, GitHub, LLM providers
- Workflow automation logic
- Knowledge graph visualization UI

### 1.4 Goals and Background Context

#### 1.4.1 Goals

**If this enhancement is successful, PMSynapse will deliver:**

- Enable developers to capture thoughts/ideas in <30 seconds without breaking flow state
- Provide AI-guided workflow suggestions (research → plan → implement) with user override capability
- Automatically generate and link research documents to originating thoughts
- Convert research into actionable implementation plans with Linear ticket creation
- Offer native VS Code integration for thought capture and workflow status visibility
- Deliver CLI tool for terminal-centric developers and automation scripting
- Visualize knowledge graph showing idea progression from conception through implementation
- Sync bidirectionally with Linear (<5 second latency) for ticket management
- Integrate with GitHub to link commits/PRs to thoughts and plans automatically
- Support 3 pilot teams with 60%+ weekly active usage for 30+ days
- Enable developers to find architectural context in <2 minutes vs current 30+ minutes
- Reduce new developer onboarding time by 40% through accessible knowledge graph
- Capture average 5+ thoughts per developer per week (habit formation)
- Achieve 70%+ daily active usage among pilot teams after 30 days
- Maintain zero data loss with local-first architecture
- Prove non-intrusive design with <10% user abandonment rate

#### 1.4.2 Background Context

**Why This Enhancement is Needed:**

Software development teams face a critical knowledge management crisis. Years of architectural decisions, technical discussions, and contextual rationale exist only in fragmented forms—scattered across Slack conversations, buried in Jira tickets, trapped in individual developers' minds, or never documented at all. This creates:

- **Onboarding friction:** New developers spend weeks reverse-engineering undocumented decisions
- **Decision paralysis:** Teams lack historical context to make informed choices
- **Executive blindness:** Leadership has no visibility into idea progression
- **Technical debt accumulation:** Undocumented assumptions become codebase landmines
- **Brownfield modernization risk:** Legacy updates carry high risk due to lost design knowledge

**Why Existing Solutions Fail:**

Current tools fall into two inadequate categories:
1. **Heavy PM systems (Jira, Asana, Monday):** Interrupt developer workflow, require duplicate data entry, treat documentation as separate from implementation
2. **Developer-first tools (Git, IDE extensions):** Capture code but not context; no bridge to executive visibility

Neither solves brownfield knowledge restoration. All assume greenfield projects with clean documentation from day one.

**How This Fits With Existing Project:**

PMSynapse already has foundational architecture (thoughts system, daemon process, IDLC configuration, Linear integration foundation). This enhancement builds on that foundation to deliver the complete MVP value proposition. The existing codebase provides:
- Proven BAML monorepo structure
- Working CLI with thoughts commands
- Daemon architecture ready for workflow engine
- Documentation demonstrating clear strategic vision
- Tech stack decisions (Rust + React + Tauri 2.0)

This enhancement completes the journey from foundation to working MVP that pilot teams can adopt.

**Strategic Differentiation:**

PMSynapse's moat is the **non-intrusive executive assistant model** combined with **brownfield capability**. By optimizing for developer velocity and capturing documentation as a side effect (rather than forcing documentation overhead), the system aligns with intrinsic motivation. The future brownfield knowledge restoration capability (Phase 2) will make PMSynapse the only solution designed for the reality that most software development happens on legacy systems.

### 1.5 Change Log

| Change | Date | Version | Description | Author |
|--------|------|---------|-------------|--------|
| Initial draft | 2025-12-13 | v1.0 | Created brownfield PRD from project brief foundation | PM Agent (John) |

---

## 2. Requirements

### 2.1 Functional Requirements

**FR1:** The system shall provide a CLI command (`snps thoughts new`) to capture thoughts/ideas with automatic metadata (timestamp, author, project context) stored in the local knowledge graph.

**FR2:** The VS Code extension shall provide a keyboard shortcut to launch thought capture UI that accepts thought input and saves to knowledge graph without leaving the IDE.

**FR3:** The workflow engine shall analyze thought content and suggest appropriate workflow path (research → plan → implement → deliver) based on context, with user override capability.

**FR4:** The system shall allow users to skip suggested workflow steps and proceed directly to any stage (e.g., skip research, go straight to implementation).

**FR5:** The system shall generate research document templates automatically when research workflow stage is selected, pre-populating metadata from the originating thought.

**FR6:** Research documents shall be automatically linked to originating thoughts in the knowledge graph with bidirectional relationships.

**FR7:** The system shall convert research findings into implementation plans with structured format (goals, technical approach, tasks, acceptance criteria).

**FR8:** Planning automation shall generate Linear tickets automatically from plan tasks, linking tickets back to plans and originating thoughts in knowledge graph.

**FR9:** The VS Code extension shall display a side panel showing the current thought's workflow status, linked research, plans, and implementation tickets.

**FR10:** The VS Code extension shall allow browsing the knowledge graph and navigating between thoughts, research, plans, and code implementations.

**FR11:** The CLI tool shall support all core operations: thought capture, research creation, plan generation, status queries, knowledge graph navigation.

**FR12:** The CLI tool shall support scripting and automation through machine-readable output formats (JSON, structured text).

**FR13:** The desktop app shall provide a web-based UI visualizing the knowledge graph with nodes for thoughts, research, plans, and implementations.

**FR14:** The knowledge graph visualization shall show relationships between entities (thought → research, research → plan, plan → implementation).

**FR15:** The Linear integration shall create tickets in Linear when plans are converted to implementations, using Linear GraphQL API.

**FR16:** The Linear integration shall sync ticket status updates back to PMSynapse knowledge graph with <5 second latency.

**FR17:** The Linear integration shall support OAuth 2.0 authentication with tokens encrypted at rest in system keychain.

**FR18:** The GitHub integration shall automatically link commits to thoughts and plans when commit messages follow convention (e.g., `[THOUGHT-123]`).

**FR19:** The GitHub integration shall display implementation progress in knowledge graph visualization based on linked commits and PR status.

**FR20:** The system shall support multiple LLM providers (OpenAI, Anthropic) for workflow suggestions, research assistance, and plan generation.

**FR21:** The daemon process shall manage all LLM API calls, integration syncing, and workflow automation as a background service.

**FR22:** The daemon shall expose RPC interface for CLI, VS Code extension, and desktop app to communicate with core services.

**FR23:** The system shall persist all data locally (thoughts, research, plans, knowledge graph) in SQLite database with zero cloud dependency. Core functions (thought capture, research creation, plan generation) shall work fully offline without internet connectivity or external service availability.

**FR24:** The knowledge graph shall track relationships: Issues/Tasks → Research/Findings → Plans/Implementations → Code/Docs with queryable graph structure.

**FR25:** The IDLC configuration system shall support the default workflow (triage → backlog → research → planning → development → validation → delivery → completed) with user customization capability.

**FR26:** Thought capture shall complete in <30 seconds from keyboard shortcut to return-to-code for flow state preservation.

**FR27:** Knowledge graph queries shall return results in <500ms for typical relationship traversal operations.

**FR28:** The system shall provide search functionality across thoughts, research, and plans with semantic similarity and keyword matching.

**FR29:** The system shall support offline mode where all core functions (thought capture, research document creation, plan generation) work without internet connectivity. LLM suggestions shall be cached locally, and integration operations (Linear, GitHub) shall queue-and-sync when connectivity is restored.

**FR30:** The knowledge graph SQLite schema shall be formally documented with table definitions, column specifications, relationship mappings, foreign key constraints, and index strategy to achieve <500ms query performance targets (NFR2).

**FR31:** The system shall support multi-user authentication with role-based access control (owner, editor, viewer) for thoughts, research, and plans to protect sensitive architectural decisions and proprietary information.

**FR32:** The data layer architecture shall abstract database implementation details behind a repository interface to support future migration from SQLite to CozoDB without breaking changes to RPC contracts, CLI commands, or application code.

### 2.2 Non-Functional Requirements

**NFR1:** Thought capture response time shall be <100ms from keyboard shortcut to input ready (UI must feel instant).

**NFR2:** Knowledge graph queries shall execute in <500ms for typical relationship traversal to maintain perceived performance.

**NFR3:** Linear/GitHub integration sync latency shall be <5 seconds to provide timely status updates.

**NFR4:** CLI commands shall execute in <1 second for common operations (thought creation, status queries, basic navigation).

**NFR5:** Desktop app cold start time shall be <3 seconds to maintain usability.

**NFR6:** The system shall maintain 95%+ uptime for integrations (Linear, GitHub, LLM providers) with graceful degradation when services are unavailable.

**NFR7:** The system shall achieve zero data loss through local-first architecture with atomic SQLite transactions.

**NFR8:** The codebase shall maintain existing Rust conventions (snake_case functions, type hints, thiserror for errors, Tokio async runtime).

**NFR9:** The frontend shall use existing tech stack (React 18, shadcn/ui, Tailwind, Zustand, Vite 6) without introducing new frameworks.

**NFR10:** The system shall support cross-platform deployment (macOS, Linux, Windows) for desktop app and CLI.

**NFR11:** API keys and OAuth tokens shall be stored in system keychain (not plaintext) with encryption at rest.

**NFR12:** The system shall log errors and important events for debugging, controlled by `RUST_LOG` environment variable.

**NFR13:** The monorepo structure shall maintain BAML pattern (Backend in `engine/`, Apps in `apps/`, shared packages in `packages/`).

**NFR14:** New code shall include test coverage with unit tests inline (`#[cfg(test)]`), integration tests in `engine/*/tests/`, and cross-component tests in `integ-tests/`.

**NFR15:** The system shall achieve <10% user abandonment rate (vs 40-60% typical for PM tools) by maintaining non-intrusive design.

**NFR16:** User satisfaction score shall be >7/10 for "doesn't interrupt my workflow" metric.

**NFR17:** The system shall scale to support 3 pilot teams (5-15 developers each) with acceptable performance.

**NFR18:** Memory usage for daemon process shall remain <500MB under typical workload (100 thoughts, 50 research docs, 30 plans).

**NFR19:** The VS Code extension shall be compatible with VS Code version 1.85+ across all platforms.

**NFR20:** Web UI shall support modern browsers (Chrome, Firefox, Safari, Edge - last 2 versions) with JavaScript and WebSocket support required.

**NFR21:** Integration failures (Linear API, GitHub API, LLM providers) shall present actionable error messages to users with recovery options (retry, skip, queue for later) rather than silent failures.

**NFR22:** The system shall queue failed integration operations (ticket creation, status sync, commit linking) locally with automatic retry when connectivity is restored, preventing data loss during outages.

### 2.3 Compatibility Requirements

**CR1: Existing API Compatibility** - The enhancement shall maintain backward compatibility with existing `snps thoughts` CLI commands. Current thought capture functionality must continue working without breaking changes to command syntax or data formats.

**CR2: Database Schema Compatibility** - The knowledge graph schema evolution shall support migration from existing SQLite schema without data loss. Existing thoughts, metadata, and relationships must be preserved during schema updates. The architecture shall support future migration to CozoDB (graph + vector database) with minimal disruption to application layer code.

**CR3: UI/UX Consistency** - New UI components (VS Code extension, desktop app, web UI) shall follow existing design patterns from shadcn/ui component library and maintain visual consistency with Tauri 2.0 styling. Color schemes, typography, and interaction patterns shall be uniform across interfaces.

**CR4: Integration Compatibility** - The system shall maintain compatibility with existing Linear integration foundation. Current Linear API contracts, OAuth flow, and data synchronization patterns shall be preserved and extended (not replaced).

---

## 3. User Interface Enhancement Goals

**Enhancement includes UI changes:** ✓ Yes - VS Code extension, desktop app, and web UI

### 3.1 Integration with Existing UI

**Design System Foundation:**

PMSynapse MVP introduces three new UI surfaces that must integrate cohesively:

1. **VS Code Extension** - Native IDE integration following VS Code extension guidelines and Webview UI Toolkit patterns
2. **Desktop App** - Tauri 2.0 application with React 18 frontend using shadcn/ui components
3. **Web UI** - Browser-based knowledge graph visualization with read-only access

**Integration Requirements:**

- **Component Library:** All UIs shall use shadcn/ui as the base component library to ensure visual consistency. Components include Button, Input, Card, Dialog, Dropdown, Table, and navigation elements.

- **Design Tokens:** Shared design system with common color palette, typography scale (font families, sizes, weights), spacing system (4px/8px grid), and border radius values.

- **Styling Approach:** Tailwind CSS utility classes for styling across all UI surfaces. VS Code extension adapts to user's theme (light/dark mode), desktop app provides theme toggle, web UI follows system preference.

- **Accessibility:** All UI components shall meet WCAG 2.1 AA standards with keyboard navigation, screen reader support, and proper ARIA attributes.

- **Responsive Design:** Desktop app and web UI shall support responsive layouts (minimum 1024px width recommended, graceful degradation to 768px). VS Code extension adapts to panel width.

**Existing Design Patterns to Follow:**

- **CLI-First Philosophy:** UI surfaces complement but don't replace CLI functionality. Power users should be able to accomplish all tasks via CLI.

- **Non-Intrusive Presentation:** UIs appear on-demand (keyboard shortcuts, explicit commands) rather than auto-populating or interrupting workflow.

- **Status Visibility:** Workflow status (triage → research → planning → development) visible at a glance without requiring navigation.

### 3.2 Modified/New Screens and Views

**VS Code Extension Views:**

1. **Thought Capture Panel**
   - Quick input form (keyboard shortcut activated)
   - Fields: thought title, description, optional tags
   - Workflow suggestion preview with override buttons
   - Recent thoughts list (last 10)

2. **Knowledge Graph Sidebar**
   - Tree view of thoughts organized by workflow stage
   - Expandable nodes showing linked research, plans, tickets
   - Click-to-navigate to related documents/files
   - Search bar for filtering thoughts

3. **Workflow Status View**
   - Current thought's progress through workflow stages
   - Linked Linear tickets with status badges
   - Linked GitHub commits/PRs
   - Quick actions (move stage, add research, create plan)

**Desktop App Screens:**

1. **Dashboard (Home)**
   - Overview metrics (total thoughts, active workflows, completion rate)
   - Recent activity feed (thoughts captured, research completed, tickets created)
   - Quick actions (capture thought, view graph, check Linear sync status)

2. **Knowledge Graph Visualization**
   - Interactive node-link diagram showing thoughts, research, plans, implementations
   - Node types distinguished by color/shape
   - Relationship edges with labels (originated-from, informs, implements)
   - Zoom/pan controls, filter by workflow stage
   - Click node to view details panel

3. **Thought Detail View**
   - Full thought metadata (author, timestamp, tags, workflow stage)
   - Linked research documents (with preview/open)
   - Associated plans and implementation tickets
   - Edit/delete controls, stage transition buttons

4. **Settings Panel**
   - Integration configuration (Linear OAuth, GitHub token, LLM provider API keys)
   - Workflow customization (IDLC stages, default paths)
   - UI preferences (theme, notification settings)
   - Data management (export, backup, clear cache)

**Web UI Views:**

1. **Read-Only Graph Visualization**
   - Same knowledge graph display as desktop app
   - No editing capabilities (view-only for stakeholders)
   - Export options (PNG, SVG, JSON)

2. **Thought Browser**
   - Searchable list of all thoughts with filters (date, author, stage, tags)
   - Detail view for selected thought
   - Permalink sharing for specific thoughts/graphs

### 3.3 UI Consistency Requirements

**Visual Consistency:**

- **Color Palette:** Primary color (brand identity), secondary colors for workflow stages, semantic colors (success: green, warning: yellow, error: red, info: blue)

- **Typography:**
  - Headings: System font stack (SF Pro on macOS, Segoe UI on Windows, Inter/Roboto on Linux)
  - Body: Same system fonts with 16px base size, 1.5 line height
  - Code: JetBrains Mono or Fira Code for monospace elements

- **Spacing:** Consistent 8px spacing unit across all components and layouts

- **Icons:** Use same icon library (Lucide Icons or Heroicons) across all UIs for consistency

**Interaction Consistency:**

- **Keyboard Shortcuts:** Consistent shortcuts across VS Code extension and desktop app (e.g., Cmd/Ctrl+Shift+T for thought capture)

- **Navigation Patterns:** Breadcrumb navigation for hierarchical views, sidebar navigation for major sections

- **Form Behavior:** Consistent input validation, error messaging placement (below fields), submit button states (loading, disabled, success)

- **Feedback Mechanisms:** Toast notifications for actions (thought saved, sync completed), progress indicators for long operations (>2 seconds)

**State Management:**

- **Loading States:** Skeleton screens for initial loads, spinners for async operations, optimistic UI updates where appropriate

- **Empty States:** Helpful empty state messages with call-to-action ("No thoughts yet. Capture your first idea!")

- **Error States:** Clear error messages with recovery suggestions (not technical jargon)

**Cross-UI Consistency Rules:**

- Thought cards display consistently (title, metadata, stage badge, action buttons) across all UIs
- Workflow stage badges use same colors and labels everywhere
- Linear ticket references show same format (icon + ID + title)
- GitHub commit links use same presentation (hash + message preview)

---

## 4. Technical Constraints and Integration Requirements

### 4.1 Existing Technology Stack

**Languages:**
- Rust (stable channel, edition 2021) - Backend engine, CLI, daemon
- TypeScript/JavaScript - Frontend (React), VS Code extension, RPC types
- SQL - Database queries and schema (SQLite)

**Frameworks:**
- **Backend:** Tokio (async runtime), rusqlite (database), serde (serialization), thiserror (error handling), clap (CLI parsing)
- **Frontend:** React 18 (UI library), Vite 6 (build tool), Tailwind CSS (styling), shadcn/ui (component library), Zustand (state management)
- **Desktop:** Tauri 2.0 with plugins (fs, shell, notification, log, store)
- **VS Code:** VS Code Extension API 1.85+, Webview UI Toolkit

**Database:**
- **Current:** SQLite 3.x via rusqlite (embedded, local-first, zero-config)
- **Future:** CozoDB (graph + vector database, WASM support, Datalog queries)
- Migration path required from SQLite → CozoDB (see FR32, CR2)

**Infrastructure:**
- **Local:** Daemon process running on developer machine (no server required for MVP)
- **Web UI:** Static site hosting (Vercel, Netlify, or CloudFlare Pages)
- **OAuth Callbacks:** Public endpoint required for Linear/GitHub OAuth flows

**External Dependencies:**
- Linear API (GraphQL, OAuth 2.0)
- GitHub API (REST v3 + GraphQL v4, OAuth Apps)
- LLM Providers: OpenAI API, Anthropic API (Claude)
- System Keychain: macOS Keychain, Windows Credential Manager, Linux Secret Service

**Version Constraints:**
- Rust: 1.70+ (for Tauri 2.0 compatibility)
- Node.js: 18+ (for pnpm workspaces, Vite 6)
- VS Code: 1.85+ (extension compatibility)
- Tauri: 2.0.x (alpha/beta currently, stable expected Q1 2026)

### 4.2 Integration Approach

**Database Integration Strategy:**

- **Local-First Architecture:** SQLite database stored in user's home directory (`~/.pmsynapse/data/graph.db`)
- **Schema Management:** Migration system using rusqlite with version tracking table
- **Atomic Transactions:** All multi-table operations wrapped in transactions for consistency
- **Backup Strategy:** Automatic daily backups to `~/.pmsynapse/backups/` with 7-day retention
- **Query Optimization:** Indexes on frequently queried columns (thought_id, workflow_stage, author, timestamp)
- **CozoDB Migration:** Abstract database layer (repository pattern) to support future CozoDB switch without breaking RPC contracts

**API Integration Strategy:**

- **Linear Integration:**
  - OAuth 2.0 flow for authentication
  - GraphQL API for ticket operations (create, update, query)
  - Webhook support (future) for real-time status updates
  - Queue-and-retry for offline scenarios (NFR22)
  - Rate limiting respect (max 5 req/sec per Linear docs)

- **GitHub Integration:**
  - OAuth Apps authentication (not personal access tokens)
  - REST API v3 for repository operations
  - GraphQL API v4 for complex queries (commit history, PR status)
  - Commit message conventions for auto-linking (`[THOUGHT-123]`, `Relates to PLAN-456`)
  - Webhook support (future) for push events

- **LLM Provider Integration:**
  - Multi-provider abstraction layer (OpenAI, Anthropic, future: local models)
  - API key management via system keychain (NFR11)
  - Request/response caching for offline mode (FR29)
  - Streaming support for long-running generations
  - Error handling and fallback (if primary provider fails, queue for retry)

**Frontend Integration Strategy:**

- **Desktop App ↔ Daemon:** IPC via Tauri commands (invoke from React → Rust handlers)
- **VS Code Extension ↔ Daemon:** Unix socket or TCP localhost connection for RPC
- **CLI ↔ Daemon:** Direct function calls (snps-cli links snps-core as library)
- **Web UI ↔ Daemon:** WebSocket connection for real-time graph updates (read-only)
- **Shared Types:** TypeScript RPC types generated from Rust types using ts-rs or similar

**Testing Integration Strategy:**

- **Unit Tests:** Inline with Rust source (`#[cfg(test)]`), Jest for TypeScript
- **Integration Tests:** `engine/*/tests/` for Rust cross-module, `integ-tests/` for cross-component
- **E2E Tests:** Tauri's testing framework for desktop app user flows
- **API Mocking:** Mock Linear/GitHub APIs in tests using wiremock or similar
- **CI/CD:** GitHub Actions running tests on Linux, macOS, Windows

### 4.3 Code Organization and Standards

**File Structure Approach:**

```
pmsynapse/
├── engine/                    # Rust workspace
│   ├── snps-core/             # Core library
│   │   ├── src/
│   │   │   ├── graph.rs       # Knowledge graph operations
│   │   │   ├── workflow.rs    # IDLC workflow engine
│   │   │   ├── integrations/  # Linear, GitHub, LLM adapters
│   │   │   └── lib.rs         # Public API
│   │   └── tests/             # Integration tests
│   └── snps-cli/              # CLI tool
│       ├── src/
│       │   ├── commands/      # CLI command implementations
│       │   ├── daemon.rs      # Daemon process
│       │   └── main.rs
│       └── tests/
├── apps/
│   ├── desktop/               # Tauri app
│   │   ├── src-tauri/         # Rust backend
│   │   └── src/               # React frontend
│   └── vscode-ext/            # VS Code extension
│       ├── src/extension.ts   # Extension entry point
│       └── src/webview/       # Webview UI
├── packages/
│   └── rpc/                   # Shared TypeScript types
├── integ-tests/               # Cross-component tests
└── docs/                      # Documentation
```

**Naming Conventions:**

- **Rust:** snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
- **TypeScript:** camelCase for functions/variables, PascalCase for types/interfaces/components
- **Files:** kebab-case for filenames (knowledge-graph.rs, thought-capture.tsx)
- **Database:** snake_case for table/column names (thoughts table, workflow_stage column)

**Coding Standards:**

- **Rust:**
  - Always use type annotations for function parameters and returns
  - Prefer `?` operator over explicit match for error propagation
  - Use thiserror for custom error types
  - Document public APIs with `///` doc comments
  - Run `cargo fmt` and `cargo clippy` before commits

- **TypeScript:**
  - Strict mode enabled (`strict: true` in tsconfig.json)
  - Explicit return types for exported functions
  - Prefer functional components with hooks (no class components)
  - PropTypes via TypeScript interfaces (no separate prop-types package)

- **Testing:**
  - Aim for 70%+ code coverage on core logic
  - Test both happy path and error scenarios
  - Integration tests for API boundaries
  - Snapshot tests for UI components (React Testing Library)

**Documentation Standards:**

- **Code Comments:** Explain "why" not "what" (code should be self-explanatory)
- **README Files:** Each package has README with overview, setup, usage
- **API Documentation:** Generate from doc comments (rustdoc for Rust, TSDoc for TypeScript)
- **Architecture Decision Records (ADRs):** Document major technical decisions in `docs/adrs/`

### 4.4 Deployment and Operations

**Build Process Integration:**

- **Monorepo Orchestration:** Turborepo manages build dependencies across packages
- **Rust Compilation:**
  - `cargo build --release -p snps-core -p snps-cli` for production builds
  - Binary output: `target/release/snps` (CLI), libraries linked by Tauri app
- **Frontend Builds:**
  - `pnpm build` in desktop/vscode-ext triggers Vite/webpack builds
  - Tauri bundles React build with Rust binary into single executable
- **Cross-Compilation:** GitHub Actions matrix builds for macOS, Linux, Windows
- **Versioning:** Semantic versioning (semver), version.txt or Cargo.toml as source of truth

**Deployment Strategy:**

- **MVP Phase (Local-Only):**
  - CLI distributed as binary via GitHub Releases (dmg/pkg for macOS, exe for Windows, deb/AppImage for Linux)
  - Desktop app distributed via same method (Tauri bundles)
  - VS Code extension published to VS Code Marketplace
  - No cloud infrastructure required (local-first architecture)

- **Web UI (Static Hosting):**
  - Build: `pnpm build` in web-ui package
  - Deploy: Static files to Vercel/Netlify/CloudFlare Pages
  - CDN distribution for global access
  - OAuth redirect URLs point to hosted domain

- **Future (Optional Cloud Sync):**
  - Cloud sync service for multi-device (AWS/GCP/Azure)
  - Database hosted in user's cloud account (PostgreSQL for multi-team)
  - End-to-end encryption for cloud-synced data

**Monitoring and Logging:**

- **Local Logging:**
  - Rust: `tracing` crate with `RUST_LOG` environment variable for level control
  - Log files: `~/.pmsynapse/logs/daemon.log` with rotation (10MB max, 5 files retained)
  - VS Code extension: Output channel for extension logs
  - Desktop app: Tauri log plugin for structured logging

- **Error Reporting:**
  - Local error logs with stack traces (no external error tracking for MVP)
  - Future: Optional Sentry integration with user opt-in

- **Performance Monitoring:**
  - Local metrics collection (thought capture time, query latency, sync duration)
  - Aggregated locally, viewable in Settings panel
  - No external APM for MVP (privacy-first approach)

**Configuration Management:**

- **User Configuration:**
  - File: `~/.pmsynapse/config.yaml` (API keys, preferences, IDLC customization)
  - Tauri store plugin for desktop app settings (theme, notifications)
  - VS Code settings for extension configuration

- **Secrets Management:**
  - API keys stored in system keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
  - OAuth tokens encrypted at rest using keyring crate
  - Never store secrets in plaintext config files

- **Environment Variables:**
  - `RUST_LOG`: Control logging level (debug, info, warn, error)
  - `PMSYNAPSE_DATA_DIR`: Override default data directory
  - `PMSYNAPSE_DAEMON_PORT`: Custom daemon RPC port (default: 50051)

### 4.5 Risk Assessment and Mitigation

**Technical Risks:**

**Risk 1: Tauri 2.0 Stability**
- **Description:** Tauri 2.0 is currently alpha/beta; API changes possible before stable release
- **Impact:** Breaking changes could require desktop app refactoring
- **Likelihood:** Medium (Tauri team committed to Q1 2026 stable)
- **Mitigation:** Pin to specific Tauri alpha version, track release notes closely, allocate 2-week buffer for migration to stable

**Risk 2: SQLite → CozoDB Migration Complexity**
- **Description:** Migrating from relational (SQLite) to graph database (CozoDB) is non-trivial
- **Impact:** Could require rewriting entire data layer, breaking compatibility
- **Likelihood:** High (architecture mismatch between relational and graph models)
- **Mitigation:** Abstract database layer from day one (FR32), design schema with graph migration in mind, validate migration path with prototype

**Risk 3: LLM API Cost and Availability**
- **Description:** OpenAI/Anthropic API costs could be prohibitive; rate limits could block features
- **Impact:** MVP becomes too expensive to run or features are unavailable during high usage
- **Likelihood:** Medium (depends on user adoption and prompt engineering efficiency)
- **Mitigation:** Aggressive caching (FR29), local model support (Ollama), cost monitoring dashboard, usage limits per user

**Integration Risks:**

**Risk 4: Linear/GitHub API Changes**
- **Description:** Third-party APIs could introduce breaking changes without notice
- **Impact:** Integration features stop working, user workflows broken
- **Likelihood:** Low-Medium (established APIs rarely break compatibility)
- **Mitigation:** API version pinning, integration test suite with real API calls (nightly), graceful degradation (NFR21-22), queue-and-retry (NFR22)

**Risk 5: OAuth Flow Complexity**
- **Description:** OAuth 2.0 callback requires public URL; local-first architecture complicates this
- **Impact:** Users can't authenticate with Linear/GitHub without additional setup
- **Likelihood:** Medium (OAuth inherently requires server-side components)
- **Mitigation:** Provide localhost callback option with port forwarding instructions, offer personal access token alternative, future: simple proxy service for OAuth callbacks

**Deployment Risks:**

**Risk 6: Cross-Platform Binary Distribution**
- **Description:** Building and distributing binaries for macOS, Linux, Windows is complex (code signing, notarization, installers)
- **Impact:** Users can't install/run PMSynapse on their platform, security warnings scare users away
- **Likelihood:** High (inevitable for desktop app distribution)
- **Mitigation:** GitHub Actions CI/CD for automated builds, invest in code signing certificates (macOS, Windows), provide multiple install methods (dmg, pkg, exe, deb, AppImage, homebrew)

**Risk 7: VS Code Extension Publishing**
- **Description:** VS Code Marketplace requires publisher verification, review process can be slow
- **Impact:** Extension availability delayed, early adopters can't install
- **Likelihood:** Low (extension publishing is well-documented)
- **Mitigation:** Complete publisher verification early, prepare extension package in advance, have manual installation option (vsix file)

**Mitigation Strategies Summary:**

- **Abstraction Layers:** Database (FR32), LLM providers, integration APIs to minimize lock-in
- **Graceful Degradation:** Offline mode (FR29), queue-and-retry (NFR22), clear error messaging (NFR21)
- **CI/CD Automation:** Automated testing, cross-platform builds, release process
- **User Control:** Manual override of automation, local data ownership, optional cloud features
- **Cost Management:** Caching, usage limits, cost monitoring, local model fallback

---

## 5. Epic and Story Structure

### 5.1 Epic Approach

**Epic Structure Decision:** Two sequential epics (Epic 1a: Core, Epic 1b: Complete)

**Rationale:**

The PMSynapse MVP represents a comprehensive product enhancement that benefits from phased delivery. Rather than a single large epic, we split into two sequential epics that enable early pilot feedback while maintaining the integrated product vision.

**Why Two Epics Instead of One:**

- **Faster Time-to-Pilot:** Epic 1a delivers minimally viable features in 2 months vs 5 months for full MVP
- **Early User Feedback:** First pilot team validates core assumptions before building advanced features in Epic 1b
- **Risk Mitigation:** If Epic 1a reveals fundamental issues (UX problems, technical debt, scope misalignment), we can adjust Epic 1b before investing full effort
- **Team Morale:** Completing Epic 1a provides psychological win and momentum for Epic 1b
- **Scope Flexibility:** If resources constrain or priorities shift, Epic 1a stands alone as "core MVP" while Epic 1b becomes "enhanced MVP"

**Why Not More Than Two Epics:**

The 8 MVP features are interdependent and share infrastructure. Splitting into 3+ epics would create artificial boundaries and complicate story dependencies. Two epics balance early delivery with cohesive product development.

**Alignment with Brownfield Context:**

The existing PMSynapse codebase has foundational architecture (thoughts system, daemon, IDLC configuration, BAML structure). Epic 1a builds on this foundation to deliver pilot-ready functionality. Epic 1b adds polish and advanced features for broader adoption.

---

### 5.2 Epic 1a: PMSynapse MVP Core (Pilot-Ready)

**Epic Goal:** Enable first pilot team to use PMSynapse daily for basic thought-to-ticket workflow without interrupting their development flow.

**Target Timeline:** 2 months (8 weeks, ~16 sprints if using 1-week sprints)

**Core Features:**
- Thought capture via CLI with <30 second flow preservation (FR1, FR26)
- Knowledge graph storage in SQLite with metadata (FR23, FR24)
- Basic workflow suggestion (research/plan/implement) with user override (FR3, FR4)
- Linear integration for ticket creation from thoughts (FR15, FR17)
- CLI commands for all core operations (FR11, FR12)
- Daemon process managing LLM calls and Linear sync (FR21, FR22)
- Offline mode with queue-and-retry (FR29, NFR21-22)

**Out of Scope (Deferred to Epic 1b):**
- VS Code extension
- Desktop app and knowledge graph visualization
- GitHub integration
- Research document generation
- Plan automation (manual planning via CLI in Epic 1a)
- Web UI

**Story Count:** 12-15 stories

**Story Themes:**
1. **Foundation (Stories 1a.1-1a.3):** Database schema, RPC contracts, daemon startup
2. **Thought Capture (Stories 1a.4-1a.6):** CLI thought capture, metadata, workflow suggestion
3. **Linear Integration (Stories 1a.7-1a.10):** OAuth flow, ticket creation, status sync, queue-and-retry
4. **Offline Mode & Polish (Stories 1a.11-1a.12):** LLM caching, error handling, documentation

**Vertical Slice Checkpoints:**
- **Checkpoint 1 (Story 1a.3):** Thought captured via CLI → stored in SQLite → queryable via CLI
- **Checkpoint 2 (Story 1a.6):** Workflow suggestion displayed → user override works → thought updated in graph
- **Checkpoint 3 (Story 1a.10):** Thought → Linear ticket creation → status sync back to graph
- **Checkpoint 4 (Story 1a.12):** Full offline cycle → queue → auto-retry when online

**Epic Exit Criteria:**
- ✓ One pilot team onboarded and using daily (5+ thoughts captured per week per developer)
- ✓ Thought capture averages <30 seconds from CLI invocation to return-to-terminal
- ✓ Linear integration creates tickets with <5 second latency (when online)
- ✓ Offline mode queues operations successfully and retries when connectivity restored
- ✓ Zero data loss incidents during pilot period
- ✓ User satisfaction >7/10 for "doesn't interrupt my workflow"
- ✓ All 12-15 stories completed with acceptance criteria met
- ✓ Documentation complete (README, CLI help, setup guide)

**Success Metrics:**
- 60%+ weekly active usage from pilot team
- Average 5+ thoughts captured per developer per week
- <10% user abandonment rate
- 10+ thoughts progressed through full workflow (capture → Linear ticket → completion)

---

### 5.3 Epic 1b: PMSynapse MVP Complete (Feature Complete)

**Epic Goal:** Deliver full MVP feature set enabling three pilot teams to adopt PMSynapse with rich UI experiences (VS Code extension, desktop app) and comprehensive integrations (GitHub, research/plan automation).

**Target Timeline:** 2-3 months after Epic 1a completion (8-12 weeks)

**Dependencies:** Epic 1a must be complete and first pilot team successfully using the system.

**Core Features:**
- VS Code extension with thought capture panel and knowledge graph sidebar (FR2, FR9, FR10)
- Desktop app with knowledge graph visualization (FR13, FR14)
- GitHub integration for commit/PR linking (FR18, FR19)
- Research document generation with templates (FR5, FR6)
- Plan automation with structured format (FR7, FR8)
- Web UI for read-only graph visualization (stakeholder view)
- Multi-user authentication and role-based access (FR31)
- Database schema documentation and CozoDB migration preparation (FR30, FR32)

**Story Count:** 13-18 stories

**Story Themes:**
1. **VS Code Extension (Stories 1b.1-1b.5):** Extension infrastructure, thought capture panel, graph sidebar, workflow status view
2. **Desktop App (Stories 1b.6-1b.9):** Tauri app foundation, graph visualization, dashboard, settings panel
3. **Advanced Workflow (Stories 1b.10-1b.12):** Research generation, plan automation, document templates
4. **GitHub Integration (Stories 1b.13-1b.14):** Commit linking, PR status display
5. **Multi-User & Polish (Stories 1b.15-1b.18):** Authentication, RBAC, web UI, schema documentation, pilot expansion

**Vertical Slice Checkpoints:**
- **Checkpoint 1 (Story 1b.5):** VS Code extension captures thought → syncs to daemon → appears in graph sidebar
- **Checkpoint 2 (Story 1b.9):** Desktop app displays knowledge graph → click node → view thought details
- **Checkpoint 3 (Story 1b.12):** Thought → workflow suggests research → auto-generate research doc → link in graph
- **Checkpoint 4 (Story 1b.14):** GitHub commit with `[THOUGHT-123]` → auto-links to thought → shows in graph visualization
- **Checkpoint 5 (Story 1b.18):** Three pilot teams operating concurrently with multi-user auth

**Epic Exit Criteria:**
- ✓ Three pilot teams onboarded and using system (15+ developers total)
- ✓ 60%+ weekly active usage sustained across all pilot teams for 30+ days
- ✓ VS Code extension published to marketplace with >7/10 user rating
- ✓ Desktop app distributed via GitHub Releases with installers for macOS/Linux/Windows
- ✓ All 8 core MVP features working end-to-end
- ✓ 70%+ daily active usage among pilot teams after 30 days
- ✓ Average 5+ thoughts per developer per week sustained
- ✓ 10+ thoughts progressed through full workflow including research/plan stages
- ✓ Knowledge graph queries consistently <500ms (NFR2)
- ✓ Documentation complete (user guides, architecture docs, API docs)
- ✓ At least 1 pilot team expresses interest in paid conversion

**Success Metrics:**
- NPS >40 from pilot teams
- 40% reduction in new developer onboarding time (measured via pilot team feedback)
- Developers find architectural context in <2 minutes (vs baseline 30+ minutes)
- Integration uptime >95% (NFR6)
- Zero data loss incidents

---

### 5.4 Story Complexity and Sequencing Guidelines

**Story Complexity Estimation (T-Shirt Sizing):**

- **Small (S) - 1-2 days:**
  - Database schema additions (add column, create index)
  - Simple CLI commands (query, list, filter)
  - Basic UI components without complex state
  - Configuration file changes
  - Documentation updates

- **Medium (M) - 3-5 days:**
  - RPC endpoint implementation with error handling
  - CLI command with validation and formatting
  - React component with local state management
  - Database migration with data preservation
  - Integration test suites

- **Large (L) - 1-2 weeks:**
  - OAuth 2.0 flow implementation
  - Workflow engine routing logic
  - Knowledge graph visualization with interactions
  - VS Code extension webview infrastructure
  - API integration with queue-and-retry

- **Extra Large (XL) - 2-3 weeks:**
  - VS Code extension complete infrastructure (activation, commands, panels)
  - Desktop app foundation (Tauri setup, IPC, basic routing)
  - Multi-user authentication and RBAC system
  - Database abstraction layer for CozoDB migration

**Story Sequencing Principles:**

1. **Foundation First:** Database schema, RPC contracts, daemon initialization before feature stories
2. **Vertical Slices:** Every 3-5 stories, complete one end-to-end user flow
3. **Dependencies Explicit:** Stories with dependencies clearly marked; blocking stories scheduled first
4. **Risk Early:** High-complexity stories (OAuth, workflow engine) tackled early to surface issues
5. **Parallel Paths:** Independent stories (UI components, documentation) can be parallelized across team members
6. **Integration Points:** Stories that integrate multiple systems (Linear + workflow, GitHub + graph) sequenced after dependent stories complete

**Theme Exit Criteria (Epic 1a):**

- **Theme 1 (Foundation) Complete:** Database created, migrations working, daemon starts successfully, RPC ping/pong works
- **Theme 2 (Thought Capture) Complete:** User can capture thought via CLI, see workflow suggestion, override if desired, query thoughts from CLI
- **Theme 3 (Linear Integration) Complete:** OAuth authenticated, tickets created in Linear from thoughts, status syncs back within <5 sec, queue-and-retry works offline
- **Theme 4 (Polish) Complete:** Offline mode functional, error messages helpful, documentation complete, pilot team successfully onboarded

**Theme Exit Criteria (Epic 1b):**

- **Theme 1 (VS Code Extension) Complete:** Extension installs, thought capture panel works, graph sidebar displays thoughts, workflow status visible
- **Theme 2 (Desktop App) Complete:** App launches, graph visualization interactive, dashboard shows metrics, settings configurable
- **Theme 3 (Advanced Workflow) Complete:** Research docs auto-generate, plans created with structure, documents linked in graph
- **Theme 4 (GitHub Integration) Complete:** Commits link to thoughts via convention, PRs display in graph, implementation progress visible
- **Theme 5 (Multi-User & Polish) Complete:** RBAC working, three pilot teams operating, web UI read-only access available, schema documented

---

## 6. Epic Details with Stories

**Note:** This section provides representative stories from both Epic 1a and Epic 1b to demonstrate story format, acceptance criteria, and integration verification requirements. Complete story breakdown (all 25-33 stories) would be detailed during sprint planning with the development team.

---

### 6.1 Epic 1a: PMSynapse MVP Core (Pilot-Ready)

**Epic Goal:** Enable first pilot team to use PMSynapse daily for basic thought-to-ticket workflow without interrupting their development flow.

---

#### Story 1a.1: Define Knowledge Graph SQLite Schema

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

#### Story 1a.2: Implement RPC Interface Between CLI and Daemon

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

#### Story 1a.4: CLI Thought Capture with Metadata

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

#### Story 1a.6: Workflow Suggestion with User Override

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

#### Story 1a.7: Linear OAuth 2.0 Authentication

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

#### Story 1a.9: Linear Ticket Creation from Thoughts

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

### 6.2 Epic 1b: PMSynapse MVP Complete (Feature Complete)

**Epic Goal:** Deliver full MVP feature set enabling three pilot teams to adopt PMSynapse with rich UI experiences and comprehensive integrations.

---

#### Story 1b.1: VS Code Extension Infrastructure and Activation

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

#### Story 1b.3: VS Code Thought Capture Panel

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

#### Story 1b.7: Desktop App Knowledge Graph Visualization

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

#### Story 1b.12: Research Document Auto-Generation

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

### 6.3 Remaining Stories

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
