use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RecordType {
    Decision,
    Strategy,
    Policy,
    Customer,
    Opportunity,
    Process,
    Hiring,
    Adr,
    Incident,
    Runbook,
    Meeting,
    Feedback,
    Legal,
}

impl RecordType {
    pub fn prefix(&self) -> &'static str {
        match self {
            RecordType::Decision => "DEC",
            RecordType::Strategy => "STR",
            RecordType::Policy => "POL",
            RecordType::Customer => "CUS",
            RecordType::Opportunity => "OPP",
            RecordType::Process => "PRC",
            RecordType::Hiring => "HIR",
            RecordType::Adr => "ADR",
            RecordType::Incident => "INC",
            RecordType::Runbook => "RUN",
            RecordType::Meeting => "MTG",
            RecordType::Feedback => "FBK",
            RecordType::Legal => "LEG",
        }
    }

    pub fn from_prefix(prefix: &str) -> Option<RecordType> {
        match prefix.to_uppercase().as_str() {
            "DEC" => Some(RecordType::Decision),
            "STR" => Some(RecordType::Strategy),
            "POL" => Some(RecordType::Policy),
            "CUS" => Some(RecordType::Customer),
            "OPP" => Some(RecordType::Opportunity),
            "PRC" => Some(RecordType::Process),
            "HIR" => Some(RecordType::Hiring),
            "ADR" => Some(RecordType::Adr),
            "INC" => Some(RecordType::Incident),
            "RUN" => Some(RecordType::Runbook),
            "MTG" => Some(RecordType::Meeting),
            "FBK" => Some(RecordType::Feedback),
            "LEG" => Some(RecordType::Legal),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<RecordType> {
        match s.to_lowercase().as_str() {
            "decision" => Some(RecordType::Decision),
            "strategy" => Some(RecordType::Strategy),
            "policy" => Some(RecordType::Policy),
            "customer" => Some(RecordType::Customer),
            "opportunity" => Some(RecordType::Opportunity),
            "process" => Some(RecordType::Process),
            "hiring" => Some(RecordType::Hiring),
            "adr" => Some(RecordType::Adr),
            "incident" => Some(RecordType::Incident),
            "runbook" => Some(RecordType::Runbook),
            "meeting" => Some(RecordType::Meeting),
            "feedback" => Some(RecordType::Feedback),
            "legal" => Some(RecordType::Legal),
            _ => RecordType::from_prefix(s),
        }
    }

    pub fn template_name(&self) -> &'static str {
        match self {
            RecordType::Decision => "decision",
            RecordType::Strategy => "strategy",
            RecordType::Policy => "policy",
            RecordType::Customer => "customer",
            RecordType::Opportunity => "opportunity",
            RecordType::Process => "process",
            RecordType::Hiring => "hiring",
            RecordType::Adr => "adr",
            RecordType::Incident => "incident",
            RecordType::Runbook => "runbook",
            RecordType::Meeting => "meeting",
            RecordType::Feedback => "feedback",
            RecordType::Legal => "legal",
        }
    }

    /// Human-readable display name for the record type
    pub fn display_name(&self) -> &'static str {
        match self {
            RecordType::Decision => "Decision",
            RecordType::Strategy => "Strategy",
            RecordType::Policy => "Policy",
            RecordType::Customer => "Customer",
            RecordType::Opportunity => "Opportunity",
            RecordType::Process => "Process",
            RecordType::Hiring => "Hiring",
            RecordType::Adr => "Architecture",
            RecordType::Incident => "Incident",
            RecordType::Runbook => "Runbook",
            RecordType::Meeting => "Meeting",
            RecordType::Feedback => "Feedback",
            RecordType::Legal => "Legal",
        }
    }
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.prefix())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Proposed,
    Accepted,
    Rejected,
    Deprecated,
    Superseded,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Status {
    pub fn from_str(s: &str) -> Option<Status> {
        match s.to_lowercase().as_str() {
            "draft" => Some(Status::Draft),
            "proposed" => Some(Status::Proposed),
            "accepted" => Some(Status::Accepted),
            "rejected" => Some(Status::Rejected),
            "deprecated" => Some(Status::Deprecated),
            "superseded" => Some(Status::Superseded),
            "active" => Some(Status::Active),
            "open" => Some(Status::Open),
            "closed" => Some(Status::Closed),
            "resolved" => Some(Status::Resolved),
            "cancelled" => Some(Status::Cancelled),
            _ => None,
        }
    }

    /// Returns true if this status indicates the record is no longer active/valid
    /// (deprecated, rejected, superseded, or cancelled)
    pub fn is_inactive(&self) -> bool {
        matches!(
            self,
            Status::Deprecated | Status::Rejected | Status::Superseded | Status::Cancelled
        )
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Draft => "draft",
            Status::Proposed => "proposed",
            Status::Accepted => "accepted",
            Status::Rejected => "rejected",
            Status::Deprecated => "deprecated",
            Status::Superseded => "superseded",
            Status::Active => "active",
            Status::Open => "open",
            Status::Closed => "closed",
            Status::Resolved => "resolved",
            Status::Cancelled => "cancelled",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Links {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supersedes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub superseded_by: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enables: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicts_with: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refines: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implements: Vec<String>,
}

impl Links {
    pub fn all_links(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        for id in &self.supersedes {
            result.push(("supersedes", id.as_str()));
        }
        for id in &self.superseded_by {
            result.push(("superseded_by", id.as_str()));
        }
        for id in &self.depends_on {
            result.push(("depends_on", id.as_str()));
        }
        for id in &self.enables {
            result.push(("enables", id.as_str()));
        }
        for id in &self.relates_to {
            result.push(("relates_to", id.as_str()));
        }
        for id in &self.conflicts_with {
            result.push(("conflicts_with", id.as_str()));
        }
        for id in &self.refines {
            result.push(("refines", id.as_str()));
        }
        for id in &self.implements {
            result.push(("implements", id.as_str()));
        }
        result
    }

    pub fn add_link(&mut self, link_type: &str, target: &str) -> Result<()> {
        let vec = self.get_vec_mut(link_type)?;
        if !vec.contains(&target.to_string()) {
            vec.push(target.to_string());
        }
        Ok(())
    }

    pub fn remove_link(&mut self, link_type: &str, target: &str) -> Result<bool> {
        let vec = self.get_vec_mut(link_type)?;
        if let Some(pos) = vec.iter().position(|x| x == target) {
            vec.remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_vec_mut(&mut self, link_type: &str) -> Result<&mut Vec<String>> {
        match link_type {
            "supersedes" => Ok(&mut self.supersedes),
            "superseded_by" => Ok(&mut self.superseded_by),
            "depends_on" => Ok(&mut self.depends_on),
            "enables" => Ok(&mut self.enables),
            "relates_to" => Ok(&mut self.relates_to),
            "conflicts_with" => Ok(&mut self.conflicts_with),
            "refines" => Ok(&mut self.refines),
            "implements" => Ok(&mut self.implements),
            _ => Err(anyhow!("Unknown link type: {}", link_type)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub r#type: RecordType,
    pub id: String,
    pub title: String,
    pub status: Status,
    pub created: NaiveDate,
    pub updated: NaiveDate,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub links: Links,
    /// Marks this record as a core principle that other records depend on
    #[serde(default)]
    pub core: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub path: PathBuf,
}

impl Record {
    pub fn parse(path: &Path) -> Result<Record> {
        let content = fs::read_to_string(path)?;
        Self::parse_content(&content, path.to_path_buf())
    }

    pub fn parse_content(content: &str, path: PathBuf) -> Result<Record> {
        // Normalize line endings (Windows \r\n -> Unix \n)
        let content = content.replace("\r\n", "\n");
        let content = content.trim_start(); // Allow leading whitespace

        // Validate frontmatter start
        if !content.starts_with("---") {
            return Err(anyhow!("Missing frontmatter: file must start with '---'"));
        }

        // Find the closing delimiter
        // Skip the opening "---" and find the next "---" on its own line
        let after_opening = &content[3..];
        let after_opening = after_opening.strip_prefix('\n').unwrap_or(after_opening);

        // Find closing "---" that starts a new line
        let closing_pos = after_opening
            .find("\n---")
            .ok_or_else(|| anyhow!("Unterminated frontmatter: missing closing '---'"))?;

        let yaml_str = &after_opening[..closing_pos];
        let after_closing = &after_opening[closing_pos + 4..]; // Skip "\n---"

        // Content is everything after the closing "---" and its newline
        let body = after_closing
            .strip_prefix('\n')
            .unwrap_or(after_closing)
            .to_string();

        // Parse YAML with better error context
        let frontmatter: Frontmatter = serde_yaml::from_str(yaml_str).map_err(|e| {
            anyhow!(
                "Invalid YAML in frontmatter: {}",
                e.to_string().lines().next().unwrap_or("unknown error")
            )
        })?;

        Ok(Record {
            frontmatter,
            content: body,
            path,
        })
    }

    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    pub fn record_type(&self) -> &RecordType {
        &self.frontmatter.r#type
    }

    pub fn status(&self) -> &Status {
        &self.frontmatter.status
    }

    pub fn save(&self) -> Result<()> {
        use fs2::FileExt;
        use std::io::Write;

        let yaml = serde_yaml::to_string(&self.frontmatter)?;
        let content = format!("---\n{}---\n{}", yaml, self.content);

        // Open file with exclusive lock to prevent concurrent writes
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        // Acquire exclusive lock (blocks if another process has the file locked)
        file.lock_exclusive()?;

        // Write content
        let mut writer = std::io::BufWriter::new(&file);
        writer.write_all(content.as_bytes())?;
        writer.flush()?;

        // Lock is automatically released when file is dropped
        Ok(())
    }

    #[allow(dead_code)]
    pub fn filename(&self) -> String {
        let slug = self
            .frontmatter
            .title
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        format!("{}-{}.md", self.frontmatter.id, slug)
    }

    /// Extract DACI roles from the document content
    /// Returns a map of role -> list of names/usernames mentioned
    pub fn extract_daci_roles(&self) -> HashMap<String, Vec<String>> {
        let mut roles: HashMap<String, Vec<String>> = HashMap::new();

        // Pattern matches lines like: - **Responsible**: Name (description)
        // or **Driver**: Name
        let role_re = Regex::new(r"(?m)^\s*[-*]?\s*\*\*([A-Za-z]+)\*\*:\s*(.+)$").unwrap();

        for cap in role_re.captures_iter(&self.content) {
            let role = cap.get(1).map(|m| m.as_str().to_lowercase());
            let people_str = cap.get(2).map(|m| m.as_str());

            if let (Some(role), Some(people)) = (role, people_str) {
                // Only capture DACI roles
                let valid_roles = [
                    "driver",
                    "approver",
                    "approvers",
                    "contributor",
                    "contributors",
                    "informed",
                    "responsible",
                    "accountable",
                    "consulted",
                ];
                if !valid_roles.contains(&role.as_str()) {
                    continue;
                }

                // Normalize role names
                let normalized_role = match role.as_str() {
                    "driver" | "responsible" => "responsible",
                    "approver" | "approvers" | "accountable" => "approver",
                    "contributor" | "contributors" | "consulted" => "consulted",
                    "informed" => "informed",
                    _ => continue,
                };

                // Extract names - split by comma or "and"
                let names: Vec<String> = people
                    .split(|c| c == ',' || c == ';')
                    .flat_map(|s| s.split(" and "))
                    .map(|s| {
                        // Remove parenthetical descriptions and clean up
                        let s = s.trim();
                        if let Some(paren_pos) = s.find('(') {
                            s[..paren_pos].trim().to_string()
                        } else {
                            s.to_string()
                        }
                    })
                    .filter(|s| !s.is_empty())
                    .collect();

                roles
                    .entry(normalized_role.to_string())
                    .or_default()
                    .extend(names);
            }
        }

        roles
    }

    /// Extract action items from the document content
    /// Returns a list of (action_text, is_completed, owner_if_any)
    pub fn extract_action_items(&self) -> Vec<(String, bool, Option<String>)> {
        let mut actions = Vec::new();

        // Pattern matches: - [x] Action text @owner or - [ ] Action text
        let action_re = Regex::new(r"(?m)^\s*[-*]\s*\[([ xX])\]\s*(.+)$").unwrap();
        let owner_re = Regex::new(r"@(\w+)").unwrap();

        for cap in action_re.captures_iter(&self.content) {
            let is_completed = cap.get(1).map(|m| m.as_str() != " ").unwrap_or(false);
            let text = cap.get(2).map(|m| m.as_str().trim().to_string());

            if let Some(text) = text {
                // Extract @owner if present
                let owner = owner_re
                    .captures(&text)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string());

                actions.push((text, is_completed, owner));
            }
        }

        actions
    }
}

#[allow(dead_code)]
pub fn parse_id(id: &str) -> Option<(RecordType, u32)> {
    let re = Regex::new(r"^([A-Z]{3})-(\d+)").ok()?;
    let caps = re.captures(id)?;
    let prefix = caps.get(1)?.as_str();
    let num: u32 = caps.get(2)?.as_str().parse().ok()?;
    let record_type = RecordType::from_prefix(prefix)?;
    Some((record_type, num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        let (rt, num) = parse_id("DEC-001").unwrap();
        assert_eq!(rt, RecordType::Decision);
        assert_eq!(num, 1);

        let (rt, num) = parse_id("STR-042-some-slug").unwrap();
        assert_eq!(rt, RecordType::Strategy);
        assert_eq!(num, 42);
    }

    #[test]
    fn test_parse_id_all_types() {
        assert!(parse_id("DEC-001").is_some());
        assert!(parse_id("STR-001").is_some());
        assert!(parse_id("POL-001").is_some());
        assert!(parse_id("CUS-001").is_some());
        assert!(parse_id("OPP-001").is_some());
        assert!(parse_id("PRC-001").is_some());
        assert!(parse_id("HIR-001").is_some());
        assert!(parse_id("ADR-001").is_some());
        assert!(parse_id("LEG-001").is_some());
    }

    #[test]
    fn test_parse_id_invalid() {
        assert!(parse_id("INVALID").is_none());
        assert!(parse_id("DEC").is_none());
        assert!(parse_id("001").is_none());
        assert!(parse_id("XXX-001").is_none());
    }

    #[test]
    fn test_record_type_from_str() {
        assert_eq!(RecordType::from_str("decision"), Some(RecordType::Decision));
        assert_eq!(RecordType::from_str("DEC"), Some(RecordType::Decision));
        assert_eq!(RecordType::from_str("invalid"), None);
    }

    #[test]
    fn test_record_type_from_str_all_types() {
        assert_eq!(RecordType::from_str("decision"), Some(RecordType::Decision));
        assert_eq!(RecordType::from_str("strategy"), Some(RecordType::Strategy));
        assert_eq!(RecordType::from_str("policy"), Some(RecordType::Policy));
        assert_eq!(RecordType::from_str("customer"), Some(RecordType::Customer));
        assert_eq!(
            RecordType::from_str("opportunity"),
            Some(RecordType::Opportunity)
        );
        assert_eq!(RecordType::from_str("process"), Some(RecordType::Process));
        assert_eq!(RecordType::from_str("hiring"), Some(RecordType::Hiring));
        assert_eq!(RecordType::from_str("adr"), Some(RecordType::Adr));
        assert_eq!(RecordType::from_str("legal"), Some(RecordType::Legal));
    }

    #[test]
    fn test_record_type_prefix() {
        assert_eq!(RecordType::Decision.prefix(), "DEC");
        assert_eq!(RecordType::Strategy.prefix(), "STR");
        assert_eq!(RecordType::Policy.prefix(), "POL");
        assert_eq!(RecordType::Customer.prefix(), "CUS");
        assert_eq!(RecordType::Opportunity.prefix(), "OPP");
        assert_eq!(RecordType::Process.prefix(), "PRC");
        assert_eq!(RecordType::Hiring.prefix(), "HIR");
        assert_eq!(RecordType::Adr.prefix(), "ADR");
        assert_eq!(RecordType::Legal.prefix(), "LEG");
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("draft"), Some(Status::Draft));
        assert_eq!(Status::from_str("proposed"), Some(Status::Proposed));
        assert_eq!(Status::from_str("accepted"), Some(Status::Accepted));
        assert_eq!(Status::from_str("deprecated"), Some(Status::Deprecated));
        assert_eq!(Status::from_str("superseded"), Some(Status::Superseded));
        assert_eq!(Status::from_str("active"), Some(Status::Active));
        assert_eq!(Status::from_str("invalid"), None);
    }

    #[test]
    fn test_links_add_and_remove() {
        let mut links = Links::default();

        // Add links
        links.add_link("depends_on", "STR-001").unwrap();
        links.add_link("relates_to", "CUS-001").unwrap();

        assert!(links.depends_on.contains(&"STR-001".to_string()));
        assert!(links.relates_to.contains(&"CUS-001".to_string()));

        // Add duplicate (should not duplicate)
        links.add_link("depends_on", "STR-001").unwrap();
        assert_eq!(links.depends_on.len(), 1);

        // Remove links
        assert!(links.remove_link("depends_on", "STR-001").unwrap());
        assert!(!links.depends_on.contains(&"STR-001".to_string()));

        // Remove non-existent
        assert!(!links.remove_link("depends_on", "STR-001").unwrap());
    }

    #[test]
    fn test_links_invalid_type() {
        let mut links = Links::default();
        assert!(links.add_link("invalid_type", "DEC-001").is_err());
    }

    #[test]
    fn test_links_all_links() {
        let mut links = Links::default();
        links.add_link("depends_on", "STR-001").unwrap();
        links.add_link("relates_to", "CUS-001").unwrap();

        let all = links.all_links();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&("depends_on", "STR-001")));
        assert!(all.contains(&("relates_to", "CUS-001")));
    }

    #[test]
    fn test_record_parse_content() {
        let content = r#"---
type: decision
id: DEC-001
title: Test Decision
status: proposed
created: 2024-01-15
updated: 2024-01-15
authors: []
tags: []
links:
  supersedes: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
---

# Test Decision

Some content here.
"#;
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();

        assert_eq!(record.id(), "DEC-001");
        assert_eq!(record.title(), "Test Decision");
        assert_eq!(*record.record_type(), RecordType::Decision);
        assert_eq!(*record.status(), Status::Proposed);
        assert!(record.content.contains("Some content here"));
    }

    #[test]
    fn test_record_parse_invalid_frontmatter() {
        let content = "No frontmatter here";
        let result = Record::parse_content(content, std::path::PathBuf::from("test.md"));
        assert!(result.is_err());
    }

    #[test]
    fn test_record_parse_windows_line_endings() {
        let content = "---\r\ntype: decision\r\nid: DEC-001\r\ntitle: Test\r\nstatus: draft\r\ncreated: 2024-01-15\r\nupdated: 2024-01-15\r\nauthors: []\r\ntags: []\r\nlinks: {}\r\n---\r\n\r\n# Content\r\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert_eq!(record.id(), "DEC-001");
    }

    #[test]
    fn test_record_parse_leading_whitespace() {
        let content = "\n  ---\ntype: decision\nid: DEC-002\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\n---\n\n# Content\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert_eq!(record.id(), "DEC-002");
    }

    #[test]
    fn test_record_parse_no_trailing_newline() {
        let content = "---\ntype: decision\nid: DEC-003\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\n---\nContent without trailing newline";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert_eq!(record.id(), "DEC-003");
        assert!(record.content.contains("Content without trailing newline"));
    }

    #[test]
    fn test_record_parse_unterminated_frontmatter() {
        let content = "---\ntype: decision\nid: DEC-001\n# Missing closing delimiter";
        let result = Record::parse_content(content, std::path::PathBuf::from("test.md"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unterminated"));
    }

    #[test]
    fn test_record_parse_missing_frontmatter() {
        let content = "# Just markdown, no frontmatter";
        let result = Record::parse_content(content, std::path::PathBuf::from("test.md"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing frontmatter"));
    }

    fn make_record_with_content(content: &str) -> Record {
        let full_content = format!(
            "---\ntype: decision\nid: DEC-001\ntitle: Test\nstatus: proposed\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {{}}\n---\n\n{}",
            content
        );
        Record::parse_content(&full_content, std::path::PathBuf::from("test.md")).unwrap()
    }

    #[test]
    fn test_extract_daci_driver_role() {
        let record = make_record_with_content("- **Driver**: Alice Smith");
        let roles = record.extract_daci_roles();

        assert!(roles.contains_key("responsible"));
        assert!(roles["responsible"].contains(&"Alice Smith".to_string()));
    }

    #[test]
    fn test_extract_daci_multiple_roles() {
        let record = make_record_with_content(
            "- **Driver**: Alice\n- **Approver**: Bob\n- **Contributor**: Carol\n- **Informed**: Dave",
        );
        let roles = record.extract_daci_roles();

        assert!(roles.contains_key("responsible"));
        assert!(roles.contains_key("approver"));
        assert!(roles.contains_key("consulted"));
        assert!(roles.contains_key("informed"));
    }

    #[test]
    fn test_extract_daci_multiple_people() {
        let record = make_record_with_content("- **Approvers**: Alice, Bob and Carol");
        let roles = record.extract_daci_roles();

        assert!(roles.contains_key("approver"));
        let approvers = &roles["approver"];
        assert!(approvers.contains(&"Alice".to_string()));
        assert!(approvers.contains(&"Bob".to_string()));
        assert!(approvers.contains(&"Carol".to_string()));
    }

    #[test]
    fn test_extract_daci_with_descriptions() {
        let record = make_record_with_content("- **Driver**: Alice (Engineering Lead)");
        let roles = record.extract_daci_roles();

        // Should extract name without description
        assert!(roles["responsible"].contains(&"Alice".to_string()));
        assert!(!roles["responsible"]
            .iter()
            .any(|s| s.contains("Engineering")));
    }

    #[test]
    fn test_extract_daci_responsible_alias() {
        let record = make_record_with_content("- **Responsible**: Alice");
        let roles = record.extract_daci_roles();

        // Responsible should map to "responsible" key
        assert!(roles.contains_key("responsible"));
        assert!(roles["responsible"].contains(&"Alice".to_string()));
    }

    #[test]
    fn test_extract_daci_accountable_alias() {
        let record = make_record_with_content("- **Accountable**: Bob");
        let roles = record.extract_daci_roles();

        // Accountable should map to "approver" key
        assert!(roles.contains_key("approver"));
        assert!(roles["approver"].contains(&"Bob".to_string()));
    }

    #[test]
    fn test_extract_daci_consulted_alias() {
        let record = make_record_with_content("- **Consulted**: Carol");
        let roles = record.extract_daci_roles();

        // Consulted should map to "consulted" key
        assert!(roles.contains_key("consulted"));
        assert!(roles["consulted"].contains(&"Carol".to_string()));
    }

    #[test]
    fn test_extract_daci_ignores_invalid_roles() {
        let record = make_record_with_content("- **Manager**: Alice\n- **Stakeholder**: Bob");
        let roles = record.extract_daci_roles();

        // Should not contain invalid roles
        assert!(!roles.contains_key("manager"));
        assert!(!roles.contains_key("stakeholder"));
        assert!(roles.is_empty());
    }

    #[test]
    fn test_extract_daci_empty_content() {
        let record = make_record_with_content("No DACI roles here");
        let roles = record.extract_daci_roles();
        assert!(roles.is_empty());
    }

    #[test]
    fn test_extract_action_items_basic() {
        let record = make_record_with_content("- [ ] Complete the task");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert!(actions[0].0.contains("Complete the task"));
        assert!(!actions[0].1); // Not completed
        assert!(actions[0].2.is_none()); // No owner
    }

    #[test]
    fn test_extract_action_items_completed() {
        let record = make_record_with_content("- [x] Done task");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert!(actions[0].1); // Completed
    }

    #[test]
    fn test_extract_action_items_completed_uppercase() {
        let record = make_record_with_content("- [X] Done task");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert!(actions[0].1); // Completed
    }

    #[test]
    fn test_extract_action_items_with_owner() {
        let record = make_record_with_content("- [ ] Review PR @alice");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].2, Some("alice".to_string()));
    }

    #[test]
    fn test_extract_action_items_multiple() {
        let record =
            make_record_with_content("- [ ] Task 1 @alice\n- [x] Task 2 @bob\n- [ ] Task 3");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 3);
        assert!(!actions[0].1);
        assert_eq!(actions[0].2, Some("alice".to_string()));
        assert!(actions[1].1);
        assert_eq!(actions[1].2, Some("bob".to_string()));
        assert!(!actions[2].1);
        assert!(actions[2].2.is_none());
    }

    #[test]
    fn test_extract_action_items_asterisk_bullet() {
        let record = make_record_with_content("* [ ] Task with asterisk");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert!(actions[0].0.contains("Task with asterisk"));
    }

    #[test]
    fn test_extract_action_items_empty() {
        let record = make_record_with_content("No action items here");
        let actions = record.extract_action_items();
        assert!(actions.is_empty());
    }

    #[test]
    fn test_extract_action_items_owner_in_middle() {
        let record = make_record_with_content("- [ ] Ping @alice for review tomorrow");
        let actions = record.extract_action_items();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].2, Some("alice".to_string()));
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Draft.to_string(), "draft");
        assert_eq!(Status::Proposed.to_string(), "proposed");
        assert_eq!(Status::Accepted.to_string(), "accepted");
        assert_eq!(Status::Deprecated.to_string(), "deprecated");
        assert_eq!(Status::Superseded.to_string(), "superseded");
        assert_eq!(Status::Active.to_string(), "active");
        assert_eq!(Status::Open.to_string(), "open");
        assert_eq!(Status::Closed.to_string(), "closed");
        assert_eq!(Status::Resolved.to_string(), "resolved");
        assert_eq!(Status::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_record_type_display() {
        assert_eq!(RecordType::Decision.to_string(), "DEC");
        assert_eq!(RecordType::Adr.to_string(), "ADR");
        assert_eq!(RecordType::Incident.to_string(), "INC");
    }

    #[test]
    fn test_record_type_template_name() {
        assert_eq!(RecordType::Decision.template_name(), "decision");
        assert_eq!(RecordType::Adr.template_name(), "adr");
        assert_eq!(RecordType::Incident.template_name(), "incident");
        assert_eq!(RecordType::Meeting.template_name(), "meeting");
        assert_eq!(RecordType::Feedback.template_name(), "feedback");
    }

    #[test]
    fn test_record_type_display_name() {
        assert_eq!(RecordType::Decision.display_name(), "Decision");
        assert_eq!(RecordType::Adr.display_name(), "Architecture");
        assert_eq!(RecordType::Incident.display_name(), "Incident");
    }

    #[test]
    fn test_record_filename() {
        let content = "---\ntype: decision\nid: DEC-001\ntitle: My Test Decision\nstatus: proposed\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\n---\n\n# Content\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();

        let filename = record.filename();
        assert!(filename.starts_with("DEC-001-"));
        assert!(filename.ends_with(".md"));
        assert!(filename.contains("my-test-decision"));
    }

    #[test]
    fn test_links_all_link_types() {
        let mut links = Links::default();
        links.supersedes = vec!["A".to_string()];
        links.superseded_by = vec!["B".to_string()];
        links.depends_on = vec!["C".to_string()];
        links.enables = vec!["D".to_string()];
        links.relates_to = vec!["E".to_string()];
        links.conflicts_with = vec!["F".to_string()];
        links.refines = vec!["G".to_string()];
        links.implements = vec!["H".to_string()];

        let all = links.all_links();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&("supersedes", "A")));
        assert!(all.contains(&("superseded_by", "B")));
        assert!(all.contains(&("depends_on", "C")));
        assert!(all.contains(&("enables", "D")));
        assert!(all.contains(&("relates_to", "E")));
        assert!(all.contains(&("conflicts_with", "F")));
        assert!(all.contains(&("refines", "G")));
        assert!(all.contains(&("implements", "H")));
    }

    #[test]
    fn test_record_parse_minimal_links() {
        let content = "---\ntype: decision\nid: DEC-001\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\n---\n\n# Test\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert!(record.frontmatter.links.all_links().is_empty());
    }

    #[test]
    fn test_record_parse_with_extra_fields() {
        let content = "---\ntype: decision\nid: DEC-001\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\nseverity: high\ncustom_field: value\n---\n\n# Test\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();

        // Extra fields should be captured in the extra map
        assert!(record.frontmatter.extra.contains_key("severity"));
        assert!(record.frontmatter.extra.contains_key("custom_field"));
    }

    #[test]
    fn test_record_core_flag() {
        let content = "---\ntype: decision\nid: DEC-001\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\ncore: true\n---\n\n# Test\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert!(record.frontmatter.core);
    }

    #[test]
    fn test_record_core_flag_default_false() {
        let content = "---\ntype: decision\nid: DEC-001\ntitle: Test\nstatus: draft\ncreated: 2024-01-15\nupdated: 2024-01-15\nauthors: []\ntags: []\nlinks: {}\n---\n\n# Test\n";
        let record = Record::parse_content(content, std::path::PathBuf::from("test.md")).unwrap();
        assert!(!record.frontmatter.core);
    }
}
