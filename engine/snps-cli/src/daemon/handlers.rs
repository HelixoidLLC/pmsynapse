use crate::daemon::events::EventType;
use crate::daemon::state::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use snps_core::graph::{EdgeType, Node, NodeType};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    pub node_type: NodeType,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateEdgeRequest {
    pub source_id: String,
    pub target_id: String,
    pub edge_type: EdgeType,
}

#[derive(Serialize)]
pub struct RelatedNodesResponse {
    pub nodes: Vec<Node>,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// GET /api/v1/nodes
pub async fn list_nodes(State(state): State<SharedState>) -> Json<Vec<Node>> {
    let nodes = state.list_nodes().await.unwrap_or_default();
    Json(nodes)
}

// POST /api/v1/nodes
pub async fn create_node(
    State(state): State<SharedState>,
    Json(req): Json<CreateNodeRequest>,
) -> Result<Json<Node>, StatusCode> {
    match state
        .create_node(req.node_type, &req.title, req.content.as_deref())
        .await
    {
        Ok(node) => {
            // Publish event for SSE subscribers
            state
                .event_bus
                .publish(EventType::NodeCreated, serde_json::to_value(&node).unwrap());
            Ok(Json(node))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /api/v1/nodes/:id
pub async fn get_node(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<Node>, StatusCode> {
    match state.get_node(&id).await {
        Some(node) => Ok(Json(node)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// POST /api/v1/edges
pub async fn create_edge(
    State(state): State<SharedState>,
    Json(req): Json<CreateEdgeRequest>,
) -> Result<Json<snps_core::graph::Edge>, StatusCode> {
    match state
        .create_edge(&req.source_id, &req.target_id, req.edge_type)
        .await
    {
        Ok(edge) => Ok(Json(edge)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /api/v1/graph/related/:id
pub async fn get_related(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<RelatedNodesResponse> {
    let nodes = state.get_related_nodes(&id).await.unwrap_or_default();
    Json(RelatedNodesResponse { nodes })
}

// SSE streaming endpoint
pub async fn sse_events(
    State(state): State<SharedState>,
) -> axum::response::Sse<
    impl futures::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>,
> {
    use axum::response::sse::{Event as SseEvent, KeepAlive};

    let mut rx = state.event_bus.subscribe();

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    let json = serde_json::to_string(&event).unwrap();
                    yield Ok(SseEvent::default().data(json));
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    // Client fell behind, continue
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    axum::response::Sse::new(stream).keep_alive(KeepAlive::default())
}
