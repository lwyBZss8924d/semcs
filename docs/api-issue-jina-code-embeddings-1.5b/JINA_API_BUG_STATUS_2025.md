# Jina API Bug Status Report - 2025-10-15

## Testing Context

Jina team reported that they fixed the "8192 token limit bug" for `jina-code-embeddings-1.5b`.
This report verifies whether the fix is working.

## Test Results Summary

**Result: ❌ BUG STILL EXISTS - No improvement detected**

The `jina-code-embeddings-1.5b` API still has the same ~1KB input limitation despite claims of supporting 8192 tokens.

## Detailed Test Results

| Test Size | Actual Bytes | Result | Request ID | Tokens Used |
|-----------|--------------|--------|------------|-------------|
| 827 bytes | 827 | ✅ SUCCESS | N/A | 207 |
| 1065 bytes | 1065 | ❌ FAILED | 00ee5760edec1651949f34bb8fcb5768 | N/A |
| 1911 bytes | 1911 | ❌ FAILED | 388ed9a48563084fb32f402cffdf3542 | N/A |
| 4655 bytes | 4655 | ❌ FAILED | 0a544263ddf13d7f2d8ba517db49710f | N/A |

## Test Configuration

```json
{
  "model": "jina-code-embeddings-1.5b",
  "input": ["<code content>"],
  "task": "nl2code.passage"
}
```

## Error Message

All failed tests returned the same error:

```json
{
  "detail": "[RID: <request_id>] Failed to encode text"
}
```

## Comparison with Previous Tests

The limitation remains **EXACTLY THE SAME** as before:

- **Previous limit**: ~1KB with task parameter
- **Current limit**: ~1KB with task parameter
- **Claimed limit**: 8192 tokens (~32KB)
- **Actual behavior**: No change detected

## Test File Details

**827 bytes test (SUCCESS)**:
- Source: First 30 lines of `cc-cli/src/main.rs`
- Content: Import statements and basic module declarations
- Token count: 207 tokens
- Embedding dimensions: 1536

**1065 bytes test (FAILED)**:
- Source: First 35 lines of `cc-cli/src/main.rs`
- Content: Extended import statements and module setup
- Error: "Failed to encode text"

## Comparison with jina-embeddings-v4

For reference, `jina-embeddings-v4` successfully handles **much larger files**:

- Successfully indexed 1986 chunks from 157 files
- Handles files with 8K+ tokens
- Uses object format: `{"text": "..."}`
- Different task parameter: `"code.passage"`

## Conclusion

**The reported fix does NOT appear to be deployed or working.**

The `jina-code-embeddings-1.5b` API continues to fail on inputs larger than ~1KB, which is:

- **Far below** the claimed 8192 token limit
- **Unchanged** from previous testing
- **Incompatible** with code indexing use cases

### Recommendation

Continue using the **hybrid strategy**:
1. Index with `jina-embeddings-v4` (handles large files)
2. Query with `jina-code-embeddings-1.5b` (code-optimized semantic search)

Both models output 1536 dimensions, making cross-model queries possible.

## Test Environment

- Date: 2025-10-15
- API Endpoint: `https://api.jina.ai/v1/embeddings`
- Test Location: `/Users/arthur/dev-space/cc`
- Test Files: `cc-cli/src/main.rs` (partial)

## Verified Request IDs

All Request IDs are real and traceable in Jina's backend:

1. `00ee5760edec1651949f34bb8fcb5768` - 1065 bytes failed
2. `388ed9a48563084fb32f402cffdf3542` - 1911 bytes failed
3. `0a544263ddf13d7f2d8ba517db49710f` - 4655 bytes failed

## Next Steps

1. Share this report with Jina team
2. Request clarification on:
   - Was the fix actually deployed?
   - What is the actual token/byte limit?
   - Are there additional parameters needed?
3. Continue using hybrid strategy until issue is resolved
