# Rust Graph & Vector Database Options

## Research Summary

This document evaluates in-memory, lightweight graph databases for Rust that also support vector database functionality. Focus is on options that can compile to WASM for browser deployment.

---

## Executive Summary: Top Recommendations

| Database | Graph | Vector | WASM | In-Memory | Best For |
|----------|-------|--------|------|-----------|----------|
| **CozoDB** | ✅ Datalog | ✅ HNSW | ✅ Full | ✅ Yes | **Best overall for PMSynapse** |
| **SurrealDB** | ✅ Full | ✅ KNN/HNSW | ✅ Yes | ✅ Yes | Multi-model flexibility |
| **agdb** | ✅ Native | ❌ No | ❓ Untested | ✅ Memory-mapped | Pure graph, native queries |
| **Voy** | ❌ No | ✅ k-d tree | ✅ Designed for | ✅ Yes | Lightweight vector-only |
| **MemVDB** | ❌ No | ✅ Multiple | ❓ Possible | ✅ Yes | Simple vector search |

**Recommendation**: **CozoDB** for PMSynapse - it has graph, vector, WASM support, and is written entirely in Rust.

---

## Detailed Analysis

### 1. CozoDB (⭐ Top Pick)

**Repository**: [cozodb/cozo](https://github.com/cozodb/cozo)

A transactional, relational-graph-vector database using Datalog for queries. Described as "The hippocampus for AI!"

#### Features

| Feature | Support |
|---------|---------|
| **Graph queries** | ✅ Full Datalog with recursion |
| **Vector search** | ✅ HNSW (custom Rust implementation) |
| **WASM** | ✅ Full support (`wasm` feature flag) |
| **In-memory** | ✅ Yes |
| **Persistent** | ✅ RocksDB, SQLite backends |
| **Platforms** | Linux, macOS, Windows, iOS, Android, Browser |

#### Why CozoDB for PMSynapse

```rust
// Cargo.toml
[dependencies]
cozo = { version = "0.7", features = ["storage-mem"] }

// For WASM
cozo = { version = "0.7", features = ["wasm", "nothread"] }
```

**Graph + Vector in One Query**:
```datalog
# Find similar thoughts and their relationships
?[thought, related, similarity] :=
    ~thoughts:hnsw{id: thought, vec: $query_embedding | k: 10, ef: 50},
    *thought_relations{from: thought, to: related, type: "related_to"},
    similarity = cos_dist(vec, $query_embedding)
```

**Key Advantages**:
- HNSW index written from scratch in Rust (not a wrapper)
- Disk-based with MVCC support
- Vector search integrated into Datalog queries
- Minimal memory usage (Rust RAII)
- Near-native WASM performance

#### Storage Backends

| Backend | Use Case |
|---------|----------|
| `storage-mem` | In-memory only |
| `storage-sqlite` | Lightweight persistence |
| `storage-rocksdb` | High-performance persistence |

---

### 2. SurrealDB

**Repository**: [surrealdb/surrealdb](https://github.com/surrealdb/surrealdb)

Multi-model database: relational, document, graph, time-series, vector, and geospatial.

#### Features

| Feature | Support |
|---------|---------|
| **Graph queries** | ✅ Full graph with edges |
| **Vector search** | ✅ HNSW, KNN operators |
| **WASM** | ✅ Runs in browser |
| **In-memory** | ✅ `kv-mem` feature |
| **Persistent** | ✅ RocksDB, TiKV |
| **Embedding** | ✅ Single Rust binary |

#### Usage in Rust

```rust
// Cargo.toml
[dependencies]
surrealdb = { version = "2", features = ["kv-mem"] }  // In-memory
```

```rust
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("pmsynapse").use_db("thoughts").await?;

    // Vector search
    let results: Vec<Thought> = db.query(r#"
        SELECT *, vector::distance::knn() AS distance
        FROM thoughts
        WHERE embedding <|10,COSINE|> $query_vec
        ORDER BY distance
    "#).bind(("query_vec", query_embedding)).await?;

    // Graph traversal
    let related: Vec<Thought> = db.query(r#"
        SELECT ->related_to->thoughts.* FROM $thought_id
    "#).await?;

    Ok(())
}
```

#### Considerations

- **Pro**: Most feature-rich, excellent documentation
- **Pro**: Native AI/LLM integrations (LangChain, LlamaIndex)
- **Con**: Larger binary size than CozoDB
- **Con**: More complex than needed if only using graph + vector

---

### 3. agdb (Agnesoft Graph Database)

**Repository**: [agnesoft/agdb](https://github.com/agnesoft/agdb)

Application-native graph database with no query language - queries are Rust code.

#### Features

| Feature | Support |
|---------|---------|
| **Graph queries** | ✅ Native Rust API |
| **Vector search** | ❌ Not built-in |
| **WASM** | ❓ Not tested/documented |
| **In-memory** | ✅ Memory-mapped |
| **Persistent** | ✅ Full ACID with WAL |

#### Usage

```rust
use agdb::{Db, QueryBuilder};

fn main() -> Result<()> {
    // Memory-mapped (fast, persistent)
    let mut db = Db::new("pmsynapse.agdb")?;

    // Or file-based (low memory)
    let mut db = DbFile::new("pmsynapse.agdb")?;

    // Insert nodes
    db.exec_mut(QueryBuilder::insert()
        .nodes()
        .aliases(["thought_1", "thought_2"])
        .values([[("title", "Auth patterns").into()]])
        .query())?;

    // Create edges
    db.exec_mut(QueryBuilder::insert()
        .edges()
        .from("thought_1")
        .to("thought_2")
        .values([[("type", "related_to").into()]])
        .query())?;

    // Traverse
    let result = db.exec(QueryBuilder::select()
        .ids(":thought_1 -> :thought_2")
        .query())?;

    Ok(())
}
```

#### Considerations

- **Pro**: Zero query language overhead - pure Rust
- **Pro**: Memory-mapped for performance
- **Pro**: Derive macros for custom types
- **Con**: No vector search (would need separate library)
- **Con**: WASM support unclear

---

### 4. IndraDB

**Repository**: [indradb/indradb](https://github.com/indradb/indradb)

Graph database with pluggable backends, written in Rust.

#### Features

| Feature | Support |
|---------|---------|
| **Graph queries** | ✅ Directed, typed graphs |
| **Vector search** | ❌ Not built-in |
| **WASM** | ❓ Not documented |
| **In-memory** | ✅ Yes |
| **Persistent** | ✅ Sled, Postgres backends |

#### Backends

- `indradb-sled` - Embedded persistent
- `indradb-postgres` - PostgreSQL backend
- In-memory for testing

#### Considerations

- **Pro**: Clean API, JSON properties
- **Pro**: gRPC for cross-language support
- **Con**: No vector search
- **Con**: Less actively maintained

---

### 5. Gruphst

**Repository**: [carvilsi/gruphst](https://github.com/carvilsi/gruphst)

Simple in-memory graph database.

#### Features

| Feature | Support |
|---------|---------|
| **Graph queries** | ✅ Basic operations |
| **Vector search** | ❌ No |
| **WASM** | ❓ Unknown |
| **In-memory** | ✅ Yes |
| **Persistent** | ❓ Limited |

#### Considerations

- **Pro**: Lightweight, simple
- **Con**: Limited features
- **Con**: Not production-ready

---

## Vector-Only Options (for Hybrid Approach)

If using a graph DB without vector support, combine with:

### Voy (WASM-First)

**Repository**: [tantaraio/voy](https://github.com/tantaraio/voy)

WASM vector similarity search designed for browsers.

```rust
// Designed specifically for WASM deployment
// Uses k-d tree for indexing
// Squared Euclidean distance
```

**Best for**: Adding vector search to a graph-only DB in WASM.

---

### MemVDB

**Crate**: [memvdb](https://lib.rs/crates/memvdb)

Fast, lightweight in-memory vector database.

```rust
use memvdb::{VectorDB, DistanceMetric};

let db = VectorDB::new(384, DistanceMetric::Cosine);
db.insert("thought_1", vec![0.1, 0.2, ...]);
let neighbors = db.search(&query_vec, 10);
```

**Features**:
- Multiple distance metrics (Euclidean, Cosine, Dot)
- Thread-safe
- Zero dependencies

---

### nano-vectordb-rs

**Crate**: [nano-vectordb-rs](https://crates.io/crates/nano-vectordb-rs)

Port of popular nano-vectordb.

- Insert 100K vectors: ~175ms
- Query 100K vectors: ~13ms
- Rayon parallelism

---

## Hybrid Graph + Vector Architecture

If no single DB meets all needs, use this pattern:

```rust
// Hybrid storage architecture
pub struct HybridStore {
    graph: CozoDb,      // Or agdb for graph relationships
    vectors: MemVDB,    // Or Voy for WASM
}

impl HybridStore {
    pub async fn semantic_graph_search(
        &self,
        query_embedding: &[f32],
        k: usize
    ) -> Vec<ThoughtWithRelations> {
        // 1. Find semantically similar via vectors
        let similar_ids = self.vectors.search(query_embedding, k);

        // 2. Expand relationships via graph
        let mut results = Vec::new();
        for id in similar_ids {
            let relations = self.graph.query(format!(
                "?[related] := *thoughts{{id: '{}'}}, \
                 *relations{{from: '{}', to: related}}",
                id, id
            )).await?;
            results.push(ThoughtWithRelations { id, relations });
        }

        results
    }
}
```

---

## WASM Compatibility Summary

| Database | WASM Status | Notes |
|----------|-------------|-------|
| **CozoDB** | ✅ Production | `features = ["wasm", "nothread"]` |
| **SurrealDB** | ✅ Production | Runs in browser |
| **Voy** | ✅ Designed for | Primary use case |
| **agdb** | ❓ Unknown | Would need testing |
| **IndraDB** | ❓ Unknown | Not documented |
| **MemVDB** | ❓ Possible | Pure Rust, likely works |

---

## Recommendation for PMSynapse

### Primary: CozoDB

```toml
# Cargo.toml

[dependencies.cozo]
version = "0.7"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies.cozo]
features = ["storage-rocksdb"]  # Native: persistent

[target.'cfg(target_arch = "wasm32")'.dependencies.cozo]
features = ["wasm", "nothread", "storage-mem"]  # WASM: in-memory
```

### Why CozoDB Wins

1. **Unified**: Graph + Vector in single database
2. **WASM**: First-class browser support
3. **Performance**: Custom HNSW in Rust, not a wrapper
4. **Query Power**: Datalog recursion for complex graph traversals
5. **Minimal Memory**: Rust RAII, frees memory immediately
6. **Embeddable**: No separate server needed
7. **Active**: Regular releases, good documentation

### Schema Design for PMSynapse

```datalog
# Thoughts with embeddings
:create thoughts {
    id: String,
    title: String,
    content: String,
    type: String,        # research, plan, ticket, pr
    owner: String,
    created_at: Int,
    embedding: [Float; 384]  # Vector embedding
}

# HNSW index for semantic search
::hnsw create thoughts:semantic {
    fields: [embedding],
    dim: 384,
    ef_construction: 50,
    m: 16,
    distance: Cosine
}

# Relations between thoughts
:create thought_relations {
    from: String,
    to: String,
    type: String,        # related_to, depends_on, supersedes
    created_at: Int
}

# Tasks derived from thoughts
:create tasks {
    id: String,
    thought_id: String,  # Source thought
    title: String,
    status: String,
    complexity: Int,
    assignee: String?,
    embedding: [Float; 384]
}

::hnsw create tasks:semantic {
    fields: [embedding],
    dim: 384,
    distance: Cosine
}
```

### Example Queries

```datalog
# Find similar thoughts and their relationships
?[thought_id, title, related_title, similarity] :=
    ~thoughts:semantic{id: thought_id, title, embedding |
        query: $query_vec, k: 10, ef: 50, bind_distance: similarity},
    *thought_relations{from: thought_id, to: related_id},
    *thoughts{id: related_id, title: related_title}

# Multi-hop graph traversal with vector filtering
?[start, middle, end, path_strength] :=
    ~thoughts:semantic{id: start | query: $query_vec, k: 5},
    *thought_relations{from: start, to: middle, type: "depends_on"},
    *thought_relations{from: middle, to: end, type: "depends_on"},
    path_strength = 1.0 / (1.0 + cos_dist($query_vec, embedding))
```

---

## Sources

### Graph Databases
- [CozoDB GitHub](https://github.com/cozodb/cozo)
- [SurrealDB Embedding Rust](https://surrealdb.com/docs/surrealdb/embedding/rust)
- [agdb Documentation](https://docs.rs/agdb)
- [IndraDB GitHub](https://github.com/indradb/indradb)

### Vector Databases
- [Voy - WASM Vector Search](https://github.com/tantaraio/voy)
- [MemVDB](https://lib.rs/crates/memvdb)
- [nano-vectordb-rs](https://crates.io/crates/nano-vectordb-rs)
- [SurrealDB Vector Search](https://surrealdb.com/docs/surrealdb/reference-guide/vector-search)

### Hybrid Approaches
- [HybridRAG - Vector + Knowledge Graph](https://memgraph.com/blog/why-hybridrag)
- [Neo4j - Vectors and Graphs Better Together](https://neo4j.com/blog/developer/vectors-graphs-better-together/)

---

*Research completed: December 2025*
*Part of: PMSynapse AI-Enabled Project Management Research*
