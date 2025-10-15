# Quantitative Tasks (Future Implementation)

Task definitions for quantitative code search evaluation.

## Planned Task Types

### 1. Natural Language → Code (nl2code)
Query: Natural language description
Expected: Relevant code snippets

Example:
```yaml
- query: "function to parse JSON from file"
  language: python
  expected_files:
    - "utils/parser.py:15-30"
  relevance: 1.0
```

### 2. Code → Code (code2code)
Query: Code snippet
Expected: Similar or related code

Example:
```yaml
- query: |
    fn read_config(path: &Path) -> Result<Config> {
        // implementation
    }
  language: rust
  expected_similar:
    - "load_settings"
    - "parse_config"
```

### 3. Code QA (qa)
Query: Question about codebase
Expected: Relevant code locations

Example:
```yaml
- query: "Where is error handling implemented?"
  expected_locations:
    - "src/errors.rs"
    - "src/lib.rs:100-150"
```

## Current Status

**Status**: 📅 Not yet implemented
**Priority**: Medium (Phase 2 expansion)
**Estimated effort**: 2-3 days

## Implementation Plan

1. Define task schema (YAML format)
2. Create 50-100 task definitions per type
3. Generate ground truth annotations
4. Implement task loader
5. Integrate with evaluation metrics

## Format Specification

```yaml
tasks:
  - id: nl2code-001
    type: nl2code
    query: "search query"
    language: rust
    ground_truth:
      - file: "path/to/file.rs"
        lines: [10, 50]
        relevance: 1.0
      - file: "path/to/other.rs"
        lines: [100, 120]
        relevance: 0.8
    metadata:
      difficulty: medium
      domain: configuration
```

## Usage (Future)

```python
from quantitative.tasks import load_tasks
from quantitative.eval import evaluate_precision_at_k

# Load tasks
tasks = load_tasks("nl2code", language="rust")

# Evaluate
for task in tasks:
    results = cs_search(task.query)
    p_at_5 = evaluate_precision_at_k(results, task.ground_truth, k=5)
```

## References

- See: `real_world/tasks/code_comprehension_tasks.yaml` for similar format
- Jina AI methodology: 25 benchmarks across multiple dimensions
