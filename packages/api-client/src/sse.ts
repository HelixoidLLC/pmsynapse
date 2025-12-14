import type { EventHandlers } from './types';

export class SSEClient {
  private event_source: EventSource | null = null;
  private connection_id: string;

  constructor(
    private url: string,
    private handlers: EventHandlers
  ) {
    this.connection_id = Math.random().toString(36).substring(7);
  }

  connect(): void {
    if (this.event_source) {
      return; // Already connected
    }

    this.event_source = new EventSource(this.url);

    this.event_source.onopen = () => {
      this.handlers.onConnect?.();
    };

    this.event_source.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        this.handlers.onMessage?.(data);
      } catch (error) {
        this.handlers.onError?.(error as Error);
      }
    };

    this.event_source.onerror = () => {
      if (this.event_source?.readyState === EventSource.CLOSED) {
        this.disconnect();
        this.handlers.onDisconnect?.();
      }
      this.handlers.onError?.(new Error('SSE connection error'));
    };
  }

  disconnect(): void {
    if (this.event_source) {
      this.event_source.close();
      this.event_source = null;
      this.handlers.onDisconnect?.();
    }
  }

  is_connected(): boolean {
    return this.event_source !== null && this.event_source.readyState === EventSource.OPEN;
  }
}
