---
layout: default
title: Large Codebases
parent: For Humans
nav_order: 7
---

# Large Codebases
{: .no_toc }

Performance tips for searching massive projects.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## What is "large"?

| Size | Files | Lines | Index time | Search time |
|------|-------|-------|------------|-------------|
| Small | <1k | <100k | <1s | <50ms |
| Medium | 1k-10k | 100k-1M | 1-5s | 50-200ms |
| Large | 10k-100k | 1M-10M | 5-30s | 200-500ms |
| Huge | >100k | >10M | 30s+ | 500ms+ |

{: .note }
ck handles up to **100k+ files** efficiently. Times shown are for first-time indexing.

---

## First-time indexing

### Optimize worker count

```bash
# Use all cores
export CK_WORKERS=$(nproc)

# Or set explicitly
export CK_WORKERS=16

# Reindex
ck --reindex .
```

More workers = faster indexing (up to your CPU limit).

### Index subdirectories separately

**Large monorepo?** Index components independently:

```bash
# Index each service
cd services/auth && ck --reindex .
cd services/api && ck --reindex .
cd services/web && ck --reindex .

# Search specific service
cd services/auth
ck --sem "token validation" .
```

Smaller indices = faster searches.

---

## Exclude unnecessary files

### Use .ckignore aggressively

```gitignore
# Test fixtures (often huge)
tests/fixtures/
test_data/
__snapshots__/

# Generated code
*.generated.*
*_gen.go
auto_*

# Vendored dependencies
vendor/
third_party/
node_modules/

# Documentation
docs/
*.md
*.txt

# Config files
*.json
*.yaml
*.toml
```

{: .tip }
**Rule of thumb:** If you rarely search it, exclude it.

### Check what's being indexed

```bash
# See file count
ck --index-status .

# Reindex with verbose output
ck --reindex . --verbose
```

---

## Search performance

### Narrow your search scope

**Don't search from repo root:**

```bash
# Slow: searches entire monorepo
ck --sem "auth" .

# Fast: searches only relevant directory
ck --sem "auth" services/auth/
```

### Use hybrid when you know keywords

**Semantic search scans all chunks:**

```bash
# Slower: semantic ranks all code
ck --sem "error handling" .
```

**Hybrid filters first:**

```bash
# Faster: only ranks code with "error"
ck --hybrid "error" .
```

### Limit results

```bash
# Only need top matches?
ck --sem "cache" --topk 10 .

# Default is 100
ck --sem "cache" .
```

Fewer results = less work.

---

## Index management

### Index location

**Default:** `.ck/` in project root

**Custom location:**

```bash
export CK_INDEX_PATH=/fast/ssd/ck-indices

ck --sem "query" .
```

{: .tip }
Put indices on fast storage (SSD) for best performance.

### When to reindex

**Automatic:** ck detects file changes and updates incrementally

**Manual reindex needed when:**
- Moving/renaming many files
- Changing branches with major differences
- Index corruption (rare)

```bash
ck --reindex .
```

### Multiple branches

**Option 1: Separate indices per branch**

```bash
export CK_INDEX_PATH=/tmp/ck-indices-$(git branch --show-current)
ck --sem "query" .
```

**Option 2: Reindex on branch switch**

```bash
# .git/hooks/post-checkout
#!/bin/bash
ck --reindex . &
```

Reindexes in background after checkout.

---

## Memory usage

### Index size on disk

**Approximate formula:**
- **Small projects (<1k files):** 5-20 MB
- **Medium projects (1k-10k files):** 20-200 MB
- **Large projects (10k-100k files):** 200 MB - 2 GB

### Runtime memory

**Indexing:** 200-500 MB per worker

**Searching:** <100 MB for most queries

{: .note }
Memory usage scales with file size and worker count, not file count.

---

## Real-world examples

### Linux kernel (~70k files)

```bash
# First index
ck --reindex .
# Time: ~45 seconds
# Index size: ~1.2 GB

# Search
ck --sem "memory allocation" .
# Time: ~300ms
# Results: 100+ matches

# Optimized search
ck --sem "memory allocation" kernel/
# Time: ~80ms
```

### Large Node.js monorepo

**Before optimization:**
```bash
# Includes node_modules/
ck --reindex .
# Time: 2+ minutes
# Index size: 3+ GB
```

**After .ckignore:**
```gitignore
node_modules/
dist/
coverage/
.next/
*.map
```

```bash
ck --reindex .
# Time: ~15 seconds
# Index size: ~400 MB
```

### Chromium (~300k files)

**Too large?** Index components:

```bash
# Index renderer only
cd src/renderer && ck --reindex .

# Index core only
cd src/core && ck --reindex .
```

---

## Benchmarking

### Measure index time

```bash
time ck --reindex .
```

### Measure search time

```bash
# Warm up (loads index into memory)
ck --sem "test" . > /dev/null

# Measure
time ck --sem "error handling" .
```

### Profile worker utilization

```bash
# Watch CPU during indexing
htop  # or Activity Monitor on macOS

# Adjust workers if needed
export CK_WORKERS=8
ck --reindex .
```

---

## Tips

{: .tip }
**First optimization: Add .ckignore** - Usually cuts index time by 50%+

{: .tip }
**Search subdirectories** - Faster than searching from root

{: .tip }
**Use hybrid for common terms** - Filters before semantic ranking

{: .tip }
**Indices are portable** - Copy `.ck/` to share with team (same code version)

---

## Troubleshooting

### Indexing too slow

**Check:**
1. Worker count: `echo $CK_WORKERS`
2. Excluded files: Review `.ckignore`
3. Disk speed: Use SSD if possible

### Searches timeout

**Try:**
1. Narrow scope: Search specific directory
2. Use hybrid: Filter with keywords first
3. Lower topk: `--topk 20`

### Out of memory

**Solutions:**
1. Reduce workers: `export CK_WORKERS=4`
2. Exclude more files: Expand `.ckignore`
3. Index subdirectories separately

### Index corruption

**Symptoms:** Crashes, missing results, errors

**Fix:**
```bash
ck --reindex .
```

Rebuilds from scratch.

---

## Next steps

**→** [Configuration](configuration.html) - .ckignore patterns and env vars

**→** [CLI reference](cli-reference.html) - All performance-related flags

**→** [Find patterns](find-patterns.html) - Efficient search strategies
