#!/usr/bin/env python3
"""
Quantitative Evaluation Metrics for cs --hybrid Benchmark

Implements standard Information Retrieval metrics:
- Precision@k (P@k)
- Recall@k (R@k)
- Mean Reciprocal Rank (MRR)
- Normalized Discounted Cumulative Gain (nDCG@k)
- Mean Average Precision (MAP)

Plus cs-specific metrics:
- Semantic recall
- AST precision
- Hybrid fusion gain
"""

import numpy as np
from typing import List, Dict, Set, Tuple
from dataclasses import dataclass


@dataclass
class SearchResult:
    """Single search result"""
    doc_id: str
    score: float
    rank: int
    relevant: bool = False


@dataclass
class EvaluationResult:
    """Complete evaluation results for a query"""
    query_id: str
    precision_at_1: float
    precision_at_5: float
    precision_at_10: float
    recall_at_10: float
    mrr: float
    ndcg_at_10: float
    average_precision: float


def precision_at_k(results: List[SearchResult], k: int) -> float:
    """
    Calculate Precision@k

    P@k = (# relevant docs in top k) / k

    Args:
        results: List of search results (sorted by rank)
        k: Cutoff rank

    Returns:
        Precision@k score
    """
    if not results or k == 0:
        return 0.0

    top_k = results[:k]
    relevant_count = sum(1 for r in top_k if r.relevant)

    return relevant_count / k


def recall_at_k(
    results: List[SearchResult],
    k: int,
    total_relevant: int
) -> float:
    """
    Calculate Recall@k

    R@k = (# relevant docs in top k) / (total # relevant docs)

    Args:
        results: List of search results (sorted by rank)
        k: Cutoff rank
        total_relevant: Total number of relevant documents

    Returns:
        Recall@k score
    """
    if not results or k == 0 or total_relevant == 0:
        return 0.0

    top_k = results[:k]
    relevant_count = sum(1 for r in top_k if r.relevant)

    return relevant_count / total_relevant


def mean_reciprocal_rank(results: List[SearchResult]) -> float:
    """
    Calculate Mean Reciprocal Rank (MRR)

    MRR = 1 / rank of first relevant document

    Args:
        results: List of search results (sorted by rank)

    Returns:
        MRR score (0 if no relevant docs found)
    """
    if not results:
        return 0.0

    for result in results:
        if result.relevant:
            return 1.0 / result.rank

    return 0.0  # No relevant documents found


def discounted_cumulative_gain(results: List[SearchResult], k: int) -> float:
    """
    Calculate Discounted Cumulative Gain (DCG)

    DCG@k = sum_{i=1}^{k} (rel_i / log2(i + 1))

    Args:
        results: List of search results (sorted by rank)
        k: Cutoff rank

    Returns:
        DCG@k score
    """
    if not results or k == 0:
        return 0.0

    dcg = 0.0
    for i, result in enumerate(results[:k], start=1):
        relevance = 1.0 if result.relevant else 0.0
        dcg += relevance / np.log2(i + 1)

    return dcg


def normalized_dcg(
    results: List[SearchResult],
    k: int,
    total_relevant: int
) -> float:
    """
    Calculate Normalized Discounted Cumulative Gain (nDCG@k)

    nDCG@k = DCG@k / IDCG@k

    where IDCG@k is the ideal DCG (all relevant docs at top)

    Args:
        results: List of search results (sorted by rank)
        k: Cutoff rank
        total_relevant: Total number of relevant documents

    Returns:
        nDCG@k score (0 if no relevant docs)
    """
    if not results or k == 0 or total_relevant == 0:
        return 0.0

    # Calculate actual DCG
    actual_dcg = discounted_cumulative_gain(results, k)

    # Calculate ideal DCG (all relevant docs at top positions)
    ideal_results = [
        SearchResult(doc_id=f"ideal_{i}", score=1.0, rank=i, relevant=True)
        for i in range(1, min(k, total_relevant) + 1)
    ]
    ideal_dcg = discounted_cumulative_gain(ideal_results, k)

    if ideal_dcg == 0:
        return 0.0

    return actual_dcg / ideal_dcg


def average_precision(results: List[SearchResult], total_relevant: int) -> float:
    """
    Calculate Average Precision (AP)

    AP = (sum of P@k for each relevant doc) / total_relevant

    Args:
        results: List of search results (sorted by rank)
        total_relevant: Total number of relevant documents

    Returns:
        AP score
    """
    if not results or total_relevant == 0:
        return 0.0

    sum_precisions = 0.0
    relevant_count = 0

    for i, result in enumerate(results, start=1):
        if result.relevant:
            relevant_count += 1
            # Precision at this position
            precision_at_i = relevant_count / i
            sum_precisions += precision_at_i

    return sum_precisions / total_relevant


