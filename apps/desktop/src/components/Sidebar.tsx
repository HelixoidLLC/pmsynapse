import { Link, useLocation } from "react-router-dom";
import {
  Home,
  LayoutDashboard,
  Network,
  FileText,
  Settings,
  GitBranch,
  Users,
  Lightbulb,
} from "lucide-react";
import { cn } from "@/lib/utils";

const navigation = [
  { name: "Home", href: "/", icon: Home },
  { name: "Dashboard", href: "/dashboard", icon: LayoutDashboard },
  { name: "Knowledge Graph", href: "/graph", icon: Network },
  { name: "Ideas", href: "/ideas", icon: Lightbulb },
  { name: "Documents", href: "/documents", icon: FileText },
  { name: "Workflows", href: "/workflows", icon: GitBranch },
  { name: "Teams", href: "/teams", icon: Users },
];

export function Sidebar() {
  const location = useLocation();

  return (
    <div className="flex w-64 flex-col border-r bg-card">
      {/* Logo */}
      <div
        className="flex h-16 items-center gap-2 border-b px-6"
        data-tauri-drag-region
      >
        <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
          <Network className="h-5 w-5" />
        </div>
        <span className="text-lg font-semibold">PMSynapse</span>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-4">
        {navigation.map((item) => {
          const isActive = location.pathname === item.href;
          return (
            <Link
              key={item.name}
              to={item.href}
              className={cn(
                "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                isActive
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-muted hover:text-foreground"
              )}
            >
              <item.icon className="h-5 w-5" />
              {item.name}
            </Link>
          );
        })}
      </nav>

      {/* Settings */}
      <div className="border-t p-4">
        <Link
          to="/settings"
          className="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:bg-muted hover:text-foreground"
        >
          <Settings className="h-5 w-5" />
          Settings
        </Link>
      </div>
    </div>
  );
}
