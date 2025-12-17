//! PMSynapse Core Library
//!
//! This crate provides the core functionality for PMSynapse:
//! - Knowledge Graph management (CozoDB)
//! - LLM integration (multi-provider)
//! - IDLC (Idea Development Lifecycle) workflow

pub mod claude;
pub mod config;
pub mod graph;
pub mod idlc;
pub mod index;
pub mod llm;
pub mod matter;
pub mod repository;

use thiserror::Error;

/// PMSynapse core error types
#[derive(Error, Debug)]
pub enum SynapseError {
    #[error("Graph error: {0}")]
    Graph(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("IDLC error: {0}")]
    Idlc(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Matter error: {0}")]
    Matter(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlParse(#[from] serde_yaml::Error),
}

/// Result type alias for PMSynapse operations
pub type Result<T> = std::result::Result<T, SynapseError>;

/// PMSynapse version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the PMSynapse core library
pub fn init() -> Result<()> {
    tracing::info!("PMSynapse Core v{} initialized", VERSION);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
