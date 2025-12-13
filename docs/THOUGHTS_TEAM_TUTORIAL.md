# Team Thoughts Tutorial: A Story-Driven Guide

*A practical tutorial that follows Team Nexus through their first week using the thoughts system.*

---

## Meet Team Nexus

**Team Nexus** is a 4-person engineering team at a startup building a project management tool:

- **Sarah** - Tech Lead, 8 years experience
- **Marcus** - Senior Backend Developer
- **Priya** - Frontend Developer
- **Alex** - Junior Developer, first job out of bootcamp

They've just adopted PMSynapse's thoughts system. This tutorial follows their journey.

---

## Day 1: Getting Started

### Story: Sarah Sets Up the Team

Sarah just finished reading about the thoughts system. She gathers the team for a 15-minute standup.

> **Sarah**: "We're going to start using a thoughts system to capture our research and plans. Think of it as a shared brain for the team. Let me show you how it works."

She opens her terminal in the project directory:

```bash
# Initialize thoughts for the project
snps thoughts init

# Output:
# ✓ Created thoughts symlink
# ✓ Created shared/ directory
# ✓ Created sarah/ personal directory
# ✓ Created global/ symlink
# ✓ Installed pre-commit hook
```

> **Alex**: "Wait, what just happened?"

> **Sarah**: "It created a `thoughts/` folder in our project, but it's actually a symlink to a central location. This means our thoughts persist across all our projects. Let me show you the structure."

```bash
ls -la thoughts/

# Output:
# shared/       <- Team knowledge (everyone reads/writes)
# sarah/        <- My personal notes
# global/       <- Cross-project patterns
# searchable/   <- AI search index (auto-generated)
```

### Exercise 1: Each Team Member Initializes

Each team member runs the same command in their terminal:

```bash
snps thoughts init
```

This creates their personal directory (`marcus/`, `priya/`, `alex/`).

### Story: Alex Asks About the Directories

> **Alex**: "What goes in each folder?"

> **Sarah**: "Great question. Here's the simple version:

| Directory | What Goes There | Who Writes | Who Reads |
|-----------|----------------|------------|-----------|
| `shared/` | Research, plans, ticket notes | Everyone | Everyone |
| `{username}/` | Personal notes, scratch work | Just you | Just you |
| `global/` | Patterns that apply across projects | Anyone | Everyone |
| `searchable/` | Don't touch - auto-generated | System | AI tools |

Let's create our first shared thought."

---

## Day 1: First Research Document

### Story: Marcus Gets Assigned a Complex Ticket

The team just got a ticket: **ENG-234: Add webhook support for task completion**.

Marcus looks at it and sighs.

> **Marcus**: "I have no idea where our notification system lives or how it works."

> **Sarah**: "Perfect use case for a research document. Before you write any code, document what you learn. Future Marcus will thank you."

### Exercise 2: Create a Research Document

Marcus creates his first research document:

```bash
snps thoughts new research "Notification System Analysis"

# Output:
# Created: thoughts/shared/research/2025-12-13-notification-system-analysis.md
# Opening in editor...
```

The template appears:

```markdown
---
date: 2025-12-13
author: marcus
type: research
status: in_progress
tags: []
---

# Research: Notification System Analysis

## Question
[What are you trying to understand?]

## Key Findings

### Finding 1: [Title]
[Details with file:line references]

## Relevant Files
- `path/to/file.rs` - [What it does]

## Open Questions
- [ ] [Question that needs answering]

## Status
- [ ] Complete
```

### Story: Marcus Investigates

Marcus spends an hour tracing the code. He updates his research document:

```markdown
---
date: 2025-12-13
author: marcus
type: research
status: in_progress
tags: [notifications, webhooks, eng-234]
---

# Research: Notification System Analysis

## Question
How do notifications currently work, and where would webhook support fit?

## Key Findings

### Finding 1: Event System Location
The notification system lives in `src/events/`:
- `src/events/mod.rs:15` - Event trait definition
- `src/events/handlers.rs:45` - Email notification handler
- `src/events/handlers.rs:89` - Slack notification handler

### Finding 2: Event Flow
```
Task Completed
     │
     ▼
