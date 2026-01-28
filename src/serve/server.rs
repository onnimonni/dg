use crate::models::{graph_to_d2, AuthorsConfig, D2Renderer, Graph};
use crate::serve::config::{DgConfig, SiteConfig};
use crate::serve::generator::markdown_to_html;
use crate::serve::templates::create_environment;
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rust_embed::Embed;
use tower_http::services::ServeDir;

// Embed static assets (KaTeX CSS, JS, fonts) for offline support
#[derive(Embed)]
#[folder = "src/serve/static/"]
struct StaticAssets;
use minijinja::context;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    docs_dir: PathBuf,
    graph: RwLock<Graph>,
    site_config: SiteConfig,
    authors_config: AuthorsConfig,
}

impl AppState {
    fn reload_graph(&self) -> Result<Graph> {
        Graph::load(&self.docs_dir)
    }
}

pub async fn run_server(docs_dir: &std::path::Path, port: u16, open: bool) -> Result<()> {
    let graph = Graph::load(docs_dir)?;
    let dg_config = DgConfig::load(docs_dir)?;
    let state = Arc::new(AppState {
        docs_dir: docs_dir.to_path_buf(),
        graph: RwLock::new(graph),
        site_config: dg_config.site.clone(),
        authors_config: dg_config.authors_config(),
    });

    // Serve static assets from docs/assets
    let assets_path = docs_dir.join("assets");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/records/{id}", get(record_handler))
        .route("/records/{id}/edit", get(edit_handler))
        .route("/timeline", get(timeline_handler))
        .route("/graph", get(graph_page_handler))
        .route("/stats", get(stats_handler))
        .route("/api/records", get(api_records))
        .route(
            "/api/records/{id}",
            get(api_record).put(save_record_handler),
        )
        .route("/api/records/{id}/raw", get(api_record_raw))
        .route("/api/graph", get(api_graph))
        .route("/diagrams/{id}", get(diagram_handler))
        .route("/reload", get(reload_handler))
        // Embedded static assets (KaTeX) for offline support
        .route("/static/{*path}", get(static_handler))
        .nest_service("/assets", ServeDir::new(assets_path))
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

    // Sort records: foundational first, then by updated date (newest first)
    let mut records: Vec<_> = graph.all_records().collect();
    records.sort_by(|a, b| {
        // Foundational records first
        let a_foundational = a.frontmatter.foundational;
        let b_foundational = b.frontmatter.foundational;
        if a_foundational != b_foundational {
            return b_foundational.cmp(&a_foundational);
        }
        // Then by updated date (newest first)
        b.frontmatter.updated.cmp(&a.frontmatter.updated)
    });

    let records_data: Vec<_> = records.iter().map(|r| record_to_json(r)).collect();

    let mut type_codes: Vec<_> = records_data
        .iter()
        .filter_map(|r| {
            r.get("type")
                .and_then(|t| t.as_str())
                .map(|s| s.to_string())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    type_codes.sort();

    let record_types: Vec<_> = type_codes
        .iter()
        .map(|code| {
            serde_json::json!({
                "code": code,
                "display": type_to_display_name(code),
            })
        })
        .collect();

    match env.get_template("index.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                current_page => "records",
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
        serde_json::Value::String(markdown_to_html(&record.content)),
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

    // Resolve author info
    let resolved_authors: Vec<_> = record
        .frontmatter
        .authors
        .iter()
        .map(|username| state.authors_config.resolve(username))
        .collect();
    ctx.insert(
        "resolved_authors".to_string(),
        serde_json::to_value(&resolved_authors).unwrap_or_default(),
    );

    match env.get_template("record.html") {
        Ok(tmpl) => match tmpl.render(
            context! { site => &state.site_config, current_page => "records", record => ctx },
        ) {
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
                site => &state.site_config,
                current_page => "graph",
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

async fn timeline_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let graph = state.graph.read().await;
    let env = create_environment();

    let timeline_data = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "created": r.frontmatter.created.to_string(),
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

    match env.get_template("timeline.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                current_page => "timeline",
                timeline_data => timeline_data.to_string(),
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
        .map(|(t, c)| {
            serde_json::json!({
                "type": t,
                "type_display": type_to_display_name(t),
                "count": c
            })
        })
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
                site => &state.site_config,
                current_page => "stats",
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

// Edit page handler
async fn edit_handler(
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

    // Read raw file content
    let raw_content = match std::fs::read_to_string(&record.path) {
        Ok(content) => content,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read file: {}", e),
            )
                .into_response()
        }
    };

    match env.get_template("edit.html") {
        Ok(tmpl) => match tmpl.render(context! {
            site => &state.site_config,
            current_page => "records",
            record_id => &id,
            record_title => record.title(),
            raw_content => raw_content,
        }) {
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

// Get raw markdown content for a record
async fn api_record_raw(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    let graph = state.graph.read().await;

    match graph.get(&id) {
        Some(record) => match std::fs::read_to_string(&record.path) {
            Ok(content) => Json(serde_json::json!({
                "id": id,
                "content": content,
                "path": record.path.to_string_lossy(),
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to read file: {}", e)})),
            )
                .into_response(),
        },
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Record not found"})),
        )
            .into_response(),
    }
}

// Save record handler
async fn save_record_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let content = match payload.get("content").and_then(|c| c.as_str()) {
        Some(c) => c,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Missing 'content' field"})),
            )
                .into_response()
        }
    };

    // Get current record path
    let record_path = {
        let graph = state.graph.read().await;
        match graph.get(&id) {
            Some(record) => record.path.clone(),
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"error": "Record not found"})),
                )
                    .into_response()
            }
        }
    };

    // Validate the new content by trying to parse it
    use crate::models::Record;
    let temp_path = record_path.with_extension("md.tmp");
    if let Err(e) = std::fs::write(&temp_path, content) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to write temp file: {}", e)})),
        )
            .into_response();
    }

    match Record::parse(&temp_path) {
        Ok(parsed) => {
            // Verify the ID matches
            if parsed.id() != id {
                let _ = std::fs::remove_file(&temp_path);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": format!("Record ID mismatch: expected '{}', got '{}'", id, parsed.id())
                    })),
                )
                    .into_response();
            }

            // Content is valid, move temp file to actual file
            if let Err(e) = std::fs::rename(&temp_path, &record_path) {
                let _ = std::fs::remove_file(&temp_path);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to save file: {}", e)})),
                )
                    .into_response();
            }

            // Reload graph
            match state.reload_graph() {
                Ok(new_graph) => {
                    let mut graph = state.graph.write().await;
                    *graph = new_graph;
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            serde_json::json!({"error": format!("Failed to reload graph: {}", e)}),
                        ),
                    )
                        .into_response();
                }
            }

            Json(serde_json::json!({
                "status": "saved",
                "id": id,
                "title": parsed.title(),
            }))
            .into_response()
        }
        Err(e) => {
            let _ = std::fs::remove_file(&temp_path);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid record format: {}", e),
                    "details": "Please check your frontmatter YAML syntax"
                })),
            )
                .into_response()
        }
    }
}

