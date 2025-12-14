import { NodesApi } from './nodes';
import { SSEClient } from './sse';
import type { ClientOptions, EventHandlers } from './types';

export class PMSynapseClient {
  private base_url: string;
  public nodes: NodesApi;
  private sse_connections: Map<string, SSEClient> = new Map();

  constructor(options: ClientOptions = {}) {
    const port = options.port ?? 7878;
    this.base_url = options.baseUrl ?? `http://127.0.0.1:${port}/api/v1`;
    this.nodes = new NodesApi(this.base_url);
  }

  async health(): Promise<boolean> {
    try {
      const response = await fetch(`${this.base_url}/health`);
      return response.ok;
    } catch {
      return false;
    }
  }

  subscribe_to_events(handlers: EventHandlers): () => void {
    const id = Math.random().toString(36).substring(7);
    const sse_client = new SSEClient(
      `${this.base_url}/stream/events`,
      handlers
    );

    sse_client.connect();
    this.sse_connections.set(id, sse_client);

    // Return unsubscribe function
    return () => {
      const client = this.sse_connections.get(id);
      if (client) {
        client.disconnect();
        this.sse_connections.delete(id);
      }
    };
  }

  disconnect(): void {
    this.sse_connections.forEach((client) => client.disconnect());
    this.sse_connections.clear();
  }
}
