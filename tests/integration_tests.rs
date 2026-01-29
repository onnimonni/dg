use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test environment with dg initialized
fn setup_test_env() -> TempDir {
    let temp = TempDir::new().unwrap();
    let docs_dir = temp.path().join("docs");

    // Run dg init
    Command::cargo_bin("dg")
        .unwrap()
        .args(["-D", docs_dir.to_str().unwrap(), "init"])
        .assert()
        .success();

    temp
}

/// Helper to run dg command in test environment
fn dg_cmd(temp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("dg").unwrap();
    cmd.args(["-D", temp.path().join("docs").to_str().unwrap()]);
    cmd
}

// ============================================================================
// Init Tests
// ============================================================================

#[test]
fn test_init_creates_directories() {
    let temp = TempDir::new().unwrap();
    let docs_dir = temp.path().join("docs");

    Command::cargo_bin("dg")
        .unwrap()
        .args(["-D", docs_dir.to_str().unwrap(), "init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Decision Graph initialized"));

    assert!(docs_dir.join(".decisions").exists());
    assert!(docs_dir.join(".templates").exists());
    assert!(docs_dir.join(".templates/decision.md").exists());
    assert!(docs_dir.join(".templates/adr.md").exists());
}

#[test]
fn test_init_idempotent() {
    let temp = setup_test_env();

    // Running init again should not fail
    dg_cmd(&temp).arg("init").assert().success();
}

// ============================================================================
// New Record Tests
// ============================================================================

#[test]
fn test_new_decision() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test Decision"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"));

    let file = temp.path().join("docs/.decisions/DEC-001-test-decision.md");
    assert!(file.exists());

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("type: decision"));
    assert!(content.contains("id: DEC-001"));
    assert!(content.contains("Test Decision"));
}

#[test]
fn test_new_adr() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "adr", "Use PostgreSQL"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ADR-001"));

    let file = temp
        .path()
        .join("docs/.decisions/ADR-001-use-postgresql.md");
    assert!(file.exists());
}

#[test]
fn test_new_multiple_records_increment_id() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "First"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"));

    dg_cmd(&temp)
        .args(["new", "decision", "Second"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-002"));

    dg_cmd(&temp)
        .args(["new", "decision", "Third"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-003"));
}

#[test]
fn test_new_with_special_characters_in_title() {
    let temp = setup_test_env();

    // Colons should be handled (YAML special char)
    dg_cmd(&temp)
        .args(["new", "adr", "ADR: Use Redis for Caching"])
        .assert()
        .success();

    let file = temp
        .path()
        .join("docs/.decisions/ADR-001-adr-use-redis-for-caching.md");
    assert!(file.exists());

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("ADR: Use Redis for Caching"));
}

// ============================================================================
// List Tests
// ============================================================================

#[test]
fn test_list_empty() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No records found"));
}

#[test]
fn test_list_with_records() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"))
        .stdout(predicate::str::contains("1 records"));
}

#[test]
fn test_list_filter_by_type() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Dec"])
        .assert()
        .success();
    dg_cmd(&temp).args(["new", "adr", "Adr"]).assert().success();

    dg_cmd(&temp)
        .args(["list", "-t", "decision"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"))
        .stdout(predicate::str::contains("1 records"));

    dg_cmd(&temp)
        .args(["list", "-t", "adr"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ADR-001"))
        .stdout(predicate::str::contains("1 records"));
}

#[test]
fn test_list_json_format() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["list", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"DEC-001\""));
}

// ============================================================================
// Show Tests
// ============================================================================

#[test]
fn test_show_record() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test Decision"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["show", "DEC-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Decision"))
        .stdout(predicate::str::contains("Type: DEC"));
}

#[test]
fn test_show_nonexistent() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["show", "DEC-999"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ============================================================================
// Link Tests
// ============================================================================

#[test]
fn test_link_records() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "strategy", "Strategy"])
        .assert()
        .success();
    dg_cmd(&temp)
        .args(["new", "decision", "Decision"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["link", "DEC-001", "depends_on", "STR-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Linked"));

    // Verify link exists in file
    let file = temp.path().join("docs/.decisions/DEC-001-decision.md");
    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("STR-001"));
}

#[test]
fn test_link_supersedes_creates_inverse() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Old"])
        .assert()
        .success();
    dg_cmd(&temp)
        .args(["new", "decision", "New"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["link", "DEC-002", "supersedes", "DEC-001"])
        .assert()
        .success()
        .stdout(predicate::str::contains("inverse"));

    // Check DEC-001 has superseded_by
    let file = temp.path().join("docs/.decisions/DEC-001-old.md");
    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("superseded_by"));
    assert!(content.contains("DEC-002"));
}

