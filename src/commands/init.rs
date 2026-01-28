use crate::templates;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(docs_dir: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let decisions_path = docs_path.join(".decisions");
    let templates_path = docs_path.join(".templates");

    // Create directories
    fs::create_dir_all(&decisions_path)?;
    fs::create_dir_all(&templates_path)?;

    println!("{} {}", "Created".green(), decisions_path.display());
    println!("{} {}", "Created".green(), templates_path.display());

    // Copy templates
    for (name, content) in templates::get_templates() {
        let template_file = templates_path.join(name);
        if !template_file.exists() {
            fs::write(&template_file, content)?;
            println!("{} {}", "Created".green(), template_file.display());
        } else {
            println!(
                "{} {} (exists)",
                "Skipped".yellow(),
                template_file.display()
            );
        }
    }

    // Create .gitkeep
    let gitkeep = decisions_path.join(".gitkeep");
    if !gitkeep.exists() {
        fs::write(&gitkeep, "")?;
    }

    println!("\n{}", "Decision Graph initialized!".green().bold());
    println!(
        "Create your first record with: {} new decision \"Your Decision Title\"",
        "dg".cyan()
    );

    Ok(())
}
