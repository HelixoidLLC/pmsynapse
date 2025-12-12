# Thoughts Workflow Tutorial

A comprehensive guide on **when** and **how** to create Thoughts, and how they interact with PRDs, specs, and the development lifecycle.

> Based on analysis of [HumanLayer](https://github.com/humanlayer/humanlayer), [Advanced Context Engineering](https://github.com/humanlayer/advanced-context-engineering-for-coding-agents), and [Catalyst](https://github.com/coalesce-labs/catalyst) workflows.

---

## Table of Contents

1. [Core Philosophy](#core-philosophy)
2. [When to Create Each Thought Type](#when-to-create-each-thought-type)
3. [The Research → Plan → Implement → Validate Workflow](#the-research--plan--implement--validate-workflow)
4. [How Thoughts Interact with PRDs and Specs](#how-thoughts-interact-with-prds-and-specs)
5. [AI Agent Integration](#ai-agent-integration)
6. [Step-by-Step Examples](#step-by-step-examples)
7. [Best Practices](#best-practices)

---

## Core Philosophy

### The Problem Thoughts Solve

AI assistants (and humans) lose context between sessions. Thoughts provide:

1. **Persistent Memory** - Knowledge survives session boundaries
2. **Structured Discovery** - Research before planning, planning before implementing
3. **High-Leverage Review** - Human attention where errors multiply most
4. **Team Collaboration** - Shared knowledge reduces duplicate work

### The Context Quality Hierarchy

From HumanLayer's context engineering principles:

```
PRIORITY (highest to lowest)
├── 1. CORRECTNESS   - Wrong information is most damaging
├── 2. COMPLETENESS  - Missing details create problems
├── 3. SIZE          - Excess content degrades performance
└── 4. TRAJECTORY    - Context should support forward progress
```

**Key Insight**: Research errors propagate through thousands of lines of code. Planning errors affect hundreds. Implementation errors are isolated.

```
REVIEW EFFORT ALLOCATION

Research Documents:    ████████████████████  40%
Plans:                 ██████████████        30%
Code:                  ██████████            20%
Other:                 █████                 10%
```

---

## When to Create Each Thought Type

### Research Documents

**Create when:**
- Starting work on unfamiliar code/feature
- Before making architectural decisions
- When multiple approaches exist
- When you don't understand how something works
- Before writing any significant code

**Don't create when:**
- The task is trivial (< 30 min)
- You already have deep knowledge of the area
- It's a simple bug fix with obvious solution

```bash
# Create research document
snps thoughts new research "Authentication Flow Analysis"
```

**Template triggers:**
| Trigger | Create Research? |
|---------|------------------|
| "How does X work?" | Yes |
| "What's the best approach for Y?" | Yes |
| "Fix typo in file Z" | No |
| "Add new feature that touches 5+ files" | Yes |
| "Implement ticket with unclear requirements" | Yes |

### Plans

**Create when:**
- After completing research on a non-trivial task
- Before implementing features that span multiple files
- When changes require coordination (database + API + UI)
- Before refactoring existing systems

**Don't create when:**
- Task is < 1 hour
- Changes are isolated to single file
- Following an existing plan

```bash
# Create plan document
snps thoughts new plan "OAuth2 Implementation"
```

**Plan triggers:**
| Trigger | Create Plan? |
|---------|--------------|
| Research completed for complex task | Yes |
| Ticket has multiple acceptance criteria | Yes |
| Changes touch 3+ files | Usually yes |
| Simple bug fix | No |
| Adding a new utility function | No |

### Ticket Context

**Create when:**
- Starting work on any tracked ticket/issue
- Requirements need clarification
- You have questions for stakeholders
- The ticket will span multiple sessions

```bash
# Create ticket context
snps thoughts new ticket "PROJ-456"
```

### PR Descriptions

**Create when:**
- Before opening a pull request
- When changes need explanation for reviewers
- To document decisions made during implementation

```bash
# Create PR context
snps thoughts new pr "Add OAuth2 Support"
```

### Personal Notes (Scratch/Journal)

**Create when:**
- Quick notes that don't fit other categories
- Daily work logs
- Personal reminders

```bash
# Create scratch note
snps thoughts new scratch "API Ideas" --scope personal

# Create daily journal
snps thoughts new journal --scope personal
```

---

## The Research → Plan → Implement → Validate Workflow

This is the core workflow pattern from HumanLayer:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    RESEARCH → PLAN → IMPLEMENT → VALIDATE                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐                                                            │
│  │  RESEARCH   │  "Understand before you act"                               │
│  │             │                                                            │
│  │  • Explore codebase                                                      │
│  │  • Map relationships                                                     │
│  │  • Document findings                                                     │
│  │  • Identify options                                                      │
│  │                                                                          │
│  │  Output: thoughts/shared/research/                                       │
│  └──────┬──────┘                                                            │
│         │                                                                    │
│         ▼                                                                    │
│  ┌─────────────┐                                                            │
│  │    PLAN     │  "Think before you code"                                   │
│  │             │                                                            │
│  │  • Define phases                                                         │
│  │  • Specify exact changes                                                 │
│  │  • Set success criteria                                                  │
│  │  • Get human approval                                                    │
│  │                                                                          │
│  │  Output: thoughts/shared/plans/                                          │
│  └──────┬──────┘                                                            │
│         │                                                                    │
│         ▼                                                                    │
│  ┌─────────────┐                                                            │
│  │ IMPLEMENT   │  "Execute methodically"                                    │
│  │             │                                                            │
│  │  • Follow plan phases                                                    │
│  │  • Update checkboxes                                                     │
│  │  • Verify each phase                                                     │
│  │  • Adapt to reality                                                      │
│  │                                                                          │
│  │  Output: Code + updated plan                                             │
│  └──────┬──────┘                                                            │
│         │                                                                    │
│         ▼                                                                    │
│  ┌─────────────┐                                                            │
│  │  VALIDATE   │  "Confirm it works"                                        │
│  │             │                                                            │
│  │  • Run automated tests                                                   │
│  │  • Manual verification                                                   │
│  │  • Document learnings                                                    │
│  │  • Create PR description                                                 │
│  │                                                                          │
│  │  Output: thoughts/shared/prs/                                            │
│  └─────────────┘                                                            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Phase Details

#### 1. Research Phase

**Purpose**: Understand the codebase before making decisions.

**Key Principles**:
- "YOUR ONLY JOB IS TO DOCUMENT AND EXPLAIN THE CODEBASE AS IT EXISTS TODAY"
- Don't suggest improvements unless asked
- Don't perform root cause analysis unless asked
- Focus on documenting what IS, not what SHOULD BE

**Process**:
```bash
# 1. Create research document
snps thoughts new research "Feature X Requirements"

# 2. Search for existing knowledge
snps thoughts search "feature X" --paths-only

# 3. Investigate codebase
# (AI uses grep, glob, read tools)

# 4. Document findings with file:line references
# Update thoughts/shared/research/feature-x-requirements.md

# 5. Sync when complete
snps thoughts sync -m "Completed Feature X research"
```

**Research Document Structure**:
```markdown
# Research: Feature X Requirements

## Date
2024-01-15

## Researcher
claude

## Question
How does the authentication system work?

## Key Findings

### Finding 1: JWT Token Flow
The system uses JWT tokens stored in...
- `src/auth/jwt.rs:45` - Token generation
- `src/middleware/auth.rs:23` - Token validation

### Finding 2: Session Management
Sessions are managed via Redis...

## Relevant Files
- `src/auth/jwt.rs` - JWT implementation
- `src/auth/session.rs` - Session handling
- `src/config/auth.yaml` - Configuration

## Open Questions
- [ ] How are refresh tokens handled?
- [ ] What's the token expiration strategy?

## Status
✅ Complete
```

#### 2. Planning Phase

**Purpose**: Translate research into precise implementation steps.

**Key Principles**:
- Plans should specify exact files and changes
- Break work into verifiable phases
- Define success criteria (automated + manual)
- Get human approval before implementation

**Process**:
```bash
# 1. Create plan (ideally after research)
snps thoughts new plan "Implement OAuth2"

# 2. Reference research
# Link to thoughts/shared/research/oauth-analysis.md

# 3. Define phases with checkboxes
# Each phase should be independently verifiable

# 4. Present for human review
# (High-leverage review point!)

# 5. Sync approved plan
snps thoughts sync -m "OAuth2 plan approved"
```

**Plan Document Structure**:
```markdown
# Plan: Implement OAuth2

## Goal
Add OAuth2 authentication alongside existing JWT auth.

## Research Reference
- [[research/oauth-analysis.md]]

## Success Criteria

### Automated Verification
- [ ] `make test` passes
- [ ] `make lint` passes
- [ ] OAuth flow integration test passes

### Manual Verification
- [ ] Can login via Google OAuth
- [ ] Existing JWT auth still works
- [ ] Session persists across page refresh

## Out of Scope
- Social login UI redesign
- Multiple OAuth providers (future work)

## Implementation Phases

### Phase 1: Database Schema
- [ ] Add `oauth_accounts` table
  - Files: `migrations/20240115_oauth.sql`
  - Changes: New table with provider, provider_id, user_id

- [ ] Update User model
  - Files: `src/models/user.rs`
  - Changes: Add oauth_accounts relation

### Phase 2: OAuth Flow
- [ ] Implement OAuth client
  - Files: `src/auth/oauth.rs` (new)
  - Changes: Google OAuth client with token exchange

- [ ] Add OAuth routes
  - Files: `src/routes/auth.rs`
  - Changes: /auth/google, /auth/google/callback

### Phase 3: Integration
- [ ] Connect OAuth to existing session
  - Files: `src/auth/session.rs`
  - Changes: Create session from OAuth token

## Rollback Plan
1. Revert migration: `make db:rollback`
2. Remove OAuth routes from router
3. Delete `src/auth/oauth.rs`

## Status
🟡 Approved - Ready for Implementation
```

#### 3. Implementation Phase

**Purpose**: Execute the plan methodically.

**Key Principles**:
- Follow the plan's intent, adapt to reality
- Complete each phase before moving to next
- Update checkboxes as you progress
- Verify at phase boundaries

**Process**:
```bash
# 1. Read plan completely
snps thoughts search "OAuth2" --doc-type plan

# 2. Implement Phase 1
# ... coding ...

# 3. Update plan checkboxes
# - [x] Add `oauth_accounts` table

# 4. Run verification
make test

# 5. Inform human: "Phase 1 complete, ready for review"

# 6. Continue to Phase 2 (or wait for approval)

# 7. Sync progress
snps thoughts sync -m "OAuth2 Phase 1 complete"
```

#### 4. Validation Phase

**Purpose**: Confirm implementation works and document for review.

**Process**:
```bash
# 1. Run all automated checks
make check test

# 2. Perform manual verification steps

# 3. Create PR description
snps thoughts new pr "Add OAuth2 Authentication"

# 4. Link to research and plan
# Reference: thoughts/shared/plans/oauth2-implementation.md

# 5. Document any deviations from plan

# 6. Sync everything
snps thoughts sync -m "OAuth2 implementation complete"
```

---

## How Thoughts Interact with PRDs and Specs

### The Document Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          DOCUMENT HIERARCHY                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  STRATEGIC (What to build)                                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  PRD (Product Requirements Document)                                 │   │
│  │  • Business goals                                                    │   │
│  │  • User stories                                                      │   │
│  │  • Success metrics                                                   │   │
│  │  • High-level requirements                                           │   │
│  │                                                                       │   │
│  │  Location: Project docs, wiki, or spec system                        │   │
│  └──────────────────────────────────┬──────────────────────────────────┘   │
│                                     │                                        │
│                                     │ informs                                │
│                                     ▼                                        │
│  TACTICAL (How to understand)                                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  RESEARCH (thoughts/shared/research/)                                │   │
│  │  • Codebase analysis                                                 │   │
│  │  • Technical feasibility                                             │   │
│  │  • Existing patterns                                                 │   │
│  │  • Implementation options                                            │   │
│  │                                                                       │   │
│  │  Answers: "How does the current system work?"                        │   │
│  │           "What approaches are possible?"                            │   │
│  └──────────────────────────────────┬──────────────────────────────────┘   │
│                                     │                                        │
│                                     │ enables                                │
│                                     ▼                                        │
│  OPERATIONAL (How to build)                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  PLAN (thoughts/shared/plans/)                                       │   │
│  │  • Exact file changes                                                │   │
│  │  • Phased approach                                                   │   │
│  │  • Success criteria                                                  │   │
│  │  • Verification steps                                                │   │
│  │                                                                       │   │
│  │  Answers: "What specific changes will we make?"                      │   │
│  └──────────────────────────────────┬──────────────────────────────────┘   │
│                                     │                                        │
│                                     │ guides                                 │
│                                     ▼                                        │
│  EXECUTION (Building it)                                                    │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  CODE + TICKET NOTES (thoughts/shared/tickets/)                      │   │
│  │  • Implementation decisions                                          │   │
│  │  • Blockers encountered                                              │   │
│  │  • Questions for stakeholders                                        │   │
│  │  • Progress tracking                                                 │   │
│  └──────────────────────────────────┬──────────────────────────────────┘   │
│                                     │                                        │
│                                     │ produces                               │
│                                     ▼                                        │
│  DELIVERY (Shipping it)                                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  PR DESCRIPTION (thoughts/shared/prs/)                               │   │
│  │  • Summary of changes                                                │   │
│  │  • Testing performed                                                 │   │
│  │  • Links to plan/research                                            │   │
│  │  • Screenshots if applicable                                         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Linking Documents

**PRD → Research**:
```markdown
# Research: Payment System Analysis

## PRD Reference
This research supports PRD-2024-015: "Subscription Billing System"
Link: https://wiki.company.com/prd/2024-015

## Questions from PRD
The PRD asks for "flexible billing cycles". This research explores:
- How current billing works
- Options for implementing flexibility
```

**Research → Plan**:
```markdown
# Plan: Implement Flexible Billing

## Research Reference
Based on: [[research/payment-system-analysis.md]]

Key findings that inform this plan:
- Current system uses monthly cycles only (research finding #2)
- Stripe supports custom billing intervals (research finding #4)
```

**Plan → Ticket**:
```markdown
# Ticket: BILL-456 - Add Weekly Billing Option

## Plan Reference
Implementation plan: [[plans/flexible-billing.md]]
This ticket covers Phase 2 of the plan.

## Scope for This Ticket
- Weekly billing option only
- Monthly billing changes in BILL-457
```

**Ticket → PR**:
```markdown
# PR: Add Weekly Billing Support

## Related Documents
- Ticket: BILL-456
- Plan: [[plans/flexible-billing.md]] (Phase 2)
- Research: [[research/payment-system-analysis.md]]
```

---

## AI Agent Integration

### For AI Coding Assistants

When working as an AI agent, follow this workflow:

#### Before Starting Any Non-Trivial Task

```bash
# 1. Search existing thoughts
snps thoughts search "<topic>" --paths-only

# 2. Read relevant research/plans
# (Use the paths returned above)

# 3. Check for existing ticket context
snps thoughts search "<ticket-id>" --paths-only
```

#### During Research Phase

```bash
# Create research if none exists
snps thoughts new research "Topic Analysis"

# Document findings with file:line references
# Focus on WHAT IS, not what SHOULD BE
# Don't suggest improvements unless asked

# Sync when complete
snps thoughts sync -m "Completed research on <topic>"
```

#### During Planning Phase

```bash
# Reference research in plan
snps thoughts new plan "Feature Implementation"

# Define phases with checkboxes
# Each phase independently verifiable
# Specify exact file changes

# Sync approved plan
snps thoughts sync -m "Plan approved for <feature>"
```

#### During Implementation Phase

```bash
# Follow plan phases sequentially
# Update checkboxes as you complete items
# Verify at phase boundaries

# If you deviate from plan, document why
# Sync progress regularly
snps thoughts sync -m "Phase 1 complete"
```

### Token-Efficient Search Pattern

AI agents should use `--paths-only` to find documents without consuming context:

```bash
# Step 1: Find relevant files (low tokens)
snps thoughts search "authentication" --paths-only
# Output:
# thoughts/shared/research/auth-patterns.md
# thoughts/shared/plans/oauth2-implementation.md

# Step 2: Read only what's needed (targeted tokens)
# Read specific files based on relevance
```

### Subagent Pattern

For complex research, spawn subagents to search without cluttering main context:

```
MAIN AGENT (implementation focused)
     │
     ├──► SUBAGENT: Search thoughts for "auth"
     │         └──► Returns: 3 relevant paths + summaries
     │
     ├──► SUBAGENT: Analyze src/auth/*.rs
     │         └──► Returns: Key functions, data flow
     │
     └──► MAIN AGENT: Continues with clean context
```

---

## Step-by-Step Examples

### Example 1: New Feature Implementation

**Scenario**: Add dark mode to the application.

```bash
# Day 1: Research
snps thoughts new research "Dark Mode Implementation Options"

# Research the codebase...
# - Find existing theme system
# - Check CSS architecture
# - Look for similar features

snps thoughts sync -m "Dark mode research complete"

# Day 1-2: Planning (after research review)
snps thoughts new plan "Implement Dark Mode"

# Write phases:
# Phase 1: Add theme context/state
# Phase 2: Create dark theme CSS variables
# Phase 3: Add toggle UI
# Phase 4: Persist preference

snps thoughts sync -m "Dark mode plan ready for review"

# Day 2-3: Implementation (after plan approval)
# Follow plan phases...
# Update checkboxes as you go...

snps thoughts sync -m "Dark mode phase 1 complete"

# Day 3: Completion
snps thoughts new pr "Add Dark Mode Support"
snps thoughts sync -m "Dark mode implementation complete"
```

### Example 2: Bug Fix with Unknown Cause

**Scenario**: Users report intermittent login failures.

```bash
# Step 1: Create ticket context
snps thoughts new ticket "BUG-789"

# Document original report
# Note reproduction steps
# List initial hypotheses

# Step 2: Research (because cause unknown)
snps thoughts new research "Login Failure Investigation"

# Trace the auth flow
# Document with file:line references
# Identify potential causes

snps thoughts sync -m "Login failure research - found likely cause"

# Step 3: Plan the fix
snps thoughts new plan "Fix Race Condition in Auth"

# Document the fix approach
# Define verification steps

snps thoughts sync -m "Auth fix plan ready"

# Step 4: Implement and verify
# Follow plan...

snps thoughts sync -m "Auth fix complete and verified"
```

### Example 3: Cross-Team Feature

**Scenario**: Feature needs coordination between frontend and backend teams.

```bash
# Shared research (both teams reference)
snps thoughts new research "API Contract for Feature X" --scope shared

# Team-specific plans
snps thoughts new plan "Feature X - Backend API" --scope shared
snps thoughts new plan "Feature X - Frontend Integration" --scope shared

# Personal notes for each team member
snps thoughts new scratch "My notes on Feature X API" --scope personal

# Sync shared work
snps thoughts sync -m "Feature X coordination docs" --push
```

---

## Best Practices

### Do's

1. **Research before planning, plan before implementing**
2. **Document with file:line references** - Makes future searches effective
3. **Update checkboxes in real-time** - Shows progress, enables resumption
4. **Sync frequently** - Preserve work across sessions
5. **Link documents** - Research → Plan → Ticket → PR
6. **Use `--paths-only` for AI search** - Token efficient
7. **Review research/plans carefully** - High-leverage review points

### Don'ts

1. **Don't skip research for complex tasks** - Understanding prevents mistakes
2. **Don't implement without a plan for multi-file changes** - Plans catch issues early
3. **Don't commit thoughts to code repo** - They're symlinked for a reason
4. **Don't suggest improvements in research** - Document what IS, not what SHOULD BE
5. **Don't create thoughts for trivial tasks** - < 30 min tasks don't need them
6. **Don't forget to sync** - Unsaved thoughts are lost

### Thought Type Decision Tree

```
START: New task arrives
  │
  ├─► Is it < 30 minutes?
  │     ├─► Yes → Just do it (no thoughts needed)
  │     └─► No → Continue ▼
  │
  ├─► Do you understand the codebase area?
  │     ├─► No → Create RESEARCH
  │     └─► Yes → Continue ▼
  │
  ├─► Does it touch multiple files?
  │     ├─► Yes → Create PLAN (after research if needed)
  │     └─► No → Continue ▼
  │
  ├─► Is it a tracked ticket?
  │     ├─► Yes → Create TICKET context
  │     └─► No → Continue ▼
  │
  └─► Will it need a PR?
        ├─► Yes → Create PR description after implementation
        └─► No → Done!
```

---

## Summary

The thoughts system transforms ephemeral conversation into persistent knowledge:

| Phase | Thought Type | Purpose |
|-------|-------------|---------|
| **Understand** | Research | Document how things work |
| **Design** | Plan | Specify what to change |
| **Track** | Ticket | Context for tracked work |
| **Ship** | PR | Explain changes for review |
| **Remember** | All | Persist knowledge across sessions |

**The core insight**: Invest time upfront in research and planning. Errors at these stages multiply into the implementation. Human review is most valuable at research and plan phases, not code review.

---

## References

- [HumanLayer Repository](https://github.com/humanlayer/humanlayer)
- [Advanced Context Engineering](https://github.com/humanlayer/advanced-context-engineering-for-coding-agents)
- [Catalyst Project](https://github.com/coalesce-labs/catalyst)
- [thoughts-locator Agent](https://github.com/humanlayer/humanlayer/blob/main/.claude/agents/thoughts-locator.md)
- [create_plan Command](https://github.com/humanlayer/humanlayer/blob/main/.claude/commands/create_plan.md)
- [implement_plan Command](https://github.com/humanlayer/humanlayer/blob/main/.claude/commands/implement_plan.md)
- [research_codebase Command](https://github.com/humanlayer/humanlayer/blob/main/.claude/commands/research_codebase.md)

---

*Part of PMSynapse AI-Enabled Project Management*
