use axum::{routing::{get, post}, Router};
use std::net::TcpListener;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use super::{handlers, state::AppState};

pub struct DaemonServer {
    port: u16,
    listener: TcpListener,
}

impl DaemonServer {
    pub fn new(requested_port: u16) -> anyhow::Result<Self> {
        // Bind to requested port (0 = dynamic allocation)
        let addr = format!("127.0.0.1:{}", requested_port);
        let listener = TcpListener::bind(&addr)?;
        let actual_port = listener.local_addr()?.port();

        Ok(Self {
            port: actual_port,
            listener,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self, db_path: &str) -> anyhow::Result<()> {
        // Print port to stdout (like HumanLayer)
        println!("HTTP_PORT={}", self.port);

        let state = Arc::new(AppState::new(db_path)?);

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .route("/api/v1/health", get(handlers::health))
            .route("/api/v1/nodes", get(handlers::list_nodes).post(handlers::create_node))
            .route("/api/v1/nodes/:id", get(handlers::get_node))
            .route("/api/v1/edges", post(handlers::create_edge))
            .route("/api/v1/graph/related/:id", get(handlers::get_related))
            .route("/api/v1/stream/events", get(handlers::sse_events))
            .with_state(state)
            .layer(cors);

        let listener = tokio::net::TcpListener::from_std(self.listener)?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(())
    }
}

async fn shutdown_signal() {
    eprintln!("[DEBUG] Waiting for Ctrl-C signal...");
    tokio::signal::ctrl_c().await.ok();
    eprintln!("[DEBUG] Received Ctrl-C, initiating graceful shutdown...");
}
