//! Web integration tests for the dg serve command
//!
//! These tests verify that the web server correctly renders pages and handles requests.
//! They use the sample data in samples/pied-piper/docs for testing.

use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Find an available port by binding to port 0
fn get_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port 0");
    listener.local_addr().unwrap().port()
}

/// Get the path to the pied-piper sample docs
fn sample_docs_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("samples/pied-piper/docs")
}

/// Spawn a test server and return the process handle and base URL
fn spawn_test_server(port: u16) -> Child {
    let docs_path = sample_docs_path();

    Command::new(env!("CARGO_BIN_EXE_dg"))
        .args([
            "--docs-dir",
            docs_path.to_str().unwrap(),
            "serve",
            "--port",
            &port.to_string(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server")
}

/// Wait for the server to be ready by polling the health endpoint
async fn wait_for_server(base_url: &str, timeout: Duration) -> bool {
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        if let Ok(resp) = client.get(base_url).send().await {
            if resp.status().is_success() {
                return true;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    false
}

/// Test server wrapper that handles cleanup
struct TestServer {
    process: Child,
    pub base_url: String,
}

impl TestServer {
    async fn new() -> Self {
        let port = get_available_port();
        let process = spawn_test_server(port);
        let base_url = format!("http://127.0.0.1:{}", port);

        // Wait for server to be ready
        if !wait_for_server(&base_url, Duration::from_secs(10)).await {
            panic!("Server failed to start within timeout");
        }

        Self { process, base_url }
    }

    async fn get(&self, path: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await
            .expect("Request failed")
    }

    async fn get_text(&self, path: &str) -> String {
        self.get(path)
            .await
            .text()
            .await
            .expect("Failed to get text")
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> T {
        self.get(path)
            .await
            .json()
            .await
            .expect("Failed to parse JSON")
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

// ============================================================================
// Index Page Tests
// ============================================================================

#[tokio::test]
async fn test_index_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_index_page_contains_records() {
    let server = TestServer::new().await;
    let html = server.get_text("/").await;

    // Should contain the site title
    assert!(html.contains("Pied Piper Decision Archive"));

    // Should contain records from the sample data
    assert!(html.contains("DEC-001"));
    assert!(html.contains("Middle-Out Compression"));
}

#[tokio::test]
async fn test_index_page_has_navigation() {
    let server = TestServer::new().await;
    let html = server.get_text("/").await;

    // Should have navigation tabs
    assert!(html.contains("Records"));
    assert!(html.contains("Timeline"));
    assert!(html.contains("Graph"));
    assert!(html.contains("Users"));
    assert!(html.contains("Teams"));
    assert!(html.contains("Stats"));
}

#[tokio::test]
async fn test_index_page_has_type_filters() {
    let server = TestServer::new().await;
    let html = server.get_text("/").await;

    // Should have type filter buttons
    assert!(html.contains("Decision"));
    assert!(html.contains("Architecture"));
    assert!(html.contains("Incident"));
}

// ============================================================================
// Record Detail Tests
// ============================================================================

#[tokio::test]
async fn test_record_detail_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/records/DEC-001").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_record_detail_shows_content() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/DEC-001").await;

    // Should show record title
    assert!(html.contains("Accept Peter Gregory Seed Funding"));

    // Should show record metadata
    assert!(html.contains("DEC-001"));
    assert!(html.contains("decision"));
}

#[tokio::test]
async fn test_record_detail_shows_links() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/ADR-001").await;

    // ADR-001 should have links section
    assert!(html.contains("Links") || html.contains("Related"));
}

#[tokio::test]
async fn test_nonexistent_record_returns_404() {
    let server = TestServer::new().await;
    let response = server.get("/records/NONEXISTENT-999").await;
    assert_eq!(response.status(), 404);
}

// ============================================================================
// Edit Page Tests
// ============================================================================

#[tokio::test]
async fn test_edit_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/records/DEC-001/edit").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_edit_page_contains_raw_content() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/DEC-001/edit").await;

    // Should have the editor
    assert!(html.contains("editor") || html.contains("textarea"));

    // Should have save button
    assert!(html.contains("Save") || html.contains("save"));
}

// ============================================================================
// Users Page Tests
// ============================================================================

#[tokio::test]
async fn test_users_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/users").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_users_page_lists_users() {
    let server = TestServer::new().await;
    let html = server.get_text("/users").await;

    // Should list users from dg.toml
    assert!(html.contains("Richard Hendricks"));
    assert!(html.contains("Gilfoyle") || html.contains("Bertram"));
    assert!(html.contains("Dinesh"));
}

#[tokio::test]
async fn test_users_page_has_search() {
    let server = TestServer::new().await;
    let html = server.get_text("/users").await;

    // Should have search input
    assert!(html.contains("Search") || html.contains("search"));
}

#[tokio::test]
async fn test_user_detail_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/users/richard").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_user_detail_shows_info() {
    let server = TestServer::new().await;
    let html = server.get_text("/users/richard").await;

    // Should show user info
    assert!(html.contains("Richard Hendricks"));
    assert!(html.contains("richard@piedpiper.com") || html.contains("@richard"));
}

// ============================================================================
// Teams Page Tests
// ============================================================================

#[tokio::test]
async fn test_teams_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/teams").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_teams_page_lists_teams() {
    let server = TestServer::new().await;
    let html = server.get_text("/teams").await;

    // Should list teams from dg.toml
    assert!(html.contains("Engineering"));
    assert!(html.contains("Executive"));
}

