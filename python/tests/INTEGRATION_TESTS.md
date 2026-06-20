# Integration Tests for mcp:// Protocol

**Status:** Pending - Requires running main.frame.mcp.city instance

## Overview

Current unit tests in `test_client.py` use mocks for isolated unit testing. Integration tests with real data require a deployed and running `main.frame.mcp.city` instance.

## Unit Tests (Current)

**Location:** `test_client.py`

**Purpose:** Isolated unit testing of MCP client functionality

**Approach:** Uses mocks to test client logic without external dependencies

**Test Coverage:**
- URL conversion (mcp:// to https://)
- Token authentication
- Connection handling
- Server info retrieval
- Tool listing
- Tool invocation
- Error handling
- Context manager usage

**Run Unit Tests:**
```bash
cd python
pytest tests/test_client.py -v
```

## Integration Tests (Planned)

**Status:** Pending - Requires main.frame.mcp.city deployment

**Purpose:** Test with real data against running main.frame.mcp.city instance

**Prerequisites:**
- main.frame.mcp.city deployed and running
- Test environment configured
- OAuth provider configured (for OAuth tests)
- Mesh domains configured (for mesh tests)

**Test Coverage (Planned):**
- Real connection to main.frame.mcp.city
- Real OAuth provider authentication
- Real mesh domain routing
- Real certificate validation
- Real file upload protocol
- Real E2E encryption
- Real job submission via COGNIT MESH

**Run Integration Tests (Once Available):**
```bash
cd python
pytest tests/integration/ -v --integration
```

## Test Results

**Unit Tests:** Pass with mocks (isolated testing)

**Integration Tests:** Pending - Requires main.frame.mcp.city deployment

## Deployment Status

**main-frame Repository:** Infrastructure designed and documented
**Deployment Status:** Not yet deployed to production
**Next Steps:** Deploy main.frame.mcp.city to enable integration testing

## Notes

- Unit tests use mocks for isolated testing (standard practice)
- Integration tests require real main.frame.mcp.city instance
- No absolute paths from local development environment in open source
- All paths are relative to repository root
