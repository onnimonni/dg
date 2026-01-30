use crate::git::GitHistory;
use crate::models::teams::TeamsConfig;
use crate::models::users::UsersConfig;
use crate::models::{graph_to_d2, AuthorsConfig, D2Renderer, Graph};
use crate::serve::config::{DgConfig, SiteConfig};
use crate::serve::generator::markdown_to_html_with_mentions;
use crate::serve::templates::create_environment;
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use rust_embed::Embed;
use tower_http::services::ServeDir;

// Embed static assets (KaTeX CSS, JS, fonts) for offline support
// Excludes build-time tools (tailwindcss binary, daisyui.mjs)
#[derive(Embed)]
#[folder = "src/serve/static/"]
#[exclude = "tailwindcss"]
#[exclude = "daisyui*.mjs"]
#[exclude = "input.css"]
struct StaticAssets;
use minijinja::context;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct AppState {
    docs_dir: PathBuf,
    graph: RwLock<Graph>,
    site_config: SiteConfig,
    authors_config: AuthorsConfig,
    users_config: UsersConfig,
    teams_config: TeamsConfig,
    valid_mentions: std::collections::HashSet<String>,
}

impl AppState {
    fn reload_graph(&self) -> Result<Graph> {
        Graph::load(&self.docs_dir)
    }

    fn has_users(&self) -> bool {
        !self.users_config.users.is_empty()
    }
}

