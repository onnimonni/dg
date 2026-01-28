use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, quiet: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let errors = graph.validate();

    if errors.is_empty() {
        if !quiet {
            println!(
                "{} All {} records are valid.",
                "OK".green().bold(),
                graph.records.len()
            );
        }
        return Ok(());
    }

    // Always print errors, even in quiet mode
    println!(
        "{} Found {} issues:\n",
        "WARN".yellow().bold(),
        errors.len()
    );

    for error in &errors {
        println!("  {} {}", "-".red(), error);
    }

    println!();

    Ok(())
}
