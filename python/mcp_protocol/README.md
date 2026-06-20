# mcp_protocol Package

**Status:** v0.1.0  
**Language:** Python  
**Purpose:** mcp:// protocol client library  

## Overview

This package contains the core mcp:// protocol client library for Python.

## Modules

### __init__.py
Package initialization and exports.

**Exports:**
- `MCPClient` - Main client class
- `MCPError` - Base exception
- `ConnectionError` - Connection failure
- `ToolNotFoundError` - Tool not found

### client.py
The main MCP client implementation.

**Class: MCPClient**
- `__init__(server_url, token)` - Initialize client
- `connect()` - Connect to server
- `get_server_info()` - Get server information
- `list_tools()` - List available tools
- `call_tool(tool_name, arguments)` - Call a tool
- `close()` - Close connection

### exceptions.py
Exception classes for error handling.

**Exceptions:**
- `MCPError` - Base exception
- `ConnectionError` - Connection failed
- `ToolNotFoundError` - Tool not found
- `AuthenticationError` - Authentication failed
- `RateLimitError` - Rate limit exceeded
- `InvalidArgumentError` - Invalid argument

## Usage

```python
from mcp_protocol import MCPClient

# Create client
client = MCPClient("mcp://example.com")

# Connect
client.connect()

# Use client
tools = client.list_tools()
result = client.call_tool("tool_name", {"param": "value"})

# Close
client.close()
```

## Development

```bash
# Install in development mode
pip install -e .

# Run tests
pytest
```

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
