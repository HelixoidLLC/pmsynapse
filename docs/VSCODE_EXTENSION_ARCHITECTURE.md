# VS Code Extension + Tauri Dual Deployment Architecture

This document describes how PMSynapse UI can run both as a VS Code extension webview and as a Tauri desktop application, sharing the same React codebase.

## Architecture Overview

```
apps/
├── desktop/              ← Tauri Desktop App (existing)
│   ├── src/              ← React UI source
│   └── src-tauri/        ← Rust backend
├── vscode-ext/           ← VS Code Extension (NEW)
│   ├── src/              ← Extension TypeScript code
│   └── dist/             ← Built extension + webview assets
└── playground/           ← Shared React UI (NEW - extract from desktop)
    ├── src/              ← React components
    └── dist/             ← Built for webview consumption

packages/
├── ui/                   ← Shared UI components (Radix + Tailwind)
├── rpc/                  ← Shared RPC type definitions
└── common/               ← Shared utilities
```

## Key Patterns from BAML

### 1. Webview Content Loading Strategy

**Development Mode:**
- React app runs on Vite dev server (port 3030)
- VS Code webview loads from `http://localhost:3030`
- Hot Module Replacement (HMR) works in webview
- Enabled via `VSCODE_DEBUG_MODE` environment variable

**Production Mode:**
- React app is pre-built and bundled with extension
- Webview loads from `dist/playground/dist/assets/`
- Content Security Policy (CSP) with nonces for security

### 2. Environment Detection in WebviewPanelHost

```typescript
// src/panels/WebviewPanelHost.ts
class WebviewPanelHost {
  private static _getWebviewContent(webview: Webview, extensionUri: Uri): string {
    const isDev = process.env.VSCODE_DEBUG_MODE === 'true';

    if (isDev) {
      // Development: Load from Vite dev server
      return `
        <!DOCTYPE html>
        <html>
          <head>
            <script type="module" src="http://localhost:3030/@vite/client"></script>
          </head>
          <body>
            <div id="root"></div>
            <script type="module" src="http://localhost:3030/src/main.tsx"></script>
          </body>
        </html>
      `;
    } else {
      // Production: Load bundled assets
      const scriptUri = getUri(webview, extensionUri, ['dist', 'playground', 'index.js']);
      const styleUri = getUri(webview, extensionUri, ['dist', 'playground', 'index.css']);
      const nonce = getNonce();

      return `
        <!DOCTYPE html>
        <html>
          <head>
            <meta http-equiv="Content-Security-Policy" content="
              default-src 'none';
              script-src 'nonce-${nonce}';
              style-src ${webview.cspSource} 'unsafe-inline';
            ">
            <link rel="stylesheet" href="${styleUri}">
          </head>
          <body>
            <div id="root"></div>
            <script nonce="${nonce}" src="${scriptUri}"></script>
          </body>
        </html>
      `;
    }
  }
}
```

### 3. RPC Communication Protocol

**Webview → Extension (requests):**
```typescript
// packages/rpc/src/webview-to-vscode.ts
export type WebviewToVSCodeCommands = {
  // File operations
  GET_FILE_CONTENT: { path: string } → { content: string };
  JUMP_TO_FILE: { path: string; line: number } → void;

  // IDLC operations
  GET_IDLC_CONFIG: void → IdlcConfig;
  CREATE_IDLC_ITEM: { title: string } → IdlcItem;
  TRANSITION_ITEM: { id: string; to: string } → void;

  // Graph operations
  QUERY_GRAPH: { query: string } → Node[];
  ADD_NODE: { node: Node } → { id: string };
};
```

**Extension → Webview (push updates):**
```typescript
// packages/rpc/src/vscode-to-webview.ts
export type VSCodeToWebviewCommands = {
  // IDE state
  CURSOR_POSITION_CHANGED: { file: string; line: number; column: number };
  ACTIVE_FILE_CHANGED: { path: string };

  // Data updates
  IDLC_ITEM_UPDATED: IdlcItem;
  GRAPH_NODE_ADDED: Node;

  // Settings
  SETTINGS_CHANGED: Settings;
};
```

### 4. Message Queueing Pattern

```typescript
// Handle race condition: extension sends message before webview ready
class WebviewPanelHost {
  private messageQueue: any[] = [];
  private isInitialized = false;

  postMessage(message: any) {
    if (this.isInitialized) {
      this._panel.webview.postMessage(message);
    } else {
      this.messageQueue.push(message);
    }
  }

  private _setWebviewMessageListener() {
    this._panel.webview.onDidReceiveMessage((msg) => {
      if (msg.command === 'INITIALIZED') {
        this.isInitialized = true;
        // Flush queued messages
        this.messageQueue.forEach(m => this._panel.webview.postMessage(m));
        this.messageQueue = [];
      }
    });
  }
}
```

### 5. Shared UI Components

Both Tauri and VS Code webview consume the same React components:

