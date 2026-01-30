use crate::models::Graph;
use crate::serve::generate_site;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, output: Option<&str>, base_url: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let output_path = match output {
        Some(o) => Path::new(o).to_path_buf(),
        None => docs_path.join("_site"),
    };

    // Normalize base_url: ensure it starts with / and doesn't end with /
    let base_url = base_url.map(|url| {
        let url = url.trim_end_matches('/');
        if url.is_empty() {
            String::new()
        } else if url.starts_with('/') {
            url.to_string()
        } else {
            format!("/{}", url)
        }
    });

    println!("{} Loading graph from {}...", "Building".cyan(), docs_dir);

    let graph = Graph::load(docs_path)?;
    let record_count = graph.all_records().count();

    println!(
        "{} Generating site for {} records...",
        "Building".cyan(),
        record_count
    );

    generate_site(&graph, &output_path, docs_path, base_url.as_deref())?;

    println!(
        "{} Static site generated at {}",
        "Done".green().bold(),
        output_path.display()
    );

    Ok(())
}
