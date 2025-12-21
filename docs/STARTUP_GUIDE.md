# PMSynapse Startup Guide

How to run the PMSynapse daemon and UI using the `snps` CLI.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Architecture Overview](#architecture-overview)
3. [Daemon Commands](#daemon-commands)
4. [UI Commands](#ui-commands)
5. [Development Mode](#development-mode)
6. [Profile Isolation](#profile-isolation)
7. [Environment Variables](#environment-variables)
8. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Production Use

```bash
# Start everything with one command
snps ui

# This will:
# 1. Auto-start the daemon if not running
# 2. Launch the desktop UI
# 3. Connect UI to daemon via Unix socket
```

### Development

```bash
# Start full development environment
snps dev

# This will:
# 1. Start daemon with 'dev' profile (isolated db/socket)
# 2. Launch Tauri in dev mode with hot reload
# 3. Set PMSYNAPSE_DEV_MODE=true
```

---

## Architecture Overview

PMSynapse uses a daemon + UI architecture similar to HumanLayer:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PMSYNAPSE ARCHITECTURE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                           DAEMON (snpsd)                             │   │
│  │                                                                       │   │
│  │  • REST API (http://127.0.0.1:7878)                                  │   │
│  │  • Unix Socket (~/.pmsynapse/daemon.sock)                            │   │
│  │  • SQLite Database (~/.pmsynapse/synapse.db)                         │   │
│  │  • Knowledge Graph Engine                                            │   │
│  │  • LLM Integration                                                   │   │
│  │  • Session Management                                                │   │
│  │                                                                       │   │
│  └────────────────────────────┬────────────────────────────────────────┘   │
│                               │                                             │
│              ┌────────────────┼────────────────┐                           │
│              │                │                │                           │
│              ▼                ▼                ▼                           │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐                  │
│  │  Desktop UI   │  │    CLI        │  │  AI Agents    │                  │
│  │  (Tauri)      │  │  (snps)       │  │  (MCP/API)    │                  │
│  └───────────────┘  └───────────────┘  └───────────────┘                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Communication

| Component | Connects Via | Purpose |
|-----------|--------------|---------|
| Desktop UI | Unix Socket | Real-time updates, graph queries |
| CLI | Unix Socket | Commands, status checks |
| AI Agents | REST API | Knowledge graph, proposals |
| External Tools | REST API | Integration, automation |

---

## Daemon Commands

### Start Daemon

```bash
# Start in background (default)
snps daemon start

# Start in foreground (for debugging)
snps daemon start --foreground

# Start with custom settings
snps daemon start --port 8080 --socket /tmp/snps.sock

# Start with profile isolation
snps daemon start --profile feature-x
```

**Options:**

| Option | Default | Description |
|--------|---------|-------------|
| `--foreground` | false | Run in foreground (don't daemonize) |
| `--socket` | `~/.pmsynapse/daemon.sock` | Unix socket path |
| `--port` | 7878 | HTTP API port (0 to disable) |
| `--db` | `~/.pmsynapse/synapse.db` | Database path |
| `--profile` | none | Profile name for isolation |

### Stop Daemon

```bash
# Graceful shutdown
snps daemon stop

# Force kill
snps daemon stop --force

# Stop specific profile
snps daemon stop --profile feature-x
```

### Check Status

```bash
# Quick status
snps daemon status

# Detailed status with paths
snps daemon status --detailed
```

**Output:**
```
PMSynapse Daemon Status

  ● Default daemon: running
  ○ Profile 'dev': stopped
  ● Profile 'feature-x': running

Paths:
  Config:   /home/user/.pmsynapse
  Socket:   /home/user/.pmsynapse/daemon.sock
  Database: /home/user/.pmsynapse/synapse.db
  Logs:     /home/user/.pmsynapse/logs/daemon.log
```

### Restart Daemon

```bash
# Restart default daemon
snps daemon restart

# Restart specific profile
snps daemon restart --profile dev
```

### View Logs

```bash
# Show last 50 lines
snps daemon logs

# Show last 100 lines
snps daemon logs -l 100

# Follow log output (like tail -f)
snps daemon logs -f

# View specific profile logs
snps daemon logs --profile dev
```

---

## UI Commands

### Launch Desktop UI

```bash
# Standard launch (auto-starts daemon)
snps ui

# Launch without starting daemon
snps ui --no-daemon

# Connect to specific daemon socket
snps ui --daemon-socket /tmp/custom.sock

# Launch in background
snps ui --detach
```

**Options:**

| Option | Description |
|--------|-------------|
| `--no-daemon` | Don't auto-start daemon |
| `--daemon-socket` | Custom socket path |
| `--detach` | Run in background |

### What Happens on `snps ui`

1. **Daemon Check**: Verifies daemon is running
2. **Auto-Start**: Starts daemon if not running (unless `--no-daemon`)
3. **Environment Setup**: Sets `PMSYNAPSE_DAEMON_SOCKET`
4. **Tauri Launch**: Runs `pnpm tauri dev` in desktop app directory

---

## Development Mode

### Full Stack Development

```bash
# Start with default 'dev' profile
snps dev

# Custom profile (useful for feature branches)
snps dev --profile my-feature

# Custom HTTP port
snps dev --port 9000
```

### Daemon-Only Development

```bash
# Just run the daemon (for backend development)
snps dev --daemon-only
```

### UI-Only Development

```bash
# Just run UI (assumes daemon already running)
snps dev --ui-only
```

### Development Environment Features

| Feature | Description |
|---------|-------------|
| **Hot Reload** | Vite HMR for React, Tauri watch for Rust |
| **Profile Isolation** | Separate database per profile |
| **Dev Mode Flag** | `PMSYNAPSE_DEV_MODE=true` environment |
| **Debug Logging** | Verbose output for troubleshooting |

### Typical Development Workflow

```bash
# Terminal 1: Start full dev environment
snps dev --profile my-feature

# Terminal 2: Run tests while developing
cargo test -p snps-core --watch

# Terminal 3: Check daemon status
snps daemon status --detailed
```

---

## Profile Isolation

Profiles allow running multiple isolated instances of PMSynapse simultaneously.

### Use Cases

- **Feature Development**: Each feature branch gets its own database
- **Testing**: Isolated test environment
- **Multiple Projects**: Separate instances per project

### How Profiles Work

```
~/.pmsynapse/
├── daemon.sock          # Default daemon socket
├── synapse.db           # Default database
├── daemon-dev.sock      # Dev profile socket
├── synapse-dev.db       # Dev profile database
├── daemon-feature-x.sock
├── synapse-feature-x.db
└── logs/
    ├── daemon.log
    ├── daemon-dev.log
    └── daemon-feature-x.log
```

### Profile Commands

```bash
# Start daemon with profile
snps daemon start --profile feature-x

# Check all profiles
snps daemon status

# Stop specific profile
snps daemon stop --profile feature-x

# Dev mode with profile
snps dev --profile feature-x

# UI connecting to profile
snps ui --daemon-socket ~/.pmsynapse/daemon-feature-x.sock
```

---

## Environment Variables

### Daemon Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `PMSYNAPSE_DAEMON_SOCKET` | `~/.pmsynapse/daemon.sock` | Unix socket path |
| `PMSYNAPSE_DAEMON_HTTP_PORT` | `7878` | HTTP API port |
| `PMSYNAPSE_DATABASE_PATH` | `~/.pmsynapse/synapse.db` | Database file |
| `PMSYNAPSE_LOG_LEVEL` | `info` | Log level (debug, info, warn, error) |

### UI Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `PMSYNAPSE_DAEMON_SOCKET` | `~/.pmsynapse/daemon.sock` | Daemon to connect to |
| `PMSYNAPSE_DEV_MODE` | `false` | Enable development features |
| `PMSYNAPSE_PROFILE` | none | Current profile name |

### Example: Custom Configuration

```bash
# Custom daemon setup
export PMSYNAPSE_DAEMON_HTTP_PORT=9000
export PMSYNAPSE_DATABASE_PATH=/data/pmsynapse/main.db
snps daemon start

# Custom UI connection
export PMSYNAPSE_DAEMON_SOCKET=/tmp/pmsynapse.sock
snps ui --no-daemon
```

---

## Troubleshooting

### Daemon Won't Start

```bash
# Check if already running
snps daemon status

# Check for stale PID file
ls -la ~/.pmsynapse/daemon*.pid

# Remove stale files
rm ~/.pmsynapse/daemon.pid ~/.pmsynapse/daemon.sock

# Try foreground mode for debugging
snps daemon start --foreground
```

### UI Can't Connect to Daemon

```bash
# Verify daemon is running
snps daemon status --detailed

# Check socket exists
ls -la ~/.pmsynapse/daemon.sock

# Check socket permissions
stat ~/.pmsynapse/daemon.sock

# Try explicit socket path
snps ui --daemon-socket ~/.pmsynapse/daemon.sock
```

### Port Already in Use

```bash
# Check what's using the port
lsof -i :7878

# Use different port
snps daemon start --port 7879
```

### Database Issues

```bash
# Check database file
ls -la ~/.pmsynapse/synapse.db

# Backup and recreate
mv ~/.pmsynapse/synapse.db ~/.pmsynapse/synapse.db.backup
snps daemon start
```

### Development Issues

```bash
# Clean build and restart
cd apps/desktop
pnpm clean
pnpm install
snps dev

# Check Tauri prerequisites
pnpm tauri info
```

---

## File Locations

### Default Paths

| File | Path | Purpose |
|------|------|---------|
| Config | `~/.pmsynapse/` | Root configuration directory |
| Database | `~/.pmsynapse/synapse.db` | SQLite knowledge graph |
| Socket | `~/.pmsynapse/daemon.sock` | Unix domain socket |
| PID | `~/.pmsynapse/daemon.pid` | Daemon process ID |
| Logs | `~/.pmsynapse/logs/` | Log files |
| Thoughts | `~/.pmsynapse/thoughts/` | Thought documents |

### Project Paths

| File | Path | Purpose |
|------|------|---------|
| Project Config | `.pmsynapse/config.yaml` | Project-specific settings |
| Team Config | `.pmsynapse/teams/*/idlc.yaml` | IDLC workflows |
| Thoughts Link | `thoughts/` → `~/.pmsynapse/...` | Symlink to thoughts |

---

## Quick Reference

```bash
# === DAEMON ===
snps daemon start              # Start daemon
snps daemon start --foreground # Start in foreground
snps daemon stop               # Stop daemon
snps daemon status             # Check status
snps daemon restart            # Restart daemon
snps daemon logs -f            # Follow logs

# === UI ===
snps ui                        # Launch UI (auto-starts daemon)
snps ui --no-daemon            # Launch without daemon
snps ui --detach               # Launch in background

# === DEVELOPMENT ===
snps dev                       # Full dev environment
snps dev --daemon-only         # Just daemon
snps dev --ui-only             # Just UI
snps dev --profile feature-x   # Isolated profile

# === PROFILES ===
snps daemon start --profile X  # Start with profile
snps daemon status             # See all profiles
snps daemon stop --profile X   # Stop profile
```

---

*Part of PMSynapse AI-Enabled Knowledge Management*
