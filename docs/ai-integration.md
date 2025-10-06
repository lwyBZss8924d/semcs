---
layout: default
title: AI Integration
nav_order: 4
---

# AI Integration

Connect ck to AI agents via the Model Context Protocol (MCP) for seamless code search capabilities.

## MCP Server

The MCP server allows AI agents like Claude Desktop, Cursor, and Windsurf to search your codebase using ck's semantic, regex, and hybrid search capabilities.

### Quick Start

```bash
# Start MCP server
ck --serve

# Server runs on stdio (for MCP clients)
# Ctrl+C to stop
```

### Claude Desktop Setup

**Recommended: Using Claude Code CLI**

```bash
# Install via Claude Code CLI
claude mcp add ck-search -s user -- ck --serve

# Verify installation
claude mcp list

# Test in Claude Code
# Type: /mcp
# You should see ck-search tools available
```

**Manual Configuration**

Edit your Claude Desktop config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`

**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

**Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"],
      "cwd": "/path/to/your/codebase"
    }
  }
}
```

**Important:** Replace `/path/to/your/codebase` with the actual directory you want to search.

**After configuration:**
1. Restart Claude Desktop
2. When prompted, approve permissions for ck-search tools
3. Test by asking: "Search for error handling in this codebase"

### Cursor Setup

Add to your Cursor settings (`.cursor/mcp_settings.json` or Cursor settings UI):

```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"],
      "cwd": "${workspaceFolder}"
    }
  }
}
```

### Windsurf Setup

Add to Windsurf MCP configuration:

```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"],
      "cwd": "${workspaceRoot}"
    }
  }
}
```

### Other MCP Clients

Any MCP-compatible client can use ck. The server communicates via stdio following the [MCP specification](https://spec.modelcontextprotocol.io).

## Available Tools

The MCP server exposes these tools to AI agents:

### `semantic_search`

Find code by meaning using embeddings.

**Parameters:**
```json
{
  "query": "authentication logic",     // Required: semantic query
  "path": ".",                         // Required: directory to search
  "top_k": 100,                        // Optional: max results (default: 100)
  "threshold": 0.5,                    // Optional: min relevance (0.0-1.0)
  "page_size": 50,                     // Optional: results per page (max: 200)
  "snippet_length": 500,               // Optional: chars per snippet
  "context_lines": 2,                  // Optional: lines of context
  "include_snippet": true,             // Optional: include code snippets
  "cursor": null                       // Optional: pagination cursor
}
```

**Returns:**
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
      "breadcrumb": "auth::authenticate_user"
    }
  ],
  "metadata": {
    "total_results": 234,
    "search_mode": "semantic",
    "model": "default",
    "threshold": 0.5
  },
  "pagination": {
    "has_next": true,
    "next_cursor": "eyJvZmZzZXQiOjUwfQ==",
    "page_size": 50,
    "total_results": 234
  }
}
```

### `regex_search`

Traditional grep-style pattern matching.

**Parameters:**
```json
{
  "pattern": "fn test_\\w+",           // Required: regex pattern
  "path": "tests/",                    // Required: directory to search
  "ignore_case": false,                // Optional: case-insensitive
  "context": 3,                        // Optional: lines of context
  "page_size": 50,                     // Optional: results per page
  "snippet_length": 500,               // Optional: chars per snippet
  "include_snippet": true,             // Optional: include code snippets
  "cursor": null                       // Optional: pagination cursor
}
```

**Returns:**
Similar structure to `semantic_search`, but without scores.

### `hybrid_search`

Combines semantic ranking with keyword filtering.

**Parameters:**
```json
{
  "query": "timeout handling",         // Required: hybrid query
  "path": "src/",                      // Required: directory to search
  "top_k": 100,                        // Optional: max results
  "threshold": 0.5,                    // Optional: min relevance
  "page_size": 50,                     // Optional: results per page
  "snippet_length": 500,               // Optional: chars per snippet
  "context_lines": 2,                  // Optional: lines of context
  "include_snippet": true,             // Optional: include code snippets
  "cursor": null                       // Optional: pagination cursor
}
```

**Returns:**
Same as `semantic_search`, with both semantic and keyword matching.

### `index_status`

Check indexing status and metadata.

**Parameters:**
```json
{
  "path": "."                          // Required: directory to check
}
```

**Returns:**
```json
{
  "indexed": true,
  "total_chunks": 15234,
  "total_files": 342,
  "index_size_bytes": 45678901,
  "last_updated": "2024-10-06T12:34:56Z",
  "model": "default"
}
```

### `reindex`

Force rebuild of search index.

**Parameters:**
```json
{
  "path": ".",                         // Required: directory to reindex
  "force": true                        // Optional: force full rebuild
}
```

**Returns:**
```json
{
  "success": true,
  "total_chunks": 15234,
  "total_files": 342,
  "duration_ms": 2345
}
```

### `health_check`

Server status and diagnostics.

**Parameters:** None

**Returns:**
```json
{
  "status": "healthy",
  "version": "0.5.3",
  "uptime_seconds": 1234
}
```

## Pagination

All search tools support pagination to handle large result sets gracefully.

### Basic Pagination

```python
# First request
response = await client.call_tool("semantic_search", {
    "query": "error handling",
    "path": ".",
    "page_size": 25
})

# Check if more results exist
if response["pagination"]["has_next"]:
    # Get next page using cursor
    next_response = await client.call_tool("semantic_search", {
        "query": "error handling",
        "path": ".",
        "cursor": response["pagination"]["next_cursor"]
    })
