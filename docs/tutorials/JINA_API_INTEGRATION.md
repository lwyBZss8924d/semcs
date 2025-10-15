# Jina AI Code Embeddings Integration

## Overview

The cc semantic code search tool now supports Jina AI's code embedding models via their API. This enables high-quality code search without downloading large model files locally.

## Features Added

### New Embedding Models

Four new Jina AI API-based models have been added to the model registry:

1. **jina-code-embeddings-0.5b** (alias: `jina-code-0.5b`)
   - 494M parameters, 512 dimensions
   - Optimized for NL2Code, code similarity, cross-language retrieval
   - Best for: Fast code search with good quality

2. **jina-code-embeddings-1.5b** (alias: `jina-code-1.5b`)
   - 1.54B parameters, 768 dimensions
   - Enhanced retrieval capabilities
   - Best for: High-quality code search, technical Q&A

3. **jina-embeddings-v3** (alias: `jina-v3`)
   - 570M parameters, 1024 dimensions
   - Multilingual text embeddings

4. **jina-embeddings-v4** (alias: `jina-v4`)
   - 3.8B parameters, 2048 dimensions
   - Multimodal (text + images)

### Code Changes

#### 1. New Module: `cc-embed/src/jina_api.rs`

Implements the `JinaApiEmbedder` struct that:
- Communicates with Jina AI's embedding API
- Supports task-specific prefixes (nl2code, code2code, qa, etc.)
- Handles authentication via `JINA_API_KEY` environment variable
- Auto-truncates inputs to avoid API errors
- Validates embedding dimensions

```rust
pub struct JinaApiEmbedder {
    client: reqwest::Client,
    api_key: String,
    model_name: String,
    dimensions: usize,
    task: Option<String>,
    api_url: String,
}
```

#### 2. Enhanced Model Registry: `cc-models/src/lib.rs`

Added four new model configurations to `ModelRegistry::default()`:
- jina-code-0.5b
- jina-code-1.5b
- jina-v3
- jina-v4

Each includes provider, dimensions, max_tokens, and description.

#### 3. Updated Embedder Creation: `cc-embed/src/lib.rs`

Modified `create_embedder_with_progress()` to:
- Detect Jina API models by prefix (`jina-code-embeddings-`, `jina-embeddings-`)
- Auto-configure dimensions and task types based on model
- Route to appropriate embedder implementation

#### 4. CLI Updates: `cc-cli/src/main.rs`

Updated help text and documentation:
- Added examples using Jina API models
- Updated `--model` parameter help text
- Documented `JINA_API_KEY` requirement

#### 5. Dependency Updates

**cc-embed/Cargo.toml:**
- Added `reqwest` with `json` and `rustls-tls` features
- Added `serde_json` for API request/response handling
- Added new `jina-api` feature flag
- Added `serial_test` dev dependency for thread-safe tests

**cc-cli/Cargo.toml:**
- Enabled `jina-api` feature for cc-embed

## Usage

### 1. Set API Key

```bash
export JINA_API_KEY="your_api_key_here"
```

Get your free API key at: https://jina.ai/?sui=apikey

### 2. Index with Jina Models

```bash
# Using smaller, faster model
cc --index --model jina-code-0.5b /path/to/project

# Using larger, more accurate model
cc --index --model jina-code-1.5b /path/to/project
```

### 3. Search Your Code

```bash
# Natural language to code search
cc --sem "error handling with retry logic" src/

# Find authentication code
cc --sem "user authentication and authorization" .

# Cross-language code search
cc --sem "HTTP client request handler" .
```

## API Reference

### Task Types

Jina code embeddings support task-specific prefixes:

| Task | Usage | Description |
|------|-------|-------------|
| `nl2code.query` | Default for semantic search | Natural language to code |
| `nl2code.passage` | Auto-applied to code chunks | Code document representation |
| `code2code.query` | Code similarity search | Find similar code patterns |
| `qa.query` | Technical Q&A | Answer questions about code |

The cc tool automatically applies appropriate task prefixes.

### API Rate Limits

