use snps_core::graph::{Edge, EdgeType, KnowledgeGraph, Node, NodeType};
use std::sync::Arc;
use uuid::Uuid;

use super::events::EventBus;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub graph: Arc<KnowledgeGraph>,
    pub event_bus: EventBus,
    #[allow(dead_code)]
    pub db_path: String,
}

impl AppState {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let mut graph = KnowledgeGraph::new(db_path)?;
        graph.init()?;
        Ok(Self {
            graph: Arc::new(graph),
            event_bus: EventBus::new(),
            db_path: db_path.to_string(),
        })
    }

    /// List all nodes
    pub async fn list_nodes(&self) -> anyhow::Result<Vec<Node>> {
        Ok(self.graph.list_nodes()?)
    }

    /// Create a new node with convenience method
    pub async fn create_node(
        &self,
        node_type: NodeType,
        title: &str,
        content: Option<&str>,
    ) -> anyhow::Result<Node> {
        let node = Node {
            id: Uuid::new_v4(),
            node_type,
            title: title.to_string(),
            content: content.unwrap_or("").to_string(),
            confidence: 1.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.graph.add_node(&node)?;

        Ok(node)
    }

    /// Get a specific node by ID
    pub async fn get_node(&self, id: &str) -> Option<Node> {
        let uuid = Uuid::parse_str(id).ok()?;
        self.graph.get_node(&uuid).ok()
    }

    /// Create an edge between nodes
    pub async fn create_edge(
        &self,
        source_id: &str,
        target_id: &str,
        edge_type: EdgeType,
    ) -> anyhow::Result<Edge> {
        let source_uuid = Uuid::parse_str(source_id)?;
        let target_uuid = Uuid::parse_str(target_id)?;

        let edge = Edge {
            id: Uuid::new_v4(),
            from_node: source_uuid,
            to_node: target_uuid,
            edge_type,
            confidence: 1.0,
            created_at: chrono::Utc::now(),
        };

        self.graph.add_edge(&edge)?;

        Ok(edge)
    }

    /// Get related nodes (stub for now)
    pub async fn get_related_nodes(&self, _id: &str) -> anyhow::Result<Vec<Node>> {
        // TODO: Implement when graph has proper query support
        Ok(vec![])
    }
}
