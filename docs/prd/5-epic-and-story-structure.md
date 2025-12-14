# 5. Epic and Story Structure

## 5.1 Epic Approach

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

## 5.2 Epic 1a: PMSynapse MVP Core (Pilot-Ready)

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

## 5.3 Epic 1b: PMSynapse MVP Complete (Feature Complete)

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

## 5.4 Story Complexity and Sequencing Guidelines

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
