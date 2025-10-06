---
layout: default
title: Search Modes
parent: For Humans
nav_order: 3
---

# Search Modes
{: .no_toc }

Three ways to search. Pick the right one for your task.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Quick guide

| Mode | When to use | Example |
|------|-------------|---------|
| **Semantic** | Find concepts | `ck --sem "error handling"` |
| **Regex** | Exact patterns | `ck "fn test_\w+"` |
| **Hybrid** | Concept + keyword | `ck --hybrid "timeout"` |

---

## Semantic search

**Find code by meaning, not just text.**

```bash
ck --sem "error handling" src/
```

Finds:
- `try/catch` blocks
- `Result<T, E>` returns
- `match` on errors
- `panic!` handlers
- Custom error types

Even when "error" or "handling" aren't in the code.

### When to use

✅ **Great for:**
- Learning a codebase
- Finding patterns ("authentication", "caching")
- Discovering similar implementations
- Conceptual searches

❌ **Not ideal for:**
- Exact string matching
- Syntax patterns
- Specific function names

### How it works

1. Analyzes code structure (functions, classes)
2. Generates embeddings (semantic vectors)
3. Compares your query to code embeddings
4. Ranks by relevance (0.0 - 1.0 score)

### Tuning results

**Lower threshold for more results:**
```bash
ck --sem "auth" --threshold 0.5 src/
```

**Limit to top results:**
```bash
ck --sem "caching" --topk 10 src/
```

---

## Regex search

**Traditional grep. Fast and precise.**

```bash
ck "fn test_\w+" tests/
```

Finds:
- `fn test_parse`
- `fn test_integration`
- `fn test_user_auth`

### When to use

✅ **Great for:**
- Exact strings ("TODO", "FIXME")
- Syntax patterns (`fn \w+`, `class.*Test`)
- Specific identifiers
- Fast searches (no indexing)

❌ **Not ideal for:**
- Conceptual searches
- Finding variations
- Learning codebases

### Grep compatibility

All your grep flags work:

```bash
# Case-insensitive
ck -i "fixme" src/

# Context lines
ck -A 3 -B 2 "error" src/

# List files only
ck -l "async fn" src/

# Count matches
ck -c "unwrap()" src/
```

---

## Hybrid search

**Semantic ranking + keyword filtering.**

```bash
ck --hybrid "timeout" src/
```

Finds code that:
1. Contains the keyword "timeout"
2. Ranked by semantic relevance to "timeout handling"

### When to use

✅ **Great for:**
- Broad concepts with known keywords
- Filtering semantic results
- "Best of both worlds"

❌ **Not ideal for:**
- Pure exploration (use semantic)
- Exact matching (use regex)

### Example

```bash
# Find authentication code mentioning "token"
ck --hybrid "token" src/

# More relevant results shown first
src/auth.rs:45 (0.92)
src/jwt.rs:12 (0.88)
src/session.rs:67 (0.75)
```

---

## Choosing the right mode

### Decision tree

```
Do you know exact text or pattern?
  ├─ Yes → Use REGEX
  └─ No → Are you exploring a concept?
      ├─ Yes → Do you know a keyword?
      │   ├─ Yes → Use HYBRID
      │   └─ No → Use SEMANTIC
      └─ No → Use REGEX
```

### Examples

| Task | Mode | Command |
|------|------|---------|
| Find TODOs | Regex | `ck "TODO" src/` |
| Learn error handling | Semantic | `ck --sem "error handling" .` |
| Find timeout code | Hybrid | `ck --hybrid "timeout" src/` |
| Find test functions | Regex | `ck "fn test_" tests/` |
| Discover retry patterns | Semantic | `ck --sem "retry backoff" .` |
| Find Config struct | Regex | `ck "struct Config" src/` |

---

## Performance

| Mode | First Run | Subsequent | Accuracy |
|------|-----------|------------|----------|
| Regex | Instant | Instant | Exact |
| Semantic | 1-2s (index) | <100ms | Conceptual |
| Hybrid | 1-2s (index) | <100ms | Filtered conceptual |

{: .note }
Semantic and hybrid create a `.ck/` index on first run. After that, searches are instant.

---

## Next steps

**→** [Find common patterns](find-patterns.html) - Real-world examples

**→** [Configure searches](configuration.html) - Thresholds, models, exclusions

**→** [CLI reference](cli-reference.html) - All flags and options
