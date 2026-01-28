//! Shared validation logic for records and graphs

use super::{Graph, Record, RecordType};
use std::collections::HashSet;

/// Core validation error types (without file paths)
#[derive(Debug, Clone)]
pub enum ValidationError {
    BrokenLink {
        from: String,
        link_type: String,
        to: String,
    },
    MissingField {
        id: String,
        field: String,
    },
    MissingInverseLink {
        from: String,
        to: String,
        link_type: String,
        expected_inverse: String,
    },
    OrphanedRecord {
        id: String,
    },
    EmptyContent {
        id: String,
    },
    MissingRequiredLink {
        id: String,
        message: String,
    },
    PrincipleConflict {
        id: String,
        conflicts_with: String,
        message: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::BrokenLink {
                from,
                link_type,
                to,
            } => write!(f, "{}: broken {} link to '{}'", from, link_type, to),
            ValidationError::MissingField { id, field } => {
                write!(f, "{}: missing required field '{}'", id, field)
            }
            ValidationError::MissingInverseLink {
                from,
                to,
                link_type,
                expected_inverse,
            } => write!(
                f,
                "{}: {} link to {} but {} doesn't have {} back",
                from, link_type, to, to, expected_inverse
            ),
            ValidationError::OrphanedRecord { id } => {
                write!(f, "{}: orphaned record (no links)", id)
            }
            ValidationError::EmptyContent { id } => {
                write!(f, "{}: record has no meaningful content", id)
            }
            ValidationError::MissingRequiredLink { id, message } => {
                write!(f, "{}: {}", id, message)
            }
            ValidationError::PrincipleConflict {
                id,
                conflicts_with,
                message,
            } => {
                write!(
                    f,
                    "{}: conflicts with foundational record {}: {}",
                    id, conflicts_with, message
                )
            }
        }
    }
}

/// Validation options
#[derive(Debug, Default)]
pub struct ValidationOptions {
    /// Check for missing tags
    pub require_tags: bool,
    /// Check for meaningful content
    pub require_content: bool,
    /// Check for orphaned records
    pub check_orphans: bool,
    /// Type-specific validation
    pub type_specific: bool,
    /// Check for principle conflicts
    pub check_principle_conflicts: bool,
}

impl ValidationOptions {
    pub fn basic() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            require_tags: true,
            require_content: true,
            check_orphans: false,
            type_specific: true,
            check_principle_conflicts: true,
        }
    }
}

/// Validate a single record against the graph
pub fn validate_record(
    record: &Record,
    graph: &Graph,
    opts: &ValidationOptions,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id().to_string();
    let valid_ids: HashSet<_> = graph.records.keys().cloned().collect();

    // Check for broken links
    errors.extend(check_broken_links(record, &valid_ids));

    // Check for missing title
    if record.title().is_empty() {
        errors.push(ValidationError::MissingField {
            id: id.clone(),
            field: "title".to_string(),
        });
    }

    // Check for missing tags (strict)
    if opts.require_tags && record.frontmatter.tags.is_empty() {
        errors.push(ValidationError::MissingField {
            id: id.clone(),
            field: "tags".to_string(),
        });
    }

    // Check for orphaned records
    if opts.check_orphans {
        let has_outgoing = !record.frontmatter.links.all_links().is_empty();
        let has_incoming = !graph.incoming_edges(&id).is_empty();

        if !has_outgoing && !has_incoming {
            errors.push(ValidationError::OrphanedRecord { id: id.clone() });
        }
    }

    // Check supersedes has inverse
    errors.extend(check_supersedes_inverse(record, graph));

    // Check for meaningful content (strict)
    if opts.require_content {
        errors.extend(check_content(record));
    }

    // Type-specific checks
    if opts.type_specific {
        errors.extend(check_type_specific(record));
    }

    // Check for conflicts with foundational records
    if opts.check_principle_conflicts {
        errors.extend(check_principle_conflicts(record, graph));
    }

    errors
}

/// Check for broken links in a record
pub fn check_broken_links(record: &Record, valid_ids: &HashSet<String>) -> Vec<ValidationError> {
    let id = record.id().to_string();
    record
        .frontmatter
        .links
        .all_links()
        .into_iter()
        .filter(|(_, target)| !valid_ids.contains(*target))
        .map(|(link_type, target)| ValidationError::BrokenLink {
            from: id.clone(),
            link_type: link_type.to_string(),
            to: target.to_string(),
        })
        .collect()
}

/// Check that supersedes links have inverse superseded_by
pub fn check_supersedes_inverse(record: &Record, graph: &Graph) -> Vec<ValidationError> {
    let id = record.id().to_string();
    record
        .frontmatter
        .links
        .supersedes
        .iter()
        .filter_map(|target| {
            graph.get(target).and_then(|target_record| {
                if !target_record.frontmatter.links.superseded_by.contains(&id) {
                    Some(ValidationError::MissingInverseLink {
                        from: id.clone(),
                        to: target.clone(),
                        link_type: "supersedes".to_string(),
                        expected_inverse: "superseded_by".to_string(),
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

/// Check for meaningful content
fn check_content(record: &Record) -> Vec<ValidationError> {
    let content_lines: Vec<&str> = record
        .content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !l.starts_with('#'))
        .filter(|l| !l.starts_with("<!--"))
        .filter(|l| !l.starts_with("**") || !l.ends_with("**:"))
        .collect();

    if content_lines.len() < 3 {
        vec![ValidationError::EmptyContent {
            id: record.id().to_string(),
        }]
    } else {
        vec![]
    }
}

/// Type-specific validation
fn check_type_specific(record: &Record) -> Vec<ValidationError> {
    match record.record_type() {
        RecordType::Adr => {
            // ADRs should typically implement or depend_on something
            let has_context_link = !record.frontmatter.links.implements.is_empty()
                || !record.frontmatter.links.depends_on.is_empty()
                || !record.frontmatter.links.relates_to.is_empty();

            if !has_context_link {
                vec![ValidationError::MissingRequiredLink {
                    id: record.id().to_string(),
                    message: "ADR should link to what it implements, depends_on, or relates_to"
                        .to_string(),
                }]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

/// Check for conflicts with foundational records
fn check_principle_conflicts(record: &Record, graph: &Graph) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id();

    // Get all foundational records
    let foundational: Vec<_> = graph.foundational_records();

    // Check if this record has conflicts_with any foundational record
    for conflict_id in &record.frontmatter.links.conflicts_with {
        if let Some(conflict_record) = graph.get(conflict_id) {
            if conflict_record.frontmatter.foundational {
                errors.push(ValidationError::PrincipleConflict {
                    id: id.to_string(),
                    conflicts_with: conflict_id.clone(),
                    message: format!(
                        "This record conflicts with '{}' which is a foundational principle",
                        conflict_record.title()
                    ),
                });
            }
        }
    }

    // Check if any foundational record has conflicts_with this record
    for foundational_record in &foundational {
        if foundational_record
            .frontmatter
            .links
            .conflicts_with
            .contains(&id.to_string())
        {
            errors.push(ValidationError::PrincipleConflict {
                id: id.to_string(),
                conflicts_with: foundational_record.id().to_string(),
                message: format!(
                    "Foundational record '{}' explicitly conflicts with this record",
                    foundational_record.title()
                ),
            });
        }
    }

    errors
}

/// Validate all records in a graph
pub fn validate_graph(graph: &Graph, opts: &ValidationOptions) -> Vec<ValidationError> {
    graph
        .all_records()
        .flat_map(|record| validate_record(record, graph, opts))
        .collect()
}