TaskCompletedEvent (src/events/types.rs:23)
     │
     ▼
EventDispatcher (src/events/dispatcher.rs:67)
     │
     ├──► EmailHandler
     └──► SlackHandler (if configured)
```

### Finding 3: Handler Interface
All handlers implement the `NotificationHandler` trait:
```rust
// src/events/mod.rs:15-22
pub trait NotificationHandler: Send + Sync {
    fn can_handle(&self, event: &dyn Event) -> bool;
    fn handle(&self, event: &dyn Event) -> Result<(), Error>;
}
```

A webhook handler would fit right in here.

## Relevant Files
- `src/events/mod.rs` - Core traits and types
- `src/events/dispatcher.rs` - Routes events to handlers
- `src/events/handlers.rs` - Existing notification handlers
- `src/config/notifications.rs` - Configuration loading

## Open Questions
- [x] Where do new handlers get registered? → `src/events/dispatcher.rs:34`
- [ ] Do we need rate limiting for webhooks?
- [ ] Should failed webhooks retry?

## Status
- [x] Complete - ready for planning
```

### Exercise 3: Sync Your Work

Marcus saves his work to the central repository:

```bash
snps thoughts sync -m "ENG-234: Completed notification system research"

# Output:
# Syncing thoughts...
#   ✓ Rebuilt searchable index (5 files)
#   ✓ Committed: ENG-234: Completed notification system research
# ✅ Sync complete
```

---

## Day 2: From Research to Plan

### Story: Sarah Reviews Marcus's Research

The next morning, Sarah reviews Marcus's research.

> **Sarah**: "This is great! You've mapped out exactly where webhooks fit. Let's turn this into a plan before you write any code."

> **Marcus**: "Why can't I just start coding? I know what to do now."

> **Sarah**: "Research tells us WHERE to make changes. A plan tells us WHAT changes to make and in WHAT ORDER. It also lets me catch design issues before you've written 500 lines."

### Exercise 4: Create a Plan Document

Marcus creates a plan based on his research:

```bash
snps thoughts new plan "ENG-234 Webhook Implementation"

# Output:
# Created: thoughts/shared/plans/2025-12-13-eng-234-webhook-implementation.md
```

He writes:

```markdown
---
date: 2025-12-13
author: marcus
type: plan
status: pending_approval
ticket: ENG-234
tags: [webhooks, notifications]
---

# Plan: ENG-234 Webhook Implementation

## Overview
Add webhook support to the notification system, allowing users to configure HTTP callbacks for task completion events.

## Research Reference
Based on: [[research/2025-12-13-notification-system-analysis.md]]

## Desired End State
- Users can configure webhook URLs in settings
- Task completion triggers HTTP POST to configured webhooks
- Failed webhooks are logged (retry in future iteration)

## What We're NOT Doing
- Retry logic for failed webhooks (future ticket)
- UI for managing webhooks (API only for now)
- Webhook signatures/authentication (future ticket)

## Implementation Phases

### Phase 1: Data Model
**Goal**: Store webhook configurations

**Changes**:
1. **Database Migration**
   - File: `migrations/20251213_add_webhooks.sql`
   - Add `webhooks` table with columns: id, user_id, url, events[], active

2. **Model**
   - File: `src/models/webhook.rs` (new)
   - Webhook struct, CRUD operations

**Success Criteria**:
- [ ] `cargo test -p models` passes
- [ ] Migration applies cleanly

---

### Phase 2: Webhook Handler
**Goal**: Implement the notification handler

**Changes**:
1. **Handler Implementation**
   - File: `src/events/handlers.rs`
   - Add `WebhookHandler` implementing `NotificationHandler` trait
   - Use reqwest for HTTP calls

2. **Register Handler**
   - File: `src/events/dispatcher.rs:34`
   - Register WebhookHandler in dispatcher

**Success Criteria**:
- [ ] Handler unit tests pass
- [ ] Integration test: webhook receives POST on task complete

---

### Phase 3: API Endpoints
**Goal**: Allow webhook CRUD via API

**Changes**:
1. **Routes**
   - File: `src/routes/webhooks.rs` (new)
   - GET /webhooks, POST /webhooks, DELETE /webhooks/:id

2. **Register Routes**
   - File: `src/routes/mod.rs`
   - Add webhook routes to router

**Success Criteria**:
- [ ] API tests pass
- [ ] Can create/list/delete webhooks via curl

---

## Manual Testing Checklist
- [ ] Create a webhook via API
- [ ] Complete a task
- [ ] Verify webhook receives POST with correct payload
- [ ] Delete webhook, verify no more POSTs

## Status
🟡 Pending Approval
```

