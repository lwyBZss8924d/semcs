---
layout: default
title: How Semantic Search Works
parent: Explanation
nav_order: 1
---

# How Semantic Search Works

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Understand the technology behind ck's semantic code search.

**You'll learn:**
- How embeddings work
- Code chunking strategies
- Similarity search algorithms
- Ranking and relevance scoring
- Local processing architecture

---

## Overview

Semantic search transforms the way we find code by understanding **what code does** rather than just what it says. Instead of matching exact text, it finds code that serves similar purposes or implements similar concepts.

### Traditional vs Semantic Search

**Traditional search (grep):**
```
Query: "error handling"
Finds: Lines containing "error" and "handling"
```

**Semantic search (ck):**
```
Query: "error handling"
Finds: try/catch blocks, Result types, match expressions, 
       error returns, validation logic, panic handlers
```

---

## The Technology Stack

### 1. Code Parsing (Tree-sitter)

**What it does:** Converts source code into Abstract Syntax Trees (ASTs)

**Why it matters:** Preserves code structure and meaning

```rust
// Code
fn handle_error(e: Error) -> Result<()> {
    match e {
        Error::Io(err) => log::error!("IO error: {}", err),
        _ => {}
    }
}

// AST representation
Function {
    name: "handle_error",
    parameters: [Error],
    return_type: Result<()>,
    body: Match {
        expression: e,
        arms: [
            Arm { pattern: Error::Io(err), body: log::error!(...) },
            Arm { pattern: _, body: {} }
        ]
    }
}
```

### 2. Code Chunking

**What it does:** Extracts meaningful code units from ASTs

**Chunk types:**
- Functions and methods
- Classes and structs
- Modules and namespaces
- Impl blocks
- Test functions

**Why chunks matter:**
- Preserve context and relationships
- Better semantic understanding
- More accurate than line-by-line search

### 3. Embedding Generation

**What it does:** Converts code chunks into high-dimensional vectors

**How it works:**
1. Code chunk is tokenized
2. Tokens are fed to embedding model
3. Model outputs semantic vector (typically 768-1536 dimensions)

**Example embedding:**
```
Code: "fn handle_error(e: Error) -> Result<()> { ... }"
Vector: [0.23, -0.45, 0.67, 0.12, -0.89, ...] (768 dimensions)
```

### 4. Vector Storage

**What it does:** Stores embeddings in efficient data structures

**Storage format:**
- Quantized vectors for memory efficiency
- Hierarchical indexing for fast retrieval
- Incremental updates for file changes

### 5. Similarity Search

**What it does:** Finds vectors most similar to query vector

**Algorithm:** Approximate Nearest Neighbor (ANN) search
- Uses cosine similarity for comparison
- Optimized for speed with approximate results
- Supports real-time search on large codebases

---

## Embedding Models

### Local Models

ck uses **local embedding models** that run entirely on your machine:

**Benefits:**
- **Privacy:** Your code never leaves your system
- **Speed:** No network latency
- **Reliability:** No dependency on external services
- **Cost:** No API fees

**Model options:**
- **Default model:** Fast, good accuracy, ~100MB
- **Large model:** Slower, better accuracy, ~400MB

### How Embeddings Understand Code

**Semantic relationships:**
```
"error handling" ≈ "exception management" ≈ "try/catch"
"authentication" ≈ "login" ≈ "user verification"
"database query" ≈ "SQL" ≈ "data retrieval"
```

**Cross-language understanding:**
```
Rust: "Result<T, E>"
Go: "error" interface
Python: "try/except"
JavaScript: "Promise.catch()"
```

All map to similar semantic vectors for "error handling" concepts.

---

## Similarity Search Process

### 1. Query Processing

```bash
ck --sem "error handling" src/
```

**Steps:**
1. Query "error handling" is tokenized
2. Same embedding model processes query
3. Query vector is generated: `[0.45, -0.23, 0.78, ...]`

### 2. Vector Comparison

**Cosine similarity calculation:**
```
similarity = (A · B) / (||A|| × ||B||)
```

**Example:**
```
Query vector:     [0.45, -0.23, 0.78, 0.12, ...]
Code vector:      [0.42, -0.19, 0.81, 0.15, ...]
Similarity:       0.92 (very high)
```

### 3. Ranking and Filtering

**Scoring process:**
1. Calculate similarity for all indexed chunks
2. Sort by similarity score (highest first)
3. Apply threshold filtering (default: 0.6)
4. Return top-k results (default: 100)

---

## Code Chunking Deep Dive

### Tree-sitter Integration

**Language support:**
- Rust, JavaScript, TypeScript, Python, Go, Java, C++, C#
- Ruby, PHP, Swift, Kotlin, Scala, Haskell, Zig
- And many more via tree-sitter grammars

**Chunking strategy:**
```rust
// Function chunk
fn authenticate_user(credentials: &Credentials) -> Result<User> {
    // Implementation
}

// Class chunk
struct DatabaseConnection {
    // Fields
}

impl DatabaseConnection {
    // Methods
}
```

### Chunk Boundaries

**Semantic boundaries:**
- Function/method boundaries
- Class/struct boundaries
- Module boundaries
- Logical code units

**Context preservation:**
- Include function signature
- Preserve parameter types
- Maintain return types
- Keep relevant comments

