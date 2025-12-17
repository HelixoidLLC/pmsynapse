//! PMSynapse CLI
//!
//! Command line interface for AI-enabled project management.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use snps_core::claude::{SessionAnalyzer, SessionExporter, SessionParser};
use snps_core::config::load_global_config;
use snps_core::index::MatterIndex;
use snps_core::matter::{generate_matter_path, MatterFrontmatter, MatterItem, MatterType};
use snps_core::repository::load_repositories;
use std::path::{Path, PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use walkdir::WalkDir;

mod daemon;

/// PMSynapse CLI - AI-enabled project management
#[derive(Parser)]
#[command(name = "snps")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize PMSynapse in the current directory
    Init {
        /// Force initialization even if already initialized
        #[arg(short, long)]
        force: bool,
    },

    /// Show current status
    Status,

    /// Sync documentation with knowledge graph
    Sync {
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,

        /// Sync specific file only
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Analyze codebase and generate assumptions
    Analyze {
        /// Quick scan with fewer assumptions
        #[arg(long)]
        quick: bool,

        /// Deep analysis with more AI inference
        #[arg(long)]
        deep: bool,
    },

    /// Manage proposals from agents
    Proposals {
        #[command(subcommand)]
        action: Option<ProposalCommands>,
    },

    /// Manage workflow templates
    Templates {
        #[command(subcommand)]
        action: Option<TemplateCommands>,
    },

    /// Manage teams and IDLC configuration
    Team {
        #[command(subcommand)]
        action: Option<TeamCommands>,
    },

    /// Query the knowledge graph
    Graph {
        /// Datalog query to execute
        query: Option<String>,

        /// Export graph to file
        #[arg(long)]
        export: Option<String>,
    },

    /// Manage project thoughts (research, plans, tickets)
    Thoughts {
        #[command(subcommand)]
        action: ThoughtsCommands,
    },

    /// Manage the PMSynapse daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonCommands,
    },

    /// Launch the PMSynapse desktop UI
    Ui {
        /// Don't auto-start daemon
        #[arg(long)]
        no_daemon: bool,

        /// Use specific daemon socket
        #[arg(long)]
        daemon_socket: Option<String>,

        /// Run in detached mode
        #[arg(long)]
        detach: bool,
    },

    /// Start development environment (daemon + UI with hot reload)
    Dev {
        /// Use specific profile (isolates daemon/db per profile)
        #[arg(long)]
        profile: Option<String>,

        /// Only start daemon (no UI)
        #[arg(long)]
        daemon_only: bool,

        /// Only start UI (assumes daemon running)
        #[arg(long)]
        ui_only: bool,

        /// Custom HTTP port for daemon
        #[arg(long)]
        port: Option<u16>,
    },

    /// Manage Claude Code sessions
    Claude {
        #[command(subcommand)]
        action: ClaudeCommands,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },

    /// Manage knowledge matter
    Matter {
        #[command(subcommand)]
        action: MatterCommands,
    },

    /// Manage matter repositories
    Repo {
        #[command(subcommand)]
        action: RepoCommands,
    },
}

#[derive(Subcommand)]
enum ThoughtsCommands {
    /// Initialize thoughts for current project
    Init {
        /// Use specific profile
        #[arg(long)]
        profile: Option<String>,

        /// Storage type: local, remote, hybrid
        #[arg(long, default_value = "local")]
        storage: String,

        /// Git remote for syncing
        #[arg(long)]
        remote: Option<String>,

        /// Skip git hook installation
        #[arg(long)]
        no_hooks: bool,

        /// Force overwrite existing setup
        #[arg(short, long)]
        force: bool,
    },

    /// Create a new thought document
    New {
        /// Document type
        #[arg(value_enum)]
        doc_type: ThoughtType,

        /// Document title
        title: String,

        /// Scope: shared, personal, global
        #[arg(long, default_value = "shared")]
        scope: String,

        /// Open in editor after creation
        #[arg(long)]
        open: bool,
    },

    /// Search through thoughts
    Search {
        /// Search query
        query: String,

        /// Search scope: all, shared, personal, global
        #[arg(long, default_value = "all")]
        scope: String,

        /// Filter by type
        #[arg(long, value_enum)]
        doc_type: Option<ThoughtType>,

        /// Return only file paths (for AI agents)
        #[arg(long)]
        paths_only: bool,

        /// Limit results
        #[arg(long, default_value = "10")]
        limit: usize,
    },

    /// List thought documents
    List {
        /// Filter by scope
        #[arg(long)]
        scope: Option<String>,

        /// Filter by type
        #[arg(long, value_enum)]
        doc_type: Option<ThoughtType>,

        /// Show N most recent
        #[arg(long)]
        recent: Option<usize>,

        /// Output format: table, json, paths
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Sync thoughts with remote
    Sync {
        /// Commit message
        #[arg(short, long)]
        message: Option<String>,

        /// Push to remote after commit
        #[arg(long)]
        push: bool,

        /// Pull from remote before commit
        #[arg(long)]
        pull: bool,

        /// Only rebuild searchable index
        #[arg(long)]
        no_commit: bool,

        /// Sync direction: both, to-central, from-central
        #[arg(long, default_value = "both")]
        direction: String,
    },

    /// Open thoughts directory
    Open {
        /// Path within thoughts
        path: Option<String>,

        /// Open in editor
        #[arg(long)]
        editor: bool,

        /// Open specific scope
        #[arg(long)]
        scope: Option<String>,
    },

    /// Show thoughts configuration and status
    Status {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Manage thoughts profiles
    Profile {
        #[command(subcommand)]
        action: ProfileCommands,
    },

    /// Manage git hooks
    Hooks {
        #[command(subcommand)]
        action: HooksCommands,
    },
}

#[derive(Clone, ValueEnum)]
enum ThoughtType {
    Research,
    Plan,
    Ticket,
    Pr,
    Scratch,
    Journal,
}

impl std::fmt::Display for ThoughtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThoughtType::Research => write!(f, "research"),
            ThoughtType::Plan => write!(f, "plan"),
            ThoughtType::Ticket => write!(f, "ticket"),
            ThoughtType::Pr => write!(f, "pr"),
            ThoughtType::Scratch => write!(f, "scratch"),
            ThoughtType::Journal => write!(f, "journal"),
        }
    }
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all profiles
    List,
    /// Create a new profile
    Create {
        /// Profile name
        name: String,
        /// Local repository path
        #[arg(long)]
        repo: Option<String>,
        /// Git remote URL
        #[arg(long)]
        remote: Option<String>,
    },
    /// Switch profile for current project
    Switch {
        /// Profile name
        name: String,
    },
    /// Show current profile
    Show,
}

#[derive(Subcommand)]
enum HooksCommands {
    /// Install git hooks
    Install {
        /// Skip pre-commit hook
        #[arg(long)]
        no_pre_commit: bool,

        /// Skip post-commit hook
        #[arg(long)]
        no_post_commit: bool,

        /// Auto-sync on post-commit (default: false)
        #[arg(long)]
        auto_sync: bool,

        /// Force overwrite existing hooks
        #[arg(short, long)]
        force: bool,
    },
    /// Uninstall git hooks
    Uninstall,
    /// Check hook status
    Status,
}

#[derive(Subcommand)]
enum DaemonCommands {
    /// Start the daemon
    Start {
        /// Run in foreground (don't daemonize)
        #[arg(long)]
        foreground: bool,

        /// Custom socket path
        #[arg(long)]
        socket: Option<String>,

        /// Custom HTTP port (0 to disable)
        #[arg(long, default_value = "7878")]
        port: u16,

        /// Custom database path
        #[arg(long)]
        db: Option<String>,

        /// Profile name for isolation
        #[arg(long)]
        profile: Option<String>,
    },

    /// Stop the daemon
    Stop {
        /// Force kill if graceful shutdown fails
        #[arg(long)]
        force: bool,

        /// Specific profile to stop
        #[arg(long)]
        profile: Option<String>,
    },

    /// Show daemon status
    Status {
        /// Show detailed status
        #[arg(long)]
        detailed: bool,
    },

    /// Restart the daemon
    Restart {
        /// Profile to restart
        #[arg(long)]
        profile: Option<String>,
    },

