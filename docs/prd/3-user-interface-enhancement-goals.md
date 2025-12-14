# 3. User Interface Enhancement Goals

**Enhancement includes UI changes:** ✓ Yes - VS Code extension, desktop app, and web UI

## 3.1 Integration with Existing UI

**Design System Foundation:**

PMSynapse MVP introduces three new UI surfaces that must integrate cohesively:

1. **VS Code Extension** - Native IDE integration following VS Code extension guidelines and Webview UI Toolkit patterns
2. **Desktop App** - Tauri 2.0 application with React 18 frontend using shadcn/ui components
3. **Web UI** - Browser-based knowledge graph visualization with read-only access

**Integration Requirements:**

- **Component Library:** All UIs shall use shadcn/ui as the base component library to ensure visual consistency. Components include Button, Input, Card, Dialog, Dropdown, Table, and navigation elements.

- **Design Tokens:** Shared design system with common color palette, typography scale (font families, sizes, weights), spacing system (4px/8px grid), and border radius values.

- **Styling Approach:** Tailwind CSS utility classes for styling across all UI surfaces. VS Code extension adapts to user's theme (light/dark mode), desktop app provides theme toggle, web UI follows system preference.

- **Accessibility:** All UI components shall meet WCAG 2.1 AA standards with keyboard navigation, screen reader support, and proper ARIA attributes.

- **Responsive Design:** Desktop app and web UI shall support responsive layouts (minimum 1024px width recommended, graceful degradation to 768px). VS Code extension adapts to panel width.

**Existing Design Patterns to Follow:**

- **CLI-First Philosophy:** UI surfaces complement but don't replace CLI functionality. Power users should be able to accomplish all tasks via CLI.

- **Non-Intrusive Presentation:** UIs appear on-demand (keyboard shortcuts, explicit commands) rather than auto-populating or interrupting workflow.

- **Status Visibility:** Workflow status (triage → research → planning → development) visible at a glance without requiring navigation.

## 3.2 Modified/New Screens and Views

**VS Code Extension Views:**

1. **Thought Capture Panel**
   - Quick input form (keyboard shortcut activated)
   - Fields: thought title, description, optional tags
   - Workflow suggestion preview with override buttons
   - Recent thoughts list (last 10)

2. **Knowledge Graph Sidebar**
   - Tree view of thoughts organized by workflow stage
   - Expandable nodes showing linked research, plans, tickets
   - Click-to-navigate to related documents/files
   - Search bar for filtering thoughts

3. **Workflow Status View**
   - Current thought's progress through workflow stages
   - Linked Linear tickets with status badges
   - Linked GitHub commits/PRs
   - Quick actions (move stage, add research, create plan)

**Desktop App Screens:**

1. **Dashboard (Home)**
   - Overview metrics (total thoughts, active workflows, completion rate)
   - Recent activity feed (thoughts captured, research completed, tickets created)
   - Quick actions (capture thought, view graph, check Linear sync status)

2. **Knowledge Graph Visualization**
   - Interactive node-link diagram showing thoughts, research, plans, implementations
   - Node types distinguished by color/shape
   - Relationship edges with labels (originated-from, informs, implements)
   - Zoom/pan controls, filter by workflow stage
   - Click node to view details panel

3. **Thought Detail View**
   - Full thought metadata (author, timestamp, tags, workflow stage)
   - Linked research documents (with preview/open)
   - Associated plans and implementation tickets
   - Edit/delete controls, stage transition buttons

4. **Settings Panel**
   - Integration configuration (Linear OAuth, GitHub token, LLM provider API keys)
   - Workflow customization (IDLC stages, default paths)
   - UI preferences (theme, notification settings)
   - Data management (export, backup, clear cache)

**Web UI Views:**

1. **Read-Only Graph Visualization**
   - Same knowledge graph display as desktop app
   - No editing capabilities (view-only for stakeholders)
   - Export options (PNG, SVG, JSON)

2. **Thought Browser**
   - Searchable list of all thoughts with filters (date, author, stage, tags)
   - Detail view for selected thought
   - Permalink sharing for specific thoughts/graphs

## 3.3 UI Consistency Requirements

**Visual Consistency:**

- **Color Palette:** Primary color (brand identity), secondary colors for workflow stages, semantic colors (success: green, warning: yellow, error: red, info: blue)

- **Typography:**
  - Headings: System font stack (SF Pro on macOS, Segoe UI on Windows, Inter/Roboto on Linux)
  - Body: Same system fonts with 16px base size, 1.5 line height
  - Code: JetBrains Mono or Fira Code for monospace elements

- **Spacing:** Consistent 8px spacing unit across all components and layouts

- **Icons:** Use same icon library (Lucide Icons or Heroicons) across all UIs for consistency

**Interaction Consistency:**

- **Keyboard Shortcuts:** Consistent shortcuts across VS Code extension and desktop app (e.g., Cmd/Ctrl+Shift+T for thought capture)

- **Navigation Patterns:** Breadcrumb navigation for hierarchical views, sidebar navigation for major sections

- **Form Behavior:** Consistent input validation, error messaging placement (below fields), submit button states (loading, disabled, success)

- **Feedback Mechanisms:** Toast notifications for actions (thought saved, sync completed), progress indicators for long operations (>2 seconds)

**State Management:**

- **Loading States:** Skeleton screens for initial loads, spinners for async operations, optimistic UI updates where appropriate

- **Empty States:** Helpful empty state messages with call-to-action ("No thoughts yet. Capture your first idea!")

- **Error States:** Clear error messages with recovery suggestions (not technical jargon)

**Cross-UI Consistency Rules:**

- Thought cards display consistently (title, metadata, stage badge, action buttons) across all UIs
- Workflow stage badges use same colors and labels everywhere
- Linear ticket references show same format (icon + ID + title)
- GitHub commit links use same presentation (hash + message preview)

---
