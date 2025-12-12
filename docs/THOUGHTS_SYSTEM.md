# PMSynapse Thoughts System

A flexible thoughts management system that keeps project knowledge separate from code, enabling AI-assisted workflows while protecting sensitive information.

## Overview

The PMSynapse thoughts system stores research, plans, tickets, and personal notes **outside** your code repository, linked via symlinks. This prevents accidental commits while enabling AI agents to search and use this context.

```
CODE REPO (public/shared)              THOUGHTS REPO (private)

my-project/                            ~/.pmsynapse/thoughts/
├── src/                               ├── profiles/
├── .pmsynapse/                        │   ├── personal/
│   └── config.yaml                    │   │   └── projects/my-project/
├── thoughts/ ←──── symlink ───────────│   │       ├── shared/
│   ├── shared/                        │   │       │   ├── research/
│   ├── [username]/                    │   │       │   ├── plans/
│   ├── global/                        │   │       │   └── tickets/
│   └── searchable/                    │   │       └── alice/
└── .gitignore (blocks thoughts/)      │   └── work-acme/
                                       ├── global/
                                       └── config.yaml
```

## Quick Start

```bash
# Initialize thoughts for current project
snps thoughts init

# Create a new research document
snps thoughts new research "Authentication Patterns"

# Create a plan
snps thoughts new plan "Implement OAuth2"

# Search thoughts
snps thoughts search "auth"

# Sync thoughts (commit + push to remote if configured)
snps thoughts sync -m "Updated auth research"

# List all thoughts
snps thoughts list

# Open thoughts directory
snps thoughts open
```

## Directory Structure

### In Your Project

After `snps thoughts init`, your project will have:

```
my-project/
├── thoughts/                    # Symlink to thoughts repo
│   ├── shared/                  # Team knowledge
│   │   ├── research/            # Technical investigations
│   │   ├── plans/               # Implementation plans
│   │   ├── tickets/             # Ticket context & notes
│   │   └── prs/                 # PR descriptions & context
│   │
│   ├── [username]/              # Your personal notes
│   │   ├── scratch.md           # Quick notes
│   │   ├── tickets/             # Personal ticket notes
│   │   └── journal/             # Daily work log
│   │
│   ├── global/                  # Cross-project patterns
│   │   ├── patterns/            # Reusable solutions
│   │   ├── learnings/           # General insights
│   │   └── templates/           # Document templates
│   │
│   └── searchable/              # AI search index (hardlinks)
│       ├── shared/...
│       ├── [username]/...
│       └── global/...
│
└── .gitignore                   # Contains: thoughts/
```

### Global Thoughts Directory

```
~/.pmsynapse/thoughts/
├── config.yaml                  # Global thoughts configuration
├── profiles/
│   ├── personal/
│   │   └── projects/
│   │       └── [project-name]/
│   │           ├── shared/
│   │           └── [username]/
│   │
│   └── [profile-name]/
│       └── projects/
│           └── [project-name]/
│
└── global/                      # Shared across all profiles
    ├── patterns/
    ├── learnings/
    └── templates/
```

## CLI Commands

### `snps thoughts init`

Initialize thoughts for the current project.

```bash
snps thoughts init [OPTIONS]

Options:
  --profile <NAME>    Use specific profile (default: auto-detect or 'personal')
  --storage <TYPE>    Storage type: local, remote, hybrid (default: local)
  --remote <URL>      Git remote for syncing
  --no-hooks          Skip git hook installation
  --force             Overwrite existing thoughts setup

Examples:
  snps thoughts init
  snps thoughts init --profile work-acme
  snps thoughts init --storage hybrid --remote git@github.com:team/thoughts.git
```

### `snps thoughts new`

Create a new thought document from template.

```bash
snps thoughts new <TYPE> <TITLE> [OPTIONS]

Types:
  research    Technical investigation document
  plan        Implementation plan
  ticket      Ticket context and notes
  pr          Pull request description
  scratch     Quick notes (personal)
  journal     Daily work log (personal)

Options:
  --scope <SCOPE>     Where to create: shared, personal, global (default: shared)
  --template <FILE>   Custom template file
  --open              Open in editor after creation

Examples:
  snps thoughts new research "GraphQL Migration"
  snps thoughts new plan "User Authentication" --scope shared
  snps thoughts new ticket "PROJ-123" --scope personal
  snps thoughts new scratch "Quick ideas" --scope personal
```