    /// View daemon logs
    Logs {
        /// Follow log output
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,

        /// Specific profile logs
        #[arg(long)]
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
enum ProposalCommands {
    /// List pending proposals
    List {
        /// Filter by agent
        #[arg(long)]
        agent: Option<String>,
    },
    /// Approve a proposal
    Approve {
        /// Proposal ID
        id: String,
    },
    /// Reject a proposal
    Reject {
        /// Proposal ID
        id: String,
        /// Rejection reason
        #[arg(short, long)]
        message: Option<String>,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates
    List,
    /// Use a template
    Use {
        /// Template name
        name: String,
    },
    /// Validate current template
    Validate,
}

#[derive(Subcommand)]
enum TeamCommands {
    /// List all teams
    List,
    /// Show team configuration
    Show {
        /// Team ID
        team_id: Option<String>,
    },
    /// Switch active team
    Switch {
        /// Team ID
        team_id: String,
    },
}

#[derive(Subcommand)]
enum ClaudeCommands {
    /// Parse and display a Claude Code session
    Parse {
        /// Path to session JSONL file
        file: String,

        /// Output format
        #[arg(long, short, value_enum, default_value = "json")]
        format: ClaudeExportFormat,

        /// Output file (defaults to stdout)
        #[arg(long, short)]
        output: Option<String>,

        /// Pretty print JSON output
        #[arg(long)]
        pretty: bool,

        /// Save to thoughts/shared/sessions/ directory
        #[arg(long)]
        save: bool,
    },

    /// List Claude Code sessions
    List {
        /// Directory to search (defaults to ~/.claude/projects/<project-path>)
        #[arg(long, short)]
        dir: Option<String>,

        /// Show most recent N sessions
        #[arg(long, default_value = "10")]
        recent: usize,

        /// Filter by project path (overrides auto-detection from cwd)
        #[arg(long)]
        project: Option<String>,

        /// Output format: table, json, paths
        #[arg(long, default_value = "table")]
        format: String,

        /// Show all sessions including agent sub-sessions
        #[arg(long)]
        all: bool,
    },

    /// Analyze session hierarchy and statistics
    Analyze {
        /// Directory to analyze (session directory)
        dir: String,

        /// Show full message tree
        #[arg(long)]
        tree: bool,

        /// Export analysis to file
        #[arg(long, short)]
        output: Option<String>,

        /// Output format
        #[arg(long, value_enum, default_value = "json")]
        format: ClaudeExportFormat,
    },

    /// Display session message tree
    Tree {
        /// Path to session JSONL file
        file: String,

        /// Maximum depth to display
        #[arg(long, default_value = "10")]
        depth: usize,

        /// Show tool calls inline
        #[arg(long)]
        tools: bool,
    },

    /// Import all sessions from Claude projects directory to thoughts
    Import {
        /// Claude projects directory
        #[arg(long, default_value = "~/.claude/projects")]
        claude_dir: String,

        /// Output format
        #[arg(long, value_enum, default_value = "markdown")]
        format: ClaudeExportFormat,

        /// Only import main sessions (skip agents)
        #[arg(long)]
        main_only: bool,

        /// Project filter (only import sessions matching this project name)
        #[arg(long)]
        project: Option<String>,
    },

    /// Convert between session export formats (JSON <-> Markdown)
    Convert {
        /// Input file (exported JSON or Markdown)
        input: String,

        /// Output format
        #[arg(long, short, value_enum)]
        format: ClaudeExportFormat,

        /// Output file (defaults to stdout)
        #[arg(long, short)]
        output: Option<String>,

        /// Pretty print JSON output
        #[arg(long)]
        pretty: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum ClaudeExportFormat {
    Json,
    Markdown,
    Md,
    Html,
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show merged configuration
    Show {
        /// Show source for each setting
        #[arg(long)]
        source: bool,
        /// Team ID for context
        #[arg(long)]
        team: Option<String>,
        /// Project ID for context
        #[arg(long)]
        project: Option<String>,
    },

    /// Sync config from shadow repo
    Sync {
        /// Project to sync
        #[arg(long)]
        project: Option<String>,
        /// Sync all projects
        #[arg(long)]
        all: bool,
    },

    /// Push config changes to shadow repo
    Push {
        /// Push team-level changes
        #[arg(long)]
        team: bool,
    },

    /// Initialize config for current context
    Init {
        /// Context type (user, team, project)
        #[arg(long)]
        context: String,
        /// Context ID
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum MatterCommands {
    /// Create new matter item
    Create {
        /// Matter type (spec, document, research, plan, insight)
        matter_type: String,
        /// Title of the matter item
        title: String,
        /// Context type (user, team, project)
        #[arg(long, default_value = "user")]
        context: String,
        /// Context ID
        #[arg(long)]
        id: Option<String>,
        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
        /// Visibility (private, shared)
        #[arg(long, default_value = "private")]
        visibility: String,
    },

    /// List matter items
    List {
        /// Filter by context type
        #[arg(long)]
        context: Option<String>,
        /// Filter by context ID
        #[arg(long)]
        id: Option<String>,
        /// Filter by matter type
        #[arg(long, short = 't')]
        matter_type: Option<String>,
        /// Filter by visibility
        #[arg(long)]
        visibility: Option<String>,
        /// Maximum results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Search matter items
    Search {
        /// Search query
        query: String,
        /// Filter by context
        #[arg(long)]
        context: Option<String>,
        /// Filter by type
        #[arg(long, short = 't')]
        matter_type: Option<String>,
        /// Filter by tags
        #[arg(long)]
        tags: Option<String>,
        /// Maximum results
        #[arg(long, default_value = "20")]
        limit: usize,
    },

    /// Show matter item details
    Show {
        /// Matter ID or file path
        matter_id: String,
    },

    /// Edit matter item
    Edit {
        /// Matter ID or file path
        matter_id: String,
    },

    /// Delete matter item
    Delete {
        /// Matter ID or file path
        matter_id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },

    /// Import matter from file
    Import {
        /// File to import
        file: PathBuf,
        /// Matter type to assign
        #[arg(long)]
        matter_type: Option<String>,
        /// Context for imported matter
        #[arg(long)]
        context: Option<String>,
    },
}

#[derive(Subcommand)]
enum RepoCommands {
    /// Initialize new matter repository
    Init {
        /// Repository path
        path: PathBuf,
        /// Context type (user, team, project)
        #[arg(long)]
        context: String,
        /// Context ID
        #[arg(long)]
        id: String,
        /// Owner type for projects
        #[arg(long)]
        owner_type: Option<String>,
        /// Owner ID for projects
        #[arg(long)]
        owner_id: Option<String>,
    },

    /// Clone remote repository
    Clone {
        /// Remote URL
        url: String,
        /// Local path
        path: Option<PathBuf>,
    },

    /// Add existing repository
    Add {
        /// Repository path
        path: PathBuf,
    },

    /// Remove repository from config
    Remove {
        /// Repository ID
        id: String,
    },

    /// List configured repositories
    List,

    /// Sync repository with remote
    Sync {
        /// Repository ID (optional, syncs all if omitted)
        id: Option<String>,
    },

    /// Rebuild repository index
    Index {
        /// Repository ID (optional, rebuilds all if omitted)
        id: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| log_level.to_string()),
        ))
        .init();

    // Print banner
    println!(
        "\n{}",
        "╔═══════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║       PMSynapse - AI Project Mgmt     ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════╝".bright_cyan()
    );
    println!();

    match cli.command {
        Commands::Init { force } => cmd_init(force),
        Commands::Status => cmd_status(),
        Commands::Sync { dry_run, file } => cmd_sync(dry_run, file),
        Commands::Analyze { quick, deep } => cmd_analyze(quick, deep),
        Commands::Proposals { action } => cmd_proposals(action),
        Commands::Templates { action } => cmd_templates(action),
        Commands::Team { action } => cmd_team(action),
        Commands::Graph { query, export } => cmd_graph(query, export),
        Commands::Thoughts { action } => cmd_thoughts(action),
        Commands::Daemon { action } => cmd_daemon(action),
        Commands::Ui {
            no_daemon,
            daemon_socket,
            detach,
        } => cmd_ui(no_daemon, daemon_socket, detach),
        Commands::Dev {
            profile,
            daemon_only,
            ui_only,
            port,
        } => cmd_dev(profile, daemon_only, ui_only, port),
        Commands::Claude { action } => cmd_claude(action),
        Commands::Config { action } => cmd_config(action),
        Commands::Matter { action } => cmd_matter(action),
        Commands::Repo { action } => cmd_repo(action),
    }
}

fn cmd_init(force: bool) -> anyhow::Result<()> {
    println!("{}", "Initializing PMSynapse...".bright_green());

    if force {
        println!("{}", "  Force mode enabled".yellow());
    }

    // Check if already initialized
    let config_path = std::path::Path::new(".pmsynapse");
    if config_path.exists() && !force {
        println!(
            "{}",
            "  Already initialized. Use --force to reinitialize.".yellow()
        );
        return Ok(());
    }

    // Create directory structure
    std::fs::create_dir_all(".pmsynapse/teams/default")?;
    std::fs::create_dir_all(".pmsynapse/templates")?;

    // Create default config
    let config = r#"# PMSynapse Configuration
version: "1.0"
team: default

llm:
  default_provider: anthropic
  providers:
    - name: anthropic
      enabled: true

graph:
  path: .pmsynapse/synapse.db

sync:
  auto: false
"#;

    std::fs::write(".pmsynapse/config.yaml", config)?;

    // Create default IDLC
    let idlc_config = snps_core::idlc::IdlcConfig::default_config();
    let idlc_yaml = serde_json::to_string_pretty(&idlc_config)?;
    std::fs::write(".pmsynapse/teams/default/idlc.json", idlc_yaml)?;

    println!("{}", "  ✓ Created .pmsynapse directory".green());
    println!("{}", "  ✓ Created default configuration".green());
    println!("{}", "  ✓ Created default IDLC workflow".green());
    println!();
    println!("{}", "PMSynapse initialized successfully!".bright_green());

    Ok(())
}

fn cmd_status() -> anyhow::Result<()> {
    println!("{}", "PMSynapse Status".bright_blue());
    println!();

    // Check initialization
    let config_path = std::path::Path::new(".pmsynapse");
    if !config_path.exists() {
        println!("{}", "  Not initialized. Run 'snps init' first.".yellow());
        return Ok(());
    }

    println!("{}", "  ✓ Initialized".green());
    println!("  Version: {}", snps_core::VERSION);
    println!("  Config: .pmsynapse/config.yaml");

    // TODO: Add more status info (graph stats, pending proposals, etc.)

    Ok(())
}

fn cmd_sync(dry_run: bool, file: Option<String>) -> anyhow::Result<()> {
    println!("{}", "Syncing documentation with graph...".bright_blue());

    if dry_run {
        println!("{}", "  (dry run - no changes will be made)".yellow());
    }

    if let Some(f) = file {
        println!("  File: {}", f);
    }

    // TODO: Implement sync logic
    println!("{}", "  ✓ Sync complete (no changes)".green());

    Ok(())
}

fn cmd_analyze(quick: bool, deep: bool) -> anyhow::Result<()> {
    let mode = if quick {
        "quick"
    } else if deep {
        "deep"
    } else {
        "standard"
    };

    println!(
        "{}",
        format!("Analyzing codebase ({} mode)...", mode).bright_blue()
    );

    // TODO: Implement analysis
    println!("{}", "  ✓ Analysis complete".green());

    Ok(())
}

fn cmd_proposals(action: Option<ProposalCommands>) -> anyhow::Result<()> {
    match action {
        Some(ProposalCommands::List { agent }) => {
            println!("{}", "Pending Proposals".bright_blue());
            if let Some(a) = agent {
                println!("  (filtered by agent: {})", a);
            }
            println!("{}", "  No pending proposals".dimmed());
        }
        Some(ProposalCommands::Approve { id }) => {
            println!("{}", format!("Approving proposal {}...", id).bright_green());
            // TODO: Implement approval
        }
        Some(ProposalCommands::Reject { id, message }) => {
            println!("{}", format!("Rejecting proposal {}...", id).bright_red());
            if let Some(m) = message {
                println!("  Reason: {}", m);
            }
            // TODO: Implement rejection
        }
        None => {
            println!("{}", "Pending Proposals".bright_blue());
            println!("{}", "  No pending proposals".dimmed());
        }
    }

    Ok(())
}

fn cmd_templates(action: Option<TemplateCommands>) -> anyhow::Result<()> {
    match action {
        Some(TemplateCommands::List) => {
            println!("{}", "Available Templates".bright_blue());
            println!("  • bmad (default)");
            println!("  • custom");
        }
        Some(TemplateCommands::Use { name }) => {
            println!(
                "{}",
                format!("Switching to template: {}", name).bright_green()
            );
            // TODO: Implement template switching
        }
        Some(TemplateCommands::Validate) => {
            println!("{}", "Validating template...".bright_blue());
            println!("{}", "  ✓ Template is valid".green());
        }
        None => {
            println!("{}", "Available Templates".bright_blue());
            println!("  • bmad (default)");
        }
    }

    Ok(())
}

fn cmd_team(action: Option<TeamCommands>) -> anyhow::Result<()> {
    match action {
        Some(TeamCommands::List) => {
            println!("{}", "Teams".bright_blue());
            println!("  • default (active)");
        }
        Some(TeamCommands::Show { team_id }) => {
            let id = team_id.unwrap_or_else(|| "default".to_string());
            println!("{}", format!("Team: {}", id).bright_blue());
            // TODO: Show team config
        }
        Some(TeamCommands::Switch { team_id }) => {
            println!(
                "{}",
                format!("Switching to team: {}", team_id).bright_green()
            );
            // TODO: Implement team switching
        }
        None => {
            println!("{}", "Teams".bright_blue());
            println!("  • default (active)");
        }
    }

    Ok(())
}

fn cmd_graph(query: Option<String>, export: Option<String>) -> anyhow::Result<()> {
    if let Some(q) = query {
        println!("{}", "Executing query...".bright_blue());
        println!("  Query: {}", q);
        // TODO: Execute query
        println!("{}", "  (no results)".dimmed());
    } else if let Some(path) = export {
        println!(
            "{}",
            format!("Exporting graph to {}...", path).bright_blue()
        );
        // TODO: Export graph
        println!("{}", "  ✓ Export complete".green());
    } else {
        println!("{}", "Knowledge Graph".bright_blue());
        println!("  Nodes: 0");
        println!("  Edges: 0");
    }

    Ok(())
}

// =============================================================================
// Daemon, UI, and Dev Commands
// =============================================================================

fn get_pmsynapse_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".pmsynapse")
}

fn get_daemon_socket_path(profile: Option<&str>) -> PathBuf {
    let base = get_pmsynapse_dir();
    match profile {
        Some(p) => base.join(format!("daemon-{}.sock", p)),
        None => base.join("daemon.sock"),
    }
}

fn get_daemon_db_path(profile: Option<&str>) -> PathBuf {
    let base = get_pmsynapse_dir();
    match profile {
        Some(p) => base.join(format!("synapse-{}.db", p)),
        None => base.join("synapse.db"),
    }
}

fn get_daemon_pid_path(profile: Option<&str>) -> PathBuf {
    let base = get_pmsynapse_dir();
    match profile {
        Some(p) => base.join(format!("daemon-{}.pid", p)),
        None => base.join("daemon.pid"),
    }
}

fn get_daemon_log_path(profile: Option<&str>) -> PathBuf {
    let base = get_pmsynapse_dir().join("logs");
    match profile {
        Some(p) => base.join(format!("daemon-{}.log", p)),
        None => base.join("daemon.log"),
    }
}

fn is_daemon_running(profile: Option<&str>) -> bool {
    let pid_path = get_daemon_pid_path(profile);
    if !pid_path.exists() {
        return false;
    }

    if let Ok(pid_str) = std::fs::read_to_string(&pid_path) {
        // Handle both "pid" and "pid:port" formats
        let pid_part = pid_str.split(':').next().unwrap_or(&pid_str);
        if let Ok(pid) = pid_part.trim().parse::<i32>() {
            // Check if process is running
            #[cfg(unix)]
            {
                use std::process::Command;
                let output = Command::new("kill").args(["-0", &pid.to_string()]).output();
                return output.map(|o| o.status.success()).unwrap_or(false);
            }
            #[cfg(windows)]
            {
                // On Windows, check with tasklist
                let output = std::process::Command::new("tasklist")
                    .args(["/FI", &format!("PID eq {}", pid)])
                    .output();
                return output
                    .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
                    .unwrap_or(false);
            }
        }
    }
    false
}

fn wait_for_health(port: u16) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://127.0.0.1:{}/api/v1/health", port);

    for _ in 0..20 {
        if client.get(&url).send().is_ok() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    anyhow::bail!("Daemon did not become healthy")
}

fn cmd_daemon(action: DaemonCommands) -> anyhow::Result<()> {
    match action {
        DaemonCommands::Start {
            foreground,
            socket,
            port,
            db,
            profile,
        } => daemon_start(foreground, socket, port, db, profile),

        DaemonCommands::Stop { force, profile } => daemon_stop(force, profile),

        DaemonCommands::Status { detailed } => daemon_status(detailed),

        DaemonCommands::Restart { profile } => {
            daemon_stop(false, profile.clone())?;
            std::thread::sleep(std::time::Duration::from_millis(500));
            daemon_start(false, None, 7878, None, profile)
        }

        DaemonCommands::Logs {
            follow,
            lines,
            profile,
        } => daemon_logs(follow, lines, profile),
    }
}

fn daemon_start(
    foreground: bool,
    socket: Option<String>,
    port: u16,
    db: Option<String>,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let profile_ref = profile.as_deref();

    if is_daemon_running(profile_ref) {
        println!(
            "{}",
            format!(
                "Daemon already running{}",
                profile_ref
                    .map(|p| format!(" (profile: {})", p))
                    .unwrap_or_default()
            )
            .yellow()
        );
        return Ok(());
    }

    println!("{}", "🚀 Starting PMSynapse daemon...".bright_cyan());

    // Ensure directories exist
    let pmsynapse_dir = get_pmsynapse_dir();
    std::fs::create_dir_all(&pmsynapse_dir)?;
    std::fs::create_dir_all(pmsynapse_dir.join("logs"))?;

    let socket_path = socket
        .map(PathBuf::from)
        .unwrap_or_else(|| get_daemon_socket_path(profile_ref));
    let db_path = db
        .map(PathBuf::from)
        .unwrap_or_else(|| get_daemon_db_path(profile_ref));
    let _pid_path = get_daemon_pid_path(profile_ref);
    let _log_path = get_daemon_log_path(profile_ref);

    println!("  Socket:   {}", socket_path.display());
    println!("  Database: {}", db_path.display());
    println!(
        "  HTTP:     {}",
        if port > 0 {
            format!("http://127.0.0.1:{}", port)
        } else {
            "disabled".to_string()
        }
    );
    if let Some(p) = &profile {
        println!("  Profile:  {}", p.bright_cyan());
    }

    if foreground {
        println!();
        println!("{}", "Running in foreground (Ctrl+C to stop)...".dimmed());
        println!();

        // Run in foreground (blocking)
        let runtime = tokio::runtime::Runtime::new()?;
        let db_path_str = db_path.to_string_lossy().to_string();
        runtime.block_on(async move {
            let server = daemon::DaemonServer::new(port)?;
            let actual_port = server.port();

            // Write PID file with port info
            let pid_path = get_daemon_pid_path(profile_ref);
            std::fs::write(&pid_path, format!("{}:{}", std::process::id(), actual_port))?;

            server.run(&db_path_str).await
        })
    } else {
        // Background mode - fork and detach
        println!();
        println!("{}", "⚠ Background daemon not yet implemented".yellow());
        println!("  Use --foreground to run in foreground mode");
        println!();
        Ok(())
    }
}

fn daemon_stop(force: bool, profile: Option<String>) -> anyhow::Result<()> {
    let profile_ref = profile.as_deref();

    println!("{}", "Stopping PMSynapse daemon...".bright_blue());

    let pid_path = get_daemon_pid_path(profile_ref);

    if !pid_path.exists() {
        println!("{}", "  Daemon is not running".dimmed());
        return Ok(());
    }

    if let Ok(pid_str) = std::fs::read_to_string(&pid_path) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            #[cfg(unix)]
            {
                use std::process::Command;
                let signal = if force { "-9" } else { "-15" };
                let result = Command::new("kill")
                    .args([signal, &pid.to_string()])
                    .output();

                match result {
                    Ok(output) if output.status.success() => {
                        println!("{}", "  ✓ Daemon stopped".green());
                    }
                    _ => {
                        println!("{}", "  ✗ Failed to stop daemon".red());
                    }
                }
            }

            #[cfg(windows)]
            {
                let result = std::process::Command::new("taskkill")
                    .args(if force {
                        vec!["/F", "/PID", &pid.to_string()]
                    } else {
                        vec!["/PID", &pid.to_string()]
                    })
                    .output();

                match result {
                    Ok(output) if output.status.success() => {
                        println!("{}", "  ✓ Daemon stopped".green());
                    }
                    _ => {
                        println!("{}", "  ✗ Failed to stop daemon".red());
                    }
                }
            }
        }
    }

    // Clean up PID file
    let _ = std::fs::remove_file(&pid_path);

    // Clean up socket file
    let socket_path = get_daemon_socket_path(profile_ref);
    let _ = std::fs::remove_file(&socket_path);

    Ok(())
}

fn daemon_status(detailed: bool) -> anyhow::Result<()> {
    println!("{}", "PMSynapse Daemon Status".bright_blue());
    println!();

    // Check default daemon
    let running = is_daemon_running(None);
    if running {
        println!("  {} Default daemon: {}", "●".green(), "running".green());
    } else {
        println!("  {} Default daemon: {}", "○".dimmed(), "stopped".dimmed());
    }

    // Check for profile daemons
    let pmsynapse_dir = get_pmsynapse_dir();
    if pmsynapse_dir.exists() {
        for entry in std::fs::read_dir(&pmsynapse_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("daemon-") && name.ends_with(".pid") {
                let profile = name
                    .strip_prefix("daemon-")
                    .and_then(|s| s.strip_suffix(".pid"))
                    .unwrap_or("unknown");

                let running = is_daemon_running(Some(profile));
                if running {
                    println!(
                        "  {} Profile '{}': {}",
                        "●".green(),
                        profile.bright_cyan(),
                        "running".green()
                    );
                } else {
                    println!(
                        "  {} Profile '{}': {}",
                        "○".dimmed(),
                        profile,
                        "stopped".dimmed()
                    );
                }
            }
        }
    }

    if detailed {
        println!();
        println!("{}", "Paths:".bright_blue());
        println!("  Config:   {}", get_pmsynapse_dir().display());
        println!("  Socket:   {}", get_daemon_socket_path(None).display());
        println!("  Database: {}", get_daemon_db_path(None).display());
        println!("  Logs:     {}", get_daemon_log_path(None).display());
    }

    Ok(())
}

fn daemon_logs(follow: bool, lines: usize, profile: Option<String>) -> anyhow::Result<()> {
    let log_path = get_daemon_log_path(profile.as_deref());

    if !log_path.exists() {
        println!("{}", "No log file found".dimmed());
        return Ok(());
    }

    if follow {
        // Use tail -f
        #[cfg(unix)]
        {
            let mut child = std::process::Command::new("tail")
                .args(["-f", "-n", &lines.to_string()])
                .arg(&log_path)
                .spawn()?;
            child.wait()?;
        }

        #[cfg(windows)]
        {
            println!("{}", "Follow mode not supported on Windows".yellow());
            // Fall through to show last N lines
        }
    }

    // Show last N lines
    let content = std::fs::read_to_string(&log_path)?;
    let all_lines: Vec<&str> = content.lines().collect();
    let start = all_lines.len().saturating_sub(lines);

    for line in &all_lines[start..] {
        println!("{}", line);
    }

    Ok(())
}

fn cmd_ui(no_daemon: bool, daemon_socket: Option<String>, detach: bool) -> anyhow::Result<()> {
    println!("{}", "🖥️  Launching PMSynapse UI...".bright_cyan());
    println!();

    let daemon_port = if !no_daemon {
        if is_daemon_running(None) {
            // Daemon already running, read port from PID file
            let pid_path = get_daemon_pid_path(None);
            if let Ok(content) = std::fs::read_to_string(&pid_path) {
                if let Some((_pid, port_str)) = content.split_once(':') {
                    port_str.parse::<u16>().ok()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // Start daemon in background
            println!("{}", "Starting daemon...".dimmed());

            let mut child = std::process::Command::new(std::env::current_exe()?)
                .args(["daemon", "start", "--foreground", "--port", "0"])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()?;

            // Read port from stdout
            use std::io::BufRead;
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let reader = std::io::BufReader::new(stdout);

            let mut port = None;
            for line in reader.lines() {
                let line = line?;
                if let Some(port_str) = line.strip_prefix("HTTP_PORT=") {
                    port = Some(port_str.parse()?);
                    break;
                }
            }

            if let Some(p) = port {
                // Wait for daemon to be ready
                wait_for_health(p)?;
                println!("  Daemon started on port {}", p);
                Some(p)
            } else {
                println!("{}", "⚠ Failed to get daemon port".yellow());
                None
            }
        }
    } else {
        None
    };

    // Find the desktop app
    let project_root = find_project_root()?;
    let desktop_dir = project_root.join("apps").join("desktop");

    if !desktop_dir.exists() {
        println!(
            "{}",
            "Desktop app not found. Run from PMSynapse project root.".red()
        );
        return Ok(());
    }

    // Set environment variables for daemon connection
    let socket_path =
        daemon_socket.unwrap_or_else(|| get_daemon_socket_path(None).to_string_lossy().to_string());

    println!("  Desktop: {}", desktop_dir.display());
    println!("  Socket:  {}", socket_path);
    if let Some(port) = daemon_port {
        println!("  Daemon:  http://127.0.0.1:{}", port);
    }
    println!();

    // Build and run the Tauri app
    let mut cmd = std::process::Command::new("pnpm");
    cmd.args(["tauri", "dev"])
        .current_dir(&desktop_dir)
        .env("PMSYNAPSE_DAEMON_SOCKET", &socket_path);

    if let Some(port) = daemon_port {
        cmd.env("PMSYNAPSE_DAEMON_PORT", port.to_string());
    }

    if detach {
        println!("{}", "Launching in detached mode...".dimmed());
        cmd.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }

        cmd.spawn()?;
        println!("{}", "✓ UI launched in background".green());
    } else {
        println!("{}", "Running UI (Ctrl+C to stop)...".dimmed());
        println!();
        let mut child = cmd.spawn()?;
        child.wait()?;
    }

    Ok(())
}

fn cmd_dev(
    profile: Option<String>,
    daemon_only: bool,
    ui_only: bool,
    port: Option<u16>,
) -> anyhow::Result<()> {
    println!(
        "{}",
        "🔧 Starting PMSynapse development environment...".bright_cyan()
    );
    println!();

    let profile_name = profile.clone().unwrap_or_else(|| "dev".to_string());
    let http_port = port.unwrap_or(7878);

    println!("  Profile: {}", profile_name.bright_cyan());
    println!(
        "  Mode:    {}",
        if daemon_only {
            "daemon only"
        } else if ui_only {
            "UI only"
        } else {
            "full stack"
        }
        .bright_yellow()
    );
    println!();

    let project_root = find_project_root()?;

    // Start daemon (unless UI-only mode)
    if !ui_only {
        println!("{}", "Starting development daemon...".bright_blue());

        let socket_path = get_daemon_socket_path(Some(&profile_name));
        let db_path = get_daemon_db_path(Some(&profile_name));

        println!("  Socket:   {}", socket_path.display());
        println!("  Database: {}", db_path.display());
        println!("  HTTP:     http://127.0.0.1:{}", http_port);

        if is_daemon_running(Some(&profile_name)) {
            println!("{}", "  Daemon already running".yellow());
        } else {
            // In a real implementation, start the daemon in background
            println!(
                "{}",
                "  ⚠ Daemon placeholder (not yet implemented)".yellow()
            );
        }
        println!();
    }

    // Start UI (unless daemon-only mode)
    if !daemon_only {
        println!("{}", "Starting development UI...".bright_blue());

        let desktop_dir = project_root.join("apps").join("desktop");
        if !desktop_dir.exists() {
            println!(
                "{}",
                "Desktop app not found. Run from PMSynapse project root.".red()
            );
            return Ok(());
        }

        let socket_path = get_daemon_socket_path(Some(&profile_name));

        println!("  Desktop: {}", desktop_dir.display());
        println!("  Socket:  {}", socket_path.display());
        println!();

        // Run Tauri dev mode
        let mut cmd = std::process::Command::new("pnpm");
        cmd.args(["tauri", "dev"])
            .current_dir(&desktop_dir)
            .env(
                "PMSYNAPSE_DAEMON_SOCKET",
                socket_path.to_string_lossy().to_string(),
            )
            .env("PMSYNAPSE_DEV_MODE", "true")
            .env("PMSYNAPSE_PROFILE", &profile_name);

        println!(
            "{}",
            "Running development server (Ctrl+C to stop)...".dimmed()
        );
        println!();

        let mut child = cmd.spawn()?;
        child.wait()?;
    } else {
        // Daemon-only mode - just wait
        println!("{}", "Daemon running. Press Ctrl+C to stop...".dimmed());
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}

fn find_project_root() -> anyhow::Result<PathBuf> {
    let mut current = std::env::current_dir()?;

    loop {
        // Check for PMSynapse markers
        if current.join("apps").join("desktop").exists()
            || current.join(".pmsynapse").exists()
            || current.join("Cargo.toml").exists() && current.join("apps").exists()
        {
            return Ok(current);
        }

        if !current.pop() {
            // Reached root without finding project
            return Ok(std::env::current_dir()?);
        }
    }
}

// =============================================================================
// Thoughts Commands
// =============================================================================

fn cmd_thoughts(action: ThoughtsCommands) -> anyhow::Result<()> {
    match action {
        ThoughtsCommands::Init {
            profile,
            storage,
            remote,
            no_hooks,
            force,
        } => thoughts_init(profile, storage, remote, no_hooks, force),

        ThoughtsCommands::New {
            doc_type,
            title,
            scope,
            open,
        } => thoughts_new(doc_type, title, scope, open),

        ThoughtsCommands::Search {
            query,
            scope,
            doc_type,
            paths_only,
            limit,
        } => thoughts_search(query, scope, doc_type, paths_only, limit),

        ThoughtsCommands::List {
            scope,
            doc_type,
            recent,
            format,
        } => thoughts_list(scope, doc_type, recent, format),

        ThoughtsCommands::Sync {
            message,
            push,
            pull,
            no_commit,
            direction,
        } => thoughts_sync(message, push, pull, no_commit, direction),

        ThoughtsCommands::Open {
            path,
            editor,
            scope,
        } => thoughts_open(path, editor, scope),

        ThoughtsCommands::Status { verbose } => thoughts_status(verbose),

        ThoughtsCommands::Profile { action } => thoughts_profile(action),

        ThoughtsCommands::Hooks { action } => thoughts_hooks(action),
    }
}

fn get_thoughts_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".pmsynapse")
        .join("thoughts")
}

fn get_username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "user".to_string())
}

