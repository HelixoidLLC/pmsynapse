# Thoughts CLI Reference

Technical reference for all `snps thoughts` commands.

---

## Commands Overview

| Command | Purpose |
|---------|---------|
| `init` | Initialize thoughts for a project |
| `new` | Create a new thought document |
| `search` | Search thought contents |
| `list` | List thought documents |
| `sync` | Sync with central repository |
| `status` | Show configuration and state |
| `open` | Open thoughts in editor/file manager |
| `profile` | Manage thought profiles |
| `hooks` | Manage git hooks |

---

## snps thoughts init

Initialize thoughts for the current project.

```bash
snps thoughts init [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--profile <NAME>` | Use specific profile | `default` |
| `--storage <TYPE>` | Storage type: `local`, `remote`, `hybrid` | `local` |
| `--remote <URL>` | Git remote for syncing | none |
| `--no-hooks` | Skip git hook installation | `false` |
| `-f, --force` | Force overwrite existing setup | `false` |

### Examples

```bash
# Basic initialization
snps thoughts init

# With specific profile
snps thoughts init --profile work

# Force reinitialize
snps thoughts init --force
```

### Created Structure

```
thoughts/                  # Symlink to central location
├── shared/               # Team documents
│   ├── research/
│   ├── plans/
│   ├── tickets/
│   └── prs/
├── [username]/           # Personal documents
├── global/               # Cross-project patterns
└── searchable/           # AI search index (auto-generated)
```

---

## snps thoughts new

Create a new thought document.

```bash
snps thoughts new <TYPE> <TITLE> [OPTIONS]
```

### Types

| Type | Directory | Purpose |
|------|-----------|---------|
| `research` | `research/` | Investigation findings |
| `plan` | `plans/` | Implementation plans |
| `ticket` | `tickets/` | Ticket context |
| `pr` | `prs/` | PR descriptions |
| `scratch` | root | Quick notes |
| `journal` | `journal/` | Daily logs |

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--scope <SCOPE>` | `shared`, `personal`, `global` | `shared` |
| `--open` | Open in editor after creation | `false` |

### Examples

```bash
# Shared research (default)
snps thoughts new research "API Analysis"

# Personal scratch note
snps thoughts new scratch "Ideas" --scope personal

# Global pattern
snps thoughts new research "Error Pattern" --scope global

# Open immediately
snps thoughts new plan "Feature X" --open
```

### Generated Filenames

Format: `YYYY-MM-DD-slugified-title.md`

Example: `2025-12-13-api-analysis.md`

---

## snps thoughts search

Search through thought documents.

```bash
snps thoughts search <QUERY> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--scope <SCOPE>` | `all`, `shared`, `personal`, `global` | `all` |
| `--doc-type <TYPE>` | Filter by document type | none |
| `--paths-only` | Return only file paths | `false` |
| `--limit <N>` | Limit results | `10` |

### Examples

```bash
# Basic search
snps thoughts search "authentication"

# Search only plans
snps thoughts search "oauth" --doc-type plan

# Get paths only (for AI agents)
snps thoughts search "webhook" --paths-only

