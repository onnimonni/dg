use crate::models::Graph;
use crate::serve::generate_site;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, output: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let output_path = match output {
        Some(o) => Path::new(o).to_path_buf(),
        None => docs_path.join("_site"),
    };

    println!("{} Loading graph from {}...", "Building".cyan(), docs_dir);

    let graph = Graph::load(docs_path)?;
    let record_count = graph.all_records().count();

    println!(
        "{} Generating site for {} records...",
        "Building".cyan(),
        record_count
    );

    generate_site(&graph, &output_path)?;

    println!(
        "{} Static site generated at {}",
        "Done".green().bold(),
        output_path.display()
    );

    Ok(())
}
