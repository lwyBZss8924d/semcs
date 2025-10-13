---
layout: default
title: CLI Reference
parent: Reference
nav_order: 1
---

# CLI Reference

Complete command-line interface documentation for cc.

## Synopsis

```bash
cc [OPTIONS] [PATTERN] [PATH...]
```

cc is a semantic code search tool that combines traditional regex search with AI-powered semantic search. It provides multiple search modes, interactive exploration, and grep-compatible output formats.

## Search Modes

### Default (Regex Search)

```bash
cc PATTERN [PATH]
```

Traditional grep-style pattern matching using regular expressions. This is the default mode when no search mode flag is specified.

**Examples:**
```bash
cc "TODO" src/
cc "fn test_\w+" tests/
cc -i "fixme" .
cc "error|warning" src/
```

**Related flags:**
- `-i, --ignore-case` - Case-insensitive search
- `-w, --word-regexp` - Match whole words only
- `-F, --fixed-strings` - Treat pattern as literal string (no regex)

### Semantic Search

```bash
cc --sem QUERY [PATH]
```

Finds code by meaning using embeddings, not just exact text matches. Ideal for conceptual searches when you know what you're looking for but not the exact terminology used in the codebase.

**Default behavior:**
- Returns top 100 results
- Threshold of ≥0.6 for relevance filtering
- Automatically indexes before search if needed

**Options:**
- `--threshold FLOAT` - Minimum relevance score (0.0-1.0, default: 0.6)
- `--topk NUM` - Maximum number of results (default: 100)
- `--limit NUM` - Alias for --topk
- `--show-scores` - Display similarity scores in output
- `--rerank` - Enable reranking for better relevance
- `--rerank-model MODEL` - Specify reranking model

**Examples:**
```bash
cc --sem "error handling" src/
cc --sem "user authentication" .
cc --sem "auth" --threshold 0.8 .           # Higher precision
cc --sem --topk 5 "authentication"          # Limit to top 5 results
cc --sem "cache" --topk 20 src/             # More results
cc --sem --show-scores "error" src/         # Show similarity scores
cc --sem --rerank "query" .                 # Enable reranking
```

### Lexical Search

```bash
cc --lex QUERY [PATH]
```

BM25 full-text search with statistical ranking. Combines keyword matching with frequency-based relevance scoring. Automatically indexes before running.

**Options:**
- `--threshold FLOAT` - Minimum BM25 score (0.0-1.0, default: 0.6)
- `--topk NUM` - Maximum number of results (default: 100)

**Examples:**
```bash
cc --lex "user authentication" src/
cc --lex "http client request" .
cc --lex --threshold 0.7 "error" src/
```

### Hybrid Search

```bash
cc --hybrid QUERY [PATH]
```

Combines regex pattern matching and semantic search using Reciprocal Rank Fusion (RRF). Best for queries that benefit from both keyword precision and semantic understanding.

**Options:**
- `--threshold FLOAT` - Minimum RRF score (typically 0.01-0.05, default: 0.02)
- `--topk NUM` - Maximum number of results (default: 100)

**Examples:**
```bash
cc --hybrid "async function" .
cc --hybrid "error" --topk 10 .
cc --hybrid "bug" --threshold 0.02 .        # RRF score threshold
cc --hybrid "timeout" src/
cc --hybrid "retry" --threshold 0.7 .
```

## Result Filtering

### Limit Results

```bash
--topk NUM        Limit to top N results
--limit NUM       Alias for --topk
```

Controls the maximum number of results returned. Applies to semantic, lexical, and hybrid searches.

**Examples:**
```bash
cc --sem --topk 5 "authentication" src/
cc --lex --limit 20 "error" .
```

### Score Thresholds

```bash
--threshold FLOAT    Minimum score for results
```

Filters results based on relevance scores:
- **Semantic/Lexical:** 0.0-1.0 (default: 0.6)
- **Hybrid (RRF):** 0.01-0.05 (default: 0.02)

Higher thresholds = fewer, more precise results. Lower thresholds = more results, broader matches.

**Examples:**
```bash
cc --sem --threshold 0.8 "query" .          # High precision
cc --sem --threshold 0.3 "pattern" src/     # Broader search
cc --hybrid --threshold 0.02 "bug" .        # RRF threshold
```

### Show Scores

```bash
--show-scores        Display similarity scores in output
```

Includes relevance scores alongside search results. Useful for tuning thresholds and understanding result quality.

**Example:**
```bash
cc --sem --show-scores "error handling" src/
```

## Output Formats

### Default (Human-Readable)

```bash
cc --sem "error" src/
```

Shows results in format: `file:line` with context snippets.

### JSON Output

```bash
--json           Traditional JSON (single array)
--jsonl          JSONL format (one JSON per line)
--json-v1        JSON v1 schema
--no-snippet     Exclude code snippets from JSONL
```

