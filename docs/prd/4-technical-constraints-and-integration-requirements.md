# 4. Technical Constraints and Integration Requirements

## 4.1 Existing Technology Stack

**Languages:**
- Rust (stable channel, edition 2021) - Backend engine, CLI, daemon
- TypeScript/JavaScript - Frontend (React), VS Code extension, RPC types
- SQL - Database queries and schema (SQLite)

**Frameworks:**
- **Backend:** Tokio (async runtime), rusqlite (database), serde (serialization), thiserror (error handling), clap (CLI parsing)
- **Frontend:** React 18 (UI library), Vite 6 (build tool), Tailwind CSS (styling), shadcn/ui (component library), Zustand (state management)
- **Desktop:** Tauri 2.0 with plugins (fs, shell, notification, log, store)
- **VS Code:** VS Code Extension API 1.85+, Webview UI Toolkit

**Database:**
- **Current:** SQLite 3.x via rusqlite (embedded, local-first, zero-config)
- **Future:** CozoDB (graph + vector database, WASM support, Datalog queries)
- Migration path required from SQLite → CozoDB (see FR32, CR2)

**Infrastructure:**
- **Local:** Daemon process running on developer machine (no server required for MVP)
- **Web UI:** Static site hosting (Vercel, Netlify, or CloudFlare Pages)
- **OAuth Callbacks:** Public endpoint required for Linear/GitHub OAuth flows

**External Dependencies:**
- Linear API (GraphQL, OAuth 2.0)
- GitHub API (REST v3 + GraphQL v4, OAuth Apps)
- LLM Providers: OpenAI API, Anthropic API (Claude)
- System Keychain: macOS Keychain, Windows Credential Manager, Linux Secret Service

**Version Constraints:**
- Rust: 1.70+ (for Tauri 2.0 compatibility)
- Node.js: 18+ (for pnpm workspaces, Vite 6)
- VS Code: 1.85+ (extension compatibility)
- Tauri: 2.0.x (alpha/beta currently, stable expected Q1 2026)

## 4.2 Integration Approach

**Database Integration Strategy:**

- **Local-First Architecture:** SQLite database stored in user's home directory (`~/.pmsynapse/data/graph.db`)
- **Schema Management:** Migration system using rusqlite with version tracking table
- **Atomic Transactions:** All multi-table operations wrapped in transactions for consistency
- **Backup Strategy:** Automatic daily backups to `~/.pmsynapse/backups/` with 7-day retention
- **Query Optimization:** Indexes on frequently queried columns (thought_id, workflow_stage, author, timestamp)
- **CozoDB Migration:** Abstract database layer (repository pattern) to support future CozoDB switch without breaking RPC contracts

**API Integration Strategy:**

- **Linear Integration:**
  - OAuth 2.0 flow for authentication
  - GraphQL API for ticket operations (create, update, query)
  - Webhook support (future) for real-time status updates
  - Queue-and-retry for offline scenarios (NFR22)
  - Rate limiting respect (max 5 req/sec per Linear docs)

- **GitHub Integration:**
  - OAuth Apps authentication (not personal access tokens)
  - REST API v3 for repository operations
  - GraphQL API v4 for complex queries (commit history, PR status)
  - Commit message conventions for auto-linking (`[THOUGHT-123]`, `Relates to PLAN-456`)
  - Webhook support (future) for push events

- **LLM Provider Integration:**
  - Multi-provider abstraction layer (OpenAI, Anthropic, future: local models)
  - API key management via system keychain (NFR11)
  - Request/response caching for offline mode (FR29)
  - Streaming support for long-running generations
  - Error handling and fallback (if primary provider fails, queue for retry)

**Frontend Integration Strategy:**

- **Desktop App ↔ Daemon:** IPC via Tauri commands (invoke from React → Rust handlers)
- **VS Code Extension ↔ Daemon:** Unix socket or TCP localhost connection for RPC
- **CLI ↔ Daemon:** Direct function calls (snps-cli links snps-core as library)
- **Web UI ↔ Daemon:** WebSocket connection for real-time graph updates (read-only)
- **Shared Types:** TypeScript RPC types generated from Rust types using ts-rs or similar

**Testing Integration Strategy:**

- **Unit Tests:** Inline with Rust source (`#[cfg(test)]`), Jest for TypeScript
- **Integration Tests:** `engine/*/tests/` for Rust cross-module, `integ-tests/` for cross-component
- **E2E Tests:** Tauri's testing framework for desktop app user flows
- **API Mocking:** Mock Linear/GitHub APIs in tests using wiremock or similar
- **CI/CD:** GitHub Actions running tests on Linux, macOS, Windows

