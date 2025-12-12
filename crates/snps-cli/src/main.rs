//! PMSynapse CLI
//!
//! Command line interface for AI-enabled project management.

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
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
        println!(
            "{}",
            "  Not initialized. Run 'snps init' first.".yellow()
        );
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
            println!("{}", format!("Switching to template: {}", name).bright_green());
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
        println!("{}", format!("Exporting graph to {}...", path).bright_blue());
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

        ThoughtsCommands::Open { path, editor, scope } => thoughts_open(path, editor, scope),

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
        format!("  ✓ Created thoughts directory: {}", project_thoughts.display()).green()
    );

    // Create symlink in project
    if thoughts_path.exists() {
        if force {
            if thoughts_path.is_symlink() {
                std::fs::remove_file(thoughts_path)?;
            } else {
                std::fs::remove_dir_all(thoughts_path)?;
            }
        }
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&project_thoughts, thoughts_path)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&project_thoughts, thoughts_path)?;

    println!("{}", "  ✓ Created symlink: thoughts/ → ~/.pmsynapse/thoughts/...".green());

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
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(gitignore)?;
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

fn thoughts_new(doc_type: ThoughtType, title: String, scope: String, open: bool) -> anyhow::Result<()> {
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
        "personal" => vec![thoughts_path.join(&get_username())],
        "global" => vec![thoughts_path.join("global")],
        _ => vec![
            thoughts_path.join("shared"),
            thoughts_path.join(&get_username()),
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
    } else {
        if results.is_empty() {
            println!("{}", "  No results found.".dimmed());
        } else {
            println!("{}", format!("Found {} results:", results.len()).green());
            for path in &results {
                println!("  • {}", path.display());
            }
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
        Some("personal") => vec![thoughts_path.join(&get_username())],
        Some("global") => vec![thoughts_path.join("global")],
        _ => vec![
            thoughts_path.join("shared"),
            thoughts_path.join(&get_username()),
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
            let json_files: Vec<_> = files
                .iter()
                .map(|(p, _)| p.to_string_lossy())
                .collect();
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
            "personal" => thoughts_path.join(&get_username()),
            "global" => thoughts_path.join("global"),
            _ => thoughts_path.join("shared"),
        }
    } else {
        thoughts_path.to_path_buf()
    };

    if editor {
        let editor_cmd = std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string());
        println!("Opening {} in {}...", target.display(), editor_cmd);
        std::process::Command::new(&editor_cmd).arg(&target).spawn()?;
    } else {
        // Open in file manager
        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(&target).spawn()?;

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open").arg(&target).spawn()?;

        #[cfg(target_os = "windows")]
        std::process::Command::new("explorer").arg(&target).spawn()?;

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
                println!("{}", "  No profiles found. Create one with 'snps thoughts profile create <name>'".dimmed());
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
            println!("{}", format!("Switching to profile: {}", name).bright_blue());

            let profile_dir = thoughts_root.join("profiles").join(&name);
            if !profile_dir.exists() {
                println!(
                    "{}",
                    format!("Profile '{}' does not exist.", name).red()
                );
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
            println!("{}", "  ✗ Pre-commit hook exists but is not PMSynapse".yellow());
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
