mod commands;
mod models;
mod templates;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dg")]
#[command(about = "Decision Graph - Text-based knowledge graph for company decisions")]
#[command(version)]
struct Cli {
    /// Path to docs directory (default: ./docs)
    #[arg(short = 'D', long, global = true, default_value = "docs")]
    docs_dir: String,

    /// Quiet mode - only output on errors
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize decision graph in current directory
    Init,

    /// Create a new record
    New {
        /// Record type: decision, strategy, policy, client, opportunity, process, hiring
        #[arg(value_name = "TYPE")]
        record_type: String,

        /// Title of the record
        #[arg(value_name = "TITLE")]
        title: String,
    },

    /// List all records
    #[command(alias = "ls")]
    List {
        /// Filter by type
        #[arg(short, long)]
        r#type: Option<String>,

        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Output format: table, json, ids
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show a record
    Show {
        /// Record ID (e.g., DEC-001)
        id: String,

        /// Show linked records
        #[arg(short, long)]
        links: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Add a link between records
    Link {
        /// Source record ID
        from: String,

        /// Link type: supersedes, depends_on, enables, relates_to, conflicts_with, refines, implements
        link_type: String,

        /// Target record ID
        to: String,
    },

    /// Remove a link between records
    Unlink {
        /// Source record ID
        from: String,

        /// Link type
        link_type: String,

        /// Target record ID
        to: String,
    },

    /// Search records
    Search {
        /// Search query
        query: String,

        /// Search in content (not just titles)
        #[arg(short, long)]
        content: bool,
    },

    /// Show graph of relationships
    Graph {
        /// Starting record ID (optional, shows all if not specified)
        id: Option<String>,

        /// Depth of traversal
        #[arg(short, long, default_value = "2")]
        depth: usize,

        /// Output format: text, dot, json
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Update record status
    Status {
        /// Record ID
        id: String,

        /// New status: draft, proposed, accepted, deprecated, superseded
        status: String,
    },

    /// Rebuild the index
    Reindex,

    /// Export records
    Export {
        /// Output format: json, csv
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Validate all records
    Validate,

    /// Show statistics
    Stats,

    /// Format markdown files
    Fmt {
        /// Check formatting without making changes (exit 1 if files need formatting)
        #[arg(short, long)]
        check: bool,

        /// Specific files to format (default: all files in .decisions)
        #[arg(value_name = "FILES")]
        files: Option<Vec<String>>,
    },

    /// Lint records for errors
    Lint {
        /// Specific files to lint (default: all files)
        #[arg(value_name = "FILES")]
        files: Option<Vec<String>>,

        /// Strict mode: require tags, content, and type-specific links
        #[arg(short, long)]
        strict: bool,

        /// Warn about orphaned records (no incoming or outgoing links)
        #[arg(short = 'o', long)]
        warn_orphans: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(&cli.docs_dir),
        Commands::New { record_type, title } => {
            commands::new::run(&cli.docs_dir, &record_type, &title)
        }
        Commands::List {
            r#type,
            status,
            tag,
            format,
        } => commands::list::run(&cli.docs_dir, r#type, status, tag, &format),
        Commands::Show { id, links, json } => commands::show::run(&cli.docs_dir, &id, links, json),
        Commands::Link {
            from,
            link_type,
            to,
        } => commands::link::run(&cli.docs_dir, &from, &link_type, &to),
        Commands::Unlink {
            from,
            link_type,
            to,
        } => commands::unlink::run(&cli.docs_dir, &from, &link_type, &to),
        Commands::Search { query, content } => commands::search::run(&cli.docs_dir, &query, content),
        Commands::Graph { id, depth, format } => {
            commands::graph::run(&cli.docs_dir, id.as_deref(), depth, &format)
        }
        Commands::Status { id, status } => commands::status::run(&cli.docs_dir, &id, &status),
        Commands::Reindex => commands::reindex::run(&cli.docs_dir),
        Commands::Export { format, output } => {
            commands::export::run(&cli.docs_dir, &format, output.as_deref())
        }
        Commands::Validate => commands::validate::run(&cli.docs_dir, cli.quiet),
        Commands::Stats => commands::stats::run(&cli.docs_dir),
        Commands::Fmt { check, files } => commands::fmt::run(&cli.docs_dir, check, files, cli.quiet),
        Commands::Lint {
            files,
            strict,
            warn_orphans,
        } => commands::lint::run(&cli.docs_dir, files, strict, warn_orphans, cli.quiet),
    }
}