fn thoughts_init(
    profile: Option<String>,
    storage: String,
    remote: Option<String>,
    no_hooks: bool,
    force: bool,
) -> anyhow::Result<()> {
    println!("{}", "🧠 Initializing PMSynapse Thoughts...".bright_cyan());
    println!();

    let thoughts_path = Path::new("thoughts");
    if thoughts_path.exists() && !force {
        println!(
            "{}",
            "  Thoughts already initialized. Use --force to reinitialize.".yellow()
        );
        return Ok(());
    }

    let profile_name = profile.unwrap_or_else(|| "personal".to_string());
    let username = get_username();

    // Create global thoughts directory
    let thoughts_root = get_thoughts_dir();
    let profile_dir = thoughts_root.join("profiles").join(&profile_name);
    let project_name = std::env::current_dir()?
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let project_thoughts = profile_dir.join("projects").join(&project_name);

    // Create directory structure
    std::fs::create_dir_all(project_thoughts.join("shared").join("research"))?;
    std::fs::create_dir_all(project_thoughts.join("shared").join("plans"))?;
    std::fs::create_dir_all(project_thoughts.join("shared").join("tickets"))?;
    std::fs::create_dir_all(project_thoughts.join("shared").join("prs"))?;
    std::fs::create_dir_all(project_thoughts.join(&username).join("tickets"))?;
    std::fs::create_dir_all(project_thoughts.join(&username).join("journal"))?;
    std::fs::create_dir_all(thoughts_root.join("global").join("patterns"))?;
    std::fs::create_dir_all(thoughts_root.join("global").join("learnings"))?;
    std::fs::create_dir_all(thoughts_root.join("global").join("templates"))?;

    println!(
        "{}",
        format!(
            "  ✓ Created thoughts directory: {}",
            project_thoughts.display()
        )
        .green()
    );

    // Create symlink in project
    if thoughts_path.exists() && force {
        if thoughts_path.is_symlink() {
            std::fs::remove_file(thoughts_path)?;
        } else {
            std::fs::remove_dir_all(thoughts_path)?;
        }
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&project_thoughts, thoughts_path)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&project_thoughts, thoughts_path)?;

    println!(
        "{}",
        "  ✓ Created symlink: thoughts/ → ~/.pmsynapse/thoughts/...".green()
    );

    // Create global symlink
    let global_link = thoughts_path.join("global");
    if !global_link.exists() {
        #[cfg(unix)]
        std::os::unix::fs::symlink(thoughts_root.join("global"), &global_link)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(thoughts_root.join("global"), &global_link)?;
    }

    // Update .gitignore
    let gitignore = Path::new(".gitignore");
    let gitignore_entry = "thoughts/";
    if gitignore.exists() {
        let content = std::fs::read_to_string(gitignore)?;
        if !content.contains(gitignore_entry) {
            let mut file = std::fs::OpenOptions::new().append(true).open(gitignore)?;
            use std::io::Write;
            writeln!(file, "\n# PMSynapse thoughts (symlinked, do not commit)")?;
            writeln!(file, "{}", gitignore_entry)?;
            println!("{}", "  ✓ Added thoughts/ to .gitignore".green());
        }
    } else {
        std::fs::write(
            gitignore,
            "# PMSynapse thoughts (symlinked, do not commit)\nthoughts/\n",
        )?;
        println!("{}", "  ✓ Created .gitignore with thoughts/ entry".green());
    }

    // Install git hooks
    if !no_hooks {
        install_thoughts_hooks(false, true, false, false)?;
    }

    // Store configuration
    let config = format!(
        r#"# Thoughts configuration for this project
profile: {}
storage: {}
remote: {}
username: {}
"#,
        profile_name,
        storage,
        remote.as_deref().unwrap_or("null"),
        username
    );
    std::fs::write(project_thoughts.join(".thoughts.yaml"), config)?;

    println!();
    println!("{}", "✅ Thoughts initialized successfully!".bright_green());
    println!();
    println!("  Profile:  {}", profile_name.bright_cyan());
    println!("  Storage:  {}", storage.bright_cyan());
    println!("  Username: {}", username.bright_cyan());
    println!();
    println!("{}", "Next steps:".bright_blue());
    println!("  • Create research: snps thoughts new research \"Topic Name\"");
    println!("  • Create a plan:   snps thoughts new plan \"Feature Name\"");
    println!("  • Search thoughts: snps thoughts search \"query\"");

    Ok(())
}

fn thoughts_new(
    doc_type: ThoughtType,
    title: String,
    scope: String,
    open: bool,
) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");
    if !thoughts_path.exists() {
        println!(
            "{}",
            "Thoughts not initialized. Run 'snps thoughts init' first.".red()
        );
        return Ok(());
    }

    let username = get_username();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Determine path based on scope and type
    let (subdir, _is_personal) = match scope.as_str() {
        "personal" => (username.clone(), true),
        "global" => ("global".to_string(), false),
        _ => ("shared".to_string(), false),
    };

    let type_dir = match doc_type {
        ThoughtType::Research => "research",
        ThoughtType::Plan => "plans",
        ThoughtType::Ticket => "tickets",
        ThoughtType::Pr => "prs",
        ThoughtType::Scratch => "",
        ThoughtType::Journal => "journal",
    };

    // Create filename from title
    let filename = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    let dir_path = if type_dir.is_empty() {
        thoughts_path.join(&subdir)
    } else {
        thoughts_path.join(&subdir).join(type_dir)
    };

    std::fs::create_dir_all(&dir_path)?;

    let file_path = dir_path.join(format!("{}.md", filename));

    if file_path.exists() {
        println!(
            "{}",
            format!("File already exists: {}", file_path.display()).yellow()
        );
        return Ok(());
    }

    // Generate content from template
    let content = match doc_type {
        ThoughtType::Research => generate_research_template(&title, &username, &date),
        ThoughtType::Plan => generate_plan_template(&title, &username, &date),
        ThoughtType::Ticket => generate_ticket_template(&title, &username, &date),
        ThoughtType::Pr => generate_pr_template(&title, &username, &date),
        ThoughtType::Scratch => generate_scratch_template(&title, &date),
        ThoughtType::Journal => generate_journal_template(&date),
    };

    std::fs::write(&file_path, content)?;

    println!(
        "{}",
        format!("✓ Created: {}", file_path.display()).bright_green()
    );

    if open {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string());
        std::process::Command::new(&editor)
            .arg(&file_path)
            .spawn()?;
        println!("  Opening in {}...", editor);
    }

    Ok(())
}

fn thoughts_search(
    query: String,
    scope: String,
    doc_type: Option<ThoughtType>,
    paths_only: bool,
    limit: usize,
) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");
    if !thoughts_path.exists() {
        println!(
            "{}",
            "Thoughts not initialized. Run 'snps thoughts init' first.".red()
        );
        return Ok(());
    }

    // Determine search paths based on scope
    let search_paths: Vec<PathBuf> = match scope.as_str() {
        "shared" => vec![thoughts_path.join("shared")],
        "personal" => vec![thoughts_path.join(get_username())],
        "global" => vec![thoughts_path.join("global")],
        _ => vec![
            thoughts_path.join("shared"),
            thoughts_path.join(get_username()),
            thoughts_path.join("global"),
        ],
    };

    // Filter by type if specified
    let type_filter = doc_type.map(|t| match t {
        ThoughtType::Research => "research",
        ThoughtType::Plan => "plans",
        ThoughtType::Ticket => "tickets",
        ThoughtType::Pr => "prs",
        ThoughtType::Scratch => "",
        ThoughtType::Journal => "journal",
    });

    if !paths_only {
        println!("{}", format!("Searching for: {}", query).bright_blue());
        println!();
    }

    let mut results = Vec::new();

    for search_path in &search_paths {
        if !search_path.exists() {
            continue;
        }

        // Use ripgrep if available, otherwise fall back to simple search
        let output = std::process::Command::new("rg")
            .args([
                "--files-with-matches",
                "--ignore-case",
                &query,
                &search_path.to_string_lossy(),
            ])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let path = PathBuf::from(line);

                // Apply type filter
                if let Some(filter) = &type_filter {
                    if !filter.is_empty() && !path.to_string_lossy().contains(filter) {
                        continue;
                    }
                }

                results.push(path);
                if results.len() >= limit {
                    break;
                }
            }
        }
    }

    if paths_only {
        for path in &results {
            println!("{}", path.display());
        }
    } else if results.is_empty() {
        println!("{}", "  No results found.".dimmed());
    } else {
        println!("{}", format!("Found {} results:", results.len()).green());
        for path in &results {
            println!("  • {}", path.display());
        }
    }

    Ok(())
}

fn thoughts_list(
    scope: Option<String>,
    doc_type: Option<ThoughtType>,
    recent: Option<usize>,
    format: String,
) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");
    if !thoughts_path.exists() {
        println!(
            "{}",
            "Thoughts not initialized. Run 'snps thoughts init' first.".red()
        );
        return Ok(());
    }

    println!("{}", "Thought Documents".bright_blue());
    println!();

    // Collect all markdown files
    let mut files: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();

    fn collect_files(dir: &Path, files: &mut Vec<(PathBuf, std::time::SystemTime)>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_files(&path, files);
                } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(meta) = path.metadata() {
                        if let Ok(modified) = meta.modified() {
                            files.push((path, modified));
                        }
                    }
                }
            }
        }
    }

    let search_dirs: Vec<PathBuf> = match scope.as_deref() {
        Some("shared") => vec![thoughts_path.join("shared")],
        Some("personal") => vec![thoughts_path.join(get_username())],
        Some("global") => vec![thoughts_path.join("global")],
        _ => vec![
            thoughts_path.join("shared"),
            thoughts_path.join(get_username()),
            thoughts_path.join("global"),
        ],
    };

    for dir in search_dirs {
        if dir.exists() {
            collect_files(&dir, &mut files);
        }
    }

    // Sort by modification time (newest first)
    files.sort_by(|a, b| b.1.cmp(&a.1));

    // Apply recent filter
    if let Some(n) = recent {
        files.truncate(n);
    }

    // Apply type filter
    if let Some(dt) = doc_type {
        let type_str = dt.to_string();
        files.retain(|(path, _)| path.to_string_lossy().contains(&type_str));
    }

    match format.as_str() {
        "json" => {
            let json_files: Vec<_> = files.iter().map(|(p, _)| p.to_string_lossy()).collect();
            println!("{}", serde_json::to_string_pretty(&json_files)?);
        }
        "paths" => {
            for (path, _) in &files {
                println!("{}", path.display());
            }
        }
        _ => {
            if files.is_empty() {
                println!("{}", "  No thought documents found.".dimmed());
            } else {
                for (path, modified) in &files {
                    let relative = path.strip_prefix(thoughts_path).unwrap_or(path);
                    let age = modified.elapsed().unwrap_or_default();
                    let age_str = if age.as_secs() < 3600 {
                        format!("{}m ago", age.as_secs() / 60)
                    } else if age.as_secs() < 86400 {
                        format!("{}h ago", age.as_secs() / 3600)
                    } else {
                        format!("{}d ago", age.as_secs() / 86400)
                    };
                    println!("  {} {}", relative.display(), age_str.dimmed());
                }
            }
        }
    }

    Ok(())
}

