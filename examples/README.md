# cc Examples

This folder demonstrates practical use cases for `cc` (semantic grep) with real code examples.

## Quick Start

1. **Index the examples first** (required for semantic/lexical/hybrid search):
   ```bash
   cc --index examples/
   ```

2. **Try the quick demo**:
   ```bash
   ./examples/quick_demo.sh
   ```

3. **Try the examples below** using the demo files in this folder.

## Demo Files

### Code Examples (`code/`)
- **`full_section_demo.py`** - Python code with classes, functions, error handling  
- **`web_server.rs`** - Rust web server with authentication, database connections, error handling
- **`api_client.js`** - JavaScript API client with HTTP requests, retry logic, authentication

### Text Samples (`text_samples/`)
- **`fixtures/`** - Simple demo text files (demo1.txt to demo10.txt)
- **`wiki_articles/`** - Wikipedia articles on various topics (AI, science, technology, etc.)

## Search Mode Examples

### 1. **Regex Search** (Default - no indexing required)
Classic grep-style pattern matching:

```bash
# Basic text search
cc "error" examples/

# Case-insensitive search  
cc -i "database" examples/

# Whole word matching
cc -w "class" examples/

# Fixed string (no regex)
cc -F "def __init__" examples/

# With line numbers and context
cc -n -C 2 "error" examples/
```

### 2. **Semantic Search** (Finds conceptually similar code)
Understands meaning, not just text matches:

```bash
# Find error handling patterns (try/catch, exception handling, etc.)
cc --sem "error handling" examples/

# Find database-related code
cc --sem "database connection" text_samples/

# Find authentication logic
cc --sem "user authentication" examples/

# Limit results and show scores
cc --sem --limit 5 --scores "data processing" examples/

# Higher precision filtering
cc --sem --threshold 0.8 "function" examples/
```

### 3. **Lexical Search** (BM25 full-text search with ranking)
Better than regex for phrase matching:

```bash
# Full-text search with relevance ranking
cc --lex "user authentication" examples/

# Multi-word phrases
cc --lex "error handling patterns" examples/
```

### 4. **Hybrid Search** (Best of both worlds)
Combines regex precision with semantic understanding:

```bash
# Most comprehensive search
cc --hybrid "async function" examples/

# Find specific patterns with semantic context
cc --hybrid "database" --limit 10 examples/

# Show relevance scores
cc --hybrid "error" --scores examples/
```

## Advanced Features

### Full Section Mode
Get complete code blocks instead of just matching lines:

```bash
# Return entire functions/classes containing matches
cc --sem --full-section "error handling" examples/

# Works with all search modes
cc --hybrid --full-section "database" examples/
```

### JSON Output
Machine-readable results for scripts and tools:

```bash
# JSON output for integration
cc --json --sem "error handling" examples/

# Pipe to jq for processing
cc --json --sem "database" examples/ | jq '.preview'
```

### Context and Formatting
```bash
# Show surrounding lines
cc -A 3 -B 1 "class" examples/    # 3 after, 1 before
cc -C 2 "def" examples/           # 2 lines of context

# File listing modes
cc -l "error" examples/           # List files with matches
cc -L "nonexistent" examples/     # List files without matches
```

## Practical Use Cases

### 1. **Code Review**
```bash
# Find potential security issues
cc --sem "sql injection" examples/
cc --sem "password storage" examples/

# Find error handling patterns
cc --sem "exception handling" examples/
```

### 2. **Code Exploration** 
```bash
# Understand new codebase
cc --sem "main entry point" examples/
cc --sem "configuration setup" examples/

# Find similar implementations
cc --sem "data validation" examples/
```

### 3. **Refactoring**
```bash
# Find deprecated patterns
cc --hybrid "old api" examples/

# Locate specific implementations
cc --sem "async operations" examples/
```

### 4. **Documentation**
```bash
# Find examples of usage
cc --sem "example usage" examples/

# Locate API patterns
cc --sem "rest api" examples/
```

## Performance Tips

- **Index once, search many times**: Use `cc --index` before semantic searches
- **Use appropriate search modes**: 
  - Regex for exact patterns
  - Semantic for concept-based searches  
  - Hybrid for comprehensive results
- **Adjust thresholds**: Lower values = more results, higher = more precise
- **Use --limit**: Control result count for large codebases

## Try It Yourself

Run these commands to see `cc` in action:

```bash
# Index the examples
cc --index examples/

# Compare different search modes on the same query
echo "=== Regex ==="
cc "error" examples/
echo "=== Semantic ==="  
cc --sem "error" examples/
echo "=== Hybrid ==="
cc --hybrid "error" examples/
```

Notice how each mode finds different but relevant results!