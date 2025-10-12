# MCP Pagination Design

This document outlines the design for adding first-class pagination and size controls to ck's MCP search tools to avoid large token responses.

## Phase 0: Design & Request/Response Shapes

### Enhanced Request Parameters

All search tools (`regex_search`, `semantic_search`, `hybrid_search`) will support these additional optional parameters:

```json
{
  // Existing parameters...
  "cursor": "optional_opaque_cursor_string",
  "page_size": 50,                    // Default: 50, Max: 200
  "include_snippet": true,            // Default: true
  "snippet_length": 500,              // Default: 500 chars
  "context_lines": 3                  // Default: 0, Max: 10
}
```

### Enhanced Response Format

All search responses will follow this structure:

```json
{
  "search": {
    "query": "original query/pattern",
    "mode": "semantic|regex|hybrid",
    "parameters": { /* original parameters */ }
  },
  "results": {
    "matches": [
      {
        "file": {
          "path": "path/to/file.rs",
          "language": "rust"
        },
        "match": {
          "span": {
            "byte_start": 1234,
            "byte_end": 1290,
            "line_start": 45,
            "line_end": 47
          },
          "content": "code snippet...",
          "score": 0.89,           // semantic/hybrid only
          "line_number": 45        // regex only
        },
        "type": "semantic_match|regex_match|hybrid_match"
      }
    ],
    "count": 25,                      // matches in this page
    "total_count": 127,               // total matches (if known)
    "has_more": true,                 // boolean
    "truncated": false                // true if results were truncated
  },
  "pagination": {
    "next_cursor": "opaque_cursor_string_or_null",
    "page_size": 50,
    "current_page": 1
  },
  "metadata": {
    "search_time_ms": 234,
    "index_stats": {
      "total_files": 1234,
      "total_chunks": 5678
    }
  }
}
```

### Cursor Format

Cursors are base64-encoded JSON containing:

```json
{
  "session_id": "uuid",
  "offset": 50,
  "search_params_hash": "sha256_of_original_params",
  "timestamp": 1703123456789,
  "version": 1
}
```

## Phase 1: SearchSession Infrastructure

### Session Management

- **Location**: `ck-cli/src/mcp/session.rs`
- **Session Storage**: In-memory HashMap with UUID keys
- **TTL**: 5 minutes from last access
- **Max Sessions**: 100 per server instance
- **Cleanup**: Background task every 60 seconds

### Session Data Structure

```rust
pub struct SearchSession {
    pub id: Uuid,
    pub search_options: SearchOptions,
    pub results: Vec<SearchResult>,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub total_count: Option<usize>,
    pub search_completed: bool,
}
```

### Memory Management

- **Page Size Limits**: Default 50, max 200 results per page
- **Content Limits**: Snippet truncation to prevent massive responses
- **Hard Ceiling**: If single page > 1MB, convert to error with guidance
- **Session Eviction**: LRU eviction when approaching memory limits

## Phase 2: Handler Updates

### Search Flow Changes

1. **First Request** (no cursor):
   - Execute full search via `ck_engine`
   - Create session and cache results
   - Return first page with `next_cursor`

2. **Subsequent Requests** (with cursor):
   - Validate cursor and retrieve session
   - Return next page from cached results
   - Update `last_accessed` timestamp

### Error Handling

- **Invalid Cursor**: MCP error -32602 (Invalid params)
- **Expired Session**: MCP error -32602 with retry guidance
- **Too Many Results**: Truncate with `truncated: true` flag

## Phase 3: CLI Separation

### Compatibility Strategy

- **CLI Path**: Unchanged, uses existing `ck_engine::search_enhanced_with_indexing_progress`
- **MCP Path**: Uses new session-based pagination layer
- **Shared Code**: Core search functionality remains in `ck_engine`
- **Feature Gates**: Session code behind `#[cfg(feature = "mcp")]`

### Default Differences

| Setting | CLI Default | MCP Default |
|---------|-------------|-------------|
| Output Format | Text/JSONL | Structured JSON |
| Result Limit | Unlimited | 50 per page |
| Snippet Length | Full | 500 chars |
| Progress Callbacks | Enabled | Limited |

## Phase 4: Documentation Updates

### README Additions

```markdown
## MCP Pagination

The MCP server supports pagination for large result sets:

### Client Usage

```python
# First page
response = await client.call_tool("semantic_search", {
    "query": "authentication",
    "path": "/path/to/code",
    "page_size": 25
})

# Next page
if response["pagination"]["next_cursor"]:
    next_response = await client.call_tool("semantic_search", {
        "cursor": response["pagination"]["next_cursor"]
    })
```

### Agent Guidance

For Claude Code and other agents:
- Use `page_size: 25-50` for initial exploration
- Set `include_snippet: false` for high-level overviews
- Use cursors to paginate through large result sets
- Consider `snippet_length: 200` for condensed results
```

## Phase 5: Testing Strategy

### Unit Tests

- Cursor encoding/decoding
- Session creation and retrieval
- TTL expiration
- Memory limits and eviction

### Integration Tests

- Multi-page search scenarios
- Expired cursor handling
- Large result set truncation
- CLI compatibility verification

### Manual Testing

- Claude Code integration
- Cursor MCP client testing
- Performance with large codebases
- Memory usage monitoring

## Implementation Notes

### Performance Considerations

- **Incremental Results**: Consider streaming results as they're found
- **Index Caching**: Reuse embedders and indices across sessions
- **Memory Monitoring**: Track session memory usage
- **Background Cleanup**: Efficient session eviction

### Security Considerations

- **Cursor Validation**: Prevent cursor tampering
- **Resource Limits**: Prevent DoS via excessive sessions
- **Path Validation**: Ensure cursors can't access unauthorized paths

### Future Enhancements

- **Persistent Sessions**: Disk-based session storage for long-running searches
- **Result Streaming**: Real-time result delivery for large searches
- **Cross-Session Caching**: Share results between similar searches
- **Advanced Filtering**: Post-search filtering without re-execution