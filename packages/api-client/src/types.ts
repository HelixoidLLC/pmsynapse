// Re-export types from RPC package
export type {
  NodeType,
  EdgeType,
  KnowledgeNode,
  KnowledgeEdge,
  CreateNodeRequest,
  CreateEdgeRequest,
} from '@pmsynapse/rpc';

// Client-specific types
export interface ClientOptions {
  baseUrl?: string;
  port?: number;
}

export interface EventHandlers {
  onConnect?: () => void;
  onDisconnect?: () => void;
  onMessage?: (event: any) => void;
  onError?: (error: Error) => void;
}
