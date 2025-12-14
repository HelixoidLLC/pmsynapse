# Project Brief: PMSynapse

## Executive Summary

**PMSynapse** is an AI-enabled project management system that automates thought management through workflow steps aligned to human project delivery. The platform solves the chronic problem of knowledge fragmentation in software development—where critical decisions, context, and architectural insights are lost in scattered conversations, undocumented code, and tribal knowledge.

Target markets include development teams working on brownfield projects, enterprise architects managing legacy systems, and executives seeking visibility into idea-to-implementation progression. The key value proposition is **non-intrusive knowledge capture** that serves both executives (strategic visibility) and developers (workflow automation) while uniquely enabling brownfield teams to restore years of lost project knowledge without disrupting existing codebases.

---

## Problem Statement

**Current State & Pain Points:**

Software development teams, especially those working on legacy systems, face a critical knowledge management crisis. Years of architectural decisions, technical discussions, and contextual rationale exist only in fragmented forms—scattered across Slack conversations, buried in Jira tickets, trapped in individual developers' minds, or never documented at all. When team members leave, retire, or switch projects, this tribal knowledge vanishes permanently.

Developers actively avoid traditional project management systems because they interrupt flow state and demand administrative overhead that feels disconnected from actual coding work. The result: teams choose between documentation (which slows them down) and velocity (which creates knowledge debt).

**Impact of the Problem:**

- **Onboarding friction:** New developers spend weeks or months reverse-engineering architectural decisions that should be documented
- **Decision paralysis:** Without historical context, teams make suboptimal choices or repeat past mistakes
- **Executive blindness:** Leadership lacks visibility into idea progression from conception through delivery
- **Technical debt accumulation:** Undocumented assumptions become landmines in the codebase
- **Brownfield modernization risk:** Legacy system updates carry high risk due to lost knowledge about original design rationale

**Why Existing Solutions Fall Short:**

Current tools fall into two categories, both inadequate:

1. **Heavy PM systems (Jira, Asana, Monday):** Designed for executives and project managers, these systems interrupt developer workflow, require duplicate data entry, and treat documentation as separate from implementation. Developers bypass them whenever possible.

2. **Developer-first tools (Git, IDE extensions):** Capture code but not context. Comments and commit messages are too granular for strategic understanding. No bridge to executive visibility or architectural documentation.

Neither category solves brownfield knowledge restoration. Tools assume greenfield projects with clean documentation from day one.

**Urgency & Importance:**

The software industry's massive technical debt crisis (estimated at $5+ trillion globally) stems partly from this knowledge management failure. As the Great Resignation continues and remote work becomes permanent, tribal knowledge transfer becomes even more critical. Teams need solutions **now** that:
- Don't require changing existing workflows
- Capture knowledge passively during normal work
- Serve multiple stakeholders simultaneously
- Work with brownfield projects (the majority of real-world software)

---

## Proposed Solution

**Core Concept & Approach:**

PMSynapse operates as an **AI-powered executive assistant for software teams**—always available, never intrusive, intelligently guiding projects from ideation through delivery. Instead of forcing developers into rigid workflows, the system observes natural work patterns and captures knowledge automatically, presenting itself only when helpful.

The platform implements a **context-aware thought management system** that:
- Accepts thoughts/ideas through multiple interfaces (CLI, VS Code extension, web portal, keyboard shortcuts)
- Uses AI to determine the appropriate workflow path (research → plan → implement → deliver)
- Intelligently decides which steps are necessary vs. skippable based on context
- Allows instant user override of any automation decision
- Captures knowledge as a byproduct of normal development work

**Key Differentiators:**

1. **Non-Intrusive by Design:** Unlike traditional PM tools, PMSynapse follows the "executive assistant" model—available on the side, summoned via keyboard shortcut, never blocking developer workflow

2. **Brownfield Knowledge Restoration:** AI agents can ingest existing codebases, reverse-engineer architectural relationships, and synthesize scattered documentation (PRDs, Jira tickets, Confluence pages) into coherent knowledge graphs

3. **Multi-Stakeholder Architecture:** Single system with different interfaces—developers get CLI/IDE tools, executives get dashboards, architects get visualization tools—all viewing the same underlying knowledge graph

