# Quantitative Datasets (Future Implementation)

This directory is reserved for standard code search datasets used in quantitative evaluation.

## Planned Datasets

### CodeSearchNet Extended
- Format: JSONL with (query, code, relevance) tuples
- Languages: Python, JavaScript, TypeScript, Rust, Go
- Tasks: Natural language → code search

### Custom Code Retrieval Benchmarks
- Domain-specific code search tasks
- API usage finding
- Bug pattern detection

### Ground Truth Data
- Manually annotated relevance judgments
- Expert-validated query-document pairs

## Current Status

**Status**: 📅 Not yet implemented
**Priority**: Medium (Phase 2 expansion)
**Estimated effort**: 1-2 days

## Implementation Plan

1. Download CodeSearchNet dataset
2. Convert to extended format with relevance scores
3. Create custom benchmark tasks
4. Generate ground truth annotations
5. Integrate with `quantitative/eval/metrics.py`

## Usage (Future)

```python
from quantitative.datasets import load_codesearchnet

# Load dataset
dataset = load_codesearchnet("python", split="test")

# Run evaluation
for query, ground_truth in dataset:
    results = cs_search(query)
    precision = evaluate_precision(results, ground_truth)
```

## References

- CodeSearchNet: https://github.com/github/CodeSearchNet
- Jina AI benchmark: https://jina.ai/news/jina-code-embeddings-sota-code-retrieval-at-0-5b-and-1-5b/
