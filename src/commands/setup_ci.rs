//! Set up CI/CD for multi-player decision graph workflows

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

const FINALIZE_WORKFLOW: &str = r#"name: Finalize Draft Records

# This workflow automatically converts draft record IDs (DEC-NEW-YYYYMMDDHHMMSS)
# to permanent incremental IDs (DEC-001) when a PR is merged to main.
# This enables multi-player mode where developers can work on records
# simultaneously without ID conflicts.

on:
  pull_request:
    types: [closed]
    branches: [main]

jobs:
  finalize:
    # Only run when PR is actually merged (not just closed)
    if: github.event.pull_request.merged == true
    runs-on: ubuntu-latest
    permissions:
      contents: write

    steps:
      - uses: actions/checkout@v4
        with:
          ref: main
          fetch-depth: 0
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Install dg
        run: |
          curl -fsSL https://github.com/onnimonni/dg/releases/latest/download/dg-linux-x86_64 -o dg
          chmod +x dg
          sudo mv dg /usr/local/bin/

      - name: Check for draft records
        id: check
        run: |
          if grep -r "\-NEW-" docs/.decisions/*.md 2>/dev/null; then
            echo "has_drafts=true" >> $GITHUB_OUTPUT
          else
            echo "has_drafts=false" >> $GITHUB_OUTPUT
          fi

      - name: Finalize draft records
        if: steps.check.outputs.has_drafts == 'true'
        run: dg finalize

      - name: Commit and push changes
        if: steps.check.outputs.has_drafts == 'true'
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add docs/.decisions/
          git diff --staged --quiet || git commit -m "chore: finalize draft record IDs [skip ci]"
          git push
"#;

const LINT_WORKFLOW: &str = r#"name: Lint Decision Graph

on:
  pull_request:
    paths:
      - 'docs/.decisions/**'
      - 'docs/dg.toml'

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install dg
        run: |
          curl -fsSL https://github.com/onnimonni/dg/releases/latest/download/dg-linux-x86_64 -o dg
          chmod +x dg
          sudo mv dg /usr/local/bin/

      - name: Lint records
        run: dg lint --strict

      - name: Validate links
        run: dg validate
"#;

pub fn run(docs_dir: &str, feature: &str, dry_run: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);

    // Detect repo root (go up from docs_dir to find .git)
    let repo_root = find_repo_root(docs_path)?;
    let workflows_dir = repo_root.join(".github/workflows");

    match feature {
        "finalize" => {
            let workflow_path = workflows_dir.join("dg-finalize.yml");
            setup_workflow(
                &workflow_path,
                FINALIZE_WORKFLOW,
                "finalize drafts",
                dry_run,
            )?;
        }
        "lint" => {
            let workflow_path = workflows_dir.join("dg-lint.yml");
            setup_workflow(&workflow_path, LINT_WORKFLOW, "lint records", dry_run)?;
        }
        "all" => {
            fs::create_dir_all(&workflows_dir)?;

            let finalize_path = workflows_dir.join("dg-finalize.yml");
            setup_workflow(
                &finalize_path,
                FINALIZE_WORKFLOW,
                "finalize drafts",
                dry_run,
            )?;

            let lint_path = workflows_dir.join("dg-lint.yml");
            setup_workflow(&lint_path, LINT_WORKFLOW, "lint records", dry_run)?;
        }
        _ => {
            return Err(anyhow!(
                "Unknown feature: {}. Use 'finalize', 'lint', or 'all'",
                feature
            ));
        }
    }

    println!("\n{}", "Setup complete!".green().bold());
    if !dry_run {
        println!("Commit the workflow files and push to enable GitHub Actions.");
    }

    Ok(())
}

fn find_repo_root(start: &Path) -> Result<std::path::PathBuf> {
    let mut current = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return Err(anyhow!("Not in a git repository")),
        }
    }
}

fn setup_workflow(path: &Path, content: &str, description: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("Would create: {} ({})", path.display(), description);
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        println!(
            "{} {} (already exists)",
            "Skipping:".yellow(),
            path.display()
        );
    } else {
        fs::write(path, content)?;
        println!(
            "{} {} ({})",
            "Created:".green(),
            path.display(),
            description
        );
    }

    Ok(())
}