Structured output for programmatic processing and tool integration.

**Examples:**
```bash
cc --sem "auth" --jsonl src/ > results.jsonl
cc --json "error" .
cc --jsonl --no-snippet "TODO" src/
```

### Grep-Compatible Flags

cc maintains compatibility with grep's most common flags:

```bash
-n, --line-number          Show line numbers
-H, --with-filename        Print filename for each match (default)
-h, --no-filename          Suppress filename output
-l, --files-with-matches   Only show filenames with matches
-L, --files-without-match  Only show files without matches
-c, --count                Show count of matches per file
-i, --ignore-case          Case-insensitive search
-v, --invert-match         Invert matching (show non-matches)
-w, --word-regexp          Match whole words only
-F, --fixed-strings        Treat pattern as literal string (no regex)
-r, --recursive            Recursive search (default)
-q, --quiet                Suppress status messages
```

**Examples:**
```bash
cc -n "error" src/                    # Show line numbers
cc -l "TODO" .                        # List files with TODOs
cc -c "unwrap()" src/                 # Count unwrap() calls
cc -i "fixme" .                       # Case-insensitive
cc -l "TODO" . | wc -l                # Count files with TODOs
cc -l "FIXME" . | xargs sed -i 's/FIXME/TODO/g'  # Pipe to xargs
```

## Context Control

Control how much surrounding code is displayed with matches:

```bash
-A NUM    Show NUM lines after each match
-B NUM    Show NUM lines before each match
-C NUM    Show NUM lines before and after each match
```

**Examples:**
```bash
cc -A 3 "error" src/                  # 3 lines after
cc -B 2 "TODO" .                      # 2 lines before
cc -C 5 "FIXME" src/                  # 5 lines before and after
cc --context 2 "pattern" .            # Alias for -C 2
```

## File Filtering

### Exclusion Options

```bash
--exclude PATTERN           Exclude files matching pattern
--exclude-dir DIR          Exclude directory
--no-default-excludes      Disable default exclusions
--no-ignore                Don't respect .gitignore
--no-ccignore              Don't respect .ccignore
```

**Examples:**
```bash
cc --exclude "*.test.js" "error" .
cc --exclude-dir node_modules "TODO" .
cc --no-default-excludes "pattern" .
```

### .ccignore Files

Create a `.ccignore` file in your project root (similar to `.gitignore`) to exclude files from search and indexing:

```
# Exclude by default
*.json
*.yaml
dist/
build/
node_modules/
target/

# Include exceptions
!important-config.json
```

**Behavior:**
- Respects `.gitignore` by default
- `.ccignore` adds additional exclusions
- Use `--no-ignore` to skip `.gitignore`
- Use `--no-ccignore` to skip `.ccignore`

## Index Management

### Check Status

```bash
cc --status [PATH]              Show index status
cc --status-verbose [PATH]      Detailed index statistics
cc --index-status [PATH]        Alias for --status
```

Shows information about index state, number of files, chunks, and freshness.

**Examples:**
```bash
cc --status .
cc --status-verbose src/
```

### Index Operations

```bash
cc --index [PATH]               Create or update index
cc --reindex [PATH]             Force complete rebuild
cc --clean [PATH]               Remove entire index
cc --clean-orphans [PATH]       Clean orphaned files only
cc --add FILE                   Add single file to index
```

**Examples:**
```bash
cc --index .                    # Update index
cc --reindex .                  # Force rebuild
cc --clean .                    # Remove index
cc --add src/main.rs            # Index single file
```

### File Inspection

```bash
cc --inspect FILE               Show detailed file metadata
cc --dump-chunks FILE           Visualize chunk boundaries
```

Debug and understand how files are indexed and chunked.

**Examples:**
```bash
cc --inspect src/main.rs
cc --dump-chunks src/lib.rs
```

## Interactive Mode

### TUI (Text User Interface)

```bash
cc --tui [PATH]
```

Launch an interactive search interface with:
- Live search results
- Arrow key navigation
- File preview
- Editor integration (press Enter to open)
- Mode switching (semantic/regex/hybrid)

**Options:**
- Can combine with `--sem` for initial query
- Defaults to current directory if no path specified

**Examples:**
```bash
cc --tui .
cc --tui src/
cc --tui --sem "auth" .                # Start with semantic query
```

**Keyboard shortcuts:**
- `↑/↓` - Navigate results
- `Enter` - Open file in editor
- `Tab` - Switch search modes
- `Esc` - Clear search
- `Ctrl+C` - Quit

## MCP Server

### Start MCP Server

```bash
cc --serve
```

Start Model Context Protocol server for AI agent integration. Runs on stdio and provides these tools:

- `semantic_search` - Semantic code search
- `regex_search` - Regex pattern matching
- `hybrid_search` - Combined semantic + regex
- `index_status` - Check index state
- `reindex` - Rebuild index
- `health_check` - Server health check

