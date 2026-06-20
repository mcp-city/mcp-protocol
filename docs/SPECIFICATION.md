# mcp:// Protocol Specification (IETF Draft)

**Status:** Draft  
**Version:** 0.1  
**Date:** June 20, 2026  
**License:** MIT License  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**Organization:** https://github.com/mcp-city  
**Powered by:** HYBRID IN. x GRN. CLOUD  

## Abstract

The mcp:// protocol is a URI scheme for Model Context Protocol (MCP) server connections, designed for fast discovery and auto-registration of MCP servers. This specification defines the minimal core protocol using JSON/HTTP for open source compatibility.

## Table of Contents

1. [Introduction](#introduction)
2. [URI Scheme](#uri-scheme)
3. [Connection Protocol](#connection-protocol)
4. [Server Discovery](#server-discovery)
5. [Auto-Registration](#auto-registration)
6. [Tool Invocation](#tool-invocation)
7. [Security](#security)
8. [Error Handling](#error-handling)
9. [Examples](#examples)

## Introduction

The mcp:// protocol provides:
- Standardized URI scheme for MCP servers
- HTTP/JSON-based communication (minimal, open source)
- DNS TXT record discovery
- Auto-registration with MCP.city registry
- Tool invocation via standard HTTP methods

## URI Scheme

### Syntax

```
mcp://[user@]host[:port]/path[?query]
```

### Components

- **scheme:** `mcp` (required)
- **userinfo:** Optional authentication (e.g., `user@`, `bot@`, `agent@`)
- **host:** Server hostname or IP (required)
- **port:** Optional port (default: 8080)
- **path:** Optional path (default: `/`)
- **query:** Optional query parameters

### Examples

```
mcp://registry.mcp.city
mcp://user@api.mcp.city:8443/
mcp://bot@mesh.abc123.mcp.city
mcp://agent@mesh.xyz789.mcp.city
mcp://example.com/mcp-server?version=1.0
```

### Mesh Domain Standard

**Format:** `mesh.{uniqueID}.mcp.city`

Mesh domains are auto-generated for users, bots, and agents with real signed certificates:

- **User Mesh Domain:** `mesh.{userID}.mcp.city` - User's personal mesh address
- **Bot Mesh Domain:** `mesh.{botID}.mcp.city` - Bot's mesh address
- **Agent Mesh Domain:** `mesh.{agentID}.mcp.city` - Cast agent's mesh address

**Features:**
- Real-time routing via COGNIT MESH
- E2E encryption for machine-to-machine communication
- Signed certificate validation via main.frame.mcp.city
- SIP translation support (real world to mcp://)
- Auto-generated with valid certificates on registration

## Connection Protocol

### HTTP Endpoints

#### 1. Server Info

**Request:**
```http
GET / HTTP/1.1
Host: registry.mcp.city
Accept: application/json
```

**Response:**
```json
{
  "name": "MCP Registry",
  "version": "1.0.0",
  "protocol": "mcp://0.1",
  "capabilities": [
    "tools/list",
    "tools/call",
    "tools/subscribe"
  ]
}
```

#### 2. Tools List

**Request:**
```http
GET /tools/list HTTP/1.1
Host: registry.mcp.city
Accept: application/json
```

**Response:**
```json
{
  "tools": [
    {
      "name": "echo",
      "description": "Echo the input message",
      "inputSchema": {
        "type": "object",
        "properties": {
          "message": {
            "type": "string"
          }
        },
        "required": ["message"]
      }
    },
    {
      "name": "calculate",
      "description": "Perform calculations",
      "inputSchema": {
        "type": "object",
        "properties": {
          "a": {"type": "number"},
          "b": {"type": "number"},
          "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]}
        },
        "required": ["a", "b", "operation"]
      }
    }
  ]
}
```

#### 3. Tool Call

**Request:**
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

**Response:**
```json
{
  "result": "Echo: Hello World",
  "success": true
}
```

## Server Discovery

### DNS TXT Record

Servers can advertise themselves via DNS TXT records:

```
_mcp._tcp.example.com. IN TXT "mcp://api.example.com:8080"
```

### Well-Known Endpoint

Servers should provide discovery at:

```
GET /.well-known/mcp HTTP/1.1
```

**Response:**
```json
{
  "server_url": "mcp://api.example.com:8080",
  "version": "1.0.0",
  "registry": "https://registry.mcp.city"
}
```

## Auto-Registration

### Registration Request

**POST to Registry:**
```http
POST /api/v1/servers/register HTTP/1.1
Host: registry.mcp.city
Content-Type: application/json
Authorization: Bearer <token>

{
  "name": "My MCP Server",
  "endpoint": "mcp://api.example.com:8080",
  "description": "Example MCP server",
  "capabilities": ["tools/list", "tools/call"],
  "version": "1.0.0"
}
```

**Response:**
```json
{
  "server_id": "srv_abc123",
  "status": "pending_verification",
  "message": "Server registered, awaiting verification"
}
```

## Tool Invocation

### Synchronous Call

```http
POST /tools/call HTTP/1.1
Host: api.example.com:8080
Content-Type: application/json

{
  "tool": "calculate",
  "arguments": {
    "a": 10,
    "b": 5,
    "operation": "add"
  }
}
```

**Response:**
```json
{
  "result": 15,
  "success": true
}
```

### Streaming Call (SSE)

```http
GET /tools/stream HTTP/1.1
Host: api.example.com:8080
Accept: text/event-stream
```

**Response:**
```
data: {"progress": 0.1}
data: {"progress": 0.5}
data: {"result": "final_output", "success": true}
```

## Security

### Authentication

**Bearer Token:**
```http
Authorization: Bearer <token>
```

**API Key:**
```http
X-API-Key: <key>
```

### OAuth Provider Attachment

Users can attach their own OAuth 2.0 provider (nothing fixed - user brings their own):

- **Custom OAuth Providers:** Google, GitHub, Auth0, Okta, Azure AD, or any OAuth 2.0 provider
- **Token Validation:** main.frame.mcp.city validates OAuth tokens and extracts user identity
- **Auto Tier Assignment:** On successful connection, main.frame automatically assigns user tier (Free, Pro, Enterprise via HYBRID IN.)
- **RBAC Integration:** OAuth provider roles map to mcp:// protocol RBAC permissions
- **Single Sign-On:** Users sign in once via their OAuth provider and access all MCP.city services

### RBAC (Role-Based Access Control)

The mcp:// protocol supports RBAC for fine-grained access control:

- **Roles:** Admin, User, Bot, Agent, Guest
- **Permissions:** Read, Write, Execute, Admin
- **Scope:** Server-level, Tool-level, Resource-level
- **Inheritance:** Roles can inherit permissions from parent roles

### TLS

All connections should use HTTPS/TLS 1.3 in production.

### Certificate Validation

- **Signed Certificates:** Mesh domains have real signed certificates
- **Validation via main.frame:** Certificates validated through main.frame.mcp.city
- **Domain to mcp:// Mapping:** Real domain validated to mcp:// protocol
- **E2E Encryption:** Machine-to-machine encryption with signed certificates

### Rate Limiting

Servers should implement rate limiting:
- Default: 100 requests/minute
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_ARGUMENT",
    "message": "Invalid argument: 'operation' must be one of: add, subtract, multiply, divide",
    "details": {
      "field": "operation",
      "value": "modulo"
    }
  }
}
```

### Error Codes

- `INVALID_ARGUMENT`: Invalid input
- `TOOL_NOT_FOUND`: Tool does not exist
- `AUTHENTICATION_FAILED`: Invalid credentials
- `RATE_LIMIT_EXCEEDED`: Too many requests
- `INTERNAL_ERROR`: Server error

## Examples

### Complete Flow

1. **Discover Server:**
```bash
curl https://example.com/.well-known/mcp
```

2. **Get Server Info:**
```bash
curl https://api.example.com:8080/
```

3. **List Tools:**
```bash
curl https://api.example.com:8080/tools/list
```

4. **Call Tool:**
```bash
curl -X POST https://api.example.com:8080/tools/call \
  -H "Content-Type: application/json" \
  -d '{"tool":"echo","arguments":{"message":"Hello"}}'
```

5. **Register with MCP.city:**
```bash
curl -X POST https://registry.mcp.city/api/v1/servers/register \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"name":"My Server","endpoint":"mcp://api.example.com:8080"}'
```

## Implementation Requirements

### Minimal Open Source Implementation

**Required:**
- HTTP/JSON endpoints (/, /tools/list, /tools/call)
- DNS TXT record support
- Well-known endpoint (/.well-known/mcp)
- Basic authentication (Bearer token)
- Error handling

**Optional (Proprietary):**
- Binary protocol (MessagePack/Protobuf)
- C4 .mcp format
- COGNIT MESH integration
- RBAC and OAuth provider attachment
- Mesh domain routing (mesh.{uniqueID}.mcp.city)
- Chunked file upload protocol (block-chunk-sequence)
- E2E encryption with signed certificates
- Advanced security features

## Compliance

Implementations MUST:
- Support HTTP/1.1 or HTTP/2
- Accept and return JSON
- Implement standard error codes
- Support TLS in production
- Implement rate limiting

Implementations SHOULD:
- Support streaming (SSE)
- Provide health checks
- Log requests for debugging
- Implement caching where appropriate

## References

- Model Context Protocol: https://modelcontextprotocol.io
- HTTP/1.1: RFC 7230-7235
- JSON: RFC 8259
- DNS TXT Records: RFC 1035

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**SEO Keywords:** mcp protocol, specification, IETF draft, Model Context Protocol, HTTP, JSON
