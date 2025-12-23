# Knowledge Management System Tutorial

Complete guide to using `snps know` - PMSynapse's unified knowledge management system.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Command Reference](#command-reference)
- [Common Workflows](#common-workflows)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

---

## Overview

The `snps know` command (alias for `snps knowledge`) manages knowledge across three scopes:
- **User**: Personal knowledge (your notes, research)
- **Team**: Shared team knowledge (standards, patterns)
- **Project**: Project-specific knowledge (this codebase)

Knowledge lives in separate git repositories (called "shadow repos") and syncs bidirectionally to your working copy.

### Key Features

✅ **Three-level scoping** - User, team, and project knowledge repos

✅ **Bidirectional sync** - Automatic timestamp-based push/pull

✅ **Conflict detection** - Warns when both local and shadow changed

✅ **Interactive prompts** - Plan preview with y/N/f confirmation

✅ **Git integration** - Auto-manages `.git/info/exclude`

✅ **Worktree support** - Works with `git worktree` setups

✅ **Smart exclusions** - Configurable patterns (`.git/`, `node_modules/`, `target/`)

✅ **Verbose mode** - Show/hide skipped files with `--verbose`

---

## Quick Start

### 1. Initialize Knowledge System

```bash
# Interactive setup (recommended for first time)
snps know init --interactive

# Or specify paths directly
snps know init \
  --user ~/personal-knowledge \
  --team ~/eng-team-knowledge \
  --project ../project-knowledge
```

This creates:
- `.pmsynapse/repositories.yaml` - Repository configuration
- `knowledge/` - Merged working copy of all repos

### 2. Add Knowledge Repositories

```bash
# Add a user repository
snps know repo add user ~/my-notes

# Add a team repository
snps know repo add team ~/team-shared --id eng-team

# Add project repository
snps know repo add project ../docs-repo
```

### 3. Sync Knowledge

```bash
# Interactive sync (shows plan, prompts for confirmation)
snps know sync

# Non-interactive sync (auto-apply without prompt)
snps know sync --apply

# Dry run (see what would change, no prompt)
snps know sync --dry-run

# Show all operations including skipped files
snps know sync --verbose

# Force resolve conflicts (newer timestamp wins)
snps know sync --force
```

### 4. Search and Browse

```bash
# Search across all knowledge
snps know search "authentication"

# List all documents
snps know list

# Check sync status
snps know status
```

---

## Core Concepts

### Shadow Repositories

Shadow repos are external git repositories containing knowledge artifacts. They're called "shadow" because they exist outside your project codebase.

**Structure**:
```
my-knowledge-repo/
├── research/
│   └── api-design.md
├── plans/
│   └── refactor-auth.md
└── .claude/                # Optional
    └── agents/
```

**Important**: Shadow repos are managed externally - `snps` only copies files.

### Precedence Rules

When the same file exists in multiple repos:
- **Project > Team > User** (higher precedence wins)
- Example: `project/research/api.md` overrides `team/research/api.md`

### Sync Behavior

**Hash + Timestamp-based Sync** (automatic direction):
- **SKIP**: Identical content hash (unchanged) - even if timestamps differ
- **PULL** (shadow → local): Shadow file is newer AND content differs
- **PUSH** (local → shadow): Local file is newer AND content differs
- **CONFLICT**: Same timestamp, different content

**Important**: Sync uses content hashes to detect changes, not just timestamps. Running `touch file.md` won't trigger a sync - you must actually modify the file content.

**Sync Plan & Apply**:
1. **Plan phase**: Scan repos, compare timestamps, build plan
2. **Preview**: Show categorized operations (PULL/PUSH/CONFLICT/SKIP)
3. **Prompt**: User confirms (y/N/f) or uses `--apply` for auto-execution
4. **Apply phase**: Execute approved operations

**Notes**:
- Push always targets project shadow repo only
- Never pushes to user/team repos (read-only)
- `--force` resolves conflicts by using newer timestamp
- `--verbose` shows SKIP operations (hidden by default)

---

## Command Reference

### Initialization

```bash
# Initialize with interactive prompts
snps know init --interactive

# Initialize with explicit paths
snps know init --user PATH1 --team PATH2 --project PATH3

# Multiple repos per context
snps know init --user PATH1 --user PATH2 --team PATH3
```

### Repository Management

```bash
# Add shadow repository
snps know repo add <context> <path> [--id ID]
# Context: user, team, or project

# Remove repository by ID
snps know repo remove <id>

# List all configured repositories
snps know repo list

# Show repository details
snps know repo show <id>
```

### Synchronization

```bash
# Interactive sync (shows plan, prompts for confirmation)
snps know sync

# Non-interactive sync (auto-apply without prompt)
snps know sync --apply

# Preview changes (dry run, no prompt, no execution)
snps know sync --dry-run

# Show all operations including skipped files
snps know sync --verbose
snps know sync --dry-run --verbose

# Force resolve conflicts (newer timestamp wins)
snps know sync --force

# Filter sync
snps know sync --context user     # Only sync user repos
snps know sync --repo <id>        # Only sync specific repo

# Check sync status
snps know status
```

### Worktree Inheritance

```bash
# Inherit configuration from parent repository (worktrees only)
snps know inherit

# Force inheritance (override existing .pmsynapse directory)
snps know inherit --force
```

**How it works**:
- Detects if current directory is a git worktree
- Creates symlink from worktree's `.pmsynapse/` to parent's `.pmsynapse/`
- Automatically runs during worktree creation via `hack/create_worktree.sh`
- Each worktree maintains independent `knowledge/` directory
- Shared configuration enables consistent shadow repo setup across worktrees

### Knowledge Operations

```bash
# Search knowledge (uses ripgrep if available)
snps know search <query>

# List knowledge documents
snps know list

# Add file to shadow repository
snps know file add <path> [--repo <id>] [--context <user|team|project>]

# Remove file from shadow repository
snps know file remove <path> [--delete-local]
```

---

## Common Workflows

### Daily Research Workflow

```bash
# Morning: Sync with team (interactive)
snps know sync
# Shows plan:
#   PULL knowledge/team-notes.md (shadow newer)
#   SKIP knowledge/my-research.md (unchanged)
# Prompt: Apply changes? (y/N/f)
# Enter: y

# Work on research
vim knowledge/research/my-topic.md

# Evening: Share with team (auto-apply)
snps know sync --apply
# Shows plan:
#   PUSH knowledge/my-topic.md (local newer)
# Automatically executes without prompt
```

### Starting a New Project

```bash
# Clone project
git clone https://github.com/company/project
cd project

# Initialize knowledge
snps know init \
  --user ~/personal-knowledge \
  --team ~/team-knowledge \
  --project ../project-knowledge

# Pull all knowledge
snps know sync --pull-only

# Verify
snps know status
snps know list
```

### Resolving Conflicts

When sync shows conflicts:

```bash
$ snps know sync
Sync Plan:
  PULL knowledge/team-notes.md (shadow newer)
  CONFLICT knowledge/plans/refactor.md
    Local:  2025-12-22 10:30:00 (hash: a1b2c3)
    Shadow: 2025-12-22 10:30:00 (hash: d4e5f6)

Apply changes? (y/N/f)
```

**Resolution Options**:

1. **Manual merge** (recommended):
   ```bash
   # Cancel sync (N), merge manually
   vim knowledge/plans/refactor.md
   # Touch file to update timestamp
   touch knowledge/plans/refactor.md
   # Re-sync (local now newer)
   snps know sync
   ```

2. **Force newer wins**:
   ```bash
   # At prompt, enter 'f' to force
   # Or use --force flag
   snps know sync --force
   ```

3. **Ignore conflict** (enter 'N'):
   ```bash
   # Skips conflicted file, applies other changes
   # Conflict remains for next sync
   ```

### Working Across Multiple Projects

Shadow repos can be shared across projects:

```bash
# Project A
cd ~/work/project-a
snps know init --user ~/personal --team ~/team

# Project B
cd ~/work/project-b
snps know init --user ~/personal --team ~/team  # Same repos!

# Changes in one project sync to shadow, then to other projects
```

### Working with Git Worktrees

Git worktrees allow multiple working directories from the same repository. The knowledge system supports worktrees with automatic configuration inheritance:

```bash
# Create worktree (automatically inherits configuration)
./hack/create_worktree.sh feature-branch

# Or create worktree manually, then inherit
git worktree add ../my-feature feature-branch
cd ../my-feature
snps know inherit

# Verify inheritance
ls -la .pmsynapse  # Should show symlink to parent
readlink .pmsynapse  # Shows: /path/to/parent/.pmsynapse

# Each worktree has independent knowledge/
ls knowledge/  # Independent working directory

# Sync works normally in worktree
snps know sync
```

**How worktree inheritance works**:
1. `.pmsynapse/` configuration is shared via symlink (points to parent)
2. `knowledge/` directory is independent per worktree
3. Shadow repositories are shared across all worktrees
4. Changes sync through shadow repos: worktree → shadow → other worktrees
5. `.git/info/exclude` is automatically managed in the shared git directory

**Benefits**:
- No need to run `snps know init` in each worktree
- Consistent shadow repo configuration across worktrees
- Independent knowledge directories for parallel work
- Automatic synchronization through shadow repositories

### Adding Files to Specific Repositories

Manually add files to shadow repos (useful for selective sharing):

```bash
# Create research document
vim knowledge/research/new-architecture.md

# Add to team repo (share with team)
snps know file add knowledge/research/new-architecture.md --context team

# Create personal notes
vim knowledge/research/my-notes.md

# Add to user repo (keep private)
snps know file add knowledge/research/my-notes.md --context user

# Verify files were copied
snps know repo list
ls ~/team-knowledge/knowledge/research/
ls ~/personal-knowledge/knowledge/research/
```

---

## Advanced Usage

### Configuration File

Location: `.pmsynapse/repositories.yaml`

```yaml
version: "1.0"

# Files matching these patterns won't be added to .git/info/exclude
git_exclude_patterns:
  - "^\\.pmsynapse/"     # Always exclude .pmsynapse/
  - "^knowledge/"        # Always exclude knowledge/
  - "^src/"              # Don't exclude src/ (even if synced)

# Directories excluded during sync (never copied)
# Uses regex patterns - matches at any level
sync_exclude_patterns:
  - "(^|/)\\.git$"           # .git directory
  - "(^|/)\\.git/"           # .git subdirectories
  - "(^|/)node_modules$"     # node_modules directory
  - "(^|/)node_modules/"     # node_modules subdirectories
  - "(^|/)target$"           # Rust build artifacts
  - "(^|/)target/"           # Rust build subdirectories
  - "(^|/)\\.pmsynapse$"     # PMSynapse config
  - "(^|/)\\.pmsynapse/"     # PMSynapse subdirectories

repositories:
  - path: /Users/igor/personal-knowledge
    id: user-personal
    description: "Personal research and notes"
    type: folder
    context: user
    enabled: true

  - path: /Users/igor/eng-team
    id: eng-team
    type: folder
    context: team
    enabled: true

  - path: ../project-knowledge
    id: project-main
    type: folder
    context: project
    enabled: true
```

### Git Integration

**Automatic .git/info/exclude Management**:

After sync, `snps` updates `.git/info/exclude`:

```
# BEGIN snps-know auto-generated
# DO NOT EDIT - Changes will be overwritten by 'snps know sync'
.pmsynapse/
knowledge/
.claude/
# END snps-know auto-generated
```

**Note**: Manual entries above/below the markers are preserved.

**Worktree Support**:

`snps` provides first-class support for git worktrees with automatic configuration inheritance:

```bash
# Initialize in main repository
cd ~/work/main
snps know init --user ~/personal --team ~/team --project ../knowledge

# Create worktree (automatically inherits configuration)
./hack/create_worktree.sh feature-branch
cd ../pmsynapse-worktrees/feature-branch

# Configuration is shared via symlink
ls -la .pmsynapse  # → /Users/igor/work/main/.pmsynapse

# Knowledge directory is independent
ls -la knowledge/  # Independent working directory

# Sync works normally
snps know sync

# Updates shared .git/info/exclude (not worktree-specific)
# Located at: ~/work/main/.git/info/exclude
```

**Manual worktree setup**:
```bash
# If worktree was created without the script
git worktree add ../my-feature feature-branch
cd ../my-feature
snps know inherit  # Inherits parent's .pmsynapse/ via symlink
snps know sync     # Works immediately with inherited config
```

**Key points**:
- `.pmsynapse/` is shared (symlink to parent)
- `knowledge/` is independent per worktree
- `.git/info/exclude` is shared across all worktrees
- Shadow repositories are shared configuration

### Selective Sync

```bash
# Sync only user context
snps know sync --context user --apply

# Sync only specific repository
snps know sync --repo eng-team --apply

# Dry run specific context
snps know sync --context team --dry-run --verbose
```

### Interactive vs Non-Interactive

**Interactive mode** (default):
```bash
$ snps know sync
Sync Plan:
  PULL knowledge/team-notes.md (shadow newer)
  PUSH knowledge/my-research.md (local newer)
  SKIP knowledge/unchanged.md (identical)

Apply changes? (y/N/f): y
✓ Applied 2 operations
```

**Non-interactive mode** (`--apply`):
```bash
$ snps know sync --apply
Sync Plan:
  PULL knowledge/team-notes.md
  PUSH knowledge/my-research.md
✓ Applied 2 operations
```

**Dry run** (`--dry-run`):
```bash
$ snps know sync --dry-run
Sync Plan (DRY RUN):
  PULL knowledge/team-notes.md
  PUSH knowledge/my-research.md
No changes applied.
```

### Repository Paths

**Absolute paths**:
```bash
snps know repo add user /Users/igor/knowledge
```

**Relative paths** (relative to project root):
```bash
snps know repo add project ../shared-knowledge
snps know repo add user ../../personal
```

### File Registration

Add files to shadow repositories (copies file from project to shadow repo):

```bash
# Add file to project repository (default)
snps know file add knowledge/research/api.md

# Add file to specific repository by ID
snps know file add knowledge/research/api.md --repo project-main

# Add file to repository by context (first matching repo)
snps know file add knowledge/research/api.md --context project
snps know file add knowledge/research/api.md --context team
snps know file add knowledge/research/api.md --context user

# Remove from shadow repository (keeps local file)
snps know file remove knowledge/research/api.md

# Remove from shadow repository and delete local file
snps know file remove knowledge/research/api.md --delete-local
```

**How it works**:
- Copies file from project to target shadow repository
- Default target: first enabled project-context repository
- `--repo` and `--context` are mutually exclusive
- File must exist in project directory
- Preserves relative path in shadow repo

---

## Troubleshooting

### "Knowledge not initialized"

```bash
$ snps know sync
Error: Knowledge not initialized. Run 'snps know init' first.
```

**Solution**:
- In main repository: Run `snps know init` to create `.pmsynapse/repositories.yaml`
- In worktree: Run `snps know inherit` to link to parent's configuration

### "Not in a worktree" (inherit command)

```bash
$ snps know inherit
✗ Not in a worktree
  This command only works in git worktrees.
  Run 'snps know init' in the main repository instead.
```

**Solution**: The `inherit` command only works in git worktrees. Use `snps know init` in the main repository.

### "Parent repository not initialized"

```bash
$ snps know inherit
✓ Detected worktree
✗ Parent repository not initialized
  Run 'snps know init' in parent repository first:
  cd /path/to/parent
```

**Solution**: Initialize knowledge system in the parent repository first, then inherit in worktree.

### Shadow repo path doesn't exist

```bash
$ snps know sync
Error: Path does not exist: /Users/igor/team-knowledge
```

**Solution**:
1. Create the directory: `mkdir -p /Users/igor/team-knowledge`
2. Or remove the repo: `snps know repo remove team-knowledge`

### Files not syncing

**Check sync status**:
```bash
snps know status
```

**Common issues**:
- Repository disabled: Check `enabled: true` in `repositories.yaml`
- Excluded pattern: Check `sync_exclude_patterns`
- Identical content: Same hash = skipped

### Conflicts persist

**When conflicts appear repeatedly**:

```bash
# Force pull (overwrite local with shadow)
rm knowledge/conflicted-file.md
snps know sync --pull-only

# Or force push (overwrite shadow with local)
snps know sync --push-only --force
```

### Git tracking synced files

If `git status` shows knowledge files:

```bash
# Verify .git/info/exclude was updated
cat .git/info/exclude

# Manually re-sync to update exclude
snps know sync

# For worktrees, check main repo's exclude
cat ../main/.git/info/exclude  # Adjust path
```

### Performance issues

For large shadow repos (10k+ files):

1. **Disable unused repos**:
   ```yaml
   repositories:
     - path: /large/repo
       enabled: false  # Temporarily disable
   ```

2. **Add exclusions** (in `.pmsynapse/repositories.yaml`):
   ```yaml
   sync_exclude_patterns:
     - "(^|/)archive/"       # Skip archive/ directory
     - "(^|/)\\.venv/"       # Skip Python virtual envs
     - "(^|/)dist/"          # Skip build outputs
     - "\\.log$"             # Skip log files
   ```

3. **Sync specific contexts**:
   ```bash
   snps know sync --context project --apply
   ```

4. **Use verbose mode selectively**:
   ```bash
   # Default: hides SKIP operations (faster output)
   snps know sync

   # Verbose: shows all operations (slower output)
   snps know sync --verbose
   ```

---

## See Also

- **PRD**: `thoughts/shared/research/2025-12-20-unified-knowledge-management-prd.md` - Full system design
- **Implementation Plan**: `thoughts/shared/plans/2025-12-21-unified-knowledge-management-implementation.md` - Technical details
- **THOUGHTS_SYSTEM.md**: Legacy thoughts system documentation
- **IDLC_CONFIGURATION.md**: Workflow configuration (separate system)

---

## Summary

**Basic workflow**:
1. `snps know init` - Set up knowledge repos
2. `snps know sync` - Sync bidirectionally (interactive)
3. Edit files in `knowledge/`
4. `snps know sync --apply` - Push changes back (auto-apply)

**Key commands**:
- `snps know init` - Initialize knowledge system
- `snps know inherit` - Inherit configuration in worktree (worktrees only)
- `snps know sync` - Interactive sync (plan → prompt → apply)
- `snps know sync --apply` - Non-interactive sync (auto-apply)
- `snps know sync --dry-run` - Preview changes without applying
- `snps know sync --verbose` - Show all operations including skips
- `snps know sync --force` - Resolve conflicts by timestamp
- `snps know repo list` - See configured repos
- `snps know status` - Check sync state
- `snps know search` - Find knowledge

**Remember**:
- Sync is timestamp-based (newer wins automatically)
- Interactive by default (shows plan, prompts y/N/f)
- `--apply` for automation/scripts (no prompt)
- `--dry-run` for previews (no prompt, no changes)
- `--verbose` shows SKIP operations (hidden by default)
- Shadow repos managed externally (git operations outside `snps`)
- Project repo gets pushes, user/team are read-only
- `.git/info/exclude` auto-updated (keeps synced files out of git)
- Exclusion patterns configurable in `repositories.yaml`
