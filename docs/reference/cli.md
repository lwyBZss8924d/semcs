---
layout: default
title: CLI Reference
parent: Reference
nav_order: 1
---

# CLI Reference

Complete command-line interface documentation for ck.

## Search Modes

### Semantic Search

```bash
ck --sem "query" [path]
```

Finds code by meaning using embeddings. Defaults to top 10 results with threshold ≥0.6.

```bash
ck --sem "error handling" src/
ck --sem "user authentication" .
ck --sem --topk 5 "authentication"    # Limit to top 5 results
ck --sem --threshold 0.8 "auth"       # Higher precision
```

### Lexical Search

```bash
ck --lex "query" [path]
```

BM25 full-text search with ranking. Automatically indexes before running.

```bash
ck --lex "user authentication" src/
ck --lex "http client request" .
```

### Hybrid Search

```bash
ck --hybrid "query" [path]
```

Combines regex and semantic results using Reciprocal Rank Fusion.

```bash
ck --hybrid "async function" .
ck --hybrid "error" --topk 10 .
ck --hybrid "bug" --threshold 0.02 .  # RRF score threshold
```

## Result Filtering

### Limit Results

```bash
ck --topk N "query" .        # Limit to top N results
ck --limit N "query" .       # Alias for --topk
```

### Score Thresholds

```bash
ck --threshold 0.8 "query" . # Minimum score (0.0-1.0 for semantic/lexical)
ck --threshold 0.02 "query" . # RRF score for hybrid (0.01-0.05)
```

### Show Scores

```bash
ck --scores "query" .        # Show similarity scores in output
```

## Output Formats

### JSON Output

```bash
ck --json "query" .          # Traditional JSON (single array)
ck --jsonl "query" .         # JSONL format (one JSON per line)
ck --json-v1 "query" .       # JSON v1 schema
ck --no-snippet "query" .    # Exclude code snippets from JSONL
```

## Interactive Mode

### TUI (Text User Interface)

```bash
ck --tui [path]
```

Interactive search interface with live results, arrow key navigation, and editor integration.

```bash
ck --tui .
ck --tui --sem "error handling" src/
```

## MCP Server Mode

### Start MCP Server

```bash
ck --serve
```

Start Model Context Protocol server for AI agent integration. Provides tools: `semantic_search`, `regex_search`, `hybrid_search`, `index_status`, `reindex`, `health_check`.

```bash
ck --serve
```

## Index Management

### Check Status

```bash
ck --status .              # Show index status
ck --status-verbose .      # Detailed statistics
```

### Index Operations

```bash
ck --index .               # Create/update index
ck --clean .               # Remove entire index
ck --clean-orphans .       # Clean orphaned files only
ck --add file.rs           # Add single file to index
ck --reindex .             # Force index update
```

### Model Management

```bash
ck --switch-model nomic-v1.5 .     # Switch embedding model
ck --switch-model jina-code --force .  # Force rebuild with new model
ck --model bge-small .              # Specify model for indexing
```

### File Inspection

```bash
ck --inspect file.rs       # Show detailed file metadata
ck --dump-chunks file.rs   # Visualize chunk boundaries
```

## Grep-Compatible Options

### Basic Options

```bash
ck -n "pattern" .           # Show line numbers
ck -i "pattern" .           # Case-insensitive search
ck -r "pattern" .           # Recursive search
ck -l "pattern" .           # List files with matches
ck -L "pattern" .           # List files without matches
ck -w "pattern" .           # Match whole words only
ck -F "pattern" .           # Fixed string (no regex)
```

### Context Options

```bash
ck -C 2 "pattern" .         # Show 2 lines of context
ck -A 3 "pattern" .         # Show 3 lines after match
ck -B 1 "pattern" .         # Show 1 line before match
```

### File Filtering

```bash
ck --exclude "node_modules" .    # Exclude directories
ck --no-default-excludes .       # Disable default exclusions
ck --no-ignore .                 # Don't respect .gitignore
ck --no-ckignore .               # Don't respect .ckignore
```

## Advanced Features

### Reranking

```bash
ck --sem --rerank "query" .      # Enable reranking for better relevance
ck --sem --rerank-model bge "query" .  # Use specific reranking model
```

### Full Section Extraction

```bash
ck --full-section "query" .      # Return complete functions/classes
```

### Quiet Mode

```bash
ck -q "query" .                  # Suppress status messages
```

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