4. **Context-Aware Automation:** System understands when documentation is needed vs when to get out of the way. A quick bug fix doesn't need a PRD; a new feature might

5. **Integration-First Philosophy:** Works with existing tools (Linear, Jira, GitHub, Microsoft ecosystem) rather than replacing them. Zero-touch integration for brownfield adoption

**Why This Will Succeed:**

Traditional PM tools fail because they optimize for control at the expense of flow. PMSynapse inverts this: it optimizes for developer velocity and captures documentation as a side effect. The psychological insight is critical—developers need **immediate gratification** (dopamine from seeing code work). By accelerating idea-to-implementation movement rather than slowing it down, PMSynapse aligns with intrinsic motivation instead of fighting it.

Brownfield capability is the strategic moat. Every competing tool assumes clean-slate projects. PMSynapse is the only solution designed for the reality that most software development happens on legacy systems with years of undocumented history.

**High-Level Product Vision:**

A developer has an idea at 2am. They hit a keyboard shortcut, speak/type the thought, and PMSynapse captures it. The AI determines it needs research first—suggests finding similar implementations in the existing codebase. Developer reviews suggestions, creates a research document that's auto-linked to the original thought. When ready to implement, system generates tickets, links them to research, and tracks progress. Meanwhile, executive dashboard shows this idea moving through stages. Architect gets notified because it touches a critical system. All automatically. All non-intrusively.

The vision: **knowledge capture you don't have to think about**, serving everyone who needs it, from the moment of ideation through post-deployment analysis.

---

## Target Users

### Primary User Segment: Individual Contributors (Developers & Engineers)

**Demographic/Firmographic Profile:**
- Software developers and engineers (5-20 years experience)
- Working in organizations with 50-10,000+ employees
- Teams managing legacy systems or brownfield projects (3+ years old codebases)
- Mix of remote, hybrid, and in-office workers
- Industries: SaaS, financial services, healthcare, enterprise software, e-commerce
- Technical stack: Polyglot (multiple languages), microservices or monolithic architectures

**Current Behaviors & Workflows:**
- Use IDE (VS Code, JetBrains) as primary work environment 8+ hours daily
- Rely on Git for version control and code review
- Jump between tools: Slack for communication, Jira/Linear for tickets, Confluence/Notion for docs
- Avoid documentation tasks until absolutely necessary
- Store knowledge in personal notes, comments, or not at all
- Depend on tribal knowledge and "ask the senior dev" for architectural context

**Specific Needs & Pain Points:**
- Need to understand "why" behind legacy code decisions without archeological digs
- Frustrated by context-switching between development and documentation tools
- Want to capture thoughts/ideas immediately without breaking flow state
- Struggle with onboarding new team members efficiently
- Fear making changes to undocumented systems ("here be dragons")
- Desire visibility into whether their ideas are progressing or stuck in management limbo

**Goals They're Trying to Achieve:**
- Ship code faster without accumulating technical debt
- Document decisions without it feeling like overhead
- Preserve architectural knowledge for future team members
- Reduce time spent in status meetings and ticket grooming
- Build confidence when modifying legacy systems
- See ideas move from concept to production quickly

### Secondary User Segment: Engineering Leadership (CTOs, VPs Engineering, Engineering Managers)

**Demographic/Firmographic Profile:**
- Technical leaders responsible for 5-500+ person engineering organizations
- Managing budgets $500K-$50M+
- Balancing technical debt vs feature delivery tradeoffs
- Reporting to CEO/Board on engineering velocity and technical health
- Organizations undergoing digital transformation or modernization initiatives
- Mix of hands-on technical and pure management responsibilities

**Current Behaviors & Workflows:**
- Spend significant time in meetings gathering status updates
- Review dashboards (Jira, GitHub analytics) for team velocity metrics
- Make architectural decisions with incomplete information about legacy systems
- Allocate resources between new features and technical debt paydown
- Advocate for engineering needs to non-technical stakeholders
- Struggle to quantify value of documentation and knowledge management

**Specific Needs & Pain Points:**
- Limited visibility into what's actually happening in brownfield codebases
- Difficulty justifying technical debt work to business stakeholders
- Teams losing productivity when senior engineers leave
- Can't accurately estimate effort for legacy system changes
- Need to demonstrate engineering team value to executive leadership
- Want proactive risk identification before problems become crises

