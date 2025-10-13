---
layout: default
title: Search Large Codebases
parent: How-To Guides
nav_order: 5
---

# Search Large Codebases

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Optimize cc for repositories with 100k+ files and maintain fast search performance.

**You'll learn:**
- Performance optimization strategies
- Index management for large repos
- Memory usage optimization
- Search speed improvements
- Scaling considerations

---

## Understanding Large Codebase Challenges

### Typical Large Repository Characteristics

- **100k+ files** - Enterprise applications, monorepos
- **Multi-language** - Mixed codebases with different languages
- **Deep directory structures** - Complex project organization
- **Generated code** - Build artifacts, auto-generated files
- **Binary files** - Images, compiled assets, data files

### Performance Impact

| Repository Size | Index Time | Index Size | Search Time |
|-----------------|------------|------------|-------------|
| Small (1k files) | 1-2 seconds | 10-50MB | <100ms |
| Medium (10k files) | 5-10 seconds | 100-500MB | <100ms |
| Large (100k files) | 30-60 seconds | 1-5GB | <200ms |
| Very Large (1M files) | 5-10 minutes | 10-50GB | <500ms |

---

## Optimization Strategies

### 1. File Filtering with .ccignore

Create a `.ccignore` file in your repository root:

```bash
# Exclude build artifacts
build/
dist/
target/
node_modules/
.git/

# Exclude generated files
*.generated.js
*.generated.ts
*.pb.go
*.pb.rs

# Exclude large data files
*.csv
*.json
*.xml
*.log

# Exclude binary files
*.png
*.jpg
*.jpeg
*.gif
*.ico
*.pdf
*.zip
*.tar.gz

# Exclude specific directories
docs/generated/
tests/fixtures/
examples/data/

# Keep important files
!important-config.json
!package.json
!Cargo.toml
```

### 2. Language-Specific Optimization

Focus on the languages that matter most:

```bash
# Only index specific languages
cc --sem "error handling" --glob "*.rs" src/
cc --sem "authentication" --glob "*.{js,ts}" src/
cc --sem "database" --glob "*.{py,js,go}" src/
```

### 3. Directory-Specific Search

Search in specific directories to reduce scope:

```bash
# Search only source code
cc --sem "pattern" src/

# Search only tests
cc --sem "test" tests/

# Search only documentation
cc --sem "API" docs/
```

---

## Index Management

### Understanding Index Storage

Indexes are stored in `.cc/` directory:

```
.cc/
├── index.bin          # Main search index
├── embeddings.bin     # Semantic embeddings
├── metadata.json      # Index metadata
└── chunks/            # Code chunk data
    ├── chunk_001.bin
    ├── chunk_002.bin
    └── ...
```

### Index Size Optimization

**Check index size:**
```bash
du -sh .cc/
```

**Typical index sizes:**
- **Small repo (1k files):** 10-50MB
- **Medium repo (10k files):** 100-500MB  
- **Large repo (100k files):** 1-5GB
- **Very large repo (1M files):** 10-50GB

### Force Reindexing

When to reindex:
- After major refactoring
- When files changed outside cc
- After updating .ccignore
- When index seems corrupted

```bash
# Force full reindex
rm -rf .cc/
cc --sem "test" .  # This will rebuild the index
```

---

## Memory Usage Optimization

### Environment Variables

```bash
# Limit worker threads (default: CPU cores)
export CC_WORKERS=4

# Adjust chunk size for embeddings
export CC_CHUNK_SIZE=512

# Set memory limit
export CC_MEMORY_LIMIT=2GB
```

### System Requirements

**Minimum for large repos:**
- **RAM:** 4GB available
- **Disk:** 20GB free space
- **CPU:** 4+ cores recommended

**Recommended for very large repos:**
- **RAM:** 8GB+ available
- **Disk:** 100GB+ free space (SSD preferred)
- **CPU:** 8+ cores

---

## Search Performance Tips

### 1. Use Appropriate Search Modes

**For large repos, prefer:**
- **Regex search** for exact patterns (fastest)
- **Hybrid search** for keyword + semantic (balanced)
- **Semantic search** for concept discovery (most accurate)

```bash
# Fast exact search
cc "fn test_" tests/

# Balanced keyword + semantic
cc --hybrid "authentication" src/

# Concept discovery
cc --sem "error handling" src/
```

### 2. Optimize Query Specificity