### `snps thoughts search`

Search through thoughts using ripgrep.

```bash
snps thoughts search <QUERY> [OPTIONS]

Options:
  --scope <SCOPE>     Search scope: all, shared, personal, global (default: all)
  --type <TYPE>       Filter by type: research, plan, ticket, pr, any
  --paths-only        Return only file paths (for AI agents)
  --limit <N>         Limit results (default: 10)
  --context <N>       Lines of context around matches (default: 2)

Examples:
  snps thoughts search "authentication"
  snps thoughts search "OAuth" --type research
  snps thoughts search "PROJ-123" --scope personal --paths-only
```

### `snps thoughts list`

List thought documents.

```bash
snps thoughts list [OPTIONS]

Options:
  --scope <SCOPE>     Filter by scope: all, shared, personal, global
  --type <TYPE>       Filter by type
  --recent <N>        Show N most recently modified (default: all)
  --format <FMT>      Output format: table, json, paths (default: table)

Examples:
  snps thoughts list
  snps thoughts list --scope shared --type research
  snps thoughts list --recent 10 --format json
```

### `snps thoughts sync`

Synchronize thoughts with remote (if configured).

```bash
snps thoughts sync [OPTIONS]

Options:
  -m, --message <MSG>  Commit message
  --push               Push to remote after commit
  --pull               Pull from remote before commit
  --no-commit          Only update searchable/ index

Examples:
  snps thoughts sync -m "Updated OAuth research"
  snps thoughts sync --pull --push
  snps thoughts sync --no-commit  # Just rebuild searchable/ index
```

### `snps thoughts open`

Open thoughts in file explorer or editor.

```bash
snps thoughts open [PATH] [OPTIONS]

Options:
  --editor             Open in default editor
  --scope <SCOPE>      Open specific scope directory

Examples:
  snps thoughts open
  snps thoughts open shared/research
  snps thoughts open --scope personal --editor
```

### `snps thoughts profile`

Manage thoughts profiles.

```bash
snps thoughts profile <COMMAND>

Commands:
  list                     List all profiles
  create <NAME>            Create a new profile
  switch <NAME>            Switch active profile for current project
  delete <NAME>            Delete a profile
  show                     Show current profile details

Options (for create):
  --repo <PATH>            Local repository path
  --remote <URL>           Git remote URL
  --clone                  Clone from remote

Examples:
  snps thoughts profile list
  snps thoughts profile create work-acme --remote git@work:thoughts.git --clone
  snps thoughts profile switch work-acme
```

### `snps thoughts hooks`

Manage git hooks for thoughts protection.

```bash
snps thoughts hooks <COMMAND>

Commands:
  install                  Install pre-commit hook
  uninstall                Remove pre-commit hook
  status                   Check hook status

Examples:
  snps thoughts hooks install
  snps thoughts hooks status
```

## Configuration

### Project Configuration (`.pmsynapse/config.yaml`)

```yaml
# PMSynapse project configuration
project:
  name: my-project

thoughts:
  profile: personal           # Which profile to use
  storage: local              # local | remote | hybrid
  remote: null                # Git remote URL (if remote/hybrid)

  auto_sync:
    on_commit: false          # Sync after git commits
    on_save: false            # Sync after file saves
    periodic_minutes: null    # Background sync interval

  scopes:
    personal: local           # Where personal thoughts live
    shared: local             # Where shared thoughts live
    global: local             # Where global thoughts live
```

### Global Configuration (`~/.pmsynapse/thoughts/config.yaml`)

```yaml
# Global thoughts configuration
default_profile: personal

profiles:
  personal:
    repo: ~/.pmsynapse/thoughts/profiles/personal
    remote: null

  work-acme:
    repo: ~/.pmsynapse/thoughts/profiles/work-acme
    remote: git@work:team-thoughts.git

# Auto-select profile based on project path
repo_mappings:
  "~/code/acme/*": work-acme
  "~/code/oss/*": personal
  "*": personal

# Username for personal directories
username: alice

# Editor for 'thoughts open --editor'
editor: code

# Sync settings
sync:
  message_template: "Sync: {timestamp} from {hostname}"
  auto_push: false

# Templates location
templates_dir: ~/.pmsynapse/thoughts/global/templates
```