**Goals They're Trying to Achieve:**
- Increase team velocity sustainably (not burning out developers)
- Reduce onboarding time for new hires
- Make data-driven decisions about technical debt prioritization
- Improve predictability of delivery timelines
- Retain institutional knowledge as team composition changes
- Communicate engineering progress to non-technical stakeholders effectively

---

## Goals & Success Metrics

### Business Objectives

- **Validate brownfield value proposition:** Successfully onboard 3 pilot brownfield teams within 6 months with >60% weekly active usage
- **Prove non-intrusive design:** Achieve <10% user abandonment rate (vs 40-60% typical for PM tools)
- **Demonstrate multi-stakeholder value:** Simultaneous satisfaction from both developers and engineering leadership in same organization
- **Establish revenue model:** Convert 1 pilot team to paying customer, validating pricing and purchase process

### User Success Metrics

- **Flow preservation:** Average <30 seconds from thought capture to return-to-code
- **Onboarding acceleration:** 40% reduction in time-to-productivity for new developers on brownfield projects
- **Knowledge accessibility:** Developers find architectural context in <2 minutes vs 30+ minutes currently
- **Adoption stickiness:** 70%+ daily active usage among pilot teams after 30 days

### Key Performance Indicators (KPIs)

- **Weekly Active Users:** 60%+ of pilot team using PMSynapse weekly (primary adoption metric)
- **Thoughts Captured:** 5+ ideas/thoughts captured per developer per week (habit formation indicator)
- **Documentation Coverage:** 50%+ of code changes have linked context (vs baseline <10%)
- **Net Promoter Score:** NPS >40 (word-of-mouth potential)
- **Integration Reliability:** 95%+ uptime for tool integrations with <5 second sync

---

## MVP Scope

### Core Features (Must Have)

- **Thought Capture System:** CLI and VS Code extension for capturing ideas/thoughts instantly via keyboard shortcut. Must support <30 second capture time to preserve flow state. Thoughts stored in local knowledge graph with automatic metadata (timestamp, author, project context).

- **Guided Workflow Engine:** Context-aware system that suggests workflow path (research → plan → implement → deliver) based on thought content. AI determines necessary steps with user override capability. Simple decision tree for MVP (expand to ML later).

- **Research Document Generation:** Automated creation of research documents linked to thoughts. Template-based generation with AI-assisted content suggestions. Searchable knowledge base of research findings.

- **Planning Automation:** Convert research into implementation plans with ticket generation. Auto-link plans to originating thoughts and research. Integration with Linear for ticket creation and status tracking.

- **VS Code Extension:** Native IDE integration for thought capture, document browsing, and workflow status visibility. Must not interrupt coding workflow. Side panel UI for knowledge graph navigation.

- **CLI Tool:** Command-line interface for all core functions (thought capture, research creation, plan generation, status queries). Designed for terminal-centric developers. Supports scripting and automation.

- **Knowledge Graph Visualization:** Basic web UI showing relationships between thoughts, research, plans, and code implementations. Read-only for MVP. Enables developers and leadership to see idea progression.

- **Linear Integration:** Two-way sync between PMSynapse and Linear. Automatic ticket creation from plans. Status updates flow back to knowledge graph. <5 second sync latency.

- **GitHub Integration:** Link commits/PRs to thoughts and plans automatically via commit message conventions. Display implementation progress in knowledge graph.

### Out of Scope for MVP

- Mobile applications (iOS/Android)
- Advanced AI codebase analysis for brownfield restoration
- Multi-team/multi-project support
- Public marketing website
- Jira/Confluence/Slack integrations (Linear/GitHub only for MVP)
- Executive dashboard with advanced analytics
- Real-time collaboration features
- Custom workflow configuration (use default research → plan → implement only)
- BMAD framework full integration (basic templates only)
- Advanced security/permissions model (single-team, trusted users only)
- API for third-party integrations
- Automated architectural diagram generation

### MVP Success Criteria

