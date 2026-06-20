# Integration Tests

**Status:** Planned  
**Purpose:** End-to-end integration tests for mcp:// protocol  

## Overview

This directory will contain integration tests that test complete flows across the mcp:// protocol implementation.

## Planned Tests

### End-to-End Flows
- Connect → List Tools → Call Tool
- Server registration flow
- Pool membership flow
- Credit earning flow
- Tier progression flow

### Cross-Component Tests
- CLI → Python client integration
- CLI → JavaScript client integration
- Python client → JavaScript client compatibility

### Real Server Tests
- Test against real MCP servers
- Test against MCP.city registry
- Test against geographic pools

## Status

Integration tests are planned for v0.2.0 release.

## Running Integration Tests

```bash
# Coming in v0.2.0
pytest tests/integration/
```

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
