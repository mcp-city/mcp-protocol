# JavaScript/TypeScript Unit Tests

**Status:** v0.1.0  
**Framework:** vitest  
**Purpose:** Unit tests for mcp:// JavaScript client library  

## Overview

This directory contains unit tests for the mcp:// JavaScript/TypeScript client library.

## Test Files

### client.test.ts
Unit tests for the MCPClient class.

**Test Coverage:**
- Client initialization
- URL conversion (mcp:// to https://)
- Connection success/failure
- Server info retrieval
- Tool listing
- Tool calling
- Error handling

## Running Tests

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run in watch mode
npm test -- --watch
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

## Dependencies

- `vitest` - Testing framework
- `@vitest/coverage-v8` - Coverage reporting

---

**Content Type:** Open Source (MIT License)  
**Access Level:** Public  
**Architect:** Kai Gartner ([LinkedIn](https://linkedin.com/in/kaigartner))  
**Powered by:** HYBRID IN. x GRN. CLOUD
