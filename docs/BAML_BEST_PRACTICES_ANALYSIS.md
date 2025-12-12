# BAML Best Practices Analysis

## Overview

This document analyzes **BAML (Basically a Made-up Language)** by BoundaryML, extracting architectural patterns and best practices applicable to PMSynapse's Rust + multi-platform strategy.

**Repository**: [BoundaryML/baml](https://github.com/BoundaryML/baml)

---

## What is BAML?

BAML is a prompting framework that transforms "prompt engineering" into **schema engineering**. It treats LLM prompts as typed functions with structured inputs and outputs.

> "The goal of BAML is to give you the expressiveness of English, but the structure of code."

### Core Philosophy

| Principle | Description |
|-----------|-------------|
| **Minimize invention** | Use existing standards where possible |
| **Standard tooling** | Require only editors and terminals |
| **Performance first** | Built in Rust for speed |
| **Accessibility** | Understandable by first-year CS students |

---

## Turbo (Turborepo) Usage in BAML

### What is Turborepo?

[Turborepo](https://turbo.build/) is a high-performance build system for JavaScript/TypeScript monorepos developed by Vercel. BAML uses it for:

1. **Task orchestration** across their multi-language monorepo
2. **Remote caching** to share build artifacts across CI and developers
3. **Parallel execution** of independent build tasks
4. **Dependency graph** management between packages

### How BAML Uses Turbo

```
┌─────────────────────────────────────────────────────────────────┐
│                    BAML MONOREPO BUILD SYSTEM                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  TURBOREPO (Orchestration Layer)                               │
│  ├── Remote cache (Vercel)                                     │
│  ├── Task dependency graph                                     │
│  ├── Parallel execution (~29 concurrent jobs)                  │
│  └── Incremental builds                                        │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  RUST CORE (Compiler + Runtime)                          │   │
│  │  └── Compiled via cargo, cached by Turbo                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                          │                                      │
│     ┌────────────────────┼────────────────────┐                │
│     │                    │                    │                │
│     ▼                    ▼                    ▼                │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐            │
│  │ TypeScript│      │  Python  │      │   Ruby   │            │
│  │  (NAPI)  │      │  (PyO3)  │      │ (rb-sys) │            │
│  └──────────┘      └──────────┘      └──────────┘            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Turbo Benefits for BAML

| Benefit | Impact |
|---------|--------|
| **Remote caching** | Build artifacts shared across CI runs and developers |
| **Incremental builds** | Only rebuild changed packages |
| **Parallelization** | ~29 concurrent build jobs |
| **Dependency awareness** | Correct build order across languages |

### Example turbo.json Pattern

```json
{
  "$schema": "https://turbo.build/schema.json",
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", "*.node"]
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": []
    },
    "lint": {
      "outputs": []
    }
  }
}
```

---

## Architecture: Rust Core + Multi-Language Bindings

### The "Write Once, Bind Everywhere" Pattern

BAML's architecture is highly relevant to PMSynapse:

```
┌─────────────────────────────────────────────────────────────────┐
│                    BAML COMPILER ARCHITECTURE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  SOURCE (.baml files)                                           │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  PEST PARSER                                              │    │
│  │  └── Grammar-based parsing                                │    │
│  └─────────────────────────────────────────────────────────┘    │
│       │                                                          │
│       ▼                                                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  COMPILATION PIPELINE                                     │    │
│  │  Source → AST → HIR → THIR → Output                       │    │
│  └─────────────────────────────────────────────────────────┘    │
│       │                                                          │
│       ├───────────────────┬──────────────────┐                  │
│       │                   │                  │                  │
│       ▼                   ▼                  ▼                  │
│  [Bytecode]         [TypeScript]        [Python]               │
│  (Runtime)          (baml_client)       (baml_client)          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Multi-Platform Build Targets

BAML compiles to **8+ platforms** per language:

| Platform | TypeScript | Python | Ruby |
|----------|------------|--------|------|
| macOS ARM64 | ✅ | ✅ | ✅ |
| macOS Intel | ✅ | ✅ | ✅ |
| Linux ARM64 | ✅ | ✅ | ✅ |
| Linux Intel | ✅ | ✅ | ✅ |
| Linux musl | ✅ | ✅ | - |
| Windows | ✅ | ✅ | - |

### FFI Bindings Pattern

```rust
// Core Rust implementation
pub struct BamlRuntime {
    // Parsing, type checking, execution
}

impl BamlRuntime {
    pub fn execute(&self, function: &str, args: Value) -> Result<Value> {
        // Core logic - shared across all platforms
    }
}

// TypeScript binding (NAPI-RS)
#[napi]
pub fn execute(function: String, args: JsObject) -> napi::Result<JsObject> {
    let runtime = get_runtime();
    runtime.execute(&function, args.into())
}

// Python binding (PyO3)
#[pyfunction]
fn execute(py: Python, function: &str, args: PyObject) -> PyResult<PyObject> {
    let runtime = get_runtime();
    runtime.execute(function, args.into())
}
```