/// Rebuild the searchable/ directory with hardlinks to all thought documents.
/// Uses path-encoded names: shared/research/topic.md -> shared-research-topic.md
fn rebuild_searchable_index(thoughts_path: &Path) -> anyhow::Result<usize> {
    let searchable_dir = thoughts_path.join("searchable");

    // Clear and recreate
    if searchable_dir.exists() {
        std::fs::remove_dir_all(&searchable_dir)?;
    }
    std::fs::create_dir_all(&searchable_dir)?;

    let mut link_count = 0;

    // Walk all directories except searchable/
    for entry in WalkDir::new(thoughts_path).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        // Skip searchable/, .git/, and hidden files
        name != "searchable" && name != ".git" && !name.starts_with('.')
    }) {
        let entry = entry?;
        let path = entry.path();

        // Only process markdown files
        if path.extension().map(|e| e == "md").unwrap_or(false) && path.is_file() {
            let relative = path.strip_prefix(thoughts_path)?;

            // Create path-encoded link name: shared/research/topic.md -> shared-research-topic.md
            let link_name = relative.to_string_lossy().replace('/', "-");
            let link_path = searchable_dir.join(&link_name);

            // Create hardlink (fails silently if source is symlink on some systems)
            match std::fs::hard_link(path, &link_path) {
                Ok(()) => link_count += 1,
                Err(_e) => {
                    // Try copy as fallback (for symlinked files)
                    if std::fs::copy(path, &link_path).is_ok() {
                        link_count += 1;
                    }
                }
            }
        }
    }

    Ok(link_count)
}

fn thoughts_sync(
    message: Option<String>,
    push: bool,
    pull: bool,
    no_commit: bool,
    direction: String,
) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");
    if !thoughts_path.exists() {
        println!(
            "{}",
            "Thoughts not initialized. Run 'snps thoughts init' first.".red()
        );
        return Ok(());
    }

    println!("{}", "Syncing thoughts...".bright_blue());

    // Resolve symlink to get actual thoughts directory (central repo)
    let central_path = std::fs::canonicalize(thoughts_path)?;

    // Determine sync direction
    let sync_from_central = direction == "both" || direction == "from-central";
    let sync_to_central = direction == "both" || direction == "to-central";

    if sync_from_central && pull {
        println!("  Pulling from central remote...");
        let output = std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&central_path)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                println!("{}", "  ✓ Pulled successfully".green());
            } else {
                println!("{}", "  ⚠ Pull failed or no remote configured".yellow());
            }
        }
    }

    // Rebuild searchable index (hardlinks)
    match rebuild_searchable_index(thoughts_path) {
        Ok(count) => {
            println!(
                "{}",
                format!("  ✓ Rebuilt searchable index ({} files)", count).green()
            );
        }
        Err(e) => {
            println!(
                "{}",
                format!("  ⚠ Failed to rebuild searchable index: {}", e).yellow()
            );
        }
    }

    if sync_to_central && !no_commit {
        let msg = message.unwrap_or_else(|| {
            format!(
                "Sync: {} from {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M"),
                hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "unknown".to_string())
            )
        });

        // Git add and commit in central repo
        let _ = std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(&central_path)
            .output();

        let output = std::process::Command::new("git")
            .args(["commit", "-m", &msg])
            .current_dir(&central_path)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                println!("{}", format!("  ✓ Committed: {}", msg).green());
            } else {
                println!("{}", "  • No changes to commit".dimmed());
            }
        }

        if push {
            println!("  Pushing to central remote...");
            let output = std::process::Command::new("git")
                .args(["push"])
                .current_dir(&central_path)
                .output();

            if let Ok(out) = output {
                if out.status.success() {
                    println!("{}", "  ✓ Pushed successfully".green());
                } else {
                    println!("{}", "  ✗ Push failed (no remote configured?)".yellow());
                }
            }
        }
    }

    println!();
    println!("{}", "✅ Sync complete".bright_green());

    Ok(())
}

fn thoughts_open(path: Option<String>, editor: bool, scope: Option<String>) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");
    if !thoughts_path.exists() {
        println!(
            "{}",
            "Thoughts not initialized. Run 'snps thoughts init' first.".red()
        );
        return Ok(());
    }

    let target = if let Some(p) = path {
        thoughts_path.join(p)
    } else if let Some(s) = scope {
        match s.as_str() {
            "personal" => thoughts_path.join(get_username()),
            "global" => thoughts_path.join("global"),
            _ => thoughts_path.join("shared"),
        }
    } else {
        thoughts_path.to_path_buf()
    };

    if editor {
        let editor_cmd = std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string());
        println!("Opening {} in {}...", target.display(), editor_cmd);
        std::process::Command::new(&editor_cmd)
            .arg(&target)
            .spawn()?;
    } else {
        // Open in file manager
        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(&target).spawn()?;

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(&target)
            .spawn()?;

        #[cfg(target_os = "windows")]
        std::process::Command::new("explorer")
            .arg(&target)
            .spawn()?;

        println!("Opening {}...", target.display());
    }

    Ok(())
}

fn thoughts_status(verbose: bool) -> anyhow::Result<()> {
    let thoughts_path = Path::new("thoughts");

    println!("{}", "Thoughts Status".bright_blue().bold());
    println!();

    // Check if initialized
    if !thoughts_path.exists() {
        println!("{}", "Status: Not initialized".red());
        println!();
        println!("Run 'snps thoughts init' to set up thoughts for this project.");
        return Ok(());
    }

    println!("{}", "Status: Initialized ✓".green());
    println!();

    // Show paths
    println!("{}", "Paths:".bright_white());
    println!("  Project symlink: {}", thoughts_path.display());

    if thoughts_path.is_symlink() {
        match std::fs::read_link(thoughts_path) {
            Ok(target) => {
                println!("  Central location: {}", target.display());

                // Check if central path exists
                let real_path = std::fs::canonicalize(thoughts_path);
                if let Ok(real) = real_path {
                    println!("  Resolved path: {}", real.display());

                    // Check git status
                    let git_dir = real.join(".git");
                    if git_dir.exists() || real.join("../.git").exists() {
                        println!("{}", "  Git repo: ✓".green());

                        // Get current branch
                        let output = std::process::Command::new("git")
                            .args(["branch", "--show-current"])
                            .current_dir(&real)
                            .output();
                        if let Ok(out) = output {
                            if out.status.success() {
                                let branch =
                                    String::from_utf8_lossy(&out.stdout).trim().to_string();
                                if !branch.is_empty() {
                                    println!("  Git branch: {}", branch);
                                }
                            }
                        }

                        // Get remote
                        let output = std::process::Command::new("git")
                            .args(["remote", "-v"])
                            .current_dir(&real)
                            .output();
                        if let Ok(out) = output {
                            if out.status.success() {
                                let remotes = String::from_utf8_lossy(&out.stdout);
                                if !remotes.trim().is_empty() {
                                    println!("{}", "  Remotes:".dimmed());
                                    for line in remotes.lines().take(2) {
                                        println!("    {}", line.dimmed());
                                    }
                                } else {
                                    println!("{}", "  No remotes configured".yellow());
                                }
                            }
                        }

                        // Check for uncommitted changes
                        let output = std::process::Command::new("git")
                            .args(["status", "--porcelain"])
                            .current_dir(&real)
                            .output();
                        if let Ok(out) = output {
                            if out.status.success() {
                                let changes = String::from_utf8_lossy(&out.stdout);
                                let change_count =
                                    changes.lines().filter(|l| !l.is_empty()).count();
                                if change_count > 0 {
                                    println!(
                                        "{}",
                                        format!("  Uncommitted changes: {}", change_count).yellow()
                                    );
                                } else {
                                    println!("{}", "  Working tree: clean ✓".green());
                                }
                            }
                        }
                    } else {
                        println!("{}", "  Git repo: Not initialized".yellow());
                        println!();
                        println!("  Consider initializing git in your thoughts directory:");
                        println!("    cd {} && git init", real.display());
                    }
                }
            }
            Err(e) => println!("{}", format!("  Error reading symlink: {}", e).red()),
        }
    } else {
        println!(
            "{}",
            "  Note: thoughts/ is not a symlink (local mode)".dimmed()
        );
    }

    println!();

    // Directory structure
    println!("{}", "Directory Structure:".bright_white());
    let scopes = ["shared", "global"];
    for scope in &scopes {
        let scope_path = thoughts_path.join(scope);
        if scope_path.exists() {
            println!("{}", format!("  {}/ ✓", scope).green());
            if verbose {
                let subdirs = ["research", "plans", "tickets", "prs"];
                for subdir in &subdirs {
                    let sub_path = scope_path.join(subdir);
                    if sub_path.exists() {
                        // Count files
                        let count = std::fs::read_dir(&sub_path)
                            .map(|d| d.filter(|e| e.is_ok()).count())
                            .unwrap_or(0);
                        println!("    {}/ ({} files)", subdir, count);
                    }
                }
            }
        }
    }

    // Personal directory
    let username = get_username();
    let personal_path = thoughts_path.join(&username);
    if personal_path.exists() {
        println!("{}", format!("  {}/ (personal) ✓", username).green());
    }

    // Searchable index
    let searchable_path = thoughts_path.join("searchable");
    if searchable_path.exists() {
        let count = std::fs::read_dir(&searchable_path)
            .map(|d| d.filter(|e| e.is_ok()).count())
            .unwrap_or(0);
        println!(
            "{}",
            format!("  searchable/ ({} hardlinks) ✓", count).green()
        );
    } else {
        println!(
            "{}",
            "  searchable/ (not built - run 'snps thoughts sync')".yellow()
        );
    }

    println!();

    // Git hooks status
    println!("{}", "Git Hooks:".bright_white());
    let pre_commit = Path::new(".git/hooks/pre-commit");
    if pre_commit.exists() {
        let content = std::fs::read_to_string(pre_commit).unwrap_or_default();
        if content.contains("PMSynapse") {
            println!("{}", "  pre-commit: Installed ✓".green());
        } else {
            println!("{}", "  pre-commit: Exists (not PMSynapse)".yellow());
        }
    } else {
        println!("{}", "  pre-commit: Not installed".dimmed());
        println!("    Run: snps thoughts hooks install");
    }

    println!();

    Ok(())
}

fn thoughts_profile(action: ProfileCommands) -> anyhow::Result<()> {
    let thoughts_root = get_thoughts_dir();

    match action {
        ProfileCommands::List => {
            println!("{}", "Thoughts Profiles".bright_blue());
            println!();

            let profiles_dir = thoughts_root.join("profiles");
            if profiles_dir.exists() {
                for entry in std::fs::read_dir(&profiles_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        println!("  • {}", name);
                    }
                }
            } else {
                println!(
                    "{}",
                    "  No profiles found. Create one with 'snps thoughts profile create <name>'"
                        .dimmed()
                );
            }
        }

        ProfileCommands::Create { name, repo, remote } => {
            println!("{}", format!("Creating profile: {}", name).bright_green());

            let profile_dir = thoughts_root.join("profiles").join(&name);
            std::fs::create_dir_all(&profile_dir)?;

            let config = format!(
                "name: {}\nrepo: {}\nremote: {}\n",
                name,
                repo.as_deref().unwrap_or(&profile_dir.to_string_lossy()),
                remote.as_deref().unwrap_or("null")
            );
            std::fs::write(profile_dir.join("profile.yaml"), config)?;

            println!("{}", "  ✓ Profile created".green());
        }

        ProfileCommands::Switch { name } => {
            println!(
                "{}",
                format!("Switching to profile: {}", name).bright_blue()
            );

            let profile_dir = thoughts_root.join("profiles").join(&name);
            if !profile_dir.exists() {
                println!("{}", format!("Profile '{}' does not exist.", name).red());
                return Ok(());
            }

            // Update project config
            let config_path = Path::new(".pmsynapse/config.yaml");
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                // Simple replacement - in production use proper YAML parsing
                let updated = if content.contains("profile:") {
                    content
                        .lines()
                        .map(|l| {
                            if l.trim_start().starts_with("profile:") {
                                format!("  profile: {}", name)
                            } else {
                                l.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    format!("{}\nthoughts:\n  profile: {}", content, name)
                };
                std::fs::write(config_path, updated)?;
            }

            println!("{}", "  ✓ Profile switched".green());
        }

        ProfileCommands::Show => {
            println!("{}", "Current Profile".bright_blue());

            // Try to read from project config
            let config_path = Path::new(".pmsynapse/config.yaml");
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                for line in content.lines() {
                    if line.trim_start().starts_with("profile:") {
                        let profile = line.split(':').nth(1).unwrap_or("personal").trim();
                        println!("  Profile: {}", profile.bright_cyan());
                        return Ok(());
                    }
                }
            }
            println!("  Profile: {} (default)", "personal".bright_cyan());
        }
    }

    Ok(())
}

fn thoughts_hooks(action: HooksCommands) -> anyhow::Result<()> {
    match action {
        HooksCommands::Install {
            no_pre_commit,
            no_post_commit,
            auto_sync,
            force,
        } => install_thoughts_hooks(no_pre_commit, no_post_commit, auto_sync, force),
        HooksCommands::Uninstall => uninstall_thoughts_hooks(),
        HooksCommands::Status => check_hooks_status(),
    }
}

fn install_thoughts_hooks(
    no_pre_commit: bool,
    no_post_commit: bool,
    auto_sync: bool,
    force: bool,
) -> anyhow::Result<()> {
    let hooks_dir = Path::new(".git/hooks");
    if !hooks_dir.exists() {
        println!(
            "{}",
            "Not a git repository. Skipping hook installation.".yellow()
        );
        return Ok(());
    }

    if !no_pre_commit {
        let pre_commit = hooks_dir.join("pre-commit");

        // Check for existing hook
        if pre_commit.exists() && !force {
            let existing = std::fs::read_to_string(&pre_commit)?;
            if !existing.contains("PMSynapse") {
                println!(
                    "{}",
                    "  Pre-commit hook exists. Use --force to overwrite.".yellow()
                );
                return Ok(());
            }
        }

        let hook_content = r#"#!/bin/bash
# PMSynapse: Prevent thoughts/ from being committed to code repo

if git diff --cached --name-only | grep -q "^thoughts/"; then
    echo "❌ ERROR: thoughts/ directory should not be committed"
    echo ""
    echo "The thoughts/ directory is symlinked from your thoughts repository."
    echo "These files should be committed there instead:"
    echo ""
    echo "  snps thoughts sync -m 'Your message'"
    echo ""
    echo "If you really need to commit these files, use:"
    echo "  git commit --no-verify"
    echo ""
    exit 1
fi

# Continue with any existing pre-commit hooks
if [ -f .git/hooks/pre-commit.backup ]; then
    . .git/hooks/pre-commit.backup
fi
"#;

        // Backup existing hook
        if pre_commit.exists() {
            let existing = std::fs::read_to_string(&pre_commit)?;
            if !existing.contains("PMSynapse") {
                std::fs::copy(&pre_commit, hooks_dir.join("pre-commit.backup"))?;
            }
        }

        std::fs::write(&pre_commit, hook_content)?;

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&pre_commit, std::fs::Permissions::from_mode(0o755))?;
        }

        println!("{}", "  ✓ Installed pre-commit hook".green());
    }

    if !no_post_commit && auto_sync {
        let post_commit = hooks_dir.join("post-commit");

        let hook_content = r#"#!/bin/bash
# PMSynapse: Auto-sync thoughts after commit

# Only sync if thoughts directory exists
if [ -d "thoughts" ]; then
    snps thoughts sync --no-commit 2>/dev/null || true
fi

# Continue with any existing post-commit hooks
if [ -f .git/hooks/post-commit.backup ]; then
    . .git/hooks/post-commit.backup
fi
"#;

        // Backup existing hook
        if post_commit.exists() {
            let existing = std::fs::read_to_string(&post_commit)?;
            if !existing.contains("PMSynapse") {
                std::fs::copy(&post_commit, hooks_dir.join("post-commit.backup"))?;
            }
        }

        std::fs::write(&post_commit, hook_content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&post_commit, std::fs::Permissions::from_mode(0o755))?;
        }

        println!(
            "{}",
            "  ✓ Installed post-commit hook (auto-sync enabled)".green()
        );
    }

    Ok(())
}

