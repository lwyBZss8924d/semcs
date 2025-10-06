---
layout: default
title: MCP API Reference
parent: For AI Agents
nav_order: 2
---

# MCP API Reference
{: .no_toc }

Complete Model Context Protocol specification for ck.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Protocol basics

**Transport:** stdio (standard input/output)

**Format:** JSON-RPC 2.0

**Protocol version:** 2024-11-05

---

## Initialization

### initialize

Handshake between client and server.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "client-name",
      "version": "1.0.0"
    }
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "ck-search",
      "version": "0.5.0"
    }
  }
}
```

### notifications/initialized

Client confirms initialization complete.

**Notification:**
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

No response expected.

---

## Tool discovery

### tools/list

Get available tools.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list"
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "semantic_search",
        "description": "Search for code semantically using embeddings",
        "inputSchema": {
          "type": "object",
          "properties": {
            "query": {"type": "string", "description": "Search query"},
            "path": {"type": "string", "description": "Directory path"},
            "threshold": {"type": "number", "description": "Min relevance (0-1)"},
            "top_k": {"type": "integer", "description": "Max results"},
            "context_lines": {"type": "integer", "description": "Context lines"}
          },
          "required": ["query", "path"]
        }
      },
      {
        "name": "regex_search",
        "description": "Search for code using regex patterns",
        "inputSchema": {
          "type": "object",
          "properties": {
            "pattern": {"type": "string", "description": "Regex pattern"},
            "path": {"type": "string", "description": "Directory path"},
            "ignore_case": {"type": "boolean", "description": "Case-insensitive"},
            "context": {"type": "integer", "description": "Context lines"}
          },
          "required": ["pattern", "path"]
        }
      },
      {
        "name": "hybrid_search",
        "description": "Combine semantic search with keyword filtering",
        "inputSchema": {
          "type": "object",
          "properties": {
            "query": {"type": "string", "description": "Search query"},
            "path": {"type": "string", "description": "Directory path"},
            "threshold": {"type": "number", "description": "Min relevance (0-1)"},
            "top_k": {"type": "integer", "description": "Max results"}
          },
          "required": ["query", "path"]
        }
      },
      {
        "name": "index_status",
        "description": "Get index statistics",
        "inputSchema": {
          "type": "object",
          "properties": {
            "path": {"type": "string", "description": "Directory path"}
          },
          "required": ["path"]
        }
      },
      {
        "name": "reindex",
        "description": "Force rebuild semantic index",
        "inputSchema": {
          "type": "object",
          "properties": {
            "path": {"type": "string", "description": "Directory path"},
            "force": {"type": "boolean", "description": "Force reindex"}
          },
          "required": ["path"]
        }
      },
      {
        "name": "health_check",
        "description": "Verify server is running",
        "inputSchema": {
          "type": "object",
          "properties": {}
        }
      }
    ]
  }
}
```

---

## Tool calls

### tools/call

Execute a tool.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "semantic_search",
    "arguments": {
      "query": "error handling",
      "path": "/home/user/project/src",
      "threshold": 0.7,
      "top_k": 10
    }
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "src/lib.rs:45-67 (0.92)\npub fn handle_error(e: Error) -> Result<()> {\n    match e {\n        Error::Io(err) => log::error!(\"IO: {}\", err),\n        Error::Parse(err) => log::error!(\"Parse: {}\", err),\n    }\n}\n\nsrc/error.rs:12-34 (0.88)\n..."
      }
    ]
  }
}
```

---

## Tool specifications

### semantic_search

**Purpose:** Find code by conceptual meaning using embeddings.

**Parameters:**

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| query | string | Yes | - | Natural language search query |
| path | string | Yes | - | Absolute directory path to search |
| threshold | number | No | 0.6 | Minimum relevance score (0.0-1.0) |
| top_k | integer | No | 100 | Maximum number of results |
| context_lines | integer | No | 0 | Lines of context around match |
| snippet_length | integer | No | 500 | Max characters per snippet |
| page_size | integer | No | 50 | Results per page |

**Returns:** Text content with file paths, line ranges, scores, and code snippets.

**Example:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "database connection pooling",
    "path": "/home/user/myapp/src",
    "threshold": 0.75,
    "top_k": 5,
    "context_lines": 2
  }
}
```

**Response format:**
```
path/to/file.rs:10-25 (0.92)
[context line]
[matched code block]
[context line]

path/to/other.rs:45-60 (0.87)
...
```

---

### regex_search

**Purpose:** Find code matching regex pattern.

**Parameters:**

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| pattern | string | Yes | - | Regular expression pattern |
| path | string | Yes | - | Absolute directory path |
| ignore_case | boolean | No | false | Case-insensitive matching |
| context | integer | No | 0 | Context lines around match |
| page_size | integer | No | 50 | Results per page |

**Returns:** Text content with file paths, line numbers, and matches.

