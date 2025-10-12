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

**Goal:** Customize ck's behavior with configuration files, environment variables, and filtering options.

**You'll learn:**
- .ckignore file configuration
- Environment variables
- Model selection
- Performance tuning
- Index management

---

## .ckignore Files

### Creating .ckignore

Create a `.ckignore` file in your repository root to control what files ck indexes:

```bash
# .ckignore
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

### .ckignore Syntax

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

### Global .ckignore

Create a global ignore file:

```bash
# ~/.ckignore
# Global exclusions
.DS_Store
Thumbs.db
*.tmp
*.swp
*.swo
```

Set environment variable:
```bash
export CK_GLOBAL_IGNORE="$HOME/.ckignore"
```

---

## Environment Variables

### Model Configuration

```bash
# Embedding model selection
export CK_MODEL=default          # default, large
export CK_MODEL_PATH=/path/to/model  # Custom model path

# Model parameters
export CK_CHUNK_SIZE=512         # Chunk size for embeddings
export CK_BATCH_SIZE=32          # Batch size for processing
```

### Index Configuration

```bash
# Index location
export CK_INDEX_PATH=/custom/path  # Default: .ck/

# Index management
export CK_WORKERS=8              # Worker threads (default: CPU cores)
export CK_MEMORY_LIMIT=2GB       # Memory limit
export CK_CACHE_SIZE=1GB         # Cache size
```

### Performance Configuration

```bash
# Search performance
export CK_SEARCH_THREADS=4       # Search threads
export CK_MAX_RESULTS=1000       # Maximum results
export CK_TIMEOUT=30             # Search timeout (seconds)

# Indexing performance
export CK_INDEX_THREADS=8        # Indexing threads
export CK_INDEX_BATCH_SIZE=100   # Indexing batch size
```

### Output Configuration

```bash
# Output format
export CK_NO_COLOR=1             # Disable colored output
export CK_JSON_OUTPUT=1          # JSON output
export CK_VERBOSE=1              # Verbose logging

# Debug options
export CK_DEBUG=1                # Debug mode
export CK_LOG_LEVEL=info         # Log level (debug, info, warn, error)
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
ck --sem "pattern" .

# Use large model
ck --sem "pattern" --model large .

# Use custom model
ck --sem "pattern" --model-path /path/to/model .
```

**Environment variable:**
```bash
export CK_MODEL=large
ck --sem "pattern" .
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
export CK_WORKERS=8

# Reduce for memory-constrained systems
export CK_WORKERS=4
```

**Chunk size:**
```bash
# Larger chunks = fewer embeddings, faster indexing
export CK_CHUNK_SIZE=1024

# Smaller chunks = more embeddings, better accuracy
export CK_CHUNK_SIZE=256
```

**Batch size:**
```bash
# Larger batches = faster processing, more memory
export CK_BATCH_SIZE=64

# Smaller batches = less memory, slower processing
export CK_BATCH_SIZE=16
```

### Search Performance

**Result limits:**
```bash
# Limit results for faster search
ck --sem "pattern" --topk 50 .

# Use thresholds to filter results
ck --sem "pattern" --threshold 0.8 .
```

**Memory usage:**
```bash
# Limit memory usage
export CK_MEMORY_LIMIT=1GB

# Adjust cache size
export CK_CACHE_SIZE=512MB
```

---

## Index Management

### Index Location

**Default location:**
```
.ck/
├── index.bin
├── embeddings.bin
├── metadata.json
└── chunks/
```

**Custom location:**
```bash
export CK_INDEX_PATH=/custom/path
```

### Index Operations

**Check index status:**
```bash
ls -la .ck/
du -sh .ck/
```

**Force reindex:**
```bash
rm -rf .ck/
ck --sem "test" .  # Rebuilds index
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
- Use .ckignore to exclude unnecessary files
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
ck --sem "pattern" --threshold 0.8 .
```

**More results:**
```bash
ck --sem "pattern" --topk 200 .
```

**More context:**
```bash
ck --sem "pattern" --context 5 .
```

**Shorter snippets:**
```bash
ck --sem "pattern" --snippet-length 200 .
```

---

## Language-Specific Configuration

### Supported Languages

ck supports 15+ programming languages:
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
export CK_CHUNKING_RULES=/path/to/rules.json
```

---

## Configuration Files

### Project Configuration

**Create `ck.toml` in project root:**
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

**Create `~/.config/ck/config.toml`:**
```toml
[model]
default_model = "default"
model_cache_dir = "~/.cache/ck/models"

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
echo $CK_MODEL_PATH

# Download model
ck --download-model large
```

**Index corruption:**
```bash
# Remove corrupted index
rm -rf .ck/

# Rebuild index
ck --sem "test" .
```

**Memory issues:**
```bash
# Reduce workers
export CK_WORKERS=2

# Reduce memory limit
export CK_MEMORY_LIMIT=512MB
```

**Performance issues:**
```bash
# Check system resources
top -p $(pgrep ck)

# Optimize .ckignore
# Reduce chunk size
export CK_CHUNK_SIZE=256
```

### Configuration Validation

**Test configuration:**
```bash
# Test with debug output
CK_DEBUG=1 ck --sem "test" .

# Check configuration
ck --config-check
```

**Validate .ckignore:**
```bash
# Test ignore patterns
ck --test-ignore .ckignore
```

---

## Best Practices

### Configuration Management

**Version control:**
- Commit `.ckignore` to repository
- Don't commit `.ck/` directory
- Document environment variables

**Team coordination:**
- Share configuration files
- Document custom settings
- Use consistent .ckignore patterns

### Performance Optimization

**For large codebases:**
- Use .ckignore to exclude build artifacts
- Reduce chunk size for memory efficiency
- Use default model for speed

**For development:**
- Use large model for better accuracy
- Increase context for better understanding
- Use higher thresholds for precision

### Security Considerations

**Sensitive files:**
- Add sensitive files to .ckignore
- Use global ignore for system files
- Review configuration before sharing

---

## Next Steps

**→ Optimize performance:** [Large Codebases](large-codebases.html)

**→ Learn search modes:** [Search Modes Explained](../explanation/search-modes.html)

**→ Integrate with editors:** [Editor Integration](editor-integration.html)

**→ Connect AI tools:** [AI Integration](../ai-integration/mcp-quickstart.html)