fn uninstall_thoughts_hooks() -> anyhow::Result<()> {
    let hooks_dir = Path::new(".git/hooks");
    let pre_commit = hooks_dir.join("pre-commit");
    let backup = hooks_dir.join("pre-commit.backup");

    if backup.exists() {
        std::fs::rename(&backup, &pre_commit)?;
        println!("{}", "  ✓ Restored original pre-commit hook".green());
    } else if pre_commit.exists() {
        std::fs::remove_file(&pre_commit)?;
        println!("{}", "  ✓ Removed pre-commit hook".green());
    } else {
        println!("{}", "  No hooks to uninstall".dimmed());
    }

    Ok(())
}

fn check_hooks_status() -> anyhow::Result<()> {
    println!("{}", "Git Hooks Status".bright_blue());

    let pre_commit = Path::new(".git/hooks/pre-commit");
    if pre_commit.exists() {
        let content = std::fs::read_to_string(pre_commit)?;
        if content.contains("PMSynapse") {
            println!("{}", "  ✓ Pre-commit hook installed".green());
        } else {
            println!(
                "{}",
                "  ✗ Pre-commit hook exists but is not PMSynapse".yellow()
            );
        }
    } else {
        println!("{}", "  ✗ Pre-commit hook not installed".red());
    }

    Ok(())
}

// =============================================================================
// Template Generators
// =============================================================================

fn generate_research_template(title: &str, username: &str, date: &str) -> String {
    format!(
        r#"# Research: {}

## Date
{}

## Question
What are we trying to understand?

## Background
Why is this research needed?

## Key Findings

### Finding 1
Description and evidence

### Finding 2
Description and evidence

## Relevant Code
- `src/path/to/file.rs` - Description

## Recommendations
1. Recommendation with rationale

## Open Questions
- [ ] Unanswered question 1

## References
- [Link title](url)

## Status
🟡 In Progress

---
*Created by {} on {}*
"#,
        title, date, username, date
    )
}

fn generate_plan_template(title: &str, username: &str, date: &str) -> String {
    format!(
        r#"# Plan: {}

## Goal
What we're building and why

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Prerequisites
- [ ] Required before starting

## Implementation Steps

### Phase 1: Setup
- [ ] **Step 1.1**: Description
  - Files: `src/file.rs`
  - Changes: What to modify

### Phase 2: Implementation
- [ ] **Step 2.1**: Description

## Testing Strategy
- Unit tests: What to test
- Integration tests: What to test

## Rollback Plan
How to undo if something goes wrong

## Status
🟡 Planning

## Progress Log
- {}: Started planning

---
*Created by {} on {}*
"#,
        title, date, username, date
    )
}

fn generate_ticket_template(title: &str, username: &str, date: &str) -> String {
    format!(
        r#"# Ticket: {}

## Original Requirements
> Paste original ticket description here

## My Understanding
What I think this ticket is asking for

## Acceptance Criteria
- [ ] Criterion from ticket

## Questions for Stakeholder
- [ ] Clarification needed on X

## Technical Approach
High-level approach to implementation

## Technical Notes
Implementation details discovered during work

## Related Resources
- Research: [[research/related-topic.md]]
- Code: `src/relevant/file.rs`

## Progress Log
- {}: Started investigation

---
*Created by {} on {}*
"#,
        title, date, username, date
    )
}

fn generate_pr_template(title: &str, username: &str, date: &str) -> String {
    format!(
        r#"# PR: {}

## Summary
Brief description of what this PR does

## Changes
- Change 1
- Change 2

## Testing Done
- [ ] Unit tests pass
- [ ] Manual testing completed

## Screenshots
(if applicable)

## Related
- Ticket: [PROJ-XXX](url)
- Research: [[research/topic.md]]

---
*Created by {} on {}*
"#,
        title, username, date
    )
}

fn generate_scratch_template(title: &str, date: &str) -> String {
    format!(
        r#"# {}

*{}*

## Notes

"#,
        title, date
    )
}

fn generate_journal_template(date: &str) -> String {
    format!(
        r#"# Journal: {}

## Today's Goals
- [ ] Goal 1
- [ ] Goal 2

## Progress
-

## Blockers
-

## Tomorrow
-

## Notes
"#,
        date
    )
}

// =============================================================================
// Claude Commands
// =============================================================================

fn cmd_claude(action: ClaudeCommands) -> anyhow::Result<()> {
    match action {
        ClaudeCommands::Parse {
            file,
            format,
            output,
            pretty,
            save,
        } => cmd_claude_parse(file, format, output, pretty, save),

        ClaudeCommands::List {
            dir,
            recent,
            project,
            format,
            all,
        } => cmd_claude_list(dir, recent, project, format, all),

        ClaudeCommands::Analyze {
            dir,
            tree,
            output,
            format,
        } => cmd_claude_analyze(dir, tree, output, format),

        ClaudeCommands::Tree { file, depth, tools } => cmd_claude_tree(file, depth, tools),

        ClaudeCommands::Import {
            claude_dir,
            format,
            main_only,
            project,
        } => cmd_claude_import(claude_dir, format, main_only, project),

        ClaudeCommands::Convert {
            input,
            format,
            output,
            pretty,
        } => cmd_claude_convert(input, format, output, pretty),
    }
}

fn get_claude_sessions_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".claude").join("projects")
}

fn expand_path(path: &str) -> PathBuf {
    let expanded = shellexpand::tilde(path);
    PathBuf::from(expanded.as_ref())
}

/// Resolve a session ID (full or partial) to a file path
/// If the input is already a path, returns it as-is
/// Otherwise searches for matching session files
fn resolve_session_path(id_or_path: &str) -> anyhow::Result<PathBuf> {
    let path = expand_path(id_or_path);

    // If it's already a valid path, use it
    if path.exists() {
        return Ok(path);
    }

    // Otherwise, treat it as a session ID and search for it
    let sessions_dir = get_claude_sessions_dir();

    // Try current project first
    if let Some(project_dir) = get_claude_project_dir() {
        if let Some(found) = search_session_in_dir(&project_dir, id_or_path)? {
            return Ok(found);
        }
    }

    // Search all projects
    for entry in std::fs::read_dir(&sessions_dir)? {
        let entry = entry?;
        let project_path = entry.path();

        if project_path.is_dir() {
            if let Some(found) = search_session_in_dir(&project_path, id_or_path)? {
                return Ok(found);
            }
        }
    }

    Err(anyhow::anyhow!(
        "Session not found: {}. Searched in {} projects.",
        id_or_path,
        sessions_dir.display()
    ))
}

fn search_session_in_dir(dir: &Path, id: &str) -> anyhow::Result<Option<PathBuf>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            // Match full session ID
            if filename.starts_with(id) && filename.ends_with(".jsonl") {
                return Ok(Some(path));
            }

            // Match partial session ID (e.g., "0c721d51" matches "0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl")
            if filename.contains(id) && filename.ends_with(".jsonl") {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}

fn cmd_claude_parse(
    file: String,
    format: ClaudeExportFormat,
    output: Option<String>,
    pretty: bool,
    save: bool,
) -> anyhow::Result<()> {
    let file_path = resolve_session_path(&file)?;

    println!(
        "{}",
        format!("Parsing session: {}", file_path.display()).bright_blue()
    );

    let project_dir = file_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let parser = SessionParser::new(project_dir.clone());
    let session = parser
        .parse_file(&file_path)
        .map_err(|e| anyhow::anyhow!("Failed to parse session: {}", e))?;

    let analyzer = SessionAnalyzer::new(project_dir);
    let stats = analyzer.analyze_session(&session);

    let exporter = SessionExporter::new();

    let result = match format {
        ClaudeExportFormat::Json => exporter
            .export_json_string(&session, &stats, pretty)
            .map_err(|e| anyhow::anyhow!("Export failed: {}", e))?,
        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
            exporter.export_markdown_string(&session, &stats)
        }
        ClaudeExportFormat::Html => exporter.export_html_string(&session, &stats, None),
    };

    // Save to thoughts directory if requested
    if save {
        let thoughts_dir = PathBuf::from("thoughts/shared/sessions");
        std::fs::create_dir_all(&thoughts_dir)?;

        let extension = match format {
            ClaudeExportFormat::Json => "json",
            ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => "md",
            ClaudeExportFormat::Html => "html",
        };

        // Create a filename from session ID (first 8 chars)
        let short_id = if session.session_id.len() >= 8 {
            &session.session_id[..8]
        } else {
            &session.session_id
        };
        let session_filename = format!("session-{}.{}", short_id, extension);
        let save_path = thoughts_dir.join(session_filename);

        std::fs::write(&save_path, &result)?;
        println!(
            "{}",
            format!("  ✓ Saved to: {}", save_path.display()).green()
        );

        // Try to run thoughts sync
        println!("{}", "  Syncing thoughts...".dimmed());
        let sync_result = std::process::Command::new("snps")
            .args(["thoughts", "sync", "--no-commit"])
            .output();

        match sync_result {
            Ok(output) if output.status.success() => {
                println!("{}", "  ✓ Thoughts index updated".green());
            }
            _ => {
                println!(
                    "{}",
                    "  ⚠ Could not auto-sync thoughts (run 'snps thoughts sync' manually)".yellow()
                );
            }
        }
    }

    if let Some(out_path) = output {
        let out_file = expand_path(&out_path);
        std::fs::write(&out_file, &result)?;
        println!(
            "{}",
            format!("✓ Exported to: {}", out_file.display()).green()
        );
    } else if !save {
        // Only print to stdout if not saving (to avoid double output)
        println!();
        println!("{}", result);
    }

    Ok(())
}

/// Session metadata extracted from JSONL file
#[derive(Debug)]
struct SessionInfo {
    path: PathBuf,
    session_id: String,
    title: Option<String>,
    message_count: usize,
    git_branch: Option<String>,
    modified: std::time::SystemTime,
    is_agent: bool,
}

/// Convert a filesystem path to Claude's project directory format
/// e.g., /Users/igor/Dev/Helixoid/pmsynapse -> -Users-igor-Dev-Helixoid-pmsynapse
fn path_to_claude_project_dir(path: &Path) -> String {
    path.to_string_lossy().replace('/', "-")
}

/// Get the Claude sessions directory for the current project
fn get_claude_project_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let project_subdir = path_to_claude_project_dir(&cwd);
    let sessions_dir = get_claude_sessions_dir().join(&project_subdir);

    if sessions_dir.exists() {
        Some(sessions_dir)
    } else {
        None
    }
}

/// Check if a filename looks like a GUID (main session) vs agent file
fn is_main_session_file(filename: &str) -> bool {
    // Main sessions are UUIDs like: 0c721d51-3a4f-4db0-b5c9-e364c9c55de4.jsonl
    // Agent sessions start with: agent-
    !filename.starts_with("agent-") && filename.ends_with(".jsonl") && filename.len() > 30
    // UUIDs are 36 chars + .jsonl
}

/// Extract session metadata from a JSONL file
fn extract_session_info(path: &Path) -> Option<SessionInfo> {
    use std::io::{BufRead, BufReader};

    let filename = path.file_name()?.to_string_lossy().to_string();
    let is_agent = filename.starts_with("agent-");

    let file = std::fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut session_id = None;
    let mut title = None;
    let mut git_branch = None;
    let mut message_count = 0;
    let mut first_user_message_found = false;

    for line in reader.lines().take(50) {
        // Read up to 50 lines to find metadata
        let line = line.ok()?;
        if line.trim().is_empty() {
            continue;
        }

        let record: serde_json::Value = serde_json::from_str(&line).ok()?;

        // Extract session ID
        if session_id.is_none() {
            session_id = record
                .get("sessionId")
                .and_then(|v| v.as_str())
                .map(String::from);
        }

        // Extract git branch
        if git_branch.is_none() {
            git_branch = record
                .get("gitBranch")
                .and_then(|v| v.as_str())
                .map(String::from);
        }

        let msg_type = record.get("type").and_then(|v| v.as_str());

        // Extract title from summary record (preferred method)
        if title.is_none() && msg_type == Some("summary") {
            title = record
                .get("summary")
                .and_then(|v| v.as_str())
                .map(String::from);
        }

        // Count messages (exclude tool results and meta messages)
        if msg_type == Some("user") || msg_type == Some("assistant") {
            // Check if it's a tool result (user message with tool_result content)
            let is_tool_result = msg_type == Some("user")
                && record
                    .get("message")
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter().any(|item| {
                            item.get("type").and_then(|t| t.as_str()) == Some("tool_result")
                        })
                    })
                    .unwrap_or(false);

            // Skip meta messages and tool results
            let is_meta = record
                .get("isMeta")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !is_tool_result && !is_meta {
                message_count += 1;
            }
        }

        // Fallback: Extract title from first non-meta user message if no summary found
        if title.is_none()
            && !first_user_message_found
            && msg_type == Some("user")
            && record.get("isMeta").and_then(|v| v.as_bool()) != Some(true)
        {
            if let Some(content) = record
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_array())
            {
                for item in content {
                    if item.get("type").and_then(|t| t.as_str()) == Some("text") {
                        if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                            // Skip command messages and system messages
                            if !text.starts_with('<') && !text.contains("<command") {
                                // Take first line, truncate to reasonable length
                                let first_line = text.lines().next().unwrap_or("");
                                let truncated: String = first_line.chars().take(60).collect();
                                title = Some(if first_line.len() > 60 {
                                    format!("{}...", truncated)
                                } else {
                                    truncated
                                });
                                first_user_message_found = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // If we still don't have a title and it's a main session, try to count all messages
    if title.is_none() && !is_agent {
        // Count remaining messages
        let file = std::fs::File::open(path).ok()?;
        let reader = BufReader::new(file);
        message_count = reader.lines().filter(|l| l.is_ok()).count();
    }

    let modified = path.metadata().ok()?.modified().ok()?;

    Some(SessionInfo {
        path: path.to_path_buf(),
        session_id: session_id.unwrap_or_else(|| filename.replace(".jsonl", "")),
        title,
        message_count,
        git_branch,
        modified,
        is_agent,
    })
}

fn cmd_claude_list(
    dir: Option<String>,
    recent: usize,
    project: Option<String>,
    format: String,
    show_all: bool,
) -> anyhow::Result<()> {
    // Determine search directory
    let search_dir = if let Some(d) = dir {
        expand_path(&d)
    } else if let Some(p) = &project {
        // Use explicit project path
        let project_subdir = if p.starts_with('-') {
            p.clone()
        } else {
            path_to_claude_project_dir(Path::new(p))
        };
        get_claude_sessions_dir().join(project_subdir)
    } else {
        // Auto-detect from current directory
        get_claude_project_dir().unwrap_or_else(get_claude_sessions_dir)
    };

    if !search_dir.exists() {
        println!(
            "{}",
            format!(
                "Claude sessions directory not found: {}",
                search_dir.display()
            )
            .yellow()
        );
        println!();
        println!(
            "{}",
            "Hint: Run this command from a project directory, or use --dir to specify a path."
                .dimmed()
        );
        return Ok(());
    }

    // Collect session files
    let mut sessions: Vec<SessionInfo> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                let filename = path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Filter agents unless --all is specified
                if !show_all && !is_main_session_file(&filename) {
                    continue;
                }

                if let Some(info) = extract_session_info(&path) {
                    sessions.push(info);
                }
            }
        }
    }

    // Sort by modification time (newest first)
    sessions.sort_by(|a, b| b.modified.cmp(&a.modified));

    // Apply recent limit
    let total_found = sessions.len();
    sessions.truncate(recent);

    if sessions.is_empty() {
        println!("{}", "No Claude Code sessions found.".dimmed());
        return Ok(());
    }

    match format.as_str() {
        "json" => {
            let json_sessions: Vec<_> = sessions
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "session_id": s.session_id,
                        "path": s.path.to_string_lossy(),
                        "title": s.title,
                        "message_count": s.message_count,
                        "git_branch": s.git_branch,
                        "is_agent": s.is_agent
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_sessions)?);
        }
        "paths" => {
            for session in &sessions {
                println!("{}", session.path.display());
            }
        }
        _ => {
            // Table format (similar to Claude's /resume)
            let showing = sessions.len().min(recent);
            if total_found > showing {
                println!(
                    "{}",
                    format!("Showing {} of {} sessions", showing, total_found).dimmed()
                );
            }
            println!();

            for session in &sessions {
                let age = session.modified.elapsed().unwrap_or_default();
                let age_str = if age.as_secs() < 60 {
                    "just now".to_string()
                } else if age.as_secs() < 3600 {
                    format!("{} minutes ago", age.as_secs() / 60)
                } else if age.as_secs() < 86400 {
                    format!("{} hours ago", age.as_secs() / 3600)
                } else {
                    format!("{} days ago", age.as_secs() / 86400)
                };

                // Title or fallback
                let title = session.title.as_deref().unwrap_or(if session.is_agent {
                    "(Agent session)"
                } else {
                    "(No title)"
                });

                // Branch info
                let branch_str = session
                    .git_branch
                    .as_ref()
                    .map(|b| format!(" · {}", b.bright_cyan()))
                    .unwrap_or_else(|| " · -".dimmed().to_string());

                // Agent indicator
                let agent_marker = if session.is_agent {
                    format!("{} ", "⚡".yellow())
                } else {
                    format!("{} ", "❯".green())
                };

                // Extract first 8 chars of session ID for display
                let session_id_short = session.session_id.chars().take(8).collect::<String>();

                println!("{}{}", agent_marker, title.bright_white());
                println!(
                    "  {} · {} · {} messages{}",
                    session_id_short.bright_black(),
                    age_str.dimmed(),
                    session.message_count,
                    branch_str
                );
            }
        }
    }

    Ok(())
}

