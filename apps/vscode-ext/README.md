# PMSynapse VS Code Extension

AI-enabled project management with knowledge graphs and IDLC workflow integration.

## Features

- **IDLC Items Panel**: View and manage Ideas, Features, Tasks, Decisions, Questions, and Assumptions
- **Knowledge Graph**: Visualize relationships between project artifacts
- **AI-Powered Suggestions**: Get intelligent recommendations from your LLM provider

## Quick Start

1. Open Command Palette (`Ctrl+Shift+P`)
2. Run `PMSynapse: Open Playground`
3. Start managing your project with AI assistance

## Commands

| Command | Description |
|---------|-------------|
| `PMSynapse: Open Playground` | Open the main PMSynapse interface |
| `PMSynapse: Create IDLC Item` | Create a new idea, feature, task, etc. |
| `PMSynapse: Show Knowledge Graph` | Visualize project relationships |

## Configuration

| Setting | Description | Default |
|---------|-------------|---------|
| `pmsynapse.llmProvider` | LLM provider (openai, anthropic, ollama) | `openai` |
| `pmsynapse.graphDatabasePath` | Path to knowledge graph DB | `.synapse/graph.db` |

## Development

See [VSCODE_EXTENSION_DEVELOPMENT.md](../../docs/VSCODE_EXTENSION_DEVELOPMENT.md) for development instructions.

```bash
# Install dependencies
pnpm install

# Start development (with watch mode)
pnpm dev

# Build for production
pnpm build

# Package as VSIX
pnpm package
```

## License

Apache-2.0
