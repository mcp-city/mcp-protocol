// Integration tests for mcp:// CLI tool
// Tests require a running server and database

use std::process::Command;
use std::time::Duration;
use std::thread;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "mcp", "--help"])
        .current_dir("../cli")
        .output()
        .expect("Failed to execute CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mcp:// protocol CLI tool"));
}

#[test]
fn test_cli_serve_command() {
    // Start server in background
    let child = Command::new("cargo")
        .args(&["run", "--", "mcp", "serve", "--port", "8081"])
        .current_dir("../cli")
        .spawn()
        .expect("Failed to start server");

    // Wait for server to start
    thread::sleep(Duration::from_secs(2));

    // Test server endpoint
    let output = Command::new("curl")
        .args(&["-s", "http://localhost:8081/"])
        .output()
        .expect("Failed to query server");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Local MCP Server"));

    // Test tools endpoint
    let output = Command::new("curl")
        .args(&["-s", "http://localhost:8081/tools/list"])
        .output()
        .expect("Failed to query tools");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("echo"));

    // Test stats endpoint
    let output = Command::new("curl")
        .args(&["-s", "http://localhost:8081/stats"])
        .output()
        .expect("Failed to query stats");

    assert!(output.status.success());

    // Kill server
    child.kill().expect("Failed to kill server");
}

#[test]
fn test_cli_stats_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "mcp", "stats"])
        .current_dir("../cli")
        .output()
        .expect("Failed to execute stats command");

    // Should fail if no database exists
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Stats database not found"));
}

#[test]
fn test_cli_tier_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "mcp", "tier"])
        .current_dir("../cli")
        .output()
        .expect("Failed to execute tier command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("MCP.city Tier System"));
    assert!(stdout.contains("Free"));
}

#[test]
fn test_cli_pool_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "mcp", "pool"])
        .current_dir("../cli")
        .output()
        .expect("Failed to execute pool command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("MCP.city Pool Management"));
    assert!(stdout.contains("EU"));
    assert!(stdout.contains("US"));
    assert!(stdout.contains("ASIA"));
}

#[test]
fn test_cli_discover_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "mcp", "discover", "example.com"])
        .current_dir("../cli")
        .output()
        .expect("Failed to execute discover command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Discovering MCP servers"));
}