fn cmd_claude_analyze(
    dir: String,
    tree: bool,
    output: Option<String>,
    format: ClaudeExportFormat,
) -> anyhow::Result<()> {
    let dir_path = expand_path(&dir);

    if !dir_path.exists() {
        println!(
            "{}",
            format!("Directory not found: {}", dir_path.display()).red()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Analyzing sessions in: {}", dir_path.display()).bright_blue()
    );
    println!();

    let analyzer = SessionAnalyzer::new(dir_path.to_string_lossy().to_string());
    let hierarchy = analyzer
        .analyze_directory(&dir_path)
        .map_err(|e| anyhow::anyhow!("Analysis failed: {}", e))?;

    // Print summary
    println!("{}", "Session Hierarchy Summary".bright_blue());
    println!();
    println!("  Main sessions: {}", hierarchy.sessions.len());
    println!(
        "  Total sessions: {}",
        hierarchy.sessions.len() + hierarchy.agents.len()
    );

    // Count agents
    let agent_count = hierarchy.agents.len();
    println!("  Agent sessions: {}", agent_count);
    println!();

    // Show tree if requested
    if tree {
        println!("{}", "Session Tree:".bright_blue());
        println!();

        for session in &hierarchy.sessions {
            let exporter = SessionExporter::new();
            let stats = analyzer.analyze_session(session);
            let title = exporter.build_thread_data(session, &stats).title;

            println!("  {} {}", "●".green(), title);
            println!("    ID: {}", session.session_id.dimmed());
            println!(
                "    Messages: {}, Tools: {}",
                stats.total_messages, stats.tool_calls
            );

            // Show child agents
            for child_id in &session.child_agents {
                // Find agent by ID
                if let Some(child) = hierarchy.agents.iter().find(|a| {
                    a.agent_id
                        .as_ref()
                        .map(|id| id == child_id)
                        .unwrap_or(false)
                }) {
                    let child_stats = analyzer.analyze_session(child);
                    println!("    {} Agent: {}", "├─".dimmed(), child_id.bright_cyan());
                    println!(
                        "    {}   Messages: {}, Tools: {}",
                        "│".dimmed(),
                        child_stats.total_messages,
                        child_stats.tool_calls
                    );
                }
            }
            println!();
        }
    }

    // Export if requested
    if let Some(out_path) = output {
        let out_file = expand_path(&out_path);

        let result = match format {
            ClaudeExportFormat::Json | ClaudeExportFormat::Html => {
                // HTML export for analyze command falls back to JSON
                serde_json::to_string_pretty(&hierarchy)?
            }
            ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
                let mut md = String::new();
                md.push_str("# Claude Code Session Analysis\n\n");
                md.push_str("## Summary\n\n");
                md.push_str(&format!("- Main sessions: {}\n", hierarchy.sessions.len()));
                md.push_str(&format!(
                    "- Total sessions: {}\n",
                    hierarchy.sessions.len() + hierarchy.agents.len()
                ));
                md.push_str(&format!("- Agent sessions: {}\n\n", agent_count));

                md.push_str("## Sessions\n\n");
                for session in &hierarchy.sessions {
                    let stats = analyzer.analyze_session(session);
                    md.push_str(&format!("### {}\n\n", session.session_id));
                    md.push_str(&format!("- Is agent: {}\n", session.is_agent));
                    md.push_str(&format!("- Messages: {}\n", stats.total_messages));
                    md.push_str(&format!("- Tool calls: {}\n", stats.tool_calls));
                    if !session.child_agents.is_empty() {
                        md.push_str(&format!(
                            "- Child agents: {}\n",
                            session.child_agents.join(", ")
                        ));
                    }
                    md.push('\n');
                }

                md.push_str("## Agents\n\n");
                for agent in &hierarchy.agents {
                    let stats = analyzer.analyze_session(agent);
                    let agent_id = agent.agent_id.as_deref().unwrap_or("unknown");
                    md.push_str(&format!("### {}\n\n", agent_id));
                    md.push_str(&format!(
                        "- Parent: {}\n",
                        agent.parent_session_id.as_deref().unwrap_or("unknown")
                    ));
                    md.push_str(&format!("- Messages: {}\n", stats.total_messages));
                    md.push_str(&format!("- Tool calls: {}\n\n", stats.tool_calls));
                }
                md
            }
        };

        std::fs::write(&out_file, result)?;
        println!(
            "{}",
            format!("✓ Analysis exported to: {}", out_file.display()).green()
        );
    }

    Ok(())
}

fn cmd_claude_tree(file: String, depth: usize, tools: bool) -> anyhow::Result<()> {
    let file_path = expand_path(&file);

    if !file_path.exists() {
        println!(
            "{}",
            format!("Session file not found: {}", file_path.display()).red()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Message tree for: {}", file_path.display()).bright_blue()
    );
    println!();

    let project_dir = file_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let parser = SessionParser::new(project_dir.clone());
    let session = parser
        .parse_file(&file_path)
        .map_err(|e| anyhow::anyhow!("Failed to parse session: {}", e))?;

    let analyzer = SessionAnalyzer::new(project_dir);
    let stats = analyzer.analyze_session(&session);

    // Build and display tree from messages directly
    println!("{}", "Message Flow:".bright_blue());
    println!();

    let mut current_depth = 0;
    for message in &session.messages {
        if current_depth >= depth {
            continue;
        }

        let indent = "  ".repeat(current_depth.min(depth));
        let role_str = message
            .role
            .as_ref()
            .map(|r| format!("{:?}", r))
            .unwrap_or_else(|| "System".to_string());
        let role_colored = match &message.role {
            Some(snps_core::claude::MessageRole::User) => role_str.bright_green(),
            Some(snps_core::claude::MessageRole::Assistant) => role_str.bright_blue(),
            Some(snps_core::claude::MessageRole::System) => role_str.bright_yellow(),
            None => role_str.dimmed(),
        };

        // Truncate content preview
        let preview = message
            .content
            .text
            .as_ref()
            .map(|s| {
                let first_line = s.lines().next().unwrap_or("");
                let truncated: String = first_line.chars().take(60).collect();
                if first_line.len() > 60 {
                    format!("{}...", truncated)
                } else {
                    truncated
                }
            })
            .unwrap_or_else(|| "(no content)".to_string());

        println!("{}[{}] {}", indent, role_colored, preview);

        if tools && !message.tool_uses.is_empty() {
            let tool_names: Vec<_> = message
                .tool_uses
                .iter()
                .map(|t| t.tool_name.as_str())
                .collect();
            println!("{}  Tools: {}", indent, tool_names.join(", ").dimmed());
        }

        current_depth += 1;
    }

    println!();
    println!(
        "{}",
        format!(
            "Total: {} messages, {} tool calls",
            stats.total_messages, stats.tool_calls
        )
        .dimmed()
    );

    Ok(())
}

fn cmd_claude_import(
    claude_dir: String,
    format: ClaudeExportFormat,
    main_only: bool,
    project_filter: Option<String>,
) -> anyhow::Result<()> {
    println!(
        "{}",
        "Importing Claude Code sessions to thoughts...".bright_blue()
    );
    println!();

    let dir_path = expand_path(&claude_dir);

    if !dir_path.exists() {
        println!(
            "{}",
            format!(
                "Claude projects directory not found: {}",
                dir_path.display()
            )
            .red()
        );
        return Ok(());
    }

    let thoughts_dir = PathBuf::from("thoughts/shared/sessions");
    std::fs::create_dir_all(&thoughts_dir)?;

    let exporter = SessionExporter::new();
    let mut imported_count = 0;
    let mut skipped_count = 0;

    println!("  Scanning: {}", dir_path.display());

    // Iterate over project directories
    for project_entry in std::fs::read_dir(&dir_path)? {
        let project_entry = project_entry?;
        let project_path = project_entry.path();

        if !project_path.is_dir() {
            continue;
        }

        let project_name = project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // Apply project filter
        if let Some(ref filter) = project_filter {
            if !project_name.contains(filter) {
                continue;
            }
        }

        // Analyze sessions in this project directory
        let analyzer = SessionAnalyzer::new(project_path.to_string_lossy().to_string());

        match analyzer.analyze_directory(&project_path) {
            Ok(hierarchy) => {
                // Process main sessions
                for session in &hierarchy.sessions {
                    let stats = analyzer.analyze_session(session);

                    let extension = match format {
                        ClaudeExportFormat::Json => "json",
                        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => "md",
                        ClaudeExportFormat::Html => "html",
                    };

                    let short_id = if session.session_id.len() >= 8 {
                        &session.session_id[..8]
                    } else {
                        &session.session_id
                    };

                    let filename = format!("session-{}.{}", short_id, extension);
                    let save_path = thoughts_dir.join(&filename);

                    // Skip if already exists
                    if save_path.exists() {
                        skipped_count += 1;
                        continue;
                    }

                    let content = match format {
                        ClaudeExportFormat::Json => exporter
                            .export_json_string(session, &stats, true)
                            .unwrap_or_default(),
                        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
                            exporter.export_markdown_string(session, &stats)
                        }
                        ClaudeExportFormat::Html => {
                            exporter.export_html_string(session, &stats, None)
                        }
                    };

                    std::fs::write(&save_path, content)?;
                    imported_count += 1;
                    println!("    {} {}", "✓".green(), filename);
                }

                // Process agents if not main_only
                if !main_only {
                    for agent in &hierarchy.agents {
                        let stats = analyzer.analyze_session(agent);

                        let agent_id = agent.agent_id.as_deref().unwrap_or("unknown");
                        let short_id = if agent_id.len() >= 8 {
                            &agent_id[..8]
                        } else {
                            agent_id
                        };

                        let extension = match format {
                            ClaudeExportFormat::Json => "json",
                            ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => "md",
                            ClaudeExportFormat::Html => "html",
                        };

                        let filename = format!("agent-{}.{}", short_id, extension);
                        let save_path = thoughts_dir.join(&filename);

                        // Skip if already exists
                        if save_path.exists() {
                            skipped_count += 1;
                            continue;
                        }

                        let content = match format {
                            ClaudeExportFormat::Json => exporter
                                .export_json_string(agent, &stats, true)
                                .unwrap_or_default(),
                            ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
                                exporter.export_markdown_string(agent, &stats)
                            }
                            ClaudeExportFormat::Html => {
                                exporter.export_html_string(agent, &stats, None)
                            }
                        };

                        std::fs::write(&save_path, content)?;
                        imported_count += 1;
                        println!("    {} {} (agent)", "✓".green(), filename);
                    }
                }
            }
            Err(e) => {
                println!(
                    "    {} Failed to analyze {}: {}",
                    "⚠".yellow(),
                    project_name,
                    e
                );
            }
        }
    }

    println!();
    println!(
        "{}",
        format!(
            "✓ Imported {} sessions ({} skipped)",
            imported_count, skipped_count
        )
        .green()
    );

    // Try to run thoughts sync
    if imported_count > 0 {
        println!();
        println!("{}", "Syncing thoughts...".dimmed());
        let sync_result = std::process::Command::new("snps")
            .args(["thoughts", "sync", "--no-commit"])
            .output();

        match sync_result {
            Ok(output) if output.status.success() => {
                println!("{}", "✓ Thoughts index updated".green());
            }
            _ => {
                println!(
                    "{}",
                    "⚠ Could not auto-sync thoughts (run 'snps thoughts sync' manually)".yellow()
                );
            }
        }
    }

    Ok(())
}

