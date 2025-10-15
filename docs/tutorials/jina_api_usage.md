# Using Jina AI Code Embeddings with cs

This guide demonstrates how to use Jina AI's code embedding models with the cs semantic code search tool.

## Prerequisites

1. **Get your Jina AI API key** (free): <https://jina.ai/?sui=apikey>
2. **Set the environment variable**:

   ```shell
   export JINA_API_KEY="your_api_key_here"
   ```

## Available Jina API Models

### Code Embedding Models

#### **jina-code-embeddings-0.5b** (`jina-code-0.5b` alias)

- 494M parameters
- 512 dimensions
- Optimized for: NL2Code, code similarity, cross-language retrieval
- Best for: Fast code search with good quality

#### **jina-code-embeddings-1.5b** (`jina-code-1.5b` alias)

- 1.54B parameters
- 768 dimensions
- Enhanced retrieval capabilities
- Best for: High-quality code search, technical Q&A

### General Embedding Models

#### **jina-embeddings-v3** (`jina-v3` alias)

- 570M parameters
- 1024 dimensions
- Multilingual text embeddings

#### **jina-embeddings-v4** (`jina-v4` alias)

- 3.8B parameters
- 2048 dimensions
- Multimodal (text + images)

### Reranker Models

#### **jina-reranker-v2-base-multilingual** (`jina-reranker-v2` alias)

- 2.5B parameters
- 1024 dimensions
- Multilingual reranker

#### **jina-reranker-v3** (`jina-reranker-v3` alias)

- 3.8B parameters
- 1024 dimensions
- Multilingual reranker

## Usage Examples

### 1. Index Your Codebase with Jina Code Embeddings

```shell
# Using the smaller, faster model (0.5b)
export JINA_API_KEY="jina_your_key_here"
cs --index --model jina-code-0.5b /path/to/your/project

# Using the larger, more accurate model (1.5b)
cs --index --model jina-code-1.5b /path/to/your/project
```

### 2. Semantic Search with Jina Models

```shell
# Search for code related to "database connection"
cs --sem "database connection" /path/to/project

# Search for error handling patterns
cs --sem "error handling with retry logic" src/

# Find authentication code
cs --sem "user authentication and authorization" .
```

### 3. Natural Language to Code Search

The Jina code embeddings excel at finding code from natural language queries:

```shell
# Find code that calculates square of numbers
cs --sem "function that calculates the square of a number" .

# Find async/await patterns
cs --sem "asynchronous function that fetches data from API" .

# Find specific algorithms
cs --sem "binary search implementation" .
```

### 4. Cross-Language Code Search

Jina code embeddings work across programming languages:

```shell
# Find similar functionality across languages
cs --sem "HTTP client request handler" .

# This will find:
# - Python: requests.get() implementations
# - Rust: reqwest usage
# - JavaScript: fetch() calls
# - Go: http.Client usage
```

### 5. Technical Q&A Style Queries

```shell
# Ask questions about your codebase
cs --sem "how to handle database transactions" .
cs --sem "where is error logging configured" .
cc --sem "what handles user session management" .
```

### 6. Advanced Usage with Filtering

```shell
# High precision search (threshold ≥ 0.8)
cs --sem "authentication middleware" --threshold 0.8 .

# Limit to top 5 results
cs --sem "database models" --limit 5 src/

# Show similarity scores
cs --sem "error handler" --scores .
```

### 7. Switch Models

```shell
# Clean and rebuild index with a different model
cs --switch-model jina-code-1.5b .

# Force rebuild even if same model
cs --switch-model jina-code-1.5b --force .
```

## Task-Specific Usage

Jina code embeddings support task-specific prefixes for optimal results:

| Task | Description | Automatic Prefix |
|------|-------------|------------------|
| `nl2code` | Natural language to code | `.query` for searches, `.passage` for code chunks |
| `code2code` | Code to code search | `.query` for search patterns |
| `code2nl` | Code to natural language | Used for documentation search |
| `qa` | Technical Q&A | `.query` for questions |

The cc tool automatically applies the appropriate task prefix based on your search mode.

## Performance Considerations

### API Rate Limits

- **With API key**: 500 requests/minute, 1M tokens/minute
- **With premium key**: 2,000 requests/minute, 5M tokens/minute

### Batch Processing

The cc tool automatically batches embedding requests for efficiency. For large codebases:

```shell
# Index will batch process files
cs --index --model jina-code-1.5b large_project/
```

### Cost Optimization

- Use `jina-code-0.5b` for development and testing (faster, cheaper)
- Use `jina-code-1.5b` for production search (better quality)
- Enable caching to avoid re-indexing unchanged files

## Comparison with Local Models

| Feature | FastEmbed (Local) | Jina API |
|---------|------------------|----------|
| **Setup** | Download models (~200MB-1GB) | API key only |
| **Speed** | Fast (local inference) | Network dependent |
| **Cost** | Free | Pay-per-use (generous free tier) |
| **Quality** | Good | Excellent |
| **Code-specific** | jina-embeddings-v2-base-code | jina-code-embeddings-1.5b |
| **Offline** | ✅ Yes | ❌ No |

## Troubleshooting

### Missing API Key Error

```shell
Error: JINA_API_KEY environment variable not set
```

**Solution**: Export your API key before running cs:

```shell
export JINA_API_KEY="your_key_here"
```

### API Rate Limit

```shell
Error: Jina API returned error status: 429 Too Many Requests
```

**Solution**: Wait a moment or upgrade to a premium API key for higher limits.

### Network Timeout

```shell
Error: Failed to send request to Jina API
```

**Solution**: Check your internet connection or increase timeout in code.

## Best Practices

1. **Cache Your Index**: cs automatically caches indexed embeddings, so you only pay once per file
2. **Incremental Updates**: Use `cc --add file.rs` to add single files without full reindex
3. **Model Selection**:
   - Use `jina-code-0.5b` for quick searches and development
   - Use `jina-code-1.5b` for production and high-quality results
4. **Batch Operations**: Let cs handle batching automatically for efficiency

## Example Workflow

```shell
# 1. Set up API key
export JINA_API_KEY=""

# 2. Index your project with Jina code embeddings
cd ~/my-project
cs --index --model jina-code-1.5b .

# 3. Perform semantic searches
cs --sem "error handling" src/
cs --sem "database connection pool" .
cs --sem "authentication middleware" --threshold 0.8

# 4. Check index status
cs --status-verbose .

# 5. Add new files incrementally
cs --add src/new_feature.rs

# 6. Switch to a different model if needed
cs --switch-model jina-code-0.5b .
```

## Integration with AI Agents

Use Jina embeddings with MCP server for AI agent integration:

```shell
# Start MCP server with Jina embeddings
export JINA_API_KEY="your_key_here"
cs --serve

# Now Claude Desktop or Cursor can semantically search your code
```

## Learn More

- **Jina AI Documentation**: <https://jina.ai/>
- **API Reference**: <https://api.jina.ai/>
- **Get API Key**: <https://jina.ai/?sui=apikey>
- **cs Documentation**: <https://github.com/lwyBZss8924d/semcs>