// Static asset handler using rust-embed for offline KaTeX support
async fn static_handler(Path(path): Path<String>) -> Response {
    let content_type = if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".ttf") {
        "font/ttf"
    } else {
        "application/octet-stream"
    };

    match StaticAssets::get(&path) {
        Some(content) => (
            [(header::CONTENT_TYPE, content_type)],
            content.data.into_owned(),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

fn type_to_display_name(type_code: &str) -> String {
    match type_code {
        "DEC" => "Decision".to_string(),
        "STR" => "Strategy".to_string(),
        "POL" => "Policy".to_string(),
        "CUS" => "Customer".to_string(),
        "OPP" => "Opportunity".to_string(),
        "PRC" => "Process".to_string(),
        "HIR" => "Hiring".to_string(),
        "ADR" => "Architecture Decision".to_string(),
        "INC" => "Incident".to_string(),
        "RUN" => "Runbook".to_string(),
        "MTG" => "Meeting".to_string(),
        other => other.to_string(),
    }
}

fn record_to_json(record: &crate::models::Record) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    let type_code = record.record_type().to_string();
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
        serde_json::Value::String(type_code.clone()),
    );
    map.insert(
        "type_display".to_string(),
        serde_json::Value::String(type_to_display_name(&type_code).to_string()),
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
