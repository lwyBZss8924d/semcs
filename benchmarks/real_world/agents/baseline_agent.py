#!/usr/bin/env python3
"""
Baseline Agent - Uses only grep/glob/rg (No semantic search)

Simulates a Coding Agent (like Claude Code) that only has access to:
- grep/rg for text search
- glob/find for file discovery
- read for file reading

This is the control group for measuring cs --hybrid's efficiency gains.
"""

import subprocess
import time
from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from pathlib import Path
import json


@dataclass
class ToolCall:
    """Record of a single tool invocation"""
    tool: str
    args: List[str]
    timestamp: float
    duration: float
    output_tokens: int  # Estimated tokens in output


@dataclass
class AgentRun:
    """Record of complete agent execution on a task"""
    task_id: str
    task_description: str
    query: str

    # Metrics
    tool_calls: List[ToolCall]
    total_calls: int
    total_duration: float
    total_output_tokens: int

    # Results
    files_found: List[str]
    ground_truth_files: List[str]
    precision: float
    recall: float
    success: bool

    # Strategy
    exploration_path: List[str]  # Order of exploration


class BaselineAgent:
    """Agent that uses only grep/glob for code search"""

    def __init__(self, repo_path: str, verbose: bool = False):
        self.repo_path = Path(repo_path)
        self.verbose = verbose
        self.tool_calls: List[ToolCall] = []

    def _estimate_tokens(self, text: str) -> int:
        """Estimate token count (rough: ~4 chars per token)"""
        return len(text) // 4

    def _run_command(self, tool: str, args: List[str]) -> str:
        """Execute a command and record metrics"""
        start = time.time()

        try:
            result = subprocess.run(
                [tool] + args,
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                timeout=30
            )
            output = result.stdout
        except subprocess.TimeoutExpired:
            output = ""
        except FileNotFoundError:
            output = ""

        duration = time.time() - start
        tokens = self._estimate_tokens(output)

        # Record tool call
        self.tool_calls.append(ToolCall(
            tool=tool,
            args=args,
            timestamp=start,
            duration=duration,
            output_tokens=tokens
        ))

        if self.verbose:
            print(f"[{tool}] {' '.join(args)} -> {len(output)} chars, {tokens} tokens")

        return output

    def glob_search(self, pattern: str) -> List[str]:
        """Use find to search for files by pattern"""
        output = self._run_command("find", [
            ".",
            "-type", "f",
            "-name", pattern,
            "-not", "-path", "*/target/*",
            "-not", "-path", "*/.git/*",
            "-not", "-path", "*/node_modules/*"
        ])

        files = [line.strip() for line in output.split('\n') if line.strip()]
        return files

    def grep_search(self, pattern: str, file_pattern: str = "*") -> List[str]:
        """Use ripgrep to search for text pattern"""
        output = self._run_command("rg", [
            "--files-with-matches",
            "--no-heading",
            pattern,
            "--glob", file_pattern,
            "--glob", "!target/",
            "--glob", "!.git/",
            "--glob", "!node_modules/"
        ])

        files = [line.strip() for line in output.split('\n') if line.strip()]
        return files

    def grep_content(self, pattern: str, context: int = 2) -> str:
        """Use ripgrep to get content with context"""
        output = self._run_command("rg", [
            pattern,
            "--context", str(context),
            "--glob", "*.rs",
            "--glob", "*.ts",
            "--glob", "!target/",
            "--glob", "!.git/"
        ])
        return output

    def read_file(self, filepath: str, limit: int = 1000) -> str:
        """Read a file (simulates Read tool)"""
        try:
            with open(self.repo_path / filepath, 'r') as f:
                content = f.read(limit * 100)  # Rough line limit
            return content
        except Exception as e:
            return f"Error reading {filepath}: {e}"

    def execute_task(self, task: Dict[str, Any]) -> AgentRun:
        """
        Execute a code comprehension task using only grep/glob.

        This simulates how a Coding Agent would need to iteratively search
        without semantic understanding or AST awareness.
        """
        self.tool_calls = []
        start_time = time.time()

        task_id = task['id']
        description = task['task']
        query_en = task['query_en']
        ground_truth = task.get('ground_truth_files', [])

        files_found = set()
        exploration_path = []

        if self.verbose:
            print(f"\n{'='*60}")
            print(f"Task: {task_id}")
            print(f"Description: {description}")
            print(f"Query: {query_en}")
            print(f"{'='*60}\n")

        # Strategy: Multiple grep searches with different keywords
        # This mimics how an agent without semantic search would iteratively explore

        keywords = query_en.split()

        # Phase 1: Search for each keyword separately
        for keyword in keywords[:3]:  # Limit to avoid explosion
            exploration_path.append(f"grep:{keyword}")
            matches = self.grep_search(keyword, "*.rs")
            files_found.update(matches[:5])  # Take top 5 per keyword

        # Phase 2: Try combined patterns
        if len(keywords) >= 2:
            combined = " ".join(keywords[:2])
            exploration_path.append(f"grep:{combined}")
            matches = self.grep_search(combined, "*.rs")
            files_found.update(matches[:5])

        # Phase 3: File pattern search if task mentions specific components
        if "config" in query_en.lower():
            exploration_path.append("glob:*config*.rs")
            matches = self.glob_search("*config*.rs")
            files_found.update(matches)

        if "error" in query_en.lower():
            exploration_path.append("grep:Result")
            matches = self.grep_search(r"Result<", "*.rs")
            files_found.update(matches[:5])

        # Phase 4: Read some files to understand (context consumption!)
        for filepath in list(files_found)[:3]:  # Read first 3 files
            exploration_path.append(f"read:{filepath}")
            self.read_file(filepath, limit=100)

        # Phase 5: Refinement based on initial findings (iterative)
        # Simulate needing multiple rounds to find relevant files
        if task.get('cross_file', False):
            # Cross-file tasks require more exploration
            exploration_path.append("grep:impl")
            impl_files = self.grep_search(r"impl\s+", "*.rs")
            files_found.update(impl_files[:3])

        # Calculate metrics
        total_duration = time.time() - start_time
        total_calls = len(self.tool_calls)
        total_tokens = sum(call.output_tokens for call in self.tool_calls)

        # Calculate precision/recall
        files_found_list = list(files_found)
        ground_truth_set = set(ground_truth)
        found_set = set(files_found_list)

        true_positives = len(found_set & ground_truth_set)
        precision = true_positives / len(found_set) if found_set else 0.0
        recall = true_positives / len(ground_truth_set) if ground_truth_set else 0.0

        # Success criteria: Find at least 70% of ground truth
        success = recall >= 0.7

        return AgentRun(
            task_id=task_id,
            task_description=description,
            query=query_en,
            tool_calls=self.tool_calls.copy(),
            total_calls=total_calls,
            total_duration=total_duration,
            total_output_tokens=total_tokens,
            files_found=files_found_list,
            ground_truth_files=ground_truth,
            precision=precision,
            recall=recall,
            success=success,
            exploration_path=exploration_path
        )

    def run_to_dict(self, run: AgentRun) -> Dict[str, Any]:
        """Convert AgentRun to JSON-serializable dict"""
        return {
            'task_id': run.task_id,
            'task_description': run.task_description,
            'query': run.query,
            'metrics': {
                'total_calls': run.total_calls,
                'total_duration': run.total_duration,
                'total_output_tokens': run.total_output_tokens,
                'precision': run.precision,
                'recall': run.recall,
                'success': run.success
            },
            'results': {
                'files_found': run.files_found,
                'ground_truth_files': run.ground_truth_files,
                'exploration_path': run.exploration_path
            },
            'tool_calls': [
                {
                    'tool': call.tool,
                    'args': call.args,
                    'duration': call.duration,
                    'output_tokens': call.output_tokens
                }
                for call in run.tool_calls
            ]
        }


