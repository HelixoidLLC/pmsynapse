# UI Architecture Patterns: React + Tauri Desktop Application

## Overview

This document captures the UI architecture patterns from HumanLayer's CodeLayer desktop application for adoption in PMSynapse. The approach combines modern React development with native desktop capabilities via Tauri.

**Reference**: [HumanLayer WUI](https://github.com/humanlayer/humanlayer/tree/main/humanlayer-wui)

---

## PMSynapse Architectural Decision: Rust Backend (Differs from HumanLayer)

### Why Rust Instead of Go

HumanLayer uses **Go** for their HLD daemon. PMSynapse will use **Rust** for a critical reason:

**Rust compiles to WebAssembly (WASM)**, enabling the same backend logic to run:
- **Native desktop** (via Tauri)
- **Native browser** (via WASM)

### The Multi-Platform Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     SHARED RUST CORE                             │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  pmsynapse-core (Rust library)                          │    │
│  │  ├── Agent orchestration                                │    │
│  │  ├── Task/Thought state management                      │    │
│  │  ├── Event bus (pub/sub)                                │    │
│  │  ├── SQLite persistence (via sql.js in WASM)            │    │
│  │  └── 12-factor agent implementation                     │    │
│  └─────────────────────────────────────────────────────────┘    │
│                          │                                       │
│            ┌─────────────┴─────────────┐                        │
│            │                           │                        │
│            ▼                           ▼                        │
│  ┌─────────────────────┐    ┌─────────────────────┐            │
│  │  NATIVE (Tauri)     │    │  BROWSER (WASM)     │            │
│  │                     │    │                     │            │
│  │  - Desktop app      │    │  - Web app          │            │
│  │  - File system      │    │  - IndexedDB        │            │
│  │  - Unix sockets     │    │  - WebSockets       │            │
│  │  - Native notifs    │    │  - Web notifs       │            │
│  │  - System tray      │    │  - PWA support      │            │
│  └─────────────────────┘    └─────────────────────┘            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Benefits of Rust + WASM

| Benefit | Description |
|---------|-------------|
| **Write once, run anywhere** | Same agent logic in desktop and browser |
| **No server required** | Users can run entirely in browser (local-first) |
| **Offline capable** | WASM + IndexedDB enables full offline mode |
| **Performance** | Near-native speed in browser |
| **Type safety** | Rust's guarantees in both environments |
| **Single codebase** | One team maintains one implementation |

### Go vs Rust for This Use Case

| Aspect | Go | Rust |
|--------|-----|------|
| **WASM support** | Poor (large binaries, limited) | Excellent (first-class) |
| **Browser runtime** | Not practical | Production-ready |
| **Desktop native** | Requires separate binary | Tauri integration |
| **Development speed** | Faster initially | Slower, but unified |
| **Long-term maintenance** | Two codebases (web + desktop) | One codebase |

### Platform-Specific Adapters

```rust
// Core trait - platform agnostic
pub trait StorageBackend {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &str, value: &[u8]) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
}

// Native implementation (Tauri)
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeStorage {
    db: rusqlite::Connection,
}

// WASM implementation (Browser)
#[cfg(target_arch = "wasm32")]
pub struct WasmStorage {
    db: sql_js::Database,  // SQLite compiled to WASM
}
```

### Conditional Compilation

```rust
// Shared core logic
pub async fn create_task(task: Task) -> Result<Task> {
    let storage = get_storage();  // Platform-specific
    storage.set(&task.id, &task.serialize()?).await?;
    event_bus::publish(Event::TaskCreated(task.clone())).await;
    Ok(task)
}

// Platform-specific entry points
#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
pub async fn create_task_command(task: Task) -> Result<Task, String> {
    create_task(task).await.map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn create_task_wasm(task: JsValue) -> Result<JsValue, JsValue> {
    let task: Task = serde_wasm_bindgen::from_value(task)?;
    let result = create_task(task).await?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
```

### Project Structure for Dual-Target

```
pmsynapse/
├── crates/
│   ├── pmsynapse-core/       # Shared Rust library
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── agents/       # Agent orchestration
│   │   │   ├── tasks/        # Task management
│   │   │   ├── thoughts/     # Thoughts system
│   │   │   ├── storage/      # Platform-agnostic trait
│   │   │   └── events/       # Event bus
│   │   └── Cargo.toml
│   │
│   ├── pmsynapse-native/     # Tauri-specific
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── commands.rs   # Tauri commands
│   │   │   └── storage.rs    # Native SQLite
│   │   └── Cargo.toml
│   │
│   └── pmsynapse-wasm/       # WASM-specific
│       ├── src/
│       │   ├── lib.rs
│       │   └── storage.rs    # IndexedDB/sql.js
│       └── Cargo.toml
│
├── apps/
│   ├── desktop/              # Tauri + React app
│   │   ├── src/              # React frontend
│   │   └── src-tauri/        # Uses pmsynapse-native
│   │
│   └── web/                  # Web app (Vite + React)
│       ├── src/              # Same React frontend
│       └── wasm/             # Uses pmsynapse-wasm
│
└── Cargo.toml                # Workspace
```

### WASM-Compatible Dependencies

| Need | Native (Tauri) | WASM (Browser) |
|------|---------------|----------------|
| **SQLite** | `rusqlite` | `sql.js` (SQLite in WASM) |
| **HTTP** | `reqwest` | `gloo-net` / `web-sys` |
| **Storage** | File system | IndexedDB |
| **Sockets** | Unix/TCP | WebSocket |
| **Async** | `tokio` | `wasm-bindgen-futures` |

### Trade-offs Accepted

| Trade-off | Impact | Mitigation |
|-----------|--------|------------|
| **Steeper learning curve** | Slower initial development | Long-term unified codebase |
| **Longer compile times** | Developer experience | Incremental compilation, `cargo watch` |
| **WASM binary size** | Initial load time | Code splitting, lazy loading |
| **Limited WASM APIs** | Some features browser-only | Feature flags, graceful degradation |

---

## Technology Stack

### Core Technologies

| Category | Technology | Version | Purpose |
|----------|------------|---------|---------|
| **Frontend Framework** | React | 19.1.0 | UI rendering and component model |
| **Language** | TypeScript | ~5.6.2 | Type safety throughout |
| **Desktop Framework** | Tauri | 2.7.0 | Native desktop packaging |
| **Backend Runtime** | Rust | - | Desktop-specific functionality |
| **Build Tool** | Vite | - | Fast development and bundling |
| **Package Manager** | Bun | - | Fast package management and testing |

### UI & Styling

| Technology | Purpose |
|------------|---------|
| **Tailwind CSS 4.1.10** | Utility-first styling |
| **Radix UI** | Headless, accessible UI primitives |
| **ShadCN Components** | Pre-styled Radix components |
| **Lucide React** | Icon library |

### State & Data

| Technology | Purpose |
|------------|---------|
| **Zustand 5.0.5** | Global state management |
| **React Router DOM 7.6.3** | Client-side routing |
| **TipTap 3.0.9** | Rich text editing |
| **React Markdown** | Markdown rendering |

### Quality & Monitoring

| Technology | Purpose |
|------------|---------|
| **ESLint** | Code linting |
| **Prettier** | Code formatting |
| **Storybook 9.1.5** | Component documentation |
| **Sentry 10.10.0** | Error tracking |
| **PostHog** | Analytics |

---

## Architecture Overview

### Multi-Process Architecture

Tauri uses a multi-process model similar to modern browsers:

```
┌─────────────────────────────────────────────────────────┐
│                    TAURI APPLICATION                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─────────────────────────────────────────────────┐    │
│  │              FRONTEND PROCESS                    │    │
│  │                                                  │    │
│  │  React App (TypeScript)                         │    │
│  │  ├── Components (ShadCN/Radix)                  │    │
│  │  ├── Hooks (State & Data)                       │    │
│  │  ├── Stores (Zustand)                           │    │
│  │  └── Services (API Clients)                     │    │
│  │                                                  │    │
│  │  Rendered in: WebView (OS native)               │    │
│  └──────────────────┬───────────────────────────────┘    │
│                     │                                    │
│                     │ IPC (Tauri Commands)               │
│                     │                                    │
│  ┌──────────────────▼───────────────────────────────┐    │
│  │              BACKEND PROCESS                     │    │
│  │                                                  │    │
│  │  Rust Core                                       │    │
│  │  ├── Command Handlers                           │    │
│  │  ├── System Integration                         │    │
│  │  ├── File System Access                         │    │
│  │  └── Socket Communication                       │    │
│  │                                                  │    │
│  └──────────────────┬───────────────────────────────┘    │
│                     │                                    │
└─────────────────────┼────────────────────────────────────┘
                      │
                      │ Unix Socket / JSON-RPC
                      │
              ┌───────▼───────┐
              │    DAEMON     │
              │   (Backend    │
              │   Service)    │
              └───────────────┘
```

### Layer Responsibilities

```
┌─────────────────────────────────────────────────────────┐
│                    FRONTEND LAYERS                       │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  COMPONENTS (React + ShadCN)                            │
│  └─ Presentation only                                   │
│  └─ Accept UI types as props                            │
│  └─ No direct daemon access                             │
│                       │                                  │
│                       ▼                                  │
│  HOOKS (Custom React Hooks)                             │
│  └─ State management                                    │
│  └─ Data transformation/enrichment                      │
│  └─ Error handling                                      │
│  └─ Loading states                                      │
│                       │                                  │
│                       ▼                                  │
│  DAEMON CLIENT (TypeScript)                             │
│  └─ Type-safe protocol communication                    │
│  └─ Low-level API calls                                 │
│                       │                                  │
│                       ▼                                  │
│  TAURI BRIDGE (IPC Commands)                            │
│  └─ TypeScript ↔ Rust translation                       │
│  └─ Invoke Rust functions from JS                       │
│                                                          │
├─────────────────────────────────────────────────────────┤
│                    BACKEND LAYERS                        │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  RUST HANDLERS                                          │
│  └─ Command implementations                             │
│  └─ System API access                                   │
│                       │                                  │
│                       ▼                                  │
│  DAEMON CLIENT (Rust)                                   │
│  └─ JSON-RPC protocol                                   │
│  └─ Socket management                                   │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Project Structure

### Directory Organization

```
pmsynapse-ui/
├── .storybook/              # Storybook configuration
├── docs/                    # Architecture documentation
├── src/
│   ├── components/          # React components
│   │   ├── ui/              # ShadCN base components
│   │   ├── features/        # Feature-specific components
│   │   └── layout/          # Layout components
│   │
│   ├── hooks/               # Custom React hooks
│   │   ├── useThoughts.ts
│   │   ├── useTasks.ts
│   │   └── useAgents.ts
│   │
│   ├── stores/              # Zustand stores
│   │   ├── appStore.ts
│   │   └── sessionStore.ts
│   │
│   ├── services/            # API clients
│   │   ├── daemonClient.ts
│   │   └── thoughtsClient.ts
│   │
│   ├── pages/               # Route pages
│   ├── contexts/            # React contexts
│   ├── types/               # TypeScript definitions
│   ├── utils/               # Helper functions
│   ├── styles/              # Global styles
│   │
│   ├── main.tsx             # Entry point
│   ├── App.tsx              # Root component
│   └── router.tsx           # Route configuration
│
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── commands/        # Tauri command handlers
│   │   └── clients/         # Backend service clients
│   │
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
│
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
└── CLAUDE.md                # AI assistant instructions
```

---

## Component Patterns

### 1. ShadCN Component Usage

**Principle**: Prefer ShadCN components over custom implementations.

```typescript
// ✅ DO: Use ShadCN components
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"

function TaskCreator() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Create Task</Button>
      </DialogTrigger>
      <DialogContent>
        <Input placeholder="Task title..." />
      </DialogContent>
    </Dialog>
  )
}

