# mcp:// CLI Tool

**Status:** v0.1.0 - Minimal Launch (Unit Tested)  
**License:** MIT License (Open Source)  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**Organization:** https://github.com/mcp-city  
**Website:** https://mcp.city  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**HYBRID IN.:** https://hybridin.io/  
**GRN.cloud:** https://grn.cloud/

## Overview

The mcp:// CLI tool provides command-line access to the mcp:// protocol for MCP server discovery, connection, and auto-registration. Built in Rust for performance and portability.

## Features

- **Fast Discovery**: DNS TXT record-based server discovery
- **Connection Management**: Connect to MCP servers
- **Tool Testing**: Test MCP tools directly from CLI
- **Tool Listing**: List available tools on servers
- **Tool Calling**: Call tools with arguments
- **Attribution**: Built-in attribution to HYBRID IN. x GRN. CLOUD

**Current Implementation (v0.1.0):**
- ✅ connect command
- ✅ discover command (DNS placeholder)
- ✅ register command (API placeholder)
- ✅ list-tools command
- ✅ call command
- ⏳ serve command (local server - upcoming)
- ⏳ mesh commands (upcoming)

## Installation

```bash
# Install from crates.io
cargo install mcp-cli

# Or build from source
git clone https://github.com/mcp-city/mcp-protocol.git
cd mcp-protocol/cli
cargo build --release
```

## Usage

```bash
# Connect to an MCP server
mcp connect registry.mcp.city

# Discover MCP servers via DNS
mcp discover example.com

# Auto-register a server
mcp register --name "My MCP Server" --endpoint "https://mcp.example.com"

# List tools
mcp tools list

# Call a tool
mcp tools call tool_name --param value

# Show mesh nodes
mcp mesh nodes

# Show help
mcp --help
```

## Configuration

Configuration file: `~/.mcp/config.toml`

```toml
[default]
registry = "registry.mcp.city"
mesh = "mesh.mcp.city"
timeout = 30
attribution = true
```

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Build release
cargo build --release
```

## Attribution

This CLI tool includes attribution to HYBRID IN. x GRN. CLOUD in all help text and output. When using this tool, you agree to reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI.

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**Attribution Required:** When using the mcp:// CLI tool, you must reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI  
**SEO Keywords:** mcp CLI, Model Context Protocol, MCP, AI agents, distributed compute, command line tool, Rust, MCP.city, open source
