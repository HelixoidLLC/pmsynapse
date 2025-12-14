import type { KnowledgeNode, CreateNodeRequest } from './types';

export class NodesApi {
  constructor(private base_url: string) {}

  async list(): Promise<KnowledgeNode[]> {
    const response = await fetch(`${this.base_url}/nodes`);
    if (!response.ok) {
      throw new Error(`Failed to list nodes: ${response.statusText}`);
    }
    const data = await response.json();
    // Convert snake_case from Rust to camelCase for TypeScript
    return data.map((node: any) => ({
      id: node.id,
      nodeType: node.node_type,
      title: node.title,
      content: node.content,
      confidence: node.confidence,
      createdAt: node.created_at,
      updatedAt: node.updated_at,
    }));
  }

  async get(id: string): Promise<KnowledgeNode> {
    const response = await fetch(`${this.base_url}/nodes/${id}`);
    if (!response.ok) {
      throw new Error(`Failed to get node: ${response.statusText}`);
    }
    const data = await response.json();
    // Convert snake_case from Rust to camelCase for TypeScript
    return {
      id: data.id,
      nodeType: data.node_type,
      title: data.title,
      content: data.content,
      confidence: data.confidence,
      createdAt: data.created_at,
      updatedAt: data.updated_at,
    };
  }

  async create(request: CreateNodeRequest): Promise<KnowledgeNode> {
    const response = await fetch(`${this.base_url}/nodes`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        node_type: request.nodeType,
        title: request.title,
        content: request.content,
      }),
    });
    if (!response.ok) {
      throw new Error(`Failed to create node: ${response.statusText}`);
    }
    const data = await response.json();
    // Convert snake_case from Rust to camelCase for TypeScript
    return {
      id: data.id,
      nodeType: data.node_type,
      title: data.title,
      content: data.content,
      confidence: data.confidence,
      createdAt: data.created_at,
      updatedAt: data.updated_at,
    };
  }
}