# Search personal notes
snps thoughts search "todo" --scope personal
```

---

## snps thoughts list

List thought documents.

```bash
snps thoughts list [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--scope <SCOPE>` | Filter by scope | all |
| `--doc-type <TYPE>` | Filter by type | all |
| `--recent <N>` | Show N most recent | all |
| `--format <FMT>` | `table`, `json`, `paths` | `table` |

### Examples

```bash
# List all
snps thoughts list

# Recent 5 documents
snps thoughts list --recent 5

# Only plans as JSON
snps thoughts list --doc-type plan --format json

# Paths only (for scripting)
snps thoughts list --format paths
```

---

## snps thoughts sync

Sync thoughts with central repository.

```bash
snps thoughts sync [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-m, --message <MSG>` | Commit message | auto-generated |
| `--push` | Push to remote after commit | `false` |
| `--pull` | Pull from remote before commit | `false` |
| `--no-commit` | Only rebuild searchable index | `false` |
| `--direction <DIR>` | `both`, `to-central`, `from-central` | `both` |

### Examples

```bash
# Basic sync (commit locally, rebuild index)
snps thoughts sync

# With message
snps thoughts sync -m "Updated research on auth"

# Full bidirectional sync
snps thoughts sync --pull --push

# Only rebuild search index
snps thoughts sync --no-commit

# Pull only
snps thoughts sync --direction from-central --pull
```

### What Sync Does

1. Optionally pulls from remote (if `--pull`)
2. Rebuilds `searchable/` directory with path-encoded hardlinks
3. Commits changes to central repo (unless `--no-commit`)
4. Optionally pushes to remote (if `--push`)

---

## snps thoughts status

Show thoughts configuration and state.

```bash
snps thoughts status [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-v, --verbose` | Show file counts per directory | `false` |

### Output Includes

- Initialization status
- Symlink paths (project → central)
- Git repository status
- Remote configuration
- Uncommitted changes count
- Directory structure
- Searchable index status
- Git hooks status

### Examples

```bash
# Basic status
snps thoughts status

# Detailed with file counts
snps thoughts status --verbose
```

---

## snps thoughts open

Open thoughts directory or file.

```bash
snps thoughts open [PATH] [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--editor` | Open in `$EDITOR` | `false` |
| `--scope <SCOPE>` | Open specific scope directory | none |

### Examples

```bash
# Open in file manager
snps thoughts open

# Open in editor
snps thoughts open --editor

# Open specific scope
snps thoughts open --scope personal

# Open specific file
snps thoughts open shared/research/api-analysis.md --editor
```

---

## snps thoughts profile

Manage thought profiles.

```bash
snps thoughts profile <COMMAND>
```

### Subcommands

| Command | Description |
|---------|-------------|
| `list` | List available profiles |
| `create <NAME>` | Create new profile |
| `switch <NAME>` | Switch active profile |
| `delete <NAME>` | Delete a profile |
| `show` | Show current profile details |

### Create Options

| Option | Description |
|--------|-------------|
| `--repo <PATH>` | Local repository path |
| `--remote <URL>` | Git remote URL |
| `--clone` | Clone from remote |

### Examples

```bash
# List profiles
snps thoughts profile list

# Create work profile
snps thoughts profile create work --remote git@github.com:company/thoughts.git --clone

# Switch to work profile
snps thoughts profile switch work

# Show current
snps thoughts profile show
```

---

## snps thoughts hooks

Manage git hooks for thoughts protection.

```bash
snps thoughts hooks <COMMAND>
```

### Subcommands

| Command | Description |
|---------|-------------|
| `install` | Install git hooks |
| `uninstall` | Remove git hooks |
| `status` | Check hook status |

### Install Options

| Option | Description | Default |
|--------|-------------|---------|
| `--no-pre-commit` | Skip pre-commit hook | `false` |
| `--no-post-commit` | Skip post-commit hook | `false` |
| `--auto-sync` | Enable auto-sync on commit | `false` |
| `-f, --force` | Force overwrite existing hooks | `false` |

### Examples

```bash
# Install default hooks
snps thoughts hooks install

# Force reinstall
snps thoughts hooks install --force

# Install with auto-sync
snps thoughts hooks install --auto-sync

# Check status
snps thoughts hooks status

# Remove hooks
snps thoughts hooks uninstall
```

### Pre-commit Hook

Prevents accidentally committing `thoughts/` to the code repository:

```bash
# Blocked by hook:
git add thoughts/
git commit -m "oops"  # ❌ Blocked

# Bypass if needed:
git commit --no-verify -m "intentional"
```

### Post-commit Hook (with --auto-sync)

Automatically rebuilds the searchable index after each commit.

---

## Document Types Reference

| Type | Scope | Directory | Filename Pattern |
|------|-------|-----------|-----------------|
| `research` | shared | `shared/research/` | `YYYY-MM-DD-title.md` |
| `plan` | shared | `shared/plans/` | `YYYY-MM-DD-title.md` |
| `ticket` | shared | `shared/tickets/` | `YYYY-MM-DD-title.md` |
| `pr` | shared | `shared/prs/` | `YYYY-MM-DD-title.md` |
| `scratch` | personal | `[user]/` | `scratch-YYYY-MM-DD-title.md` |
| `journal` | personal | `[user]/journal/` | `YYYY-MM-DD.md` |

---

## Searchable Index

The `thoughts/searchable/` directory contains path-encoded hardlinks:

| Original Path | Searchable Path |
|---------------|-----------------|
| `shared/research/api.md` | `searchable/shared-research-api.md` |
| `user/tickets/eng-123.md` | `searchable/user-tickets-eng-123.md` |
| `global/patterns/errors.md` | `searchable/global-patterns-errors.md` |

**Purpose**: Enables AI tools that don't follow symlinks to search all documents.

**Rebuild**: Run `snps thoughts sync --no-commit`

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `EDITOR` | Editor for `--open` flag | `code` |
| `USER` / `USERNAME` | Personal directory name | system user |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Not initialized (run `snps thoughts init`) |

---

*Part of PMSynapse AI-Enabled Project Management*
