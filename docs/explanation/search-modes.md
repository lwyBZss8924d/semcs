---
layout: default
title: Search Modes Compared
parent: Explanation
nav_order: 1
---

# Search Modes

cc supports four search modes, each optimized for different use cases.

## Semantic Search (`--sem`)

Find code by meaning using local embeddings. Defaults to top 10 results with threshold ≥0.6.

```bash
cc --sem "error handling" src/
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
cc --sem "user authentication" src/

# Find async task spawning patterns
cc --sem "spawn async task" .

# Find configuration loading logic
cc --sem "load config from file" src/

# Find retry/resilience patterns
cc --sem "retry with backoff" .

# Find database query code
cc --sem "database query execution" src/

# Find logging implementations
cc --sem "structured logging" src/
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
cc --sem "error handling" --threshold 0.75 src/
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
cc --lex "user authentication" src/
cc --lex "http client request" .
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
cc "TODO" src/
cc "fn \w+_test" src/
cc -i "fixme" src/
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

cc is designed as a drop-in grep replacement:

```bash
# Standard grep patterns work
cc "pattern" file.txt
cc -r "pattern" directory/
cc -i "case-insensitive" .
cc -n "show line numbers" file.rs

# Extended grep features
cc -R "recursive" .
cc -v "invert match" file.txt
cc -l "files with matches" src/
cc -c "count matches" .
```

### Examples

```bash
# Find todos
cc "TODO" src/

# Find function definitions
cc "^fn " src/lib.rs

# Case-insensitive search
cc -i "fixme" .

# Find test functions
cc "fn test_\w+" tests/

# Find specific imports
cc "^use std::" src/

# Count occurrences
cc -c "unwrap()" src/

# List files containing pattern
cc -l "async fn" src/
```

## Hybrid Search (`--hybrid`)

Combines regex and semantic results using Reciprocal Rank Fusion.

```bash
cc --hybrid "timeout" src/
cc --hybrid "error" --topk 10 .
cc --hybrid "bug" --threshold 0.02 .  # RRF score threshold
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
cc --hybrid "timeout" src/

# Find error handling that mentions "retry"
cc --hybrid "retry" .

# Find config code mentioning "env"
cc --hybrid "env" src/

# Find logging with "error" level
cc --hybrid "error" --sem "logging" src/
```

### Combining with Semantic Queries

You can provide both a semantic query and keywords:

```bash
# Semantic: "authentication"
# Keyword filter: "token"
cc --hybrid "token" --sem "authentication" src/
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
| Find todos | Regex | `cc "TODO" src/` |
| Find error handling | Semantic | `cc --sem "error handling" .` |
| Find timeout code | Hybrid | `cc --hybrid "timeout" src/` |
| Find test functions | Regex | `cc "fn test_" tests/` |
| Learn retry patterns | Semantic | `cc --sem "retry with exponential backoff" .` |
| Find specific type | Regex | `cc "struct Config" src/` |
| Find caching logic | Semantic | `cc --sem "cache implementation" .` |
| Find println debugging | Hybrid | `cc --hybrid "println" --sem "debugging" .` |

## Performance Characteristics

### Indexing

**Semantic and Hybrid:**
- First search creates an index (~1-2 seconds for medium repos)
- Subsequent searches are instant (uses cached index)
- Delta updates on file changes (very fast)
- Index stored in `.cc/` directory

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

Typical index sizes (in `.cc/` directory):

- **Small repo** (1k files): ~10-50MB
- **Medium repo** (10k files): ~100-500MB
- **Large repo** (100k files): ~1-5GB

Indexes are automatically delta-updated when files change.

## Advanced Options

### Relevance Threshold

Control minimum score for semantic results:

```bash
# Only show results > 0.75 relevance
cc --sem "error handling" --threshold 0.75 src/

# Show more results (lower threshold)
cc --sem "authentication" --threshold 0.5 src/
```

### Top-K Results

Limit number of results:

```bash
# Show only top 10 most relevant results
cc --sem "caching" --topk 10 src/
```

### Model Selection

Choose embedding model (impacts speed vs accuracy):

```bash
# Fast, smaller model (default)
cc --sem "pattern" .

# Larger, more accurate model
cc --sem "pattern" --model large .
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
cc --sem "authentication" . | less

# 2. Narrow with hybrid to find specific implementation
cc --hybrid "jwt" --sem "authentication" src/

# 3. Exact regex to find all uses
cc "verify_jwt" src/
```

### Iterative Refinement

Start broad, narrow down:

```bash
# Too broad?
cc --sem "error" .

# Add keyword filtering
cc --hybrid "error" --sem "network requests" src/

# Or increase threshold
cc --sem "network errors" --threshold 0.8 src/
```

## See Also

- [TUI Guide](tui-guide.html) - Interactive search with all modes
- [Advanced Usage](advanced-usage.html) - Index management, model selection
- [CLI Reference](cli-reference.html) - Complete flag documentation