```typescript
// packages/ui/src/index.ts
export { Button } from './components/button';
export { Card } from './components/card';
export { Sidebar } from './components/sidebar';
export { KnowledgeGraph } from './components/knowledge-graph';
export { IdlcBoard } from './components/idlc-board';

// apps/desktop/src/App.tsx (Tauri)
import { Button, Sidebar } from '@pmsynapse/ui';

// apps/playground/src/App.tsx (VS Code webview)
import { Button, Sidebar } from '@pmsynapse/ui';
```

## Implementation Plan

### Phase 1: Extract Shared UI Package
1. Create `packages/ui/` with shared components
2. Move common components from `apps/desktop/src/components/`
3. Configure Tailwind CSS for package

### Phase 2: Create Playground App
1. Create `apps/playground/` as standalone Vite app
2. Configure for webview compatibility (CSP, HMR)
3. Add Tauri-like API bridge for VS Code

### Phase 3: Create VS Code Extension
1. Create `apps/vscode-ext/` structure
2. Implement WebviewPanelHost
3. Add RPC communication layer
4. Bundle playground assets

### Phase 4: Shared Backend Integration
1. Create `packages/rpc/` for type definitions
2. Implement Tauri commands → VS Code RPC adapter
3. Ensure feature parity between platforms

## Directory Structure (Target)

```
pmsynapse/
├── apps/
│   ├── desktop/                 # Tauri app
│   │   ├── src/                 # React (imports from @pmsynapse/ui)
│   │   ├── src-tauri/           # Rust backend
│   │   └── package.json
│   ├── playground/              # Shared webview React app
│   │   ├── src/
│   │   │   ├── main.tsx
│   │   │   ├── App.tsx
│   │   │   └── vscode-api.ts    # VS Code API bridge
│   │   ├── vite.config.ts
│   │   └── package.json
│   └── vscode-ext/              # VS Code extension
│       ├── src/
│       │   ├── extension.ts     # Activation entry
│       │   ├── panels/
│       │   │   └── WebviewPanelHost.ts
│       │   ├── utils/
│       │   │   ├── getUri.ts
│       │   │   └── getNonce.ts
│       │   └── commands/
│       │       └── index.ts
│       ├── dist/
│       │   ├── extension.js     # Built extension
│       │   └── playground/      # Built webview assets
│       ├── package.json
│       └── tsup.config.ts
├── packages/
│   ├── ui/                      # Shared UI components
│   │   ├── src/
│   │   │   ├── components/
│   │   │   └── index.ts
│   │   ├── tailwind.config.js
│   │   └── package.json
│   ├── rpc/                     # Shared RPC types
│   │   ├── src/
│   │   │   ├── webview-to-vscode.ts
│   │   │   └── vscode-to-webview.ts
│   │   └── package.json
│   └── common/                  # Shared utilities
│       └── package.json
├── crates/                      # Rust crates (existing)
├── pnpm-workspace.yaml
└── turbo.json
```

## VS Code Extension package.json

```json
{
  "name": "pmsynapse",
  "displayName": "PMSynapse",
  "description": "AI-enabled project management with knowledge graphs",
  "version": "0.1.0",
  "publisher": "HelixoidLLC",
  "engines": {
    "vscode": "^1.85.0"
  },
  "categories": ["Other"],
  "activationEvents": [
    "onStartupFinished"
  ],
  "main": "./dist/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "pmsynapse.openPlayground",
        "title": "PMSynapse: Open Playground"
      },
      {
        "command": "pmsynapse.createIdlcItem",
        "title": "PMSynapse: Create IDLC Item"
      }
    ],
    "viewsContainers": {
      "activitybar": [
        {
          "id": "pmsynapse",
          "title": "PMSynapse",
          "icon": "resources/icon.svg"
        }
      ]
    }
  },
  "scripts": {
    "dev": "tsup --watch",
    "build": "tsup && pnpm --filter @pmsynapse/playground build",
    "package": "vsce package"
  },
  "dependencies": {
    "@pmsynapse/rpc": "workspace:*"
  },
  "devDependencies": {
    "@types/vscode": "^1.85.0",
    "tsup": "^8.0.0",
    "typescript": "^5.7.0"
  }
}
```

## Build Pipeline

```bash
# Development (hot reload)
pnpm dev                          # Start all dev servers
# - Playground on :3030
# - Extension watches for changes
# - Launch VS Code with F5

# Production build
pnpm build                        # Build all packages
pnpm --filter vscode-ext package  # Create .vsix file

# Turbo pipeline (turbo.json)
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**"]
    },
    "@pmsynapse/vscode-ext#build": {
      "dependsOn": ["@pmsynapse/playground#build", "@pmsynapse/ui#build"],
      "outputs": ["dist/**", "*.vsix"]
    }
  }
}
```

## Security Considerations

### Content Security Policy (CSP)
- All scripts must use nonces in production
- External resources blocked by default
- Only `vscode-resource:` allowed for styles/images

### Credential Handling
- VS Code SecretStorage for API keys
- Never expose credentials in webview HTML
- Use RPC to request credentials from extension

## References

- [BAML VS Code Extension](https://github.com/BoundaryML/baml/tree/canary/typescript/vscode-ext)
- [VS Code Webview API](https://code.visualstudio.com/api/extension-guides/webview)
- [Tauri + VS Code Patterns](https://tauri.app/v1/guides/getting-started/prerequisites)