def main():
    """Test baseline agent on sample tasks"""
    import yaml

    # Load tasks
    tasks_file = Path(__file__).parent.parent / "tasks" / "code_comprehension_tasks.yaml"
    with open(tasks_file) as f:
        content = f.read()
        # Parse YAML manually to skip metadata
        tasks = []
        for doc in yaml.safe_load_all(content):
            if isinstance(doc, list):
                tasks = doc
                break
            elif isinstance(doc, dict) and 'id' in doc:
                tasks.append(doc)

    # Test on first 3 easy tasks
    repo_path = Path(__file__).parent.parent.parent.parent
    agent = BaselineAgent(repo_path, verbose=True)

    results = []
    for task in tasks[:3]:  # Test first 3 tasks
        if task.get('difficulty') == 'easy':
            print(f"\nTesting: {task['id']}")
            run = agent.execute_task(task)
            results.append(agent.run_to_dict(run))

            print(f"\nResults:")
            print(f"  Tool calls: {run.total_calls}")
            print(f"  Duration: {run.total_duration:.2f}s")
            print(f"  Output tokens: {run.total_output_tokens}")
            print(f"  Precision: {run.precision:.2%}")
            print(f"  Recall: {run.recall:.2%}")
            print(f"  Success: {run.success}")

    # Save results
    output_file = Path(__file__).parent.parent / "results" / "baseline_test.json"
    output_file.parent.mkdir(exist_ok=True)
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"\n\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()