**MVP is successful if:**
- 3 pilot teams successfully onboard and use system for real projects
- 60%+ weekly active usage sustained for 30+ days
- Developers capture average 5+ thoughts per week
- At least 10 thoughts successfully progress through full workflow (capture → research → plan → implement)
- Zero data loss incidents
- <30 second average thought capture time maintained
- Users report satisfaction score >7/10 for "doesn't interrupt my workflow"
- 1 pilot team expresses interest in paid conversion

**MVP fails if:**
- Users abandon system after <2 weeks
- Developers bypass system and return to old habits
- Workflow automation adds friction rather than removing it
- Integration reliability <90% uptime
- No measurable improvement in onboarding time or context accessibility

---

## Post-MVP Vision

### Phase 2 Features

**Brownfield Knowledge Restoration (The Differentiator):**
Once MVP validates core workflow, build the strategic moat—AI-powered codebase analysis that reverse-engineers architectural decisions and relationships. This includes:
- Automated scanning of legacy codebases to identify components and dependencies
- Ingestion and synthesis of scattered documentation (Jira tickets, Confluence pages, GitHub issues, PRDs)
- AI-generated architectural documentation showing "why" behind code decisions
- Component relationship mapping in knowledge graph
- Historical decision timeline reconstruction

**Expanded Integration Ecosystem:**
- Jira (enterprise ticket management)
- Confluence/Notion (documentation platforms)
- Slack/Microsoft Teams (communication)
- Azure DevOps, GitLab (alternative Git platforms)
- Figma (design-to-development linking)

**Mobile Applications:**
- iOS and Android apps for on-the-go thought capture
- Voice-to-text thought entry
- Push notifications for workflow milestones
- Offline mode with sync

**Advanced Executive Dashboard:**
- Real-time idea progression analytics
- Team velocity metrics and trends
- Knowledge graph health indicators
- ROI calculation for documentation investment
- Custom reporting and exports

**Multi-Team/Multi-Project Support:**
- Organization-level knowledge graph spanning multiple teams
- Cross-team knowledge sharing and discovery
- Permissions model for sensitive projects
- Team-specific workflow customization

### Long-term Vision

**The Industry Standard for Brownfield Modernization:**
PMSynapse becomes the default tool that legacy system teams adopt when beginning modernization efforts. Before touching code, teams use PMSynapse to restore lost knowledge, creating a comprehensive understanding of "what we have and why it exists." This foundation dramatically reduces risk and accelerates transformation.

**AI Agent Ecosystem:**
Specialized AI agents handle different aspects of software development:
- Research Agent: Scans codebases, analyzes patterns, synthesizes findings
- Planning Agent: Generates implementation plans, estimates effort, identifies risks
- Documentation Agent: Maintains living documentation that updates with code changes
- Integration Agent: Intelligently routes information between tools based on context
- Onboarding Agent: Creates personalized learning paths for new team members

**Zero-Touch Enterprise Adoption:**
Existing brownfield teams can deploy PMSynapse without modifying their codebase or changing workflows. The system observes existing work patterns (Git commits, Jira activity, Slack conversations) and builds knowledge graphs passively. Teams gain value before committing to active participation.

**Developer Copilot Integration:**
Deep integration with GitHub Copilot, Cursor, and other AI coding assistants. When developers ask "why does this code work this way?", the copilot surfaces PMSynapse knowledge graph context automatically. Architectural decisions become instantly accessible at the point of coding.

### Expansion Opportunities

**Vertical Market Specialization:**
- **Financial Services:** Regulatory compliance documentation, audit trails, SOX requirements
- **Healthcare:** HIPAA compliance, clinical decision documentation, FDA validation paths
- **Government/Defense:** Security clearance tracking, decision audit trails, compliance workflows

**Platform Play:**
- Public API enabling third-party integrations and custom workflows
- Marketplace for community-built templates, integrations, and AI agents
- White-label offering for consulting firms serving enterprise clients

**Enterprise Knowledge Management:**
Expand beyond engineering to become organization-wide knowledge platform:
- Product management (idea → feature → launch workflows)
- Sales engineering (customer conversation → technical solution → implementation)
- Customer success (issue → root cause → resolution → knowledge base)

**Training & Certification:**
- PMSynapse certification program for developers and architects
- Best practices training for brownfield modernization
- Consulting services for enterprise deployment and customization