fn cmd_claude_convert(
    input: String,
    format: ClaudeExportFormat,
    output: Option<String>,
    pretty: bool,
) -> anyhow::Result<()> {
    use snps_core::claude::{SessionAnalyzer, SessionExporter};

    let input_path = expand_path(&input);

    if !input_path.exists() {
        println!(
            "{}",
            format!("Input file not found: {}", input_path.display()).red()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Converting: {}", input_path.display()).bright_blue()
    );

    // Read and deserialize the JSON export file
    let json_content = std::fs::read_to_string(&input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input file: {}", e))?;

    let thread_data: snps_core::claude::ThreadData = serde_json::from_str(&json_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON export: {}", e))?;

    // Convert ThreadData back to Session for export
    let session = thread_data_to_session(&thread_data);
    let analyzer = SessionAnalyzer::new(String::new());
    let stats = analyzer.analyze_session(&session);

    let exporter = SessionExporter::new();

    match format {
        ClaudeExportFormat::Json => {
            let json = if pretty {
                serde_json::to_string_pretty(&thread_data)?
            } else {
                serde_json::to_string(&thread_data)?
            };

            if let Some(output_path) = output {
                std::fs::write(&output_path, json)?;
                println!("{}", format!("✓ Exported to: {}", output_path).green());
            } else {
                println!("{}", json);
            }
        }
        ClaudeExportFormat::Markdown | ClaudeExportFormat::Md => {
            if let Some(output_path) = output {
                let out_path = PathBuf::from(output_path);
                exporter.export_markdown(&session, &stats, &out_path)?;
                println!(
                    "{}",
                    format!("✓ Exported to: {}", out_path.display()).green()
                );
            } else {
                println!(
                    "{}",
                    "Error: Markdown export requires --output option".red()
                );
                return Err(anyhow::anyhow!("Markdown export requires output file"));
            }
        }
        ClaudeExportFormat::Html => {
            if let Some(output_path) = output {
                let out_path = PathBuf::from(output_path);
                exporter.export_html(&session, &stats, &out_path, None)?;
                println!(
                    "{}",
                    format!("✓ Exported to: {}", out_path.display()).green()
                );
            } else {
                // Output HTML to stdout
                let html = exporter.export_html_string(&session, &stats, None);
                println!("{}", html);
            }
        }
    }

    Ok(())
}

/// Convert ThreadData back to Session for re-export
fn thread_data_to_session(
    thread_data: &snps_core::claude::ThreadData,
) -> snps_core::claude::Session {
    use snps_core::claude::{Message, MessageContent, Session, SessionMetadata};

    let messages: Vec<Message> = thread_data
        .messages
        .iter()
        .map(|tm| Message {
            uuid: tm.uuid.clone(),
            parent_uuid: tm.parent_uuid.clone(),
            is_sidechain: false,
            message_type: tm.message_type.clone(),
            role: tm.role.clone(),
            timestamp: tm.timestamp,
            content: MessageContent {
                text: tm.content.clone(),
                thinking: tm.thinking.clone(),
                raw_content: serde_json::Value::Null,
            },
            tool_uses: tm.tool_uses.clone(),
        })
        .collect();

    let metadata = SessionMetadata {
        cwd: thread_data.metadata.cwd.clone(),
        version: thread_data.metadata.version.clone(),
        git_branch: thread_data.metadata.git_branch.clone(),
        start_time: thread_data.created_at,
        end_time: thread_data.updated_at,
        duration_seconds: thread_data.metadata.duration_seconds,
        message_count: thread_data.metadata.message_count,
        tool_call_count: thread_data.metadata.tool_call_count,
        file_size_bytes: 0,
    };

    Session {
        session_id: thread_data.thread_id.clone(),
        is_agent: thread_data.metadata.is_agent,
        agent_id: None,
        parent_session_id: thread_data.metadata.parent_session_id.clone(),
        metadata,
        messages,
        child_agents: thread_data.child_agents.clone(),
    }
}

fn cmd_config(action: ConfigCommands) -> anyhow::Result<()> {
    match action {
        ConfigCommands::Show {
            source,
            team,
            project,
        } => config_show(source, team.as_deref(), project.as_deref()),
        ConfigCommands::Sync { project, all } => config_sync(project.as_deref(), all),
        ConfigCommands::Push { team } => config_push(team),
        ConfigCommands::Init { context, id } => config_init(&context, &id),
    }
}

fn config_show(
    show_source: bool,
    team_id: Option<&str>,
    project_id: Option<&str>,
) -> anyhow::Result<()> {
    use snps_core::config::load_merged_config;

    println!("{}", "Loading configuration...".bright_blue());

    let merged = load_merged_config(team_id, project_id)?;
    let config = &merged.config;

    println!("\n{}", "Configuration:".bright_green().bold());
    println!("{}", "═".repeat(60).bright_green());

    if show_source {
        println!("\n{}", "[Version]".bright_yellow());
        println!(
            "  version: {} ({})",
            config.version,
            merged
                .sources
                .get("version")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );

        println!("\n{}", "[Repositories]".bright_yellow());
        println!(
            "  repositories_root: {} ({})",
            config.repositories_root.display(),
            merged
                .sources
                .get("repositories_root")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );

        println!("\n{}", "[Defaults]".bright_yellow());
        println!(
            "  editor: {} ({})",
            config.defaults.editor,
            merged
                .sources
                .get("defaults.editor")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  matter_type: {} ({})",
            config.defaults.matter_type,
            merged
                .sources
                .get("defaults.matter_type")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  visibility: {} ({})",
            config.defaults.visibility,
            merged
                .sources
                .get("defaults.visibility")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );

        println!("\n{}", "[User]".bright_yellow());
        println!(
            "  id: {} ({})",
            config.user.id,
            merged
                .sources
                .get("user.id")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  name: {} ({})",
            config.user.name,
            merged
                .sources
                .get("user.name")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  email: {} ({})",
            config.user.email,
            merged
                .sources
                .get("user.email")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );

        println!("\n{}", "[Search]".bright_yellow());
        println!(
            "  index_enabled: {} ({})",
            config.search.index_enabled,
            merged
                .sources
                .get("search.index_enabled")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  index_db: {} ({})",
            config.search.index_db.display(),
            merged
                .sources
                .get("search.index_db")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  watch_for_changes: {} ({})",
            config.search.watch_for_changes,
            merged
                .sources
                .get("search.watch_for_changes")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  exclude_patterns: {:?} ({})",
            config.search.exclude_patterns,
            merged
                .sources
                .get("search.exclude_patterns")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );

        println!("\n{}", "[Sync]".bright_yellow());
        println!(
            "  auto_sync: {} ({})",
            config.sync.auto_sync,
            merged
                .sources
                .get("sync.auto_sync")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  sync_interval_minutes: {} ({})",
            config.sync.sync_interval_minutes,
            merged
                .sources
                .get("sync.sync_interval_minutes")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
        println!(
            "  remote_portal_url: {:?} ({})",
            config.sync.remote_portal_url,
            merged
                .sources
                .get("sync.remote_portal_url")
                .map(|s| s.to_string())
                .unwrap_or_default()
        );
    } else {
        println!("\n{}", "[Version]".bright_yellow());
        println!("  version: {}", config.version);

        println!("\n{}", "[Repositories]".bright_yellow());
        println!(
            "  repositories_root: {}",
            config.repositories_root.display()
        );

        println!("\n{}", "[Defaults]".bright_yellow());
        println!("  editor: {}", config.defaults.editor);
        println!("  matter_type: {}", config.defaults.matter_type);
        println!("  visibility: {}", config.defaults.visibility);

        println!("\n{}", "[User]".bright_yellow());
        println!("  id: {}", config.user.id);
        println!("  name: {}", config.user.name);
        println!("  email: {}", config.user.email);

        println!("\n{}", "[Search]".bright_yellow());
        println!("  index_enabled: {}", config.search.index_enabled);
        println!("  index_db: {}", config.search.index_db.display());
        println!("  watch_for_changes: {}", config.search.watch_for_changes);
        println!("  exclude_patterns: {:?}", config.search.exclude_patterns);

        println!("\n{}", "[Sync]".bright_yellow());
        println!("  auto_sync: {}", config.sync.auto_sync);
        println!(
            "  sync_interval_minutes: {}",
            config.sync.sync_interval_minutes
        );
        println!("  remote_portal_url: {:?}", config.sync.remote_portal_url);
    }

    println!();
    if !show_source {
        println!(
            "{}",
            "Tip: Use --source to see where each setting comes from".bright_black()
        );
    }

    Ok(())
}

fn config_sync(_project: Option<&str>, _all: bool) -> anyhow::Result<()> {
    println!("{}", "Config sync not yet implemented".yellow());
    println!("This will sync configuration from shadow repository");
    Ok(())
}

fn config_push(_team: bool) -> anyhow::Result<()> {
    println!("{}", "Config push not yet implemented".yellow());
    println!("This will push configuration changes to shadow repository");
    Ok(())
}

fn config_init(context: &str, id: &str) -> anyhow::Result<()> {
    use snps_core::config::{get_pmsynapse_global_dir, save_global_config, GlobalConfig};

    println!(
        "{}",
        format!("Initializing config for {} context: {}", context, id).bright_blue()
    );

    let config_dir = match context {
        "user" | "personal" => get_pmsynapse_global_dir(),
        "team" => get_pmsynapse_global_dir().join("teams").join(id),
        "project" => {
            println!("{}", "Project config requires --team flag".red());
            return Err(anyhow::anyhow!("Project config requires team context"));
        }
        _ => {
            println!("{}", format!("Unknown context type: {}", context).red());
            return Err(anyhow::anyhow!("Unknown context type"));
        }
    };

    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.yaml");

    if config_path.exists() {
        println!(
            "{}",
            format!("Config already exists at: {}", config_path.display()).yellow()
        );
        return Ok(());
    }

    let default_config = GlobalConfig::default();
    save_global_config(&default_config)?;

    println!(
        "{}",
        format!("✓ Created config at: {}", config_path.display()).green()
    );

    Ok(())
}

fn cmd_matter(action: MatterCommands) -> anyhow::Result<()> {
    match action {
        MatterCommands::Create {
            matter_type,
            title,
            context,
            id,
            tags,
            visibility,
        } => matter_create(
            &matter_type,
            &title,
            &context,
            id.as_deref(),
            tags.as_deref(),
            &visibility,
        ),
        MatterCommands::List {
            context,
            id,
            matter_type,
            visibility,
            limit,
        } => matter_list(
            context.as_deref(),
            id.as_deref(),
            matter_type.as_deref(),
            visibility.as_deref(),
            limit,
        ),
        MatterCommands::Search {
            query,
            context,
            matter_type,
            tags,
            limit,
        } => matter_search(
            &query,
            context.as_deref(),
            matter_type.as_deref(),
            tags.as_deref(),
            limit,
        ),
        MatterCommands::Show { matter_id } => matter_show(&matter_id),
        MatterCommands::Edit { matter_id } => matter_edit(&matter_id),
        MatterCommands::Delete { matter_id, force } => matter_delete(&matter_id, force),
        MatterCommands::Import {
            file,
            matter_type,
            context,
        } => matter_import(&file, matter_type.as_deref(), context.as_deref()),
    }
}

fn cmd_repo(action: RepoCommands) -> anyhow::Result<()> {
    match action {
        RepoCommands::Init {
            path,
            context,
            id,
            owner_type,
            owner_id,
        } => repo_init(
            &path,
            &context,
            &id,
            owner_type.as_deref(),
            owner_id.as_deref(),
        ),
        RepoCommands::Clone { url, path } => repo_clone(&url, path.as_ref()),
        RepoCommands::Add { path } => repo_add(&path),
        RepoCommands::Remove { id } => repo_remove(&id),
        RepoCommands::List => repo_list(),
        RepoCommands::Sync { id } => repo_sync(id.as_deref()),
        RepoCommands::Index { id } => repo_index(id.as_deref()),
    }
}

fn matter_create(
    matter_type: &str,
    title: &str,
    context: &str,
    id: Option<&str>,
    tags: Option<&str>,
    visibility: &str,
) -> anyhow::Result<()> {
    println!("{}", "Creating matter item...".bright_green());

    // Parse matter type
    let matter_type: MatterType = serde_json::from_str(&format!("\"{}\"", matter_type))
        .map_err(|e| anyhow::anyhow!("Invalid matter type '{}': {}", matter_type, e))?;

    // Parse context type
    let context_type: snps_core::matter::ContextType =
        serde_json::from_str(&format!("\"{}\"", context))
            .map_err(|e| anyhow::anyhow!("Invalid context type '{}': {}", context, e))?;

    // Parse visibility
    let visibility: snps_core::matter::Visibility =
        serde_json::from_str(&format!("\"{}\"", visibility))
            .map_err(|e| anyhow::anyhow!("Invalid visibility '{}': {}", visibility, e))?;

    // Get global config for defaults
    let config = load_global_config()?;

    // Determine repository based on context
    let repositories = load_repositories()?;
    let repo = repositories
        .repositories
        .iter()
        .find(|r| {
            let repo_context = serde_json::to_string(&r.context_type)
                .unwrap()
                .trim_matches('"')
                .to_string();
            repo_context == context && (id.is_none() || r.context_id == id.unwrap_or(""))
        })
        .ok_or_else(|| anyhow::anyhow!("No repository found for context: {}", context))?;

    // Generate file path
    let file_path = generate_matter_path(&repo.path, &matter_type, title, &visibility);

    // Parse tags
    let tags: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Create frontmatter
    let frontmatter = MatterFrontmatter {
        matter_type,
        title: title.to_string(),
        context_type,
        context_id: id.unwrap_or(&config.user.id).to_string(),
        visibility,
        tags,
        created_at: chrono::Utc::now(),
        created_by: config.user.name.clone(),
        updated_at: None,
        updated_by: None,
        version: Some(1),
        status: Some("draft".to_string()),
        folder_path: None,
    };

    // Create matter item
    let matter = MatterItem {
        frontmatter,
        content: format!("# {}\n\n<!-- Start writing your content here -->\n", title),
        file_path: file_path.clone(),
    };

    // Save to disk
    matter.save()?;

    println!(
        "{}",
        format!("✓ Created matter file at: {}", file_path.display()).green()
    );

    // Index the file
    let index_db = config.search.index_db;
    if config.search.index_enabled {
        if let Ok(index) = MatterIndex::new(index_db.to_str().unwrap()) {
            if let Err(e) = index.index_file(&file_path) {
                eprintln!("{}", format!("⚠ Failed to index file: {}", e).yellow());
            } else {
                println!("{}", "✓ Indexed matter file".green());
            }
        }
    }

    Ok(())
}

fn matter_list(
    context: Option<&str>,
    id: Option<&str>,
    matter_type: Option<&str>,
    visibility: Option<&str>,
    limit: usize,
) -> anyhow::Result<()> {
    println!("{}", "Listing matter items...".bright_green());

    // Load repositories
    let repositories = load_repositories()?;

    // Filter repositories based on context
    let filtered_repos: Vec<_> = repositories
        .repositories
        .iter()
        .filter(|r| {
            context.is_none_or(|c| {
                serde_json::to_string(&r.context_type)
                    .unwrap()
                    .trim_matches('"')
                    == c
            }) && id.is_none_or(|i| r.context_id == i)
        })
        .collect();

    if filtered_repos.is_empty() {
        println!("{}", "No repositories found matching criteria".yellow());
        return Ok(());
    }

    let mut count = 0;
    for repo in filtered_repos {
        println!(
            "\n{}",
            format!("Repository: {}", repo.path.display()).bright_cyan()
        );

        // Walk the repository and find matter files
        for entry in walkdir::WalkDir::new(&repo.path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                !e.path()
                    .components()
                    .any(|c| c.as_os_str() == ".git" || c.as_os_str() == "node_modules")
            })
        {
            if count >= limit {
                break;
            }

            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "md" {
                        if let Ok(matter) = MatterItem::parse_file(&entry.path().to_path_buf()) {
                            // Apply filters
                            if let Some(mt) = matter_type {
                                let mt_parsed: MatterType =
                                    serde_json::from_str(&format!("\"{}\"", mt))?;
                                if matter.frontmatter.matter_type != mt_parsed {
                                    continue;
                                }
                            }

                            if let Some(vis) = visibility {
                                let vis_parsed: snps_core::matter::Visibility =
                                    serde_json::from_str(&format!("\"{}\"", vis))?;
                                if matter.frontmatter.visibility != vis_parsed {
                                    continue;
                                }
                            }

                            // Display matter item
                            println!(
                                "  {} - {} ({})",
                                matter.frontmatter.title.bright_white(),
                                serde_json::to_string(&matter.frontmatter.matter_type)?
                                    .trim_matches('"')
                                    .cyan(),
                                entry.path().strip_prefix(&repo.path).unwrap().display()
                            );
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!(
        "\n{}",
        format!("Found {} matter items", count).bright_green()
    );
    Ok(())
}

fn matter_search(
    query: &str,
    _context: Option<&str>,
    _matter_type: Option<&str>,
    _tags: Option<&str>,
    limit: usize,
) -> anyhow::Result<()> {
    println!("{}", format!("Searching for: {}", query).bright_green());

    let config = load_global_config()?;
    let index_db = config.search.index_db;

    if !config.search.index_enabled {
        println!(
            "{}",
            "⚠ Search index is disabled. Enable it in config.".yellow()
        );
        return Ok(());
    }

    let index = MatterIndex::new(index_db.to_str().unwrap())?;
    let results = index.search(query)?;

    println!(
        "{}",
        format!("Found {} results:", results.len()).bright_cyan()
    );

    for (i, result) in results.iter().take(limit).enumerate() {
        println!(
            "\n{}. {} ({})",
            i + 1,
            result.title.bright_white(),
            serde_json::to_string(&result.matter_type)?
                .trim_matches('"')
                .cyan()
        );
        println!("   Path: {}", result.file_path.display());
        println!("   Tags: {}", result.tags.join(", "));
        println!("   Created: {}", result.created_at.format("%Y-%m-%d %H:%M"));
    }

    Ok(())
}

fn matter_show(matter_id: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(matter_id);

    if !path.exists() {
        return Err(anyhow::anyhow!("Matter file not found: {}", matter_id));
    }

    let matter = MatterItem::parse_file(&path)?;

    println!("{}", "─".repeat(80).bright_cyan());
    println!("{}", matter.frontmatter.title.bright_white().bold());
    println!("{}", "─".repeat(80).bright_cyan());
    println!(
        "Type: {}",
        serde_json::to_string(&matter.frontmatter.matter_type)?.trim_matches('"')
    );
    println!(
        "Context: {} / {}",
        serde_json::to_string(&matter.frontmatter.context_type)?.trim_matches('"'),
        matter.frontmatter.context_id
    );
    println!(
        "Visibility: {}",
        serde_json::to_string(&matter.frontmatter.visibility)?.trim_matches('"')
    );
    println!("Tags: {}", matter.frontmatter.tags.join(", "));
    println!(
        "Created: {} by {}",
        matter.frontmatter.created_at.format("%Y-%m-%d %H:%M"),
        matter.frontmatter.created_by
    );
    if let Some(updated_at) = matter.frontmatter.updated_at {
        println!("Updated: {}", updated_at.format("%Y-%m-%d %H:%M"));
    }
    println!("{}", "─".repeat(80).bright_cyan());
    println!("\n{}", matter.content);

    Ok(())
}

fn matter_edit(matter_id: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(matter_id);

    if !path.exists() {
        return Err(anyhow::anyhow!("Matter file not found: {}", matter_id));
    }

    let config = load_global_config()?;
    let editor = std::env::var("EDITOR").unwrap_or(config.defaults.editor);

    println!("{}", format!("Opening in {}...", editor).bright_green());

    let status = std::process::Command::new(&editor).arg(&path).status()?;

    if status.success() {
        println!("{}", "✓ File edited successfully".green());

        // Re-index if enabled
        if config.search.index_enabled {
            let index_db = config.search.index_db;
            if let Ok(index) = MatterIndex::new(index_db.to_str().unwrap()) {
                if let Err(e) = index.index_file(&path) {
                    eprintln!("{}", format!("⚠ Failed to re-index file: {}", e).yellow());
                } else {
                    println!("{}", "✓ Re-indexed matter file".green());
                }
            }
        }
    } else {
        return Err(anyhow::anyhow!("Editor exited with error"));
    }

    Ok(())
}

fn matter_delete(matter_id: &str, force: bool) -> anyhow::Result<()> {
    let path = PathBuf::from(matter_id);

    if !path.exists() {
        return Err(anyhow::anyhow!("Matter file not found: {}", matter_id));
    }

    if !force {
        print!(
            "{}",
            "Are you sure you want to delete this file? (y/N): ".yellow()
        );
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Cancelled".yellow());
            return Ok(());
        }
    }

    // Remove from index first
    let config = load_global_config()?;
    if config.search.index_enabled {
        let index_db = config.search.index_db;
        if let Ok(index) = MatterIndex::new(index_db.to_str().unwrap()) {
            if let Err(e) = index.remove_from_index(&path) {
                eprintln!(
                    "{}",
                    format!("⚠ Failed to remove from index: {}", e).yellow()
                );
            }
        }
    }

    // Delete file
    std::fs::remove_file(&path)?;
    println!("{}", format!("✓ Deleted: {}", path.display()).green());

    Ok(())
}

fn matter_import(
    file: &Path,
    matter_type: Option<&str>,
    context: Option<&str>,
) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Importing: {}", file.display()).bright_green()
    );

    if !file.exists() {
        return Err(anyhow::anyhow!("File not found: {}", file.display()));
    }

    // Try to parse as existing matter file
    if let Ok(matter) = MatterItem::parse_file(&file.to_path_buf()) {
        println!(
            "{}",
            "File already has valid frontmatter, copying as-is...".cyan()
        );

        // Determine target repository
        let repositories = load_repositories()?;
        let context_str = context.unwrap_or("user");

        let repo = repositories
            .repositories
            .iter()
            .find(|r| {
                let repo_context = serde_json::to_string(&r.context_type)
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
                repo_context == context_str
            })
            .ok_or_else(|| anyhow::anyhow!("No repository found for context: {}", context_str))?;

        let target_path = generate_matter_path(
            &repo.path,
            &matter.frontmatter.matter_type,
            &matter.frontmatter.title,
            &matter.frontmatter.visibility,
        );

        // Copy the matter item to new location
        let new_matter = MatterItem {
            frontmatter: matter.frontmatter,
            content: matter.content,
            file_path: target_path,
        };
        new_matter.save()?;

        println!(
            "{}",
            format!("✓ Imported to: {}", new_matter.file_path.display()).green()
        );
    } else {
        // File doesn't have frontmatter, create new matter item
        println!(
            "{}",
            "No frontmatter found, creating new matter item...".cyan()
        );

        let content = std::fs::read_to_string(file)?;
        let title = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Imported Document");

        // Use provided matter type or default to Document
        let matter_type_str = matter_type.unwrap_or("document");
        let matter_type: MatterType = serde_json::from_str(&format!("\"{}\"", matter_type_str))?;

        let context_str = context.unwrap_or("user");
        let context_type: snps_core::matter::ContextType =
            serde_json::from_str(&format!("\"{}\"", context_str))?;

        let config = load_global_config()?;
        let repositories = load_repositories()?;

        let repo = repositories
            .repositories
            .iter()
            .find(|r| {
                let repo_context = serde_json::to_string(&r.context_type)
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
                repo_context == context_str
            })
            .ok_or_else(|| anyhow::anyhow!("No repository found for context: {}", context_str))?;

        let file_path = generate_matter_path(
            &repo.path,
            &matter_type,
            title,
            &snps_core::matter::Visibility::Private,
        );

        let frontmatter = MatterFrontmatter {
            matter_type,
            title: title.to_string(),
            context_type,
            context_id: config.user.id.clone(),
            visibility: snps_core::matter::Visibility::Private,
            tags: vec![],
            created_at: chrono::Utc::now(),
            created_by: config.user.name.clone(),
            updated_at: None,
            updated_by: None,
            version: Some(1),
            status: Some("imported".to_string()),
            folder_path: None,
        };

        let matter = MatterItem {
            frontmatter,
            content,
            file_path: file_path.clone(),
        };

        matter.save()?;
        println!(
            "{}",
            format!("✓ Imported to: {}", file_path.display()).green()
        );
    }

    Ok(())
}

fn repo_init(
    path: &Path,
    context: &str,
    id: &str,
    owner_type: Option<&str>,
    owner_id: Option<&str>,
) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Initializing matter repository at {}...", path.display()).bright_cyan()
    );
    println!();

    // Parse context type
    let context_type: snps_core::repository::ContextType = match context.to_lowercase().as_str() {
        "user" => snps_core::repository::ContextType::User,
        "team" => snps_core::repository::ContextType::Team,
        "project" => snps_core::repository::ContextType::Project,
        _ => {
            println!("{}", format!("Invalid context type: {}", context).red());
            println!("  Valid types: user, team, project");
            return Err(anyhow::anyhow!("Invalid context type"));
        }
    };

    // Create repository directory
    std::fs::create_dir_all(path)?;

    // Create .pmsynapse directory
    let config_dir = path.join(".pmsynapse");
    std::fs::create_dir_all(&config_dir)?;

    // Create context.yaml
    #[derive(Serialize)]
    struct ContextInfo {
        context_type: String,
        id: String,
        visibility: String,
    }

    #[derive(Serialize)]
    struct Owner {
        owner_type: String,
        id: String,
    }

    #[derive(Serialize)]
    struct RepositoryContext {
        context: ContextInfo,
        owner: Option<Owner>,
    }

    let context_config = RepositoryContext {
        context: ContextInfo {
            context_type: context.to_string(),
            id: id.to_string(),
            visibility: "private".to_string(),
        },
        owner: owner_type.map(|ot| Owner {
            owner_type: ot.to_string(),
            id: owner_id.unwrap_or_default().to_string(),
        }),
    };

    let context_path = config_dir.join("context.yaml");
    let yaml = serde_yaml::to_string(&context_config)?;
    std::fs::write(&context_path, yaml)?;

    println!("{}", "  ✓ Created .pmsynapse/context.yaml".green());

    // Initialize git if not already a repo
    if !path.join(".git").exists() {
        let output = std::process::Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()?;

        if output.status.success() {
            println!("{}", "  ✓ Initialized git repository".green());
        } else {
            println!("{}", "  ⚠ Failed to initialize git repository".yellow());
        }
    } else {
        println!("{}", "  ✓ Git repository already exists".green());
    }

    // Generate repository ID from path
    let repo_id = format!(
        "{}-{}",
        context,
        path.file_name().and_then(|n| n.to_str()).unwrap_or("repo")
    );

    // Add to global repositories.yaml
    let mapping = snps_core::repository::RepositoryMapping {
        id: repo_id.clone(),
        path: path.canonicalize()?,
        context_type,
        context_id: id.to_string(),
        visibility: snps_core::repository::Visibility::Private,
        role: None,
        auto_index: true,
        sync: snps_core::repository::RepositorySyncConfig::default(),
    };

    snps_core::repository::add_repository(mapping)?;
    println!(
        "{}",
        format!("  ✓ Added to repositories config (id: {})", repo_id).green()
    );

    println!();
    println!("{}", "Repository initialized successfully!".bright_green());
    println!("  Path: {}", path.display());
    println!("  Context: {} ({})", context, id);
    println!("  ID: {}", repo_id);

    Ok(())
}

fn repo_clone(url: &str, path: Option<&PathBuf>) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Cloning repository from {}...", url).bright_cyan()
    );

    // Determine target path
    let target_path = if let Some(p) = path {
        p.clone()
    } else {
        // Extract repo name from URL
        let repo_name = url
            .split('/')
            .next_back()
            .and_then(|s| s.strip_suffix(".git"))
            .unwrap_or("repo");
        PathBuf::from(repo_name)
    };

    // Clone using git
    let output = std::process::Command::new("git")
        .args(["clone", url, &target_path.to_string_lossy()])
        .output()?;

    if !output.status.success() {
        println!("{}", "  ✗ Clone failed".red());
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);
        return Err(anyhow::anyhow!("Git clone failed"));
    }

    println!(
        "{}",
        format!("  ✓ Cloned to {}", target_path.display()).green()
    );

    // Check if it has a context.yaml file
    let context_path = target_path.join(".pmsynapse/context.yaml");
    if context_path.exists() {
        println!("{}", "  ✓ Found .pmsynapse/context.yaml".green());
        // Automatically add to repositories
        repo_add(&target_path)?;
    } else {
        println!(
            "{}",
            "  ⚠ No .pmsynapse/context.yaml found. Use 'snps repo add' to register.".yellow()
        );
    }

    Ok(())
}

