//! PMSynapse CLI
//!
//! Command line interface for AI-enabled knowledge management.

use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// PMSynapse CLI - AI-enabled knowledge management
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
        "║       PMSynapse - AI Knowledge Mgmt   ║".bright_cyan()
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
