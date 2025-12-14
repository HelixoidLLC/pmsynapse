import { useState, useEffect } from "react";
import {
  Network,
  FileText,
  GitBranch,
  CheckCircle,
  Clock,
  Plus,
} from "lucide-react";
import { Button } from "../components/ui/button";

// VS Code API
declare const acquireVsCodeApi: () => any;
const vscode = acquireVsCodeApi();

const stats = [
  { name: "Total Nodes", value: "0", icon: Network, color: "text-blue-500" },
  { name: "Documents", value: "0", icon: FileText, color: "text-green-500" },
  { name: "Active Tasks", value: "0", icon: GitBranch, color: "text-purple-500" },
];

const recentActivity = [
  {
    type: "created",
    title: "Project initialized",
    time: "Just now",
    icon: CheckCircle,
    color: "text-green-500",
  },
];

const pendingItems = [
  {
    title: "No pending items",
    description: "Create some nodes to get started",
    icon: Clock,
  },
];

export function DashboardPage() {
  const [nodes, setNodes] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load_nodes = async () => {
    try {
      setLoading(true);
      setError(null);
      vscode.postMessage({ type: 'getNodes' });
    } catch (err: any) {
      setError(err.toString());
      console.error('Failed to load nodes:', err);
      setLoading(false);
    }
  };

  const create_test_node = async () => {
    try {
      setLoading(true);
      setError(null);
      const timestamp = new Date().toLocaleTimeString();
      console.log('Creating test node...');
      vscode.postMessage({
        type: 'createNode',
        data: {
          nodeType: 'idea',
          title: `Test Idea ${timestamp}`,
          content: 'This is a test node created from the VS Code webview'
        }
      });
    } catch (err: any) {
      setError(err.toString());
      console.error('Failed to create node:', err);
      setLoading(false);
    }
  };

  useEffect(() => {
    load_nodes();

    // Listen for messages from extension
    const handle_message = (event: MessageEvent) => {
      const message = event.data;
      switch (message.type) {
        case 'nodesData':
          setNodes(message.data);
          setLoading(false);
          console.log('Loaded nodes:', message.data);
          break;
        case 'error':
          setError(message.error);
          setLoading(false);
          break;
        case 'nodeCreated':
          // Reload nodes after creation
          load_nodes();
          break;
      }
    };

    window.addEventListener('message', handle_message);

    // Auto-refresh every 3 seconds to sync with desktop app changes
    const refresh_interval = setInterval(() => {
      load_nodes();
    }, 3000);

    return () => {
      window.removeEventListener('message', handle_message);
      clearInterval(refresh_interval);
    };
  }, []);

  return (
    <div className="flex flex-col gap-6 p-6">
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-muted-foreground">
          Overview of your project's knowledge graph and workflow status.
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-3 gap-4">
        {stats.map((stat) => (
          <div
            key={stat.name}
            className="flex items-center gap-4 rounded-lg border bg-card p-6"
          >
            <div className={`${stat.color}`}>
              <stat.icon className="h-8 w-8" />
            </div>
            <div>
              <p className="text-2xl font-bold">{stat.value}</p>
              <p className="text-sm text-muted-foreground">{stat.name}</p>
            </div>
          </div>
        ))}
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-2 gap-6">
        {/* Recent Activity */}
        <div className="rounded-lg border bg-card">
          <div className="border-b p-4">
            <h2 className="font-semibold">Recent Activity</h2>
          </div>
          <div className="p-4">
            {recentActivity.length === 0 ? (
              <p className="text-sm text-muted-foreground">No recent activity</p>
            ) : (
              <ul className="space-y-3">
                {recentActivity.map((item, i) => (
                  <li key={i} className="flex items-center gap-3">
                    <item.icon className={`h-5 w-5 ${item.color}`} />
                    <div className="flex-1">
                      <p className="text-sm font-medium">{item.title}</p>
                      <p className="text-xs text-muted-foreground">{item.time}</p>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>

        {/* Pending Items */}
        <div className="rounded-lg border bg-card">
          <div className="border-b p-4">
            <h2 className="font-semibold">Pending Review</h2>
          </div>
          <div className="p-4">
            {pendingItems.map((item, i) => (
              <div key={i} className="flex items-start gap-3">
                <item.icon className="h-5 w-5 text-muted-foreground" />
                <div>
                  <p className="text-sm font-medium">{item.title}</p>
                  <p className="text-xs text-muted-foreground">
                    {item.description}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Knowledge Graph Preview */}
      <div className="rounded-lg border bg-card">
        <div className="border-b p-4 flex items-center justify-between">
          <h2 className="font-semibold">Knowledge Graph</h2>
          <div className="flex gap-2">
            <Button
              size="sm"
              variant="outline"
              onClick={load_nodes}
              disabled={loading}
            >
              Refresh
            </Button>
            <Button
              size="sm"
              onClick={create_test_node}
              disabled={loading}
            >
              <Plus className="h-4 w-4 mr-1" />
              Create Test Node
            </Button>
          </div>
        </div>
        <div className="p-4">
          {error && (
            <div className="mb-4 p-3 rounded-lg bg-destructive/10 text-destructive text-sm">
              <strong>Error:</strong> {error}
            </div>
          )}
          {loading ? (
            <div className="flex h-64 items-center justify-center">
              <p className="text-sm text-muted-foreground">Loading...</p>
            </div>
          ) : nodes.length === 0 ? (
            <div className="flex h-64 items-center justify-center">
              <div className="text-center">
                <Network className="mx-auto h-12 w-12 text-muted-foreground/50" />
                <p className="mt-4 text-sm text-muted-foreground">
                  No nodes yet. Click "Create Test Node" to get started.
                </p>
              </div>
            </div>
          ) : (
            <div className="space-y-2">
              <p className="text-sm text-muted-foreground mb-2">
                {nodes.length} node{nodes.length !== 1 ? 's' : ''} in graph
              </p>
              <div className="space-y-2 max-h-64 overflow-y-auto">
                {nodes.map((node: any, i: number) => (
                  <div key={i} className="p-3 rounded-lg border bg-muted/50">
                    <div className="flex items-start gap-2">
                      <Network className="h-4 w-4 mt-0.5 text-blue-500" />
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium truncate">{node.title}</p>
                        <p className="text-xs text-muted-foreground">
                          {node.node_type} • {node.id?.substring(0, 8)}...
                        </p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
