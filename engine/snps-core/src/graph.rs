//! Knowledge Graph Module
//!
//! Provides graph-based knowledge management using SQLite (MVP).
//! Will migrate to CozoDB for Datalog and vector search when dependencies stabilize.

use crate::{Result, SynapseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    db_path: String,
    initialized: bool,
    // Temporary in-memory storage until database is implemented
    nodes: Arc<Mutex<HashMap<Uuid, Node>>>,
    edges: Arc<Mutex<HashMap<Uuid, Edge>>>,
}

impl KnowledgeGraph {
    /// Create a new knowledge graph
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
            initialized: false,
            nodes: Arc::new(Mutex::new(HashMap::new())),
            edges: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Initialize the knowledge graph with schema
    pub fn init(&mut self) -> Result<()> {
        tracing::info!("Initializing knowledge graph at {}", self.db_path);

        // TODO: Initialize CozoDB and create schema
        // For now, just mark as initialized
        self.initialized = true;

        Ok(())
    }

    /// Add a node to the graph
    pub fn add_node(&self, node: &Node) -> Result<Uuid> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Adding node: {:?}", node.id);
        let mut nodes = self.nodes.lock().unwrap();
        nodes.insert(node.id, node.clone());
        Ok(node.id)
    }

    /// Add an edge to the graph
    pub fn add_edge(&self, edge: &Edge) -> Result<Uuid> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Adding edge: {:?}", edge.id);
        let mut edges = self.edges.lock().unwrap();
        edges.insert(edge.id, edge.clone());
        Ok(edge.id)
    }

    /// Query nodes by type
    pub fn query_by_type(&self, node_type: &NodeType) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Querying nodes of type: {:?}", node_type);
        // TODO: Query CozoDB
        Ok(vec![])
    }

    /// Find related nodes
    pub fn find_related(&self, node_id: Uuid, depth: u32) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Finding nodes related to {} at depth {}", node_id, depth);
        // TODO: Recursive query in CozoDB
        Ok(vec![])
    }

    /// List all nodes
    pub fn list_nodes(&self) -> Result<Vec<Node>> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Listing all nodes");
        let nodes = self.nodes.lock().unwrap();
        Ok(nodes.values().cloned().collect())
    }

    /// Get a specific node by ID
    pub fn get_node(&self, node_id: &Uuid) -> Result<Node> {
        if !self.initialized {
            return Err(SynapseError::Graph("Graph not initialized".into()));
        }

        tracing::debug!("Getting node: {}", node_id);
        let nodes = self.nodes.lock().unwrap();
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| SynapseError::Graph("Node not found".into()))
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new("./synapse.db")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut graph = KnowledgeGraph::new(":memory:");
        assert!(graph.init().is_ok());
        assert!(graph.initialized);
    }
}
