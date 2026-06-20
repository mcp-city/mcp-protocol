# JavaScript/TypeScript Source Code

**Status:** v0.1.0  
**Language:** TypeScript  
**Purpose:** mcp:// protocol client library  

## Overview

This directory contains the TypeScript source code for the mcp:// JavaScript client library.

## Files

### index.ts
Package entry point and exports.

**Exports:**
- `MCPClient` - Main client class
- `MCPError` - Base exception
- `ConnectionError` - Connection failure
- `ToolNotFoundError` - Tool not found

### client.ts
The main MCP client implementation.

**Class: MCPClient**
- `constructor(serverUrl, token)` - Initialize client
- `connect()` - Connect to server
- `getServerInfo()` - Get server information
- `listTools()` - List available tools
- `callTool(toolName, arguments)` - Call a tool
- `close()` - Close connection

### exceptions.ts
Exception classes for error handling.

**Exceptions:**
- `MCPError` - Base exception
- `ConnectionError` - Connection failed
- `ToolNotFoundError` - Tool not found
- `AuthenticationError` - Authentication failed
- `RateLimitError` - Rate limit exceeded
- `InvalidArgumentError` - Invalid argument

## Usage

```typescript
import { MCPClient } from './client';

// Create client
const client = new MCPClient('mcp://example.com');

// Connect
await client.connect();

// Use client
const tools = await client.listTools();
const result = await client.callTool('tool_name', { param: 'value' });

// Close
client.close();
```

## Development

```bash
# Build
npm run build

# Watch mode
npm run build -- --watch
```

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
