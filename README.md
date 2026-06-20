# mcp:// Protocol

**Status:** v0.1.0 - Minimal Launch (Unit Tested)  
**License:** MIT License (Open Source)  
**Repository:** https://github.com/mcp-city/mcp-protocol  
**Organization:** https://github.com/mcp-city  
**Website:** https://mcp.city  
**Powered by:** [HYBRID IN.](https://hybridin.io/) x [GRN.cloud](https://grn.cloud/)  
**HYBRID IN.:** https://hybridin.io/  
**GRN.cloud:** https://grn.cloud/

## Overview

The mcp:// protocol is a custom URI scheme for Model Context Protocol (MCP) server connections, designed for ultra-fast discovery and auto-registration of MCP servers. This protocol is developed and maintained by the MCP.city open source organization, a partnership between HYBRID IN. and GRN.cloud.

## Features

- **Fast Connection**: Binary protocol (MessagePack/Protobuf) for minimal overhead
- **Auto-Discovery**: DNS TXT record support for automatic server discovery
- **Auto-Registry**: Servers announcing via mcp:// auto-register in MCP.city
- **Well-Known Endpoints**: Standard /.well-known/mcp endpoints for discovery
- **Modular Design**: Pluggable components for extensibility
- **MCP.city Integration**: Seamless integration with MCP.city marketplace and mesh
- **Tier-Based Access**: Free tier for basic usage, paid beta for advanced features

## Protocol Specification

See [SPECIFICATION.md](./docs/SPECIFICATION.md) for the complete IETF draft specification.

## CLI Tool

See [cli/](./cli/) for the mcp:// CLI tool implementation.

## Client Libraries

- Python: See [python/](./python/)
- JavaScript: See [javascript/](./javascript/)

## Testing

See [tests/](./tests/) for unit tests and integration tests.

## Quick Start

### Using the CLI Tool

```bash
# Install the CLI tool
cargo install mcp-cli

# Connect to an MCP server
mcp connect registry.mcp.city

# Discover MCP servers via DNS
mcp discover example.com

# Auto-register a server
mcp register --name "My MCP Server" --endpoint "https://mcp.example.com"
```

### Using the Python Client

```python
from mcp_protocol import MCPClient

# Connect to an MCP server
client = MCPClient("mcp://registry.mcp.city")
client.connect()

# List tools
tools = client.list_tools()

# Call a tool
result = client.call_tool("tool_name", {"param": "value"})
```

### Using the JavaScript Client

```javascript
import { MCPClient } from '@mcp-city/mcp-protocol';

// Connect to an MCP server
const client = new MCPClient('mcp://registry.mcp.city');
await client.connect();

// List tools
const tools = await client.listTools();

// Call a tool
const result = await client.callTool('tool_name', { param: 'value' });
```

## Modular Structure

This repository is organized as a modular open source project:

```
mcp-protocol/
├── cli/              # Rust CLI tool implementation
├── python/            # Python client library
├── javascript/        # JavaScript/TypeScript client library
├── docs/              # Protocol specification and documentation
├── tests/             # Unit tests and integration tests
└── examples/          # Usage examples
```

Each module is independently maintainable and can be used standalone.

## Tier-Based Access

**Free Tier (Open Source):**
- Basic mcp:// protocol (HTTP/JSON)
- Local mesh routing (single node)
- Basic discovery (DNS TXT + well-known)
- Standard MCP tools/list and tools/call
- Rate limit: 100/min
- Community support
- Self-hosted MCP servers

**Paid Beta Tier (Proprietary - HYBRID IN. Hosting):**
- Advanced mcp:// protocol (Binary + COGNIT MESH)
- COGNIT MESH integration (cognit.mesh.hybridin.io) - **Proprietary to HYBRID IN.**
- WHYBLE NODEs access (distributed compute) - **Hosted by HYBRID IN.**
- Geographic mesh routing (EU, US, ASIA) - **Hosted by HYBRID IN.**
- SSE streaming (real-time updates) - **Hosted by HYBRID IN.**
- Priority job queues (urgent, high, normal, low) - **Hosted by HYBRID IN.**
- Multi-tenant isolation (client_id context) - **Hosted by HYBRID IN.**
- Qdrant knowledge storage (vector embeddings) - **Hosted by HYBRID IN.**
- Mesh federation (cross-mesh communication) - **Hosted by HYBRID IN.**
- Advanced analytics dashboard - **Hosted by HYBRID IN.**
- Rate limit: 1000/min
- Priority support
- **MCP Server Hosting**: HYBRID IN. hosts your MCP server in their infrastructure during beta

**For Paid Beta Hosting:**
Contact HYBRID IN. for paid beta hosting and enterprise solutions:
- Website: https://hybridin.io/
- Commercial inquiries: See consultancy page at https://mcp.city/consults/hybridin-in

**HYBRID IN. Hosting (Beta):**
During the beta phase, HYBRID IN. offers hosting for MCP servers as part of the paid beta tier. This includes:
- Managed deployment on WHYBLE NODEs
- Geographic routing optimization
- Automatic scaling
- 99.9% uptime SLA
- 24/7 monitoring
- Security hardening
- Backup and disaster recovery

## Open Source & Donations

**Support Open Source Development:**
The mcp:// protocol is open source (MIT License) and maintained by the MCP.city organization. We accept donations to support ongoing development:

**Crypto Donations:**
- **Stellar (XLM):** GBLGZEPTY2L5FWHA4VTTVGRJILQUDQGV6PNCW2FQAVGS3DG6VMSZ7Q4D
- **Bitcoin (BTC):** bc1q8ejku5jd5kd8hn0c2rtcwjelh0df8hh06gusjh
- **Ethereum (ETH):** 0x82cdAbdFE73488f8Ad15fbb6e3f09c196f7eF79c
- **TRON (TRX):** TEB9GYLmz4Vyqko2EJ5xBqKMELaEECCbad
- **Ripple (XRP):** rsHpVD2TsAhQckgjpE37dA4NSFcVzVNFmw
- **Solana (SOL):** 4wPssAqCQ51L4hBmvoQcitMsLUxrNk9jdwBPG1owRUNa

**Traditional Donations:**
- GitHub Sponsors: https://github.com/sponsors/mcp-city

**Donation Usage:**
- Protocol development and maintenance
- CI/CD infrastructure
- Community tools and resources
- Documentation and tutorials
- Bug bounties and security audits
- Server costs and infrastructure

**Wall of Support:**
Donors who contribute will be featured on our Wall of Support at https://mcp.city/supporters. After verification, your donation will be displayed with:
- Your name or organization
- Donation amount (optional)
- Custom message (optional)
- Timestamp

**Commercial Contact:**
For commercial inquiries, partnerships, and enterprise solutions:
- Robert Schaeffer: [LinkedIn](https://linkedin.com/in/robert-schaeffer) (Commercial)
- Kai Gartner: [LinkedIn](https://linkedin.com/in/kaigartner) (Architect & Open Source)

## Documentation

- [Specification](./docs/SPECIFICATION.md)
- [CLI Tool Guide](./docs/CLI_GUIDE.md)
- [Client Libraries](./docs/CLIENT_LIBRARIES.md)
- [FAQ](./docs/FAQ.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

## License

MIT License - See [LICENSE](./LICENSE) for details.

## Contact

- GitHub: https://github.com/mcp-city/mcp-protocol
- Community: https://community.mcp.city

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD  
**Attribution Required:** When using the mcp:// protocol, you must reference "Powered by: HYBRID IN. x GRN. CLOUD" in your documentation and UI  
**SEO Keywords:** mcp protocol, Model Context Protocol, MCP, AI agents, distributed compute, mesh architecture, server-to-server communication, MCP.city, open source
