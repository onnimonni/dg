use crate::models::{Graph, Record};
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// D2 diagram renderer
pub struct D2Renderer {
    d2_path: PathBuf,
}

impl D2Renderer {
    /// Create a new D2Renderer, finding the d2 binary in PATH
    pub fn new() -> Result<Self> {
        let d2_path = which::which("d2").map_err(|_| {
            anyhow!("d2 binary not found in PATH. Install with: brew install d2 or see https://d2lang.com")
        })?;
        Ok(Self { d2_path })
    }

    /// Check if d2 is available
    #[allow(dead_code)]
    pub fn is_available() -> bool {
        which::which("d2").is_ok()
    }

    /// Render D2 source to SVG
    pub fn render_svg(&self, d2_source: &str) -> Result<String> {
        let mut child = Command::new(&self.d2_path)
            .args(["-", "-"]) // stdin to stdout
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Write D2 source to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(d2_source.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("d2 render failed: {}", stderr));
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    /// Render D2 source to file
    #[allow(dead_code)]
    pub fn render_to_file(&self, d2_source: &str, output_path: &std::path::Path) -> Result<()> {
        // Create temp file for D2 source
        let temp_dir = std::env::temp_dir();
        let temp_d2 = temp_dir.join("graph.d2");
        std::fs::write(&temp_d2, d2_source)?;

        let output = Command::new(&self.d2_path)
            .args([temp_d2.to_str().unwrap(), output_path.to_str().unwrap()])
            .output()?;

        // Cleanup temp file
        let _ = std::fs::remove_file(&temp_d2);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("d2 render failed: {}", stderr));
        }

        Ok(())
    }
}

/// Generate D2 source from graph
pub fn graph_to_d2(graph: &Graph, subset: Option<&HashSet<String>>) -> String {
    let mut d2 = String::new();

    // Header and styling
    d2.push_str("direction: right\n\n");

    // Record type styles
    d2.push_str("classes: {\n");
    d2.push_str("  decision: { style: { fill: \"#4CAF50\" } }\n");
    d2.push_str("  strategy: { style: { fill: \"#2196F3\" } }\n");
    d2.push_str("  policy: { style: { fill: \"#FF9800\" } }\n");
    d2.push_str("  customer: { style: { fill: \"#9C27B0\" } }\n");
    d2.push_str("  opportunity: { style: { fill: \"#E91E63\" } }\n");
    d2.push_str("  process: { style: { fill: \"#00BCD4\" } }\n");
    d2.push_str("  hiring: { style: { fill: \"#795548\" } }\n");
    d2.push_str("  adr: { style: { fill: \"#607D8B\" } }\n");
    d2.push_str("  incident: { style: { fill: \"#F44336\" } }\n");
    d2.push_str("  runbook: { style: { fill: \"#8BC34A\" } }\n");
    d2.push_str("  meeting: { style: { fill: \"#03A9F4\" } }\n");
    d2.push_str("  core: { style: { stroke: \"#FFD700\"; stroke-width: 3 } }\n");
    d2.push_str("}\n\n");

    // Nodes
    for record in graph.all_records() {
        if let Some(subset) = subset {
            if !subset.contains(record.id()) {
                continue;
            }
        }
        d2.push_str(&node_to_d2(record));
    }

    d2.push('\n');

    // Edges
    for edge in &graph.edges {
        if let Some(subset) = subset {
            if !subset.contains(&edge.from) || !subset.contains(&edge.to) {
                continue;
            }
        }

        let style = match edge.link_type.as_str() {
            "supersedes" => " { style: { stroke: \"red\"; stroke-width: 2 } }",
            "depends_on" => " { style: { stroke: \"blue\" } }",
            "enables" => " { style: { stroke: \"green\"; stroke-dash: 3 } }",
            "relates_to" => " { style: { stroke: \"gray\"; stroke-dash: 5 } }",
            "conflicts_with" => " { style: { stroke: \"orange\"; stroke-width: 2 } }",
            "refines" => " { style: { stroke: \"purple\" } }",
            "implements" => " { style: { stroke: \"teal\" } }",
            _ => "",
        };

        d2.push_str(&format!(
            "{} -> {}: {}{}\n",
            escape_d2_id(&edge.from),
            escape_d2_id(&edge.to),
            edge.link_type,
            style
        ));
    }

    d2
}

fn node_to_d2(record: &Record) -> String {
    let class = record.record_type().to_string().to_lowercase();
    let title = escape_d2_label(record.title());
    let id = escape_d2_id(record.id());

    let mut classes = vec![class];
    if record.frontmatter.core {
        classes.push("core".to_string());
    }

    format!(
        "{}: {} {{\n  class: [{}]\n}}\n",
        id,
        title,
        classes.join("; ")
    )
}

fn escape_d2_id(s: &str) -> String {
    // D2 IDs with special chars need quoting
    if s.contains('-') || s.contains(' ') {
        format!("\"{}\"", s)
    } else {
        s.to_string()
    }
}

fn escape_d2_label(s: &str) -> String {
    // Labels always quoted, escape internal quotes
    format!("\"{}\"", s.replace('"', "\\\""))
}
