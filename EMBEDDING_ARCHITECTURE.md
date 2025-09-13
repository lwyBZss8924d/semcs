# Enhanced Embedding Architecture for ck

## Overview
This document outlines the enhanced embedding strategy to handle large code chunks that exceed model context windows.

## Current Issues
- BGE-Small-EN-V1.5: 512 token limit, 384 dimensions
- Large functions (1363+ tokens) exceed context window
- Information loss from truncation

## Proposed Solution

### 1. Model Upgrade
**Primary Model: `NomicEmbedTextV15`**
- Context Window: 8192 tokens (16x increase)
- Dimensions: 768 (2x increase) 
- Best for code + text hybrid content

### 2. Striding Strategy for Ultra-Large Chunks

Even with 8192 tokens, some files might exceed limits. Implement striding:

```
Original Large Chunk (12000 tokens)
├── Stride 1: tokens 0-7168     (with 1024 overlap)
├── Stride 2: tokens 6144-13312 (with 1024 overlap) 
└── Stride 3: tokens 12288-12000
```

**Parameters:**
- Window Size: 7168 tokens (leaving buffer for special tokens)
- Overlap: 1024 tokens (12.5% overlap for context continuity)
- Stride: 6144 tokens (window - overlap)

### 3. Hierarchical Retrieval

For strided chunks, implement multi-level retrieval:

```
Query → Semantic Search
├── Regular Chunks (≤8192 tokens) → Direct Results
└── Strided Chunks → Aggregate & Rerank
    ├── All strides scored independently
    ├── Scores aggregated per original chunk
    └── Best stride + context returned
```

### 4. Reranking Pipeline

Implement cross-encoder reranking for improved relevance:

**Models Available:**
- `BGE Reranker Base` (EN/CN)
- `JINA Reranker V1 Turbo EN` (fastest)
- `JINA Reranker V2 Base Multilingual`

**Reranking Flow:**
1. Semantic search returns top-50 candidates
2. Reranker scores query-candidate pairs
3. Return top-10 reranked results

## Implementation Phases

### Phase 1: Model Upgrade
- [x] Research available models
- [x] Identify Nomic V1.5 as best option
- [ ] Update embedding configuration
- [ ] Add model selection CLI option

### Phase 2: Enhanced Chunking
- [ ] Add token counting utilities
- [ ] Implement striding logic for large chunks
- [ ] Update chunk metadata with stride info

### Phase 3: Hierarchical Retrieval
- [ ] Detect strided chunks during search
- [ ] Aggregate stride scores
- [ ] Return best stride with context

### Phase 4: Reranking
- [ ] Add reranking model support
- [ ] Implement cross-encoder pipeline
- [ ] Add reranking CLI flags

## Data Structures

### Enhanced Chunk
```rust
pub struct Chunk {
    pub span: Span,
    pub text: String,
    pub chunk_type: ChunkType,
    pub stride_info: Option<StrideInfo>, // NEW
}

pub struct StrideInfo {
    pub original_chunk_id: String,
    pub stride_index: usize,
    pub total_strides: usize,
    pub overlap_start: usize,
    pub overlap_end: usize,
}
```

### Search Results
```rust
pub struct SearchResult {
    pub chunk: Chunk,
    pub score: f32,
    pub rerank_score: Option<f32>, // NEW
    pub is_stride: bool,           // NEW
}
```

## Configuration

### CLI Options
```bash
# Model selection
ck --model nomic-v1.5 --sem "query" src/
ck --model jina-code --sem "function" src/

# Striding options
ck --stride-window 7168 --stride-overlap 1024 --sem "query" src/

# Reranking
ck --rerank --rerank-model jina-turbo --sem "query" src/
ck --rerank-topk 50 --topk 10 --sem "query" src/
```

### Config File
```toml
[embedding]
model = "nomic-v1.5"
context_window = 8192
stride_window = 7168
stride_overlap = 1024

[reranking]
enabled = true
model = "jina-reranker-v1-turbo-en"
candidate_count = 50
final_count = 10
```

## Benefits

1. **Larger Context**: 8192 vs 512 tokens (16x improvement)
2. **Better Quality**: 768 vs 384 dimensions (2x improvement)
3. **No Information Loss**: Striding preserves all content
4. **Improved Relevance**: Reranking boosts precision
5. **Hierarchical Understanding**: Function/class level + detailed content

## Migration Strategy

1. **Backward Compatible**: Keep BGE-Small as fallback
2. **Progressive Rollout**: Add flags for new features
3. **Index Migration**: Auto-detect and rebuild with new model
4. **Performance Testing**: Benchmark against current implementation