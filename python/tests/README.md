# Python Unit Tests

**Status:** v0.1.0  
**Framework:** pytest  
**Purpose:** Unit tests for mcp_protocol package  

## Overview

This directory contains unit tests for the mcp:// Python client library.

## Test Files

### test_client.py
Unit tests for the MCPClient class.

**Test Coverage:**
- Client initialization
- URL conversion (mcp:// to https://)
- Connection success/failure
- Server info retrieval
- Tool listing
- Tool calling
- Error handling
- Context manager usage

## Running Tests

```bash
# Run all tests
pytest

# Run with verbose output
pytest -v

# Run with coverage
pytest --cov=mcp_protocol

# Run specific test
pytest tests/test_client.py::TestMCPClient::test_connect_success
```

## Test Results

**Current Coverage:** Core functionality tested

**Test Status:**
- ✅ Client initialization
- ✅ URL conversion
- ✅ Connection handling
- ✅ Server info
- ✅ Tool listing
- ✅ Tool calling
- ✅ Error handling
- ✅ Context manager

## Dependencies

- `pytest` - Testing framework
- `requests-mock` - HTTP mocking (add to requirements)

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
