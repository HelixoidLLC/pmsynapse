# PMSynapse

AI-enabled project management with knowledge graphs and configurable workflows.

## Features

- **IDLC Workflows** - Idea Development Lifecycle management per team

- **Knowledge Graph** - Track relationships between issues, research, plans, and code

- **Thoughts System** - Document-based knowledge management with searchable index

- **Claude Integration** - Import and analyze Claude Code sessions

- **Multi-provider LLM** - OpenAI, Anthropic, and local model support

- **Desktop & CLI** - Cross-platform Tauri app + powerful CLI tool

## Tutorials

- **[Knowledge System Tutorial](docs/user-guide/KNOWLEDGE_SYSTEM_TUTORIAL.md)** - Complete guide to `snps know` - unified knowledge management with shadow repositories, bidirectional sync, and multi-level scoping

## Installation

### Quick Install

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/scripts/install.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
irm https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/scripts/install.ps1 | iex
```

### Manual Installation

Download the latest release for your platform:

| Platform | Download |
|----------|----------|
| Linux x64 | [snps-linux-amd64.tar.gz](https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-linux-amd64.tar.gz) |
| Linux ARM64 | [snps-linux-arm64.tar.gz](https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-linux-arm64.tar.gz) |
| macOS Intel | [snps-macos-amd64.tar.gz](https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-macos-amd64.tar.gz) |
| macOS Apple Silicon | [snps-macos-arm64.tar.gz](https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-macos-arm64.tar.gz) |
| Windows x64 | [snps-windows-amd64.exe.zip](https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-windows-amd64.exe.zip) |

**Extract and install:**

```bash
# Linux/macOS
tar -xzf snps-*.tar.gz
sudo mv snps /usr/local/bin/
snps --version

# Windows
# Extract ZIP and move snps.exe to a folder in your PATH
```

## Getting Started

```bash
# Initialize thoughts system
snps thoughts init
```

### Common Commands

```bash
# Thoughts management
snps thoughts new research "Topic"
snps thoughts search "query"
snps thoughts list --recent 10
```

## Configuration

PMSynapse stores configuration in \`~/.pmsynapse/config.yaml\`:

```yaml
# LLM Provider settings
llm:
  provider: anthropic  # or openai, local
  api_key: \${ANTHROPIC_API_KEY}
  model: claude-3-5-sonnet-20241022

# Database location
database:
  path: ~/.pmsynapse/pmsynapse.db

# IDLC workflow defaults
idlc:
  default_workflow: standard
```

## System Requirements

- **Linux:** x86_64 or ARM64, glibc 2.31+
- **macOS:** 10.13+ (High Sierra or later), Intel or Apple Silicon
- **Windows:** Windows 10/11, x86_64

## Verification

Verify checksums after download:

```bash
# Download checksum file
curl -L "https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-linux-amd64.tar.gz.sha256" \\
  -o snps.tar.gz.sha256

# Verify
sha256sum -c snps.tar.gz.sha256
```

## Documentation

- [Installation Guide](https://github.com/HelixoidLLC/pmsynapse#installation)
- [Quick Start Tutorial](#getting-started)
- [Configuration Reference](#configuration)

## Support

For support, open an issue on GitHub.

## License

Copyright © 2024-2025 Helixoid LLC. All rights reserved.

Licensed under the Apache License, Version 2.0.

## Updates

PMSynapse CLI updates are available through GitHub releases. Check for updates:

```bash
# Check current version
snps --version

# The CLI will automatically check for updates daily
# Or force check: snps update
```
