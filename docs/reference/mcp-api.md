---
layout: default
title: MCP API Reference
parent: Reference
nav_order: 2
---

# MCP API Reference

Complete Model Context Protocol specification for ck. All tools, parameters, and response formats.

## Overview

ck implements the Model Context Protocol (MCP) to provide AI agents with semantic code search capabilities.

**Protocol Details:**
- **MCP Version:** 2024-11-05
- **Transport:** JSON-RPC over stdio
- **ck Version:** 0.5.0+

## Server Setup

### Starting the Server

```bash
ck --serve
```

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

## Available Tools

### semantic_search

Find code by semantic meaning using embeddings.

**Parameters:**
```json
{
  "query": "string",           // Required: semantic search query
  "path": "string",            // Required: directory to search
  "threshold": 0.6,            // Optional: min relevance (0.0-1.0)
  "top_k": 10,                 // Optional: max results (default: 10)
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

### regex_search

Traditional grep-style pattern matching.

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

### hybrid_search

Combines regex and semantic results using Reciprocal Rank Fusion.

**Parameters:**
```json
{
  "query": "string",           // Required: search query
  "path": "string",            // Required: directory to search
  "threshold": 0.02,           // Optional: min RRF score (0.01-0.05)
  "top_k": 10,                 // Optional: max results
  "page_size": 50,             // Optional: results per page
  "snippet_length": 500,       // Optional: chars per snippet
  "context_lines": 2,          // Optional: lines of context
  "include_snippet": true,     // Optional: include code snippets
  "cursor": null               // Optional: pagination cursor
}
```

### index_status

Check indexing status and metadata.

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

### reindex

Force rebuild of semantic index.

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

### health_check

Server status and diagnostics.

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

// Get next page using cursor
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

## Best Practices

### For AI Agents

**Start broad, then narrow:**
```json
// 1. Broad semantic search
{"name": "semantic_search", "arguments": {"query": "authentication", "path": "./"}}

// 2. Narrow to specific area
{"name": "semantic_search", "arguments": {"query": "JWT validation", "path": "./src/auth"}}
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

## Troubleshooting

### Server Issues

**Server not starting:**
```bash
# Check version
ck --version

# Test server
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ck --serve
```

**No results from search:**
- Check if path exists and is readable
- Verify index status
- Try lower threshold
- Check .ckignore rules
