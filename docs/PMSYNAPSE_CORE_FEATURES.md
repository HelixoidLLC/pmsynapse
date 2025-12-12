# PMSynapse Core Features: End-to-End Project Management

## Overview

PMSynapse is an AI-enabled, end-to-end project management system that guides users from **idea to implementation**. It combines:

- **Semantic Knowledge Graph** for evolving project understanding
- **Dual documentation** (human-readable + graph-queryable)
- **Multi-provider LLM integration** (OpenRouter-style)
- **Customizable conversational rules** (starting with BMAD method)
- **Legacy codebase analysis** with assumption generation

---

## Part 1: Knowledge Graph of Thoughts Architecture

### Inspiration

Based on [Knowledge Graph of Thoughts (KGoT)](https://github.com/spcl/knowledge-graph-of-thoughts) by ETH Zurich, PMSynapse uses a **dynamic knowledge graph** that evolves as understanding deepens.

### Why Graph > Linear Thinking

```
Chain-of-Thought:     A → B → C → D          (linear, no backtracking)
Tree-of-Thoughts:     A → B → C
                        ↘ D → E              (branching, limited connections)
Knowledge Graph:      A ↔ B ↔ C
                      ↕   ↕   ↕
                      D ↔ E ↔ F              (interconnected, evolving)
```

**Key benefits**:
- Captures relationships between concepts
- Supports non-linear discovery
- Enables "what depends on X?" queries
- Evolves without losing history

### Graph Database: CozoDB

PMSynapse uses **CozoDB** as its unified graph + vector database:

| Capability | How CozoDB Handles It |
|------------|----------------------|
| **Graph queries** | Datalog with recursion, path finding |
| **Vector search** | Built-in HNSW index for semantic similarity |
| **WASM support** | First-class browser deployment |
| **Combined queries** | "Find tasks related to auth, similar to 'security'" |

```datalog
// Example: Find all tasks affected by a decision
?[task, impact] :=
  *decisions[did, "Use WebSockets"],
  *impacts[did, component_id],
  *tasks[tid, task, _, component_id],
  impact = "direct"
```

---

## Part 2: The Journey from Idea to Implementation

### Full Lifecycle Coverage

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        PMSynapse: IDEA → IMPLEMENTATION                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. IDEATION           2. SPECIFICATION        3. ARCHITECTURE              │
│  ┌──────────────┐     ┌──────────────┐        ┌──────────────┐             │
│  │ "I want to   │ ──► │ PRD with     │ ──►    │ System       │             │
│  │  build X"    │     │ user stories │        │ design docs  │             │
│  └──────────────┘     └──────────────┘        └──────────────┘             │
│         │                    │                       │                      │
│         ▼                    ▼                       ▼                      │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │               SEMANTIC KNOWLEDGE GRAPH                       │           │
│  │  (All entities, relationships, decisions, rationale)         │           │
│  └─────────────────────────────────────────────────────────────┘           │
│         │                    │                       │                      │
│         ▼                    ▼                       ▼                      │
│  4. IMPLEMENTATION     5. TESTING              6. DEPLOYMENT               │
│  ┌──────────────┐     ┌──────────────┐        ┌──────────────┐             │
│  │ Code with    │ ◄─► │ Test cases   │ ◄─►    │ Release      │             │
│  │ linked docs  │     │ linked to    │        │ notes with   │             │
│  │              │     │ requirements │        │ full trace   │             │
│  └──────────────┘     └──────────────┘        └──────────────┘             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Everything is a Node

In PMSynapse, every artifact is a node in the knowledge graph:

| Node Type | Examples | Key Relationships |
|-----------|----------|-------------------|
| `Idea` | "Real-time collaboration" | `inspires` → Feature |
| `Feature` | "Live cursors" | `requires` → Task |
| `Task` | "Implement cursor sync" | `produces` → Code |
| `Decision` | "Use WebSockets not polling" | `impacts` → Architecture |
| `Question` | "How handle offline?" | `blocks` → Task |
| `Assumption` | "Users have stable internet" | `validates` → Decision |
| `Code` | `cursor-sync.ts` | `implements` → Feature |
| `Test` | `cursor.test.ts` | `verifies` → Code |
| `Document` | `architecture.md` | `describes` → Component |

### Graph Evolution Patterns

The graph evolves through defined patterns:

```
PATTERN 1: Decomposition
┌──────────┐          ┌──────────┐          ┌──────────┐
│  Epic    │ ─splits─►│ Feature  │ ─breaks─►│  Task    │
└──────────┘   into   └──────────┘   into   └──────────┘

PATTERN 2: Discovery
┌──────────┐          ┌──────────┐          ┌──────────┐
│  Task    │ ─raises─►│ Question │ ─leads──►│ Decision │
└──────────┘          └──────────┘    to    └──────────┘

PATTERN 3: Validation
┌──────────┐          ┌──────────┐          ┌──────────┐
│Assumption│ ─tested─►│   Test   │ ─yields─►│ Evidence │
└──────────┘    by    └──────────┘          └──────────┘

PATTERN 4: Implementation
┌──────────┐          ┌──────────┐          ┌──────────┐
│  Task    │─produces►│   Code   │ ─builds─►│ Artifact │
└──────────┘          └──────────┘          └──────────┘
```

---

## Part 3: Assumption Confidence System

### Design Principle

Users should always know **how much the AI guessed** vs. what's confirmed. Every assumption has a probability score.

### Node Structure

```rust
struct Assumption {
    id: Uuid,
    content: String,              // "This service uses PostgreSQL"
    confidence: f32,              // 0.0 - 1.0
    source: InferenceSource,      // How we know this
    status: AssumptionStatus,     // Unconfirmed | Confirmed | Denied | Superseded
    evidence: Vec<Evidence>,      // What supports this
    created_by: AgentId,          // "agent:analyzer"
    confirmed_by: Option<UserId>, // "user:alice"
}

enum InferenceSource {
    ExplicitConfig,     // Base: 0.95 - Found in config files
    DependencyAnalysis, // Base: 0.85 - Import/dependency detection
    PatternMatching,    // Base: 0.70 - Code pattern recognition
    NamingConvention,   // Base: 0.55 - File/variable naming
    AiInference,        // Base: 0.40 - LLM reasoning
    StructuralSimilarity, // Base: 0.30 - "Looks like" analysis
}
```

### Confidence Calculation

```
BASE CONFIDENCE (from source type)
  + Corroborating evidence:     +0.15 per additional source
  + Consistency across files:   +0.10
  - Contradictory signals:      -0.20
  - Single occurrence:          -0.10
  - Outdated pattern:           -0.15
  = FINAL CONFIDENCE
```

### Visual Indicators

```
90-100%  🟢 SOLID      "Confirmed by config/user"
70-89%   🟡 LIKELY     "Strong evidence, not confirmed"
50-69%   🟠 UNCERTAIN  "Multiple signals, some conflict"
30-49%   🔴 GUESS      "AI inference, needs validation"
0-29%    ⚫ UNKNOWN    "Placeholder, requires human input"
```

### In Documentation Display

```markdown
# Architecture

The system uses PostgreSQL 🟡73% for persistence.
Authentication is handled via JWT 🟢95%.
The caching layer appears to be Redis 🔴35%.

> ⚠️ 3 assumptions need confirmation
```

---

## Part 4: Dual Representation (Text + Graph)

### Synchronized Documentation

Every piece of documentation exists in TWO forms that stay synchronized:

```
┌────────────────────────────────────────────────────────────────┐
│                    DUAL REPRESENTATION                          │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  HUMAN VIEW (Markdown)              GRAPH VIEW (Semantic)      │
│  ┌─────────────────────────┐       ┌─────────────────────────┐ │
│  │ # Authentication        │ ◄───► │ (Auth)──implements──►   │ │
│  │                         │       │    │     (JWT)           │ │
│  │ We use JWT tokens for   │       │    │                     │ │
│  │ authentication. Tokens  │       │    ├──requires──►(Secret)│ │
│  │ expire after 24 hours.  │       │    │                     │ │
│  │                         │       │    └──expires──►(24h)    │ │
│  │ ## Why JWT?             │       │                          │ │
│  │ Decision: Stateless...  │       │ (Decision)──chose──►(JWT)│ │
│  │                         │       │     └──rejected──►(Session)│
│  └─────────────────────────┘       └─────────────────────────┘ │
│                                                                 │
└────────────────────────────────────────────────────────────────┘
```

### Explicit Sync Mechanism

Sync is **explicit** via `pms sync` command (not automatic):

```
┌─────────────────────────────────────────────────────────────────┐
│              EXPLICIT DOCUMENTATION ↔ GRAPH SYNC                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│    ┌──────────────┐                    ┌──────────────┐         │
│    │  Markdown    │                    │   Graph      │         │
│    │  Documents   │                    │   (CozoDB)   │         │
│    └──────────────┘                    └──────────────┘         │
│           │                                    │                 │
│           │ (Edit freely)                      │                 │
│           ▼                                    │                 │
│    ┌──────────────┐                           │                 │
│    │  Pending     │◄─── Diff tracked          │                 │
│    │  Changes     │                           │                 │
│    └──────────────┘                           │                 │
│           │                                    │                 │
│           │ User: `pms sync`                  │                 │
│           ▼                                    ▼                 │
│    ┌─────────────────────────────────────────────────────┐     │
│    │                 SYNC PREVIEW                         │     │
│    │                                                      │     │
│    │  Changes detected in: docs/architecture.md          │     │
│    │                                                      │     │
│    │  GRAPH UPDATES:                                     │     │
│    │  + ADD node: Component("CacheService")              │     │
│    │  + ADD edge: CacheService --uses--> Redis           │     │
│    │  ~ UPDATE: AuthService.description                  │     │
│    │  - REMOVE edge: API --calls--> DeprecatedService    │     │
│    │                                                      │     │
│    │  [Apply All] [Review Each] [Cancel]                 │     │
│    └─────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### CLI Commands

```bash
pms sync              # Preview and apply changes
pms sync --dry-run    # Preview only
pms sync --force      # Apply without preview
pms sync --file X.md  # Sync specific file
pms diff              # Show pending changes
pms graph export      # Export graph to JSON
pms graph query "..."  # Run Datalog query
```

---

## Part 5: Agent Proposal & Approval Workflow

### Design Principle

Agents can **propose** changes to the knowledge graph, but humans **approve** them. This maintains oversight while enabling AI assistance.

### Proposal Lifecycle

```
DRAFT ──► PROPOSED ──► REVIEWED ──► APPROVED ──► APPLIED
              │             │            │
              │             │            └──► REJECTED
              │             └──► NEEDS_CHANGES
              └──► WITHDRAWN
```

### Proposal Structure

```rust
struct Proposal {
    id: ProposalId,
    proposal_type: ProposalType,      // GraphModification, DocUpdate, etc.
    agent: AgentId,                   // "agent:architecture-analyzer"
    confidence: f32,                  // Agent's confidence in this change
    rationale: String,                // Why this change is proposed
    changes: Vec<GraphChange>,        // What to change
    evidence: Vec<EvidenceRef>,       // Supporting evidence
    impact_analysis: ImpactAnalysis,  // What's affected
    created_at: Timestamp,
    expires_at: Timestamp,            // Auto-expire if not reviewed
}

struct GraphChange {
    operation: Operation,  // AddNode, UpdateNode, RemoveNode, AddEdge, etc.
    target: NodeOrEdge,
    old_value: Option<Value>,
    new_value: Value,
}
```

### Approval UI

```
┌──────────────────────────────────────────────────────────────┐
│ 📋 PENDING PROPOSALS (3)                                     │
│                                                              │
│ ┌──────────────────────────────────────────────────────────┐│
│ │ 🤖 architecture-analyzer • 2 hours ago                   ││
│ │                                                          ││
│ │ "Add PaymentService component to architecture"           ││
│ │                                                          ││
│ │ Confidence: ████████░░ 78%                               ││
│ │                                                          ││
│ │ Changes:                                                 ││
│ │  + Node: PaymentService (component)                      ││
│ │  + Edge: PaymentService → Stripe (integrates)            ││
│ │  + Edge: OrderService → PaymentService (calls)           ││
│ │                                                          ││
│ │ Evidence:                                                ││
│ │  • New file: src/services/payment.ts                     ││
│ │  • Import in: src/services/order.ts                      ││
│ │  • Config: stripe key in .env.example                    ││
│ │                                                          ││
│ │ [✓ Approve] [✏️ Edit] [❌ Reject] [💬 Comment]           ││
│ └──────────────────────────────────────────────────────────┘│
└──────────────────────────────────────────────────────────────┘
```

### CLI Commands

```bash
pms proposals                    # List pending proposals
pms proposals --agent X          # Filter by agent
pms proposals approve <id>       # Approve one
pms proposals approve --all-high # Approve all ≥80% confidence
pms proposals reject <id> -m "reason"
pms proposals auto-approve --threshold 0.9  # Set auto-approve rule
```

### Auto-Approval Rules (Optional)

```yaml
# .pmsynapse/approval-rules.yaml

rules:
  - agent: "dependency-tracker"
    auto_approve_if:
      confidence: ">= 0.95"
      change_type: "add_dependency"

  - agent: "doc-generator"
    auto_approve_if:
      confidence: ">= 0.90"
      affects_nodes: "<= 3"

  - agent: "*"
    require_human: true  # Default: always require review
```

---

## Part 6: Legacy Codebase Analysis

### Bootstrap Pipeline

When PMSynapse is installed on an existing codebase, it runs a 4-phase analysis:

```
┌─────────────────────────────────────────────────────────────────┐
│              LEGACY CODEBASE ANALYSIS PIPELINE                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  PHASE 1: SCAN                                                  │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ • File structure analysis (detect patterns)            │     │
│  │ • Dependency graph extraction (package.json, imports)  │     │
│  │ • API surface detection (routes, endpoints)            │     │
│  │ • Architecture pattern recognition                     │     │
│  │   (MVC? Microservices? Monolith? Hexagonal?)          │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│                          ▼                                       │
│  PHASE 2: INFER ASSUMPTIONS                                     │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Create assumption nodes with confidence scores:         │     │
│  │                                                         │     │
│  │ [ASSUMPTION: 87%] "This appears to be a REST API"      │     │
│  │   └─ Evidence: Express routes, HTTP verbs in handlers  │     │
│  │                                                         │     │
│  │ [ASSUMPTION: 65%] "Authentication uses JWT"            │     │
│  │   └─ Evidence: jsonwebtoken in deps, token patterns    │     │
│  │                                                         │     │
│  │ [ASSUMPTION: 43%] "Database is PostgreSQL"             │     │
│  │   └─ Evidence: pg in deps, but no explicit config      │     │
│  │                                                         │     │
│  │ [UNKNOWN: ???] "Purpose of /legacy/utils/helper.js"    │     │
│  │   └─ Action: Flag for human documentation              │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│                          ▼                                       │
│  PHASE 3: HUMAN INTERACTION                                     │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Present assumptions as questions:                       │     │
│  │                                                         │     │
│  │ "I detected JWT tokens. Is this your auth system?"     │     │
│  │   [✓ Confirm] [✗ Deny] [📝 Clarify]                    │     │
│  │                                                         │     │
│  │ "What does UserService.reconcile() do?"                │     │
│  │   [📝 Add documentation]                               │     │
│  │                                                         │     │
│  │ "Found 3 undocumented API endpoints. Document now?"    │     │
│  │   [📝 Document] [⏭️ Skip] [🔇 Ignore pattern]          │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│                          ▼                                       │
│  PHASE 4: KNOWLEDGE GRAPH POPULATION                            │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ • Confirmed assumptions → Fact nodes (confidence: 1.0) │     │
│  │ • Human clarifications → Documentation nodes           │     │
│  │ • Unknowns → Question nodes (for later resolution)     │     │
│  │ • Code files → linked to inferred components           │     │
│  │ • Dependencies → edges in graph                        │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### CLI Commands for Legacy Analysis

```bash
pms init                     # Initialize on existing codebase
pms analyze                  # Run full analysis
pms analyze --quick          # Fast scan, fewer assumptions
pms analyze --deep           # Deep analysis, more AI inference
pms assumptions              # List all assumptions
pms assumptions --unconfirmed # Show only unconfirmed
pms confirm <id>             # Confirm an assumption
pms deny <id> --correct "actual value"  # Deny and correct
pms questions                # Show questions needing answers
```

---

## Part 7: Multi-Provider LLM Integration

### OpenRouter-Style Architecture

PMSynapse integrates with multiple LLM providers through a unified interface, inspired by [OpenRouter](https://openrouter.ai/):

```
┌─────────────────────────────────────────────────────────────────┐
│              MULTI-PROVIDER LLM INTEGRATION                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  APPLICATION LAYER                                              │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  PMSynapse Agents                                       │     │
│  │  (Analyzer, Architect, Coder, Reviewer, etc.)          │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│                          ▼                                       │
│  UNIFIED LLM INTERFACE                                          │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  LlmClient {                                            │     │
│  │    fn complete(prompt, config) -> Response             │     │
│  │    fn stream(prompt, config) -> Stream<Chunk>          │     │
│  │    fn embed(text) -> Vec<f32>                          │     │
│  │  }                                                      │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│                          ▼                                       │
│  ROUTER LAYER                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  • Model selection (by task type, cost, speed)         │     │
│  │  • Automatic fallback on provider errors               │     │
│  │  • Load balancing across providers                     │     │
│  │  • Cost tracking and budgeting                         │     │
│  │  • Rate limiting and retry logic                       │     │
│  └────────────────────────────────────────────────────────┘     │
│                          │                                       │
│     ┌────────────────────┼────────────────────┐                 │
│     │                    │                    │                 │
│     ▼                    ▼                    ▼                 │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐              │
│  │ OpenAI   │      │ Anthropic│      │  Google  │              │
│  │ GPT-4o   │      │ Claude   │      │ Gemini   │              │
│  └──────────┘      └──────────┘      └──────────┘              │
│     │                    │                    │                 │
│     ▼                    ▼                    ▼                 │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐              │
│  │ Ollama   │      │ Together │      │ Groq     │              │
│  │ (Local)  │      │   AI     │      │          │              │
│  └──────────┘      └──────────┘      └──────────┘              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration

```yaml
# .pmsynapse/llm-config.yaml

providers:
  openai:
    api_key: ${OPENAI_API_KEY}
    models:
      - gpt-4o
      - gpt-4o-mini

  anthropic:
    api_key: ${ANTHROPIC_API_KEY}
    models:
      - claude-sonnet-4-20250514
      - claude-3-5-haiku-20241022

  google:
    api_key: ${GOOGLE_API_KEY}
    models:
      - gemini-1.5-pro
      - gemini-1.5-flash

  ollama:
    base_url: http://localhost:11434
    models:
      - llama3.2
      - codellama

  openrouter:
    api_key: ${OPENROUTER_API_KEY}
    # Access to 300+ models via single key

routing:
  default_model: "anthropic/claude-sonnet-4-20250514"

  task_routing:
    code_generation:
      primary: "anthropic/claude-sonnet-4-20250514"
      fallback: "openai/gpt-4o"

    analysis:
      primary: "openai/gpt-4o"
      fallback: "google/gemini-1.5-pro"

    quick_tasks:
      primary: "anthropic/claude-3-5-haiku-20241022"
      fallback: "openai/gpt-4o-mini"

    embedding:
      primary: "openai/text-embedding-3-small"
      fallback: "local/nomic-embed-text"

  fallback:
    enabled: true
    max_retries: 3
    retry_delay_ms: 1000

budget:
  daily_limit_usd: 50.00
  alert_threshold: 0.8  # Alert at 80% of budget
  cost_tracking: true
```

### Provider Abstraction

```rust
// Unified provider trait
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    async fn stream(&self, request: CompletionRequest) -> Result<impl Stream<Item = Chunk>>;
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
    fn supported_models(&self) -> &[ModelInfo];
    fn cost_per_token(&self, model: &str) -> TokenCost;
}

// Implementations for each provider
pub struct OpenAiProvider { client: OpenAiClient }
pub struct AnthropicProvider { client: AnthropicClient }
pub struct GoogleProvider { client: GoogleClient }
pub struct OllamaProvider { base_url: Url }
pub struct OpenRouterProvider { client: OpenRouterClient }  // Meta-provider
```

---

## Part 8: Customizable Conversational Rules

### Template System

PMSynapse supports **customizable conversational rules** through YAML/Markdown templates. Different teams can use different methodologies.

### Starting Point: BMAD Method

PMSynapse ships with templates based on [BMAD Method](https://github.com/bmad-code-org/BMAD-METHOD) (Breakthrough Method for Agile AI-Driven Development):

```
┌─────────────────────────────────────────────────────────────────┐
│                    BMAD METHOD INTEGRATION                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  CORE CONCEPTS                                                  │
│  ├── 21 specialized AI agents across 4 modules                  │
│  ├── 50+ guided workflows                                       │
│  ├── Scale-adaptive intelligence (task complexity → depth)      │
│  └── 4-phase lifecycle: Analysis → Planning → Design → Build    │
│                                                                  │
│  AGENT PERSONAS                                                 │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  Agent       │ Role            │ Personality           │     │
│  │  ────────────┼─────────────────┼─────────────────────  │     │
│  │  PM          │ Requirements    │ User-focused, clear   │     │
│  │  Architect   │ System design   │ Pragmatic, thorough   │     │
│  │  Developer   │ Implementation  │ Precise, efficient    │     │
│  │  Analyst     │ Research        │ Curious, detailed     │     │
│  │  UX Designer │ User experience │ Empathetic, creative  │     │
│  │  Tester      │ Quality         │ Skeptical, methodical │     │
│  │  Tech Writer │ Documentation   │ Clear, structured     │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
│  WORKFLOW TRACKS                                                │
│  ├── Quick Flow:    < 5 min  (bug fixes, small changes)        │
│  ├── BMad Method:   < 15 min (features, products)              │
│  └── Enterprise:    < 30 min (compliance-heavy systems)        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Template Structure

```
.pmsynapse/
├── templates/
│   ├── bmad/                      # Default: BMAD method
│   │   ├── agents/
│   │   │   ├── pm.md              # PM persona definition
│   │   │   ├── architect.md       # Architect persona
│   │   │   ├── developer.md       # Developer persona
│   │   │   └── ...
│   │   ├── workflows/
│   │   │   ├── quick-fix.yaml     # Quick fix workflow
│   │   │   ├── new-feature.yaml   # New feature workflow
│   │   │   ├── greenfield.yaml    # New project workflow
│   │   │   └── ...
│   │   └── phases/
│   │       ├── analysis.yaml
│   │       ├── planning.yaml
│   │       ├── solutioning.yaml
│   │       └── implementation.yaml
│   │
│   └── custom/                    # Team customizations
│       └── my-team/
│           ├── agents/
│           └── workflows/
│
└── config.yaml                    # Active template selection
```

### Agent Definition Format

```markdown
<!-- .pmsynapse/templates/bmad/agents/architect.md -->

# Architect Agent

## Identity
You are the System Architect for this project. You design robust,
scalable systems that meet both functional and non-functional requirements.

## Personality
- **Communication style**: Pragmatic, thorough, asks clarifying questions
- **Decision-making**: Evidence-based, considers trade-offs explicitly
- **Documentation**: Creates ADRs, component diagrams, sequence diagrams

## Core Principles
1. Design for change - systems evolve
2. Explicit trade-offs - no perfect solutions
3. Document decisions - future you will thank you
4. Start simple - complexity is earned

## Commands
- `/design <component>` - Create component design
- `/adr <decision>` - Generate Architecture Decision Record
- `/review <design>` - Review existing design
- `/diagram <type>` - Generate architecture diagram

## Dependencies
- Requires: PRD from PM agent
- Produces: Architecture docs, ADRs, component diagrams
- Feeds into: Developer agent, Tester agent
```

### Workflow Definition Format

```yaml
# .pmsynapse/templates/bmad/workflows/new-feature.yaml

name: New Feature Workflow
description: Complete workflow for implementing a new feature
track: bmad_method  # quick_flow | bmad_method | enterprise
estimated_time: 15min

phases:
  - id: analysis
    agent: analyst
    tasks:
      - gather_requirements
      - competitive_analysis
      - identify_risks
    outputs:
      - analysis_brief

  - id: planning
    agent: pm
    depends_on: [analysis]
    tasks:
      - write_user_stories
      - define_acceptance_criteria
      - prioritize_scope
    outputs:
      - prd
      - user_stories

  - id: design
    agent: architect
    depends_on: [planning]
    tasks:
      - component_design
      - api_design
      - data_model
    outputs:
      - architecture_doc
      - adr_records

  - id: implementation
    agent: developer
    depends_on: [design]
    tasks:
      - setup_scaffolding
      - implement_core
      - write_tests
    outputs:
      - code
      - tests

  - id: review
    agent: reviewer
    depends_on: [implementation]
    tasks:
      - code_review
      - security_check
      - performance_check
    outputs:
      - review_report

checkpoints:
  - after: planning
    requires_approval: true
    message: "PRD ready for review. Continue to design?"

  - after: design
    requires_approval: true
    message: "Architecture approved? Continue to implementation?"
```

### Customization Examples

```yaml
# .pmsynapse/config.yaml

template:
  base: bmad                    # Start with BMAD
  overrides:
    - custom/my-team            # Apply team customizations

agents:
  architect:
    personality:
      communication_style: "Very terse, bullet points only"
    extra_principles:
      - "Always consider GDPR compliance"
      - "Prefer serverless over containers"

workflows:
  new-feature:
    add_phase:
      - id: compliance_review
        agent: compliance_officer
        after: design
        tasks:
          - gdpr_check
          - security_audit

    checkpoints:
      - after: compliance_review
        requires_approval: true
        approvers: ["security-team"]

# Team-specific agent additions
custom_agents:
  - id: compliance_officer
    template: .pmsynapse/templates/custom/compliance.md
```

### CLI Commands for Templates

```bash
pms templates list              # List available templates
pms templates use bmad          # Switch to BMAD method
pms templates use custom/my-team # Use custom template
pms templates create my-new     # Create new template from current
pms templates export            # Export current config
pms templates validate          # Validate template syntax

pms workflow list               # List available workflows
pms workflow run new-feature    # Start a workflow
pms workflow status             # Show current workflow progress
pms workflow skip <phase>       # Skip a phase (with confirmation)
```

---

## Part 9: Summary of Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Graph Database** | CozoDB unified | Single DB for graph + vector, WASM support |
| **Assumption confidence** | 0.0-1.0 probability | Users see AI guessing level + evidence |
| **Doc ↔ Graph sync** | Explicit `pms sync` | Prevents noise, user controls timing |
| **Agent changes** | Proposal → Approval | Audit trail, human oversight, batch ops |
| **LLM integration** | OpenRouter-style multi-provider | Flexibility, fallback, cost control |
| **Conversational rules** | Template-based (BMAD default) | Team customization, methodology flexibility |
| **Legacy analysis** | 4-phase with human validation | Balance AI speed with human accuracy |

---

## Sources

- [Knowledge Graph of Thoughts](https://github.com/spcl/knowledge-graph-of-thoughts) - ETH Zurich
- [BMAD Method](https://github.com/bmad-code-org/BMAD-METHOD) - AI-driven agile framework
- [OpenRouter](https://openrouter.ai/) - Multi-provider LLM routing
- [CozoDB](https://github.com/cozodb/cozo) - Graph + Vector database

---

*Document version: 1.0*
*Created: December 2025*
*Part of: PMSynapse Architecture Documentation*
