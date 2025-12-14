# 2. Requirements

## 2.1 Functional Requirements

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

## 2.2 Non-Functional Requirements

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

## 2.3 Compatibility Requirements

**CR1: Existing API Compatibility** - The enhancement shall maintain backward compatibility with existing `snps thoughts` CLI commands. Current thought capture functionality must continue working without breaking changes to command syntax or data formats.

**CR2: Database Schema Compatibility** - The knowledge graph schema evolution shall support migration from existing SQLite schema without data loss. Existing thoughts, metadata, and relationships must be preserved during schema updates. The architecture shall support future migration to CozoDB (graph + vector database) with minimal disruption to application layer code.

**CR3: UI/UX Consistency** - New UI components (VS Code extension, desktop app, web UI) shall follow existing design patterns from shadcn/ui component library and maintain visual consistency with Tauri 2.0 styling. Color schemes, typography, and interaction patterns shall be uniform across interfaces.

**CR4: Integration Compatibility** - The system shall maintain compatibility with existing Linear integration foundation. Current Linear API contracts, OAuth flow, and data synchronization patterns shall be preserved and extended (not replaced).

---
