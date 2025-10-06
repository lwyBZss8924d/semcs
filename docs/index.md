---
layout: default
title: Home
nav_order: 1
---

# ck - Semantic Code Search

**ck (seek)** finds code by meaning, not just keywords. It's grep that understands what you're looking for — search for "error handling" and find try/catch blocks, error returns, and exception handling code even when those exact words aren't present.

## Quick Start

```bash
# Install from crates.io
cargo install ck-search

# Just search — ck builds and updates indexes automatically
ck --sem "error handling" src/
ck --sem "authentication logic" src/
ck --sem "database connection pooling" src/

# Traditional grep-compatible search still works
ck -n "TODO" *.rs
ck -R "TODO|FIXME" .

# Combine both: semantic relevance + keyword filtering
ck --hybrid "connection timeout" src/

# Interactive TUI mode
ck --tui src/
```

## Key Features

- **🔍 Semantic Search**: Find code by meaning, not just text matching
- **🎨 Interactive TUI**: Beautiful terminal UI with live search and preview
- **🤖 AI Agent Integration**: MCP server for Claude Desktop, Cursor, and more
- **⚡ Drop-in grep**: Compatible with your existing grep workflows
- **🎯 Hybrid Search**: Combine semantic understanding with keyword precision
- **⚙️ Automatic Indexing**: Delta updates keep your index fresh
- **📁 Smart Filtering**: Respects .gitignore and .ckignore

## Documentation

### Getting Started
- [Installation](installation.html) - Install on macOS, Linux, Windows
- [Search Modes](search-modes.html) - Semantic, regex, and hybrid search
- [TUI Guide](tui-guide.html) - Interactive terminal UI

### Integration
- [AI Integration](ai-integration.html) - MCP server for AI agents
- [Advanced Usage](advanced-usage.html) - Index management, model selection

### Reference
- [Language Support](language-support.html) - Supported languages and chunking
- [CLI Reference](cli-reference.html) - Complete command-line options

## Example Searches

```bash
# Semantic search - understands concepts
ck --sem "error handling" src/
ck --sem "async task spawning" .
ck --sem "configuration loading" src/

# Hybrid - semantic + keyword filtering
ck --hybrid "timeout handling" src/
ck --hybrid "retry logic" .

# Classic regex (grep-compatible)
ck "TODO" src/
ck -r "FIXME|XXX" .
ck -n "fn main" src/

# Interactive TUI
ck --tui .
```

## Quick Links

- [GitHub Repository](https://github.com/BeaconBay/ck)
- [Crates.io](https://crates.io/crates/ck-search)
- [Report Issues](https://github.com/BeaconBay/ck/issues)
- [Changelog](https://github.com/BeaconBay/ck/blob/main/CHANGELOG.md)
