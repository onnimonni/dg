use crate::models::Graph;
use crate::serve::config::{DgConfig, SiteConfig};
use crate::serve::templates::create_environment;
use anyhow::Result;
use minijinja::context;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn generate_site(graph: &Graph, output_dir: &Path, docs_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    let records_dir = output_dir.join("records");
    fs::create_dir_all(&records_dir)?;

    // Load site config
    let site_config = SiteConfig::load(docs_dir)?;

    // Load users/teams for mention validation
    let dg_config = DgConfig::load(docs_dir)?;
    let valid_mentions: HashSet<String> = dg_config
        .users
        .keys()
        .chain(dg_config.teams.keys())
        .cloned()
        .collect();

    let env = create_environment();

    // Generate index page
    let index_tmpl = env.get_template("index.html")?;
    let records_data: Vec<_> = graph.all_records().map(record_to_context).collect();

    let mut record_types: Vec<String> = records_data
        .iter()
        .filter_map(|r| {
            r.get("type")
                .and_then(|t| t.as_str())
                .map(|s| s.to_string())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    record_types.sort();

    let index_html = index_tmpl.render(context! {
        site => site_config,
        records => records_data,
        record_types => record_types,
    })?;
    fs::write(output_dir.join("index.html"), index_html)?;

    // Generate individual record pages
    let record_tmpl = env.get_template("record.html")?;
    for record in graph.all_records() {
        let mut ctx = record_to_context(record);

        // Add content as HTML using pulldown-cmark
        let content_html = markdown_to_html_with_mentions(&record.content, &valid_mentions);
        ctx.insert(
            "content_html".to_string(),
            serde_json::Value::String(content_html),
        );

        // Add links
        let links: Vec<_> = record
            .frontmatter
            .links
            .all_links()
            .iter()
            .map(|(lt, target)| {
                serde_json::json!({
                    "type": lt,
                    "target": target,
                })
            })
            .collect();
        ctx.insert("links".to_string(), serde_json::Value::Array(links));

        let record_html = record_tmpl.render(context! {
            site => site_config,
            record => ctx,
        })?;
        fs::write(
            records_dir.join(format!("{}.html", record.id())),
            record_html,
        )?;
    }

    // Generate graph page
    let graph_tmpl = env.get_template("graph.html")?;
    let graph_data = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "core": r.frontmatter.core,
            })
        }).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|e| {
            serde_json::json!({
                "source": e.from,
                "target": e.to,
                "type": e.link_type,
            })
        }).collect::<Vec<_>>(),
    });
    let graph_html = graph_tmpl.render(context! {
        site => site_config,
        graph_data => graph_data.to_string(),
    })?;
    fs::write(output_dir.join("graph.html"), graph_html)?;

    // Generate stats page
    let stats_tmpl = env.get_template("stats.html")?;
    let stats = graph.stats();
    let core_count = graph.core_records().len();

    let by_type: Vec<_> = stats
        .by_type
        .iter()
        .map(|(t, c)| serde_json::json!({ "type": t, "count": c }))
        .collect();

    let by_status: Vec<_> = stats
        .by_status
        .iter()
        .map(|(s, c)| serde_json::json!({ "status": s, "count": c }))
        .collect();

    let stats_ctx = serde_json::json!({
        "total_records": stats.total_records,
        "total_edges": stats.total_edges,
        "core": core_count,
        "by_type": by_type,
        "by_status": by_status,
    });
    let stats_html = stats_tmpl.render(context! {
        site => site_config,
        stats => stats_ctx,
    })?;
    fs::write(output_dir.join("stats.html"), stats_html)?;

    // Copy logo if specified
    if let Some(ref logo_path) = site_config.logo {
        let src = docs_dir.join(logo_path);
        if src.exists() {
            let dest = output_dir.join(logo_path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src, &dest)?;
        }
    }

    Ok(())
}

fn record_to_context(record: &crate::models::Record) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "id".to_string(),
        serde_json::Value::String(record.id().to_string()),
    );
    map.insert(
        "title".to_string(),
        serde_json::Value::String(record.title().to_string()),
    );
    map.insert(
        "type".to_string(),
        serde_json::Value::String(record.record_type().to_string()),
    );
    map.insert(
        "status".to_string(),
        serde_json::Value::String(record.status().to_string()),
    );
    map.insert(
        "created".to_string(),
        serde_json::Value::String(record.frontmatter.created.to_string()),
    );
    map.insert(
        "updated".to_string(),
        serde_json::Value::String(record.frontmatter.updated.to_string()),
    );
    map.insert(
        "core".to_string(),
        serde_json::Value::Bool(record.frontmatter.core),
    );
    map.insert(
        "tags".to_string(),
        serde_json::Value::Array(
            record
                .frontmatter
                .tags
                .iter()
                .map(|t| serde_json::Value::String(t.clone()))
                .collect(),
        ),
    );
    map.insert(
        "authors".to_string(),
        serde_json::Value::Array(
            record
                .frontmatter
                .authors
                .iter()
                .map(|a| serde_json::Value::String(a.clone()))
                .collect(),
        ),
    );
    map
}

/// Convert markdown to HTML using pulldown-cmark (without mention validation)
pub fn markdown_to_html(md: &str) -> String {
    markdown_to_html_with_mentions(md, &HashSet::new())
}

/// Convert markdown to HTML with validated @mentions
pub fn markdown_to_html_with_mentions(md: &str, valid_mentions: &HashSet<String>) -> String {
    // Strip HTML comments before rendering
    let comment_re = Regex::new(r"<!--[\s\S]*?-->").unwrap();
    let cleaned = comment_re.replace_all(md, "");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&cleaned, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Linkify @mentions (only valid ones if validation set provided)
    linkify_mentions(&html_output, valid_mentions)
}

/// Convert @username mentions to clickable links (only if user/team exists)
fn linkify_mentions(html: &str, valid_mentions: &HashSet<String>) -> String {
    let mention_re = Regex::new(r"@([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
    mention_re
        .replace_all(html, |caps: &regex::Captures| {
            let username = &caps[1];
            // Only create link if valid_mentions is empty (no validation) or username exists
            if valid_mentions.is_empty() || valid_mentions.contains(username) {
                format!(
                    r#"<a href="/users/{}" class="mention text-piper-light hover:underline">@{}</a>"#,
                    username, username
                )
            } else {
                // Keep as plain text for non-existent users
                format!("@{}", username)
            }
        })
        .to_string()
}
