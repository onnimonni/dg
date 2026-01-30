use crate::models::d2::D2Renderer;
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
use std::sync::OnceLock;

// Lazy-initialized D2 renderer (None if d2 binary not available)
static D2_RENDERER: OnceLock<Option<D2Renderer>> = OnceLock::new();

fn get_d2_renderer() -> Option<&'static D2Renderer> {
    D2_RENDERER.get_or_init(|| D2Renderer::new().ok()).as_ref()
}

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

    // Render D2 code blocks to SVG (server-side)
    let html_output = render_d2_blocks(&html_output);

    // Linkify @mentions (only valid ones if validation set provided)
    linkify_mentions(&html_output, valid_mentions)
}

/// Render D2 code blocks to inline SVG
fn render_d2_blocks(html: &str) -> String {
    // Match <pre><code class="language-d2">...</code></pre> blocks
    let d2_re = Regex::new(r#"<pre><code class="language-d2">([\s\S]*?)</code></pre>"#).unwrap();

    d2_re
        .replace_all(html, |caps: &regex::Captures| {
            let d2_code = &caps[1];
            // Unescape HTML entities in the code
            let unescaped = html_unescape(d2_code);

            match get_d2_renderer() {
                Some(renderer) => match renderer.render_svg(&unescaped) {
                    Ok(svg) => {
                        format!(
                            r#"<div class="d2-container my-4 bg-slate-800 rounded-lg p-4 overflow-x-auto">{}</div>"#,
                            svg
                        )
                    }
                    Err(e) => {
                        // Render error: show code with error message
                        let escaped_code = htmlescape::encode_minimal(&unescaped);
                        let escaped_err = htmlescape::encode_minimal(&e.to_string());
                        format!(
                            r#"<div class="d2-error my-4">
                                <div class="text-red-400 text-sm mb-2">D2 render error: {}</div>
                                <pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto"><code class="language-d2">{}</code></pre>
                            </div>"#,
                            escaped_err, escaped_code
                        )
                    }
                },
                None => {
                    // D2 not available: show code with info message
                    let escaped_code = htmlescape::encode_minimal(&unescaped);
                    format!(
                        r#"<div class="d2-unavailable my-4">
                            <div class="text-slate-500 text-sm mb-2">D2 not installed. <a href="https://d2lang.com" class="text-piper-light hover:underline" target="_blank">Install d2</a> to render this diagram.</div>
                            <pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto"><code class="language-d2">{}</code></pre>
                        </div>"#,
                        escaped_code
                    )
                }
            }
        })
        .to_string()
}

/// Unescape common HTML entities
fn html_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2_code_block_rendering() {
        let md = r#"# Test

```d2
a -> b
```

After.
"#;
        let html = markdown_to_html(md);

        // Should contain d2-container if D2 is available, or d2-unavailable if not
        assert!(
            html.contains("d2-container") || html.contains("d2-unavailable"),
            "Expected d2 container in output, got: {}",
            html
        );
        // Should NOT contain raw code block with language-d2 class unchanged
        // (it should be wrapped in a d2-container or d2-unavailable div)
        assert!(
            html.contains("d2-") && !html.contains("<pre><code class=\"language-d2\">a -&gt; b"),
            "D2 code should be processed, not left as raw code block"
        );
    }

    #[test]
    fn test_non_d2_code_blocks_preserved() {
        let md = r#"# Test

```rust
fn main() {}
```

After.
"#;
        let html = markdown_to_html(md);

        // Rust code block should be preserved as-is
        assert!(
            html.contains("language-rust"),
            "Rust code block should be preserved"
        );
    }

    #[test]
    fn test_mermaid_code_blocks_preserved() {
        let md = r#"# Test

```mermaid
graph TD
A --> B
```

After.
"#;
        let html = markdown_to_html(md);

        // Mermaid should be preserved for client-side rendering
        assert!(
            html.contains("language-mermaid"),
            "Mermaid code block should be preserved for client-side rendering"
        );
    }

    #[test]
    fn test_html_unescape() {
        assert_eq!(html_unescape("a &lt; b"), "a < b");
        assert_eq!(html_unescape("a &gt; b"), "a > b");
        assert_eq!(html_unescape("a &amp; b"), "a & b");
        assert_eq!(html_unescape("&quot;quoted&quot;"), "\"quoted\"");
    }
}
