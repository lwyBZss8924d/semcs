---
layout: default
title: MCP Quick Start
parent: For Humans Using AI Tools
nav_order: 1
---

# MCP Quick Start
{: .no_toc }

Connect ck to Claude Desktop and other AI tools in 5 minutes.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## What is MCP?

**Model Context Protocol (MCP)** - A standard protocol for AI agents to access external tools.

ck implements MCP, allowing AI agents to:
- Search code semantically
- Find patterns with regex
- Combine semantic + keyword search
- Check index status
- Trigger reindexing

---

## Claude Desktop setup

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

### 4. Test it

In Claude Desktop:

```
Search for error handling in ~/projects/myapp
```

Claude will use ck's semantic search automatically!

---

## Command-line testing

Test MCP server without Claude Desktop:

### Start server

```bash
ck --serve
```

Server runs on stdio, waiting for JSON-RPC messages.

### Send test request

**initialize:**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ck --serve
```

**tools/list:**
```bash
printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}\n{"jsonrpc":"2.0","id":2,"method":"tools/list"}\n' | ck --serve
```

---

## Available tools

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

## Integration patterns

### Exploratory search

**Agent task:** "Understand how authentication works"

**Tool calls:**
1. `semantic_search(query="authentication", path="./src", top_k=5)`
2. Read returned files
3. `semantic_search(query="token validation", path="./src/auth")`
4. Synthesize understanding

### Refactoring assistance

**Agent task:** "Find all database queries"

**Tool calls:**
1. `hybrid_search(query="database query", path="./src")`
2. `regex_search(pattern="SELECT .* FROM", path="./src")`
3. Combine results, propose refactoring

### Code review

**Agent task:** "Find error handling issues"

**Tool calls:**
1. `semantic_search(query="error handling", path="./src")`
2. Check each result for best practices
3. `regex_search(pattern="unwrap\\(\\)|expect\\(", path="./src")`
4. Report findings

---

## Best practices

### For AI agents

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

### For integrators

**Handle pagination:**
- Default `top_k` is 100
- For large codebases, may need multiple calls

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

## Debugging

### Server not starting

**Check:**
```bash
ck --serve --help
```

Should show MCP server options.

**Check version:**
```bash
ck --version
```

Must be 0.5.0+.

### Claude Desktop not finding server

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

### No results from search

**Test command-line:**
```bash
ck --sem "your query" /path/to/project
```

If CLI works but MCP doesn't:
1. Check path is absolute in MCP call
2. Verify path exists and is readable
3. Check index status with `index_status` tool

---

## Tips

{: .tip }
**Use absolute paths:** MCP servers may have different working directory than expected

{: .tip }
**Index once, search many:** First search triggers indexing; subsequent searches are instant

{: .tip }
**Combine with file reading:** Use search to find relevant files, then read them for context

---

## Next steps

**→** [MCP API Reference](mcp-api.html) - Complete protocol documentation

**→** [Setup guides](setup-guides.html) - Integration with other AI tools

**→** [Examples](examples.html) - Real-world agent workflows
