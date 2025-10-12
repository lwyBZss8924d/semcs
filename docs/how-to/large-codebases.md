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

**Goal:** Optimize ck for repositories with 100k+ files and maintain fast search performance.

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

### 1. File Filtering with .ckignore

Create a `.ckignore` file in your repository root:

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
ck --sem "error handling" --glob "*.rs" src/
ck --sem "authentication" --glob "*.{js,ts}" src/
ck --sem "database" --glob "*.{py,js,go}" src/
```

### 3. Directory-Specific Search

Search in specific directories to reduce scope:

```bash
# Search only source code
ck --sem "pattern" src/

# Search only tests
ck --sem "test" tests/

# Search only documentation
ck --sem "API" docs/
```

---

## Index Management

### Understanding Index Storage

Indexes are stored in `.ck/` directory:

```
.ck/
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
du -sh .ck/
```

**Typical index sizes:**
- **Small repo (1k files):** 10-50MB
- **Medium repo (10k files):** 100-500MB  
- **Large repo (100k files):** 1-5GB
- **Very large repo (1M files):** 10-50GB

### Force Reindexing

When to reindex:
- After major refactoring
- When files changed outside ck
- After updating .ckignore
- When index seems corrupted

```bash
# Force full reindex
rm -rf .ck/
ck --sem "test" .  # This will rebuild the index
```

---

## Memory Usage Optimization

### Environment Variables

```bash
# Limit worker threads (default: CPU cores)
export CK_WORKERS=4

# Adjust chunk size for embeddings
export CK_CHUNK_SIZE=512

# Set memory limit
export CK_MEMORY_LIMIT=2GB
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
ck "fn test_" tests/

# Balanced keyword + semantic
ck --hybrid "authentication" src/

# Concept discovery
ck --sem "error handling" src/
```

### 2. Optimize Query Specificity

**More specific queries = faster results:**

```bash
# Good: Specific and focused
ck --sem "JWT token validation" src/auth/

# Less optimal: Too broad
ck --sem "auth" .
```

### 3. Use Thresholds Effectively

```bash
# High precision, fewer results
ck --sem "pattern" --threshold 0.8 src/

# Broader search, more results
ck --sem "pattern" --threshold 0.5 src/
```

### 4. Limit Result Count

```bash
# Limit to top 10 results
ck --sem "pattern" --topk 10 src/

# Default is 100, max is 1000
ck --sem "pattern" --topk 50 src/
```

---

## Scaling Considerations

### Monorepo Strategies

**For monorepos with multiple projects:**

```bash
# Search specific project
ck --sem "pattern" apps/frontend/src/

# Search shared libraries
ck --sem "pattern" libs/shared/src/

# Search across all projects
ck --sem "pattern" .
```

### Distributed Development

**For teams working on large codebases:**

```bash
# Share index (advanced)
# Copy .ck/ directory to team members
# Note: Indexes are machine-specific, sharing not recommended

# Better: Each developer builds their own index
# Index building is fast after first time
```

### CI/CD Integration

**For automated workflows:**

```bash
# In CI pipeline
ck --sem "security" --threshold 0.8 src/ > security_scan.txt
ck --sem "performance" --threshold 0.7 src/ > performance_scan.txt
```

---

## Monitoring and Diagnostics

### Check Index Health

```bash
# Check if index exists
ls -la .ck/

# Check index metadata
cat .ck/metadata.json
```

### Performance Monitoring

**Track search performance:**
```bash
# Time your searches
time ck --sem "pattern" src/

# Monitor memory usage
top -p $(pgrep ck)
```

### Common Issues and Solutions

**Issue: Slow first search**
- **Cause:** Index building
- **Solution:** Normal behavior, subsequent searches are fast

**Issue: High memory usage**
- **Cause:** Large index in memory
- **Solution:** Reduce CK_WORKERS, optimize .ckignore

**Issue: Disk space full**
- **Cause:** Large index files
- **Solution:** Clean up .ckignore, remove old indexes

**Issue: Search returns no results**
- **Cause:** Files excluded by .ckignore
- **Solution:** Check .ckignore rules, use --debug flag

---

## Advanced Techniques

### Parallel Indexing

```bash
# Use multiple cores for indexing
export CK_WORKERS=8
ck --sem "test" .  # Will use 8 threads
```

### Incremental Updates

ck automatically updates indexes when files change:
- **File modified:** Index updated incrementally
- **File added:** Added to index
- **File deleted:** Removed from index
- **No changes:** Index reused

### Custom Chunking

For very large files, consider splitting:

```bash
# Search in specific file ranges
ck --sem "pattern" --glob "*.rs" src/ | head -100
```

---

## Best Practices

### 1. Regular Maintenance

```bash
# Weekly: Check index size
du -sh .ck/

# Monthly: Clean up old indexes
find . -name ".ck" -type d -mtime +30 -exec rm -rf {} \;

# As needed: Reindex after major changes
rm -rf .ck/ && ck --sem "test" .
```

### 2. Team Coordination

- **Share .ckignore** via version control
- **Document search strategies** for common patterns
- **Set up CI/CD** with ck for automated analysis

### 3. Performance Monitoring

- **Track search times** for common queries
- **Monitor index sizes** across different repos
- **Optimize .ckignore** based on usage patterns

---

## Troubleshooting

### Common Problems

**Problem: Index building takes forever**
```bash
# Check what's being indexed
ck --sem "test" --debug .

# Optimize .ckignore
# Reduce CK_WORKERS if memory constrained
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
# Reduce CK_WORKERS
export CK_WORKERS=2

# Optimize .ckignore to exclude large files
# Consider searching smaller directories
```

---

## Next Steps

**→ Optimize further:** [Performance Tuning](performance-tuning.html)

**→ Configure filtering:** [Configuration](configuration.html)

**→ Integrate with CI/CD:** [CI/CD Integration](ci-cd.html)

**→ Learn about architecture:** [Architecture Deep Dive](../explanation/architecture.html)