## 4.3 Code Organization and Standards

**File Structure Approach:**

```
pmsynapse/
├── engine/                    # Rust workspace
│   ├── snps-core/             # Core library
│   │   ├── src/
│   │   │   ├── graph.rs       # Knowledge graph operations
│   │   │   ├── workflow.rs    # IDLC workflow engine
│   │   │   ├── integrations/  # Linear, GitHub, LLM adapters
│   │   │   └── lib.rs         # Public API
│   │   └── tests/             # Integration tests
│   └── snps-cli/              # CLI tool
│       ├── src/
│       │   ├── commands/      # CLI command implementations
│       │   ├── daemon.rs      # Daemon process
│       │   └── main.rs
│       └── tests/
├── apps/
│   ├── desktop/               # Tauri app
│   │   ├── src-tauri/         # Rust backend
│   │   └── src/               # React frontend
│   └── vscode-ext/            # VS Code extension
│       ├── src/extension.ts   # Extension entry point
│       └── src/webview/       # Webview UI
├── packages/
│   └── rpc/                   # Shared TypeScript types
├── integ-tests/               # Cross-component tests
└── docs/                      # Documentation
```

**Naming Conventions:**

- **Rust:** snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
- **TypeScript:** camelCase for functions/variables, PascalCase for types/interfaces/components
- **Files:** kebab-case for filenames (knowledge-graph.rs, thought-capture.tsx)
- **Database:** snake_case for table/column names (thoughts table, workflow_stage column)

**Coding Standards:**

- **Rust:**
  - Always use type annotations for function parameters and returns
  - Prefer `?` operator over explicit match for error propagation
  - Use thiserror for custom error types
  - Document public APIs with `///` doc comments
  - Run `cargo fmt` and `cargo clippy` before commits

- **TypeScript:**
  - Strict mode enabled (`strict: true` in tsconfig.json)
  - Explicit return types for exported functions
  - Prefer functional components with hooks (no class components)
  - PropTypes via TypeScript interfaces (no separate prop-types package)

- **Testing:**
  - Aim for 70%+ code coverage on core logic
  - Test both happy path and error scenarios
  - Integration tests for API boundaries
  - Snapshot tests for UI components (React Testing Library)

**Documentation Standards:**

- **Code Comments:** Explain "why" not "what" (code should be self-explanatory)
- **README Files:** Each package has README with overview, setup, usage
- **API Documentation:** Generate from doc comments (rustdoc for Rust, TSDoc for TypeScript)
- **Architecture Decision Records (ADRs):** Document major technical decisions in `docs/adrs/`

## 4.4 Deployment and Operations

**Build Process Integration:**

- **Monorepo Orchestration:** Turborepo manages build dependencies across packages
- **Rust Compilation:**
  - `cargo build --release -p snps-core -p snps-cli` for production builds
  - Binary output: `target/release/snps` (CLI), libraries linked by Tauri app
- **Frontend Builds:**
  - `pnpm build` in desktop/vscode-ext triggers Vite/webpack builds
  - Tauri bundles React build with Rust binary into single executable
- **Cross-Compilation:** GitHub Actions matrix builds for macOS, Linux, Windows
- **Versioning:** Semantic versioning (semver), version.txt or Cargo.toml as source of truth

**Deployment Strategy:**

- **MVP Phase (Local-Only):**
  - CLI distributed as binary via GitHub Releases (dmg/pkg for macOS, exe for Windows, deb/AppImage for Linux)
  - Desktop app distributed via same method (Tauri bundles)
  - VS Code extension published to VS Code Marketplace
  - No cloud infrastructure required (local-first architecture)

- **Web UI (Static Hosting):**
  - Build: `pnpm build` in web-ui package
  - Deploy: Static files to Vercel/Netlify/CloudFlare Pages
  - CDN distribution for global access
  - OAuth redirect URLs point to hosted domain

- **Future (Optional Cloud Sync):**
  - Cloud sync service for multi-device (AWS/GCP/Azure)
  - Database hosted in user's cloud account (PostgreSQL for multi-team)
  - End-to-end encryption for cloud-synced data

**Monitoring and Logging:**

- **Local Logging:**
  - Rust: `tracing` crate with `RUST_LOG` environment variable for level control
  - Log files: `~/.pmsynapse/logs/daemon.log` with rotation (10MB max, 5 files retained)
  - VS Code extension: Output channel for extension logs
  - Desktop app: Tauri log plugin for structured logging

