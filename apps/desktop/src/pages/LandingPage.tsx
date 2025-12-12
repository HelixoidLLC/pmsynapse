import { Link } from "react-router-dom";
import {
  Network,
  Brain,
  GitBranch,
  Users,
  Sparkles,
  ArrowRight,
  Zap,
  Shield,
  Globe,
} from "lucide-react";
import { Button } from "@/components/ui/button";

const features = [
  {
    icon: Network,
    title: "Knowledge Graph",
    description:
      "Dynamic semantic graph that evolves as your project understanding deepens. Connect ideas, decisions, and code.",
  },
  {
    icon: Brain,
    title: "AI-Powered Analysis",
    description:
      "Multi-provider LLM integration with confidence scoring. Know exactly how much AI guessed vs. confirmed.",
  },
  {
    icon: GitBranch,
    title: "IDLC Workflows",
    description:
      "Customizable Idea Development Lifecycle. Each team can define their own workflow from idea to deployment.",
  },
  {
    icon: Users,
    title: "Team Collaboration",
    description:
      "Proposals and approvals workflow. AI suggests, humans decide. Full audit trail of all changes.",
  },
];

const highlights = [
  {
    icon: Zap,
    title: "Fast",
    description: "Rust-powered backend with WASM support",
  },
  {
    icon: Shield,
    title: "Secure",
    description: "Local-first with optional cloud sync",
  },
  {
    icon: Globe,
    title: "Universal",
    description: "Desktop, browser, and CLI interfaces",
  },
];

export function LandingPage() {
  return (
    <div className="flex flex-col gap-12">
      {/* Hero Section */}
      <section className="flex flex-col items-center gap-6 text-center">
        <div className="flex h-20 w-20 items-center justify-center rounded-2xl bg-primary text-primary-foreground shadow-lg">
          <Sparkles className="h-10 w-10" />
        </div>
        <h1 className="text-4xl font-bold tracking-tight">
          Welcome to{" "}
          <span className="text-primary">PMSynapse</span>
        </h1>
        <p className="max-w-2xl text-lg text-muted-foreground">
          AI-enabled end-to-end project management. Guide ideas from inception
          to implementation with semantic knowledge graphs and customizable
          workflows.
        </p>
        <div className="flex gap-4">
          <Button asChild size="lg">
            <Link to="/dashboard">
              Get Started
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
          </Button>
          <Button variant="outline" size="lg">
            View Documentation
          </Button>
        </div>
      </section>

      {/* Highlights */}
      <section className="grid grid-cols-3 gap-4">
        {highlights.map((item) => (
          <div
            key={item.title}
            className="flex items-center gap-4 rounded-lg border bg-card p-4"
          >
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
              <item.icon className="h-5 w-5" />
            </div>
            <div>
              <h3 className="font-semibold">{item.title}</h3>
              <p className="text-sm text-muted-foreground">{item.description}</p>
            </div>
          </div>
        ))}
      </section>

      {/* Features Grid */}
      <section>
        <h2 className="mb-6 text-2xl font-semibold">Core Features</h2>
        <div className="grid grid-cols-2 gap-6">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="flex gap-4 rounded-lg border bg-card p-6"
            >
              <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
                <feature.icon className="h-6 w-6" />
              </div>
              <div>
                <h3 className="mb-2 font-semibold">{feature.title}</h3>
                <p className="text-sm text-muted-foreground">
                  {feature.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Quick Start */}
      <section className="rounded-lg border bg-card p-6">
        <h2 className="mb-4 text-xl font-semibold">Quick Start</h2>
        <div className="space-y-3">
          <div className="flex items-center gap-3">
            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-xs font-bold text-primary-foreground">
              1
            </div>
            <code className="rounded bg-muted px-2 py-1 text-sm">
              snps init
            </code>
            <span className="text-sm text-muted-foreground">
              Initialize PMSynapse in your project
            </span>
          </div>
          <div className="flex items-center gap-3">
            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-xs font-bold text-primary-foreground">
              2
            </div>
            <code className="rounded bg-muted px-2 py-1 text-sm">
              snps analyze
            </code>
            <span className="text-sm text-muted-foreground">
              Analyze your codebase and generate knowledge graph
            </span>
          </div>
          <div className="flex items-center gap-3">
            <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-xs font-bold text-primary-foreground">
              3
            </div>
            <code className="rounded bg-muted px-2 py-1 text-sm">
              snps sync
            </code>
            <span className="text-sm text-muted-foreground">
              Sync documentation with the knowledge graph
            </span>
          </div>
        </div>
      </section>

      {/* Version */}
      <footer className="text-center text-sm text-muted-foreground">
        PMSynapse v0.1.0 • Built with Rust + React + Tauri
      </footer>
    </div>
  );
}