**Example:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "async fn \\w+_handler",
    "path": "/home/user/myapp/src",
    "ignore_case": false,
    "context": 3
  }
}
```

---

### hybrid_search

**Purpose:** Combine semantic understanding with keyword filtering.

**Parameters:**

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| query | string | Yes | - | Search query (used for both semantic and keyword) |
| path | string | Yes | - | Absolute directory path |
| threshold | number | No | 0.6 | Minimum relevance score (0.0-1.0) |
| top_k | integer | No | 100 | Maximum results |
| context_lines | integer | No | 0 | Lines of context |

**How it works:**
1. Filters code containing query keywords
2. Ranks filtered results semantically
3. Returns best matches

**Best for:** Queries where you know a keyword but want semantic ranking.

**Example:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "timeout",
    "path": "/home/user/myapp",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

---

### index_status

**Purpose:** Check semantic index health and statistics.

**Parameters:**

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| path | string | Yes | - | Absolute directory path |

**Returns:** JSON with index statistics.

**Example:**
```json
{
  "name": "index_status",
  "arguments": {
    "path": "/home/user/myapp"
  }
}
```

**Response:**
```json
{
  "indexed": true,
  "file_count": 1247,
  "chunk_count": 8934,
  "index_size_mb": 45.2,
  "last_updated": "2025-01-15T10:30:00Z"
}
```

---

### reindex

**Purpose:** Force rebuild of semantic index.

**Parameters:**

| Name | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| path | string | Yes | - | Absolute directory path |
| force | boolean | No | true | Force even if index up-to-date |

**Returns:** Status message.

**Use when:**
- Files changed outside ck's knowledge
- Index corruption suspected
- Major codebase restructuring

**Example:**
```json
{
  "name": "reindex",
  "arguments": {
    "path": "/home/user/myapp",
    "force": true
  }
}
```

**Response:**
```json
{
  "status": "success",
  "files_indexed": 1247,
  "chunks_created": 8934,
  "time_seconds": 4.2
}
```

---

### health_check

**Purpose:** Verify server is running and responsive.

**Parameters:** None

**Returns:** Status message.

**Example:**
```json
{
  "name": "health_check",
  "arguments": {}
}
```

**Response:**
```json
{
  "status": "healthy",
  "version": "0.5.0"
}
```

---

## Error handling

### Error response format

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32600,
    "message": "Invalid path: /nonexistent",
    "data": {
      "details": "Directory does not exist or is not readable"
    }
  }
}
```

### Error codes

| Code | Meaning | Common causes |
|------|---------|---------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Malformed JSON-RPC |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Missing required parameter |
| -32603 | Internal error | Server error |

### Common errors

**Path not found:**
```json
{
  "code": -32602,
  "message": "Invalid path: /bad/path"
}
```

**Regex syntax error:**
```json
{
  "code": -32602,
  "message": "Invalid regex pattern: [unclosed"
}
```

**Index error:**
```json
{
  "code": -32603,
  "message": "Failed to load index"
}
```

---

## Pagination

Large result sets support pagination.

**Request with cursor:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "test",
    "path": "/project",
    "page_size": 25,
    "cursor": "eyJvZmZzZXQiOjI1fQ=="
  }
}
```

**Response with next cursor:**
```json
{
  "content": [...],
  "next_cursor": "eyJvZmZzZXQiOjUwfQ==",
  "has_more": true
}
```

---

## Performance considerations

### Indexing

**First search triggers indexing:**
- Small projects (<1k files): 1-2 seconds
- Medium projects (1k-10k files): 2-10 seconds
- Large projects (10k+ files): 10-30+ seconds

**Recommendation:** Call `index_status` first, warn user if not indexed.

### Search latency

**After indexing:**
- Semantic: 50-200ms
- Regex: 10-50ms
- Hybrid: 50-200ms

**Factors:**
- Project size
- Query complexity
- Result count
- Context lines requested

### Rate limiting

No built-in rate limiting. Server is single-threaded per instance.

---

## Best practices

### For AI agents

**1. Check index status first:**
```json
{"name": "index_status", "arguments": {"path": "/project"}}
```

**2. Use appropriate search mode:**
- Conceptual → `semantic_search`
- Patterns → `regex_search`
- Keywords + concepts → `hybrid_search`

**3. Adjust threshold based on results:**
- Too many results → increase threshold
- Too few results → decrease threshold

**4. Use context lines for clarity:**
```json
{"context_lines": 3}
```

**5. Paginate large results:**
- Start with `page_size: 25`
- Use `cursor` for subsequent pages

### For integrators

**1. Use absolute paths:**
```json
{"path": "/full/path/to/project"}
```

**2. Handle async indexing:**
- First search may take seconds
- Show progress indicator

**3. Cache index status:**
- Don't call `index_status` on every search
- Cache for duration of session

**4. Graceful degradation:**
- If semantic search fails, fall back to regex
- If server unavailable, inform user

---

## Examples

### Exploratory workflow

```json
// 1. Check if indexed
{"name": "index_status", "arguments": {"path": "/project"}}

// 2. Broad search
{"name": "semantic_search", "arguments": {
  "query": "authentication",
  "path": "/project",
  "top_k": 5
}}

// 3. Narrow down
{"name": "semantic_search", "arguments": {
  "query": "JWT token validation",
  "path": "/project/src/auth",
  "threshold": 0.8
}}

// 4. Find specific usage
{"name": "regex_search", "arguments": {
  "pattern": "verify_token\\(",
  "path": "/project/src"
}}
```

### Refactoring workflow

```json
// 1. Find pattern
{"name": "hybrid_search", "arguments": {
  "query": "unwrap",
  "path": "/project/src",
  "threshold": 0.7
}}

// 2. Confirm with regex
{"name": "regex_search", "arguments": {
  "pattern": "\\.unwrap\\(\\)",
  "path": "/project/src",
  "context": 2
}}

// 3. Find proper error handling examples
{"name": "semantic_search", "arguments": {
  "query": "error propagation with ?",
  "path": "/project/src",
  "top_k": 3
}}
```

---

## Next steps

**→** [MCP Quick Start](mcp-quickstart.html) - Get started in 5 minutes

**→** [Setup Guides](setup-guides.html) - Integration with specific AI tools

**→** [Examples](examples.html) - Real-world agent workflows