## Templates

### Research Template

```markdown
# Research: {title}

## Date
{date}

## Question
What are we trying to understand?

## Background
Why is this research needed?

## Key Findings

### Finding 1
Description and evidence

### Finding 2
Description and evidence

## Relevant Code
- `src/path/to/file.rs` - Description
- `src/another/file.ts` - Description

## Recommendations
1. Recommendation with rationale
2. Another recommendation

## Open Questions
- [ ] Unanswered question 1
- [ ] Unanswered question 2

## References
- [Link title](url)
- [Another reference](url)

## Status
🟡 In Progress | ✅ Complete | 🔴 Blocked

---
*Created by {username} on {date}*
```

### Plan Template

```markdown
# Plan: {title}

## Goal
What we're building and why

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Prerequisites
- [ ] Required before starting
- [ ] Another prerequisite

## Implementation Steps

### Phase 1: {phase_name}
- [ ] **Step 1.1**: Description
  - Files: `src/file.rs`
  - Changes: What to modify
- [ ] **Step 1.2**: Description

### Phase 2: {phase_name}
- [ ] **Step 2.1**: Description

## Testing Strategy
- Unit tests: What to test
- Integration tests: What to test
- Manual verification: Steps

## Rollback Plan
How to undo if something goes wrong

## Timeline Estimate
- Phase 1: X story points
- Phase 2: Y story points

## Dependencies
- External: API access, credentials
- Internal: Other team's work

## Status
🟡 Planning | 🔵 In Progress | ✅ Complete | 🔴 Blocked

## Progress Log
- {date}: Started planning
- {date}: Completed Phase 1

---
*Created by {username} on {date}*
```

### Ticket Template

```markdown
# Ticket: {ticket_id} - {title}

## Original Requirements
> Paste original ticket description here

## My Understanding
What I think this ticket is asking for

## Acceptance Criteria
- [ ] Criterion from ticket
- [ ] Another criterion

## Questions for Stakeholder
- [ ] Clarification needed on X
- [ ] Edge case: what if Y?

## Technical Approach
High-level approach to implementation

## Technical Notes
Implementation details discovered during work

## Related Resources
- Research: [[research/related-topic.md]]
- Plan: [[plans/feature-plan.md]]
- Code: `src/relevant/file.rs`

## Progress Log
- {date}: Started investigation
- {date}: Found approach, starting implementation
- {date}: Completed, ready for review

## Time Tracking
- Research: X hours
- Implementation: Y hours
- Review/fixes: Z hours

---
*Ticket link: [PROJ-123](url)*
*Created by {username} on {date}*
```

## Git Hooks

### Pre-commit Hook

Installed by `snps thoughts hooks install`:

```bash
#!/bin/bash
# PMSynapse: Prevent thoughts/ from being committed to code repo

if git diff --cached --name-only | grep -q "^thoughts/"; then
    echo "❌ ERROR: thoughts/ directory should not be committed"
    echo ""
    echo "The thoughts/ directory is symlinked from your thoughts repository."
    echo "These files should be committed there instead:"
    echo ""
    echo "  snps thoughts sync -m 'Your message'"
    echo ""
    echo "If you really need to commit these files, use:"
    echo "  git commit --no-verify"
    echo ""
    exit 1
fi
```

### Post-commit Hook (Optional)

Auto-sync thoughts after code commits:

```bash
#!/bin/bash
# PMSynapse: Auto-sync thoughts after commit

if command -v snps &> /dev/null; then
    if [ -d "thoughts" ] && [ -L "thoughts" ]; then
        snps thoughts sync --no-commit -m "Auto-sync after $(git rev-parse --short HEAD)"
    fi
fi
```

## AI Agent Integration

### Thoughts Locator Pattern

For AI agents, use the `--paths-only` flag to efficiently find relevant documents:

```bash
# Find without reading (token efficient)
snps thoughts search "authentication" --paths-only

# Output:
# thoughts/shared/research/auth-patterns.md
# thoughts/shared/plans/oauth2-implementation.md
# thoughts/personal/tickets/AUTH-123.md
```

### System Prompt Addition

When working with AI agents, add this to the system prompt:

```markdown
## Project Thoughts

This project has a thoughts/ directory for documentation:

- `thoughts/shared/` - Team knowledge (research, plans, tickets)
- `thoughts/[username]/` - Personal notes
- `thoughts/global/` - Cross-project patterns

**Before implementing**, search for relevant context:
```bash
snps thoughts search "<topic>" --paths-only
```

**After completing work**, update relevant thoughts:
```bash
snps thoughts sync -m "Updated after implementing X"
```
```

### Searchable Index

The `thoughts/searchable/` directory contains hardlinks to all searchable content, optimized for AI tools that don't follow symlinks:

```bash
# Rebuild searchable index
snps thoughts sync --no-commit
```

## Workflow Examples

### Starting a New Feature

```bash
# 1. Create research document
snps thoughts new research "Feature X Requirements"

# 2. Do research, update the document
code thoughts/shared/research/feature-x-requirements.md

# 3. Create implementation plan
snps thoughts new plan "Implement Feature X"

# 4. Sync thoughts
snps thoughts sync -m "Added Feature X research and plan"

# 5. Implement the feature
# ... coding ...

# 6. Update plan with progress
snps thoughts sync -m "Completed Phase 1 of Feature X"
```

### Working on a Ticket

```bash
# 1. Create ticket context
snps thoughts new ticket "PROJ-456"

# 2. Research existing patterns
snps thoughts search "similar functionality"

# 3. Add notes as you work
code thoughts/personal/tickets/PROJ-456.md

# 4. Sync periodically
snps thoughts sync -m "PROJ-456 progress update"
```

### Sharing Knowledge

```bash
# 1. Create shared research
snps thoughts new research "Best Practices for X" --scope shared

# 2. Sync to team remote
snps thoughts sync --push -m "Added best practices documentation"

# 3. Team members pull updates
snps thoughts sync --pull
```

## Privacy Model

| Scope | Visibility | Default Storage | Sync Behavior |
|-------|------------|-----------------|---------------|
| `personal` | Only you | Local | Optional remote |
| `shared` | Team | Remote | Auto-push available |
| `global` | All your projects | Local | Sync across machines |

### Privacy Guarantees

1. **Code repo never contains thoughts** - Blocked by git hooks
2. **Personal thoughts stay personal** - Separate directories, optional sync
3. **Shared thoughts are explicit** - Must be created in `shared/` scope
4. **Profiles isolate contexts** - Work thoughts separate from personal

## Troubleshooting

### Thoughts not appearing in project

```bash
# Check if symlink exists and is valid
ls -la thoughts/

# Reinitialize if broken
snps thoughts init --force
```

### Search not finding documents

```bash
# Rebuild searchable index
snps thoughts sync --no-commit

# Check searchable directory
ls thoughts/searchable/
```

### Git hook blocking commits incorrectly

```bash
# Check if thoughts/ is actually staged
git status

# If legitimate, bypass once
git commit --no-verify -m "message"

# Or remove from staging
git reset HEAD thoughts/
```

### Profile not switching

```bash
# Check current profile
snps thoughts profile show

# Verify profile exists
snps thoughts profile list

# Check repo mapping
cat ~/.pmsynapse/thoughts/config.yaml
```

## Migration from Other Systems

### From scattered markdown files

```bash
# Initialize thoughts
snps thoughts init

# Move existing docs
mv docs/research/* thoughts/shared/research/
mv docs/plans/* thoughts/shared/plans/
mv notes/* thoughts/personal/

# Rebuild index
snps thoughts sync -m "Migrated from docs/"
```

### From HumanLayer

PMSynapse thoughts are compatible with HumanLayer's structure. Simply:

```bash
# Point to existing thoughts repo
snps thoughts init --storage local

# Update symlink to existing thoughts
ln -sf ~/humanlayer-thoughts thoughts
```

---

*Part of PMSynapse AI-Enabled Project Management*