**More specific queries = faster results:**

```bash
# Good: Specific and focused
cc --sem "JWT token validation" src/auth/

# Less optimal: Too broad
cc --sem "auth" .
```

### 3. Use Thresholds Effectively

```bash
# High precision, fewer results
cc --sem "pattern" --threshold 0.8 src/

# Broader search, more results
cc --sem "pattern" --threshold 0.5 src/
```

### 4. Limit Result Count

```bash
# Limit to top 10 results
cc --sem "pattern" --topk 10 src/

# Default is 100, max is 1000
cc --sem "pattern" --topk 50 src/
```

---

## Scaling Considerations

### Monorepo Strategies

**For monorepos with multiple projects:**

```bash
# Search specific project
cc --sem "pattern" apps/frontend/src/

# Search shared libraries
cc --sem "pattern" libs/shared/src/

# Search across all projects
cc --sem "pattern" .
```

### Distributed Development

**For teams working on large codebases:**

```bash
# Share index (advanced)
# Copy .cc/ directory to team members
# Note: Indexes are machine-specific, sharing not recommended

# Better: Each developer builds their own index
# Index building is fast after first time
```

### CI/CD Integration

**For automated workflows:**

```bash
# In CI pipeline
cc --sem "security" --threshold 0.8 src/ > security_scan.txt
cc --sem "performance" --threshold 0.7 src/ > performance_scan.txt
```

---

## Monitoring and Diagnostics

### Check Index Health

```bash
# Check if index exists
ls -la .cc/

# Check index metadata
cat .cc/metadata.json
```

### Performance Monitoring

**Track search performance:**
```bash
# Time your searches
time cc --sem "pattern" src/

# Monitor memory usage
top -p $(pgrep cc)
```

### Common Issues and Solutions

**Issue: Slow first search**
- **Cause:** Index building
- **Solution:** Normal behavior, subsequent searches are fast

**Issue: High memory usage**
- **Cause:** Large index in memory
- **Solution:** Reduce CC_WORKERS, optimize .ccignore

**Issue: Disk space full**
- **Cause:** Large index files
- **Solution:** Clean up .ccignore, remove old indexes

**Issue: Search returns no results**
- **Cause:** Files excluded by .ccignore
- **Solution:** Check .ccignore rules, use --debug flag

---

## Advanced Techniques

### Parallel Indexing

```bash
# Use multiple cores for indexing
export CC_WORKERS=8
cc --sem "test" .  # Will use 8 threads
```

### Incremental Updates

cc automatically updates indexes when files change:
- **File modified:** Index updated incrementally
- **File added:** Added to index
- **File deleted:** Removed from index
- **No changes:** Index reused

### Custom Chunking

For very large files, consider splitting:

```bash
# Search in specific file ranges
cc --sem "pattern" --glob "*.rs" src/ | head -100
```

---

## Best Practices

### 1. Regular Maintenance

```bash
# Weekly: Check index size
du -sh .cc/

# Monthly: Clean up old indexes
find . -name ".cc" -type d -mtime +30 -exec rm -rf {} \;

# As needed: Reindex after major changes
rm -rf .cc/ && cc --sem "test" .
```

### 2. Team Coordination

- **Share .ccignore** via version control
- **Document search strategies** for common patterns
- **Set up CI/CD** with cc for automated analysis

### 3. Performance Monitoring

- **Track search times** for common queries
- **Monitor index sizes** across different repos
- **Optimize .ccignore** based on usage patterns

---

## Troubleshooting

### Common Problems

**Problem: Index building takes forever**
```bash
# Check what's being indexed
cc --sem "test" --debug .

# Optimize .ccignore
# Reduce CC_WORKERS if memory constrained
```

**Problem: Search is slow**
```bash
# Use more specific queries
# Try regex instead of semantic
# Reduce --topk value
# Increase --threshold
```

**Problem: Out of memory**
```bash
# Reduce CC_WORKERS
export CC_WORKERS=2

# Optimize .ccignore to exclude large files
# Consider searching smaller directories
```

---

## Next Steps

**→ Optimize further:** [Performance Tuning](performance-tuning.html)

**→ Configure filtering:** [Configuration](configuration.html)

**→ Integrate with CI/CD:** [CI/CD Integration](ci-cd.html)

**→ Learn about architecture:** [Architecture Deep Dive](../explanation/architecture.html)
