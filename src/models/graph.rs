use crate::models::{Record, RecordType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Re-export validation from shared module
pub use super::validation::{validate_graph, ValidationError, ValidationOptions};

/// Path through dependency graph
#[derive(Debug, Clone)]
pub struct DependencyPath {
    pub nodes: Vec<String>,
    pub link_types: Vec<String>,
}

impl DependencyPath {
    pub fn new(start: String) -> Self {
        Self {
            nodes: vec![start],
            link_types: Vec::new(),
        }
    }

    pub fn extend(&self, next_id: String, link_type: String) -> Self {
        let mut nodes = self.nodes.clone();
        let mut link_types = self.link_types.clone();
        nodes.push(next_id);
        link_types.push(link_type);
        Self { nodes, link_types }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub record_type: RecordType,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub link_type: String,
}

#[derive(Debug)]
pub struct Graph {
    pub records: HashMap<String, Record>,
    pub edges: Vec<GraphEdge>,
    docs_dir: Option<std::path::PathBuf>,
}

/// Index file structure for fast metadata access
#[derive(Debug, Serialize, Deserialize)]
struct IndexFile {
    generated: String,
    records: Vec<IndexRecord>,
    stats: IndexStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexRecord {
    id: String,
    title: String,
    #[serde(rename = "type")]
    record_type: String,
    status: String,
    tags: Vec<String>,
    links: Vec<IndexLink>,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexLink {
    #[serde(rename = "type")]
    link_type: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexStats {
    total: usize,
    edges: usize,
}

impl Graph {
    pub fn load(docs_dir: &Path) -> Result<Graph> {
        let decisions_dir = docs_dir.join(".decisions");
        let mut records = HashMap::new();
        let mut edges = Vec::new();

        if !decisions_dir.exists() {
            return Ok(Graph {
                records,
                edges,
                docs_dir: Some(docs_dir.to_path_buf()),
            });
        }

        for entry in WalkDir::new(&decisions_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                match Record::parse(path) {
                    Ok(record) => {
                        let id = record.id().to_string();

                        // Collect edges
                        for (link_type, target) in record.frontmatter.links.all_links() {
                            edges.push(GraphEdge {
                                from: id.clone(),
                                to: target.to_string(),
                                link_type: link_type.to_string(),
                            });
                        }

                        records.insert(id, record);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(Graph {
            records,
            edges,
            docs_dir: Some(docs_dir.to_path_buf()),
        })
    }

    /// Save the index file for fast metadata access
    pub fn save_index(&self) -> Result<()> {
        let docs_dir = match &self.docs_dir {
            Some(d) => d,
            None => return Ok(()),
        };

        let index_path = docs_dir.join(".index.json");

        let index = IndexFile {
            generated: chrono::Local::now().to_rfc3339(),
            records: self
                .records
                .values()
                .map(|r| {
                    let mut tags: Vec<String> = r.frontmatter.tags.clone();
                    tags.sort();
                    IndexRecord {
                        id: r.id().to_string(),
                        title: r.title().to_string(),
                        record_type: r.record_type().to_string(),
                        status: r.status().to_string(),
                        tags,
                        links: r
                            .frontmatter
                            .links
                            .all_links()
                            .iter()
                            .map(|(lt, target)| IndexLink {
                                link_type: lt.to_string(),
                                target: target.to_string(),
                            })
                            .collect(),
                        path: r
                            .path
                            .strip_prefix(docs_dir)
                            .unwrap_or(&r.path)
                            .to_string_lossy()
                            .to_string(),
                    }
                })
                .collect(),
            stats: IndexStats {
                total: self.records.len(),
                edges: self.edges.len(),
            },
        };

        fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Record> {
        self.records.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Record> {
        self.records.get_mut(id)
    }

    pub fn all_records(&self) -> impl Iterator<Item = &Record> {
        self.records.values()
    }

    #[allow(dead_code)]
    pub fn records_by_type(&self, record_type: &RecordType) -> Vec<&Record> {
        self.records
            .values()
            .filter(|r| r.record_type() == record_type)
            .collect()
    }

    pub fn outgoing_edges(&self, id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.from == id).collect()
    }

    pub fn incoming_edges(&self, id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.to == id).collect()
    }

    pub fn neighbors(&self, id: &str, depth: usize) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((id.to_string(), 0));
        visited.insert(id.to_string());

        while let Some((current, current_depth)) = queue.pop_front() {
            if current_depth >= depth {
                continue;
            }

            // Outgoing
            for edge in self.outgoing_edges(&current) {
                if !visited.contains(&edge.to) {
                    visited.insert(edge.to.clone());
                    queue.push_back((edge.to.clone(), current_depth + 1));
                }
            }

            // Incoming
            for edge in self.incoming_edges(&current) {
                if !visited.contains(&edge.from) {
                    visited.insert(edge.from.clone());
                    queue.push_back((edge.from.clone(), current_depth + 1));
                }
            }
        }

        visited
    }

    pub fn next_id(&self, record_type: &RecordType) -> String {
        let prefix = record_type.prefix();
        let max_num = self
            .records
            .keys()
            .filter(|id| id.starts_with(prefix))
            .filter_map(|id| id.split('-').nth(1).and_then(|s| s.parse::<u32>().ok()))
            .max()
            .unwrap_or(0);

        format!("{}-{:03}", prefix, max_num + 1)
    }

    pub fn search(&self, query: &str, include_content: bool) -> Vec<&Record> {
        let query_lower = query.to_lowercase();
        self.records
            .values()
            .filter(|r| {
                r.title().to_lowercase().contains(&query_lower)
                    || r.id().to_lowercase().contains(&query_lower)
                    || r.frontmatter
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
                    || (include_content && r.content.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn validate(&self) -> Vec<ValidationError> {
        validate_graph(self, &ValidationOptions::basic())
    }

    pub fn stats(&self) -> GraphStats {
        let mut by_type = HashMap::new();
        let mut by_status = HashMap::new();

        for record in self.records.values() {
            *by_type.entry(record.record_type().to_string()).or_insert(0) += 1;
            *by_status.entry(record.status().to_string()).or_insert(0) += 1;
        }

        GraphStats {
            total_records: self.records.len(),
            total_edges: self.edges.len(),
            by_type,
            by_status,
        }
    }

    pub fn to_dot(&self, subset: Option<&HashSet<String>>) -> String {
        let mut dot = String::from("digraph DecisionGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box, style=rounded];\n\n");

        // Color map for record types
        let type_colors = [
            ("DEC", "#4CAF50"),
            ("STR", "#2196F3"),
            ("POL", "#FF9800"),
            ("CUS", "#9C27B0"),
            ("OPP", "#E91E63"),
            ("PRC", "#00BCD4"),
            ("HIR", "#795548"),
            ("ADR", "#607D8B"),
            ("INC", "#F44336"),
            ("RUN", "#8BC34A"),
            ("MTG", "#03A9F4"),
        ];

        for record in self.records.values() {
            if let Some(subset) = subset {
                if !subset.contains(record.id()) {
                    continue;
                }
            }

            let color = type_colors
                .iter()
                .find(|(t, _)| record.id().starts_with(t))
                .map(|(_, c)| *c)
                .unwrap_or("#999999");

            let label = format!("{}\\n{}", record.id(), truncate(record.title(), 30));
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor=\"{}\", style=\"filled,rounded\"];\n",
                record.id(),
                label,
                color
            ));
        }

        dot.push('\n');

        // Edge styles for link types
        let edge_styles = [
            ("supersedes", "bold, color=red"),
            ("depends_on", "color=blue"),
            ("enables", "color=green, style=dashed"),
            ("relates_to", "color=gray, style=dotted"),
            ("conflicts_with", "color=orange, style=bold"),
            ("refines", "color=purple"),
            ("implements", "color=teal"),
        ];

        for edge in &self.edges {
            if let Some(subset) = subset {
                if !subset.contains(&edge.from) || !subset.contains(&edge.to) {
                    continue;
                }
            }

            let style = edge_styles
                .iter()
                .find(|(t, _)| edge.link_type == *t)
                .map(|(_, s)| *s)
                .unwrap_or("");

            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\", {}];\n",
                edge.from, edge.to, edge.link_type, style
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Returns all records marked as foundational
    pub fn foundational_records(&self) -> Vec<&Record> {
        self.records
            .values()
            .filter(|r| r.frontmatter.foundational)
            .collect()
    }

    /// Trace dependencies backward through depends_on links (BFS)
    /// Returns all paths from the given ID to its dependencies
    pub fn trace_dependencies(&self, id: &str) -> Vec<DependencyPath> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(DependencyPath::new(id.to_string()));
        visited.insert(id.to_string());

        while let Some(current_path) = queue.pop_front() {
            let current_id = current_path.nodes.last().unwrap();

            // Find depends_on edges from current node
            let deps: Vec<_> = self
                .edges
                .iter()
                .filter(|e| &e.from == current_id && e.link_type == "depends_on")
                .collect();

            let has_deps = !deps.is_empty();
            if !has_deps && current_path.nodes.len() > 1 {
                // End of chain - save this path
                paths.push(current_path);
            } else {
                for edge in &deps {
                    if !visited.contains(&edge.to) {
                        visited.insert(edge.to.clone());
                        let new_path = current_path.extend(edge.to.clone(), edge.link_type.clone());
                        queue.push_back(new_path);
                    }
                }
                // If we had deps but also want to save intermediate paths
                if current_path.nodes.len() > 1 && has_deps {
                    paths.push(current_path);
                }
            }
        }

        paths
    }

    /// Trace dependents forward - what depends on this record (BFS)
    /// Returns all paths from records that depend on the given ID
    pub fn trace_dependents(&self, id: &str) -> Vec<DependencyPath> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(DependencyPath::new(id.to_string()));
        visited.insert(id.to_string());

        while let Some(current_path) = queue.pop_front() {
            let current_id = current_path.nodes.last().unwrap();

            // Find records that have depends_on pointing TO current node
            let dependents: Vec<_> = self
                .edges
                .iter()
                .filter(|e| &e.to == current_id && e.link_type == "depends_on")
                .collect();

            let has_dependents = !dependents.is_empty();
            if !has_dependents && current_path.nodes.len() > 1 {
                paths.push(current_path);
            } else {
                for edge in &dependents {
                    if !visited.contains(&edge.from) {
                        visited.insert(edge.from.clone());
                        let new_path =
                            current_path.extend(edge.from.clone(), edge.link_type.clone());
                        queue.push_back(new_path);
                    }
                }
                if current_path.nodes.len() > 1 && has_dependents {
                    paths.push(current_path);
                }
            }
        }

        paths
    }

    /// Get related records for context (all neighbors + their links)
    pub fn context(&self, query: &str, depth: usize) -> ContextResult<'_> {
        let matching = self.search(query, true);
        let mut all_ids: HashSet<String> = matching.iter().map(|r| r.id().to_string()).collect();

        // Expand to neighbors
        for record in &matching {
            let neighbors = self.neighbors(record.id(), depth);
            all_ids.extend(neighbors);
        }

        let records: Vec<&Record> = all_ids.iter().filter_map(|id| self.get(id)).collect();

        let edges: Vec<&GraphEdge> = self
            .edges
            .iter()
            .filter(|e| all_ids.contains(&e.from) && all_ids.contains(&e.to))
            .collect();

        ContextResult { records, edges }
    }
}

/// Result of context query
pub struct ContextResult<'a> {
    pub records: Vec<&'a Record>,
    pub edges: Vec<&'a GraphEdge>,
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

#[derive(Debug)]
pub struct GraphStats {
    pub total_records: usize,
    pub total_edges: usize,
    pub by_type: HashMap<String, usize>,
    pub by_status: HashMap<String, usize>,
}
