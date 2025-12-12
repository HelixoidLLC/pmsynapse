//! LLM Integration Module
//!
//! Provides multi-provider LLM integration with:
//! - OpenAI, Anthropic, Google, Ollama support
//! - OpenRouter-style routing
//! - Automatic fallback and load balancing

use crate::{Result, SynapseError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// LLM Provider trait for multi-provider support
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Complete a prompt
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Stream a completion
    async fn stream(&self, request: CompletionRequest) -> Result<CompletionStream>;

    /// Get embeddings for text
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;

    /// Check if the provider is available
    async fn health_check(&self) -> Result<bool>;
}

/// Completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub model: String,
    pub usage: Usage,
    pub finish_reason: FinishReason,
}

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Reason for completion finish
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    Error,
}

/// Streaming completion (placeholder)
pub struct CompletionStream {
    // TODO: Implement streaming with async iterators
}

/// LLM Router for multi-provider support
pub struct LlmRouter {
    providers: Vec<Box<dyn LlmProvider>>,
    default_provider: usize,
}

impl LlmRouter {
    /// Create a new LLM router
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            default_provider: 0,
        }
    }

    /// Add a provider to the router
    pub fn add_provider(&mut self, provider: Box<dyn LlmProvider>) {
        self.providers.push(provider);
    }

    /// Complete a prompt with automatic fallback
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        if self.providers.is_empty() {
            return Err(SynapseError::Llm("No providers configured".into()));
        }

        // Try each provider in order
        let mut last_error = None;
        for provider in &self.providers {
            match provider.complete(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    tracing::warn!("Provider {} failed: {}", provider.name(), e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| SynapseError::Llm("All providers failed".into())))
    }

    /// Get embeddings with fallback
    pub async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if self.providers.is_empty() {
            return Err(SynapseError::Llm("No providers configured".into()));
        }

        // Use default provider for embeddings
        self.providers[self.default_provider].embed(texts).await
    }
}

impl Default for LlmRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_request() {
        let request = CompletionRequest {
            model: "gpt-4".into(),
            messages: vec![Message {
                role: Role::User,
                content: "Hello".into(),
            }],
            max_tokens: Some(100),
            temperature: Some(0.7),
            system_prompt: None,
        };

        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.messages.len(), 1);
    }

    #[test]
    fn test_router_creation() {
        let router = LlmRouter::new();
        assert!(router.providers.is_empty());
    }
}
