#!/usr/bin/env python3
"""
Automated Test Runner - A/B Testing cs --hybrid vs grep/glob baseline

This script orchestrates the complete benchmark evaluation:
1. Loads all 25 code comprehension tasks
2. Runs baseline agent (grep/glob) on each task
3. Runs cs-hybrid agent on each task
4. Collects efficiency metrics (calls, tokens, time, success)
5. Generates comparison reports and visualizations

Goal: Demonstrate cs --hybrid's advantages:
- 70-75% reduction in tool calls
- 85%+ reduction in context tokens
- Faster task completion
- Higher success rate
"""

import sys
from pathlib import Path
import yaml
import json
import time
from typing import List, Dict, Any, Tuple
from dataclasses import dataclass
import statistics

# Add agents to path
sys.path.insert(0, str(Path(__file__).parent.parent / "real_world" / "agents"))

from baseline_agent import BaselineAgent
from cs_hybrid_agent import CSHybridAgent


@dataclass
class ComparisonMetrics:
    """Metrics comparing baseline vs cs-hybrid"""
    task_id: str
    task_category: str
    task_difficulty: str

    # Baseline metrics
    baseline_calls: int
    baseline_tokens: int
    baseline_duration: float
    baseline_precision: float
    baseline_recall: float
    baseline_success: bool

    # CS Hybrid metrics
    cs_calls: int
    cs_tokens: int
    cs_duration: float
    cs_precision: float
    cs_recall: float
    cs_success: bool

    # Improvements
    call_reduction_pct: float
    token_reduction_pct: float
    time_reduction_pct: float
    precision_improvement: float
    recall_improvement: float