// ❌ DON'T: Create custom components for standard UI
function CustomButton({ onClick, children }) {
  return (
    <button
      onClick={onClick}
      className="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded"
    >
      {children}
    </button>
  )
}
```

### 2. Hooks for Everything

**Principle**: Components never access daemon/services directly. Always use hooks.

```typescript
// ✅ DO: Use hooks for data access
import { useTasks } from "@/hooks/useTasks"

function TaskList() {
  const { tasks, isLoading, error, createTask } = useTasks()

  if (isLoading) return <Skeleton />
  if (error) return <ErrorMessage error={error} />

  return (
    <ul>
      {tasks.map(task => (
        <TaskItem key={task.id} task={task} />
      ))}
    </ul>
  )
}

// ❌ DON'T: Access services directly in components
import { daemonClient } from "@/services/daemonClient"

function TaskList() {
  const [tasks, setTasks] = useState([])

  useEffect(() => {
    // Direct daemon access - BAD!
    daemonClient.getTasks().then(setTasks)
  }, [])
}
```

### 3. UI Types vs Protocol Types

**Principle**: Components use UI types, not protocol types.

```typescript
// types/ui.ts - UI-specific types
export interface TaskUI {
  id: string
  title: string
  status: TaskStatus
  complexity: number
  assignee?: string
  formattedDueDate: string  // Pre-formatted for display
  priorityLabel: string     // Human-readable
}

