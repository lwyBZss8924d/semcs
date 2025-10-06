---
layout: default
title: CLI Reference
parent: Reference
nav_order: 1
---

# CLI Reference

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

Complete command-line interface documentation for ck. All commands, flags, and options.

## Basic Usage

```bash
ck [OPTIONS] <PATTERN> [PATH]
ck [OPTIONS] --sem <QUERY> [PATH]
ck [OPTIONS] --hybrid <QUERY> [PATH]
ck [OPTIONS] --tui [PATH]
ck [OPTIONS] --serve
```

---

## Search Commands

### Semantic Search

```bash
ck --sem <QUERY> [PATH]
```

**Description:** Search code by meaning using semantic embeddings.

**Parameters:**
- `<QUERY>` - Semantic search query (required)
- `[PATH]` - Directory to search (default: current directory)

**Examples:**
```bash
ck --sem "error handling" src/
ck --sem "user authentication" .
ck --sem "database connection" apps/
```

### Regex Search (Default)

```bash
ck <PATTERN> [PATH]
```

**Description:** Traditional grep-style pattern matching.

**Parameters:**
- `<PATTERN>` - Regex pattern (required)
- `[PATH]` - Directory to search (default: current directory)

**Examples:**
```bash
ck "TODO" src/
ck "fn test_" tests/
ck "^use std::" src/
```

### Hybrid Search

```bash
ck --hybrid <QUERY> [PATH]
```

**Description:** Combines semantic ranking with keyword filtering.

**Parameters:**
- `<QUERY>` - Search query (required)
- `[PATH]` - Directory to search (default: current directory)

**Examples:**
```bash
ck --hybrid "timeout" src/
ck --hybrid "error" --sem "handling" src/
```

---

## Interactive Mode

### TUI (Text User Interface)

```bash
ck --tui [PATH]
```

**Description:** Launch interactive search interface.

**Parameters:**
- `[PATH]` - Directory to search (default: current directory)

**Options:**
- `--sem <QUERY>` - Start with semantic query
- `--regex <PATTERN>` - Start with regex pattern

**Examples:**
```bash
ck --tui .
ck --tui --sem "error handling" src/
ck --tui --regex "TODO" .
```

---

## MCP Server Mode

### Start MCP Server

```bash
ck --serve
```

**Description:** Start Model Context Protocol server for AI agent integration.

**Options:**
- `--port <PORT>` - Server port (default: stdio)
- `--host <HOST>` - Server host (default: localhost)

**Examples:**
```bash
ck --serve
ck --serve --port 8080
```

---

## Search Options

### Result Filtering

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--threshold <FLOAT>` | Minimum relevance score (0.0-1.0) | 0.6 | `--threshold 0.8` |
| `--topk <INT>` | Maximum number of results | 100 | `--topk 50` |
| `--context <INT>` | Lines of context around matches | 2 | `--context 5` |

### Output Format

| Option | Description | Example |
|--------|-------------|---------|
| `--jsonl` | Output in JSONL format | `--jsonl` |
| `--json` | Output in JSON format | `--json` |
| `--pretty` | Pretty-print JSON output | `--pretty` |
| `--no-color` | Disable colored output | `--no-color` |

### File Filtering

| Option | Description | Example |
|--------|-------------|---------|
| `--glob <PATTERN>` | Glob pattern for file matching | `--glob "*.rs"` |
| `--ignore-case` | Case-insensitive search | `--ignore-case` |
| `--invert-match` | Invert match results | `--invert-match` |

---

## Grep-Compatible Options

### Basic Grep Options

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--recursive` | `-r` | Search directories recursively | `-r` |
| `--ignore-case` | `-i` | Case-insensitive search | `-i` |
| `--line-number` | `-n` | Show line numbers | `-n` |
| `--count` | `-c` | Count matches per file | `-c` |
| `--files-with-matches` | `-l` | Show only filenames | `-l` |
| `--invert-match` | `-v` | Invert match results | `-v` |

### Advanced Grep Options