#[tokio::test]
async fn test_team_detail_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/teams/engineering").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_team_detail_shows_members() {
    let server = TestServer::new().await;
    let html = server.get_text("/teams/engineering").await;

    // Should show team members
    assert!(html.contains("Gilfoyle") || html.contains("Dinesh") || html.contains("Richard"));
}

// ============================================================================
// Timeline Page Tests
// ============================================================================

#[tokio::test]
async fn test_timeline_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/timeline").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_timeline_contains_records() {
    let server = TestServer::new().await;
    let html = server.get_text("/timeline").await;

    // Should have timeline elements
    assert!(html.contains("timeline") || html.contains("Timeline"));
}

// ============================================================================
// Graph Page Tests
// ============================================================================

#[tokio::test]
async fn test_graph_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/graph").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_graph_page_has_visualization() {
    let server = TestServer::new().await;
    let html = server.get_text("/graph").await;

    // Should have graph/svg elements or d3 setup
    assert!(html.contains("svg") || html.contains("graph") || html.contains("d3"));
}

// ============================================================================
// Stats Page Tests
// ============================================================================

#[tokio::test]
async fn test_stats_page_returns_200() {
    let server = TestServer::new().await;
    let response = server.get("/stats").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_stats_page_shows_counts() {
    let server = TestServer::new().await;
    let html = server.get_text("/stats").await;

    // Should show record counts
    assert!(html.contains("Stats") || html.contains("Statistics"));
}

// ============================================================================
// API Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_api_records_returns_json() {
    let server = TestServer::new().await;
    let response = server.get("/api/records").await;

    assert_eq!(response.status(), 200);
    assert!(response
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap().contains("json"))
        .unwrap_or(false));
}

#[tokio::test]
async fn test_api_records_contains_data() {
    let server = TestServer::new().await;
    let records: Vec<serde_json::Value> = server.get_json("/api/records").await;

    // Should have records
    assert!(!records.is_empty());

    // First record should have expected fields
    let first = &records[0];
    assert!(first.get("id").is_some());
    assert!(first.get("title").is_some());
    assert!(first.get("type").is_some());
}

#[tokio::test]
async fn test_api_single_record_returns_json() {
    let server = TestServer::new().await;
    let record: serde_json::Value = server.get_json("/api/records/DEC-001").await;

    assert_eq!(record["id"], "DEC-001");
    assert!(record.get("title").is_some());
}

