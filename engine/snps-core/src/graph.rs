//! Knowledge Graph Module
//!
//! Provides graph-based knowledge management using CozoDB with Datalog queries.

use crate::{Result, SynapseError};
use cozo::{DataValue, DbInstance, ScriptMutability};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// A node in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub node_type: NodeType,
    pub title: String,
    pub content: String,
    pub confidence: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Types of nodes in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Idea,
    Feature,
    Task,
    Decision,
    Question,
    Assumption,
    Code,
    Test,
    Document,
    Research,
    Plan,
    Completion,
}

/// An edge (relationship) between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub edge_type: EdgeType,
    pub confidence: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Types of edges in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    Inspires,
    Requires,
    Produces,
    Impacts,
    Blocks,
    Validates,
    Implements,
    Verifies,
    Describes,
    Informs,
    Enables,
    Completes,
}

/// Knowledge Graph manager
pub struct KnowledgeGraph {
    pub(crate) db: DbInstance,
    db_path: String,
    initialized: bool,
}

impl KnowledgeGraph {
    /// Create a new knowledge graph with CozoDB
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DbInstance::new("sqlite", db_path, "")
            .map_err(|e| SynapseError::Graph(format!("Failed to create CozoDB instance: {}", e)))?;

        Ok(Self {
            db,
            db_path: db_path.to_string(),
            initialized: false,
        })
    }

    /// Initialize the knowledge graph with schema
    pub fn init(&mut self) -> Result<()> {
        // Skip if already initialized
        if self.initialized {
            return Ok(());
        }

        tracing::info!("Initializing knowledge graph at {}", self.db_path);

        // Create nodes relation (ignore if exists)
        let _ = self.db.run_script(
            r#"
            :create nodes {
                id: String
                =>
                node_type: String,
                title: String,
                content: String,
                confidence: Float,
                created_at: String,
                updated_at: String
            }
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        // Create edges relation (ignore if exists)
        let _ = self.db.run_script(
            r#"
            :create edges {
                id: String
                =>
                from_node: String,
                to_node: String,
                edge_type: String,
                confidence: Float,
                created_at: String
            }
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        // Create index on from_node for efficient edge queries (ignore if exists)
        let _ = self.db.run_script(
            r#"
            ::index create edges:from_idx {from_node}
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        // Create index on to_node for efficient reverse edge queries (ignore if exists)
        let _ = self.db.run_script(
            r#"
            ::index create edges:to_idx {to_node}
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        // Create matter relation for indexing file-based matter (ignore if exists)
        let _ = self.db.run_script(
            r#"
            :create matter {
                id: String
                =>
                repository_id: String,
                file_path: String,
                matter_type: String,
                title: String,
                context_type: String,
                context_id: String,
                visibility: String,
                tags: String,
                created_at: String,
                updated_at: String,
                created_by: String,
                content_hash: String
            }
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        // Create index on matter file_path for efficient lookups (ignore if exists)
        let _ = self.db.run_script(
            r#"
            ::index create matter:path_idx {file_path}
            "#,
            Default::default(),
            ScriptMutability::Mutable,
        );

        self.initialized = true;
        tracing::info!("Knowledge graph schema initialized successfully");

        Ok(())
    }

    /// Add a node to the graph
    pub fn add_node(&self, node: &Node) -> Result<Uuid> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Adding node: {:?}", node.id);

        let node_type_str = serde_json::to_string(&node.node_type)
            .map_err(|e| SynapseError::Graph(format!("Failed to serialize node type: {}", e)))?
            .trim_matches('"')
            .to_string();

        let params = BTreeMap::from([
            ("id".to_string(), DataValue::Str(node.id.to_string().into())),
            (
                "node_type".to_string(),
                DataValue::Str(node_type_str.into()),
            ),
            (
                "title".to_string(),
                DataValue::Str(node.title.clone().into()),
            ),
            (
                "content".to_string(),
                DataValue::Str(node.content.clone().into()),
            ),
            (
                "confidence".to_string(),
                DataValue::from(node.confidence as f64),
            ),
            (
                "created_at".to_string(),
                DataValue::Str(node.created_at.to_rfc3339().into()),
            ),
            (
                "updated_at".to_string(),
                DataValue::Str(node.updated_at.to_rfc3339().into()),
            ),
        ]);

        self.db
            .run_script(
                r#"
                ?[id, node_type, title, content, confidence, created_at, updated_at] <- [[
                    $id, $node_type, $title, $content, $confidence, $created_at, $updated_at
                ]]
                :put nodes { id, node_type, title, content, confidence, created_at, updated_at }
                "#,
                params,
                ScriptMutability::Mutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to add node: {}", e)))?;

        Ok(node.id)
    }

    /// Add an edge to the graph
    pub fn add_edge(&self, edge: &Edge) -> Result<Uuid> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Adding edge: {:?}", edge.id);

        let edge_type_str = serde_json::to_string(&edge.edge_type)
            .map_err(|e| SynapseError::Graph(format!("Failed to serialize edge type: {}", e)))?
            .trim_matches('"')
            .to_string();

        let params = BTreeMap::from([
            ("id".to_string(), DataValue::Str(edge.id.to_string().into())),
            (
                "from_node".to_string(),
                DataValue::Str(edge.from_node.to_string().into()),
            ),
            (
                "to_node".to_string(),
                DataValue::Str(edge.to_node.to_string().into()),
            ),
            (
                "edge_type".to_string(),
                DataValue::Str(edge_type_str.into()),
            ),
            (
                "confidence".to_string(),
                DataValue::from(edge.confidence as f64),
            ),
            (
                "created_at".to_string(),
                DataValue::Str(edge.created_at.to_rfc3339().into()),
            ),
        ]);

        self.db
            .run_script(
                r#"
                ?[id, from_node, to_node, edge_type, confidence, created_at] <- [[
                    $id, $from_node, $to_node, $edge_type, $confidence, $created_at
                ]]
                :put edges { id, from_node, to_node, edge_type, confidence, created_at }
                "#,
                params,
                ScriptMutability::Mutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to add edge: {}", e)))?;

        Ok(edge.id)
    }

    /// Query nodes by type
    pub fn query_by_type(&self, node_type: &NodeType) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Querying nodes of type: {:?}", node_type);

        let node_type_str = serde_json::to_string(node_type)
            .map_err(|e| SynapseError::Graph(format!("Failed to serialize node type: {}", e)))?
            .trim_matches('"')
            .to_string();

        let params = BTreeMap::from([(
            "node_type".to_string(),
            DataValue::Str(node_type_str.into()),
        )]);

        let result = self
            .db
            .run_script(
                r#"
                ?[id, node_type, title, content, confidence, created_at, updated_at] :=
                    *nodes{id, node_type, title, content, confidence, created_at, updated_at},
                    node_type == $node_type
                "#,
                params,
                ScriptMutability::Immutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to query nodes by type: {}", e)))?;

        self.rows_to_nodes(result.rows)
    }

    /// Find related nodes
    pub fn find_related(&self, node_id: Uuid, depth: u32) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Finding nodes related to {} at depth {}", node_id, depth);

        let params = BTreeMap::from([
            (
                "start_node".to_string(),
                DataValue::Str(node_id.to_string().into()),
            ),
            ("depth".to_string(), DataValue::from(depth as i64)),
        ]);

        // Find nodes directly connected from the start node
        let result = self
            .db
            .run_script(
                r#"
                ?[id, node_type, title, content, confidence, created_at, updated_at] :=
                    *edges{from_node, to_node: id},
                    from_node == $start_node,
                    *nodes{id, node_type, title, content, confidence, created_at, updated_at}
                "#,
                params,
                ScriptMutability::Immutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to find related nodes: {}", e)))?;

        self.rows_to_nodes(result.rows)
    }

    /// List all nodes
    pub fn list_nodes(&self) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Listing all nodes");

        let result = self
            .db
            .run_script(
                r#"
                ?[id, node_type, title, content, confidence, created_at, updated_at] :=
                    *nodes{id, node_type, title, content, confidence, created_at, updated_at}
                "#,
                Default::default(),
                ScriptMutability::Immutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to list nodes: {}", e)))?;

        self.rows_to_nodes(result.rows)
    }

    /// Get a specific node by ID
    pub fn get_node(&self, node_id: &Uuid) -> Result<Node> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Getting node: {}", node_id);

        let params =
            BTreeMap::from([("id".to_string(), DataValue::Str(node_id.to_string().into()))]);

        let result = self
            .db
            .run_script(
                r#"
                ?[id, node_type, title, content, confidence, created_at, updated_at] :=
                    *nodes{id, node_type, title, content, confidence, created_at, updated_at},
                    id == $id
                "#,
                params,
                ScriptMutability::Immutable,
            )
            .map_err(|e| SynapseError::Graph(format!("Failed to get node: {}", e)))?;

        let nodes = self.rows_to_nodes(result.rows)?;
        nodes
            .into_iter()
            .next()
            .ok_or_else(|| SynapseError::Graph("Node not found".into()))
    }

    /// Convert CozoDB rows to Node structs
    fn rows_to_nodes(&self, rows: Vec<Vec<DataValue>>) -> Result<Vec<Node>> {
        rows.into_iter()
            .map(|row| {
                if row.len() != 7 {
                    return Err(SynapseError::Graph(format!(
                        "Invalid row length: expected 7, got {}",
                        row.len()
                    )));
                }

                let id = row[0]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid id type".into()))?
                    .parse::<Uuid>()
                    .map_err(|e| SynapseError::Graph(format!("Failed to parse UUID: {}", e)))?;

                let node_type_str = row[1]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid node_type type".into()))?;
                let node_type: NodeType = serde_json::from_str(&format!("\"{}\"", node_type_str))
                    .map_err(|e| {
                    SynapseError::Graph(format!("Failed to parse node type: {}", e))
                })?;

                let title = row[2]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid title type".into()))?
                    .to_string();

                let content = row[3]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid content type".into()))?
                    .to_string();

                let confidence = row[4]
                    .get_float()
                    .ok_or_else(|| SynapseError::Graph("Invalid confidence type".into()))?
                    as f32;

                let created_at_str = row[5]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid created_at type".into()))?;
                let created_at = chrono::DateTime::parse_from_rfc3339(created_at_str)
                    .map_err(|e| SynapseError::Graph(format!("Failed to parse created_at: {}", e)))?
                    .with_timezone(&chrono::Utc);

                let updated_at_str = row[6]
                    .get_str()
                    .ok_or_else(|| SynapseError::Graph("Invalid updated_at type".into()))?;
                let updated_at = chrono::DateTime::parse_from_rfc3339(updated_at_str)
                    .map_err(|e| SynapseError::Graph(format!("Failed to parse updated_at: {}", e)))?
                    .with_timezone(&chrono::Utc);

                Ok(Node {
                    id,
                    node_type,
                    title,
                    content,
                    confidence,
                    created_at,
                    updated_at,
                })
            })
            .collect()
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new("./synapse.db").expect("Failed to create default KnowledgeGraph")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_node_creation() {
        let node = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Idea,
            title: "Test Idea".into(),
            content: "This is a test idea".into(),
            confidence: 1.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(node.node_type, NodeType::Idea);
    }

    #[test]
    fn test_graph_init() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).unwrap();
        assert!(graph.init().is_ok());
        assert!(graph.initialized);
    }

    #[test]
    fn test_add_and_get_node() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).unwrap();
        graph.init().unwrap();

        let node = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Idea,
            title: "Test Idea".into(),
            content: "This is a test idea".into(),
            confidence: 0.9,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let node_id = graph.add_node(&node).unwrap();
        assert_eq!(node_id, node.id);

        let retrieved = graph.get_node(&node_id).unwrap();
        assert_eq!(retrieved.id, node.id);
        assert_eq!(retrieved.title, node.title);
        assert_eq!(retrieved.node_type, node.node_type);
    }

    #[test]
    fn test_list_nodes() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).unwrap();
        graph.init().unwrap();

        let node1 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Idea,
            title: "Idea 1".into(),
            content: "Content 1".into(),
            confidence: 0.9,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let node2 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Task,
            title: "Task 1".into(),
            content: "Content 2".into(),
            confidence: 0.8,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        graph.add_node(&node1).unwrap();
        graph.add_node(&node2).unwrap();

        let nodes = graph.list_nodes().unwrap();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_query_by_type() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).unwrap();
        graph.init().unwrap();

        let node1 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Idea,
            title: "Idea 1".into(),
            content: "Content 1".into(),
            confidence: 0.9,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let node2 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Task,
            title: "Task 1".into(),
            content: "Content 2".into(),
            confidence: 0.8,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        graph.add_node(&node1).unwrap();
        graph.add_node(&node2).unwrap();

        let ideas = graph.query_by_type(&NodeType::Idea).unwrap();
        assert_eq!(ideas.len(), 1);
        assert_eq!(ideas[0].node_type, NodeType::Idea);
    }

    #[test]
    fn test_add_edge() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).unwrap();
        graph.init().unwrap();

        let node1 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Idea,
            title: "Idea 1".into(),
            content: "Content 1".into(),
            confidence: 0.9,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let node2 = Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Task,
            title: "Task 1".into(),
            content: "Content 2".into(),
            confidence: 0.8,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        graph.add_node(&node1).unwrap();
        graph.add_node(&node2).unwrap();

        let edge = Edge {
            id: Uuid::new_v4(),
            from_node: node1.id,
            to_node: node2.id,
            edge_type: EdgeType::Inspires,
            confidence: 0.9,
            created_at: chrono::Utc::now(),
        };

        let edge_id = graph.add_edge(&edge).unwrap();
        assert_eq!(edge_id, edge.id);
    }
}