---

## Technical Considerations

### Platform Requirements

- **Target Platforms:**
  - Desktop: macOS, Linux, Windows (via Tauri 2.0 cross-platform framework)
  - VS Code Extension: Cross-platform (wherever VS Code runs)
  - Web UI: Modern browsers (Chrome, Firefox, Safari, Edge - last 2 versions)
  - CLI: macOS, Linux, Windows (native binaries)

- **Browser/OS Support:**
  - Desktop requires WebKit2GTK on Linux
  - VS Code version 1.85+ required for extension
  - Web UI requires JavaScript enabled, WebSocket support
  - No IE11 support (modern browsers only)

- **Performance Requirements:**
  - Thought capture: <100ms response time from keyboard shortcut to input ready
  - Knowledge graph queries: <500ms for typical relationship traversal
  - Integration sync: <5 second latency for Linear/GitHub updates
  - CLI commands: <1 second execution for common operations
  - Desktop app startup: <3 seconds cold start

### Technology Preferences

- **Frontend:**
  - React 18+ (component library, hooks)
  - shadcn/ui for UI components (accessibility, customization)
  - Tailwind CSS for styling
  - Zustand for state management (lightweight vs Redux)
  - Vite 6 for build tooling

- **Backend:**
  - Rust (async runtime: Tokio)
  - Core library: `snps-core` (knowledge graph, LLM integration, workflow engine)
  - CLI tool: `snps-cli` (daemon, commands)
  - Error handling: thiserror
  - Serialization: serde

- **Database:**
  - SQLite (rusqlite) for MVP knowledge graph storage
  - Local-first architecture (no cloud dependency for core functionality)
  - Future migration path to CozoDB (graph + vector DB capabilities)
  - Consider PostgreSQL for multi-team/cloud deployment in Phase 2

- **Hosting/Infrastructure:**
  - Local-first for MVP (desktop app + local daemon)
  - Web UI hosted statically (Vercel, Netlify, CloudFlare Pages)
  - Future: Cloud sync service for multi-device (AWS, GCP, or Azure)
  - Linear/GitHub OAuth callbacks require public endpoints

### Architecture Considerations

- **Repository Structure:**
  - Monorepo using BAML pattern (Backend + Apps + Monorepo Layout)
  - Rust isolated in `engine/` directory (workspace with `snps-core`, `snps-cli`)
  - Apps in `apps/` (desktop Tauri app, VS Code extension)
  - Shared TypeScript types in `packages/rpc/`
  - Turborepo for orchestration

- **Service Architecture:**
  - Daemon process (`snps daemon`) runs locally, manages knowledge graph
  - CLI, VS Code extension, desktop app all communicate with daemon via RPC
  - Daemon handles LLM API calls, integration syncing, workflow automation
  - Stateless web UI for read-only knowledge graph visualization

- **Integration Requirements:**
  - Linear API (OAuth 2.0, GraphQL)
  - GitHub API (OAuth Apps, REST + GraphQL)
  - LLM providers: Multi-provider support (OpenAI, Anthropic, local models)
  - Future: Jira REST API, Confluence API, Slack API

- **Security/Compliance:**
  - API keys stored in system keychain (not plaintext)
  - OAuth tokens encrypted at rest
  - Local-first architecture minimizes data exposure
  - GDPR compliance: user data stored locally, opt-in for cloud sync
  - Enterprise: SSO integration (SAML, OIDC) in Phase 2
  - Audit logging for compliance-sensitive deployments

---

## Risks & Open Questions

### Key Risks

- **Adoption Risk:** Developers may not change habits even if tool is non-intrusive. Breaking established workflows (even bad ones) requires significant value demonstration. If developers don't capture thoughts consistently, knowledge graph remains sparse and less useful.

- **Technical Complexity - Brownfield AI:** Core differentiator (brownfield knowledge restoration) is technically unproven. AI codebase analysis at scale may be prohibitively expensive, slow, or inaccurate. Without this capability, product becomes "another PM tool" without strategic moat.

- **Integration Reliability:** Dependency on third-party APIs (Linear, GitHub, LLM providers) creates fragility. API changes, rate limits, or downtime outside our control. If integrations break frequently, users lose trust and abandon system.