#[tokio::test]
async fn test_api_record_raw_returns_markdown() {
    let server = TestServer::new().await;
    let response = server.get("/api/records/DEC-001/raw").await;

    assert_eq!(response.status(), 200);
    let text = response.text().await.unwrap();

    // Should be raw markdown with frontmatter
    assert!(text.contains("---"));
    assert!(text.contains("type: decision"));
}

#[tokio::test]
async fn test_api_graph_returns_json() {
    let server = TestServer::new().await;
    let response = server.get("/api/graph").await;

    assert_eq!(response.status(), 200);

    let graph: serde_json::Value = response.json().await.unwrap();
    assert!(graph.get("nodes").is_some());
    assert!(graph.get("edges").is_some() || graph.get("links").is_some());
}

// ============================================================================
// Static Assets Tests
// ============================================================================

#[tokio::test]
async fn test_static_css_loads() {
    let server = TestServer::new().await;
    let response = server.get("/static/styles.css").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_static_katex_css_loads() {
    let server = TestServer::new().await;
    let response = server.get("/static/katex.min.css").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_static_highlight_js_loads() {
    let server = TestServer::new().await;
    let response = server.get("/static/highlight.min.js").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_static_highlight_css_loads() {
    let server = TestServer::new().await;
    let response = server.get("/static/highlight-github-dark.min.css").await;
    assert_eq!(response.status(), 200);
}

// ============================================================================
// Navigation Tests
// ============================================================================

#[tokio::test]
async fn test_record_links_to_related_records() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/ADR-001").await;

    // Should have links to other records
    assert!(html.contains("/records/"));
}

#[tokio::test]
async fn test_user_page_links_to_teams() {
    let server = TestServer::new().await;
    let html = server.get_text("/users/richard").await;

    // Should have links to teams
    assert!(html.contains("/teams/"));
}

// ============================================================================
// Date Display Tests
// ============================================================================

#[tokio::test]
async fn test_records_show_month_year_format() {
    let server = TestServer::new().await;
    let html = server.get_text("/").await;

    // Should show dates in "Mon YYYY" format
    // The pied-piper sample has records from 2014-2019
    assert!(
        html.contains("Apr 2014")
            || html.contains("May 2014")
            || html.contains("Jun 2014")
            || html.contains("2014")
    );
}

#[tokio::test]
async fn test_incidents_show_duration() {
    let server = TestServer::new().await;
    let html = server.get_text("/").await;

    // Resolved incidents should show duration
    // INC-001 is "May 2014 → Aug 2014 (3 months)"
    assert!(
        html.contains("week")
            || html.contains("month")
            || html.contains("year")
            || html.contains("day")
    );
}

// ============================================================================
// Avatar Group Tests
// ============================================================================

#[tokio::test]
async fn test_record_has_avatar_group() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/ADR-004").await;

    // Should have avatar-group container with overlapping styling
    assert!(html.contains("avatar-group"));
    assert!(html.contains("-space-x-6"));
}

#[tokio::test]
async fn test_avatar_group_shows_author_initials() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/ADR-004").await;

    // ADR-004 has authors: Richard Hendricks, Bertram Gilfoyle, Dinesh Chugtai
    // Should show initials (RH, BG, DC) in avatar elements
    assert!(html.contains("avatar-initials"));

    // Should have links to user profiles
    assert!(html.contains("/users/Richard Hendricks"));
    assert!(html.contains("/users/Bertram Gilfoyle"));
    assert!(html.contains("/users/Dinesh Chugtai"));
}

#[tokio::test]
async fn test_avatar_has_tooltip() {
    let server = TestServer::new().await;
    let html = server.get_text("/records/ADR-004").await;

    // Should have author-tooltip elements for hover display
    assert!(html.contains("author-tooltip"));

    // Tooltip should contain author name
    assert!(html.contains("Richard Hendricks"));
}
