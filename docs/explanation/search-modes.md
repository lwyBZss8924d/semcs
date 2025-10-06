---
layout: default
title: Search Modes Compared
parent: Explanation
nav_order: 1
---

# Search Modes

ck supports four search modes, each optimized for different use cases.

## Semantic Search (`--sem`)

Find code by meaning using local embeddings. Defaults to top 10 results with threshold ≥0.6.

```bash
ck --sem "error handling" src/
```

**What it finds:**
- `try/catch` blocks
- `Result<T, E>` returns  
- `match` expressions on errors
- Custom error types
- Panic handling
- Even when exact words aren't present

**Technology:**
- Local embedding models (no cloud API)
- Semantic understanding of programming concepts
- Relevance scoring (0.0 - 1.0)
- Cross-language pattern recognition

### When to Use

**Best for:**
- Finding conceptual patterns ("authentication", "caching", "rate limiting")
- Learning how something is implemented in a codebase
- Finding similar implementations across different files
- Code discovery and exploration

**Not ideal for:**
- Exact string matching
- Syntax patterns (use regex)
- Very specific identifiers (use regex)

### Examples

```bash
# Find authentication-related code
ck --sem "user authentication" src/

# Find async task spawning patterns
ck --sem "spawn async task" .

# Find configuration loading logic
ck --sem "load config from file" src/

# Find retry/resilience patterns
ck --sem "retry with backoff" .

# Find database query code
ck --sem "database query execution" src/

# Find logging implementations
ck --sem "structured logging" src/
```

### Understanding Scores

Results are ranked by semantic similarity:

- **0.9 - 1.0**: Extremely relevant match
- **0.8 - 0.9**: Highly relevant
- **0.7 - 0.8**: Relevant
- **0.6 - 0.7**: Moderately relevant
- **< 0.6**: May be tangentially related

**Tip:** Use `--threshold` to filter low-scoring results:

```bash
# Only show highly relevant matches
ck --sem "error handling" --threshold 0.75 src/
```

### Chunking Strategy

Semantic search operates on **code chunks** - meaningful units of code identified by tree-sitter:

**Chunk types:**
- Functions
- Methods
- Classes/Structs
- Modules
- Impl blocks

**Why chunks?**
- More accurate than line-by-line search
- Preserves code context
- Better semantic understanding
- Respects language structure

See [Language Support](language-support.html) for chunking details per language.

## Lexical Search (`--lex`)

BM25 full-text search with ranking. Automatically indexes before running.

```bash
ck --lex "user authentication" src/
ck --lex "http client request" .
```

**What it finds:**
- Full-text matches with relevance ranking
- Better than regex for phrases
- Handles synonyms and related terms
- Fast text-based search

**Technology:**
- BM25 ranking algorithm
- Automatic indexing
- Fast text processing
- No embedding computation

### When to Use

**Best for:**
- Phrase-based searches
- Full-text search with ranking
- When you want better than regex but faster than semantic

## Regex Search (default)

Traditional grep-style pattern matching. No indexing required.

```bash
ck "TODO" src/
ck "fn \w+_test" src/
ck -i "fixme" src/
```

**What it finds:**
- Exact text matches
- Regex patterns
- Syntax patterns
- Specific identifiers

**Technology:**
- Uses ripgrep's regex engine
- No indexing required
- Grep-compatible flags
- Maximum performance

### When to Use

**Best for:**
- Finding exact strings ("TODO", "FIXME", specific function names)
- Syntax patterns (`fn \w+`, `class \w+Test`)
- Fast searches without indexing overhead
- Grep-compatible workflows
- Simple text matching

**Not ideal for:**
- Conceptual searches
- Finding variations of an implementation
- Learning how something is done

### Grep Compatibility

ck is designed as a drop-in grep replacement:

```bash
# Standard grep patterns work
ck "pattern" file.txt
ck -r "pattern" directory/
ck -i "case-insensitive" .
ck -n "show line numbers" file.rs

# Extended grep features
ck -R "recursive" .
ck -v "invert match" file.txt
ck -l "files with matches" src/
ck -c "count matches" .
```

### Examples

```bash
# Find todos
ck "TODO" src/

# Find function definitions
ck "^fn " src/lib.rs

# Case-insensitive search
ck -i "fixme" .

# Find test functions
ck "fn test_\w+" tests/

# Find specific imports
ck "^use std::" src/

# Count occurrences
ck -c "unwrap()" src/

# List files containing pattern
ck -l "async fn" src/
```

## Hybrid Search (`--hybrid`)

Combines regex and semantic results using Reciprocal Rank Fusion.

```bash
ck --hybrid "timeout" src/
ck --hybrid "error" --topk 10 .
ck --hybrid "bug" --threshold 0.02 .  # RRF score threshold
```

**What it finds:**
- Combines regex and semantic search results
- Uses RRF (Reciprocal Rank Fusion) for ranking
- Balances precision and recall
- Filters by RRF score (0.01-0.05 range)