- **Market Validation:** Assumption that brownfield knowledge fragmentation is urgent enough to pay for. Teams may acknowledge problem but not prioritize solving it. Alternative: teams accept tribal knowledge loss as "cost of doing business."

- **Competition from AI Assistants:** GitHub Copilot, Cursor, and similar AI coding tools may add context/documentation features, making standalone PM tool obsolete. Risk that developer copilots become "good enough" for knowledge management.

- **Solo Founder Bandwidth:** Development pace limited by single person. Feature requests, bug fixes, pilot support, and new development competing for limited time. Risk of burnout or slow progress killing momentum.

- **Pricing Model Uncertainty:** Unclear what customers will pay and in what model (per-seat, per-team, usage-based). Wrong pricing strategy could leave money on table or price out target market.

### Open Questions

- **Workflow Customization:** Should teams be able to customize workflows (beyond research → plan → implement), or does flexibility undermine the opinionated design that makes the system valuable?

- **Open Source vs Proprietary:** Should core engine be open source to build community trust and adoption, or keep proprietary to protect competitive advantage? Hybrid model possible?

- **Pilot Selection Strategy:** Should we target teams we know personally (easier onboarding, more forgiving) or seek diverse unknown teams (better validation but higher risk of failure)?

- **Integration Depth vs Breadth:** Better to integrate deeply with 2-3 tools or broadly with 10+ tools? Does depth (two-way sync, rich metadata) matter more than coverage?

- **AI Model Strategy:** Build on OpenAI/Anthropic APIs, support local models (Ollama, etc.), or both? Privacy-conscious teams may demand local-only option. Cost vs capability tradeoff.

- **Monetization Timing:** When to introduce pricing? Free during pilots, then paid? Freemium with paid tiers? Time-limited trials? Risk of anchoring on "free" vs risk of scaring away pilots with pricing.

- **Multi-Tenant Architecture:** Build for single-team from start, or architect for multi-team/organization from day one? Single-team simpler but may require painful refactor later.

### Areas Needing Further Research

- **Competitive Analysis Deep Dive:** Comprehensive review of Linear, Notion, Coda, Obsidian, and emerging AI-powered tools. What features do they have that we need? What gaps can we exploit?

- **LLM Cost Modeling:** Estimate API costs for typical usage patterns (thoughts per day, codebase analysis frequency, document generation). Determine if costs are sustainable at target price points.

- **Brownfield AI Feasibility Study:** Technical spike to prove/disprove brownfield codebase analysis capabilities. Can we accurately extract architectural decisions from legacy code? What's accuracy vs cost tradeoff?

- **Developer Workflow Ethnography:** Observe actual developer workflows in brownfield teams. Where do knowledge capture opportunities exist? What triggers thought capture? When do developers seek context?

- **Security/Compliance Requirements:** Research specific requirements for target industries (financial services, healthcare). What certifications/audits needed? SOC2, HIPAA, GDPR implications?

- **GTD Methodology Mapping:** Detailed analysis of how Getting Things Done principles map to software development workflows. What's applicable? What needs adaptation?

- **VS Code Extension Limitations:** Technical research into VS Code extension API constraints. What's possible vs impossible for thought capture, graph visualization, workflow integration?

---

## Appendices

### A. Research Summary

**Brainstorming Session (2025-12-13):**
Comprehensive ideation session exploring automation around thought management and workflow steps aligned to human project delivery. Key findings:

- **Core Insights:**
  - Developer psychology matters more than features—systems fail when they interrupt flow state
  - Brownfield knowledge restoration is the key differentiator vs greenfield-focused tools
  - Multi-stakeholder alignment requires different interfaces to same underlying system
  - Context-aware automation is non-negotiable—rigid workflows will be abandoned

- **Strategic Themes:**
  - Non-intrusive integration (executive assistant model, available on-demand)
  - Serving both executives and developers equally
  - Brownfield knowledge restoration capability
  - Leveraging existing research → plan → implement workflow patterns

- **Priority Initiatives:**
  1. Develop shared visualization architecture (foundation for all UIs)
  2. Activate thought-thinking process for existing projects (dogfooding)
  3. Onboard system to brownfield project (real-world validation)

