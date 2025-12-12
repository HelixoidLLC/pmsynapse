# HumanLayer Thoughts System Analysis

## Overview

This document analyzes the **HumanLayer thoughts management system** and extracts unique ideas for building a flexible thoughts management system that allows users to store thoughts locally or remotely, separate from their project code.

**Repository**: [humanlayer/humanlayer](https://github.com/humanlayer/humanlayer)
**Key Innovation**: Separate storage + seamless integration for developer thoughts

---

## The Core Problem Thoughts Solves

### Why Separate Thoughts from Code?

| Problem | Description |
|---------|-------------|
| **Accidental Commits** | Private notes, research, tickets get committed to public repos |
| **Context Loss** | AI assistants lose project knowledge between sessions |
| **Team Silos** | Knowledge trapped in individual developer's notes |
| **Search Fragmentation** | Notes scattered across multiple tools/locations |
| **Privacy Concerns** | Some thoughts should never leave local machine |

### The HumanLayer Solution

Keep thoughts in a **separate repository** with **symlinks into projects**, protected by git hooks that prevent accidental commits while enabling AI search.

```
CODE REPO                          THOUGHTS REPO
(public/shared)                    (private/separate)

project/                           ~/thoughts/
├── src/                           ├── shared/
├── tests/                         │   ├── research/
├── thoughts/ ←──── symlink ────── │   ├── plans/
│   ├── shared/ ───────────────────│   ├── tickets/
│   ├── alice/ ────────────────────│   └── prs/
│   ├── global/ ───────────────────├── alice/
│   └── searchable/ (hardlinks)    │   └── tickets/
└── .git/                          ├── global/
    └── hooks/                     └── .git/
        └── pre-commit (blocks)
```

---

## Unique Ideas to Inherit

### 1. Directory Structure Philosophy

**Four-Layer Organization**:

| Directory | Purpose | Visibility |
|-----------|---------|------------|
| `thoughts/shared/` | Team-wide documents | All team members |
| `thoughts/[username]/` | Personal notes | Individual only |
| `thoughts/global/` | Cross-repository patterns | All projects |
| `thoughts/searchable/` | AI-accessible mirror | Read-only, all content |

**Key Insight**: Separate ownership (personal vs shared) while maintaining unified search.

```
thoughts/
├── shared/                    # Team knowledge
│   ├── research/              # Technical research
│   │   ├── auth-patterns.md
│   │   └── api-design.md
│   ├── plans/                 # Implementation plans
│   │   └── q1-roadmap.md
│   ├── tickets/               # Ticket context
│   │   └── PROJ-123.md
│   └── prs/                   # PR descriptions & context
│       └── pr-456.md
│
├── alice/                     # Alice's private thoughts
│   ├── tickets/
│   │   └── PROJ-123.md        # Her notes on ticket
│   └── scratch.md             # Working notes
│
├── bob/                       # Bob's private thoughts
│   └── ...
│
├── global/                    # Cross-project patterns
│   ├── coding-standards.md
│   └── architecture-patterns.md
│
└── searchable/                # READ-ONLY mirror (hardlinks)
    ├── shared/...             # All shared content
    ├── alice/...              # Alice's content (when she searches)
    └── global/...             # All global content
```

### 2. Symlink + Hardlink Architecture

**Symlinks for Access**:
```bash
# In project directory
thoughts/ → ~/thoughts/projects/my-project/
```

**Hardlinks for Search**:
```bash
# Read-only directory with hardlinks to all searchable content
thoughts/searchable/
├── shared/research/api.md  # hardlink → thoughts/shared/research/api.md
└── alice/notes.md          # hardlink → thoughts/alice/notes.md
```

**Why Hardlinks for Searchable?**
- AI tools can search without following symlinks
- Read-only by convention (modifications go to source)
- No copy overhead (same inode)
- Automatic updates when source changes

### 3. Git Hook Protection

**Pre-commit Hook**:
```bash
#!/bin/bash
# Prevent thoughts directory from being committed

if git diff --cached --name-only | grep -q "^thoughts/"; then
    echo "ERROR: thoughts/ directory should not be committed"
    echo "These files are symlinked from your thoughts repository"
    exit 1
fi
```

**Post-commit Hook**:
```bash
#!/bin/bash
# Auto-sync thoughts after each commit

if command -v hlyr &> /dev/null; then
    hlyr thoughts sync -m "Auto-sync after commit $(git rev-parse --short HEAD)"
fi
```

**Key Insight**: Use git hooks for **protection** AND **automation**.

### 4. Multi-Profile Support

**Problem**: Developers work across multiple contexts (personal, client A, client B, open source).

**Solution**: Named profiles with different thoughts repositories.

```json
// ~/.config/humanlayer/humanlayer.json
{
  "thoughts": {
    "profiles": {
      "personal": {
        "repo": "~/thoughts-personal",
        "repos_dir": "~/thoughts-personal/projects",
        "global_dir": "~/thoughts-personal/global"
      },
      "client-acme": {
        "repo": "~/thoughts-acme",
        "repos_dir": "~/thoughts-acme/projects",
        "global_dir": "~/thoughts-acme/global"
      },
      "opensource": {
        "repo": "~/thoughts-oss",
        "repos_dir": "~/thoughts-oss/projects",
        "global_dir": "~/thoughts-oss/global"
      }
    },
    "repo_mappings": {
      "~/code/acme/*": "client-acme",
      "~/code/oss/*": "opensource",
      "*": "personal"
    }
  }
}
```

**Commands**:
```bash
hlyr thoughts profile create client-acme --repo ~/thoughts-acme
hlyr thoughts profile list
hlyr thoughts init --profile client-acme
```

### 5. AI-First Searchability

**Thoughts-Locator Agent Pattern**:

A specialized agent that ONLY locates (doesn't analyze) relevant documents:

```markdown
# thoughts-locator.md

You are a document locator for the thoughts/ directory.

## Your Tools
- Grep: Search file contents
- Glob: Find files by pattern
- LS: List directories

## Directory Structure
- thoughts/shared/ - Team documents
- thoughts/[username]/ - Personal notes
- thoughts/global/ - Cross-repo patterns
- thoughts/searchable/ - Search index

## Your Job
1. Search for documents matching the query
2. Categorize findings (tickets, research, plans, PRs)
3. Return PATHS, not content
4. Fix searchable/ paths → actual paths

## Critical Rule
If you find: thoughts/searchable/shared/research/api.md
Report as:  thoughts/shared/research/api.md
```

**Key Insight**: Separate "finding" from "reading" for token efficiency.

### 6. Synchronization Model

**Automatic Sync Points**:
1. **Post-commit**: Sync thoughts after code commits
2. **Manual**: Explicit sync with message
3. **Periodic**: Background sync (optional)

**Sync Command**:
```bash
hlyr thoughts sync -m "Updated research on auth patterns"
```

**What Sync Does**:
1. Commits changes in thoughts repo
2. Updates searchable/ hardlinks
3. Optionally pushes to remote

### 7. Privacy-First Design

**Privacy Layers**:

| Layer | Visibility | Storage |
|-------|------------|---------|
| Personal | Only me | Local or private remote |
| Shared | Team | Team remote |
| Global | All my projects | Local or synced |

**Never Committed to Code Repos**:
- Personal thoughts stay personal
- Shared thoughts live in separate repo
- Code repo only has symlinks (which are gitignored)

```gitignore
# .gitignore in code repo
thoughts/
```

---

## Context Engineering Principles

### From HumanLayer's "Advanced Context Engineering"

HumanLayer coined "context engineering" and developed key principles that inform how thoughts should be structured.

### 1. The Context Quality Hierarchy

```
PRIORITY (highest to lowest)
├── 1. CORRECTNESS - No wrong information
├── 2. COMPLETENESS - All necessary details
├── 3. SIZE - Minimal noise
└── 4. TRAJECTORY - Supports forward progress
```

**Application to Thoughts**: Notes should be **correct**, **complete**, **concise**, and **actionable**.

### 2. Frequent Intentional Compaction

When context fills up, **pause and compact** into structured artifacts:

```markdown
## Compaction Template

### End Goal
[What we're trying to achieve]

### Current Approach
[How we're tackling it]

### Completed Steps
- [x] Step 1
- [x] Step 2

### Current Obstacle
[What's blocking progress]

### Key Learnings
- Insight 1
- Insight 2
```

**Application to Thoughts**: Thoughts should follow templates that enable quick context restoration.

### 3. Research → Plan → Implement → Validate

```
┌─────────────────────────────────────────────────────────┐
│                    WORKFLOW PHASES                       │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  RESEARCH (thoughts/shared/research/)                   │
│  └─ Understand codebase, map relationships              │
│  └─ Generate structured findings                        │
│  └─ HIGHEST LEVERAGE for human review                   │
│                                                          │
│  PLAN (thoughts/shared/plans/)                          │
│  └─ Outline precise implementation steps                │
│  └─ Specify exact file modifications                    │
│  └─ HIGH LEVERAGE for human review                      │
│                                                          │
│  IMPLEMENT (code repo)                                  │
│  └─ Execute plan sequentially                           │
│  └─ Verify each component                               │
│  └─ Lower leverage (errors are isolated)                │
│                                                          │
│  VALIDATE (thoughts/shared/prs/)                        │
│  └─ Document what was done                              │
│  └─ Capture learnings                                   │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 4. High-Leverage Human Review

**Key Insight**: Research errors propagate through thousands of lines. Planning errors affect hundreds. Implementation errors are isolated.

```
REVIEW EFFORT ALLOCATION

Research Documents:    ████████████████████  40%
Plans:                 ██████████████        30%
Code:                  ██████████            20%
Other:                 █████                 10%
```

**Application to Thoughts**: Structure thoughts to support efficient human review at each phase.

### 5. Subagent Pattern for Context Preservation

**Problem**: Searching/exploring fills up context with noise.

**Solution**: Use subagents for search, return only summaries.

```
MAIN AGENT (implementation focused)
     │
     ├──► SUBAGENT: Search thoughts for "auth patterns"
     │         └──► Returns: 3 relevant files, 200 token summary
     │
     ├──► SUBAGENT: Find all references to UserService
     │         └──► Returns: 5 files, key relationships
     │
     └──► MAIN AGENT: Continues with clean context
```

**Application to Thoughts**: Thoughts-locator finds files, separate reader agent extracts content.

---

## Ideas for Your System

### 1. Storage Choice at Project Setup

```
┌─────────────────────────────────────────────────────────┐
│              THOUGHTS STORAGE OPTIONS                    │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Option A: LOCAL ONLY                                   │
│  └─ ~/thoughts/projects/[project-name]/                 │
│  └─ Never leaves machine                                │
│  └─ Best for: Personal projects, sensitive work         │
│                                                          │
│  Option B: LOCAL + PRIVATE REMOTE                       │
│  └─ ~/thoughts/ → git@private:thoughts.git              │
│  └─ Synced to your private repo                         │
│  └─ Best for: Cross-machine access, backup              │
│                                                          │
│  Option C: TEAM SHARED                                  │
│  └─ ~/thoughts/ → git@team:team-thoughts.git            │
│  └─ Shared with team members                            │
│  └─ Best for: Team collaboration                        │
│                                                          │
│  Option D: HYBRID (Recommended)                         │
│  └─ Personal: local or private remote                   │
│  └─ Shared: team remote                                 │
│  └─ Global: synced across all                           │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 2. Initialization Wizard

```bash
$ pmsynapse thoughts init

🧠 PMSynapse Thoughts Setup

? Where would you like to store thoughts for this project?
  ○ Local only (~/thoughts/local/my-project)
  ○ Private remote (sync to your private git)
  ○ Team shared (sync to team git)
  ● Hybrid (personal local, shared remote)

? Select or create a profile:
  ○ personal (default)
  ○ work-acme
  ● Create new profile...

? Profile name: client-beta

? Personal thoughts storage:
  ● Local: ~/thoughts/client-beta/[username]/
  ○ Remote: git@private:thoughts-beta.git

? Shared thoughts storage:
  ○ Local: ~/thoughts/client-beta/shared/
  ● Remote: git@team:beta-thoughts.git

✅ Thoughts initialized for my-project
   Profile: client-beta
   Personal: ~/thoughts/client-beta/alice/ (local)
   Shared: git@team:beta-thoughts.git (remote)

   Symlinks created:
   ./thoughts/alice → ~/thoughts/client-beta/alice
   ./thoughts/shared → ~/thoughts/client-beta/shared
   ./thoughts/global → ~/thoughts/global
```

### 3. Recommended Directory Structure

```
~/thoughts/                           # Root thoughts directory
├── config.json                       # Global configuration
├── global/                           # Cross-project thoughts
│   ├── patterns/                     # Reusable patterns
│   ├── learnings/                    # General learnings
│   └── templates/                    # Note templates
│
├── profiles/
│   ├── personal/                     # Personal profile
│   │   ├── projects/
│   │   │   └── my-side-project/
│   │   └── [username]/
│   │
│   ├── work-acme/                    # Work profile
│   │   ├── projects/
│   │   │   ├── acme-api/
│   │   │   └── acme-web/
│   │   ├── shared/                   # Synced with team
│   │   └── [username]/               # Personal work notes
│   │
│   └── opensource/                   # OSS profile
│       ├── projects/
│       └── shared/
│
└── .gitignore
```

### 4. Document Templates

**Research Template**:
```markdown
# Research: [Topic]

## Date
YYYY-MM-DD

## Question
What are we trying to understand?

## Key Findings
1. Finding 1
2. Finding 2

## Relevant Files
- `src/auth/jwt.ts` - Current implementation
- `src/middleware/auth.ts` - Usage

## Recommendations
- Recommendation 1
- Recommendation 2

## Open Questions
- [ ] Question 1
- [ ] Question 2

## References
- [Link 1](url)
- [Link 2](url)
```

**Plan Template**:
```markdown
# Plan: [Feature/Task]

## Goal
What we're building

## Prerequisites
- [ ] Prereq 1
- [ ] Prereq 2

## Implementation Steps

### Phase 1: [Name]
- [ ] Step 1.1: Description
  - Files: `src/file.ts`
  - Changes: What to modify
- [ ] Step 1.2: Description

### Phase 2: [Name]
- [ ] Step 2.1: Description

## Testing Strategy
- Unit tests for X
- Integration tests for Y

## Rollback Plan
How to undo if needed

## Status
🟡 In Progress / ✅ Complete / 🔴 Blocked
```

**Ticket Context Template**:
```markdown
# Ticket: [ID] - [Title]

## Original Requirements
Copy from ticket system

## My Understanding
What I think this means

## Questions for PM/Stakeholder
- [ ] Question 1
- [ ] Question 2

## Technical Notes
Implementation details discovered

## Related Thoughts
- [[research/related-topic.md]]
- [[plans/feature-plan.md]]

## Progress Log
- YYYY-MM-DD: Started research
- YYYY-MM-DD: Found approach
```

### 5. Sync Strategies

```typescript
interface ThoughtsSyncConfig {
  // When to auto-sync
  autoSync: {
    onCommit: boolean;        // After git commit
    onSave: boolean;          // After file save
    periodic: number | null;  // Minutes, or null for disabled
  };

  // What to sync
  syncScope: {
    personal: 'local' | 'remote';
    shared: 'local' | 'remote';
    global: 'local' | 'remote';
  };

  // Conflict resolution
  conflicts: 'prompt' | 'theirs' | 'ours' | 'manual';

  // Sync message
  messageTemplate: string;  // e.g., "Sync: {timestamp} from {hostname}"
}
```

### 6. AI Integration Patterns

**Search Tool for AI Agents**:
```typescript
interface ThoughtsSearchTool {
  name: 'search_thoughts';
  description: 'Search project thoughts and documentation';
  parameters: {
    query: string;           // What to search for
    scope: 'all' | 'shared' | 'personal' | 'global';
    type?: 'research' | 'plan' | 'ticket' | 'pr' | 'any';
    limit?: number;
  };
  returns: {
    files: Array<{
      path: string;
      relevance: number;
      snippet: string;
    }>;
  };
}
```

**Context Injection**:
```markdown
## System Prompt Addition

When working on this project, you have access to a thoughts/ directory:

- thoughts/shared/ - Team knowledge (research, plans, tickets, PRs)
- thoughts/[your-name]/ - Your personal notes
- thoughts/global/ - Cross-project patterns

Before implementing:
1. Search thoughts for relevant context
2. Check for existing research on the topic
3. Look for related plans or tickets

After completing work:
1. Update relevant thought documents
2. Create new research/plan if significant learning occurred
```

---

## Benefits Summary

### For Individual Developers
- Private notes never accidentally committed
- Context preserved across sessions
- Personal knowledge base grows over time
- Cross-project patterns accessible everywhere

### For Teams
- Shared research reduces duplicate work
- Plans visible before implementation
- PR context available for review
- Knowledge persists when people leave

### For AI Assistants
- Searchable context outside code
- Research available for decisions
- Historical patterns inform new work
- Token-efficient (search then read)

### For Organizations
- Knowledge doesn't live in Slack/email
- Decisions documented and searchable
- Onboarding accelerated
- Institutional memory preserved

---

## Implementation Recommendations

### Phase 1: Core System
1. Local thoughts directory structure
2. Symlink creation for projects
3. Git hook protection
4. Basic CLI (init, sync, status)

### Phase 2: Multi-Profile
1. Profile configuration
2. Repository mappings
3. Profile switching
4. Remote sync support

### Phase 3: AI Integration
1. Thoughts-locator agent
2. Search tool for AI assistants
3. Template enforcement
4. Auto-compaction suggestions

### Phase 4: Team Features
1. Shared thoughts repositories
2. Conflict resolution
3. Access controls
4. Activity feeds

---

## Sources

- [HumanLayer GitHub Repository](https://github.com/humanlayer/humanlayer)
- [HumanLayer Thoughts Locator Agent](https://github.com/humanlayer/humanlayer/blob/main/.claude/agents/thoughts-locator.md)
- [Advanced Context Engineering for Coding Agents](https://github.com/humanlayer/advanced-context-engineering-for-coding-agents/blob/main/ace-fca.md)
- [Anthropic - Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents)
- [Catalyst Project (Uses HumanLayer Thoughts)](https://github.com/coalesce-labs/catalyst)

---

*Analysis completed: December 2025*
*Part of: PMSynapse AI-Enabled Project Management Research*
