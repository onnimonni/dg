//! Shared validation logic for records and graphs

use super::{Graph, Record, RecordType, TeamsConfig, UsersConfig};
use regex::Regex;
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
    EmptySection {
        id: String,
        heading: String,
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
    InvalidUserMention {
        id: String,
        username: String,
        line: usize,
    },
    InvalidActionItemOwner {
        id: String,
        owner: String,
        line: usize,
    },
    DraftRecord {
        id: String,
    },
    CodeBlockMissingLanguage {
        id: String,
        line: usize,
    },
    // Config validation errors
    TeamCircularParent {
        team: String,
        cycle: Vec<String>,
    },
    TeamMissingParent {
        team: String,
        parent: String,
    },
    TeamLeadNotUser {
        team: String,
        lead: String,
    },
    TeamLeadNotMember {
        team: String,
        lead: String,
    },
    UserInNonexistentTeam {
        user: String,
        team: String,
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
            ValidationError::EmptySection { id, heading } => {
                write!(f, "{}: section '{}' has no content", id, heading)
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
                    "{}: conflicts with core record {}: {}",
                    id, conflicts_with, message
                )
            }
            ValidationError::InvalidUserMention { id, username, line } => {
                write!(
                    f,
                    "{}: line {}: unknown user mention @{}",
                    id, line, username
                )
            }
            ValidationError::InvalidActionItemOwner { id, owner, line } => {
                write!(
                    f,
                    "{}: line {}: unknown action item owner '{}'",
                    id, line, owner
                )
            }
            ValidationError::DraftRecord { id } => {
                write!(f, "{}: draft record (use 'dg finalize' before merging)", id)
            }
            ValidationError::CodeBlockMissingLanguage { id, line } => {
                write!(
                    f,
                    "{}: line {}: code block missing language identifier (use ```bash, ```yaml, etc.)",
                    id, line
                )
            }
            // Config validation errors
            ValidationError::TeamCircularParent { team, cycle } => {
                write!(
                    f,
                    "dg.toml: team '{}' has circular parent: {}",
                    team,
                    cycle.join(" -> ")
                )
            }
            ValidationError::TeamMissingParent { team, parent } => {
                write!(
                    f,
                    "dg.toml: team '{}' references non-existent parent '{}'",
                    team, parent
                )
            }
            ValidationError::TeamLeadNotUser { team, lead } => {
                write!(
                    f,
                    "dg.toml: team '{}' lead '{}' is not defined in [users]",
                    team, lead
                )
            }
            ValidationError::TeamLeadNotMember { team, lead } => {
                write!(
                    f,
                    "dg.toml: team '{}' lead '{}' is not a member of that team",
                    team, lead
                )
            }
            ValidationError::UserInNonexistentTeam { user, team } => {
                write!(
                    f,
                    "dg.toml: user '{}' belongs to non-existent team '{}'",
                    user, team
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
    /// Check for valid @username mentions
    pub check_user_mentions: bool,
    /// Check for valid action item owners
    pub check_action_items: bool,
    /// Check for code blocks without language identifiers
    pub check_code_blocks: bool,
    /// Users config for validation
    pub users_config: Option<UsersConfig>,
    /// Teams config for validation (teams can be action item owners)
    pub teams_config: Option<TeamsConfig>,
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
            check_user_mentions: false,
            check_action_items: false,
            check_code_blocks: true,
            users_config: None,
            teams_config: None,
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

    // Check for draft records (should be finalized before merging)
    if crate::commands::new::is_draft_id(&id) {
        errors.push(ValidationError::DraftRecord { id: id.clone() });
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

    // Check for conflicts with core records
    if opts.check_principle_conflicts {
        errors.extend(check_principle_conflicts(record, graph));
    }

    // Check for valid @username mentions
    if opts.check_user_mentions {
        if let (Some(users), Some(teams)) = (&opts.users_config, &opts.teams_config) {
            errors.extend(check_user_mentions(record, users, teams));
        }
    }

    // Check for valid action item owners
    if opts.check_action_items {
        if let (Some(users), Some(teams)) = (&opts.users_config, &opts.teams_config) {
            errors.extend(check_action_items(record, users, teams));
        }
    }

    // Check for code blocks without language identifiers
    if opts.check_code_blocks {
        errors.extend(check_code_blocks(record));
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
    let mut errors = Vec::new();
    let id = record.id().to_string();

    // Check overall content
    let content_lines: Vec<&str> = record
        .content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !l.starts_with('#'))
        .filter(|l| !l.starts_with("<!--"))
        .filter(|l| !l.starts_with("**") || !l.ends_with("**:"))
        .collect();

    if content_lines.len() < 3 {
        errors.push(ValidationError::EmptyContent { id: id.clone() });
    }

    // Check for empty sections (headings without content)
    errors.extend(check_empty_sections(record));

    errors
}

/// Check for headings that have no content below them
fn check_empty_sections(record: &Record) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id().to_string();
    let lines: Vec<&str> = record.content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a heading (starts with #)
        if line.starts_with('#') {
            let heading_level = line.chars().take_while(|c| *c == '#').count();
            let heading = line.trim_start_matches('#').trim().to_string();

            // Look for content between this heading and the next heading of same or higher level
            let mut has_content = false;
            let mut has_subheading = false;
            let mut j = i + 1;

            while j < lines.len() {
                let next_line = lines[j].trim();

                // Check if this is a heading
                if next_line.starts_with('#') {
                    let next_level = next_line.chars().take_while(|c| *c == '#').count();
                    // Stop at same level or higher (smaller number = higher level)
                    if next_level <= heading_level {
                        break;
                    }
                    // This is a subheading - parent section is not empty
                    has_subheading = true;
                    break;
                }

                // Check if this line has meaningful content
                // Skip: empty lines, HTML comments, placeholder text
                if !next_line.is_empty()
                    && !next_line.starts_with("<!--")
                    && !next_line.ends_with("-->")
                    && !is_placeholder_text(next_line)
                {
                    has_content = true;
                    break;
                }

                j += 1;
            }

            // Only report error if heading has no content AND no subheadings
            if !has_content && !has_subheading && !heading.is_empty() {
                errors.push(ValidationError::EmptySection {
                    id: id.clone(),
                    heading,
                });
            }
        }

        i += 1;
    }

    errors
}

/// Check if text is placeholder/template text that shouldn't count as content
fn is_placeholder_text(text: &str) -> bool {
    let lower = text.to_lowercase();
    // Common placeholder patterns
    lower.contains("[todo")
        || lower.contains("[tbd")
        || lower.contains("[placeholder")
        || lower.contains("[insert")
        || lower.contains("[add ")
        || lower.contains("[describe")
        || lower.contains("[explain")
        || lower.starts_with("todo:")
        || lower.starts_with("tbd:")
        || (lower.starts_with('[') && lower.ends_with(']'))
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

/// Check for conflicts with core records
fn check_principle_conflicts(record: &Record, graph: &Graph) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id();

    // Get all core records
    let core: Vec<_> = graph.core_records();

    // Check if this record has conflicts_with any core record
    for conflict_id in &record.frontmatter.links.conflicts_with {
        if let Some(conflict_record) = graph.get(conflict_id) {
            if conflict_record.frontmatter.core {
                errors.push(ValidationError::PrincipleConflict {
                    id: id.to_string(),
                    conflicts_with: conflict_id.clone(),
                    message: format!(
                        "This record conflicts with '{}' which is a core principle",
                        conflict_record.title()
                    ),
                });
            }
        }
    }

    // Check if any core record has conflicts_with this record
    for core_record in &core {
        if core_record
            .frontmatter
            .links
            .conflicts_with
            .contains(&id.to_string())
        {
            errors.push(ValidationError::PrincipleConflict {
                id: id.to_string(),
                conflicts_with: core_record.id().to_string(),
                message: format!(
                    "Foundational record '{}' explicitly conflicts with this record",
                    core_record.title()
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

/// Check for invalid @username mentions in content
pub fn check_user_mentions(
    record: &Record,
    users_config: &UsersConfig,
    teams_config: &TeamsConfig,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id().to_string();

    // Regex to match @username (letters, numbers, underscores, hyphens)
    let mention_re = Regex::new(r"@([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();

    for (line_num, line) in record.content.lines().enumerate() {
        for cap in mention_re.captures_iter(line) {
            let username = &cap[1];
            // Check if it's a valid user OR team
            if !users_config.exists(username) && !teams_config.exists(username) {
                errors.push(ValidationError::InvalidUserMention {
                    id: id.clone(),
                    username: username.to_string(),
                    line: line_num + 1,
                });
            }
        }
    }

    errors
}

/// Check for invalid action item owners in INC/RUN records
pub fn check_action_items(
    record: &Record,
    users_config: &UsersConfig,
    teams_config: &TeamsConfig,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id().to_string();

    // Only check INC and RUN records
    let record_type = record.record_type();
    if *record_type != RecordType::Incident && *record_type != RecordType::Runbook {
        return errors;
    }

    let mention_re = Regex::new(r"@([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();

    // Track if we're in an action items section
    let mut in_action_items = false;

    for (line_num, line) in record.content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for action items heading
        if trimmed.starts_with('#') && trimmed.to_lowercase().contains("action") {
            in_action_items = true;
            continue;
        }

        // Exit action items section on next heading
        if trimmed.starts_with('#') && in_action_items {
            in_action_items = false;
            continue;
        }

        if !in_action_items {
            continue;
        }

        // Parse table rows: | Action | Owner | Due | Status |
        if trimmed.starts_with('|') && !trimmed.contains("---") {
            // Split by | and look for owner column (usually 2nd after Action)
            let cells: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
            // cells[0] is empty (before first |), cells[1] is Action, cells[2] is Owner
            if cells.len() > 2 {
                let owner_cell = cells[2];
                for cap in mention_re.captures_iter(owner_cell) {
                    let username = &cap[1];
                    if !users_config.exists(username) && !teams_config.exists(username) {
                        errors.push(ValidationError::InvalidActionItemOwner {
                            id: id.clone(),
                            owner: username.to_string(),
                            line: line_num + 1,
                        });
                    }
                }
            }
        }

        // Parse bullet list items: - [ ] Task @owner or - [x] Task @owner
        if trimmed.starts_with("- [ ]")
            || trimmed.starts_with("- [x]")
            || trimmed.starts_with("- [X]")
        {
            for cap in mention_re.captures_iter(line) {
                let username = &cap[1];
                if !users_config.exists(username) && !teams_config.exists(username) {
                    errors.push(ValidationError::InvalidActionItemOwner {
                        id: id.clone(),
                        owner: username.to_string(),
                        line: line_num + 1,
                    });
                }
            }
        }
    }

    errors
}

/// Check for code blocks without language identifiers
pub fn check_code_blocks(record: &Record) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let id = record.id().to_string();

    // Track if we're inside a code block
    let mut in_code_block = false;

    for (line_num, line) in record.content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for code block start
        if trimmed.starts_with("```") {
            if !in_code_block {
                // Starting a code block - check if it has a language
                let after_backticks = trimmed.trim_start_matches('`');

                // Must have a language identifier (text, plaintext, etc. are allowed)
                if after_backticks.is_empty() {
                    errors.push(ValidationError::CodeBlockMissingLanguage {
                        id: id.clone(),
                        line: line_num + 1,
                    });
                }
                in_code_block = true;
            } else {
                // Ending a code block
                in_code_block = false;
            }
        }
    }

    errors
}

/// Validate dg.toml config for impossible team/user graphs
pub fn validate_config(
    users_config: &UsersConfig,
    teams_config: &TeamsConfig,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Check for teams referencing non-existent parents
    for (team_id, team) in &teams_config.teams {
        if let Some(ref parent) = team.parent {
            if !teams_config.exists(parent) {
                errors.push(ValidationError::TeamMissingParent {
                    team: team_id.clone(),
                    parent: parent.clone(),
                });
            }
        }
    }

    // Check for circular parent references
    for team_id in teams_config.teams.keys() {
        if let Some(cycle) = detect_team_cycle(team_id, teams_config) {
            errors.push(ValidationError::TeamCircularParent {
                team: team_id.clone(),
                cycle,
            });
        }
    }

    // Check that team leads exist as users
    for (team_id, team) in &teams_config.teams {
        if let Some(ref lead) = team.lead {
            if !users_config.exists(lead) {
                errors.push(ValidationError::TeamLeadNotUser {
                    team: team_id.clone(),
                    lead: lead.clone(),
                });
            }
        }
    }

    // Check that team leads are members of the team they lead
    for (team_id, team) in &teams_config.teams {
        if let Some(ref lead) = team.lead {
            if let Some(user) = users_config.get(lead) {
                if !user.teams.contains(team_id) {
                    errors.push(ValidationError::TeamLeadNotMember {
                        team: team_id.clone(),
                        lead: lead.clone(),
                    });
                }
            }
        }
    }

    // Check that users don't belong to non-existent teams
    for (user_id, user) in &users_config.users {
        for team in &user.teams {
            if !teams_config.exists(team) {
                errors.push(ValidationError::UserInNonexistentTeam {
                    user: user_id.clone(),
                    team: team.clone(),
                });
            }
        }
    }

    errors
}

/// Detect cycles in team parent hierarchy
fn detect_team_cycle(start_team: &str, teams_config: &TeamsConfig) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut current = start_team;

    while let Some(team) = teams_config.get(current) {
        if visited.contains(current) {
            // Found a cycle - reconstruct the cycle path
            path.push(current.to_string());
            return Some(path);
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        match &team.parent {
            Some(parent) => current = parent,
            None => return None, // No cycle, reached root
        }
    }

    None // Parent doesn't exist (separate error)
}

#[cfg(test)]
mod mention_tests {
    use super::*;
    use crate::models::record::{Frontmatter, Links, Status};
    use crate::models::teams::Team;
    use crate::models::users::User;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    fn make_test_record(content: &str) -> Record {
        Record {
            path: std::path::PathBuf::from("test.md"),
            frontmatter: Frontmatter {
                r#type: RecordType::Decision,
                id: "DEC-001".to_string(),
                title: "Test Record".to_string(),
                status: Status::Proposed,
                created: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                updated: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                authors: vec![],
                tags: vec![],
                links: Links::default(),
                core: false,
                extra: HashMap::new(),
            },
            content: content.to_string(),
        }
    }

    #[test]
    fn test_valid_mention() {
        let mut users = UsersConfig::default();
        users.users.insert("richard".to_string(), User::default());
        let teams = TeamsConfig::default();

        let record = make_test_record("Hello @richard, how are you?");
        let errors = check_user_mentions(&record, &users, &teams);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_invalid_mention() {
        let users = UsersConfig::default();
        let teams = TeamsConfig::default();

        let record = make_test_record("Hello @unknown, how are you?");
        let errors = check_user_mentions(&record, &users, &teams);
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            ValidationError::InvalidUserMention { username, .. } => {
                assert_eq!(username, "unknown");
            }
            _ => panic!("Expected InvalidUserMention"),
        }
    }

    #[test]
    fn test_team_mention_valid() {
        let users = UsersConfig::default();
        let mut teams = TeamsConfig::default();
        teams.teams.insert(
            "platform".to_string(),
            Team {
                name: "Platform".to_string(),
                ..Default::default()
            },
        );

        let record = make_test_record("Assigned to @platform team");
        let errors = check_user_mentions(&record, &users, &teams);
        assert!(errors.is_empty());
    }

    fn make_incident_record(content: &str) -> Record {
        Record {
            path: std::path::PathBuf::from("test.md"),
            frontmatter: Frontmatter {
                r#type: RecordType::Incident,
                id: "INC-001".to_string(),
                title: "Test Incident".to_string(),
                status: Status::Open,
                created: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                updated: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                authors: vec![],
                tags: vec![],
                links: Links::default(),
                core: false,
                extra: HashMap::new(),
            },
            content: content.to_string(),
        }
    }

    #[test]
    fn test_action_item_table_valid() {
        let mut users = UsersConfig::default();
        users.users.insert("richard".to_string(), User::default());
        let teams = TeamsConfig::default();

        let content = "## Action Items\n| Action | Owner | Due | Status |\n|--------|-------|-----|--------|\n| Fix bug | @richard | 2024-01-15 | Open |";
        let record = make_incident_record(content);
        let errors = check_action_items(&record, &users, &teams);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_action_item_table_invalid() {
        let users = UsersConfig::default();
        let teams = TeamsConfig::default();

        let content = "## Action Items\n| Action | Owner | Due | Status |\n|--------|-------|-----|--------|\n| Fix bug | @unknown | 2024-01-15 | Open |";
        let record = make_incident_record(content);
        let errors = check_action_items(&record, &users, &teams);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_action_item_bullet_valid() {
        let mut users = UsersConfig::default();
        users.users.insert("gilfoyle".to_string(), User::default());
        let teams = TeamsConfig::default();

        let content = "## Action Items\n- [ ] Review PR @gilfoyle\n- [x] Deploy fix @gilfoyle";
        let record = make_incident_record(content);
        let errors = check_action_items(&record, &users, &teams);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_action_item_bullet_invalid() {
        let users = UsersConfig::default();
        let teams = TeamsConfig::default();

        let content = "## Action Items\n- [ ] Review PR @nobody";
        let record = make_incident_record(content);
        let errors = check_action_items(&record, &users, &teams);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_action_item_team_owner() {
        let users = UsersConfig::default();
        let mut teams = TeamsConfig::default();
        teams.teams.insert(
            "platform".to_string(),
            Team {
                name: "Platform".to_string(),
                ..Default::default()
            },
        );

        let content = "## Action Items\n- [ ] Deploy to prod @platform";
        let record = make_incident_record(content);
        let errors = check_action_items(&record, &users, &teams);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_action_item_decision_ignored() {
        let users = UsersConfig::default();
        let teams = TeamsConfig::default();

        // Decision records should not check action items
        let content = "## Action Items\n- [ ] Something @unknown";
        let record = make_test_record(content); // DEC type
        let errors = check_action_items(&record, &users, &teams);
        assert!(errors.is_empty());
    }
}

#[cfg(test)]
mod config_validation_tests {
    use super::*;
    use crate::models::teams::Team;
    use crate::models::users::User;

    #[test]
    fn test_valid_config() {
        let mut users = UsersConfig::default();
        users.users.insert(
            "richard".to_string(),
            User {
                teams: vec!["engineering".to_string()],
                ..Default::default()
            },
        );

        let mut teams = TeamsConfig::default();
        teams.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                lead: Some("richard".to_string()),
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_circular_parent() {
        let users = UsersConfig::default();
        let mut teams = TeamsConfig::default();

        teams.teams.insert(
            "team-a".to_string(),
            Team {
                name: "Team A".to_string(),
                parent: Some("team-b".to_string()),
                ..Default::default()
            },
        );
        teams.teams.insert(
            "team-b".to_string(),
            Team {
                name: "Team B".to_string(),
                parent: Some("team-a".to_string()),
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        let cycle_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::TeamCircularParent { .. }))
            .collect();
        assert_eq!(cycle_errors.len(), 2);
    }

    #[test]
    fn test_missing_parent() {
        let users = UsersConfig::default();
        let mut teams = TeamsConfig::default();

        teams.teams.insert(
            "child".to_string(),
            Team {
                name: "Child".to_string(),
                parent: Some("nonexistent".to_string()),
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            &errors[0],
            ValidationError::TeamMissingParent { team, parent }
            if team == "child" && parent == "nonexistent"
        ));
    }

    #[test]
    fn test_lead_not_user() {
        let users = UsersConfig::default();
        let mut teams = TeamsConfig::default();

        teams.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                lead: Some("nobody".to_string()),
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        assert!(errors.iter().any(
            |e| matches!(e, ValidationError::TeamLeadNotUser { lead, .. } if lead == "nobody")
        ));
    }

    #[test]
    fn test_lead_not_member() {
        let mut users = UsersConfig::default();
        users.users.insert("richard".to_string(), User::default()); // Not in engineering team

        let mut teams = TeamsConfig::default();
        teams.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                lead: Some("richard".to_string()),
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        assert!(errors.iter().any(
            |e| matches!(e, ValidationError::TeamLeadNotMember { team, lead } if team == "engineering" && lead == "richard")
        ));
    }

    #[test]
    fn test_user_in_nonexistent_team() {
        let mut users = UsersConfig::default();
        users.users.insert(
            "richard".to_string(),
            User {
                teams: vec!["nonexistent".to_string()],
                ..Default::default()
            },
        );

        let teams = TeamsConfig::default();

        let errors = validate_config(&users, &teams);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            &errors[0],
            ValidationError::UserInNonexistentTeam { user, team }
            if user == "richard" && team == "nonexistent"
        ));
    }

    #[test]
    fn test_secondary_team_no_lead_is_valid() {
        let mut users = UsersConfig::default();
        users.users.insert(
            "richard".to_string(),
            User {
                teams: vec!["founders".to_string()],
                ..Default::default()
            },
        );

        let mut teams = TeamsConfig::default();
        teams.teams.insert(
            "founders".to_string(),
            Team {
                name: "Founders".to_string(),
                // No lead - this is a secondary/hashtag team
                ..Default::default()
            },
        );

        let errors = validate_config(&users, &teams);
        assert!(
            errors.is_empty(),
            "Secondary teams without leads should be valid"
        );
    }
}
