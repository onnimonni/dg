use crate::models::{Graph, Record, RecordType};
use anyhow::{anyhow, Result};
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Change a record's type while preserving chronological order
pub fn run(docs_dir: &str, record_id: &str, new_type: &str, force: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Validate source record exists
    let source_record = graph
        .get(record_id)
        .ok_or_else(|| anyhow!("Record not found: {}", record_id))?;

    // Validate target type
    let target_type = RecordType::from_str(new_type)
        .ok_or_else(|| anyhow!("Unknown record type: {}", new_type))?;

    // Check if already the target type
    if source_record.record_type() == &target_type {
        return Err(anyhow!(
            "{} is already of type {}",
            record_id,
            target_type.prefix()
        ));
    }

    // Collect records of target type sorted by created date
    let mut target_records: Vec<&Record> = graph
        .all_records()
        .filter(|r| r.record_type() == &target_type)
        .collect();
    target_records.sort_by_key(|r| r.frontmatter.created);

    // Find chronological position
    let source_created = source_record.frontmatter.created;
    let chronological_position = target_records
        .iter()
        .position(|r| r.frontmatter.created > source_created)
        .unwrap_or(target_records.len());

    // Build the change plan
    let plan = build_change_plan(
        docs_path,
        &graph,
        source_record,
        &target_type,
        &target_records,
        chronological_position,
    )?;

    // Display the plan
    display_plan(&plan);

    // Ask for confirmation
    if !force {
        print!("\n{} Apply these changes? [y/N] ", "?".cyan().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Aborted", "✗".red());
            return Ok(());
        }
    }

    // Execute the plan
    execute_plan(docs_path, &plan)?;

    // Rebuild index
    let graph = Graph::load(docs_path)?;
    graph.save_index()?;

    println!("\n{} Retype complete!", "✓".green().bold());
    println!(
        "  {} → {}",
        plan.old_id.yellow().strikethrough(),
        plan.new_id.green().bold()
    );

    Ok(())
}

#[derive(Debug)]
struct ChangePlan {
    old_id: String,
    new_id: String,
    old_path: PathBuf,
    new_path: PathBuf,
    new_type: RecordType,
    /// Records that need their IDs shifted (old_id -> new_id)
    renames: Vec<RenameAction>,
    /// Files with link updates (file_path -> list of changes)
    link_updates: HashMap<PathBuf, Vec<IdReplacement>>,
    /// Code file mentions
    code_mentions: Vec<CodeMention>,
}

#[derive(Debug, Clone)]
struct RenameAction {
    old_id: String,
    new_id: String,
    old_path: PathBuf,
    new_path: PathBuf,
}

#[derive(Debug)]
struct IdReplacement {
    #[allow(dead_code)]
    old_id: String,
    #[allow(dead_code)]
    new_id: String,
    context: String,
}

#[derive(Debug)]
struct CodeMention {
    file: PathBuf,
    line: usize,
    #[allow(dead_code)]
    content: String,
    old_id: String,
    new_id: String,
}

