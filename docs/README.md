# mcp:// Protocol Documentation

**Status:** v0.1.0  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**License:** MIT License (Open Source)  
**Powered by:** HYBRID IN. x GRN. CLOUD  

## Overview

This directory contains the complete documentation for the mcp:// protocol, including the IETF draft specification, implementation guides, and reference materials.

## Documentation Files

### SPECIFICATION.md
The complete IETF draft specification for the mcp:// protocol.

**Topics:**
- URI scheme definition
- Connection protocol (HTTP/JSON)
- Server discovery (DNS TXT, well-known endpoints)
- Auto-registration with MCP.city
- Tool invocation
- Security requirements
- Error handling
- Compliance requirements

**Example URI:**
```
mcp://registry.mcp.city
mcp://token@api.mcp.city:8443/
mcp://example.com/mcp-server?version=1.0
```

### CLI_GUIDE.md (Coming Soon)
Complete guide for the mcp:// CLI tool.

**Topics:**
- Installation
- Configuration
- Command reference
- Usage examples
- Troubleshooting

### CLIENT_LIBRARIES.md (Coming Soon)
Guide for using the Python and JavaScript client libraries.

**Topics:**
- Installation
- Quick start
- API reference
- Examples
- Best practices

### FAQ.md (Coming Soon)
Frequently asked questions about the mcp:// protocol.

**Topics:**
- General questions
- Technical questions
- Security questions
- Licensing questions

## Reading Order

1. **SPECIFICATION.md** - Start here for protocol understanding
2. **CLI_GUIDE.md** - Learn to use the CLI tool
3. **CLIENT_LIBRARIES.md** - Integrate with your applications
4. **FAQ.md** - Common questions and answers

## Code Examples

### Python Client Example

```python
from mcp_protocol import MCPClient

# Connect to an MCP server
client = MCPClient("mcp://registry.mcp.city")
client.connect()

# List tools
tools = client.list_tools()
print(f"Available tools: {tools}")

# Call a tool
result = client.call_tool("echo", {"message": "Hello World"})
print(f"Result: {result}")
```

### JavaScript Client Example

```javascript
import { MCPClient } from '@mcp-city/protocol';

// Connect to an MCP server
const client = new MCPClient('mcp://registry.mcp.city');
await client.connect();

// List tools
const tools = await client.listTools();
console.log('Available tools:', tools);

// Call a tool
const result = await client.callTool('echo', { message: 'Hello World' });
console.log('Result:', result);
```

### CLI Example

```bash
# Connect to a server
mcp connect registry.mcp.city

# List tools
mcp list-tools registry.mcp.city

# Call a tool
mcp call registry.mcp.city echo --message "Hello World"
```

## Protocol Endpoints

### Server Info
```http
GET / HTTP/1.1
Host: registry.mcp.city
Accept: application/json
```

### List Tools
```http
GET /tools/list HTTP/1.1
Host: registry.mcp.city
Accept: application/json
```

### Call Tool
```http
POST /tools/call HTTP/1.1
Host: registry.mcp.city
Content-Type: application/json

{
  "tool": "echo",
  "arguments": {
    "message": "Hello World"
  }
}
```

## Contributing to Documentation

To contribute to the documentation:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

**Documentation Style:**
- Use clear, concise language
- Include code examples for all APIs
- Use proper Markdown formatting
- Add attribution footer

## License

MIT License - See [../LICENSE](../LICENSE) for details.

## Attribution Required

When using the mcp:// protocol, you must reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI.

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**SEO Keywords:** mcp protocol documentation, specification, API reference, guide
