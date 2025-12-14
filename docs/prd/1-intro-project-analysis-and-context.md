# 1. Intro Project Analysis and Context

## 1.1 Existing Project Overview

### 1.1.1 Analysis Source

**Source:** IDE-based fresh analysis combined with existing project documentation

The PMSynapse project has extensive documentation in the `docs/` folder (25+ documents) including:
- Comprehensive project brief (`docs/brief.md`)
- Brainstorming session results documenting strategic direction
- Architecture and design documents (12-Factor Agents, AI Team Coordination, UI patterns)
- System-specific documentation (Thoughts System, IDLC Configuration, Project Workflow)
- Technical research (BAML best practices, database options, VS Code extension architecture)

**Note:** No formal `document-project` analysis was previously run. This PRD leverages existing organic documentation created during project development.

### 1.1.2 Current Project State

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

## 1.2 Documentation Analysis

### 1.2.1 Available Documentation

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

## 1.3 Enhancement Scope Definition

### 1.3.1 Enhancement Type

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

### 1.3.2 Enhancement Description

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

### 1.3.3 Impact Assessment

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

## 1.4 Goals and Background Context

### 1.4.1 Goals

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

### 1.4.2 Background Context

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

## 1.5 Change Log

| Change | Date | Version | Description | Author |
|--------|------|---------|-------------|--------|
| Initial draft | 2025-12-13 | v1.0 | Created brownfield PRD from project brief foundation | PM Agent (John) |

---