fn build_change_plan(
    docs_path: &Path,
    graph: &Graph,
    source_record: &Record,
    target_type: &RecordType,
    target_records: &[&Record],
    chronological_position: usize,
) -> Result<ChangePlan> {
    let decisions_path = docs_path.join("decisions");
    let prefix = target_type.prefix();
    let old_id = source_record.id().to_string();

    // Calculate the new ID based on chronological position
    // Position 0 means before all existing records -> should be 001
    // Position 1 means after first record -> should be 002
    // etc.
    let target_number = chronological_position + 1;
    let new_id = format!("{}-{:03}", prefix, target_number);

    // Build rename actions for records that need to shift
    let mut renames = Vec::new();

    // Find records that need to shift (those with number >= target_number)
    for record in target_records.iter() {
        if let Some(num) = record
            .id()
            .split('-')
            .nth(1)
            .and_then(|s| s.parse::<u32>().ok())
        {
            if num >= target_number as u32 {
                let new_num = num + 1;
                let shifted_id = format!("{}-{:03}", prefix, new_num);

                // Generate new filename
                let new_filename = generate_filename(&shifted_id, record.title());
                let old_path = record.path.clone();
                let new_path = decisions_path.join(&new_filename);

                renames.push(RenameAction {
                    old_id: record.id().to_string(),
                    new_id: shifted_id,
                    old_path,
                    new_path,
                });
            }
        }
    }

    // Sort renames in reverse order (highest number first) to avoid conflicts
    renames.sort_by(|a, b| {
        let a_num: u32 = a
            .old_id
            .split('-')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let b_num: u32 = b
            .old_id
            .split('-')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        b_num.cmp(&a_num)
    });

    // Generate new path for source record
    let new_filename = generate_filename(&new_id, source_record.title());
    let old_path = source_record.path.clone();
    let new_path = decisions_path.join(&new_filename);

    // Build complete ID mapping (old -> new)
    let mut id_mapping: HashMap<String, String> = HashMap::new();
    id_mapping.insert(old_id.clone(), new_id.clone());
    for rename in &renames {
        id_mapping.insert(rename.old_id.clone(), rename.new_id.clone());
    }

    // Find all link updates needed in all record files
    let mut link_updates: HashMap<PathBuf, Vec<IdReplacement>> = HashMap::new();

    for record in graph.all_records() {
        let mut updates = Vec::new();

        // Check all links in this record
        for (link_type, target_id) in record.frontmatter.links.all_links() {
            if let Some(new_target_id) = id_mapping.get(target_id) {
                updates.push(IdReplacement {
                    old_id: target_id.to_string(),
                    new_id: new_target_id.clone(),
                    context: format!("{}: {} → {}", link_type, target_id, new_target_id),
                });
            }
        }

        if !updates.is_empty() {
            link_updates.insert(record.path.clone(), updates);
        }
    }

    // Find code mentions
    let code_mentions = find_code_mentions(docs_path, &id_mapping)?;

    Ok(ChangePlan {
        old_id,
        new_id,
        old_path,
        new_path,
        new_type: target_type.clone(),
        renames,
        link_updates,
        code_mentions,
    })
}

fn generate_filename(id: &str, title: &str) -> String {
    let slug = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    format!("{}-{}.md", id, slug)
}