- **Error Reporting:**
  - Local error logs with stack traces (no external error tracking for MVP)
  - Future: Optional Sentry integration with user opt-in

- **Performance Monitoring:**
  - Local metrics collection (thought capture time, query latency, sync duration)
  - Aggregated locally, viewable in Settings panel
  - No external APM for MVP (privacy-first approach)

**Configuration Management:**

- **User Configuration:**
  - File: `~/.pmsynapse/config.yaml` (API keys, preferences, IDLC customization)
  - Tauri store plugin for desktop app settings (theme, notifications)
  - VS Code settings for extension configuration

- **Secrets Management:**
  - API keys stored in system keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
  - OAuth tokens encrypted at rest using keyring crate
  - Never store secrets in plaintext config files

- **Environment Variables:**
  - `RUST_LOG`: Control logging level (debug, info, warn, error)
  - `PMSYNAPSE_DATA_DIR`: Override default data directory
  - `PMSYNAPSE_DAEMON_PORT`: Custom daemon RPC port (default: 50051)

## 4.5 Risk Assessment and Mitigation

**Technical Risks:**

**Risk 1: Tauri 2.0 Stability**
- **Description:** Tauri 2.0 is currently alpha/beta; API changes possible before stable release
- **Impact:** Breaking changes could require desktop app refactoring
- **Likelihood:** Medium (Tauri team committed to Q1 2026 stable)
- **Mitigation:** Pin to specific Tauri alpha version, track release notes closely, allocate 2-week buffer for migration to stable

**Risk 2: SQLite → CozoDB Migration Complexity**
- **Description:** Migrating from relational (SQLite) to graph database (CozoDB) is non-trivial
- **Impact:** Could require rewriting entire data layer, breaking compatibility
- **Likelihood:** High (architecture mismatch between relational and graph models)
- **Mitigation:** Abstract database layer from day one (FR32), design schema with graph migration in mind, validate migration path with prototype

**Risk 3: LLM API Cost and Availability**
- **Description:** OpenAI/Anthropic API costs could be prohibitive; rate limits could block features
- **Impact:** MVP becomes too expensive to run or features are unavailable during high usage
- **Likelihood:** Medium (depends on user adoption and prompt engineering efficiency)
- **Mitigation:** Aggressive caching (FR29), local model support (Ollama), cost monitoring dashboard, usage limits per user

**Integration Risks:**

**Risk 4: Linear/GitHub API Changes**
- **Description:** Third-party APIs could introduce breaking changes without notice
- **Impact:** Integration features stop working, user workflows broken
- **Likelihood:** Low-Medium (established APIs rarely break compatibility)
- **Mitigation:** API version pinning, integration test suite with real API calls (nightly), graceful degradation (NFR21-22), queue-and-retry (NFR22)

**Risk 5: OAuth Flow Complexity**
- **Description:** OAuth 2.0 callback requires public URL; local-first architecture complicates this
- **Impact:** Users can't authenticate with Linear/GitHub without additional setup
- **Likelihood:** Medium (OAuth inherently requires server-side components)
- **Mitigation:** Provide localhost callback option with port forwarding instructions, offer personal access token alternative, future: simple proxy service for OAuth callbacks

**Deployment Risks:**

**Risk 6: Cross-Platform Binary Distribution**
- **Description:** Building and distributing binaries for macOS, Linux, Windows is complex (code signing, notarization, installers)
- **Impact:** Users can't install/run PMSynapse on their platform, security warnings scare users away
- **Likelihood:** High (inevitable for desktop app distribution)
- **Mitigation:** GitHub Actions CI/CD for automated builds, invest in code signing certificates (macOS, Windows), provide multiple install methods (dmg, pkg, exe, deb, AppImage, homebrew)

**Risk 7: VS Code Extension Publishing**
- **Description:** VS Code Marketplace requires publisher verification, review process can be slow
- **Impact:** Extension availability delayed, early adopters can't install
- **Likelihood:** Low (extension publishing is well-documented)
- **Mitigation:** Complete publisher verification early, prepare extension package in advance, have manual installation option (vsix file)

**Mitigation Strategies Summary:**

- **Abstraction Layers:** Database (FR32), LLM providers, integration APIs to minimize lock-in
- **Graceful Degradation:** Offline mode (FR29), queue-and-retry (NFR22), clear error messaging (NFR21)
- **CI/CD Automation:** Automated testing, cross-platform builds, release process
- **User Control:** Manual override of automation, local data ownership, optional cloud features
- **Cost Management:** Caching, usage limits, cost monitoring, local model fallback

---
