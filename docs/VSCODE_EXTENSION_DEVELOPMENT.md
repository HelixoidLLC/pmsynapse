# VS Code Extension Development Guide

This guide covers how to develop, debug, and test the PMSynapse VS Code extension with continuous hot reload.

## Prerequisites

- **Node.js** 18+ and **pnpm** 8+
- **VS Code** 1.85.0 or later
- **Rust** toolchain (for core library integration)

## Quick Start

```bash
# 1. Install dependencies
pnpm install

# 2. Build the RPC package (required first)
pnpm --filter @pmsynapse/rpc build

# 3. Start development
pnpm dev:vscode
```

## Project Structure

```
apps/vscode-ext/
├── src/
│   ├── extension.ts          # Extension entry point
│   ├── panels/
│   │   └── WebviewPanelHost.ts   # Webview panel manager
│   ├── commands/
│   │   └── index.ts          # Command registrations
│   └── utils/
│       ├── getUri.ts         # Resource URI helper
│       └── getNonce.ts       # CSP nonce generator
├── webview-ui/               # React UI (symlink or copy from desktop)
├── dist/                     # Compiled extension output
├── package.json              # Extension manifest
├── tsconfig.json
└── tsup.config.ts
```

## Development Modes

### Mode 1: Extension + Vite Dev Server (Recommended)

This mode provides the fastest development cycle with hot reload for both the extension and UI.

**Terminal 1 - Start Vite dev server:**
```bash
cd apps/desktop
pnpm dev
# Runs on http://localhost:3030
```

**Terminal 2 - Watch extension code:**
```bash
cd apps/vscode-ext
pnpm dev
# Watches and rebuilds on changes
```

**Terminal 3 - Launch VS Code Extension Host:**
1. Open VS Code in the `apps/vscode-ext` directory
2. Press `F5` or go to **Run > Start Debugging**
3. Select **"Extension Development Host"**

The extension will load the UI from the Vite dev server, giving you:
- Instant UI updates (React Fast Refresh)
- Extension code updates on rebuild + reload (`Ctrl+R` in Extension Host)

### Mode 2: Full Extension Debug

For debugging extension-specific code:

```bash
# Build everything first
pnpm build

# Then launch via VS Code debugger (F5)
```

## Setting Up VS Code for Development

### 1. Create Launch Configuration

Create `.vscode/launch.json` in `apps/vscode-ext/`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Run Extension",
      "type": "extensionHost",
      "request": "launch",
      "args": [
        "--extensionDevelopmentPath=${workspaceFolder}"
      ],
      "outFiles": [
        "${workspaceFolder}/dist/**/*.js"
      ],
      "preLaunchTask": "npm: dev",
      "env": {
        "VSCODE_DEBUG_MODE": "true"
      }
    },
    {
      "name": "Run Extension (Production)",
      "type": "extensionHost",
      "request": "launch",
      "args": [
        "--extensionDevelopmentPath=${workspaceFolder}"
      ],
      "outFiles": [
        "${workspaceFolder}/dist/**/*.js"
      ],
      "preLaunchTask": "npm: build"
    }
  ]
}
```

### 2. Create Tasks Configuration

Create `.vscode/tasks.json` in `apps/vscode-ext/`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "npm",
      "script": "dev",
      "problemMatcher": "$tsc-watch",
      "isBackground": true,
      "presentation": {
        "reveal": "never"
      },
      "group": {
        "kind": "build",
        "isDefault": true
      }
    },
    {
      "type": "npm",
      "script": "build",
      "problemMatcher": "$tsc",
      "presentation": {
        "reveal": "silent"
      }
    }
  ]
}
```

### 3. Recommended VS Code Settings

Create `.vscode/settings.json` in `apps/vscode-ext/`:

```json
{
  "typescript.tsdk": "node_modules/typescript/lib",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "explicit"
  }
}
```

## Development Workflow

### Daily Development Cycle

```bash
# Start everything in watch mode
pnpm dev:all
```

Or manually in separate terminals:

```bash
# Terminal 1: RPC package (if modifying shared types)
cd packages/rpc && pnpm dev

# Terminal 2: Vite dev server for UI
cd apps/desktop && pnpm dev

# Terminal 3: Extension watcher
cd apps/vscode-ext && pnpm dev

# Terminal 4: VS Code Extension Host (or use F5)
code apps/vscode-ext && # Press F5
```

### Making Changes

| Change Type | Action Required |
|-------------|-----------------|
| UI components (React) | Auto-refresh via Vite HMR |
| Extension code (TypeScript) | Rebuild triggers, then `Ctrl+R` in Extension Host |
| RPC types | Rebuild RPC package, restart watchers |
| package.json (extension manifest) | Restart Extension Host |

### Reloading the Extension

In the Extension Development Host window:
- **Quick reload**: `Ctrl+R` (Windows/Linux) or `Cmd+R` (macOS)
- **Full restart**: Close and re-launch with `F5`
- **Reload Window**: `Ctrl+Shift+P` → "Developer: Reload Window"