**Example:**
```bash
cc --serve
```

See [MCP API Reference](mcp-api.html) for integration details.

## Model Selection

### Embedding Models

```bash
--model MODEL_NAME              Specify embedding model
--switch-model MODEL [PATH]     Switch model for existing index
```

Choose or change the embedding model used for semantic search:

**Available models:**
- `nomic-v1.5` - Fast, good accuracy (default)
- `jina-code` - Optimized for code
- `bge-small` - Compact, fast
- `large` - Slower, better accuracy

**Examples:**
```bash
cc --sem "auth" --model large src/
cc --model bge-small --sem "cache" .
cc --switch-model jina-code .              # Switch model
cc --switch-model jina-code --force .      # Force rebuild
```

**Note:** Changing models requires reindexing to generate new embeddings.

## Advanced Features

### Reranking

```bash
--rerank                    Enable reranking for better relevance
--rerank-model MODEL        Specify reranking model
```

Applies a second-stage reranking model to improve result quality for semantic search.

**Examples:**
```bash
cc --sem --rerank "query" .
cc --sem --rerank-model bge "authentication" src/
```

### Full Section Extraction

```bash
--full-section              Return complete functions/classes
```

Returns entire code sections (functions, classes, methods) instead of just matching lines.

**Example:**
```bash
cc --full-section "error" src/
```

## Environment Variables

```bash
EDITOR          Editor for TUI (default: $VISUAL or vi)
VISUAL          Fallback editor for TUI
CC_MODEL        Default embedding model
CC_WORKERS      Worker threads for indexing
CC_INDEX_PATH   Custom index location
```

**Examples:**
```bash
export EDITOR=nvim
export CC_MODEL=large
export CC_WORKERS=4
cc --tui .
```

## Exit Codes

- `0` - Success (matches found)
- `1` - No matches found
- `2` - Error occurred

Use in scripts:
```bash
if cc "TODO" src/; then
    echo "Found TODOs"
else
    echo "No TODOs found"
fi
```

## Examples

### Development Workflow

```bash
# Quick code exploration
cc --tui .

# Find specific patterns
cc --sem "authentication" src/

# Search tests
cc "fn test_" tests/

# Find todos and fixmes
cc "TODO|FIXME" .

# Find error handling
cc --sem "error handling" --threshold 0.7 src/

# Search with context
cc -C 3 "panic!" src/
```

### CI/CD Integration

```bash
# Security scan
cc --sem "security vulnerability" --threshold 0.8 src/ > security.txt

# Performance analysis
cc --sem "performance bottleneck" --jsonl src/ > perf.jsonl

# Code quality checks
cc "TODO|FIXME|HACK" --context 2 src/ > review.txt

# Count unsafe code blocks
cc -c "unsafe" src/
```

### Export and Processing

```bash
# Export results to JSON
cc --sem "auth" --jsonl src/ > results.jsonl

# Count matches per file
cc -l "TODO" . | wc -l

# Find and replace across files
cc -l "FIXME" . | xargs sed -i 's/FIXME/TODO/g'

# Pipe to other tools
cc --sem "cache" --jsonl . | jq '.[] | .file'
```

### Threshold Tuning

```bash
# Too many results? Increase threshold
cc --sem "test" --threshold 0.8 tests/

# Too few results? Lower threshold
cc --sem "cache" --threshold 0.3 src/

# Find exact threshold sweet spot
cc --sem --show-scores "pattern" . | less
```

### Advanced Searches

```bash
# Combine semantic search with file filtering
cc --sem "database query" --exclude "*.test.rs" src/

# Hybrid search with context
cc --hybrid "async" -C 5 src/

# Full function extraction
cc --full-section --sem "authentication" src/

# Search with reranking
cc --sem --rerank --topk 10 "error handling" src/
```

## Troubleshooting

### Command not found

```bash
# Add Cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or reinstall
cargo install cc
```

### No results found

```bash
# Check if pattern works with plain regex first
cc "fn " src/

# Lower threshold for semantic search
cc --sem "pattern" --threshold 0.3 src/

# Check index status
cc --status .

# Rebuild index
cc --reindex .
```

### Memory issues

```bash
# Reduce worker threads
export CC_WORKERS=2

# Clean and rebuild index
cc --clean .
cc --index .
```

### Slow searches

```bash
# Use faster model
cc --sem "query" --model bge-small src/

# Reduce result count
cc --sem --topk 10 "query" .

# Check index status
cc --status-verbose .
```

## See Also

- [TUI Reference](tui.html) - Interactive interface guide
- [Configuration](configuration.html) - Configuration options and .ccignore
- [MCP API](mcp-api.html) - MCP server API reference
- [Search Modes](../explanation/search-modes.html) - When to use each search mode
- [Architecture](../explanation/architecture.html) - How cc works internally