```

### Pagination Parameters

- **page_size**: Results per page (default: 50, max: 200)
- **cursor**: Opaque pagination token from previous response
- **total_results**: Total available results across all pages

### Best Practices

1. **Start with default page_size** (50) - good balance
2. **Use smaller page_size** (10-25) for exploratory searches
3. **Increase page_size** (100-200) when you need more context
4. **Check `has_next`** before requesting more pages
5. **Pass cursor exactly** as received (it's opaque base64)

## Example Agent Usage

### Claude Desktop Example

```
User: Find all error handling in this codebase

Claude: I'll search for error handling patterns.

[Uses semantic_search tool]
- query: "error handling"
- path: "."
- top_k: 50
- threshold: 0.7

Found 23 relevant results:

1. src/lib.rs:45-67 (score: 0.92)
   Function: handle_error
   This function implements comprehensive error handling...

2. src/error.rs:12-34 (score: 0.88)
   Enum: AppError
   Custom error type with detailed variants...

[etc.]

Would you like me to examine any specific implementation?
```

### Programmatic Example

```python
import anthropic

# Create MCP client
client = anthropic.Client()

# Search for authentication code
result = await client.call_tool("semantic_search", {
    "query": "user authentication",
    "path": "/path/to/code",
    "top_k": 20,
    "threshold": 0.75,
    "page_size": 10
})

# Process results
for item in result["results"]:
    print(f"{item['file']}:{item['line_start']}")
    print(f"  Score: {item['score']:.2f}")
    print(f"  {item['snippet'][:100]}...")
    print()

# Get more results if available
if result["pagination"]["has_next"]:
    more_results = await client.call_tool("semantic_search", {
        "query": "user authentication",
        "path": "/path/to/code",
        "cursor": result["pagination"]["next_cursor"]
    })
```

## JSONL Output (Custom Workflows)

For custom AI agent integrations, ck can output JSONL (JSON Lines):

```bash
# Generate JSONL output
ck --sem "error handling" --jsonl src/ > results.jsonl

# Each line is a complete JSON object
{"file":"src/lib.rs","line_start":45,"line_end":67,"score":0.92,...}
{"file":"src/error.rs","line_start":12,"line_end":34,"score":0.88,...}
```

### JSONL Format

Each line contains:

```json
{
  "file": "src/auth.rs",
  "line_start": 45,
  "line_end": 67,
  "score": 0.92,
  "text": "pub fn authenticate_user(...) {...}",
  "chunk_type": "function",
  "breadcrumb": "auth::authenticate_user",
  "estimated_tokens": 123
}
```

### Using JSONL in Agents

```python
import json

# Read JSONL results
results = []
with open('results.jsonl', 'r') as f:
    for line in f:
        results.append(json.loads(line))

# Process results
for result in results:
    if result['score'] > 0.8:
        print(f"Highly relevant: {result['file']}")
        # Feed to AI model, generate summary, etc.
```

## Configuration

### Model Selection

Choose embedding model via environment variable:

```bash
# Default model (fast, good accuracy)
ck --serve

# Larger model (slower, better accuracy)
CK_MODEL=large ck --serve
```

See [Advanced Usage](advanced-usage.html) for model details.

### Index Location

Indexes are stored in `.ck/` directory at repository root.

To use a different location:

```bash
CK_INDEX_PATH=/custom/path ck --serve
```

### Performance Tuning

```bash
# Increase worker threads for indexing
CK_WORKERS=8 ck --serve

# Adjust chunk size for embeddings
CK_CHUNK_SIZE=512 ck --serve
```

## Troubleshooting

### MCP Server Not Connecting

**Check server is running:**
```bash
# Test server directly
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ck --serve
```

**Verify Claude Desktop config:**
- Check JSON syntax
- Verify `cwd` path exists
- Restart Claude Desktop

**Check permissions:**
- Approve tool permissions when prompted
- Review approved tools in Claude settings

### Slow Search Performance

**First search is slow (indexing):**
- Normal: 1-2 seconds for medium repos
- Large repos (>10k files) may take longer
- Subsequent searches are fast (<100ms)

**Every search is slow:**
- Check index exists: `ls .ck/`
- Try reindexing: Use `reindex` tool
- Reduce `top_k` parameter

### No Results Found

**Check search mode:**
- Semantic: Requires indexed files
- Verify index status: Use `index_status` tool

**Check path:**
- Verify `path` parameter is correct
- Use absolute paths for clarity

**Check threshold:**
- Lower threshold: `"threshold": 0.3`
- Use `regex_search` to verify files exist

### Large Token Usage

**Reduce result size:**
```json
{
  "page_size": 10,              // Fewer results
  "snippet_length": 200,         // Shorter snippets
  "include_snippet": false,      // Omit snippets
  "top_k": 20                    // Limit total results
}
```

**Use pagination:**
- Request small pages (10-25 results)
- Let AI agent decide if more needed
- Avoids overwhelming context window

## Security Considerations

### Local Execution Only

- MCP server runs **locally**
- No data sent to cloud APIs
- Embeddings generated on-device
- Your code never leaves your machine

### File Access

- Server can only access files in `cwd`
- Respects `.gitignore` and `.ckignore`
- No write access (read-only search)

### Tool Permissions

- Claude Desktop requires explicit approval
- Review requested tools before approving
- Can revoke permissions anytime

## See Also

- [Search Modes](search-modes.html) - Semantic, regex, and hybrid search
- [Advanced Usage](advanced-usage.html) - Model selection and tuning
- [MCP Specification](https://spec.modelcontextprotocol.io) - Official MCP docs