- **Free tier**: 500 requests/minute, 1M tokens/minute
- **Premium**: 2,000 requests/minute, 5M tokens/minute

### Error Handling

The implementation includes comprehensive error handling:
- Missing API key detection with helpful error message
- Network timeout handling (120s default)
- Dimension validation
- HTTP status code checking

## Testing

### Unit Tests

Three test functions added to `cc-embed/src/jina_api.rs`:

1. **test_jina_api_embedder_requires_api_key**: Verifies API key validation
2. **test_jina_api_embedder_creation_with_api_key**: Tests embedder instantiation
3. **test_jina_api_embedder_empty_input**: Tests empty input handling
4. **test_jina_api_real_request** (ignored): Integration test requiring real API key

All tests use `#[serial]` attribute to prevent race conditions with environment variables.

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run Jina API tests specifically
cargo test --package cc-embed jina_api

# Run integration test (requires JINA_API_KEY)
cargo test --package cc-embed test_jina_api_real_request -- --ignored
```

## Architecture

### Request Flow

1. User runs: `cc --index --model jina-code-1.5b .`
2. CLI calls `create_embedder("jina-code-embeddings-1.5b")`
3. Detects "jina-code-embeddings-" prefix
4. Creates `JinaApiEmbedder` with 768 dimensions, "nl2code.query" task
5. During indexing, chunks are embedded via API
6. Embeddings cached in `.cs/index/`

### Sync/Async Bridge

The `Embedder` trait requires a sync `embed()` method, but reqwest is async. Solution:

```rust
let runtime = tokio::runtime::Handle::try_current()
    .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
    .context("Failed to get tokio runtime")?;

let response = runtime.block_on(async {
    // async API call
})?;
```

This reuses existing tokio runtime if available, or creates a new one.

## Configuration

### Model Selection Guidelines

| Use Case | Recommended Model | Rationale |
|----------|------------------|-----------|
| Development/Testing | jina-code-0.5b | Faster, cheaper |
| Production Search | jina-code-1.5b | Better quality |
| Multilingual Code | jina-v3 | Language support |
| Multimodal (text+images) | jina-v4 | Image understanding |

### Cost Optimization

1. **Use caching**: cc automatically caches embeddings
2. **Incremental updates**: `cc --add file.rs` for single files
3. **Choose appropriate model**: Don't use jina-v4 if you only need text
4. **Batch efficiently**: cc batches requests automatically

## Comparison: Local vs API Models

| Feature | FastEmbed (Local) | Jina API |
|---------|------------------|----------|
| Setup | Download ~200MB-1GB models | Just API key |
| Speed | Fast (local inference) | Network dependent |
| Cost | Free | Pay-per-use (generous free tier) |
| Quality | Good | Excellent |
| Code-specific | jina-embeddings-v2-base-code | jina-code-embeddings-1.5b |
| Offline | ✅ Yes | ❌ No |
| Model updates | Manual | Automatic |

## Future Enhancements

Potential improvements:
1. Support for custom task types via CLI flags
2. Request retry with exponential backoff
3. Response caching to reduce API calls
4. Batch size configuration
5. Timeout configuration via CLI
6. Support for Jina's reranker API

## Documentation

- **User Guide**: `/examples/jina_api_usage.md`
- **API Docs**: https://api.jina.ai/
- **Model Info**: https://jina.ai/
- **Get API Key**: https://jina.ai/?sui=apikey

## Troubleshooting

### Common Issues

1. **Missing API Key**
   ```
   Error: JINA_API_KEY environment variable not set
   ```
   Solution: `export JINA_API_KEY="your_key"`

2. **Rate Limit**
   ```
   Error: 429 Too Many Requests
   ```
   Solution: Wait or upgrade to premium key

3. **Network Timeout**
   ```
   Error: Failed to send request
   ```
   Solution: Check internet connection

## Contributing

When adding new Jina API models:

1. Add model config to `cc-models/src/lib.rs`
2. Add dimension/task mapping in `cc-embed/src/lib.rs`
3. Update CLI help text
4. Add tests
5. Update documentation

## License

This feature maintains the project's dual license: MIT OR Apache-2.0
