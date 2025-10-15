# Quantitative Test Agents (Future)

Agent implementations for quantitative benchmark evaluation.

## Purpose

Implement baseline agents for standard code search benchmarks:
- Compare cs --hybrid against existing tools
- Standardized evaluation on CodeSearchNet, etc.
- Academic-quality benchmarking

## Planned Agents

### 1. CS Hybrid Agent
Implementation: Uses `cs --hybrid` for all queries

```python
class CSHybridAgent:
    def search(self, query: str, language: str) -> List[Result]:
        # Run cs --hybrid with optimized parameters
        return cs_hybrid_search(query, language)
```

### 2. BM25 Baseline Agent
Implementation: Pure lexical search (ripgrep)

```python
class BM25Agent:
    def search(self, query: str, language: str) -> List[Result]:
        # Use ripgrep with BM25 scoring
        return bm25_search(query, language)
```

### 3. AST-grep Agent
Implementation: Structural pattern matching

```python
class ASTGrepAgent:
    def search(self, query: str, language: str) -> List[Result]:
        # Convert query to AST pattern
        pattern = query_to_ast_pattern(query)
        return ast_grep_search(pattern, language)
```

### 4. Semantic-only Agent
Implementation: Pure vector search (no BM25, no AST)

```python
class SemanticOnlyAgent:
    def search(self, query: str, language: str) -> List[Result]:
        # Just semantic embeddings
        return semantic_search(query, language)
```

### 5. Jina AI Agent (Reference)
Implementation: Call Jina AI API for comparison

```python
class JinaReferenceAgent:
    def search(self, query: str, language: str) -> List[Result]:
        # Use Jina embeddings API
        return jina_search(query, language)
```

## Current Status

**Status**: 📅 Not yet implemented
**Priority**: Medium (Phase 2 expansion)
**Estimated effort**: 2-3 days

## Implementation Plan

1. Define agent interface (abstract base class)
2. Implement each agent
3. Create standardized evaluation harness
4. Run comparative benchmarks
5. Generate comparison reports

## Agent Interface

```python
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List

@dataclass
class SearchResult:
    file: str
    lines: tuple[int, int]
    score: float
    content: str

class CodeSearchAgent(ABC):
    """Abstract base class for code search agents"""

    @abstractmethod
    def search(self, query: str, language: str, topk: int = 10) -> List[SearchResult]:
        """
        Search for code matching the query.

        Args:
            query: Natural language or code query
            language: Programming language (rust, python, etc.)
            topk: Number of results to return

        Returns:
            List of search results ranked by relevance
        """
        pass

    @abstractmethod
    def name(self) -> str:
        """Agent name for reporting"""
        pass
```

## Usage (Future)

```python
from quantitative.agents import CSHybridAgent, BM25Agent, ASTGrepAgent
from quantitative.eval import evaluate_agents
from quantitative.tasks import load_tasks

# Load agents
agents = [
    CSHybridAgent(),
    BM25Agent(),
    ASTGrepAgent(),
]

# Load tasks
tasks = load_tasks("nl2code", language="rust")

# Evaluate
results = evaluate_agents(agents, tasks)

# Generate report
print(f"CS Hybrid P@5: {results['CSHybrid']['p_at_5']:.3f}")
print(f"BM25 P@5: {results['BM25']['p_at_5']:.3f}")
print(f"Improvement: {(results['CSHybrid']['p_at_5'] - results['BM25']['p_at_5']) * 100:.1f}%")
```

## Expected Results

Based on Jina AI benchmarks and our design:

| Agent | P@5 | R@10 | MRR | nDCG@10 |
|-------|-----|------|-----|---------|
| **CS Hybrid** | **0.75** | **0.85** | **0.80** | **0.82** |
| BM25 | 0.45 | 0.60 | 0.50 | 0.55 |
| AST-grep | 0.60 | 0.70 | 0.65 | 0.68 |
| Semantic-only | 0.70 | 0.80 | 0.75 | 0.78 |
| Jina Reference | 0.72 | 0.82 | 0.78 | 0.80 |

**Key insight**: CS Hybrid should match or exceed Jina Reference by combining:
- Semantic understanding (Jina embeddings)
- Lexical matching (BM25)
- Structural awareness (AST)
- Cross-encoder reranking

## Benefits

- Academic-quality comparison
- Publishable benchmark results
- Validate design decisions
- Identify improvement opportunities

## References

- Related: `real_world/agents/` for real-world task agents
- Jina AI benchmark: https://jina.ai/news/jina-code-embeddings-sota-code-retrieval-at-0-5b-and-1-5b/