pub async fn run_server(
    docs_dir: &std::path::Path,
    port: u16,
    open: bool,
    watch: bool,
) -> Result<()> {
    let graph = Graph::load(docs_dir)?;
    let dg_config = DgConfig::load(docs_dir)?;
    let valid_mentions: std::collections::HashSet<String> = dg_config
        .users
        .keys()
        .chain(dg_config.teams.keys())
        .cloned()
        .collect();
    let state = Arc::new(AppState {
        docs_dir: docs_dir.to_path_buf(),
        graph: RwLock::new(graph),
        site_config: dg_config.site.clone(),
        authors_config: dg_config.authors_config(),
        users_config: dg_config.users_config(),
        teams_config: dg_config.teams_config(),
        valid_mentions,
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
        .route("/users", get(users_handler))
        .route("/users/{username}", get(user_handler))
        .route("/teams", get(teams_handler))
        .route("/teams/{id}", get(team_handler))
        .route("/teams/{id}/history", get(team_history_handler))
        .route("/api/records", get(api_records))
        .route(
            "/api/records/{id}",
            get(api_record).put(save_record_handler),
        )
        .route("/api/records/{id}/raw", get(api_record_raw))
        .route("/api/render", axum::routing::post(api_render))
        .route("/api/graph", get(api_graph))
        .route("/diagrams/{id}", get(diagram_handler))
        .route("/reload", get(reload_handler))
        // Embedded static assets (KaTeX) for offline support
        .route("/static/{*path}", get(static_handler))
        .nest_service("/assets", ServeDir::new(assets_path))
        .with_state(state.clone());

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("Server running at http://{}", addr);
    if watch {
        println!("Watching for file changes...");
    }

    if open {
        let _ = open_browser(&format!("http://{}", addr));
    }

    // Start file watcher if enabled
    if watch {
        let watch_state = state.clone();
        let watch_dir = docs_dir.to_path_buf();
        tokio::spawn(async move {
            if let Err(e) = run_file_watcher(watch_dir, watch_state).await {
                eprintln!("File watcher error: {}", e);
            }
        });
    }

    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_file_watcher(docs_dir: PathBuf, state: Arc<AppState>) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                // Only trigger on modify/create/delete events for .md files
                let dominated_paths: Vec<_> = event
                    .paths
                    .iter()
                    .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
                    .collect();
                if !dominated_paths.is_empty() {
                    let _ = tx.blocking_send(());
                }
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
    )?;

    // Watch the decisions directory
    let decisions_dir = docs_dir.join("decisions");
    if decisions_dir.exists() {
        watcher.watch(&decisions_dir, RecursiveMode::Recursive)?;
    }

    // Debounce: wait a bit after changes to batch multiple saves
    let mut last_reload = std::time::Instant::now();
    let debounce_duration = Duration::from_millis(500);

    while rx.recv().await.is_some() {
        let now = std::time::Instant::now();
        if now.duration_since(last_reload) > debounce_duration {
            last_reload = now;
            if let Ok(new_graph) = state.reload_graph() {
                let mut graph = state.graph.write().await;
                *graph = new_graph;
                println!("  ↻ Reloaded graph");
            }
        }
    }

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

    // Sort records by updated date (newest first)
    let mut records: Vec<_> = graph.all_records().collect();
    records.sort_by(|a, b| b.frontmatter.updated.cmp(&a.frontmatter.updated));

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
                has_users => state.has_users(),
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
        serde_json::Value::String(markdown_to_html_with_mentions(
            &record.content,
            &state.valid_mentions,
        )),
    );

    // Add links (outgoing)
    let links: Vec<_> = record
        .frontmatter
        .links
        .all_links()
        .iter()
        .map(|(lt, target)| {
            let title = graph.get(target).map(|r| r.title().to_string());
            serde_json::json!({
                "type": lt,
                "target": target,
                "title": title,
            })
        })
        .collect();
    ctx.insert("links".to_string(), serde_json::Value::Array(links));

    // Add backlinks (incoming) - "Referenced by" section
    let backlinks: Vec<_> = graph
        .incoming_edges(&id)
        .iter()
        .map(|edge| {
            let title = graph.get(&edge.from).map(|r| r.title().to_string());
            serde_json::json!({
                "type": &edge.link_type,
                "source": &edge.from,
                "title": title,
            })
        })
        .collect();
    ctx.insert("backlinks".to_string(), serde_json::Value::Array(backlinks));

    // Resolve author info with team memberships
    let resolved_authors: Vec<_> = record
        .frontmatter
        .authors
        .iter()
        .map(|username| {
            let base = state.authors_config.resolve(username);
            // Check if we have user config - prefer user name/email over authors config
            let user = state.users_config.get(username);
            let name = user
                .map(|u| u.display_name(username))
                .unwrap_or_else(|| base.name.clone());
            let email = user
                .and_then(|u| u.email.clone())
                .or_else(|| base.email.clone());
            let teams: Vec<String> = user.map(|u| u.teams.clone()).unwrap_or_default();
            let avatar_url = user
                .map(|u| u.avatar(username))
                .unwrap_or_else(|| base.avatar_url.clone());
            let initials = user
                .map(|u| u.initials(username))
                .unwrap_or_else(|| base.initials.clone());
            serde_json::json!({
                "username": username,
                "name": name,
                "email": email,
                "avatar_url": avatar_url,
                "initials": initials,
                "teams": teams,
            })
        })
        .collect();
    ctx.insert(
        "resolved_authors".to_string(),
        serde_json::Value::Array(resolved_authors),
    );

    match env.get_template("record.html") {
        Ok(tmpl) => match tmpl.render(
            context! { site => &state.site_config, has_users => state.has_users(), current_page => "records", record => ctx },
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
            // Resolve author names
            let authors: Vec<String> = r.frontmatter.authors.iter().map(|username| {
                state.users_config.get(username)
                    .map(|u| u.display_name(username))
                    .unwrap_or_else(|| username.to_string())
            }).collect();
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "type_name": r.record_type().display_name(),
                "date": r.frontmatter.created.to_string(),
                "authors": authors,
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

    match env.get_template("graph.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
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

    match env.get_template("timeline.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
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
    let core_count = graph.core_records().len();

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
        "core": core_count,
        "by_type": by_type,
        "by_status": by_status,
    });

    match env.get_template("stats.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
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

// Server-side markdown rendering endpoint (prevents XSS from client-side regex parsing)
async fn api_render(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let markdown = match payload.get("markdown").and_then(|m| m.as_str()) {
        Some(m) => m,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Missing 'markdown' field"})),
            )
                .into_response()
        }
    };

    let html = markdown_to_html_with_mentions(markdown, &state.valid_mentions);
    Json(serde_json::json!({ "html": html })).into_response()
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
            has_users => state.has_users(),
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

// Users list handler
async fn users_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let env = create_environment();

    let mut users: Vec<_> = state
        .users_config
        .users
        .iter()
        .map(|(username, user)| {
            serde_json::json!({
                "username": username,
                "name": user.display_name(username),
                "initials": user.initials(username),
                "avatar_url": user.avatar(username),
                "email": user.email,
                "teams": user.teams,
                "roles": user.roles,
                "is_deprecated": user.is_deprecated(),
                "is_llm": user.roles.contains(&"llm".to_string()),
            })
        })
        .collect();

    users.sort_by(|a, b| {
        let a_name = a["name"].as_str().unwrap_or("");
        let b_name = b["name"].as_str().unwrap_or("");
        a_name.cmp(b_name)
    });

    match env.get_template("users.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
                current_page => "users",
                users => users,
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

// Single user handler
async fn user_handler(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let env = create_environment();

    let user = match state.users_config.get(&username) {
        Some(u) => u,
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("User not found: {}", username),
            )
                .into_response()
        }
    };

    let user_data = serde_json::json!({
        "username": &username,
        "name": user.display_name(&username),
        "initials": user.initials(&username),
        "avatar_url": user.avatar(&username),
        "email": user.email,
        "github": user.github,
        "teams": user.teams,
        "roles": user.roles,
        "is_deprecated": user.is_deprecated(),
        "is_llm": user.roles.contains(&"llm".to_string()),
        "deprecated_date": user.deprecated_date,
        "deprecated_note": user.deprecated_note,
    });

    // Build combined records list with authorship and DACI roles
    let graph = state.graph.read().await;
    let mention_pattern = format!("@{}", username);
    let user_display_name = user.display_name(&username);

    // Collect all records where user is author or has DACI role
    let mut user_records: Vec<serde_json::Value> = Vec::new();
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    for record in graph.all_records() {
        let is_author = record.frontmatter.authors.contains(&username);

        // Check DACI roles
        let roles = record.extract_daci_roles();
        let mut user_daci_role: Option<String> = None;

        for (role, names) in &roles {
            let is_assigned = names.iter().any(|name| {
                let name_lower = name.to_lowercase();
                name_lower.contains(&username.to_lowercase())
                    || name_lower.contains(&user_display_name.to_lowercase())
                    || user_display_name.to_lowercase().contains(&name_lower)
            });

            if is_assigned {
                user_daci_role = Some(role.clone());
                break;
            }
        }

        // Include if author or has DACI role
        if is_author || user_daci_role.is_some() {
            if !seen_ids.contains(record.id()) {
                seen_ids.insert(record.id().to_string());
                user_records.push(serde_json::json!({
                    "id": record.id(),
                    "title": record.title(),
                    "status": record.status().to_string(),
                    "date": record.frontmatter.created.to_string(),
                    "is_author": is_author,
                    "daci_role": user_daci_role,
                    "core": record.frontmatter.core,
                }));
            }
        }
    }

    // Sort by date descending
    user_records.sort_by(|a, b| {
        let date_a = a.get("date").and_then(|d| d.as_str()).unwrap_or("");
        let date_b = b.get("date").and_then(|d| d.as_str()).unwrap_or("");
        date_b.cmp(date_a)
    });

    // Find records that mention this user (but not authored by them)
    let mentioned_in: Vec<_> = graph
        .all_records()
        .filter(|r| {
            r.content.contains(&mention_pattern) && !r.frontmatter.authors.contains(&username)
        })
        .map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "status": r.status().to_string(),
            })
        })
        .collect();

    // Find action items assigned to this user
    let mut action_items: Vec<serde_json::Value> = Vec::new();

    for record in graph.all_records() {
        for (text, completed, owner) in record.extract_action_items() {
            // Check if this action is assigned to the user
            if let Some(ref owner_name) = owner {
                if owner_name.to_lowercase() == username.to_lowercase() {
                    action_items.push(serde_json::json!({
                        "record_id": record.id(),
                        "record_title": record.title(),
                        "text": text,
                        "completed": completed,
                    }));
                }
            }
        }
    }

    match env.get_template("user.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
                current_page => "users",
                user => user_data,
                user_records => user_records,
                mentioned_in => mentioned_in,
                action_items => action_items,
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

// Teams list handler
async fn teams_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let env = create_environment();

    let mut teams: Vec<_> = state
        .teams_config
        .teams
        .iter()
        .map(|(id, team)| {
            // Count members
            let member_count = state
                .users_config
                .users
                .values()
                .filter(|u| u.teams.contains(id))
                .count();

            serde_json::json!({
                "id": id,
                "name": team.name,
                "lead": team.lead,
                "parent": team.parent,
                "member_count": member_count,
            })
        })
        .collect();

    teams.sort_by(|a, b| {
        let a_name = a["name"].as_str().unwrap_or("");
        let b_name = b["name"].as_str().unwrap_or("");
        a_name.cmp(b_name)
    });

    match env.get_template("teams.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
                current_page => "teams",
                teams => teams,
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

// Single team handler
async fn team_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let env = create_environment();

    let team = match state.teams_config.get(&id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, format!("Team not found: {}", id)).into_response(),
    };

    let team_data = serde_json::json!({
        "id": &id,
        "name": team.name,
        "lead": team.lead,
        "parent": team.parent,
        "description": team.description,
        "email": team.email,
    });

    // Find team members
    let members: Vec<_> = state
        .users_config
        .users
        .iter()
        .filter(|(_, u)| u.teams.contains(&id))
        .map(|(username, user)| {
            serde_json::json!({
                "username": username,
                "name": user.display_name(username),
                "avatar_url": user.avatar(username),
            })
        })
        .collect();

    // Find sub-teams
    let sub_teams: Vec<_> = state
        .teams_config
        .children(&id)
        .iter()
        .map(|(child_id, child)| {
            serde_json::json!({
                "id": child_id,
                "name": child.name,
                "lead": child.lead,
            })
        })
        .collect();

    match env.get_template("team.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
                current_page => "teams",
                team => team_data,
                members => members,
                sub_teams => sub_teams,
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

async fn team_history_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let env = create_environment();

    let team = match state.teams_config.get(&id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, format!("Team not found: {}", id)).into_response(),
    };

    // Try to get git history
    let history = match GitHistory::new(&state.docs_dir) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Git history not available (not a git repository)",
            )
                .into_response()
        }
    };

    let snapshots = match history.team_history(&id) {
        Ok(s) => s,
        Err(_) => vec![],
    };

    let history_data: Vec<_> = snapshots
        .iter()
        .map(|s| {
            serde_json::json!({
                "commit": s.commit,
                "date": s.date.format("%Y-%m-%d").to_string(),
                "message": s.message,
                "members": s.members,
                "joined": s.joined,
                "left": s.left,
            })
        })
        .collect();

    // Get all-time members
    let all_time_members = history.all_time_members(&id).unwrap_or_default();

    match env.get_template("team_history.html") {
        Ok(tmpl) => {
            match tmpl.render(context! {
                site => &state.site_config,
                has_users => state.has_users(),
                current_page => "teams",
                team_id => &id,
                team_name => &team.name,
                history => history_data,
                all_time_members => all_time_members,
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
        "ADR" => "Architecture".to_string(),
        "INC" => "Incident".to_string(),
        "RUN" => "Runbook".to_string(),
        "MTG" => "Meeting".to_string(),
        "FBK" => "Feedback".to_string(),
        "LEG" => "Legal".to_string(),
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
    let created_str = record.frontmatter.created.to_string();
    let updated_str = record.frontmatter.updated.to_string();
    map.insert(
        "created".to_string(),
        serde_json::Value::String(created_str.clone()),
    );
    map.insert(
        "updated".to_string(),
        serde_json::Value::String(updated_str.clone()),
    );
    // Extract years for period display (e.g., "1980 → 1990" for deprecated records)
    map.insert(
        "created_year".to_string(),
        serde_json::Value::String(
            created_str
                .split('-')
                .next()
                .unwrap_or(&created_str)
                .to_string(),
        ),
    );
    map.insert(
        "updated_year".to_string(),
        serde_json::Value::String(
            updated_str
                .split('-')
                .next()
                .unwrap_or(&updated_str)
                .to_string(),
        ),
    );
    map.insert(
        "core".to_string(),
        serde_json::Value::Bool(record.frontmatter.core),
    );
    // Check if this is a draft record (ID contains -NEW-)
    map.insert(
        "is_draft".to_string(),
        serde_json::Value::Bool(record.id().contains("-NEW-")),
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