def mean_average_precision(
    all_results: Dict[str, List[SearchResult]],
    relevance_judgments: Dict[str, Set[str]]
) -> float:
    """
    Calculate Mean Average Precision (MAP) across all queries

    MAP = mean(AP for all queries)

    Args:
        all_results: Dict mapping query_id to list of results
        relevance_judgments: Dict mapping query_id to set of relevant doc_ids

    Returns:
        MAP score
    """
    if not all_results:
        return 0.0

    ap_scores = []

    for query_id, results in all_results.items():
        relevant_docs = relevance_judgments.get(query_id, set())
        if not relevant_docs:
            continue

        # Mark relevant results
        marked_results = [
            SearchResult(
                doc_id=r.doc_id,
                score=r.score,
                rank=r.rank,
                relevant=(r.doc_id in relevant_docs)
            )
            for r in results
        ]

        ap = average_precision(marked_results, len(relevant_docs))
        ap_scores.append(ap)

    if not ap_scores:
        return 0.0

    return np.mean(ap_scores)


def evaluate_query(
    results: List[SearchResult],
    relevant_docs: Set[str]
) -> EvaluationResult:
    """
    Evaluate a single query with all metrics

    Args:
        results: List of search results
        relevant_docs: Set of relevant document IDs

    Returns:
        EvaluationResult with all metrics
    """
    # Mark relevant results
    marked_results = [
        SearchResult(
            doc_id=r.doc_id,
            score=r.score,
            rank=r.rank,
            relevant=(r.doc_id in relevant_docs)
        )
        for r in results
    ]

    total_relevant = len(relevant_docs)

    return EvaluationResult(
        query_id=results[0].doc_id if results else "unknown",
        precision_at_1=precision_at_k(marked_results, 1),
        precision_at_5=precision_at_k(marked_results, 5),
        precision_at_10=precision_at_k(marked_results, 10),
        recall_at_10=recall_at_k(marked_results, 10, total_relevant),
        mrr=mean_reciprocal_rank(marked_results),
        ndcg_at_10=normalized_dcg(marked_results, 10, total_relevant),
        average_precision=average_precision(marked_results, total_relevant)
    )


# ============================================================================
# cs-specific metrics
# ============================================================================

def semantic_recall(
    semantic_matches: Set[str],
    ground_truth: Set[str]
) -> float:
    """
    Measure how many relevant items were found through semantic search

    Args:
        semantic_matches: Documents found through semantic search
        ground_truth: All relevant documents

    Returns:
        Semantic recall score
    """
    if not ground_truth:
        return 0.0

    semantic_relevant = semantic_matches & ground_truth
    return len(semantic_relevant) / len(ground_truth)


def ast_precision(
    ast_matches: Set[str],
    ground_truth: Set[str]
) -> float:
    """
    Measure precision of AST pattern matching

    Args:
        ast_matches: Documents found through AST search
        ground_truth: All relevant documents

    Returns:
        AST precision score
    """
    if not ast_matches:
        return 0.0

    ast_relevant = ast_matches & ground_truth
    return len(ast_relevant) / len(ast_matches)


def hybrid_fusion_gain(
    hybrid_score: float,
    semantic_score: float,
    lexical_score: float,
    ast_score: float = 0.0
) -> float:
    """
    Measure the gain from hybrid fusion over best single method

    Args:
        hybrid_score: Score from hybrid search
        semantic_score: Score from semantic-only search
        lexical_score: Score from lexical-only search
        ast_score: Score from AST-only search (optional)

    Returns:
        Relative improvement from fusion
    """
    best_single = max(semantic_score, lexical_score, ast_score)

    if best_single == 0:
        return 0.0

    return (hybrid_score - best_single) / best_single


def calculate_f1_score(precision: float, recall: float) -> float:
    """
    Calculate F1 score from precision and recall

    F1 = 2 * (P * R) / (P + R)

    Args:
        precision: Precision value
        recall: Recall value

    Returns:
        F1 score
    """
    if precision + recall == 0:
        return 0.0

    return 2 * (precision * recall) / (precision + recall)


# ============================================================================
# Example usage
# ============================================================================

if __name__ == "__main__":
    # Example: Evaluate a search query
    results = [
        SearchResult("doc1", 0.95, 1),
        SearchResult("doc2", 0.87, 2),
        SearchResult("doc3", 0.76, 3),
        SearchResult("doc4", 0.68, 4),
        SearchResult("doc5", 0.55, 5),
    ]

    relevant_docs = {"doc1", "doc3", "doc7"}  # doc7 not in results

    eval_result = evaluate_query(results, relevant_docs)

    print("Evaluation Results:")
    print(f"  P@1:  {eval_result.precision_at_1:.3f}")
    print(f"  P@5:  {eval_result.precision_at_5:.3f}")
    print(f"  P@10: {eval_result.precision_at_10:.3f}")
    print(f"  R@10: {eval_result.recall_at_10:.3f}")
    print(f"  MRR:  {eval_result.mrr:.3f}")
    print(f"  nDCG@10: {eval_result.ndcg_at_10:.3f}")
    print(f"  AP:   {eval_result.average_precision:.3f}")