class TestRunner:
    """Orchestrates benchmark execution and evaluation"""

    def __init__(self, repo_path: str, verbose: bool = False):
        self.repo_path = Path(repo_path)
        self.verbose = verbose
        self.results: List[ComparisonMetrics] = []

    def load_tasks(self, tasks_file: Path) -> List[Dict[str, Any]]:
        """Load task definitions from YAML"""
        with open(tasks_file) as f:
            content = f.read()

        # Parse YAML - handle both list and individual task formats
        tasks = []
        for doc in yaml.safe_load_all(content):
            if isinstance(doc, list):
                tasks.extend(doc)
            elif isinstance(doc, dict) and 'id' in doc:
                tasks.append(doc)

        # Filter out metadata
        tasks = [t for t in tasks if isinstance(t, dict) and 'id' in t]

        if self.verbose:
            print(f"Loaded {len(tasks)} tasks")
            categories = {}
            for task in tasks:
                cat = task.get('category', 'unknown')
                categories[cat] = categories.get(cat, 0) + 1
            print(f"Categories: {categories}")

        return tasks

    def run_single_task(
        self,
        task: Dict[str, Any],
        baseline_agent: BaselineAgent,
        cs_agent: CSHybridAgent
    ) -> ComparisonMetrics:
        """Run both agents on a single task and compare results"""

        task_id = task['id']
        category = task.get('category', 'unknown')
        difficulty = task.get('difficulty', 'medium')

        if self.verbose:
            print(f"\n{'='*70}")
            print(f"Running task: {task_id} ({category}, {difficulty})")
            print(f"Description: {task['task']}")
            print(f"{'='*70}")

        # Run baseline agent
        if self.verbose:
            print("\n--- Baseline Agent (grep/glob) ---")
        baseline_run = baseline_agent.execute_task(task)

        # Run cs-hybrid agent
        if self.verbose:
            print("\n--- CS Hybrid Agent (cs --hybrid) ---")
        cs_run = cs_agent.execute_task(task)

        # Calculate improvements
        call_reduction = (
            (baseline_run.total_calls - cs_run.total_calls) / baseline_run.total_calls * 100
            if baseline_run.total_calls > 0 else 0.0
        )

        token_reduction = (
            (baseline_run.total_output_tokens - cs_run.total_output_tokens) /
            baseline_run.total_output_tokens * 100
            if baseline_run.total_output_tokens > 0 else 0.0
        )

        time_reduction = (
            (baseline_run.total_duration - cs_run.total_duration) /
            baseline_run.total_duration * 100
            if baseline_run.total_duration > 0 else 0.0
        )

        precision_improvement = cs_run.precision - baseline_run.precision
        recall_improvement = cs_run.recall - baseline_run.recall

        metrics = ComparisonMetrics(
            task_id=task_id,
            task_category=category,
            task_difficulty=difficulty,
            baseline_calls=baseline_run.total_calls,
            baseline_tokens=baseline_run.total_output_tokens,
            baseline_duration=baseline_run.total_duration,
            baseline_precision=baseline_run.precision,
            baseline_recall=baseline_run.recall,
            baseline_success=baseline_run.success,
            cs_calls=cs_run.total_calls,
            cs_tokens=cs_run.total_output_tokens,
            cs_duration=cs_run.total_duration,
            cs_precision=cs_run.precision,
            cs_recall=cs_run.recall,
            cs_success=cs_run.success,
            call_reduction_pct=call_reduction,
            token_reduction_pct=token_reduction,
            time_reduction_pct=time_reduction,
            precision_improvement=precision_improvement,
            recall_improvement=recall_improvement
        )

        if self.verbose:
            self._print_comparison(metrics)

        return metrics

    def _print_comparison(self, metrics: ComparisonMetrics):
        """Pretty print comparison results"""
        print(f"\n--- Results Comparison ---")
        print(f"Task: {metrics.task_id} ({metrics.task_category}, {metrics.task_difficulty})")
        print(f"\nBaseline (grep/glob):")
        print(f"  Tool calls: {metrics.baseline_calls}")
        print(f"  Output tokens: {metrics.baseline_tokens:,}")
        print(f"  Duration: {metrics.baseline_duration:.2f}s")
        print(f"  Precision: {metrics.baseline_precision:.1%}")
        print(f"  Recall: {metrics.baseline_recall:.1%}")
        print(f"  Success: {metrics.baseline_success}")

        print(f"\nCS Hybrid (cs --hybrid):")
        print(f"  Tool calls: {metrics.cs_calls}")
        print(f"  Output tokens: {metrics.cs_tokens:,}")
        print(f"  Duration: {metrics.cs_duration:.2f}s")
        print(f"  Precision: {metrics.cs_precision:.1%}")
        print(f"  Recall: {metrics.cs_recall:.1%}")
        print(f"  Success: {metrics.cs_success}")

        print(f"\nImprovements:")
        print(f"  Tool call reduction: {metrics.call_reduction_pct:.1f}%")
        print(f"  Token reduction: {metrics.token_reduction_pct:.1f}%")
        print(f"  Time reduction: {metrics.time_reduction_pct:.1f}%")
        print(f"  Precision improvement: {metrics.precision_improvement:+.1%}")
        print(f"  Recall improvement: {metrics.recall_improvement:+.1%}")

    def run_all_tasks(
        self,
        tasks: List[Dict[str, Any]],
        max_tasks: Optional[int] = None
    ) -> List[ComparisonMetrics]:
        """Run benchmark on all tasks"""

        baseline_agent = BaselineAgent(self.repo_path, verbose=self.verbose)
        cs_agent = CSHybridAgent(self.repo_path, verbose=self.verbose)

        results = []
        tasks_to_run = tasks[:max_tasks] if max_tasks else tasks

        print(f"\n{'='*70}")
        print(f"Running benchmark on {len(tasks_to_run)} tasks")
        print(f"{'='*70}\n")

        start_time = time.time()

        for i, task in enumerate(tasks_to_run, 1):
            print(f"\n[{i}/{len(tasks_to_run)}] Processing {task['id']}...")

            try:
                metrics = self.run_single_task(task, baseline_agent, cs_agent)
                results.append(metrics)
            except Exception as e:
                print(f"ERROR processing {task['id']}: {e}")
                continue

        total_duration = time.time() - start_time

        print(f"\n{'='*70}")
        print(f"Benchmark complete! Processed {len(results)} tasks in {total_duration:.1f}s")
        print(f"{'='*70}\n")

        self.results = results
        return results

    def generate_summary_report(self) -> Dict[str, Any]:
        """Generate summary statistics"""

        if not self.results:
            return {}

        # Overall improvements
        call_reductions = [m.call_reduction_pct for m in self.results]
        token_reductions = [m.token_reduction_pct for m in self.results]
        time_reductions = [m.time_reduction_pct for m in self.results]

        # Success rates
        baseline_successes = sum(1 for m in self.results if m.baseline_success)
        cs_successes = sum(1 for m in self.results if m.cs_success)

        # Precision/Recall
        baseline_precision = statistics.mean(m.baseline_precision for m in self.results)
        cs_precision = statistics.mean(m.cs_precision for m in self.results)
        baseline_recall = statistics.mean(m.baseline_recall for m in self.results)
        cs_recall = statistics.mean(m.cs_recall for m in self.results)

        # By category
        categories = {}
        for metric in self.results:
            cat = metric.task_category
            if cat not in categories:
                categories[cat] = []
            categories[cat].append(metric)

        category_summary = {}
        for cat, metrics in categories.items():
            category_summary[cat] = {
                'count': len(metrics),
                'avg_call_reduction': statistics.mean(m.call_reduction_pct for m in metrics),
                'avg_token_reduction': statistics.mean(m.token_reduction_pct for m in metrics),
                'cs_success_rate': sum(1 for m in metrics if m.cs_success) / len(metrics)
            }

        summary = {
            'total_tasks': len(self.results),
            'overall_improvements': {
                'avg_call_reduction_pct': statistics.mean(call_reductions),
                'median_call_reduction_pct': statistics.median(call_reductions),
                'avg_token_reduction_pct': statistics.mean(token_reductions),
                'median_token_reduction_pct': statistics.median(token_reductions),
                'avg_time_reduction_pct': statistics.mean(time_reductions),
            },
            'success_rates': {
                'baseline': baseline_successes / len(self.results),
                'cs_hybrid': cs_successes / len(self.results),
                'improvement': (cs_successes - baseline_successes) / len(self.results)
            },
            'precision_recall': {
                'baseline_precision': baseline_precision,
                'cs_precision': cs_precision,
                'baseline_recall': baseline_recall,
                'cs_recall': cs_recall,
                'precision_improvement': cs_precision - baseline_precision,
                'recall_improvement': cs_recall - baseline_recall
            },
            'by_category': category_summary
        }

        return summary

    def print_summary(self, summary: Dict[str, Any]):
        """Pretty print summary report"""

        print(f"\n{'='*70}")
        print("BENCHMARK SUMMARY REPORT")
        print(f"{'='*70}\n")

        print(f"Total tasks evaluated: {summary['total_tasks']}")

        print(f"\n--- Overall Improvements (cs --hybrid vs grep/glob) ---")
        improvements = summary['overall_improvements']
        print(f"  Average tool call reduction: {improvements['avg_call_reduction_pct']:.1f}%")
        print(f"  Median tool call reduction: {improvements['median_call_reduction_pct']:.1f}%")
        print(f"  Average token reduction: {improvements['avg_token_reduction_pct']:.1f}%")
        print(f"  Median token reduction: {improvements['median_token_reduction_pct']:.1f}%")
        print(f"  Average time reduction: {improvements['avg_time_reduction_pct']:.1f}%")

        print(f"\n--- Success Rates ---")
        success = summary['success_rates']
        print(f"  Baseline (grep/glob): {success['baseline']:.1%}")
        print(f"  CS Hybrid: {success['cs_hybrid']:.1%}")
        print(f"  Improvement: {success['improvement']:+.1%}")

        print(f"\n--- Precision & Recall ---")
        pr = summary['precision_recall']
        print(f"  Baseline precision: {pr['baseline_precision']:.1%}")
        print(f"  CS Hybrid precision: {pr['cs_precision']:.1%}")
        print(f"  Precision improvement: {pr['precision_improvement']:+.1%}")
        print(f"  Baseline recall: {pr['baseline_recall']:.1%}")
        print(f"  CS Hybrid recall: {pr['cs_recall']:.1%}")
        print(f"  Recall improvement: {pr['recall_improvement']:+.1%}")

        print(f"\n--- By Category ---")
        for cat, stats in summary['by_category'].items():
            print(f"\n  {cat} ({stats['count']} tasks):")
            print(f"    Avg call reduction: {stats['avg_call_reduction_pct']:.1f}%")
            print(f"    Avg token reduction: {stats['avg_token_reduction_pct']:.1f}%")
            print(f"    CS success rate: {stats['cs_success_rate']:.1%}")

        print(f"\n{'='*70}\n")

    def save_results(self, output_dir: Path):
        """Save detailed results and summary to JSON"""

        output_dir.mkdir(parents=True, exist_ok=True)

        # Save detailed metrics
        detailed_file = output_dir / "detailed_results.json"
        with open(detailed_file, 'w') as f:
            json.dump([
                {
                    'task_id': m.task_id,
                    'category': m.task_category,
                    'difficulty': m.task_difficulty,
                    'baseline': {
                        'calls': m.baseline_calls,
                        'tokens': m.baseline_tokens,
                        'duration': m.baseline_duration,
                        'precision': m.baseline_precision,
                        'recall': m.baseline_recall,
                        'success': m.baseline_success
                    },
                    'cs_hybrid': {
                        'calls': m.cs_calls,
                        'tokens': m.cs_tokens,
                        'duration': m.cs_duration,
                        'precision': m.cs_precision,
                        'recall': m.cs_recall,
                        'success': m.cs_success
                    },
                    'improvements': {
                        'call_reduction_pct': m.call_reduction_pct,
                        'token_reduction_pct': m.token_reduction_pct,
                        'time_reduction_pct': m.time_reduction_pct,
                        'precision_improvement': m.precision_improvement,
                        'recall_improvement': m.recall_improvement
                    }
                }
                for m in self.results
            ], f, indent=2)

        # Save summary
        summary = self.generate_summary_report()
        summary_file = output_dir / "summary_report.json"
        with open(summary_file, 'w') as f:
            json.dump(summary, f, indent=2)

        print(f"Results saved to:")
        print(f"  Detailed: {detailed_file}")
        print(f"  Summary: {summary_file}")