- **Key Questions Explored:**
  - How to make system natural and non-intrusive?
  - What are best practices from ideation to implementation?
  - How to serve executives and enterprise architects?
  - How to restore knowledge in brownfield projects?
  - How to reverse-engineer missing documentation?

Full brainstorming session results: `docs/brainstorming-session-results.md`

**Existing Architecture Research:**
PMSynapse codebase already includes foundational work:
- Thoughts system implementation (CLI-based knowledge capture)
- IDLC (Idea Development Lifecycle) configurable workflows
- Linear integration for ticket management
- Desktop app (Tauri 2.0 + React)
- Knowledge graph storage (SQLite with CozoDB migration path)

Documentation references: `docs/THOUGHTS_SYSTEM.md`, `docs/IDLC_CONFIGURATION.md`, `docs/PROJECT_WORKFLOW.md`

### C. References

**Project Documentation:**
- `CLAUDE.md` - Project overview and development guidelines
- `docs/STARTUP_GUIDE.md` - Daemon and UI setup
- `docs/THOUGHTS_SYSTEM.md` - Thoughts management system
- `docs/THOUGHTS_WORKFLOW_TUTORIAL.md` - Workflow guidance with stories
- `docs/IDLC_CONFIGURATION.md` - Workflow configuration
- `docs/PROJECT_WORKFLOW.md` - Default workflow and Linear integration
- `docs/PMSYNAPSE_CORE_FEATURES.md` - Feature overview
- `docs/12_FACTOR_AGENTS_DESIGN.md` - Agent architecture
- `docs/AI_TEAM_COORDINATION_PATTERNS.md` - Multi-agent patterns
- `docs/CLAUDE_SESSION_PARSER.md` - Claude session management

**Technical Stack References:**
- Tauri 2.0: https://v2.tauri.app/
- React 18: https://react.dev/
- Rust async (Tokio): https://tokio.rs/
- shadcn/ui: https://ui.shadcn.com/
- Linear API: https://developers.linear.app/
- GitHub API: https://docs.github.com/en/rest

**Methodology & Frameworks:**
- Getting Things Done (GTD): David Allen's productivity methodology
- BMAD-METHOD™: Business analysis and ideation framework (used in brainstorming)
- Diataxis: Documentation framework (referenced in project docs)

---

## Next Steps

### Immediate Actions

1. **Review and validate this Project Brief** - Confirm alignment with vision, identify gaps or corrections needed

2. **Break down Priority #1 (Shared Visualization Architecture) into tickets** - Review existing architecture research in docs folder, create detailed implementation tickets in Linear

3. **Define MVP feature prioritization** - Map core features to development phases, identify dependencies and critical path

4. **Conduct brownfield AI feasibility spike** - Technical research to validate/invalidate brownfield codebase analysis capability (2-3 days)

5. **Document pilot selection criteria** - Define what makes a good pilot team, create outreach strategy

6. **Initiate PRD creation process** - Transition from project brief to detailed Product Requirements Document with PM agent

### PM Handoff

This Project Brief provides the full context for **PMSynapse**. The next phase is creating a detailed Product Requirements Document (PRD) that translates this strategic vision into specific, actionable requirements.

**Recommended approach:**
- Work with PM agent in PRD Generation Mode
- Review this brief thoroughly to understand context
- Create PRD section by section following the established template
- Focus on MVP scope initially (defer Phase 2 features to separate PRD)
- Validate technical feasibility assumptions during PRD process
- Ask for clarification or suggest improvements as needed

**Key areas requiring PM attention:**
- User stories for each core feature (thought capture, workflow engine, integrations)
- Acceptance criteria for "non-intrusive" design (quantify <30 second capture time, etc.)
- Integration specifications (Linear API contracts, GitHub webhook handling)
- Knowledge graph data model and relationships
- VS Code extension UX flows and constraints
- Success metrics implementation (how to measure WAU, thought capture rate, etc.)

**Critical questions to address in PRD:**
- Exact workflow routing logic (when research needed vs optional?)
- Data persistence model (SQLite schema, migration strategy)
- Error handling for integration failures
- Offline mode behavior and sync conflict resolution
- Initial onboarding flow for new users/teams

---

*Project Brief completed 2025-12-13*

