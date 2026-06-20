# mcp:// Python Client Library

**Status:** v0.1.0 - Minimal Launch (Unit Tested)  
**License:** MIT License (Open Source)  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**Organization:** https://github.com/mcp-city  
**Website:** https://mcp.city  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**HYBRID IN.:** https://hybridin.io/  
**GRN.cloud:** https://grn.cloud/

## Overview

The mcp:// Python client library provides a Pythonic interface to the mcp:// protocol for MCP server connections, tool discovery, and execution.

## Features

- **Easy Integration**: Simple Python API for MCP connections
- **Type Hints**: Full type annotations for IDE support
- **Error Handling**: Comprehensive error handling
- **HTTP/JSON**: Standard HTTP/JSON communication
- **Attribution**: Built-in attribution to HYBRID IN. x GRN. CLOUD

**Current Implementation (v0.1.0):**
- ✅ MCPClient class
- ✅ connect() method
- ✅ get_server_info() method
- ✅ list_tools() method
- ✅ call_tool() method
- ✅ Exception handling (MCPError, ConnectionError, ToolNotFoundError)
- ✅ Unit tests with pytest
- ⏳ Async support (upcoming)
- ⏳ Auto-discovery (upcoming)

## Installation

```bash
pip install mcp-protocol
```

## Usage

```python
from mcp_protocol import MCPClient

# Connect to an MCP server
client = MCPClient("mcp://registry.mcp.city")
await client.connect()

# List tools
tools = await client.list_tools()

# Call a tool
result = await client.call_tool("tool_name", {"param": "value"})

# Disconnect
await client.disconnect()
```

## Async Usage

```python
import asyncio
from mcp_protocol import AsyncMCPClient

async def main():
    client = AsyncMCPClient("mcp://registry.mcp.city")
    await client.connect()
    
    tools = await client.list_tools()
    print(f"Available tools: {tools}")
    
    await client.disconnect()

asyncio.run(main())
```

## Configuration

```python
from mcp_protocol import MCPClient, Config

config = Config(
    registry="registry.mcp.city",
    mesh="mesh.mcp.city",
    timeout=30,
    attribution=True
)

client = MCPClient("mcp://registry.mcp.city", config=config)
```

## Development

```bash
# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run with coverage
pytest --cov=mcp_protocol

# Format code
black mcp_protocol

# Lint
ruff check mcp_protocol
```

## Attribution

This Python client library includes attribution to HYBRID IN. x GRN. CLOUD in all documentation and headers. When using this library, you agree to reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI.

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**Attribution Required:** When using the mcp:// Python client, you must reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI  
**SEO Keywords:** mcp Python, Model Context Protocol, MCP, AI agents, distributed compute, Python library, async, MCP.city, open source
