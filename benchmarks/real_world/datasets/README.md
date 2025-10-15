# Real-World Test Repositories (Future)

External codebases for real-world benchmark evaluation.

## Purpose

Test cs --hybrid performance on diverse, large-scale production codebases:
- Different programming languages
- Various architectural patterns
- Realistic code complexity
- Industry-standard projects

## Planned Repositories

### Rust Codebases

1. **rust-analyzer** (~300k LoC)
   - Large Rust IDE project
   - Complex type system usage
   - Good for architecture understanding tasks

2. **tokio** (~100k LoC)
   - Async runtime
   - Macro-heavy code
   - Good for cross-file relationship tasks

3. **serde** (~50k LoC)
   - Serialization framework
   - Good for API integration tasks

### TypeScript Codebases

4. **typescript** (~500k LoC)
   - TypeScript compiler itself
   - Good for multilingual tasks (TS + Rust comparison)

5. **vscode** (~400k LoC)
   - Visual Studio Code
   - Good for large-scale navigation tasks

### Python Codebases

6. **django** (~300k LoC)
   - Web framework
   - Good for refactoring tasks

## Current Status

**Status**: 📅 Not yet implemented
**Priority**: Low (Phase 3 expansion)
**Estimated effort**: 1 day setup + indexing time

## Implementation Plan

1. Clone repositories (or use shallow clones)
2. Index with cs (jina-v4 model)
3. Create task definitions specific to each repo
4. Run automated benchmarks
5. Generate per-repo performance reports

## Directory Structure (Future)

```
real_world/datasets/
├── rust-analyzer/
│   ├── .git/          # Shallow clone
│   ├── .cs/           # Index
│   └── tasks.yaml     # Repo-specific tasks
├── tokio/
├── serde/
├── typescript/
├── vscode/
└── django/
```

## Usage (Future)

```python
from real_world.datasets import clone_and_index

# Setup test repository
repo = clone_and_index("rust-analyzer", depth=1)

# Run benchmark on this repo
runner = TestRunner(repo.path)
results = runner.run_all_tasks()
```

## Indexing Requirements

Estimated space requirements:
- Source code: ~2 GB (all repos)
- Indexes (.cs/): ~500 MB per repo = ~3 GB total
- **Total**: ~5 GB

Estimated indexing time (with jina-v4):
- ~10-30 minutes per large repo
- Can run overnight

## Benefits

- Validate on production-quality code
- Discover edge cases
- Benchmark scalability
- Generate realistic performance data

## References

- rust-analyzer: https://github.com/rust-lang/rust-analyzer
- tokio: https://github.com/tokio-rs/tokio
- serde: https://github.com/serde-rs/serde
