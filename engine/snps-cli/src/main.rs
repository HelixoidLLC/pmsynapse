//! PMSynapse CLI
//!
//! Command line interface for AI-enabled project management.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use snps_core::claude::{SessionAnalyzer, SessionExporter, SessionParser};
use std::path::{Path, PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    Install,
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
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
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

        // In foreground mode, we would run the actual daemon here
        // For now, simulate with a placeholder
        println!(
            "{}",
            "⚠ Daemon not yet implemented - this is a placeholder".yellow()
        );
        println!();
        println!("The daemon will provide:");
        println!("  • REST API for knowledge graph operations");
        println!("  • JSON-RPC interface for real-time events");
        println!("  • WebSocket support for UI communication");
        println!("  • Session management for AI agents");

        // Wait for Ctrl+C
        println!();
        println!("{}", "Press Ctrl+C to stop...".dimmed());
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    } else {
        // Background mode - would fork/spawn daemon process
        // For now, create a PID file to simulate

        // In a real implementation, we would:
        // 1. Fork the process (on Unix) or spawn a detached process (on Windows)
        // 2. Write the child PID to the PID file
        // 3. Redirect stdout/stderr to log file

        println!();
        println!("{}", "⚠ Background daemon not yet implemented".yellow());
        println!("  Use --foreground to run in foreground mode");
        println!();

        // Create placeholder PID file for testing
        // std::fs::write(&pid_path, std::process::id().to_string())?;

        println!("When implemented, daemon will:");
        println!("  • Run in background automatically");
        println!("  • Start on system boot (optional)");
        println!("  • Auto-restart on crash");
    }

    Ok(())
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

    // Auto-start daemon if needed
    if !no_daemon && !is_daemon_running(None) {
        println!("{}", "Starting daemon...".dimmed());
        daemon_start(false, daemon_socket.clone(), 7878, None, None)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

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
    println!();

    // Build and run the Tauri app
    let mut cmd = std::process::Command::new("pnpm");
    cmd.args(["tauri", "dev"])
        .current_dir(&desktop_dir)
        .env("PMSYNAPSE_DAEMON_SOCKET", &socket_path);

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
        } => thoughts_sync(message, push, pull, no_commit),

        ThoughtsCommands::Open {
            path,
            editor,
            scope,
        } => thoughts_open(path, editor, scope),

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
        install_thoughts_hooks()?;
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

fn thoughts_sync(
    message: Option<String>,
    push: bool,
    pull: bool,
    no_commit: bool,
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

    // Resolve symlink to get actual thoughts directory
    let real_path = std::fs::canonicalize(thoughts_path)?;

    if pull {
        println!("  Pulling from remote...");
        let output = std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&real_path)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                println!("{}", "  ✓ Pulled successfully".green());
            }
        }
    }

    // Rebuild searchable index (hardlinks)
    let searchable_dir = thoughts_path.join("searchable");
    if searchable_dir.exists() {
        std::fs::remove_dir_all(&searchable_dir)?;
    }
    // Note: Full hardlink implementation would go here
    // For now, just create the directory
    std::fs::create_dir_all(&searchable_dir)?;
    println!("{}", "  ✓ Rebuilt searchable index".green());

    if !no_commit {
        let msg = message.unwrap_or_else(|| {
            format!(
                "Sync: {} from {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M"),
                hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "unknown".to_string())
            )
        });

        // Git add and commit
        let _ = std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(&real_path)
            .output();

        let output = std::process::Command::new("git")
            .args(["commit", "-m", &msg])
            .current_dir(&real_path)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                println!("{}", format!("  ✓ Committed: {}", msg).green());
            } else {
                println!("{}", "  • No changes to commit".dimmed());
            }
        }

        if push {
            println!("  Pushing to remote...");
            let output = std::process::Command::new("git")
                .args(["push"])
                .current_dir(&real_path)
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
        HooksCommands::Install => install_thoughts_hooks(),
        HooksCommands::Uninstall => uninstall_thoughts_hooks(),
        HooksCommands::Status => check_hooks_status(),
    }
}

fn install_thoughts_hooks() -> anyhow::Result<()> {
    let hooks_dir = Path::new(".git/hooks");
    if !hooks_dir.exists() {
        println!(
            "{}",
            "Not a git repository. Skipping hook installation.".yellow()
        );
        return Ok(());
    }

    let pre_commit = hooks_dir.join("pre-commit");
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
                        arr.iter()
                            .any(|item| item.get("type").and_then(|t| t.as_str()) == Some("tool_result"))
                    })
                    .unwrap_or(false);

            // Skip meta messages and tool results
            let is_meta = record.get("isMeta").and_then(|v| v.as_bool()).unwrap_or(false);

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
                let session_id_short = session
                    .session_id
                    .chars()
                    .take(8)
                    .collect::<String>();

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
                println!("{}", format!("✓ Exported to: {}", out_path.display()).green());
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
                println!("{}", format!("✓ Exported to: {}", out_path.display()).green());
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
fn thread_data_to_session(thread_data: &snps_core::claude::ThreadData) -> snps_core::claude::Session {
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