| Option | Description | Example |
|--------|-------------|---------|
| `--max-count <N>` | Stop after N matches | `--max-count 10` |
| `--after-context <N>` | Show N lines after match | `--after-context 3` |
| `--before-context <N>` | Show N lines before match | `--before-context 3` |
| `--context <N>` | Show N lines before/after | `--context 3` |

---

## Configuration Options

### Model Selection

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--model <NAME>` | Embedding model to use | `default` | `--model large` |
| `--model-path <PATH>` | Path to custom model | - | `--model-path ./model` |

### Index Management

| Option | Description | Example |
|--------|-------------|---------|
| `--reindex` | Force rebuild of index | `--reindex` |
| `--index-path <PATH>` | Custom index location | `--index-path ./custom` |
| `--no-index` | Skip indexing, use existing | `--no-index` |

### Performance Tuning

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--workers <N>` | Number of worker threads | CPU cores | `--workers 4` |
| `--chunk-size <N>` | Chunk size for embeddings | 512 | `--chunk-size 256` |
| `--batch-size <N>` | Batch size for processing | 32 | `--batch-size 16` |

---

## Environment Variables

### Model Configuration

```bash
export CK_MODEL=large              # Embedding model
export CK_MODEL_PATH=/path/to/model # Custom model path
```

### Index Configuration

```bash
export CK_INDEX_PATH=/custom/path  # Index location
export CK_WORKERS=8                # Worker threads
export CK_CHUNK_SIZE=512           # Chunk size
```

### Performance Configuration

```bash
export CK_BATCH_SIZE=32            # Batch size
export CK_MEMORY_LIMIT=2GB         # Memory limit
export CK_CACHE_SIZE=1GB           # Cache size
```

### Output Configuration

```bash
export CK_NO_COLOR=1               # Disable colors
export CK_JSON_OUTPUT=1            # JSON output
export CK_VERBOSE=1                # Verbose logging
```

---

## Common Usage Patterns

### Basic Search Patterns

```bash
# Find todos
ck "TODO" src/

# Find test functions
ck "fn test_" tests/

# Find imports
ck "^use " src/

# Case-insensitive search
ck -i "error" src/
```

### Semantic Search Patterns

```bash
# Find error handling
ck --sem "error handling" src/

# Find authentication code
ck --sem "user authentication" .

# Find database operations
ck --sem "database query" src/

# High precision search
ck --sem "JWT validation" --threshold 0.8 src/
```

### Advanced Patterns

```bash
# Search specific file types
ck --sem "pattern" --glob "*.rs" src/

# Limit results
ck --sem "pattern" --topk 10 src/

# JSON output
ck --sem "pattern" --jsonl src/ > results.jsonl

# Interactive mode
ck --tui --sem "pattern" src/
```

---

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | File not found |
| 4 | Permission denied |
| 5 | Index error |
| 6 | Model error |

---

## Examples

### Development Workflow

```bash
# Quick code exploration
ck --tui .

# Find specific patterns
ck --sem "authentication" src/

# Search tests
ck "fn test_" tests/

# Find todos
ck "TODO|FIXME" .
```

### CI/CD Integration

```bash
# Security scan
ck --sem "security" --threshold 0.8 src/ > security.txt

# Performance analysis
ck --sem "performance" --jsonl src/ > perf.jsonl

# Code review
ck "TODO|FIXME|HACK" --context 2 src/
```

### AI Integration

```bash
# Start MCP server
ck --serve

# Test MCP server
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ck --serve
```

---

## Troubleshooting

### Common Issues

**Command not found:**
```bash
# Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

**Slow first search:**
```bash
# Normal behavior - indexing takes time
# Subsequent searches are fast
```

**No results found:**
```bash
# Check if files exist
ck "fn " src/

# Lower threshold
ck --sem "pattern" --threshold 0.3 src/

# Check .ckignore
cat .ckignore
```

**Memory issues:**
```bash
# Reduce workers
export CK_WORKERS=2

# Optimize .ckignore
# Search smaller directories
```

---

## Related Commands

- **[TUI Reference](tui.html)** - Interactive interface commands
- **[Configuration](configuration.html)** - Configuration options
- **[MCP API](mcp-api.html)** - MCP server API
- **[Search Modes](../explanation/search-modes.html)** - Search mode explanations
