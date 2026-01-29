mod commands;
mod git;
mod models;
mod serve;
mod templates;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

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

        /// Use temporary draft ID (for multi-player mode, finalize before merging)
        #[arg(long)]
        draft: bool,
    },

    /// Finalize draft records by converting temporary IDs to permanent ones
    Finalize {
        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
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

    /// Edit a record in $EDITOR
    Edit {
        /// Record ID (e.g., DEC-001)
        id: String,
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

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Output format: table, json, ids
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show graph of relationships
    Graph {
        /// Starting record ID (optional, shows all if not specified)
        id: Option<String>,

        /// Depth of traversal
        #[arg(short, long, default_value = "2")]
        depth: usize,

        /// Output format: text, dot, d2, json
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Render format for d2: svg
        #[arg(short, long)]
        render: Option<String>,

        /// Output file path (for rendered output)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Update record status
    Status {
        /// Record ID
        id: String,

        /// New status: draft, proposed, accepted, deprecated, superseded
        status: String,
    },

    /// Resolve an incident
    Resolve {
        /// Incident ID (e.g., INC-001)
        id: String,

        /// Resolution note (optional)
        #[arg(short, long)]
        note: Option<String>,
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

    /// Show changes since last commit
    Diff {
        /// Base reference to compare against (default: HEAD)
        #[arg(short, long)]
        base: Option<String>,
    },

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

        /// Check for conflicts with foundational principles
        #[arg(short = 'p', long)]
        principles: bool,

        /// Validate @username mentions and action item owners
        #[arg(short = 'u', long)]
        check_users: bool,
    },

    /// List foundational records (core principles)
    Principles {
        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Trace why a record exists (follow depends_on backward)
    Why {
        /// Record ID
        id: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show impact of changing a record (what depends on it)
    Impact {
        /// Record ID
        id: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Get context for a topic (LLM-friendly)
    Context {
        /// Topic to search for
        topic: String,

        /// Depth of neighbor traversal
        #[arg(short, long, default_value = "2")]
        depth: usize,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Generate static site
    Build {
        /// Output directory (default: docs/_site)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Start HTTP server for browsing records
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Open browser automatically
        #[arg(long)]
        open: bool,

        /// Watch for file changes and auto-reload
        #[arg(short, long)]
        watch: bool,
    },

    /// Suggest missing decisions from git commits
    Suggest {
        /// Look at commits since this date (e.g., "1 week ago", "2024-01-01")
        #[arg(short, long, default_value = "1 week ago")]
        since: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// List orphaned records (no links)
    Orphans {
        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show timeline of records
    Timeline {
        /// Number of records to show
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Sort by: created, updated
        #[arg(short, long, default_value = "updated")]
        sort: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Change a record's type while preserving chronological order
    Retype {
        /// Record ID to change (e.g., DEC-002)
        id: String,

        /// New type: decision, strategy, policy, customer, adr, incident, etc.
        #[arg(value_name = "NEW_TYPE")]
        new_type: String,

        /// Apply changes without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Set up GitHub Actions for multi-player workflows
    SetupCi {
        /// Feature to set up: finalize, lint, or all
        #[arg(value_name = "FEATURE", default_value = "all")]
        feature: String,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage users
    Users {
        #[command(subcommand)]
        action: UsersAction,
    },

    /// Manage teams
    Teams {
        #[command(subcommand)]
        action: TeamsAction,
    },

    /// View historical configurations from git
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
}

#[derive(Subcommand)]
enum UsersAction {
    /// List all users
    List {
        /// Include deprecated users
        #[arg(long)]
        include_deprecated: bool,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show user details
    Show {
        /// Username
        username: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Add a new user
    Add {
        /// Username
        username: String,

        /// Display name
        #[arg(long)]
        name: Option<String>,

        /// Email address
        #[arg(long)]
        email: Option<String>,

        /// GitHub username
        #[arg(long)]
        github: Option<String>,

        /// Teams (comma-separated)
        #[arg(long, value_delimiter = ',')]
        teams: Option<Vec<String>>,
    },

    /// Mark a user as deprecated
    Deprecate {
        /// Username
        username: String,

        /// Reason for deprecation
        #[arg(long)]
        note: Option<String>,
    },

    /// Import users from GitHub organization
    ImportGithub {
        /// GitHub organization name
        #[arg(long)]
        org: String,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum TeamsAction {
    /// List all teams
    List {
        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show team details
    Show {
        /// Team ID
        id: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Create a new team
    Create {
        /// Team ID (e.g., "engineering", "platform")
        id: String,

        /// Team display name
        #[arg(long)]
        name: String,

        /// Team lead username
        #[arg(long)]
        lead: Option<String>,

        /// Parent team ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// Add a member to a team
    AddMember {
        /// Team ID
        team: String,

        /// Username to add
        username: String,
    },

    /// Remove a member from a team
    RemoveMember {
        /// Team ID
        team: String,

        /// Username to remove
        username: String,
    },

    /// Import teams from GitHub organization
    ImportGithub {
        /// GitHub organization name
        #[arg(long)]
        org: String,

        /// Preview changes without applying
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum HistoryAction {
    /// Show configuration history
    Config {
        /// Show config at specific date (YYYY-MM-DD)
        #[arg(long)]
        at: Option<String>,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show team membership history
    Team {
        /// Team ID
        id: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show user tenure across teams
    User {
        /// Username
        username: String,

        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(&cli.docs_dir),
        Commands::New {
            record_type,
            title,
            draft,
        } => commands::new::run(&cli.docs_dir, &record_type, &title, draft),
        Commands::Finalize { dry_run } => commands::finalize::run(&cli.docs_dir, dry_run),
        Commands::List {
            r#type,
            status,
            tag,
            format,
        } => commands::list::run(&cli.docs_dir, r#type, status, tag, &format),
        Commands::Show { id, links, json } => commands::show::run(&cli.docs_dir, &id, links, json),
        Commands::Edit { id } => commands::edit::run(&cli.docs_dir, &id),
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
        Commands::Search {
            query,
            content,
            tag,
            format,
        } => commands::search::run(&cli.docs_dir, &query, content, tag.as_deref(), &format),
        Commands::Graph {
            id,
            depth,
            format,
            render,
            output,
        } => commands::graph::run(
            &cli.docs_dir,
            id.as_deref(),
            depth,
            &format,
            render.as_deref(),
            output.as_deref(),
        ),
        Commands::Status { id, status } => commands::status::run(&cli.docs_dir, &id, &status),
        Commands::Resolve { id, note } => {
            commands::resolve::run(&cli.docs_dir, &id, note.as_deref())
        }
        Commands::Reindex => commands::reindex::run(&cli.docs_dir),
        Commands::Export { format, output } => {
            commands::export::run(&cli.docs_dir, &format, output.as_deref())
        }
        Commands::Validate => commands::validate::run(&cli.docs_dir, cli.quiet),
        Commands::Stats => commands::stats::run(&cli.docs_dir),
        Commands::Diff { base } => commands::diff::run(&cli.docs_dir, base.as_deref()),
        Commands::Fmt { check, files } => {
            commands::fmt::run(&cli.docs_dir, check, files, cli.quiet)
        }
        Commands::Lint {
            files,
            strict,
            warn_orphans,
            principles,
            check_users,
        } => commands::lint::run(
            &cli.docs_dir,
            files,
            strict,
            warn_orphans,
            principles,
            check_users,
            cli.quiet,
        ),
        Commands::Principles { format } => commands::principles::run(&cli.docs_dir, &format),
        Commands::Why { id, format } => commands::why::run(&cli.docs_dir, &id, &format),
        Commands::Impact { id, format } => commands::impact::run(&cli.docs_dir, &id, &format),
        Commands::Context {
            topic,
            depth,
            format,
        } => commands::context::run(&cli.docs_dir, &topic, depth, &format),
        Commands::Build { output } => commands::build::run(&cli.docs_dir, output.as_deref()),
        Commands::Serve { port, open, watch } => {
            commands::serve::run(&cli.docs_dir, port, open, watch)
        }
        Commands::Suggest { since, format } => {
            commands::suggest::run(&cli.docs_dir, Some(&since), &format)
        }
        Commands::Orphans { format } => commands::orphans::run(&cli.docs_dir, &format),
        Commands::Timeline {
            limit,
            sort,
            format,
        } => commands::timeline::run(&cli.docs_dir, limit, &sort, &format),
        Commands::Retype {
            id,
            new_type,
            force,
        } => commands::retype::run(&cli.docs_dir, &id, &new_type, force),
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "dg", &mut std::io::stdout());
            Ok(())
        }
        Commands::SetupCi { feature, dry_run } => {
            commands::setup_ci::run(&cli.docs_dir, &feature, dry_run)
        }
        Commands::Users { action } => match action {
            UsersAction::List {
                include_deprecated,
                format,
            } => commands::users::list(&cli.docs_dir, include_deprecated, &format),
            UsersAction::Show { username, format } => {
                commands::users::show(&cli.docs_dir, &username, &format)
            }
            UsersAction::Add {
                username,
                name,
                email,
                github,
                teams,
            } => commands::users::add(
                &cli.docs_dir,
                &username,
                name.as_deref(),
                email.as_deref(),
                github.as_deref(),
                teams,
            ),
            UsersAction::Deprecate { username, note } => {
                commands::users::deprecate(&cli.docs_dir, &username, note.as_deref())
            }
            UsersAction::ImportGithub { org, dry_run } => {
                commands::users::import_github(&cli.docs_dir, &org, dry_run)
            }
        },
        Commands::Teams { action } => match action {
            TeamsAction::List { format } => commands::teams::list(&cli.docs_dir, &format),
            TeamsAction::Show { id, format } => commands::teams::show(&cli.docs_dir, &id, &format),
            TeamsAction::Create {
                id,
                name,
                lead,
                parent,
            } => commands::teams::create(
                &cli.docs_dir,
                &id,
                &name,
                lead.as_deref(),
                parent.as_deref(),
            ),
            TeamsAction::AddMember { team, username } => {
                commands::teams::add_member(&cli.docs_dir, &team, &username)
            }
            TeamsAction::RemoveMember { team, username } => {
                commands::teams::remove_member(&cli.docs_dir, &team, &username)
            }
            TeamsAction::ImportGithub { org, dry_run } => {
                commands::teams::import_github(&cli.docs_dir, &org, dry_run)
            }
        },
        Commands::History { action } => match action {
            HistoryAction::Config { at, format } => {
                commands::history::config(&cli.docs_dir, at.as_deref(), &format)
            }
            HistoryAction::Team { id, format } => {
                commands::history::team(&cli.docs_dir, &id, &format)
            }
            HistoryAction::User { username, format } => {
                commands::history::user(&cli.docs_dir, &username, &format)
            }
        },
    }
}