---

## Performance Characteristics

### Indexing Performance

**Time complexity:**
- **Parsing:** O(n) where n = lines of code
- **Embedding:** O(m) where m = number of chunks
- **Indexing:** O(m log m) for hierarchical indexing

**Typical performance:**
- **Small repo (1k files):** 1-2 seconds
- **Medium repo (10k files):** 5-10 seconds
- **Large repo (100k files):** 30-60 seconds

### Search Performance

**Time complexity:**
- **Vector search:** O(log m) with approximate algorithms
- **Ranking:** O(k) where k = number of results
- **Total:** O(log m + k)

**Typical performance:**
- **Any repo size:** <200ms for search
- **Memory usage:** ~1-2GB for large repos
- **Disk usage:** ~10-50MB per 1000 files

---

## Accuracy and Relevance

### Relevance Scoring

**Score interpretation:**
- **0.9-1.0:** Extremely relevant (exact semantic match)
- **0.8-0.9:** Highly relevant (very similar concept)
- **0.7-0.8:** Relevant (related concept)
- **0.6-0.7:** Moderately relevant (somewhat related)
- **<0.6:** Low relevance (may be tangentially related)

### Factors Affecting Accuracy

**Query quality:**
- Specific queries work better than vague ones
- "error handling" > "error"
- "user authentication" > "auth"

**Code quality:**
- Well-named functions score higher
- Comments improve semantic understanding
- Consistent naming patterns help

**Model limitations:**
- Very new programming concepts may not be well-represented
- Domain-specific terminology may need keyword hints
- Very short code snippets may lack context

---

## Local Processing Architecture

### Why Local Processing?

**Privacy:**
- Your code never leaves your machine
- No data sent to cloud services
- Complete control over your intellectual property

**Performance:**
- No network latency
- Consistent performance regardless of internet
- Can work offline

**Reliability:**
- No dependency on external services
- No API rate limits
- No service outages

### Processing Pipeline

```
Source Code
    ↓
Tree-sitter Parser
    ↓
AST Generation
    ↓
Chunk Extraction
    ↓
Local Embedding Model
    ↓
Vector Generation
    ↓
Local Index Storage
    ↓
Similarity Search
    ↓
Ranked Results
```

---

## Comparison with Other Approaches

### vs Traditional Search

| Aspect | Traditional (grep) | Semantic (ck) |
|--------|-------------------|---------------|
| **Speed** | Instant | ~200ms |
| **Accuracy** | Exact match only | Conceptual match |
| **Discovery** | Limited | High |
| **Cross-language** | No | Yes |
| **Context** | Line-based | Chunk-based |

### vs Cloud-based Semantic Search

| Aspect | Cloud-based | ck (Local) |
|--------|-------------|------------|
| **Privacy** | Code sent to cloud | Code stays local |
| **Speed** | Network dependent | Consistent |
| **Cost** | Per-query fees | One-time setup |
| **Reliability** | Service dependent | Always available |
| **Customization** | Limited | Full control |

---

## Advanced Topics

### Embedding Model Selection

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

### Custom Model Training

**Future feature:** Train models on your specific codebase
- Better understanding of domain-specific terms
- Improved accuracy for your code patterns
- Requires significant computational resources

### Multi-language Codebases

**Cross-language search:**
- Find similar patterns across different languages
- Understand equivalent implementations
- Discover architectural patterns

**Example:**
```
Query: "error handling"
Finds: try/catch (JavaScript), Result<T,E> (Rust), 
       error interface (Go), try/except (Python)
```

---

## Limitations and Considerations

### Current Limitations

**Model limitations:**
- May not understand very new programming concepts
- Domain-specific terminology may need keyword hints
- Very short code snippets lack context

**Performance considerations:**
- First search requires indexing (1-60 seconds)
- Large codebases require significant disk space
- Memory usage scales with codebase size

### Best Practices

**For better results:**
- Use specific, descriptive queries
- Combine with keyword hints when needed
- Adjust threshold based on use case
- Use hybrid search for precision

**For performance:**
- Exclude generated files with .ckignore
- Search specific directories when possible
- Use regex for exact pattern matching
- Consider model size vs accuracy trade-offs

---

## Future Developments

### Planned Improvements

**Model enhancements:**
- Better understanding of domain-specific code
- Improved cross-language semantic matching
- Support for more programming languages

**Performance optimizations:**
- Faster indexing algorithms
- Better memory efficiency
- Incremental model updates

**Feature additions:**
- Custom model training
- Multi-modal search (code + comments + docs)
- Real-time collaborative indexing

---

## Summary

Semantic search represents a fundamental shift in how we find and understand code. By leveraging local embedding models and tree-sitter parsing, ck provides:

- **Conceptual search** that understands what code does
- **Privacy-first** architecture with local processing
- **Cross-language** understanding of similar patterns
- **Fast, accurate** results on codebases of any size

The technology combines the best of modern AI with practical software engineering needs, making code discovery more intuitive and powerful than ever before.

---

## Next Steps

**→** [Search Modes Compared](search-modes.html) - Compare semantic, regex, and hybrid search

**→** [Architecture Deep Dive](architecture.html) - System design and components

**→** [Performance Characteristics](performance.html) - Speed, memory, and scaling