// types/protocol.ts - Wire format types
export interface TaskProtocol {
  id: string
  title: string
  status: number           // Raw enum value
  complexity_score: number // Snake case from backend
  assignee_id?: string
  due_date: string         // ISO timestamp
  priority: number
}

// hooks/useTasks.ts - Transformation layer
export function useTasks() {
  const [rawTasks, setRawTasks] = useState<TaskProtocol[]>([])

  // Transform protocol types to UI types
  const tasks: TaskUI[] = rawTasks.map(raw => ({
    id: raw.id,
    title: raw.title,
    status: TaskStatus[raw.status],
    complexity: raw.complexity_score,
    assignee: raw.assignee_id,
    formattedDueDate: formatDate(raw.due_date),
    priorityLabel: getPriorityLabel(raw.priority)
  }))

  return { tasks, ... }
}

// components/TaskItem.tsx - Uses UI types only
interface TaskItemProps {
  task: TaskUI  // ✅ UI type, not protocol type
}

function TaskItem({ task }: TaskItemProps) {
  return (
    <div>
      <h3>{task.title}</h3>
      <span>{task.priorityLabel}</span>
      <span>{task.formattedDueDate}</span>
    </div>
  )
}
```

### 4. React 19 Patterns

**Principle**: Use React 19 features, avoid deprecated patterns.

```typescript
// ✅ DO: Use ref directly (React 19)
function Input({ ref, ...props }) {
  return <input ref={ref} {...props} />
}