fn find_code_mentions(
    docs_path: &Path,
    id_mapping: &HashMap<String, String>,
) -> Result<Vec<CodeMention>> {
    let mut mentions = Vec::new();

    // Find project root
    let project_root = find_project_root(docs_path)
        .unwrap_or_else(|| docs_path.parent().unwrap_or(docs_path).to_path_buf());

    // Build regex pattern for all IDs we're renaming
    let ids: Vec<&str> = id_mapping.keys().map(|s| s.as_str()).collect();
    if ids.is_empty() {
        return Ok(mentions);
    }

    let pattern = format!(
        r"\b({})\b",
        ids.iter()
            .map(|id| regex::escape(id))
            .collect::<Vec<_>>()
            .join("|")
    );
    let re = Regex::new(&pattern)?;

    let code_extensions = [
        "rs", "py", "js", "ts", "tsx", "jsx", "rb", "go", "java", "kt", "swift", "c", "cpp", "h",
        "hpp", "cs", "php", "sh", "bash", "zsh", "yaml", "yml", "toml", "json", "sql", "ex", "exs",
        "erl", "hs", "ml", "scala", "clj",
    ];

    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_entry(|e| !is_ignored(e.path()))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Skip markdown files in decisions (handled separately)
        if path.to_string_lossy().contains("decisions")
            && path.extension().is_some_and(|e| e == "md")
        {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !code_extensions.contains(&ext) {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            for (line_num, line) in content.lines().enumerate() {
                for cap in re.captures_iter(line) {
                    let matched_id = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    if let Some(new_id) = id_mapping.get(matched_id) {
                        mentions.push(CodeMention {
                            file: path.to_path_buf(),
                            line: line_num + 1,
                            content: line.to_string(),
                            old_id: matched_id.to_string(),
                            new_id: new_id.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(mentions)
}

fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".git").exists()
            || current.join("Cargo.toml").exists()
            || current.join("package.json").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn is_ignored(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    name.starts_with('.')
        || name == "node_modules"
        || name == "target"
        || name == "dist"
        || name == "build"
        || name == "vendor"
        || name == "__pycache__"
}

fn display_plan(plan: &ChangePlan) {
    println!("{}", "Retype Change Plan".green().bold());
    println!("{}", "═".repeat(50));

    // Primary change
    println!("\n{}", "Primary Record Change:".white().bold());
    println!(
        "  {} {} → {}",
        "◆".cyan(),
        plan.old_id.yellow(),
        plan.new_id.green().bold()
    );
    println!(
        "    {} {}",
        "File:".dimmed(),
        plan.new_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );

    // Records that need renumbering
    if !plan.renames.is_empty() {
        println!(
            "\n{} ({}):",
            "Records to Renumber".white().bold(),
            plan.renames.len()
        );
        for rename in &plan.renames {
            println!(
                "  {} {} → {}",
                "◇".cyan(),
                rename.old_id.yellow(),
                rename.new_id.green()
            );
        }
    }

    // Link updates
    if !plan.link_updates.is_empty() {
        let total_updates: usize = plan.link_updates.values().map(|v| v.len()).sum();
        println!(
            "\n{} ({} in {} files):",
            "Link Updates".white().bold(),
            total_updates,
            plan.link_updates.len()
        );
        for (path, updates) in &plan.link_updates {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            println!("  {} {}:", "│".dimmed(), filename.yellow());
            for update in updates {
                println!("    {} {}", "└".dimmed(), update.context.dimmed());
            }
        }
    }

    // Code mentions
    if !plan.code_mentions.is_empty() {
        println!(
            "\n{} ({}):",
            "Code Mentions to Update".white().bold(),
            plan.code_mentions.len()
        );
        for mention in &plan.code_mentions {
            let filename = mention
                .file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            println!(
                "  {} {}:{} {} → {}",
                "│".dimmed(),
                filename.yellow(),
                mention.line.to_string().dimmed(),
                mention.old_id.yellow(),
                mention.new_id.green()
            );
        }
    }

    // Summary
    println!("\n{}", "─".repeat(50));
    let total_changes = 1
        + plan.renames.len()
        + plan.link_updates.values().map(|v| v.len()).sum::<usize>()
        + plan.code_mentions.len();
    println!(
        "{} {} total changes across {} files",
        "Summary:".white().bold(),
        total_changes.to_string().cyan(),
        (1 + plan.renames.len()
            + plan.link_updates.len()
            + plan
                .code_mentions
                .iter()
                .map(|m| &m.file)
                .collect::<std::collections::HashSet<_>>()
                .len())
        .to_string()
        .cyan()
    );
}

fn execute_plan(_docs_path: &Path, plan: &ChangePlan) -> Result<()> {
    println!("\n{}", "Executing changes...".dimmed());

    // Build complete ID mapping upfront
    let mut id_mapping: HashMap<String, String> = HashMap::new();
    id_mapping.insert(plan.old_id.clone(), plan.new_id.clone());
    for rename in &plan.renames {
        id_mapping.insert(rename.old_id.clone(), rename.new_id.clone());
    }

    // Track files we've processed to avoid double-processing
    let mut processed_files: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    // 1. Rename existing records that need to shift (in reverse order to avoid conflicts)
    // Apply full ID mapping to each file as we process it
    for rename in &plan.renames {
        println!(
            "  {} Renaming {} → {}",
            "→".dimmed(),
            rename.old_id,
            rename.new_id
        );

        let content = fs::read_to_string(&rename.old_path)?;

        // First update the type (just in case) and ID in frontmatter
        let updated_content = update_record_id_only(&content, &rename.old_id, &rename.new_id)?;

        // Then apply the full ID mapping to fix all link references
        let updated_content = replace_ids_in_links(&updated_content, &id_mapping);

        fs::write(&rename.old_path, &updated_content)?;
        fs::rename(&rename.old_path, &rename.new_path)?;

        processed_files.insert(rename.new_path.clone());
    }

    // 2. Update the source record (change type and ID)
    println!(
        "  {} Converting {} → {}",
        "→".dimmed(),
        plan.old_id,
        plan.new_id
    );
    let content = fs::read_to_string(&plan.old_path)?;
    let updated_content =
        update_record_type_and_id_only(&content, &plan.new_type, &plan.old_id, &plan.new_id)?;
    let updated_content = replace_ids_in_links(&updated_content, &id_mapping);
    fs::write(&plan.old_path, &updated_content)?;
    fs::rename(&plan.old_path, &plan.new_path)?;

    processed_files.insert(plan.new_path.clone());

    // 3. Update links in all other affected record files
    for path in plan.link_updates.keys() {
        // Skip files we've already processed
        if path == &plan.old_path || plan.renames.iter().any(|r| &r.old_path == path) {
            continue;
        }

        if path.exists() && !processed_files.contains(path) {
            let content = fs::read_to_string(path)?;
            let updated = replace_ids_in_content(&content, &id_mapping);
            fs::write(path, updated)?;
            processed_files.insert(path.clone());
        }
    }

    // 4. Update code mentions
    for mention in &plan.code_mentions {
        if !processed_files.contains(&mention.file) && mention.file.exists() {
            let content = fs::read_to_string(&mention.file)?;
            let updated = replace_ids_in_content(&content, &id_mapping);
            fs::write(&mention.file, updated)?;
            processed_files.insert(mention.file.clone());
        }
    }

    Ok(())
}

/// Update only the ID in frontmatter (not links or other references)
fn update_record_id_only(content: &str, old_id: &str, new_id: &str) -> Result<String> {
    let re = Regex::new(&format!(r"(?m)^id:\s*{}\s*$", regex::escape(old_id)))?;
    Ok(re.replace(content, format!("id: {}", new_id)).to_string())
}

/// Update type and ID in frontmatter only (not links or other references)
fn update_record_type_and_id_only(
    content: &str,
    new_type: &RecordType,
    old_id: &str,
    new_id: &str,
) -> Result<String> {
    // Replace type in frontmatter
    let type_re = Regex::new(r"(?m)^type:\s*\w+\s*$")?;
    let result = type_re.replace(content, format!("type: {}", new_type.template_name()));

    // Replace id in frontmatter
    let id_re = Regex::new(&format!(r"(?m)^id:\s*{}\s*$", regex::escape(old_id)))?;
    Ok(id_re
        .replace(&result, format!("id: {}", new_id))
        .to_string())
}

/// Replace IDs only in the links section (not the id: field)
fn replace_ids_in_links(content: &str, id_mapping: &HashMap<String, String>) -> String {
    if id_mapping.is_empty() {
        return content.to_string();
    }

    // Find the links: section and only replace within it
    // This avoids accidentally changing the record's own ID in frontmatter
    if let Some(links_start) = content.find("\nlinks:") {
        let before_links = &content[..links_start + 1];
        let links_section = &content[links_start + 1..];

        // Find where links section ends (next non-indented line or end of frontmatter)
        let links_end = links_section
            .find("\n---")
            .or_else(|| {
                links_section
                    .lines()
                    .skip(1) // Skip "links:" line
                    .position(|line| {
                        !line.is_empty() && !line.starts_with(' ') && !line.starts_with('\t')
                    })
                    .map(|pos| {
                        links_section
                            .lines()
                            .take(pos + 1)
                            .map(|l| l.len() + 1)
                            .sum::<usize>()
                    })
            })
            .unwrap_or(links_section.len());

        let links_content = &links_section[..links_end];
        let after_links = &links_section[links_end..];

        // Replace IDs only in the links section using single-pass replacement
        let pattern = format!(
            r"\b({})\b",
            id_mapping
                .keys()
                .map(|id| regex::escape(id))
                .collect::<Vec<_>>()
                .join("|")
        );

        let re = Regex::new(&pattern).unwrap();
        let updated_links = re
            .replace_all(links_content, |caps: &regex::Captures| {
                let matched = caps.get(0).unwrap().as_str();
                id_mapping
                    .get(matched)
                    .cloned()
                    .unwrap_or_else(|| matched.to_string())
            })
            .to_string();

        format!("{}{}{}", before_links, updated_links, after_links)
    } else {
        content.to_string()
    }
}

fn replace_ids_in_content(content: &str, id_mapping: &HashMap<String, String>) -> String {
    if id_mapping.is_empty() {
        return content.to_string();
    }

    // Build a single regex that matches any of the IDs
    // This ensures we don't chain replacements (e.g., DEC-002 → ADR-001 → ADR-002)
    let pattern = format!(
        r"\b({})\b",
        id_mapping
            .keys()
            .map(|id| regex::escape(id))
            .collect::<Vec<_>>()
            .join("|")
    );

    let re = Regex::new(&pattern).unwrap();

    // Replace all matches in a single pass using a callback
    re.replace_all(content, |caps: &regex::Captures| {
        let matched = caps.get(0).unwrap().as_str();
        id_mapping
            .get(matched)
            .cloned()
            .unwrap_or_else(|| matched.to_string())
    })
    .to_string()
}
