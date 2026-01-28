use crate::models::{graph_to_d2, D2Renderer, Graph};
use crate::serve::templates::create_environment;
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use minijinja::context;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    docs_dir: PathBuf,
    graph: RwLock<Graph>,
}

impl AppState {
    fn reload_graph(&self) -> Result<Graph> {
        Graph::load(&self.docs_dir)
    }
}

pub async fn run_server(docs_dir: &std::path::Path, port: u16, open: bool) -> Result<()> {
    let graph = Graph::load(docs_dir)?;
    let state = Arc::new(AppState {
        docs_dir: docs_dir.to_path_buf(),
        graph: RwLock::new(graph),
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/records/{id}", get(record_handler))
        .route("/graph", get(graph_page_handler))
        .route("/stats", get(stats_handler))
        .route("/api/records", get(api_records))
        .route("/api/records/{id}", get(api_record))
        .route("/api/graph", get(api_graph))
        .route("/diagrams/{id}", get(diagram_handler))
        .route("/reload", get(reload_handler))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("Server running at http://{}", addr);

    if open {
        let _ = open_browser(&format!("http://{}", addr));
    }

    axum::serve(listener, app).await?;
    Ok(())
}

fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()?;
    }
    Ok(())
}

async fn index_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let graph = state.graph.read().await;
    let env = create_environment();

    let records_data: Vec<_> = graph.all_records().map(|r| record_to_json(r)).collect();

    let mut record_types: Vec<_> = records_data
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

    match env.get_template("index.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                records => records_data,
                record_types => record_types,
            }) {
                Ok(html) => Html(html).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Render error: {}", e),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", e),
        )
            .into_response(),
    }
}

async fn record_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let graph = state.graph.read().await;

    let record = match graph.get(&id) {
        Some(r) => r,
        None => {
            return (StatusCode::NOT_FOUND, format!("Record not found: {}", id)).into_response()
        }
    };

    let env = create_environment();
    let mut ctx = record_to_json(record);

    // Add content as HTML
    ctx.insert(
        "content_html".to_string(),
        serde_json::Value::String(simple_markdown_to_html(&record.content)),
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

    match env.get_template("record.html") {
        Ok(tmpl) => match tmpl.render(context! { record => ctx }) {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Render error: {}", e),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", e),
        )
            .into_response(),
    }
}

async fn graph_page_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let graph = state.graph.read().await;
    let env = create_environment();

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

    match env.get_template("graph.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                graph_data => graph_data.to_string(),
            }) {
                Ok(html) => Html(html).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Render error: {}", e),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", e),
        )
            .into_response(),
    }
}

async fn stats_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let graph = state.graph.read().await;
    let env = create_environment();
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

    match env.get_template("stats.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                stats => stats_ctx,
            }) {
                Ok(html) => Html(html).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Render error: {}", e),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {}", e),
        )
            .into_response(),
    }
}

async fn api_records(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let graph = state.graph.read().await;

    let mut records: Vec<_> = graph.all_records().collect();

    // Filter by type
    if let Some(record_type) = params.get("type") {
        records.retain(|r| r.record_type().to_string() == *record_type);
    }

    // Filter by status
    if let Some(status) = params.get("status") {
        records.retain(|r| r.status().to_string() == *status);
    }

    // Filter by tag
    if let Some(tag) = params.get("tag") {
        let tag_lower = tag.to_lowercase();
        records.retain(|r| {
            r.frontmatter
                .tags
                .iter()
                .any(|t| t.to_lowercase() == tag_lower)
        });
    }

    let output: Vec<_> = records.iter().map(|r| record_to_json(r)).collect();
    Json(output)
}

async fn api_record(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    let graph = state.graph.read().await;

    match graph.get(&id) {
        Some(record) => {
            let mut data = record_to_json(record);
            data.insert(
                "content".to_string(),
                serde_json::Value::String(record.content.clone()),
            );

            let links: Vec<_> = record
                .frontmatter
                .links
                .all_links()
                .iter()
                .map(|(lt, target)| serde_json::json!({ "type": lt, "target": target }))
                .collect();
            data.insert("links".to_string(), serde_json::Value::Array(links));

            Json(data).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not found"})),
        )
            .into_response(),
    }
}

async fn api_graph(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let graph = state.graph.read().await;

    let output = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "status": r.status().to_string(),
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

    Json(output)
}

async fn diagram_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let graph = state.graph.read().await;

    // Check if record exists
    if graph.get(&id).is_none() {
        return (StatusCode::NOT_FOUND, "Record not found").into_response();
    }

    let depth = params
        .get("depth")
        .and_then(|d| d.parse().ok())
        .unwrap_or(2);

    let subset = graph.neighbors(&id, depth);
    let d2_source = graph_to_d2(&graph, Some(&subset));

    match params.get("format").map(|s| s.as_str()) {
        Some("d2") => d2_source.into_response(),
        _ => {
            // Default to SVG
            match D2Renderer::new() {
                Ok(renderer) => match renderer.render_svg(&d2_source) {
                    Ok(svg) => {
                        ([(axum::http::header::CONTENT_TYPE, "image/svg+xml")], svg).into_response()
                    }
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Render error: {}", e),
                    )
                        .into_response(),
                },
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("D2 not available: {}", e),
                )
                    .into_response(),
            }
        }
    }
}

async fn reload_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.reload_graph() {
        Ok(new_graph) => {
            let mut graph = state.graph.write().await;
            *graph = new_graph;
            Json(serde_json::json!({"status": "reloaded"}))
        }
        Err(e) => Json(serde_json::json!({"error": e.to_string()})),
    }
}

fn record_to_json(record: &crate::models::Record) -> serde_json::Map<String, serde_json::Value> {
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

        if line.trim().is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            continue;
        }

        if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", escape_html(&line[4..])));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", escape_html(&line[3..])));
        } else if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", escape_html(&line[2..])));
        } else if line.starts_with("- ") || line.starts_with("* ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", escape_html(&line[2..])));
        } else {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str(&format!("<p>{}</p>\n", escape_html(line)));
        }
    }

    if in_list {
        html.push_str("</ul>\n");
    }
    if in_code_block {
        html.push_str("</code></pre>\n");
    }

    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
