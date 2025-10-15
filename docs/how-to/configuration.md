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

**Goal:** Customize cs's behavior with configuration files, environment variables, and filtering options.

**You'll learn:**
- .csignore file configuration
- Environment variables
- Model selection
- Performance tuning
- Index management

---

## .csignore Files

### Creating .csignore

Create a `.csignore` file in your repository root to control what files cc indexes:

```bash
# .csignore
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

### .csignore Syntax

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

### Global .csignore

Create a global ignore file:

```bash
# ~/.csignore
# Global exclusions
.DS_Store
Thumbs.db
*.tmp
*.swp
*.swo
```

Set environment variable:
```bash
export CS_GLOBAL_IGNORE="$HOME/.csignore"
```

---

## Environment Variables

### Model Configuration

```bash
# Embedding model selection
export CS_MODEL=default          # default, large
export CS_MODEL_PATH=/path/to/model  # Custom model path

# Model parameters
export CS_CHUNK_SIZE=512         # Chunk size for embeddings
export CS_BATCH_SIZE=32          # Batch size for processing
```

### Index Configuration

```bash
# Index location
export CS_INDEX_PATH=/custom/path  # Default: .cs/

# Index management
export CS_WORKERS=8              # Worker threads (default: CPU cores)
export CS_MEMORY_LIMIT=2GB       # Memory limit
export CS_CACHE_SIZE=1GB         # Cache size
```

### Performance Configuration

```bash
# Search performance
export CS_SEARCH_THREADS=4       # Search threads
export CS_MAX_RESULTS=1000       # Maximum results
export CS_TIMEOUT=30             # Search timeout (seconds)

# Indexing performance
export CS_INDEX_THREADS=8        # Indexing threads
export CS_INDEX_BATCH_SIZE=100   # Indexing batch size
```

### Output Configuration

```bash
# Output format
export CS_NO_COLOR=1             # Disable colored output
export CS_JSON_OUTPUT=1          # JSON output
export CS_VERBOSE=1              # Verbose logging

# Debug options
export CS_DEBUG=1                # Debug mode
export CS_LOG_LEVEL=info         # Log level (debug, info, warn, error)
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
cs --sem "pattern" .

# Use large model
cs --sem "pattern" --model large .

# Use custom model
cs --sem "pattern" --model-path /path/to/model .
```

**Environment variable:**
```bash
export CS_MODEL=large
cs --sem "pattern" .
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
export CS_WORKERS=8

# Reduce for memory-constrained systems
export CS_WORKERS=4
```

**Chunk size:**
```bash
# Larger chunks = fewer embeddings, faster indexing
export CS_CHUNK_SIZE=1024

# Smaller chunks = more embeddings, better accuracy
export CS_CHUNK_SIZE=256
```

**Batch size:**
```bash
# Larger batches = faster processing, more memory
export CS_BATCH_SIZE=64

# Smaller batches = less memory, slower processing
export CS_BATCH_SIZE=16
```

### Search Performance

**Result limits:**
```bash
# Limit results for faster search
cs --sem "pattern" --topk 50 .

# Use thresholds to filter results
cs --sem "pattern" --threshold 0.8 .
```

**Memory usage:**
```bash
# Limit memory usage
export CS_MEMORY_LIMIT=1GB

# Adjust cache size
export CS_CACHE_SIZE=512MB
```

---

## Index Management

### Index Location

**Default location:**
```
.cs/
├── index.bin
├── embeddings.bin
├── metadata.json
└── chunks/
```

**Custom location:**
```bash
export CS_INDEX_PATH=/custom/path
```

### Index Operations

**Check index status:**
```bash
ls -la .cs/
du -sh .cs/
```

**Force reindex:**
```bash
rm -rf .cs/
cs --sem "test" .  # Rebuilds index
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
- Use .csignore to exclude unnecessary files
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
cs --sem "pattern" --threshold 0.8 .
```

**More results:**
```bash
cs --sem "pattern" --topk 200 .
```

**More context:**
```bash
cs --sem "pattern" --context 5 .
```

**Shorter snippets:**
```bash
cs --sem "pattern" --snippet-length 200 .
```

---

## Language-Specific Configuration

### Supported Languages

cs supports 15+ programming languages:
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
export CS_CHUNKING_RULES=/path/to/rules.json
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

**Create `~/.config/cs/config.toml`:**
```toml
# Model configuration
index_model = "jina-v4"
query_model = "jina-code-1.5b"

# Search defaults
default_topk = 10
default_threshold = 0.6
default_search_mode = "regex"

# Output formatting
default_output_format = "text"
show_scores_default = false
line_numbers_default = false

# Reranking
rerank_enabled = false
rerank_model = "jina"

# Other preferences
quiet_mode = false
```

**Using the config command:**
```bash
# Initialize config with defaults
cs --config init

# View all settings
cs --config list

# Get specific setting
cs --config get index-model

# Set a value
cs --config set index-model jina-v4

# Show config file path
cs --config path
```

---

## Troubleshooting

### Common Configuration Issues

**Model not found:**
```bash
# Check model path
echo $CS_MODEL_PATH

# Download model
cs --download-model large
```

**Index corruption:**
```bash
# Remove corrupted index
rm -rf .cs/

# Rebuild index
cs --sem "test" .
```

**Memory issues:**
```bash
# Reduce workers
export CS_WORKERS=2

# Reduce memory limit
export CS_MEMORY_LIMIT=512MB
```

**Performance issues:**
```bash
# Check system resources
top -p $(pgrep cs)

# Optimize .csignore
# Reduce chunk size
export CS_CHUNK_SIZE=256
```

### Configuration Validation

**Test configuration:**
```bash
# Test with debug output
CS_DEBUG=1 cs --sem "test" .

# Check configuration
cs --config-check
```

**Validate .csignore:**
```bash
# Test ignore patterns
cs --test-ignore .csignore
```

---

## Best Practices

### Configuration Management

**Version control:**
- Commit `.csignore` to repository
- Don't commit `.cs/` directory
- Document environment variables

**Team coordination:**
- Share configuration files
- Document custom settings
- Use consistent .csignore patterns

### Performance Optimization

**For large codebases:**
- Use .csignore to exclude build artifacts
- Reduce chunk size for memory efficiency
- Use default model for speed

**For development:**
- Use large model for better accuracy
- Increase context for better understanding
- Use higher thresholds for precision

### Security Considerations

**Sensitive files:**
- Add sensitive files to .csignore
- Use global ignore for system files
- Review configuration before sharing

---

## Next Steps

**→ Optimize performance:** [Large Codebases](large-codebases.html)

**→ Learn search modes:** [Search Modes Explained](../explanation/search-modes.html)

**→ Integrate with editors:** [Editor Integration](editor-integration.html)

**→ Connect AI tools:** [AI Integration](../ai-integration/mcp-quickstart.html)
