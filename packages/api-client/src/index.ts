// Main client
export { PMSynapseClient } from './client';

// APIs
export { NodesApi } from './nodes';
export { SSEClient } from './sse';

// Types
export type {
  ClientOptions,
  EventHandlers,
  NodeType,
  EdgeType,
  KnowledgeNode,
  KnowledgeEdge,
  CreateNodeRequest,
  CreateEdgeRequest,
} from './types';
