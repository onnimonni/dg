use crate::models::Graph;
use crate::serve::templates::create_environment;
use anyhow::Result;
use minijinja::context;
use std::fs;
use std::path::Path;

pub fn generate_site(graph: &Graph, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    let records_dir = output_dir.join("records");
    fs::create_dir_all(&records_dir)?;

    let env = create_environment();

    // Generate index page
    let index_tmpl = env.get_template("index.html")?;
    let records_data: Vec<_> = graph.all_records().map(|r| record_to_context(r)).collect();

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
        records => records_data,
        record_types => record_types,
    })?;
    fs::write(output_dir.join("index.html"), index_html)?;

    // Generate individual record pages
    let record_tmpl = env.get_template("record.html")?;
    for record in graph.all_records() {
        let mut ctx = record_to_context(record);

        // Add content as HTML (basic markdown conversion)
        let content_html = simple_markdown_to_html(&record.content);
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

        let record_html = record_tmpl.render(context! { record => ctx })?;
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
                "foundational": r.frontmatter.foundational,
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
        graph_data => graph_data.to_string(),
    })?;
    fs::write(output_dir.join("graph.html"), graph_html)?;

    // Generate stats page
    let stats_tmpl = env.get_template("stats.html")?;
    let stats = graph.stats();
    let foundational_count = graph.foundational_records().len();

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
        "foundational": foundational_count,
        "by_type": by_type,
        "by_status": by_status,
    });
    let stats_html = stats_tmpl.render(context! {
        stats => stats_ctx,
    })?;
    fs::write(output_dir.join("stats.html"), stats_html)?;

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
        "foundational".to_string(),
        serde_json::Value::Bool(record.frontmatter.foundational),
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

fn simple_markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_list = false;

    for line in md.lines() {
        // Code blocks
        if line.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&escape_html(line));
            html.push('\n');
            continue;
        }

        // Empty line
        if line.trim().is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str("<br>\n");
            continue;
        }

        // Headers
        if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", escape_html(&line[4..])));
            continue;
        }
        if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", escape_html(&line[3..])));
            continue;
        }
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", escape_html(&line[2..])));
            continue;
        }

        // List items
        if line.starts_with("- ") || line.starts_with("* ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", inline_markdown(&line[2..])));
            continue;
        }

        // Regular paragraph
        if in_list {
            html.push_str("</ul>\n");
            in_list = false;
        }
        html.push_str(&format!("<p>{}</p>\n", inline_markdown(line)));
    }

    if in_list {
        html.push_str("</ul>\n");
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }

    html
}

fn inline_markdown(text: &str) -> String {
    let mut result = escape_html(text);

    // Bold
    let re = regex::Regex::new(r"\*\*(.+?)\*\*").unwrap();
    result = re.replace_all(&result, "<strong>$1</strong>").to_string();

    // Italic
    let re = regex::Regex::new(r"\*(.+?)\*").unwrap();
    result = re.replace_all(&result, "<em>$1</em>").to_string();

    // Code
    let re = regex::Regex::new(r"`(.+?)`").unwrap();
    result = re.replace_all(&result, "<code>$1</code>").to_string();

    // Links
    let re = regex::Regex::new(r"\[(.+?)\]\((.+?)\)").unwrap();
    result = re
        .replace_all(&result, r#"<a href="$2">$1</a>"#)
        .to_string();

    result
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
