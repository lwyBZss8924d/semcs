---
layout: default
title: MCP API Reference
parent: AI Integration
nav_order: 2
---

# MCP API Reference

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

Complete Model Context Protocol specification for ck. All tools, parameters, and response formats.

## Overview

ck implements the Model Context Protocol (MCP) to provide AI agents with semantic code search capabilities. The server exposes tools for searching code, managing indexes, and retrieving metadata.

### Protocol Version

- **MCP Version:** 2024-11-05
- **ck Version:** 0.5.0+
- **Transport:** JSON-RPC over stdio

---

## Server Setup

### Starting the Server

```bash
ck --serve
```

**Options:**
- `--port <PORT>` - HTTP port (default: stdio)
- `--host <HOST>` - HTTP host (default: localhost)

### Client Configuration

**Claude Desktop:**
```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"]
    }
  }
}
```

**Custom client:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "my-client",
      "version": "1.0.0"
    }
  }
}
```

---

## Available Tools

### semantic_search

Find code by semantic meaning using embeddings.

**Tool Name:** `semantic_search`

**Parameters:**
```json
{
  "query": "string",           // Required: semantic search query
  "path": "string",            // Required: directory to search
  "threshold": 0.6,            // Optional: min relevance (0.0-1.0)
  "top_k": 100,                // Optional: max results
  "context_lines": 2,          // Optional: lines of context
  "snippet_length": 500,       // Optional: chars per snippet
  "include_snippet": true,     // Optional: include code snippets
  "cursor": null               // Optional: pagination cursor
}
```

**Response:**
```json
{
  "results": [
    {
      "file": "src/auth.rs",
      "line_start": 45,
      "line_end": 67,
      "score": 0.92,
      "snippet": "pub fn authenticate_user(credentials: &Credentials) -> Result<User> {...}",
      "chunk_type": "function",
      "breadcrumb": "auth::authenticate_user",
      "estimated_tokens": 123
    }
  ],
  "metadata": {
    "total_results": 234,
    "search_mode": "semantic",
    "model": "default",
    "threshold": 0.6
  },
  "pagination": {
    "has_next": true,
    "next_cursor": "eyJvZmZzZXQiOjUwfQ==",
    "page_size": 50,
    "total_results": 234
  }
}
```

**Example Usage:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "/home/user/project",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

### regex_search

Traditional grep-style pattern matching.

**Tool Name:** `regex_search`

**Parameters:**
```json
{
  "pattern": "string",         // Required: regex pattern
  "path": "string",            // Required: directory to search
  "ignore_case": false,        // Optional: case-insensitive
  "context": 3,                // Optional: lines of context
  "page_size": 50,             // Optional: results per page
  "snippet_length": 500,       // Optional: chars per snippet
  "include_snippet": true,     // Optional: include code snippets
  "cursor": null               // Optional: pagination cursor
}
```

**Response:**
```json
{
  "results": [
    {
      "file": "src/lib.rs",
      "line_start": 12,
      "line_end": 12,
      "snippet": "fn test_authentication() {",
      "chunk_type": "function",
      "breadcrumb": "test_authentication",
      "estimated_tokens": 45
    }
  ],
  "metadata": {
    "total_results": 15,
    "search_mode": "regex",
    "pattern": "fn test_\\w+"
  },
  "pagination": {
    "has_next": false,
    "next_cursor": null,
    "page_size": 50,
    "total_results": 15
  }
}
```

**Example Usage:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "fn test_\\w+",
    "path": "/home/user/project/tests",
    "ignore_case": true,
    "context": 3
  }
}
```

### hybrid_search

Combines semantic ranking with keyword filtering.

**Tool Name:** `hybrid_search`

**Parameters:**
```json
{
  "query": "string",           // Required: search query
  "path": "string",            // Required: directory to search
  "threshold": 0.6,            // Optional: min relevance (0.0-1.0)
  "top_k": 100,                // Optional: max results
  "page_size": 50,             // Optional: results per page
  "snippet_length": 500,       // Optional: chars per snippet
  "context_lines": 2,          // Optional: lines of context
  "include_snippet": true,     // Optional: include code snippets
  "cursor": null               // Optional: pagination cursor
}
```

**Response:**
Same as `semantic_search` but with `search_mode: "hybrid"`

**Example Usage:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "timeout",
    "path": "/home/user/project/src",
    "threshold": 0.7
  }
}
```

### index_status

Check indexing status and metadata.

**Tool Name:** `index_status`

**Parameters:**
```json
{
  "path": "string"             // Required: directory to check
}
```

**Response:**
```json
{
  "indexed": true,
  "total_chunks": 15234,
  "total_files": 342,
  "index_size_bytes": 45678901,
  "last_updated": "2024-10-06T12:34:56Z",
  "model": "default",
  "languages": ["rust", "javascript", "python"],
  "excluded_files": 1234,
  "indexing_time_seconds": 45.2
}
```

**Example Usage:**
```json
{
  "name": "index_status",
  "arguments": {
    "path": "/home/user/project"
  }
}
```

### reindex

Force rebuild of semantic index.

**Tool Name:** `reindex`

**Parameters:**
```json
{
  "path": "string",            // Required: directory to reindex
  "force": false               // Optional: force reindex even if up-to-date
}
```

**Response:**
```json
{
  "success": true,
  "total_chunks": 15234,
  "total_files": 342,
  "duration_ms": 2345,
  "index_size_bytes": 45678901
}
```

**Example Usage:**
```json
{
  "name": "reindex",
  "arguments": {
    "path": "/home/user/project",
    "force": true
  }
}
```

### health_check

Server status and diagnostics.

**Tool Name:** `health_check`

**Parameters:** None

**Response:**
```json
{
  "status": "healthy",
  "version": "0.5.3",
  "uptime_seconds": 1234,
  "memory_usage_mb": 256,
  "active_connections": 1
}
```

**Example Usage:**
```json
{
  "name": "health_check",
  "arguments": {}
}
```

---

## Pagination

All search tools support pagination for handling large result sets.

### Basic Pagination

```json
// First request
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "/home/user/project",
    "page_size": 25
  }
}

