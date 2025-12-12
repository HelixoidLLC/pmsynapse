# Planned Features

This document tracks features that are planned for future implementation.

---

## WASM Browser Support

**Status**: Planned
**Priority**: Medium
**Blocked By**: CozoDB/SQLite WASM compatibility

### Description

Add WebAssembly (WASM) support to enable PMSynapse to run directly in web browsers without requiring the Tauri desktop application.

### Why It Was Deferred

The `snps-wasm` crate was removed in commit `97a2b91` due to dependency conflicts:

1. **CozoDB doesn't compile to WASM** - CozoDB relies on RocksDB/SQLite backends that require native compilation
2. **rusqlite with bundled SQLite** - The `bundled` feature compiles native C code, incompatible with WASM
3. **Dependency version conflicts** - rayon + graph_builder had incompatible versions causing compilation failures

### Implementation Path

When revisiting WASM support, consider these approaches:

#### Option A: Standalone WASM Module
Create a lightweight `snps-wasm` crate that:
- Uses IndexedDB for browser storage (via `idb` crate)
- Implements graph operations in pure Rust (no CozoDB)
- Shares type definitions with `snps-core` but not implementation

#### Option B: Use sql.js
- Compile SQLite to WASM using [sql.js](https://github.com/sql-js/sql.js)
- Bridge from Rust WASM to JavaScript SQLite
- More complex but maintains SQLite compatibility

#### Option C: Wait for CozoDB WASM Support
- Monitor CozoDB project for WASM support
- May require upstream contributions

### Acceptance Criteria

- [ ] `snps-wasm` crate compiles to `wasm32-unknown-unknown` target
- [ ] Core IDLC operations work in browser
- [ ] Knowledge graph queries work (may be limited)
- [ ] Browser storage persists across sessions
- [ ] Bundle size < 2MB gzipped

### References

- Original removal commit: `97a2b91`
- CozoDB: https://github.com/cozodb/cozo
- sql.js: https://github.com/sql-js/sql.js
- idb crate: https://crates.io/crates/idb

---

## CozoDB Integration

**Status**: Planned
**Priority**: High
**Blocked By**: Upstream dependency conflicts (rayon + graph_builder)

### Description

Replace rusqlite with CozoDB for unified graph + vector database capabilities.

### Why It Was Deferred

CozoDB 0.7.6 has dependency conflicts:
- `graph_builder` crate incompatible with newer `rayon` versions
- Causes compilation errors with `par_iter()` type mismatches

### Implementation Path

1. Monitor CozoDB releases for dependency updates
2. When compatible, update `Cargo.toml`:
   ```toml
   cozo = { version = "0.8+", features = ["storage-sqlite", "graph-algo"] }
   ```
3. Migrate `snps-core/src/graph/mod.rs` to use CozoDB Datalog queries
4. Add vector search capabilities (HNSW index)

### Benefits

- Datalog queries for graph traversal
- Built-in vector similarity search
- Single database for all storage needs
- Better query expressiveness than raw SQL