// ❌ DON'T: Use forwardRef (deprecated in React 19)
const Input = forwardRef((props, ref) => {
  return <input ref={ref} {...props} />
})
```

---

## State Management with Zustand

### Store Structure

```typescript
// stores/appStore.ts
import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

interface AppState {
  // State
  selectedTaskId: string | null
  sidebarOpen: boolean
  theme: 'light' | 'dark' | 'system'

  // Actions
  selectTask: (id: string | null) => void
  toggleSidebar: () => void
  setTheme: (theme: 'light' | 'dark' | 'system') => void
}

export const useAppStore = create<AppState>()(
  devtools(
    persist(
      (set) => ({
        // Initial state
        selectedTaskId: null,
        sidebarOpen: true,
        theme: 'system',

        // Actions
        selectTask: (id) => set({ selectedTaskId: id }),
        toggleSidebar: () => set((state) => ({
          sidebarOpen: !state.sidebarOpen
        })),
        setTheme: (theme) => set({ theme }),
      }),
      { name: 'pmsynapse-app-store' }
    )
  )
)
```

### Store Best Practices

```typescript
// ✅ DO: Use selectors for performance
function TaskTitle() {
  const title = useAppStore(state =>
    state.tasks.find(t => t.id === state.selectedTaskId)?.title
  )
  return <h1>{title}</h1>
}

// ❌ DON'T: Subscribe to entire store
function TaskTitle() {
  const { tasks, selectedTaskId } = useAppStore()
  const title = tasks.find(t => t.id === selectedTaskId)?.title
  return <h1>{title}</h1>
}

// ✅ DO: Separate stores by domain
const useTaskStore = create(...)
const useThoughtsStore = create(...)
const useUIStore = create(...)

// ❌ DON'T: One massive store for everything
const useEverythingStore = create(...)
```

---

## Keyboard Navigation

### Vim-Style Shortcuts

HumanLayer implements vim-style keyboard navigation:

```typescript
// hooks/useKeyboardNavigation.ts
import { useHotkeys } from 'react-hotkeys-hook'