### Story: The Plan Review

Sarah reviews the plan during their 1:1.

> **Sarah**: "I like the phased approach. One question - what's the webhook payload format?"

> **Marcus**: "Good catch. I hadn't specified that."

He adds to Phase 2:

```markdown
**Webhook Payload**:
```json
{
  "event": "task.completed",
  "timestamp": "2025-12-13T10:30:00Z",
  "data": {
    "task_id": "uuid",
    "task_title": "string",
    "completed_by": "user_id"
  }
}
```
```

> **Sarah**: "Perfect. Plan approved. Update the status and sync."

```bash
# Marcus updates status to: 🟢 Approved

snps thoughts sync -m "ENG-234: Plan approved, ready for implementation"
```

---

## Day 3: Implementation with Checkboxes

### Story: Marcus Implements Phase 1

Marcus starts implementing. As he completes items, he checks them off:

```bash
# Marcus edits the plan file directly
# Changes: - [ ] to - [x]

snps thoughts sync -m "ENG-234: Phase 1 complete"
```

### Exercise 5: Track Progress in Your Plan

The plan now shows:

```markdown
### Phase 1: Data Model
**Goal**: Store webhook configurations

**Changes**:
1. **Database Migration**
   - File: `migrations/20251213_add_webhooks.sql`
   - Add `webhooks` table with columns: id, user_id, url, events[], active

2. **Model**
   - File: `src/models/webhook.rs` (new)
   - Webhook struct, CRUD operations

**Success Criteria**:
- [x] `cargo test -p models` passes
- [x] Migration applies cleanly

✅ Phase 1 Complete
```

### Story: Priya Needs to Find Marcus's Work

Meanwhile, Priya is working on a frontend feature that will eventually display webhooks. She needs to understand the data model.

> **Priya**: "Marcus, can you explain the webhook model?"

> **Marcus**: "Just check my research and plan."

```bash
# Priya searches the thoughts
snps thoughts search "webhook"

# Output:
# thoughts/shared/research/2025-12-13-notification-system-analysis.md
# thoughts/shared/plans/2025-12-13-eng-234-webhook-implementation.md
```

> **Priya**: "Oh nice, I can see exactly what fields you're using. I'll build the UI to match."

---

## Day 4: Personal Notes and Team Sync

### Story: Alex's Learning Journey

Alex is assigned their first real ticket: **ENG-240: Fix date formatting bug**.

They feel overwhelmed but remember Sarah's advice: "Document as you learn."

### Exercise 6: Use Personal Notes

Alex creates a personal scratch note:

```bash
snps thoughts new scratch "Learning date handling" --scope personal

# Output:
# Created: thoughts/alex/scratch-2025-12-13-learning-date-handling.md
```

They write notes as they learn:

```markdown
# Learning: Date Handling in Our Codebase

## Things I'm Learning

### Timezone Handling
- We store everything in UTC (found in `src/utils/time.rs:15`)
- Convert to user timezone on display (frontend handles this)

### Date Libraries
- Backend uses `chrono` crate
- Frontend uses `date-fns`

## Questions for Sarah
- Should I use `chrono::DateTime<Utc>` or `chrono::NaiveDateTime`?
- What's our convention for date serialization in APIs?

## Mistakes I Made
- Tried to format dates in Rust for the API response - wrong!
- The frontend expects ISO8601 strings, we format there
```

> **Sarah** (later): "These personal notes are perfect. You're building your own knowledge base. When you learn the answers, update the doc."

### Story: Team Sync Meeting

Friday afternoon, Sarah runs a quick sync:

> **Sarah**: "Let's see what everyone documented this week."

```bash
snps thoughts list --recent 10

# Output:
# SHARED
# 📄 research/2025-12-13-notification-system-analysis.md (marcus)
# 📄 plans/2025-12-13-eng-234-webhook-implementation.md (marcus)
#
# PERSONAL
# 📄 alex/scratch-2025-12-13-learning-date-handling.md
# 📄 priya/scratch-webhook-ui-ideas.md
```

> **Sarah**: "Great coverage. Marcus, your research will help anyone who touches notifications in the future. Alex, promote your date handling learnings to shared when you're confident - others will benefit."

### Exercise 7: Promote Personal to Shared

When Alex's notes are solid, they copy to shared:

```bash
# Copy relevant parts to shared research
snps thoughts new research "Date Handling Conventions" --scope shared

# Update with curated content (not all personal notes, just the useful parts)

snps thoughts sync -m "Added date handling conventions to shared knowledge"
```

---

## Day 5: Cross-Session Continuity

### Story: Marcus Gets Sick

Marcus is out sick on Monday. The webhook feature is 80% done but the team needs to ship it.

> **Sarah**: "No problem. Priya, can you finish the webhook API tests?"

> **Priya**: "I've never touched that code..."

> **Sarah**: "Check Marcus's thoughts."

### Exercise 8: Resume Someone Else's Work

Priya finds exactly what she needs:

```bash
# Find Marcus's plan
snps thoughts search "webhook" --doc-type plan

# Read the plan
cat thoughts/shared/plans/2025-12-13-eng-234-webhook-implementation.md
```

The plan shows:
- Phase 1: ✅ Complete
- Phase 2: ✅ Complete
- Phase 3: 🟡 In Progress (API tests remaining)

Priya can see exactly where Marcus left off and what's needed.

```markdown
### Phase 3: API Endpoints
**Goal**: Allow webhook CRUD via API

**Changes**:
1. **Routes**
   - File: `src/routes/webhooks.rs` (new)
   - [x] GET /webhooks
   - [x] POST /webhooks
   - [x] DELETE /webhooks/:id

2. **Register Routes**
   - File: `src/routes/mod.rs`
   - [x] Add webhook routes to router

**Success Criteria**:
- [ ] API tests pass     <-- THIS IS WHAT'S LEFT
- [x] Can create/list/delete webhooks via curl
```

> **Priya**: "Got it. I just need to write the API tests. The code is done."

She completes the feature and updates the plan:

```bash
snps thoughts sync -m "ENG-234: Completed API tests, feature ready for review"
```

---

## Week 2: Advanced Patterns

### Story: Sarah Creates Team Patterns

After a week, Sarah notices the team is solving the same problems repeatedly. She creates global patterns:

```bash
snps thoughts new research "Error Handling Pattern" --scope global
```

```markdown
# Pattern: Error Handling in API Routes

## When to Use
Any new API endpoint in `src/routes/`

## The Pattern
```rust
// Always use Result with our custom AppError
pub async fn handler() -> Result<Json<Response>, AppError> {
    let data = service.get_data()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(Response { data }))
}
```

## Examples in Codebase
- `src/routes/tasks.rs:45` - Good example
- `src/routes/users.rs:23` - Good example

## Anti-patterns to Avoid
- Don't use `unwrap()` in handlers
- Don't return raw strings as errors
```

Now every team member (and AI assistant) can find this pattern:

