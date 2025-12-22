# Rust Engine - Code Conventions

**CRITICAL**: These conventions MUST be followed when working with Rust code in this directory.

## Module Organization

### Use Named Files (Rust 2018+)
```
src/graph.rs              # ✅ CORRECT
src/graph/mod.rs          # ❌ WRONG - avoid mod.rs
```

### Directories Only for Submodules
Use directory structure only when a module has child modules:
```
src/
├── graph.rs              # Parent module
└── graph/
    ├── node.rs           # graph::node submodule
    └── edge.rs           # graph::edge submodule
```

**Rule**: If a module has no submodules, use a single `.rs` file. Never create `mod.rs` unnecessarily.
