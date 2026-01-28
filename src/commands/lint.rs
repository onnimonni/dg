use crate::models::{validation, Graph, Record, ValidationError, ValidationOptions};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

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
        matches!(self.error, ValidationError::OrphanedRecord { .. })
    }
}

pub fn run(
    docs_dir: &str,
    files: Option<Vec<String>>,
    strict: bool,
    warn_orphans: bool,
    quiet: bool,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let opts = if strict {
        ValidationOptions {
            require_tags: true,
            require_content: true,
            check_orphans: warn_orphans,
            type_specific: true,
        }
    } else {
        ValidationOptions {
            check_orphans: warn_orphans,
            ..ValidationOptions::basic()
        }
    };

    let lint_errors = if let Some(file_list) = files {
        lint_files(&graph, docs_path, &file_list, &opts)
    } else {
        lint_all(&graph, &opts)
    };

    let (errors, warnings): (Vec<_>, Vec<_>) =
        lint_errors.into_iter().partition(|e| !e.is_warning());

    if !warnings.is_empty() && warn_orphans && !quiet {
        println!("{} {} warnings:\n", "WARN".yellow().bold(), warnings.len());
        for warn in &warnings {
            println!("  {} {}", "⚠".yellow(), warn);
        }
        println!();
    }

    if errors.is_empty() {
        if !quiet {
            println!("{} All records pass lint checks", "OK".green().bold());
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
    let decisions_dir = docs_path.join(".decisions");

    for file_path in files {
        let path = Path::new(file_path);

        // Skip non-decision files
        if !file_path.contains(".decisions") && !path.starts_with(&decisions_dir) {
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

        if !file_path.contains(".decisions") {
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