---

## Key Best Practices from BAML

### 1. Schema Engineering > Prompt Engineering

Instead of string manipulation:

```python
# Bad: String soup
prompt = f"Extract {field} from {text} and return as JSON"
```

Use typed schemas:

```baml
// Good: Schema-driven
function ExtractInfo(text: string) -> Info {
  client GPT4
  prompt #"
    Extract information from: {{ text }}

    {{ ctx.output_format }}
  "#
}

class Info {
  name string
  email string
  phone string?
}
```

### 2. Schema-Aligned Parsing (SAP)

BAML's SAP algorithm handles LLM output variations:

| Challenge | SAP Solution |
|-----------|--------------|
| Markdown in JSON | Strips formatting, extracts structure |
| Chain-of-thought | Ignores reasoning, extracts answer |
| Partial outputs | Streams typed chunks progressively |
| No tool-calling | Works without native JSON mode |

### 3. Functions as First-Class Citizens

Every prompt is a typed function:

```baml
function ClassifySentiment(text: string) -> Sentiment {
  client Claude
  prompt #"
    Classify the sentiment of: {{ text }}

    {{ ctx.output_format }}
  "#
}

enum Sentiment {
  POSITIVE
  NEGATIVE
  NEUTRAL
}
```

### 4. Multi-Model Portability

Single definition works across models:

```baml
client<llm> GPT4 {
  provider openai
  model gpt-4o
}

client<llm> Claude {
  provider anthropic
  model claude-3-5-sonnet
}

client<llm> Gemini {
  provider google
  model gemini-1.5-pro
}

// Same function, any client
function Summarize(doc: string) -> Summary {
  client GPT4  // Easily swappable
  ...
}
```

---

## Build System Patterns to Adopt

### 1. Monorepo with Turborepo

```
pmsynapse/
├── turbo.json              # Turbo configuration
├── pnpm-workspace.yaml     # Workspace definition
├── crates/
│   └── pmsynapse-core/     # Rust core library
├── packages/
│   ├── cli/                # CLI tool (Rust)
│   ├── typescript/         # TypeScript SDK
│   └── python/             # Python SDK (future)
└── apps/
    ├── desktop/            # Tauri app (Phase 2)
    └── web/                # WASM app (Phase 3)
```

### 2. Remote Caching Strategy

```json
// turbo.json
{
  "remoteCache": {
    "signature": true  // Verify cache integrity
  },
  "pipeline": {
    "build": {
      "cache": true,
      "outputs": ["dist/**", "target/**"]
    }
  }
}
```

### 3. Cross-Platform CI Matrix

```yaml
# .github/workflows/build.yml
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
```

---

## Relevance to PMSynapse

### Direct Applicability

| BAML Pattern | PMSynapse Application |
|--------------|----------------------|
| **Rust core + bindings** | Same architecture for CLI → WASM → Desktop |
| **Turborepo** | Coordinate Rust, TypeScript, React builds |
| **Schema engineering** | Type-safe task/thought definitions |
| **SAP parsing** | Handle varied LLM outputs for agents |
| **Multi-platform CI** | Build for desktop + web + CLI |

### Recommended Adoption

#### Phase 1 (CLI - Current)

```
pmsynapse/
├── turbo.json
├── crates/
│   ├── pmsynapse-core/     # Core library
│   └── pmsynapse-cli/      # CLI binary
└── packages/
    └── types/              # Shared TypeScript types
```

#### Phase 2 (Desktop)

Add Tauri app, share core with CLI.

#### Phase 3 (WASM)

Same core compiles to browser via `wasm-pack`.

---

## Key Learnings Summary

### Why BAML Succeeded

1. **Rust performance** - Compiler so fast users don't notice it
2. **Type safety everywhere** - From schema to generated client
3. **Model independence** - Works Day-1 with new models
4. **Developer experience** - VSCode integration, instant feedback
5. **Build optimization** - Turborepo caching saves hours

### Anti-Patterns to Avoid

| Anti-Pattern | Better Approach |
|--------------|-----------------|
| String-based prompts | Schema-driven definitions |
| Single-language focus | Multi-platform from start |
| Manual build orchestration | Turborepo automation |
| Platform-specific code in core | Trait-based abstraction |

---

## Sources

- [BoundaryML/baml GitHub](https://github.com/BoundaryML/baml)
- [Turborepo Documentation](https://turbo.build/repo/docs)
- [BAML Multi-Platform Build System - DeepWiki](https://deepwiki.com/BoundaryML/baml/8.2-multi-platform-build-system)
- [Language Clients and FFI - DeepWiki](https://deepwiki.com/BoundaryML/baml/3-language-clients-and-ffi)

---

*Analysis completed: December 2025*
*Part of: PMSynapse AI-Enabled Project Management Research*
