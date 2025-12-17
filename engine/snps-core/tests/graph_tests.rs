//! Integration tests for the knowledge graph module.
//!
//! Following BAML's test organization pattern where integration tests
//! are separated from unit tests and placed in the `tests/` directory.

use snps_core::graph::{Edge, EdgeType, KnowledgeGraph, Node, NodeType};
use tempfile::tempdir;
use uuid::Uuid;

#[test]
fn test_graph_initialization() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");
    assert!(graph.init().is_ok());
}

#[test]
fn test_graph_requires_initialization() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");

    // Operations should fail on uninitialized graph
    let node = create_test_node(NodeType::Idea, "Test");
    let result = graph.add_node(&node);
    assert!(result.is_err());
}

#[test]
fn test_add_node() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");
    graph.init().expect("Failed to init graph");

    let node = create_test_node(NodeType::Feature, "Test Feature");
    let result = graph.add_node(&node);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), node.id);
}

#[test]
fn test_add_edge() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");
    graph.init().expect("Failed to init graph");

    let node1 = create_test_node(NodeType::Feature, "Feature A");
    let node2 = create_test_node(NodeType::Task, "Task 1");

    graph.add_node(&node1).expect("Failed to add node 1");
    graph.add_node(&node2).expect("Failed to add node 2");

    let edge = create_test_edge(node1.id, node2.id, EdgeType::Implements);
    let result = graph.add_edge(&edge);
    assert!(result.is_ok());
}

#[test]
fn test_query_by_type() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");
    graph.init().expect("Failed to init graph");

    // Add some nodes
    let task1 = create_test_node(NodeType::Task, "Task 1");
    let task2 = create_test_node(NodeType::Task, "Task 2");
    let feature = create_test_node(NodeType::Feature, "Feature 1");

    graph.add_node(&task1).expect("Failed to add task 1");
    graph.add_node(&task2).expect("Failed to add task 2");
    graph.add_node(&feature).expect("Failed to add feature");

    // Query tasks should return the two tasks we added
    let tasks = graph
        .query_by_type(&NodeType::Task)
        .expect("Failed to query");
    assert_eq!(tasks.len(), 2);
}

#[test]
fn test_find_related() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut graph = KnowledgeGraph::new(db_path.to_str().unwrap()).expect("Failed to create graph");
    graph.init().expect("Failed to init graph");

    let node1 = create_test_node(NodeType::Idea, "Test Idea");
    let node2 = create_test_node(NodeType::Task, "Related Task");

    graph.add_node(&node1).expect("Failed to add node 1");
    graph.add_node(&node2).expect("Failed to add node 2");

    // Add edge from node1 to node2
    let edge = create_test_edge(node1.id, node2.id, EdgeType::Inspires);
    graph.add_edge(&edge).expect("Failed to add edge");

    // Find related should return node2
    let related = graph
        .find_related(node1.id, 1)
        .expect("Failed to find related");
    assert_eq!(related.len(), 1);
    assert_eq!(related[0].id, node2.id);
}

#[test]
fn test_node_types() {
    // Verify all node types can be created
    let types = vec![
        NodeType::Idea,
        NodeType::Feature,
        NodeType::Task,
        NodeType::Decision,
        NodeType::Question,
        NodeType::Assumption,
        NodeType::Code,
        NodeType::Test,
        NodeType::Document,
        NodeType::Research,
        NodeType::Plan,
        NodeType::Completion,
    ];

    for node_type in types {
        let node = create_test_node(node_type.clone(), "Test");
        assert_eq!(node.node_type, node_type);
    }
}

#[test]
fn test_edge_types() {
    // Verify all edge types can be created
    let types = vec![
        EdgeType::Inspires,
        EdgeType::Requires,
        EdgeType::Produces,
        EdgeType::Impacts,
        EdgeType::Blocks,
        EdgeType::Validates,
        EdgeType::Implements,
        EdgeType::Verifies,
        EdgeType::Describes,
        EdgeType::Informs,
        EdgeType::Enables,
        EdgeType::Completes,
    ];

    let from = Uuid::new_v4();
    let to = Uuid::new_v4();

    for edge_type in types {
        let edge = create_test_edge(from, to, edge_type.clone());
        assert_eq!(edge.edge_type, edge_type);
    }
}

#[test]
fn test_default_graph() {
    let graph = KnowledgeGraph::default();
    // Default creates with "./synapse.db" path
    assert!(graph.query_by_type(&NodeType::Idea).is_err());
}

// Helper functions

fn create_test_node(node_type: NodeType, title: &str) -> Node {
    let now = chrono::Utc::now();
    Node {
        id: Uuid::new_v4(),
        node_type,
        title: title.to_string(),
        content: format!("Content for {}", title),
        confidence: 1.0,
        created_at: now,
        updated_at: now,
    }
}

fn create_test_edge(from_node: Uuid, to_node: Uuid, edge_type: EdgeType) -> Edge {
    Edge {
        id: Uuid::new_v4(),
        from_node,
        to_node,
        edge_type,
        confidence: 1.0,
        created_at: chrono::Utc::now(),
    }
}
