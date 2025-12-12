import {
  Network,
  FileText,
  GitBranch,
  CheckCircle,
  Clock,
} from "lucide-react";

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
    description: "Run 'snps analyze' to get started",
    icon: Clock,
  },
];

export function DashboardPage() {
  return (
    <div className="flex flex-col gap-6">
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
        <div className="border-b p-4">
          <h2 className="font-semibold">Knowledge Graph</h2>
        </div>
        <div className="flex h-64 items-center justify-center p-4">
          <div className="text-center">
            <Network className="mx-auto h-12 w-12 text-muted-foreground/50" />
            <p className="mt-4 text-sm text-muted-foreground">
              Your knowledge graph will appear here.
            </p>
            <p className="text-xs text-muted-foreground">
              Run <code className="rounded bg-muted px-1">snps analyze</code> to
              populate it.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
