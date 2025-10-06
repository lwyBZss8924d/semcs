---
layout: default
title: CLI Reference
parent: For Humans
nav_order: 10
---

# CLI Reference
{: .no_toc }

Complete command-line reference.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Basic syntax

```bash
ck [OPTIONS] [PATTERN] [PATH...]
```

---

## Search modes

### Semantic search

```bash
ck --sem QUERY [PATH]
```

Search by meaning, not exact text.

**Options:**
- `--threshold FLOAT` - Minimum relevance (0.0-1.0, default: 0.6)
- `--topk NUM` - Maximum results (default: 100)

**Examples:**
```bash
ck --sem "error handling" src/
ck --sem "auth" --threshold 0.8 .
ck --sem "cache" --topk 20 src/
```

### Regex search (default)

```bash
ck PATTERN [PATH]
```

Traditional grep-style pattern matching.

**Examples:**
```bash
ck "TODO" src/
ck "fn test_\w+" tests/
ck -i "fixme" .
```

### Hybrid search

```bash
ck --hybrid QUERY [PATH]
```

Semantic ranking + keyword filtering.

**Examples:**
```bash
ck --hybrid "timeout" src/
ck --hybrid "retry" --threshold 0.7 .
```

---

## Interactive mode

```bash
ck --tui [PATH]
```

Launch visual search interface.

**Options:**
- Can combine with `--sem` for initial query
- Defaults to current directory if no path

**Examples:**
```bash
ck --tui .
ck --tui src/
ck --tui --sem "auth" .
```

---

## Output formats

### Default (human-readable)

```bash
ck --sem "error" src/
```

Shows file:line with context.

### JSON Lines

```bash
ck --sem "error" --jsonl src/
```

One JSON object per line. Great for programmatic use.

### Grep-compatible flags

```bash
-n, --line-number       Show line numbers
-l, --files-with-matches  Only show filenames
-L, --files-without-match Only show files without matches
-c, --count             Show count of matches
-i, --ignore-case       Case-insensitive
-v, --invert-match      Invert matching
```

**Examples:**
```bash
ck -n "error" src/
ck -l "TODO" .
ck -c "unwrap()" src/
```

---

## Context control

```bash
-A NUM    Show NUM lines after match
-B NUM    Show NUM lines before match
-C NUM    Show NUM lines before and after
```

**Examples:**
```bash
ck -A 3 "error" src/
ck -B 2 "TODO" .
ck -C 5 "FIXME" src/
```

---

## File filtering

### Exclusions

```bash
--exclude PATTERN    Exclude files matching pattern
--exclude-dir DIR    Exclude directory
```

**Examples:**
```bash
ck --exclude "*.test.js" "error" .
ck --exclude-dir node_modules "TODO" .
```

### Ignore files

```bash
--no-ignore      Skip .gitignore
--no-ckignore    Skip .ckignore
```

**.ckignore** - Like .gitignore but for search:
```
# Exclude by default
*.json
*.yaml
dist/
build/

# Include exceptions
!important-config.json
```

---

## Index management

### Check status

```bash
ck --index-status [PATH]
```

Shows index statistics.

### Force reindex

```bash
ck --reindex [PATH]
```

Rebuild index from scratch.

---

## Model selection

```bash
--model MODEL_NAME
```

Choose embedding model (affects semantic search).

**Options:**
- `default` - Fast, good accuracy (default)
- `large` - Slower, better accuracy

**Example:**
```bash
ck --sem "auth" --model large src/
```

---

## MCP server

```bash
ck --serve
```

Start Model Context Protocol server for AI agents.

Runs on stdio. See [For AI Agents](../for-agents/) for integration.

---

## Help & version

```bash
ck --help              Show help
ck --version           Show version
ck --generate-completions SHELL  Generate shell completions
```

---

## Environment variables

```bash
EDITOR          Editor for TUI (default: $VISUAL or vi)
CK_MODEL        Default embedding model
CK_WORKERS      Worker threads for indexing
CK_INDEX_PATH   Custom index location
```

**Example:**
```bash
export EDITOR=nvim
export CK_MODEL=large
ck --tui .
```

---

## Exit codes

- `0` - Success (matches found)
- `1` - No matches
- `2` - Error

---

## Examples

### Common workflows

**Find and open in editor:**
```bash
# Find with TUI, open with Enter
ck --tui src/
```

**Export results to JSON:**
```bash
ck --sem "auth" --jsonl src/ > results.jsonl
```

**Search with threshold tuning:**
```bash
# Too many results?
ck --sem "test" --threshold 0.8 tests/

# Too few results?
ck --sem "cache" --threshold 0.5 src/
```

**Combine with other tools:**
```bash
# Count matches per file
ck -l "TODO" . | wc -l

# Feed to xargs
ck -l "FIXME" . | xargs sed -i 's/FIXME/TODO/g'
```

---

## Next steps

**→** [Search modes](search-modes.html) - When to use each mode

**→** [Find patterns](find-patterns.html) - Common searches

**→** [Configuration](configuration.html) - .ckignore and settings
