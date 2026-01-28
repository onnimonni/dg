use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    graph.save_index()?;

    let index_path = docs_path.join(".index.json");
    println!(
        "{} {} ({} records, {} edges)",
        "Reindexed".green(),
        index_path.display(),
        graph.records.len(),
        graph.edges.len()
    );

    Ok(())
}
