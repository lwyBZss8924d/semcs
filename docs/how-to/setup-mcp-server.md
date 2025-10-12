---
layout: default
title: Setup MCP Server
parent: How-To Guides
nav_order: 1
---

# Setup MCP Server

Connect ck to Claude Desktop and other AI tools using the Model Context Protocol (MCP).

## What is MCP?

**Model Context Protocol (MCP)** is a standard protocol that allows AI agents to access external tools.

ck implements MCP, giving AI agents the ability to:
- Search code semantically
- Find patterns with regex
- Combine semantic + keyword search (hybrid)
- Check index status
- Trigger reindexing

---

## Claude Desktop Setup

### 1. Install ck

```bash
cargo install ck-search
```

Verify:
```bash
ck --version
```

### 2. Configure Claude Desktop

**macOS/Linux:** Edit `~/Library/Application Support/Claude/claude_desktop_config.json`

**Windows:** Edit `%APPDATA%\Claude\claude_desktop_config.json`

Add:
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

### 3. Restart Claude Desktop

1. Quit Claude Desktop completely
2. Reopen
3. Look for MCP indicator in bottom-left

### 4. Test It

In Claude Desktop:

```
Search for error handling in ~/projects/myapp
```

Claude will use ck's semantic search automatically!

---

## Other AI Tools

### Cursor IDE

Edit Cursor settings (Cmd/Ctrl+Shift+P → "Cursor Settings"):

```json
{
  "mcp": {
    "servers": {
      "ck-search": {
        "command": "ck",
        "args": ["--serve"]
      }
    }
  }
}
```

### Windsurf

Add to Windsurf MCP configuration:

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

### Custom MCP Client

Any MCP-compatible client can connect to ck's MCP server:

```bash
# Start server (runs on stdio)
ck --serve
```

Server accepts JSON-RPC 2.0 messages on stdin and responds on stdout.

---

## Command-Line Testing

Test MCP server without an AI tool:

### Start Server

```bash
ck --serve
```

Server runs on stdio, waiting for JSON-RPC messages.

### Send Test Request

**Initialize:**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ck --serve
```

**List tools:**
```bash
printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}\n{"jsonrpc":"2.0","id":2,"method":"tools/list"}\n' | ck --serve
```

---

## Available Tools

### semantic_search

Find code by meaning.

**Parameters:**
- `query` (required) - What you're looking for
- `path` (required) - Directory to search
- `threshold` (optional) - Min relevance (0.0-1.0, default: 0.6)
- `top_k` (optional) - Max results (default: 100)
- `context_lines` (optional) - Lines of context around match

**Example:**
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

Find code with pattern matching.

**Parameters:**
- `pattern` (required) - Regex pattern
- `path` (required) - Directory to search
- `ignore_case` (optional) - Case-insensitive (default: false)
- `context` (optional) - Context lines around match

**Example:**
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

Semantic + keyword filtering.

**Parameters:**
- `query` (required) - Search query
- `path` (required) - Directory to search
- `threshold` (optional) - Min relevance (0.0-1.0, default: 0.6)
- `top_k` (optional) - Max results (default: 100)

**Example:**
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

Check index health and statistics.

**Parameters:**
- `path` (required) - Directory path

**Returns:**
- Indexed file count
- Index size
- Last update time

### reindex

Force rebuild of semantic index.

**Parameters:**
- `path` (required) - Directory to reindex
- `force` (optional) - Force reindex even if up-to-date

**Use when:**
- Files changed outside ck
- Index corruption
- Major refactoring

---

## Best Practices

### For AI Agents

**Start broad, then narrow:**
```
1. semantic_search("authentication", "./")
2. semantic_search("JWT validation", "./src/auth")
```

**Combine search modes:**
```
1. semantic_search("configuration") - Find config code
2. regex_search("config\\.\\w+") - Find config access
```

**Use threshold wisely:**
- `0.5-0.6` - Exploratory, cast wide net
- `0.7-0.8` - Focused, high precision
- `0.8+` - Very specific matches only

### For Integrators

**Use absolute paths:**
- MCP servers may have different working directory than expected
- Always use absolute paths in tool calls

**Handle first-search indexing:**
- First search triggers indexing (1-5 seconds for medium repos)
- Subsequent searches are instant
- Check `index_status` first if needed

**Error handling:**
- Invalid paths return error
- Regex syntax errors return error
- Handle gracefully, inform user

---

## Troubleshooting

### Server Not Starting

**Check version:**
```bash
ck --version
```

Must be 0.5.0+.

**Test server:**
```bash
ck --serve --help
```

Should show MCP server options.

### Claude Desktop Not Finding Server

**Verify config:**
```bash
# macOS/Linux
cat ~/Library/Application\ Support/Claude/claude_desktop_config.json

# Windows
type %APPDATA%\Claude\claude_desktop_config.json
```

**Check logs:**
- macOS: `~/Library/Logs/Claude/mcp*.log`
- Windows: `%APPDATA%\Claude\logs\mcp*.log`

### No Results from Search

**Test command-line first:**
```bash
ck --sem "your query" /path/to/project
```

If CLI works but MCP doesn't:
1. Check path is absolute in MCP call
2. Verify path exists and is readable
3. Check index status with `index_status` tool

---

## Example Workflows

### Exploratory Search

**Agent task:** "Understand how authentication works"

**Tool calls:**
1. `semantic_search(query="authentication", path="./src", top_k=5)`
2. Read returned files
3. `semantic_search(query="token validation", path="./src/auth")`
4. Synthesize understanding

### Refactoring Assistance

**Agent task:** "Find all database queries"

**Tool calls:**
1. `hybrid_search(query="database query", path="./src")`
2. `regex_search(pattern="SELECT .* FROM", path="./src")`
3. Combine results, propose refactoring

### Code Review

**Agent task:** "Find error handling issues"

**Tool calls:**
1. `semantic_search(query="error handling", path="./src")`
2. Check each result for best practices
3. `regex_search(pattern="unwrap\\(\\)|expect\\(", path="./src")`
4. Report findings

---

## Next Steps

- **API details:** [MCP API Reference](../reference/mcp-api.html)
- **Agent examples:** [Agent Workflows](agent-workflows.html)
- **Configuration:** [Configuration Guide](configuration.html)