```bash
snps thoughts search "error handling" --scope global
```

---

## The Team's Thought System Workflow

After two weeks, Team Nexus has established their workflow:

```
┌─────────────────────────────────────────────────────────────┐
│                    TEAM NEXUS WORKFLOW                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. TICKET ARRIVES                                           │
│     │                                                        │
│     ├──► Simple (< 2 hours)?                                │
│     │    └──► Just do it                                    │
│     │                                                        │
│     └──► Complex?                                           │
│          └──► Create RESEARCH document                      │
│                                                              │
│  2. RESEARCH PHASE                                           │
│     │                                                        │
│     ├──► Explore codebase                                   │
│     ├──► Document findings with file:line refs              │
│     ├──► Get review from lead                               │
│     └──► snps thoughts sync                                 │
│                                                              │
│  3. PLANNING PHASE                                           │
│     │                                                        │
│     ├──► Create PLAN from research                          │
│     ├──► Define phases with checkboxes                      │
│     ├──► Get approval from lead                             │
│     └──► snps thoughts sync                                 │
│                                                              │
│  4. IMPLEMENTATION PHASE                                     │
│     │                                                        │
│     ├──► Follow plan phases                                 │
│     ├──► Check off items as completed                       │
│     ├──► Sync after each phase                              │
│     └──► Ask for review at phase boundaries                 │
│                                                              │
│  5. COMPLETION                                               │
│     │                                                        │
│     ├──► Create PR with link to plan                        │
│     ├──► Update plan status to complete                     │
│     └──► Promote learnings to global patterns               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Quick Reference Card

### Commands You'll Use Daily

```bash
# Start new work
snps thoughts new research "Topic"     # Understand something
snps thoughts new plan "Feature"       # Plan implementation
snps thoughts new ticket "ENG-123"     # Track ticket context

# Find existing knowledge
snps thoughts search "keyword"         # Search all thoughts
snps thoughts list --recent 5          # See recent activity

# Save your work
snps thoughts sync -m "message"        # Commit and index

# Check system health
snps thoughts status                   # See configuration
```

### When to Create Each Type

| Situation | Thought Type |
|-----------|-------------|
| "I don't understand how X works" | Research |
| "I need to implement feature Y" | Plan (after research) |
| "I'm working on ticket ENG-123" | Ticket |
| "Random idea I might forget" | Scratch (personal) |
| "Pattern others should follow" | Research (global) |

### The Golden Rule

> **Research errors multiply into thousands of lines of code.**
> **Planning errors multiply into hundreds of lines.**
> **Implementation errors are isolated.**
>
> Invest your review time where errors have the highest impact.

---

## Your First Week Checklist

- [ ] Day 1: Run `snps thoughts init` in your project
- [ ] Day 1: Create your first research document
- [ ] Day 2: Search for existing thoughts before starting work
- [ ] Day 3: Create a plan for a multi-file change
- [ ] Day 4: Use checkboxes to track progress
- [ ] Day 5: Review the week's thoughts with your team

---

## Troubleshooting

### "I can't find the thoughts directory"
```bash
snps thoughts init --force
```

### "My changes aren't showing up for teammates"
```bash
snps thoughts sync --push
```

### "AI can't find my documents"
```bash
snps thoughts sync  # Rebuilds the searchable/ index
```

### "I committed thoughts to the code repo"
The pre-commit hook should prevent this. If it didn't:
```bash
git reset HEAD thoughts/
snps thoughts hooks install
```

---

## Summary

The thoughts system is a team knowledge base that:

1. **Persists across sessions** - Your research survives when you close the terminal
2. **Shares knowledge** - Team members find each other's work
3. **Guides AI assistants** - They search thoughts before asking you
4. **Tracks progress** - Checkboxes show where you are
5. **Enables handoffs** - Someone can pick up where you left off

Start small: one research document, one plan. Build the habit. Your future self and teammates will thank you.

---

*Part of PMSynapse AI-Enabled Project Management*
