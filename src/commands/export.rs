use crate::models::Graph;
use anyhow::Result;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(docs_dir: &str, format: &str, output: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let content = match format {
        "csv" => export_csv(&graph),
        _ => export_json(&graph)?,
    };

    if let Some(output_path) = output {
        fs::write(output_path, &content)?;
        eprintln!("Exported to {}", output_path);
    } else {
        io::stdout().write_all(content.as_bytes())?;
    }

    Ok(())
}

fn export_json(graph: &Graph) -> Result<String> {
    let records: Vec<_> = graph
        .all_records()
        .map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "status": r.status().to_string(),
                "created": r.frontmatter.created.to_string(),
                "updated": r.frontmatter.updated.to_string(),
                "authors": r.frontmatter.authors,
                "tags": r.frontmatter.tags,
                "links": {
                    "supersedes": r.frontmatter.links.supersedes,
                    "superseded_by": r.frontmatter.links.superseded_by,
                    "depends_on": r.frontmatter.links.depends_on,
                    "enables": r.frontmatter.links.enables,
                    "relates_to": r.frontmatter.links.relates_to,
                    "conflicts_with": r.frontmatter.links.conflicts_with,
                },
                "content": r.content.trim(),
            })
        })
        .collect();

    Ok(serde_json::to_string_pretty(&records)?)
}

fn export_csv(graph: &Graph) -> String {
    let mut csv = String::from("id,type,status,title,created,updated,tags\n");

    for record in graph.all_records() {
        let tags = record.frontmatter.tags.join(";");
        csv.push_str(&format!(
            "{},{},{},\"{}\",{},{},\"{}\"\n",
            record.id(),
            record.record_type(),
            record.status(),
            record.title().replace('"', "\"\""),
            record.frontmatter.created,
            record.frontmatter.updated,
            tags
        ));
    }

    csv
}
