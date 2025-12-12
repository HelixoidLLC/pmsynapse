# PMSynapse Project Workflow

## Overview

This document defines the standard workflow for managing projects in PMSynapse. The workflow is designed to guide issues from initial triage through research, planning, development, and completion.

> **Integration**: This workflow can be managed via [Linear MCP](https://linear.app/docs) for seamless AI-assisted issue tracking.

---

## Workflow Stages

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PMSYNAPSE PROJECT WORKFLOW                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────┐    ┌─────────┐    ┌───────────┐    ┌─────────┐    ┌─────────┐ │
│  │ TRIAGE  │───►│ BACKLOG │───►│ UNSTARTED │───►│ STARTED │───►│COMPLETED│ │
│  └─────────┘    └─────────┘    └───────────┘    └─────────┘    └─────────┘ │
│       │                                                              │       │
│       └──────────────────────► CANCELED ◄────────────────────────────┘       │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Stage Details

### 1. Triage

Initial intake for all new issues, ideas, and requests.

| Status | Description |
|--------|-------------|
| `triage` | New item awaiting initial review and categorization |

**Entry criteria**: Any new issue, bug report, feature request, or idea
**Exit criteria**: Item is categorized, prioritized, and moved to Backlog or Canceled

**Actions in this stage**:
- Assign initial labels (bug, feature, enhancement, etc.)
- Assess urgency and impact
- Identify affected components
- Link to related issues

---

### 2. Backlog

Prioritized queue of work items accepted for future work.

| Status | Description |
|--------|-------------|
| `backlog` | Accepted item waiting to be scheduled |
| `postits` | Quick notes, ideas, or reminders for future consideration |

**Entry criteria**: Item triaged and accepted
**Exit criteria**: Item scheduled for a sprint/cycle and moved to Unstarted

**Actions in this stage**:
- Refine priority based on roadmap
- Group related items
- Estimate rough effort
- Assign to milestones/projects

---

### 3. Unstarted

Work items that are scheduled but not yet actively being worked on.

| Status | Description | Graph Node Type |
|--------|-------------|-----------------|
| `todo` | Ready to be picked up | `Task` |
| `spec-needed` | Requires specification/PRD before work | `Question` |
| `research-needed` | Requires investigation before planning | `Question` |
| `research-in-progress` | Active research/investigation | `Task` |
| `research-in-review` | Research complete, awaiting review | `Proposal` |
| `ready-for-plan` | Research done, ready for planning | `Task` |

**Entry criteria**: Item scheduled for current or upcoming cycle
**Exit criteria**: Planning begins (moves to Started)

**Workflow within Unstarted**:

```
┌─────────────────────────────────────────────────────────────────┐
│                      UNSTARTED FLOW                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐                                                   │
│  │   todo   │ ─────────────────────────────────┐               │
│  └──────────┘                                   │               │
│       │                                         │               │
│       │ (needs clarity?)                        │               │
│       ▼                                         │               │
│  ┌──────────────┐                              │               │
│  │ spec-needed  │                              │               │
│  └──────────────┘                              │               │
│       │                                         │               │
│       │ (needs investigation?)                  │               │
│       ▼                                         │               │
│  ┌─────────────────┐    ┌─────────────────┐    │               │
│  │ research-needed │───►│research-in-progress│  │               │
│  └─────────────────┘    └─────────────────┘    │               │
│                               │                 │               │
│                               ▼                 │               │
│                        ┌──────────────────┐    │               │
│                        │research-in-review│    │               │
│                        └──────────────────┘    │               │
│                               │                 │               │
│                               ▼                 │               │
│                        ┌────────────────┐      │               │
│                        │ ready-for-plan │◄─────┘               │
│                        └────────────────┘                       │
│                               │                                 │
│                               ▼                                 │
│                         TO STARTED                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 4. Started

Active work in progress.

| Status | Description | Graph Node Type |
|--------|-------------|-----------------|
| `plan-in-progress` | Creating implementation plan | `Document` |
| `plan-in-review` | Plan awaiting approval | `Proposal` |
| `ready-for-dev` | Plan approved, ready for coding | `Task` |
| `in-dev` | Active development/coding | `Task` |
| `code-review` | Code complete, awaiting review | `Proposal` |
| `ready-for-deploy` | Approved, awaiting deployment | `Task` |

**Entry criteria**: Planning has begun
**Exit criteria**: Deployed to production (moves to Completed)

**Workflow within Started**:

```
┌─────────────────────────────────────────────────────────────────┐
│                       STARTED FLOW                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  FROM UNSTARTED (ready-for-plan)                                │
│       │                                                          │
│       ▼                                                          │
│  ┌──────────────────┐    ┌─────────────────┐                    │
│  │ plan-in-progress │───►│ plan-in-review  │                    │
│  └──────────────────┘    └─────────────────┘                    │
│                               │                                  │
│                               │ (approved?)                      │
│                               ▼                                  │
│                        ┌───────────────┐                        │
│                        │ ready-for-dev │                        │
│                        └───────────────┘                        │
│                               │                                  │
│                               ▼                                  │
│                        ┌───────────────┐                        │
│                        │    in-dev     │◄─────┐                 │
│                        └───────────────┘      │                 │
│                               │               │ (changes        │
│                               ▼               │  requested)     │
│                        ┌───────────────┐      │                 │
│                        │  code-review  │──────┘                 │
│                        └───────────────┘                        │
│                               │                                  │
│                               │ (approved?)                      │
│                               ▼                                  │
│                        ┌─────────────────┐                      │
│                        │ ready-for-deploy│                      │
│                        └─────────────────┘                      │
│                               │                                  │
│                               ▼                                  │
│                         TO COMPLETED                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 5. Completed

Successfully finished work items.

| Status | Description |
|--------|-------------|
| `done` | Work completed and deployed |

**Entry criteria**: Code deployed and verified
**Exit criteria**: N/A (terminal state)

**Actions in this stage**:
- Update documentation
- Link to release notes
- Archive related research/plans
- Update knowledge graph relationships

---

### 6. Canceled

Work items that will not be completed.

| Status | Description |
|--------|-------------|
| `canceled` | Work stopped, not to be completed |

**Entry criteria**: Decision made to not proceed (from any stage)
**Exit criteria**: N/A (terminal state)

**Reasons for cancellation**:
- Duplicate of existing work
- No longer relevant
- Superseded by other work
- Out of scope
- Blocked indefinitely

---

## Linear MCP Integration

### What is Linear MCP?

[Linear MCP](https://touchlab.co/linear-mcp-for-ai) is a Model Context Protocol server that integrates with Linear's issue tracking system. It enables AI assistants to manage issues through natural language.

### Setup

Add to your MCP configuration:

```json
{
  "mcpServers": {
    "linear": {
      "command": "npx",
      "args": ["-y", "@anthropic/linear-mcp"],
      "env": {
        "LINEAR_API_KEY": "${LINEAR_API_KEY}"
      }
    }
  }
}
```

Or use Linear's official MCP endpoint:
```
https://mcp.linear.app/sse
```

### Available MCP Actions

| Action | Description |
|--------|-------------|
| `create_issue` | Create new issue with title, description, assignee, state, priority |
| `update_issue` | Modify existing issue properties |
| `get_issue` | Retrieve issue details, attachments, comments |
| `list_issues` | List issues with filters (project, assignee, state) |
| `add_comment` | Add comment to an issue |
| `search_issues` | Search issues by query |

### Integration with PMSynapse

```yaml
# .pmsynapse/integrations/linear.yaml

linear:
  enabled: true
  api_key: ${LINEAR_API_KEY}
  team_id: ${LINEAR_TEAM_ID}

  # Map PMSynapse statuses to Linear states
  status_mapping:
    # Triage
    triage: "Triage"

    # Backlog
    backlog: "Backlog"
    postits: "Backlog"

    # Unstarted
    todo: "Todo"
    spec-needed: "Todo"
    research-needed: "Todo"
    research-in-progress: "In Progress"
    research-in-review: "In Review"
    ready-for-plan: "Todo"

    # Started
    plan-in-progress: "In Progress"
    plan-in-review: "In Review"
    ready-for-dev: "Todo"
    in-dev: "In Progress"
    code-review: "In Review"
    ready-for-deploy: "Done"

    # Completed
    done: "Done"

    # Canceled
    canceled: "Canceled"

  # Sync settings
  sync:
    direction: bidirectional  # linear_to_snps | snps_to_linear | bidirectional
    on_create: true
    on_update: true
    on_comment: true

  # Auto-create labels
  labels:
    - name: "snps-managed"
      description: "Managed by PMSynapse"
    - name: "has-research"
      description: "Has associated research in knowledge graph"
    - name: "has-plan"
      description: "Has implementation plan"
```

### CLI Commands

```bash
# Sync with Linear
snps linear sync                    # Sync all issues
snps linear sync --since 7d         # Sync last 7 days
snps linear push <issue-id>         # Push local changes to Linear
snps linear pull <issue-id>         # Pull Linear changes locally

# Create issues via CLI
snps issue create "Fix auth bug" --status triage
snps issue move <id> --status in-dev
snps issue comment <id> "Started investigation"

# Link to knowledge graph
snps issue link <id> --node <node-id>  # Link issue to graph node
snps issue graph <id>                   # Show related graph nodes
```

---

## Workflow Automation Rules

### Auto-Transitions

```yaml
# .pmsynapse/workflow-rules.yaml

rules:
  # Auto-move to research-in-progress when assigned
  - trigger: assigned
    from: research-needed
    to: research-in-progress

  # Auto-move to code-review when PR created
  - trigger: pr_created
    from: in-dev
    to: code-review

  # Auto-move to done when PR merged
  - trigger: pr_merged
    from: ready-for-deploy
    to: done

  # Auto-create graph nodes
  - trigger: status_changed
    to: research-in-progress
    action: create_research_node

  - trigger: status_changed
    to: plan-in-progress
    action: create_plan_node
```

### Agent Proposals for Status Changes

When agents analyze work, they can propose status changes:

```
┌──────────────────────────────────────────────────────────────┐
│ 🤖 workflow-agent • Just now                                 │
│                                                              │
│ "Move SNPS-123 from 'research-in-progress' to               │
│  'research-in-review'"                                       │
│                                                              │
│ Rationale:                                                   │
│  • Research document completed (docs/research/auth-flow.md) │
│  • All questions answered in knowledge graph                │
│  • No blocking unknowns remaining                           │
│                                                              │
│ [✓ Approve] [✏️ Edit] [❌ Reject]                            │
└──────────────────────────────────────────────────────────────┘
```

---

## Knowledge Graph Integration

Each workflow status maps to graph operations:

| Status Transition | Graph Action |
|-------------------|--------------|
| → `research-needed` | Create `Question` node |
| → `research-in-progress` | Create `Research` node, link to `Question` |
| `research-in-review` → `ready-for-plan` | Create `Finding` nodes from research |
| → `plan-in-progress` | Create `Plan` node |
| `plan-in-review` → `ready-for-dev` | Create `Task` nodes from plan |
| → `in-dev` | Link `Code` nodes as they're created |
| → `done` | Create `Completion` node, link all artifacts |
| → `canceled` | Create `Cancellation` node with reason |

### Example Graph for a Feature

```
┌─────────────────────────────────────────────────────────────────┐
│                    FEATURE: Add Dark Mode                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  (Issue:SNPS-42)                                                │
│       │                                                          │
│       ├──raised──►(Question: "How to persist theme?")           │
│       │                │                                         │
│       │                └──answered_by──►(Research: theme-study) │
│       │                                        │                 │
│       │                                        └──found──►       │
│       │                                          (Finding: "Use  │
│       │                                           localStorage") │
│       │                                                          │
│       ├──planned_by──►(Plan: dark-mode-plan)                    │
│       │                    │                                     │
│       │                    ├──defines──►(Task: Create toggle)   │
│       │                    ├──defines──►(Task: Add CSS vars)    │
│       │                    └──defines──►(Task: Persist pref)    │
│       │                                        │                 │
│       │                                        └──produces──►    │
│       │                                          (Code: theme.ts)│
│       │                                                          │
│       └──completed_by──►(Completion: v1.2.0-release)            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Status Reference Card

### Quick Reference

| Stage | Status | Next Status |
|-------|--------|-------------|
| **Triage** | `triage` | → `backlog` or `canceled` |
| **Backlog** | `backlog` | → `todo` |
| | `postits` | → `backlog` or `canceled` |
| **Unstarted** | `todo` | → `spec-needed` or `ready-for-plan` |
| | `spec-needed` | → `research-needed` or `ready-for-plan` |
| | `research-needed` | → `research-in-progress` |
| | `research-in-progress` | → `research-in-review` |
| | `research-in-review` | → `ready-for-plan` or `research-in-progress` |
| | `ready-for-plan` | → `plan-in-progress` |
| **Started** | `plan-in-progress` | → `plan-in-review` |
| | `plan-in-review` | → `ready-for-dev` or `plan-in-progress` |
| | `ready-for-dev` | → `in-dev` |
| | `in-dev` | → `code-review` |
| | `code-review` | → `ready-for-deploy` or `in-dev` |
| | `ready-for-deploy` | → `done` |
| **Completed** | `done` | (terminal) |
| **Canceled** | `canceled` | (terminal) |

### CLI Quick Commands

```bash
# Move through workflow
snps issue triage <id>              # Move to triage
snps issue accept <id>              # Move to backlog
snps issue start-research <id>      # Move to research-in-progress
snps issue ready-plan <id>          # Move to ready-for-plan
snps issue start-dev <id>           # Move to in-dev
snps issue request-review <id>      # Move to code-review
snps issue deploy <id>              # Move to ready-for-deploy
snps issue complete <id>            # Move to done
snps issue cancel <id> -m "reason"  # Move to canceled
```

---

## Sources

- [Linear MCP Integration](https://touchlab.co/linear-mcp-for-ai) - Touchlab
- [Model Context Protocol](https://www.anthropic.com/news/model-context-protocol) - Anthropic
- [Linear MCP Server](https://glama.ai/mcp/servers/@vinayak-mehta/linear-mcp) - Glama

---

*Document version: 1.0*
*Created: December 2025*
*Part of: PMSynapse Workflow Documentation*