#[test]
fn test_link_nonexistent_source() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["link", "DEC-999", "relates_to", "DEC-001"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_link_nonexistent_target() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["link", "DEC-001", "relates_to", "DEC-999"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ============================================================================
// Unlink Tests
// ============================================================================

#[test]
fn test_unlink_records() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "A"])
        .assert()
        .success();
    dg_cmd(&temp)
        .args(["new", "decision", "B"])
        .assert()
        .success();
    dg_cmd(&temp)
        .args(["link", "DEC-001", "relates_to", "DEC-002"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["unlink", "DEC-001", "relates_to", "DEC-002"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Unlinked"));

    // Verify link removed
    let file = temp.path().join("docs/.decisions/DEC-001-a.md");
    let content = fs::read_to_string(&file).unwrap();
    assert!(!content.contains("DEC-002") || content.contains("relates_to: []"));
}

// ============================================================================
// Search Tests
// ============================================================================

#[test]
fn test_search_by_title() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "PostgreSQL Database"])
        .assert()
        .success();
    dg_cmd(&temp)
        .args(["new", "decision", "Redis Cache"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["search", "postgres"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"))
        .stdout(predicate::str::contains("PostgreSQL"));
}

#[test]
fn test_search_no_results() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["search", "nonexistent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No records found"));
}

// ============================================================================
// Status Tests
// ============================================================================

#[test]
fn test_status_update() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["status", "DEC-001", "accepted"])
        .assert()
        .success();

    // Verify status changed
    let file = temp.path().join("docs/.decisions/DEC-001-test.md");
    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("status: accepted"));
}

// ============================================================================
// Validate Tests
// ============================================================================

#[test]
fn test_validate_clean() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"));
}

#[test]
fn test_validate_broken_link() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    // Manually add broken link
    let file = temp.path().join("docs/.decisions/DEC-001-test.md");
    let content = fs::read_to_string(&file).unwrap();
    let modified = content.replace("depends_on: []", "depends_on: [NONEXISTENT-001]");
    fs::write(&file, modified).unwrap();

    dg_cmd(&temp)
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("broken"));
}

// ============================================================================
// Format Tests
// ============================================================================

#[test]
fn test_fmt_check_clean() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    // Format first
    dg_cmd(&temp).arg("fmt").assert().success();

    // Check should pass
    dg_cmd(&temp).args(["fmt", "--check"]).assert().success();
}

// ============================================================================
// Lint Tests
// ============================================================================

#[test]
fn test_lint_clean() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .arg("lint")
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"));
}

#[test]
fn test_lint_strict_missing_tags() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    // Strict mode should fail on missing tags
    dg_cmd(&temp)
        .args(["lint", "--strict"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("missing required field"));
}

// ============================================================================
// Stats Tests
// ============================================================================

#[test]
fn test_stats() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();
    dg_cmd(&temp).args(["new", "adr", "ADR"]).assert().success();

    dg_cmd(&temp)
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total records: 2"))
        .stdout(predicate::str::contains("DEC"));
}

// ============================================================================
// Graph Tests
// ============================================================================

#[test]
fn test_graph_text() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .arg("graph")
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC"));
}

#[test]
fn test_graph_dot() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["graph", "-f", "dot"])
        .assert()
        .success()
        .stdout(predicate::str::contains("digraph"))
        .stdout(predicate::str::contains("DEC-001"));
}

#[test]
fn test_graph_json() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["graph", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"nodes\""))
        .stdout(predicate::str::contains("\"edges\""));
}

// ============================================================================
// Export Tests
// ============================================================================

#[test]
fn test_export_json() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .args(["export", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DEC-001"));
}

// ============================================================================
// Reindex Tests
// ============================================================================

#[test]
fn test_reindex() {
    let temp = setup_test_env();

    dg_cmd(&temp)
        .args(["new", "decision", "Test"])
        .assert()
        .success();

    dg_cmd(&temp)
        .arg("reindex")
        .assert()
        .success()
        .stdout(predicate::str::contains("Reindexed"));

    let index = temp.path().join("docs/.index.json");
    assert!(index.exists());
}

// ============================================================================
// Sample Project Validation Tests
// ============================================================================

/// Helper to run dg command in a sample directory
fn dg_sample_cmd(sample_dir: &str) -> Command {
    let mut cmd = Command::cargo_bin("dg").unwrap();
    let docs_path = format!("samples/{}/docs", sample_dir);
    cmd.args(["-D", &docs_path]);
    cmd
}

#[test]
fn test_sample_microsoft_validates() {
    dg_sample_cmd("microsoft")
        .arg("validate")
        .assert()
        .success();
}

#[test]
fn test_sample_microsoft_lint() {
    // Basic lint should pass
    dg_sample_cmd("microsoft").arg("lint").assert().success();
}

#[test]
fn test_sample_microsoft_list() {
    dg_sample_cmd("microsoft")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("records"));
}

#[test]
fn test_sample_pied_piper_validates() {
    dg_sample_cmd("pied-piper")
        .arg("validate")
        .assert()
        .success();
}

#[test]
fn test_sample_pied_piper_lint() {
    // Basic lint should pass
    dg_sample_cmd("pied-piper").arg("lint").assert().success();
}

#[test]
fn test_sample_pied_piper_list() {
    dg_sample_cmd("pied-piper")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("records"));
}

#[test]
fn test_sample_pied_piper_stats() {
    dg_sample_cmd("pied-piper")
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total records"));
}

#[test]
fn test_sample_microsoft_stats() {
    dg_sample_cmd("microsoft")
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total records"));
}
