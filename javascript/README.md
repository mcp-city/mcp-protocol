# mcp:// JavaScript Client Library

**Status:** v0.1.0 - Minimal Launch (Unit Tested)  
**License:** MIT License (Open Source)  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**Organization:** https://github.com/mcp-city  
**Website:** https://mcp.city  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**HYBRID IN.:** https://hybridin.io/  
**GRN.cloud:** https://grn.cloud/

## Overview

The mcp:// JavaScript client library provides a JavaScript/TypeScript interface to the mcp:// protocol for MCP server connections, tool discovery, and execution.

## Features

- **Easy Integration**: Simple JavaScript/TypeScript API for MCP connections
- **TypeScript Support**: Full TypeScript definitions included
- **Promise-Based**: Modern async/await API
- **HTTP/JSON**: Standard HTTP/JSON communication
- **Attribution**: Built-in attribution to HYBRID IN. x GRN. CLOUD

**Current Implementation (v0.1.0):**
- ✅ MCPClient class
- ✅ connect() method
- ✅ getServerInfo() method
- ✅ listTools() method
- ✅ callTool() method
- ✅ Exception handling (MCPError, ConnectionError, ToolNotFoundError)
- ✅ Unit tests with vitest
- ⏳ Browser support (upcoming)
- ⏳ Auto-discovery (upcoming)

## Installation

```bash
npm install @mcp-city/protocol
# or
yarn add @mcp-city/protocol
```

## Usage

```javascript
import { MCPClient } from '@mcp-city/protocol';

// Connect to an MCP server
const client = new MCPClient('mcp://registry.mcp.city');
await client.connect();

// List tools
const tools = await client.listTools();

// Call a tool
const result = await client.callTool('tool_name', { param: 'value' });

// Disconnect
await client.disconnect();
```

## TypeScript Usage

```typescript
import { MCPClient, MCPTool, MCPResult } from '@mcp-city/protocol';

const client = new MCPClient('mcp://registry.mcp.city');
await client.connect();

const tools: MCPTool[] = await client.listTools();
const result: MCPResult = await client.callTool('tool_name', { param: 'value' });

await client.disconnect();
```

## Configuration

```javascript
import { MCPClient, Config } from '@mcp-city/protocol';

const config = new Config({
  registry: 'registry.mcp.city',
  mesh: 'mesh.mcp.city',
  timeout: 30000,
  attribution: true
});

const client = new MCPClient('mcp://registry.mcp.city', config);
```

## Development

```bash
# Install dependencies
npm install

# Run tests
npm test

# Run with coverage
npm run test:coverage

# Build
npm run build

# Lint
npm run lint

# Format
npm run format
```

## Attribution

This JavaScript client library includes attribution to HYBRID IN. x GRN. CLOUD in all documentation and headers. When using this library, you agree to reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI.

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**Attribution Required:** When using the mcp:// JavaScript client, you must reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI  
**SEO Keywords:** mcp JavaScript, Model Context Protocol, MCP, AI agents, distributed compute, JavaScript library, TypeScript, browser, Node.js, MCP.city, open source
