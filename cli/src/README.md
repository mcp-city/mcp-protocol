# CLI Source Code

**Status:** v0.1.0  
**Language:** Rust  
**Purpose:** mcp:// CLI tool implementation  

## Overview

This directory contains the source code for the mcp:// CLI tool, built in Rust for performance and portability.

## Files

### main.rs
The main entry point for the CLI tool.

**Features:**
- Command-line argument parsing (clap)
- HTTP client (reqwest)
- JSON serialization (serde)
- Error handling (anyhow)

**Commands:**
- `connect` - Connect to an MCP server
- `discover` - Discover MCP servers via DNS
- `register` - Register a server with MCP.city
- `list-tools` - List tools on a server
- `call` - Call a tool on a server

## Usage

```bash
# Build the CLI
cd cli
cargo build --release

# Run the CLI
cargo run -- connect registry.mcp.city

# Install locally
cargo install --path .
```

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Check code
cargo clippy
```

## Dependencies

- `clap` - Command-line argument parsing
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` - Serialization
- `anyhow` - Error handling

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
