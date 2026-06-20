# mcp:// Protocol Examples

**Status:** v0.1.0  
**Purpose:** Usage examples for the mcp:// protocol  

## Overview

This directory contains example MCP servers and usage examples for the mcp:// protocol.

## Example Servers

### Echo Server (Python)
A simple echo server that returns the input message.

**File:** `echo_server.py`

```python
from mcp_protocol import MCPClient

# Example usage
client = MCPClient("mcp://example.com")
client.connect()
result = client.call_tool("echo", {"message": "Hello World"})
print(result)
```

### Calculator Server (JavaScript)
A calculator server that performs basic arithmetic.

**File:** `calculator_server.js`

```javascript
import { MCPClient } from '@mcp-city/protocol';

// Example usage
const client = new MCPClient('mcp://example.com');
await client.connect();
const result = await client.callTool('calculate', { a: 10, b: 5, operation: 'add' });
console.log(result);
```

## Usage Examples

### Python Examples

#### Basic Connection
```python
from mcp_protocol import MCPClient

client = MCPClient("mcp://registry.mcp.city")
client.connect()
```

#### List Tools
```python
tools = client.list_tools()
for tool in tools:
    print(f"{tool['name']}: {tool['description']}")
```

#### Call Tool
```python
result = client.call_tool("echo", {"message": "Hello"})
print(result)
```

### JavaScript Examples

#### Basic Connection
```javascript
import { MCPClient } from '@mcp-city/protocol';

const client = new MCPClient('mcp://registry.mcp.city');
await client.connect();
```

#### List Tools
```javascript
const tools = await client.listTools();
tools.forEach(tool => {
    console.log(`${tool.name}: ${tool.description}`);
});
```

#### Call Tool
```javascript
const result = await client.callTool('echo', { message: 'Hello' });
console.log(result);
```

### CLI Examples

#### Connect to Server
```bash
mcp connect registry.mcp.city
```

#### List Tools
```bash
mcp list-tools registry.mcp.city
```

#### Call Tool
```bash
mcp call registry.mcp.city echo --message "Hello World"
```

## Running Examples

```bash
# Python examples
python examples/echo_server.py

# JavaScript examples
node examples/calculator_server.js

# CLI examples
mcp connect example.com
```

## Contributing Examples

To contribute an example:

1. Create a new file in this directory
2. Add a README entry
3. Include code comments
4. Test the example
5. Submit a pull request

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
