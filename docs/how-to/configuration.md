---
layout: default
title: Configuration
parent: How-To Guides
nav_order: 6
---

# Configuration

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Customize cc's behavior with configuration files, environment variables, and filtering options.

**You'll learn:**
- .ccignore file configuration
- Environment variables
- Model selection
- Performance tuning
- Index management

---

## .ccignore Files

### Creating .ccignore

Create a `.ccignore` file in your repository root to control what files cc indexes:

```bash
# .ccignore
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

### .ccignore Syntax

**Pattern matching:**
- `*` - Matches any characters
- `?` - Matches single character
- `**` - Matches directories recursively
- `!` - Negation (include file)

**Examples:**
```bash
# Exclude all .log files
*.log

# Exclude build directories
build/
dist/

# Exclude all files in generated/ subdirectories
**/generated/**

# Exclude but keep specific file
*.json
!important-config.json

# Exclude by extension
*.{png,jpg,jpeg}

# Exclude by pattern
*test*
*_test.*
```

### Global .ccignore

Create a global ignore file:

```bash
# ~/.ccignore
# Global exclusions
.DS_Store
Thumbs.db
*.tmp
*.swp
*.swo
```

Set environment variable:
```bash
export CC_GLOBAL_IGNORE="$HOME/.ccignore"
```

---

## Environment Variables

### Model Configuration

```bash
# Embedding model selection
export CC_MODEL=default          # default, large
export CC_MODEL_PATH=/path/to/model  # Custom model path

# Model parameters
export CC_CHUNK_SIZE=512         # Chunk size for embeddings
export CC_BATCH_SIZE=32          # Batch size for processing
```

### Index Configuration

```bash
# Index location
export CC_INDEX_PATH=/custom/path  # Default: .cc/

# Index management
export CC_WORKERS=8              # Worker threads (default: CPU cores)
export CC_MEMORY_LIMIT=2GB       # Memory limit
export CC_CACHE_SIZE=1GB         # Cache size
```

### Performance Configuration

```bash
# Search performance
export CC_SEARCH_THREADS=4       # Search threads
export CC_MAX_RESULTS=1000       # Maximum results
export CC_TIMEOUT=30             # Search timeout (seconds)

# Indexing performance
export CC_INDEX_THREADS=8        # Indexing threads
export CC_INDEX_BATCH_SIZE=100   # Indexing batch size
```

### Output Configuration

```bash
# Output format
export CC_NO_COLOR=1             # Disable colored output
export CC_JSON_OUTPUT=1          # JSON output
export CC_VERBOSE=1              # Verbose logging

# Debug options
export CC_DEBUG=1                # Debug mode
export CC_LOG_LEVEL=info         # Log level (debug, info, warn, error)
```

---

## Model Selection

### Available Models

**Default model:**
- **Size:** ~100MB
- **Speed:** Fast
- **Accuracy:** Good for most use cases
- **Best for:** General code search

**Large model:**
- **Size:** ~400MB
- **Speed:** Slower
- **Accuracy:** Better for complex concepts
- **Best for:** Research, complex codebases

### Selecting a Model

**Command line:**
```bash
# Use default model
cc --sem "pattern" .

# Use large model
cc --sem "pattern" --model large .

# Use custom model
cc --sem "pattern" --model-path /path/to/model .
```

**Environment variable:**
```bash
export CC_MODEL=large
cc --sem "pattern" .
```

### Model Performance

| Model | Size | Speed | Accuracy | Use Case |
|-------|------|-------|----------|----------|
| default | 100MB | Fast | Good | General search |
| large | 400MB | Slower | Better | Complex concepts |

---

## Performance Tuning

### Indexing Performance

**Worker threads:**
```bash
# Use all CPU cores (default)
export CC_WORKERS=8

# Reduce for memory-constrained systems
export CC_WORKERS=4
```

**Chunk size:**
```bash
# Larger chunks = fewer embeddings, faster indexing
export CC_CHUNK_SIZE=1024

# Smaller chunks = more embeddings, better accuracy
export CC_CHUNK_SIZE=256
```

**Batch size:**
```bash
# Larger batches = faster processing, more memory
export CC_BATCH_SIZE=64

# Smaller batches = less memory, slower processing
export CC_BATCH_SIZE=16
```

### Search Performance

**Result limits:**
```bash
# Limit results for faster search
cc --sem "pattern" --topk 50 .

# Use thresholds to filter results
cc --sem "pattern" --threshold 0.8 .
```

**Memory usage:**
```bash
# Limit memory usage
export CC_MEMORY_LIMIT=1GB

# Adjust cache size
export CC_CACHE_SIZE=512MB
```

---

## Index Management

### Index Location

**Default location:**
```
.cc/
├── index.bin
├── embeddings.bin
├── metadata.json
└── chunks/
```

**Custom location:**
```bash
export CC_INDEX_PATH=/custom/path
```

### Index Operations

**Check index status:**
```bash
ls -la .cc/
du -sh .cc/
```

**Force reindex:**
```bash
rm -rf .cc/
cc --sem "test" .  # Rebuilds index
```

**Incremental updates:**
- Index updates automatically when files change
- No manual intervention needed
- Very fast for small changes

### Index Size Optimization

**Typical sizes:**
- **Small repo (1k files):** 10-50MB
- **Medium repo (10k files):** 100-500MB
- **Large repo (100k files):** 1-5GB

**Optimization strategies:**
- Use .ccignore to exclude unnecessary files
- Reduce chunk size for smaller indexes
- Use default model instead of large model

---

## Search Configuration

### Default Search Parameters

```bash
# Default values
--threshold 0.6
--topk 100
--context 2
--snippet-length 500
```

### Customizing Search Behavior

**Higher precision:**
```bash
cc --sem "pattern" --threshold 0.8 .
```

**More results:**
```bash
cc --sem "pattern" --topk 200 .
```

**More context:**
```bash
cc --sem "pattern" --context 5 .
```

**Shorter snippets:**
```bash
cc --sem "pattern" --snippet-length 200 .
```

---

## Language-Specific Configuration

### Supported Languages

cc supports 15+ programming languages:
- Rust, JavaScript, TypeScript, Python, Go, Java, C++, C#
- Ruby, PHP, Swift, Kotlin, Scala, Haskell, Zig

### Language-Specific Settings

**Chunking strategies:**
- **Rust:** Functions, impl blocks, modules
- **JavaScript:** Functions, classes, modules
- **Python:** Functions, classes, modules
- **Go:** Functions, methods, packages

**Custom chunking (future):**
```bash
# Custom chunking rules
export CC_CHUNKING_RULES=/path/to/rules.json
```

---

## Configuration Files

### Project Configuration

**Create `cc.toml` in project root:**
```toml
[model]
name = "large"
chunk_size = 512
batch_size = 32

[index]
workers = 8
memory_limit = "2GB"
cache_size = "1GB"

[search]
default_threshold = 0.7
default_topk = 50
default_context = 3

[ignore]
patterns = [
    "*.log",
    "build/",
    "dist/",
    "node_modules/"
]
```

### Global Configuration

**Create `~/.config/cc/config.toml`:**
```toml
[model]
default_model = "default"
model_cache_dir = "~/.cache/cc/models"

[index]
default_workers = 4
default_memory_limit = "1GB"

[search]
default_format = "human"
enable_colors = true
```

---

## Troubleshooting

### Common Configuration Issues

**Model not found:**
```bash
# Check model path
echo $CC_MODEL_PATH

# Download model
cc --download-model large
```

**Index corruption:**
```bash
# Remove corrupted index
rm -rf .cc/

# Rebuild index
cc --sem "test" .
```

**Memory issues:**
```bash
# Reduce workers
export CC_WORKERS=2

# Reduce memory limit
export CC_MEMORY_LIMIT=512MB
```

**Performance issues:**
```bash
# Check system resources
top -p $(pgrep cc)

# Optimize .ccignore
# Reduce chunk size
export CC_CHUNK_SIZE=256
```

### Configuration Validation

**Test configuration:**
```bash
# Test with debug output
CC_DEBUG=1 cc --sem "test" .

# Check configuration
cc --config-check
```

**Validate .ccignore:**
```bash
# Test ignore patterns
cc --test-ignore .ccignore
```

---

## Best Practices

### Configuration Management

**Version control:**
- Commit `.ccignore` to repository
- Don't commit `.cc/` directory
- Document environment variables

**Team coordination:**
- Share configuration files
- Document custom settings
- Use consistent .ccignore patterns

### Performance Optimization

**For large codebases:**
- Use .ccignore to exclude build artifacts
- Reduce chunk size for memory efficiency
- Use default model for speed

**For development:**
- Use large model for better accuracy
- Increase context for better understanding
- Use higher thresholds for precision

### Security Considerations

**Sensitive files:**
- Add sensitive files to .ccignore
- Use global ignore for system files
- Review configuration before sharing

---

## Next Steps

**→ Optimize performance:** [Large Codebases](large-codebases.html)

**→ Learn search modes:** [Search Modes Explained](../explanation/search-modes.html)

**→ Integrate with editors:** [Editor Integration](editor-integration.html)

**→ Connect AI tools:** [AI Integration](../ai-integration/mcp-quickstart.html)