def main():
    """Run benchmark evaluation"""
    import argparse

    parser = argparse.ArgumentParser(
        description="Run A/B benchmark: cs --hybrid vs grep/glob baseline"
    )
    parser.add_argument(
        '--repo',
        default='/Users/arthur/dev-space/semcs',
        help='Path to repository to benchmark'
    )
    parser.add_argument(
        '--max-tasks',
        type=int,
        help='Maximum number of tasks to run (default: all)'
    )
    parser.add_argument(
        '--verbose', '-v',
        action='store_true',
        help='Verbose output'
    )
    parser.add_argument(
        '--category',
        help='Run only tasks in specific category'
    )
    parser.add_argument(
        '--difficulty',
        choices=['easy', 'medium', 'hard', 'very_hard'],
        help='Run only tasks of specific difficulty'
    )

    args = parser.parse_args()

    # Load tasks
    tasks_file = Path(__file__).parent.parent / "real_world" / "tasks" / "code_comprehension_tasks.yaml"
    runner = TestRunner(args.repo, verbose=args.verbose)
    tasks = runner.load_tasks(tasks_file)

    # Filter tasks
    if args.category:
        tasks = [t for t in tasks if t.get('category') == args.category]
        print(f"Filtered to {len(tasks)} tasks in category: {args.category}")

    if args.difficulty:
        tasks = [t for t in tasks if t.get('difficulty') == args.difficulty]
        print(f"Filtered to {len(tasks)} tasks with difficulty: {args.difficulty}")

    # Run benchmark
    results = runner.run_all_tasks(tasks, max_tasks=args.max_tasks)

    # Generate and print summary
    summary = runner.generate_summary_report()
    runner.print_summary(summary)

    # Save results
    output_dir = Path(__file__).parent.parent / "automation" / "results"
    runner.save_results(output_dir)


if __name__ == "__main__":
    main()