export function useKeyboardNavigation(items: any[]) {
  const [selectedIndex, setSelectedIndex] = useState(0)
  const [selectionAnchor, setSelectionAnchor] = useState<number | null>(null)

  // j/k for navigation
  useHotkeys('j', () => {
    setSelectedIndex(i => Math.min(i + 1, items.length - 1))
  }, [items.length])

  useHotkeys('k', () => {
    setSelectedIndex(i => Math.max(i - 1, 0))
  }, [])

  // Shift+j/k for range selection
  useHotkeys('shift+j', () => {
    if (selectionAnchor === null) {
      setSelectionAnchor(selectedIndex)
    }
    setSelectedIndex(i => Math.min(i + 1, items.length - 1))
  }, [selectedIndex, selectionAnchor, items.length])

  // x for toggle selection
  useHotkeys('x', () => {
    toggleSelection(selectedIndex)
  }, [selectedIndex])

  // e for action (archive/unarchive)
  useHotkeys('e', () => {
    performAction(getSelectedItems())
  }, [])

  return {
    selectedIndex,
    selectedItems: computeSelectedItems(selectedIndex, selectionAnchor),
    // ... other values
  }
}
```

### Stateless Anchor Management

**Key Pattern**: Calculate selection dynamically, don't store in state.

```typescript
// ✅ DO: Compute selection from anchor and current index
function computeSelectedItems(
  currentIndex: number,
  anchor: number | null
): number[] {
  if (anchor === null) return [currentIndex]

  const start = Math.min(anchor, currentIndex)
  const end = Math.max(anchor, currentIndex)

  return Array.from({ length: end - start + 1 }, (_, i) => start + i)
}

// ❌ DON'T: Store selected items in state (sync issues)
const [selectedItems, setSelectedItems] = useState<number[]>([])
```

---

## Tauri Integration

### Command Pattern

```typescript
// Frontend: Invoke Tauri command
import { invoke } from '@tauri-apps/api/core'

async function createTask(task: CreateTaskRequest): Promise<Task> {
  return await invoke('create_task', { task })
}

// Backend (Rust): Command handler
#[tauri::command]
async fn create_task(task: CreateTaskRequest) -> Result<Task, String> {
    let daemon = get_daemon_client().await?;
    daemon.create_task(task).await
        .map_err(|e| e.to_string())
}
```

### Tauri Plugins Used

```json
{
  "@tauri-apps/plugin-clipboard-manager": "^2.2.1",
  "@tauri-apps/plugin-fs": "^2.2.0",
  "@tauri-apps/plugin-notification": "^2.2.1",
  "@tauri-apps/plugin-global-shortcut": "^2.2.0"
}
```

### Communication Flow

```
React Component
      │
      │ useTasks()
      ▼
Custom Hook
      │
      │ daemonClient.createTask()
      ▼
TypeScript Client
      │
      │ invoke('create_task', { task })
      ▼
Tauri IPC Bridge
      │
      │ tauri::command
      ▼
Rust Command Handler
      │
      │ daemon.create_task(task)
      ▼
Rust Daemon Client
      │
      │ JSON-RPC over Unix Socket
      ▼
Backend Daemon Service
```

---

## Styling with Tailwind + ShadCN

### CSS Variable Theming

```css
/* styles/globals.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
    --primary-foreground: 210 40% 98%;
    --secondary: 210 40% 96.1%;
    --secondary-foreground: 222.2 47.4% 11.2%;
    --muted: 210 40% 96.1%;
    --muted-foreground: 215.4 16.3% 46.9%;
    --accent: 210 40% 96.1%;
    --accent-foreground: 222.2 47.4% 11.2%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 210 40% 98%;
    --border: 214.3 31.8% 91.4%;
    --input: 214.3 31.8% 91.4%;
    --ring: 222.2 84% 4.9%;
    --radius: 0.5rem;
  }

  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    /* ... dark mode values */
  }
}
```

### Component Variants with cn()

```typescript
// lib/utils.ts
import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// Usage in components
function TaskCard({ task, className }: TaskCardProps) {
  return (
    <div className={cn(
      "rounded-lg border p-4",
      task.priority === 'high' && "border-red-500",
      task.status === 'completed' && "opacity-60",
      className
    )}>
      {/* ... */}
    </div>
  )
}
```

---

## Error Handling

### User-Friendly Error Messages

```typescript
// utils/errors.ts
export function formatError(error: unknown): string {
  if (error instanceof DaemonConnectionError) {
    return "Unable to connect to PMSynapse service. Please ensure it's running."
  }
  if (error instanceof AuthenticationError) {
    return "Session expired. Please log in again."
  }
  if (error instanceof ValidationError) {
    return error.userMessage
  }

  // Log technical error for debugging
  console.error('Unexpected error:', error)

  // Return generic message to user
  return "Something went wrong. Please try again."
}