**Benefits:**
- Best of both worlds
- Keyword precision + semantic understanding
- Fewer false positives than pure semantic
- More context than pure regex

### When to Use Hybrid Search

✅ **Great for:**
- Broad semantic concepts with a known keyword
- Filtering semantic results to specific terms
- Finding the most relevant uses of a keyword
- Balancing precision and discovery

❌ **Not ideal for:**
- Pure exploration (use semantic)
- Exact pattern matching (use regex)

### Examples

```bash
# Find timeout-related code (ranked by relevance)
ck --hybrid "timeout" src/

# Find error handling that mentions "retry"
ck --hybrid "retry" .

# Find config code mentioning "env"
ck --hybrid "env" src/

# Find logging with "error" level
ck --hybrid "error" --sem "logging" src/
```

### Combining with Semantic Queries

You can provide both a semantic query and keywords:

```bash
# Semantic: "authentication"
# Keyword filter: "token"
ck --hybrid "token" --sem "authentication" src/
```

This finds authentication-related code that mentions "token".

## Choosing the Right Mode

### Decision Tree

```
Do you know the exact text/pattern?
├─ Yes → Use REGEX mode
│
└─ No → Do you want to find a concept/pattern?
    ├─ Yes → Do you know a keyword that should appear?
    │   ├─ Yes → Use HYBRID mode
    │   └─ No → Use SEMANTIC mode
    │
    └─ No → Use REGEX mode
```

### Real-World Examples

| Task | Mode | Example |
|------|------|---------|
| Find todos | Regex | `ck "TODO" src/` |
| Find error handling | Semantic | `ck --sem "error handling" .` |
| Find timeout code | Hybrid | `ck --hybrid "timeout" src/` |
| Find test functions | Regex | `ck "fn test_" tests/` |
| Learn retry patterns | Semantic | `ck --sem "retry with exponential backoff" .` |
| Find specific type | Regex | `ck "struct Config" src/` |
| Find caching logic | Semantic | `ck --sem "cache implementation" .` |
| Find println debugging | Hybrid | `ck --hybrid "println" --sem "debugging" .` |

## Performance Characteristics

### Indexing

**Semantic and Hybrid:**
- First search creates an index (~1-2 seconds for medium repos)
- Subsequent searches are instant (uses cached index)
- Delta updates on file changes (very fast)
- Index stored in `.ck/` directory

**Regex:**
- No indexing required
- Instant startup
- Searches file contents directly

### Search Speed

| Mode | Cold Start | Subsequent | Accuracy |
|------|------------|------------|----------|
| Regex | Instant | Instant | Exact match |
| Semantic | 1-2s (indexing) | <100ms | Conceptual |
| Hybrid | 1-2s (indexing) | <100ms | Filtered conceptual |

### Index Size

Typical index sizes (in `.ck/` directory):

- **Small repo** (1k files): ~10-50MB
- **Medium repo** (10k files): ~100-500MB
- **Large repo** (100k files): ~1-5GB

Indexes are automatically delta-updated when files change.

## Advanced Options

### Relevance Threshold

Control minimum score for semantic results:

```bash
# Only show results > 0.75 relevance
ck --sem "error handling" --threshold 0.75 src/

# Show more results (lower threshold)
ck --sem "authentication" --threshold 0.5 src/
```

### Top-K Results

Limit number of results:

```bash
# Show only top 10 most relevant results
ck --sem "caching" --topk 10 src/
```

### Model Selection

Choose embedding model (impacts speed vs accuracy):

```bash
# Fast, smaller model (default)
ck --sem "pattern" .

# Larger, more accurate model
ck --sem "pattern" --model large .
```

See [Advanced Usage](advanced-usage.html) for model details.

## Tips & Tricks

### Effective Semantic Queries

**Good queries:**
```bash
"error handling"              # Specific concept
"database connection pooling" # Precise pattern
"rate limiting"              # Implementation detail
"user authentication"        # Clear goal
```

**Less effective queries:**
```bash
"the code"                   # Too vague
"stuff"                      # Not specific
"things that do things"      # Not actionable
```

### Combining Modes

You can use different modes in sequence:

```bash
# 1. Broad semantic search to understand patterns
ck --sem "authentication" . | less

# 2. Narrow with hybrid to find specific implementation
ck --hybrid "jwt" --sem "authentication" src/

# 3. Exact regex to find all uses
ck "verify_jwt" src/
```

### Iterative Refinement

Start broad, narrow down:

```bash
# Too broad?
ck --sem "error" .

# Add keyword filtering
ck --hybrid "error" --sem "network requests" src/

# Or increase threshold
ck --sem "network errors" --threshold 0.8 src/
```

## See Also

- [TUI Guide](tui-guide.html) - Interactive search with all modes
- [Advanced Usage](advanced-usage.html) - Index management, model selection
- [CLI Reference](cli-reference.html) - Complete flag documentation
