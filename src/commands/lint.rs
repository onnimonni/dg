use crate::models::{validation, Graph, Record, ValidationError, ValidationOptions};
use crate::serve::config::DgConfig;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

/// Semantic validation errors that become warnings in non-strict mode
fn is_semantic_warning(error: &ValidationError) -> bool {
    matches!(
        error,
        ValidationError::SemanticMissingField { .. }
            | ValidationError::SemanticMissingLinkType { .. }
            | ValidationError::SemanticMissingSection { .. }
            | ValidationError::SemanticResolvedMissingSection { .. }
    )
}

/// Lint error with file path context
#[derive(Debug)]
pub struct LintError {
    pub file: String,
    pub error: ValidationError,
}

impl std::fmt::Display for LintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.error)
    }
}

impl LintError {
    fn new(file: &str, error: ValidationError) -> Self {
        Self {
            file: file.to_string(),
            error,
        }
    }

    fn is_warning(&self) -> bool {
        matches!(
            self.error,
            ValidationError::OrphanedRecord { .. } | ValidationError::PrincipleConflict { .. }
        )
    }

    /// Check if this is a semantic validation error (warning in non-strict mode)
    fn is_semantic(&self) -> bool {
        is_semantic_warning(&self.error)
    }
}

pub fn run(
    docs_dir: &str,
    files: Option<Vec<String>>,
    strict: bool,
    warn_orphans: bool,
    check_principles: bool,
    check_users: bool,
    quiet: bool,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Load config and validate if checking users
    let config = DgConfig::load(docs_path)?;
    let config_errors: Vec<LintError> = if check_users {
        config
            .validate()
            .into_iter()
            .map(|e| LintError::new("dg.toml", e))
            .collect()
    } else {
        vec![]
    };

    // Load user/team config if checking users
    let (users_config, teams_config) = if check_users {
        (Some(config.users_config()), Some(config.teams_config()))
    } else {
        (None, None)
    };

    // Get validation config for semantic rules
    let validation_config = config.validation_config().clone();
    let has_semantic_rules = validation_config.adr.is_some()
        || validation_config.incident.is_some()
        || validation_config.process.is_some()
        || validation_config.decision.is_some()
        || validation_config.strategy.is_some()
        || validation_config.policy.is_some()
        || validation_config.customer.is_some()
        || validation_config.opportunity.is_some()
        || validation_config.hiring.is_some()
        || validation_config.runbook.is_some()
        || validation_config.meeting.is_some()
        || validation_config.feedback.is_some()
        || validation_config.legal.is_some();

    let opts = if strict {
        ValidationOptions {
            require_tags: true,
            require_content: true,
            check_orphans: warn_orphans,
            type_specific: true,
            check_principle_conflicts: true,
            check_user_mentions: check_users,
            check_action_items: check_users,
            check_code_blocks: true,
            users_config: users_config.clone(),
            teams_config: teams_config.clone(),
            validation_config: Some(validation_config.clone()),
            check_semantic: has_semantic_rules,
        }
    } else {
        ValidationOptions {
            check_orphans: warn_orphans,
            check_principle_conflicts: check_principles,
            check_user_mentions: check_users,
            check_action_items: check_users,
            check_code_blocks: true,
            users_config,
            teams_config,
            validation_config: Some(validation_config),
            check_semantic: has_semantic_rules,
            ..ValidationOptions::basic()
        }
    };

    let mut lint_errors = if let Some(file_list) = files {
        lint_files(&graph, docs_path, &file_list, &opts)
    } else {
        lint_all(&graph, &opts)
    };

    // Add config validation errors
    lint_errors.extend(config_errors);

    // In non-strict mode, semantic errors are treated as warnings
    let (errors, warnings): (Vec<_>, Vec<_>) = if strict {
        // In strict mode, semantic errors are real errors
        lint_errors.into_iter().partition(|e| !e.is_warning())
    } else {
        // In non-strict mode, semantic errors become warnings
        lint_errors
            .into_iter()
            .partition(|e| !e.is_warning() && !e.is_semantic())
    };

    // Collect semantic warnings separately for display
    let semantic_warnings: Vec<_> = if !strict {
        warnings.iter().filter(|e| e.is_semantic()).collect()
    } else {
        vec![]
    };

    // Show warnings
    if !warnings.is_empty() && (warn_orphans || check_principles || has_semantic_rules) && !quiet {
        println!("{} {} warnings:\n", "WARN".yellow().bold(), warnings.len());
        for warn in &warnings {
            println!("  {} {}", "⚠".yellow(), warn);
        }
        println!();
    }

    if errors.is_empty() {
        if !quiet {
            if !semantic_warnings.is_empty() {
                println!(
                    "{} All records pass lint checks ({} semantic warnings, use --strict to enforce)",
                    "OK".green().bold(),
                    semantic_warnings.len()
                );
            } else {
                println!("{} All records pass lint checks", "OK".green().bold());
            }
        }
        Ok(())
    } else {
        println!("{} {} errors:\n", "ERROR".red().bold(), errors.len());
        for error in &errors {
            println!("  {} {}", "✗".red(), error);
        }
        println!();
        std::process::exit(1);
    }
}

fn lint_files(
    graph: &Graph,
    docs_path: &Path,
    files: &[String],
    opts: &ValidationOptions,
) -> Vec<LintError> {
    let mut errors = Vec::new();
    let decisions_dir = docs_path.join("decisions");

    for file_path in files {
        let path = Path::new(file_path);

        // Skip non-decision files
        if !file_path.contains("decisions") && !path.starts_with(&decisions_dir) {
            continue;
        }

        let record = match Record::parse(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{} Failed to parse {}: {}", "ERROR".red(), file_path, e);
                continue;
            }
        };

        let file_str = record.path.display().to_string();
        for error in validation::validate_record(&record, graph, opts) {
            errors.push(LintError::new(&file_str, error));
        }
    }

    errors
}

fn lint_all(graph: &Graph, opts: &ValidationOptions) -> Vec<LintError> {
    graph
        .all_records()
        .flat_map(|record| {
            let file_str = record.path.display().to_string();
            validation::validate_record(record, graph, opts)
                .into_iter()
                .map(move |error| LintError::new(&file_str, error))
        })
        .collect()
}

/// Validate that new files being committed have proper links
#[allow(dead_code)]
pub fn lint_new_files(_graph: &Graph, new_files: &[String], _docs_path: &Path) -> Vec<LintError> {
    let mut errors = Vec::new();

    for file_path in new_files {
        let path = Path::new(file_path);

        if !file_path.contains("decisions") {
            continue;
        }

        let record = match Record::parse(path) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let has_links = !record.frontmatter.links.all_links().is_empty();

        if !has_links {
            errors.push(LintError::new(
                file_path,
                ValidationError::MissingRequiredLink {
                    id: record.id().to_string(),
                    message: "new record should link to existing records".to_string(),
                },
            ));
        }
    }

    errors
}