fn repo_add(path: &Path) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Adding repository at {}...", path.display()).bright_cyan()
    );

    // Check if path exists
    if !path.exists() {
        println!(
            "{}",
            format!("  ✗ Path does not exist: {}", path.display()).red()
        );
        return Err(anyhow::anyhow!("Path does not exist"));
    }

    // Check for context.yaml
    let context_path = path.join(".pmsynapse/context.yaml");
    if !context_path.exists() {
        println!(
            "{}",
            "  ✗ Not a matter repository (missing .pmsynapse/context.yaml)".red()
        );
        return Err(anyhow::anyhow!("Missing context.yaml"));
    }

    // Load context.yaml
    #[derive(Deserialize)]
    struct ContextInfo {
        context_type: String,
        id: String,
        visibility: Option<String>,
    }

    #[derive(Deserialize)]
    struct RepositoryContext {
        context: ContextInfo,
    }

    let context_content = std::fs::read_to_string(&context_path)?;
    let repo_context: RepositoryContext = serde_yaml::from_str(&context_content)?;

    // Parse context type
    let context_type: snps_core::repository::ContextType =
        match repo_context.context.context_type.to_lowercase().as_str() {
            "user" => snps_core::repository::ContextType::User,
            "team" => snps_core::repository::ContextType::Team,
            "project" => snps_core::repository::ContextType::Project,
            _ => {
                println!(
                    "{}",
                    format!(
                        "Invalid context type in context.yaml: {}",
                        repo_context.context.context_type
                    )
                    .red()
                );
                return Err(anyhow::anyhow!("Invalid context type"));
            }
        };

    // Parse visibility
    let visibility = match repo_context
        .context
        .visibility
        .as_deref()
        .unwrap_or("private")
        .to_lowercase()
        .as_str()
    {
        "private" => snps_core::repository::Visibility::Private,
        "shared" => snps_core::repository::Visibility::Shared,
        "mixed" => snps_core::repository::Visibility::Mixed,
        _ => snps_core::repository::Visibility::Private,
    };

    // Generate repository ID
    let repo_id = format!(
        "{}-{}",
        repo_context.context.context_type,
        path.file_name().and_then(|n| n.to_str()).unwrap_or("repo")
    );

    // Add to repositories.yaml
    let mapping = snps_core::repository::RepositoryMapping {
        id: repo_id.clone(),
        path: path.canonicalize()?,
        context_type,
        context_id: repo_context.context.id.clone(),
        visibility,
        role: None,
        auto_index: true,
        sync: snps_core::repository::RepositorySyncConfig::default(),
    };

    snps_core::repository::add_repository(mapping)?;

    println!("{}", "  ✓ Repository added".green());
    println!("  ID: {}", repo_id);
    println!(
        "  Context: {} ({})",
        repo_context.context.context_type, repo_context.context.id
    );

    Ok(())
}

fn repo_remove(id: &str) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("Removing repository '{}'...", id).bright_cyan()
    );

    snps_core::repository::remove_repository(id)?;

    println!("{}", "  ✓ Repository removed from config".green());
    println!("  (repository files not deleted)");

    Ok(())
}

fn repo_list() -> anyhow::Result<()> {
    println!("{}", "Configured Repositories".bright_blue());
    println!();

    let config = snps_core::repository::load_repositories()?;

    if config.repositories.is_empty() {
        println!("{}", "  No repositories configured".dimmed());
        println!();
        println!("Use 'snps repo init' to create a new repository");
        println!("Use 'snps repo add' to add an existing repository");
        return Ok(());
    }

    for repo in &config.repositories {
        println!("{}", format!("● {}", repo.id).bright_cyan());
        println!("  Path:    {}", repo.path.display());
        println!("  Context: {:?} ({})", repo.context_type, repo.context_id);
        println!("  Visibility: {:?}", repo.visibility);
        if let Some(ref remote) = repo.sync.remote {
            println!("  Remote:  {}", remote);
        }
        println!();
    }

    println!(
        "{}",
        format!("Total: {} repositories", config.repositories.len()).dimmed()
    );

    Ok(())
}

fn repo_sync(id: Option<&str>) -> anyhow::Result<()> {
    if let Some(repo_id) = id {
        println!(
            "{}",
            format!("Syncing repository '{}'...", repo_id).bright_cyan()
        );
    } else {
        println!("{}", "Syncing all repositories...".bright_cyan());
    }

    let config = snps_core::repository::load_repositories()?;
    let repos_to_sync: Vec<_> = if let Some(repo_id) = id {
        config
            .repositories
            .iter()
            .filter(|r| r.id == repo_id)
            .collect()
    } else {
        config.repositories.iter().collect()
    };

    if repos_to_sync.is_empty() {
        println!("{}", "  No repositories to sync".dimmed());
        return Ok(());
    }

    for repo in repos_to_sync {
        println!();
        println!("{}", format!("Syncing {}...", repo.id).bright_blue());

        if !repo.sync.enabled {
            println!("{}", "  ⚠ Sync disabled for this repository".yellow());
            continue;
        }

        if repo.sync.remote.is_none() {
            println!("{}", "  ⚠ No remote configured".yellow());
            continue;
        }

        // Pull from remote
        let output = std::process::Command::new("git")
            .args(["pull", "origin", &repo.sync.branch])
            .current_dir(&repo.path)
            .output()?;

        if output.status.success() {
            println!("{}", "  ✓ Pulled from remote".green());
        } else {
            println!("{}", "  ✗ Pull failed".red());
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("    {}", stderr);
        }

        // Push to remote
        let output = std::process::Command::new("git")
            .args(["push", "origin", &repo.sync.branch])
            .current_dir(&repo.path)
            .output()?;

        if output.status.success() {
            println!("{}", "  ✓ Pushed to remote".green());
        } else {
            println!("{}", "  ⚠ Push failed (may be up to date)".yellow());
        }
    }

    println!();
    println!("{}", "Sync complete".bright_green());

    Ok(())
}

fn repo_index(id: Option<&str>) -> anyhow::Result<()> {
    if let Some(repo_id) = id {
        println!(
            "{}",
            format!("Rebuilding index for repository '{}'...", repo_id).bright_cyan()
        );
    } else {
        println!(
            "{}",
            "Rebuilding index for all repositories...".bright_cyan()
        );
    }

    let config = snps_core::repository::load_repositories()?;
    let repos_to_index: Vec<_> = if let Some(repo_id) = id {
        config
            .repositories
            .iter()
            .filter(|r| r.id == repo_id)
            .collect()
    } else {
        config.repositories.iter().collect()
    };

    if repos_to_index.is_empty() {
        println!("{}", "  No repositories to index".dimmed());
        return Ok(());
    }

    for repo in repos_to_index {
        println!();
        println!("{}", format!("Indexing {}...", repo.id).bright_blue());
        println!("  Path: {}", repo.path.display());

        if !repo.auto_index {
            println!("{}", "  ⚠ Auto-index disabled for this repository".yellow());
            continue;
        }

        // TODO: Implement actual indexing with CozoDB when Phase 3 is complete
        // For now, just count files
        let mut file_count = 0;
        for entry in WalkDir::new(&repo.path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "markdown" {
                        file_count += 1;
                    }
                }
            }
        }

        println!(
            "{}",
            format!("  ✓ Found {} markdown files", file_count).green()
        );
        println!(
            "{}",
            "  (full indexing pending Phase 3 implementation)".dimmed()
        );
    }

    println!();
    println!("{}", "Index rebuild complete".bright_green());

    Ok(())
}