// Response includes pagination info
{
  "pagination": {
    "has_next": true,
    "next_cursor": "eyJvZmZzZXQiOjI1fQ==",
    "page_size": 25,
    "total_results": 234
  }
}

// Get next page
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "/home/user/project",
    "cursor": "eyJvZmZzZXQiOjI1fQ=="
  }
}
```

### Pagination Parameters

- **page_size**: Results per page (default: 50, max: 200)
- **cursor**: Opaque pagination token from previous response
- **total_results**: Total available results across all pages

---

## Error Handling

### Error Response Format

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "tool": "semantic_search",
      "parameter": "query",
      "reason": "Query cannot be empty"
    }
  }
}
```

### Common Error Codes

| Code | Description | Example |
|------|-------------|---------|
| -32602 | Invalid params | Missing required parameter |
| -32603 | Internal error | Index corruption |
| -32000 | Server error | File system error |
| -32001 | Tool error | Invalid regex pattern |

### Error Examples

**Invalid path:**
```json
{
  "error": {
    "code": -32000,
    "message": "Path not found",
    "data": {
      "path": "/nonexistent/path"
    }
  }
}
```

**Invalid regex:**
```json
{
  "error": {
    "code": -32001,
    "message": "Invalid regex pattern",
    "data": {
      "pattern": "[invalid regex",
      "reason": "Unclosed character class"
    }
  }
}
```

---

## Best Practices

### For AI Agents

**Start broad, then narrow:**
```json
// 1. Broad semantic search
{"name": "semantic_search", "arguments": {"query": "authentication", "path": "./"}}

// 2. Narrow to specific area
{"name": "semantic_search", "arguments": {"query": "JWT validation", "path": "./src/auth"}}
```

**Combine search modes:**
```json
// 1. Find config code semantically
{"name": "semantic_search", "arguments": {"query": "configuration", "path": "./"}}

// 2. Find config access patterns
{"name": "regex_search", "arguments": {"pattern": "config\\.\\w+", "path": "./"}}
```

**Use thresholds wisely:**
- `0.5-0.6`: Exploratory, cast wide net
- `0.7-0.8`: Focused, high precision
- `0.8+`: Very specific matches only

### For Integrators

**Handle pagination:**
- Default `page_size` is 50
- For large codebases, may need multiple calls
- Check `has_next` before requesting more pages

**Check index status first:**
```json
{"name": "index_status", "arguments": {"path": "/project"}}
```

If not indexed, first search will trigger indexing (1-5 seconds).

**Error handling:**
- Invalid paths return error
- Regex syntax errors return error
- Handle gracefully, inform user

---

## Example Workflows

### Code Exploration

**Agent task:** "Understand how authentication works"

```json
// 1. Find authentication code
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication",
    "path": "./src",
    "top_k": 5
  }
}

// 2. Find token validation
{
  "name": "semantic_search",
  "arguments": {
    "query": "token validation",
    "path": "./src/auth"
  }
}
```

### Refactoring Assistance

**Agent task:** "Find all database queries"

```json
// 1. Find database code
{
  "name": "hybrid_search",
  "arguments": {
    "query": "database query",
    "path": "./src"
  }
}

// 2. Find SQL patterns
{
  "name": "regex_search",
  "arguments": {
    "pattern": "SELECT .* FROM",
    "path": "./src"
  }
}
```

### Code Review

**Agent task:** "Find error handling issues"

```json
// 1. Find error handling
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "./src"
  }
}

// 2. Find potential issues
{
  "name": "regex_search",
  "arguments": {
    "pattern": "unwrap\\(\\)|expect\\(",
    "path": "./src"
  }
}
```

---

## Performance Considerations

### Indexing Performance

- **First search:** Triggers indexing (1-60 seconds)
- **Subsequent searches:** Instant (<200ms)
- **Index updates:** Automatic on file changes

### Search Performance

- **Semantic search:** ~100-200ms
- **Regex search:** ~50-100ms
- **Hybrid search:** ~150-250ms

### Memory Usage

- **Small repos:** ~100-500MB
- **Large repos:** ~1-5GB
- **Very large repos:** ~10-50GB

---

## Security Considerations

### Local Execution

- MCP server runs **locally**
- No data sent to cloud APIs
- Embeddings generated on-device
- Your code never leaves your machine

### File Access

- Server can only access files in specified `path`
- Respects `.gitignore` and `.ckignore`
- No write access (read-only search)

### Tool Permissions

- Claude Desktop requires explicit approval
- Review requested tools before approving
- Can revoke permissions anytime

---

## Troubleshooting

### Server Issues

**Server not starting:**
```bash
# Check version
ck --version

# Test server
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ck --serve
```

**Connection issues:**
- Verify MCP client configuration
- Check server logs
- Ensure ck is in PATH

### Search Issues

**No results:**
- Check if path exists and is readable
- Verify index status
- Try lower threshold
- Check .ckignore rules

**Slow performance:**
- Use absolute paths
- Search smaller directories
- Reduce `top_k` parameter
- Use regex for exact patterns

---

## Related Documentation

- **[MCP Quick Start](mcp-quickstart.html)** - Getting started guide
- **[Setup Guides](setup-guides.html)** - Integration with specific tools
- **[Examples](examples.html)** - Real-world usage examples
- **[CLI Reference](../reference/cli.html)** - Command-line interface