// hooks/useTasks.ts
export function useTasks() {
  const [error, setError] = useState<string | null>(null)

  async function createTask(data: CreateTaskData) {
    try {
      await daemonClient.createTask(data)
      setError(null)
    } catch (e) {
      setError(formatError(e))
    }
  }

  return { error, createTask }
}
```

### Error Boundaries

```typescript
// components/ErrorBoundary.tsx
import { ErrorBoundary as ReactErrorBoundary } from 'react-error-boundary'

function ErrorFallback({ error, resetErrorBoundary }) {
  return (
    <div className="flex flex-col items-center justify-center p-8">
      <h2 className="text-lg font-semibold">Something went wrong</h2>
      <p className="text-muted-foreground">{formatError(error)}</p>
      <Button onClick={resetErrorBoundary} className="mt-4">
        Try again
      </Button>
    </div>
  )
}

// App.tsx
function App() {
  return (
    <ReactErrorBoundary FallbackComponent={ErrorFallback}>
      <Router />
    </ReactErrorBoundary>
  )
}
```

---

## Development Workflow

### Scripts

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx",
    "lint:fix": "eslint . --ext ts,tsx --fix",
    "typecheck": "tsc --noEmit",
    "test": "bun test",
    "storybook": "storybook dev -p 6006",
    "build-storybook": "storybook build",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  }
}
```

### Quality Checks

```bash
# Run before committing
bun run lint        # Check code style
bun run typecheck   # Verify types
bun test            # Run tests
```

### Storybook for Components

```typescript
// components/ui/Button.stories.tsx
import type { Meta, StoryObj } from '@storybook/react'
import { Button } from './button'

const meta: Meta<typeof Button> = {
  component: Button,
  tags: ['autodocs'],
}

export default meta
type Story = StoryObj<typeof Button>

export const Primary: Story = {
  args: {
    children: 'Button',
    variant: 'default',
  },
}

export const Destructive: Story = {
  args: {
    children: 'Delete',
    variant: 'destructive',
  },
}
```

---

## Log Management

### Platform-Specific Paths

| Platform | Log Location |
|----------|-------------|
| macOS | `~/Library/Logs/dev.pmsynapse.app/` |
| Windows | `%APPDATA%\dev.pmsynapse.app\logs\` |
| Linux | `~/.config/dev.pmsynapse.app/logs/` |

### Log Rotation

- Files auto-rotate at 50MB
- Keep last 5 rotated files
- Use structured JSON logging for machine parsing

---

## Implementation Checklist for PMSynapse

### Phase 1: Foundation
- [ ] Initialize Tauri + React + TypeScript project
- [ ] Set up Tailwind CSS with ShadCN
- [ ] Configure ESLint, Prettier, TypeScript
- [ ] Create directory structure
- [ ] Set up Zustand stores

### Phase 2: Core UI
- [ ] Install ShadCN base components
- [ ] Create layout components (sidebar, header, main)
- [ ] Implement router and pages
- [ ] Add keyboard navigation hooks

### Phase 3: Backend Integration
- [ ] Create TypeScript daemon client
- [ ] Implement Tauri commands (Rust)
- [ ] Build custom hooks for each domain
- [ ] Add error handling layer

### Phase 4: Features
- [ ] Task management UI
- [ ] Thoughts browser
- [ ] Agent status dashboard
- [ ] Settings and preferences

### Phase 5: Polish
- [ ] Add Storybook stories
- [ ] Implement dark mode
- [ ] Add loading states and skeletons
- [ ] Integrate Sentry error tracking

---

## Sources

- [HumanLayer WUI Repository](https://github.com/humanlayer/humanlayer/tree/main/humanlayer-wui)
- [Tauri Process Model](https://v2.tauri.app/concept/process-model/)
- [ShadCN/UI Documentation](https://ui.shadcn.com/)
- [Radix UI Documentation](https://www.radix-ui.com/)
- [Zustand Documentation](https://zustand-demo.pmnd.rs/)
- [Vercel Academy - ShadCN/UI](https://vercel.com/academy/shadcn-ui)

---

*Document created: December 2025*
*Part of: PMSynapse AI-Enabled Project Management Research*
