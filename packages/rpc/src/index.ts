/**
 * @pmsynapse/rpc
 *
 * Shared RPC types and utilities for communication between:
 * - VS Code extension ↔ Webview
 * - Tauri frontend ↔ Backend
 * - CLI ↔ Core library
 */

// ============================================================================
// Message Types
// ============================================================================

/**
 * Base message structure for all RPC communication
 */
export interface RpcMessage<T = unknown> {
  id: string;
  type: string;
  payload?: T;
  timestamp: number;
}

/**
 * Request message (client → server)
 */
export interface RpcRequest<T = unknown> extends RpcMessage<T> {
  method: string;
}

/**
 * Response message (server → client)
 */
export interface RpcResponse<T = unknown> extends RpcMessage<T> {
  requestId: string;
  success: boolean;
  error?: RpcError;
}

/**
 * Error details
 */
export interface RpcError {
  code: string;
  message: string;
  details?: unknown;
}

// ============================================================================
// IDLC Types (shared between all frontends)
// ============================================================================

/**
 * Node types in the knowledge graph
 */
export type NodeType =
  | "idea"
  | "feature"
  | "task"
  | "decision"
  | "question"
  | "assumption"
  | "code"
  | "test"
  | "document"
  | "research"
  | "plan"
  | "completion";

/**
 * Edge types in the knowledge graph
 */
export type EdgeType =
  | "inspires"
  | "requires"
  | "produces"
  | "impacts"
  | "blocks"
  | "validates"
  | "implements"
  | "verifies"
  | "describes"
  | "informs"
  | "enables"
  | "completes";

/**
 * A node in the knowledge graph
 */
export interface KnowledgeNode {
  id: string;
  nodeType: NodeType;
  title: string;
  content: string;
  confidence: number;
  createdAt: string;
  updatedAt: string;
  metadata?: Record<string, unknown>;
}

/**
 * An edge in the knowledge graph
 */
export interface KnowledgeEdge {
  id: string;
  fromNode: string;
  toNode: string;
  edgeType: EdgeType;
  confidence: number;
  createdAt: string;
  metadata?: Record<string, unknown>;
}

// ============================================================================
// API Methods
// ============================================================================

/**
 * Available RPC methods
 */
export const RpcMethods = {
  // Node operations
  CREATE_NODE: "node.create",
  GET_NODE: "node.get",
  UPDATE_NODE: "node.update",
  DELETE_NODE: "node.delete",
  LIST_NODES: "node.list",
  QUERY_NODES: "node.query",

  // Edge operations
  CREATE_EDGE: "edge.create",
  GET_EDGE: "edge.get",
  DELETE_EDGE: "edge.delete",
  LIST_EDGES: "edge.list",

  // Graph operations
  GET_RELATED: "graph.related",
  GET_SUBGRAPH: "graph.subgraph",
  SEARCH: "graph.search",

  // LLM operations
  COMPLETE: "llm.complete",
  EMBED: "llm.embed",
  SUGGEST: "llm.suggest",

  // System operations
  HEALTH: "system.health",
  CONFIG: "system.config",
} as const;

// ============================================================================
// Request/Response Payloads
// ============================================================================

export interface CreateNodeRequest {
  nodeType: NodeType;
  title: string;
  content: string;
  confidence?: number;
  metadata?: Record<string, unknown>;
}

export interface CreateNodeResponse {
  node: KnowledgeNode;
}

export interface QueryNodesRequest {
  nodeType?: NodeType;
  query?: string;
  limit?: number;
  offset?: number;
}

export interface QueryNodesResponse {
  nodes: KnowledgeNode[];
  total: number;
}

export interface CreateEdgeRequest {
  fromNode: string;
  toNode: string;
  edgeType: EdgeType;
  confidence?: number;
  metadata?: Record<string, unknown>;
}

export interface CreateEdgeResponse {
  edge: KnowledgeEdge;
}

export interface GetRelatedRequest {
  nodeId: string;
  depth?: number;
  edgeTypes?: EdgeType[];
}

export interface GetRelatedResponse {
  nodes: KnowledgeNode[];
  edges: KnowledgeEdge[];
}

export interface LlmCompleteRequest {
  prompt: string;
  model?: string;
  maxTokens?: number;
  temperature?: number;
  systemPrompt?: string;
}

export interface LlmCompleteResponse {
  content: string;
  model: string;
  usage: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

// ============================================================================
// Utilities
// ============================================================================

/**
 * Generate a unique message ID
 */
export function generateMessageId(): string {
  return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Create an RPC request
 */
export function createRequest<T>(method: string, payload?: T): RpcRequest<T> {
  return {
    id: generateMessageId(),
    type: "request",
    method,
    payload,
    timestamp: Date.now(),
  };
}

/**
 * Create an RPC response
 */
export function createResponse<T>(
  requestId: string,
  payload?: T,
  error?: RpcError
): RpcResponse<T> {
  return {
    id: generateMessageId(),
    type: "response",
    requestId,
    success: !error,
    payload,
    error,
    timestamp: Date.now(),
  };
}

/**
 * Create an error response
 */
export function createErrorResponse(
  requestId: string,
  code: string,
  message: string,
  details?: unknown
): RpcResponse<undefined> {
  return createResponse<never>(requestId, undefined, { code, message, details });
}