## Adding Scripts to package.json

Add these scripts to `apps/vscode-ext/package.json`:

```json
{
  "scripts": {
    "dev": "tsup --watch",
    "build": "tsup",
    "package": "vsce package --no-dependencies",
    "publish": "vsce publish --no-dependencies",
    "typecheck": "tsc --noEmit"
  }
}
```

And to root `package.json`:

```json
{
  "scripts": {
    "dev:vscode": "pnpm --filter pmsynapse-vscode dev",
    "dev:ui": "pnpm --filter @pmsynapse/desktop dev",
    "dev:all": "concurrently \"pnpm dev:ui\" \"pnpm dev:vscode\"",
    "build:vscode": "pnpm --filter pmsynapse-vscode build"
  }
}
```

## Testing the Extension

### Manual Testing

1. Launch Extension Host (`F5`)
2. Open Command Palette (`Ctrl+Shift+P`)
3. Run "PMSynapse: Open Playground"
4. Verify the webview loads correctly

### Automated Testing

```bash
cd apps/vscode-ext
pnpm test
```

Create test files in `apps/vscode-ext/src/test/`:

```typescript
// extension.test.ts
import * as assert from 'assert';
import * as vscode from 'vscode';

suite('Extension Test Suite', () => {
  test('Extension should activate', async () => {
    const ext = vscode.extensions.getExtension('pmsynapse.pmsynapse');
    assert.ok(ext);
    await ext.activate();
    assert.strictEqual(ext.isActive, true);
  });
});
```

## Debugging Tips

### Debug Extension Code

1. Set breakpoints in `apps/vscode-ext/src/**/*.ts`
2. Launch with `F5`
3. Trigger the code path (run a command, open a panel)
4. VS Code will pause at breakpoints

### Debug Webview Content

1. In Extension Host, open the webview
2. Open Developer Tools: `Ctrl+Shift+P` → "Developer: Open Webview Developer Tools"
3. Use Chrome DevTools to inspect/debug the React app

### View Extension Logs

```typescript
// In extension code
console.log('Debug message'); // Goes to Debug Console

// Or use output channel
const outputChannel = vscode.window.createOutputChannel('PMSynapse');
outputChannel.appendLine('Debug message');
outputChannel.show();
```

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `VSCODE_DEBUG_MODE` | Enable dev server loading | `false` |
| `PMSYNAPSE_DEV_PORT` | Vite dev server port | `3030` |

Set in launch configuration or `.env`:

```json
{
  "env": {
    "VSCODE_DEBUG_MODE": "true",
    "PMSYNAPSE_DEV_PORT": "3030"
  }
}
```

## Building for Production

### Build Extension

```bash
cd apps/vscode-ext

# Build the extension
pnpm build

# Build webview UI (copy from desktop build)
cd ../desktop && pnpm build
cp -r dist ../vscode-ext/webview-ui/dist
```

### Package as VSIX

```bash
# Install vsce if needed
pnpm add -g @vscode/vsce

# Package
cd apps/vscode-ext
vsce package --no-dependencies

# Output: pmsynapse-0.1.0.vsix
```

### Install VSIX Locally

```bash
code --install-extension pmsynapse-0.1.0.vsix
```

Or in VS Code:
1. `Ctrl+Shift+P` → "Extensions: Install from VSIX..."
2. Select the `.vsix` file

## Sharing UI Between Tauri and VS Code

The UI is shared via the `packages/rpc` package and common React components:

```
packages/
├── rpc/              # Shared RPC types and utilities
└── ui/               # Shared React components (future)

apps/
├── desktop/src/      # Tauri frontend (React)
└── vscode-ext/
    └── webview-ui/   # VS Code webview (same React code)
```

### Approach 1: Symlink (Development)

```bash
cd apps/vscode-ext
ln -s ../desktop/src webview-ui
```

### Approach 2: Workspace Package (Recommended)

Create `packages/ui/` with shared components, then import in both apps:

```typescript
// In both desktop and vscode-ext
import { Dashboard, KnowledgeGraph } from '@pmsynapse/ui';
```

## Troubleshooting

### Extension Not Loading

1. Check Output panel → "Extension Host"
2. Verify `dist/extension.js` exists
3. Check for TypeScript errors: `pnpm typecheck`

### Webview Shows Blank

1. Check if Vite dev server is running (dev mode)
2. Verify webview-ui/dist exists (prod mode)
3. Open Webview DevTools to see errors

### Hot Reload Not Working

1. Ensure `VSCODE_DEBUG_MODE=true` is set
2. Check Vite dev server is running on correct port
3. Reload the Extension Host window

### Changes Not Reflected

1. Rebuild: `pnpm build`
2. Reload Extension Host: `Ctrl+R`
3. If still stuck, restart VS Code completely

## Next Steps

- [ ] Set up webview-ui with shared React components
- [ ] Add unit tests for extension commands
- [ ] Configure CI/CD for VSIX publishing
- [ ] Add telemetry for usage analytics
