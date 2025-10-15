# SEMCS-Benchmarks Implementation Tasks

**Version**: 2.0
**Timeline**: 5 weeks
**Status**: Ready to implement

---

## Week 1-2: Dataset Construction

### Goal: Create 70 high-quality benchmark tasks with verified ground truth

### Dataset Construction Tasks

#### Task 1.1: Setup Dataset Infrastructure (Day 1)

- [ ] Create directory: `benchmarks/datasets/codex/`
- [ ] Create `tasks.yaml` schema template
- [ ] Create `ground_truth/` directory
- [ ] Create `metadata.json` structure
- [ ] Document dataset format in README

**Deliverable**: Empty but structured dataset directories

---

#### Task 1.2: AST-Based Task Generation (Days 2-3)

- [ ] Write Python script: `automation/dataset_builder.py`
- [ ] Implement AST parsers for TypeScript
- [ ] Extract patterns:
  - [ ] All function definitions
  - [ ] All class definitions
  - [ ] All interface definitions
  - [ ] All import statements
  - [ ] All error handling blocks
- [ ] Generate candidate tasks (aim for 100+ candidates)
- [ ] Score candidates by quality (complexity, uniqueness)

**Deliverable**: `automation/dataset_builder.py` + 100+ candidate tasks

**Example Code**:

```python
# automation/dataset_builder.py
import ast
from pathlib import Path
from tree_sitter import Language, Parser

def extract_functions(file_path):
    """Extract all function definitions from TypeScript file"""
    # Use tree-sitter to parse TypeScript
    # Return list of (function_name, file, lines)
    pass

def generate_task_from_function(func_name, file, lines):
    """Generate benchmark task from function"""
    return {
        "id": f"comp-{next_id()}",
        "category": "simple_search",
        "task": f"Find the implementation of {func_name} function",
        "query_en": f"{func_name} function implementation",
        "ground_truth": {
            "files": [file],
            "snippets": [{"file": file, "lines": lines}]
        }
    }
```

---

#### Task 1.3: Manual Task Curation (Days 4-7)

- [ ] Review 100+ candidate tasks
- [ ] Select 70 best tasks (14 per category)
- [ ] Manually verify ground truth for each:
  - [ ] Run cs and baseline searches manually
  - [ ] Confirm file paths are correct
  - [ ] Extract relevant code snippets
  - [ ] Document expected tool calls
- [ ] Balance difficulty (easy:medium:hard = 50:30:20)
- [ ] Write `ground_truth/{task_id}.json` for each task

**Deliverable**: 70 fully verified tasks in `tasks.yaml`

**Categories** (14 tasks each):

1. **simple_search**: "Find all X pattern"
2. **architecture**: "Trace flow from A to B"
3. **cross_file**: "Find all files that use X"
4. **refactoring**: "Locate all code for consolidation"
5. **multilingual**: Mixed TS/JS/JSON/MD tasks

**Example Ground Truth**:

```json
{
  "task_id": "comp-001",
  "files": [
    "src/agent/codingAgent.ts",
    "src/core/session.ts"
  ],
  "snippets": [
    {
      "file": "src/agent/codingAgent.ts",
      "lines": "145-160",
      "content": "export class CodingAgent implements Agent {...}"
    }
  ],
  "expected_understanding": {
    "key_concepts": ["Agent initialization", "Hook registration"],
    "why_important": "Core agent architecture pattern"
  },
  "manual_verification": {
    "cs_hybrid_calls": 2,
    "baseline_calls": 7,
    "notes": "cs finds it immediately via semantic similarity"
  }
}
```

---

#### Task 1.4: Dataset Validation (Days 8-10)

- [ ] Run pilot test: Execute 10 tasks manually
- [ ] Verify expected metrics match reality
- [ ] Adjust difficulty labels if needed
- [ ] Check for duplicate/similar tasks
- [ ] Write dataset statistics report

**Deliverable**: Validated dataset + statistics report

**Statistics to collect**:

- Tasks per category
- Difficulty distribution
- Average ground truth files per task
- Total lines of code covered
- Repository coverage percentage

---

## Week 2-3: Agent Implementation

### Goal: Implement main orchestrator + 2 subagents + hooks

### Agent Implementation Tasks

#### Task 2.1: Create Agent Infrastructure (Day 11)

- [ ] Create directory: `benchmarks/agents/`
- [ ] Create `agents/prompts/` for system prompts
- [ ] Create `agents/hooks/` for hook implementations
- [ ] Install Claude Code Agent SDK: `uv add claude-agent-sdk`
- [ ] Write base agent class with common utilities

**Deliverable**: Agent directory structure + SDK installed

---

#### Task 2.2: Write System Prompts (Days 12-13)

- [ ] Write `prompts/main_workflow.md` (Prompt I)
- [ ] Write `prompts/cs_usage_guide.md` (Prompt II)
- [ ] Write `prompts/gradient_descent.md` (Prompt III)
- [ ] Write `prompts/rl_rewards.md` (Prompt IV)
- [ ] Test prompt length (ensure < 3000 tokens each)

**Deliverable**: 4 complete system prompt files

---

#### Task 2.3: Implement Hooks (Days 14-15)

- [ ] Create `hooks/metrics_hook.py`:
  - [ ] PreToolUse hook for counting calls
  - [ ] Track tool names and timestamps
  - [ ] Export to dict format
- [ ] Create `hooks/cost_tracker_hook.py`:
  - [ ] PostToolUse hook for cost collection
  - [ ] Parse usage data from SDK
  - [ ] Accumulate total cost per subagent
- [ ] Create `hooks/verification_hook.py`:
  - [ ] PostToolUse hook for file detection
  - [ ] Parse file paths from tool output
  - [ ] Compare with ground truth
  - [ ] Calculate P/R/F1 in real-time

**Deliverable**: 3 working hook implementations

**Example Hook**:

```python
# hooks/metrics_hook.py
from claude_agent_sdk import Hook, HookInput, HookContext

class MetricsHook:
    def __init__(self):
        self.metrics = {}

    @Hook("PreToolUse")
    async def collect_metrics(self, input_data: HookInput, tool_use_id: str, context: HookContext):
        agent_name = context.get("agent_name", "unknown")
        tool_name = input_data["tool_name"]

        if agent_name not in self.metrics:
            self.metrics[agent_name] = {"tool_calls": 0, "tools_used": []}

        self.metrics[agent_name]["tool_calls"] += 1
        self.metrics[agent_name]["tools_used"].append({
            "tool": tool_name,
            "timestamp": time.time(),
            "tool_use_id": tool_use_id
        })

        return {}  # No blocking
```

---

#### Task 2.4: Implement Subagents (Days 16-18)

- [ ] Create `cs_hybrid_subagent.py`:
  - [ ] Define AgentDefinition with Bash access
  - [ ] Load prompts: CS guide + Navigation + RL rewards
  - [ ] Configure tools: `["Bash", "Read", "Grep", "Glob"]`
- [ ] Create `baseline_subagent.py`:
  - [ ] Define AgentDefinition without Bash
  - [ ] Load prompts: RL rewards only
  - [ ] Configure tools: `["Read", "Grep", "Glob"]`
- [ ] Test both subagents independently on 3 tasks

**Deliverable**: 2 working subagent implementations

**Example Subagent**:

```python
# agents/cs_hybrid_subagent.py
from claude_agent_sdk import AgentDefinition

def create_cs_hybrid_agent(prompts_dir: Path) -> AgentDefinition:
    """Create CS-hybrid subagent with semantic search access"""

    # Load prompts
    cs_guide = (prompts_dir / "cs_usage_guide.md").read_text()
    navigation = (prompts_dir / "gradient_descent.md").read_text()
    rl_rewards = (prompts_dir / "rl_rewards.md").read_text()

    system_prompt = f"{cs_guide}\n\n{navigation}\n\n{rl_rewards}"

    return AgentDefinition(
        description="Code search agent using cs semantic search tool",
        prompt=system_prompt,
        tools=["Bash", "Read", "Grep", "Glob"],
        model="claude-sonnet-4-5",
    )
```

---

#### Task 2.5: Implement Main Orchestrator (Days 19-21)

- [ ] Create `main_orchestrator.py`:
  - [ ] Load task from dataset
  - [ ] Create ClaudeAgentOptions with both subagents
  - [ ] Attach all hooks
  - [ ] Spawn both subagents concurrently
  - [ ] Collect results from both
  - [ ] Run verification
  - [ ] Calculate winner
  - [ ] Export session data
- [ ] Test on 5 diverse tasks
- [ ] Debug and fix issues

**Deliverable**: Working main orchestrator

**Example Orchestrator**:

```python
# agents/main_orchestrator.py
from claude_agent_sdk import ClaudeAgentOptions, query, AgentDefinition
import asyncio

class MainBenchmarkOrchestrator:
    def __init__(self, task_config, hooks):
        self.task = task_config
        self.hooks = hooks
        self.results = {"baseline": {}, "cs_hybrid": {}, "comparison": {}}

    async def run_benchmark(self):
        """Execute A/B test with concurrent subagents"""

        # Load subagent definitions
        cs_hybrid = create_cs_hybrid_agent(Path("agents/prompts"))
        baseline = create_baseline_agent(Path("agents/prompts"))

        # Configure options
        options = ClaudeAgentOptions(
            agents={
                "cs-hybrid": cs_hybrid,
                "baseline": baseline,
            },
            hooks=self.hooks,
        )

        # Main agent prompt
        main_prompt = f"""
You are the SEMCS Benchmark Orchestrator.

TASK: {self.task['task']}
QUERY: {self.task['query_en']}

Execute this task using BOTH subagents concurrently:
1. Launch cs-hybrid subagent
2. Launch baseline subagent
3. Collect their results
4. Verify against ground truth
5. Report winner
"""

        # Run main agent (spawns subagents)
        session_data = []
        async for message in query(prompt=main_prompt, options=options):
            session_data.append(message)
            # Process messages...

        # Extract results
        self._extract_results(session_data)
        self._calculate_winner()

        return self.results
```

---

## Week 3-4: CLI Implementation

### Goal: Build CLI runners with session export

### CLI Implementation Tasks Tasks

#### Task 3.1: Non-REPL Runner (Days 22-24)

- [ ] Create `cli/bench_runner.py`:
  - [ ] Load dataset from YAML
  - [ ] Iterate through all tasks
  - [ ] Run orchestrator for each task
  - [ ] Export session JSON immediately
  - [ ] Show progress bar (use tqdm)
  - [ ] Handle errors gracefully
- [ ] Add command-line arguments:
  - [ ] `--dataset`: Dataset path
  - [ ] `--category`: Filter by category
  - [ ] `--max-tasks`: Limit number of tasks
  - [ ] `--output`: Output directory
  - [ ] `--verbose`: Detailed logging
- [ ] Test full run with 10 tasks

**Deliverable**: Working non-REPL runner

**Example CLI**:

```python
# cli/bench_runner.py
import argparse
from tqdm import tqdm

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True)
    parser.add_argument("--category", default=None)
    parser.add_argument("--max-tasks", type=int, default=None)
    parser.add_argument("--output", default="results/")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    # Load tasks
    tasks = load_dataset(args.dataset)
    if args.category:
        tasks = [t for t in tasks if t["category"] == args.category]
    if args.max_tasks:
        tasks = tasks[:args.max_tasks]

    # Run benchmark
    results = []
    for task in tqdm(tasks, desc="Running benchmark"):
        orchestrator = MainBenchmarkOrchestrator(task, hooks)
        result = await orchestrator.run_benchmark()
        results.append(result)
        export_session(task["id"], result, args.output)

    # Export summary
    export_csv_summary(results, args.output)
    export_json_summary(results, args.output)
```

---

#### Task 3.2: Session Export (Days 25-26)

- [ ] Implement session JSON export:
  - [ ] Extract all messages from SDK
  - [ ] Extract tool uses and outputs
  - [ ] Extract timestamps
  - [ ] Extract costs
  - [ ] Save to `results/{run_id}/sessions/{task_id}.json`
- [ ] Implement CSV summary export:
  - [ ] Columns: task_id, category, baseline_*, cs_hybrid_*, winner
  - [ ] Calculate aggregate statistics
  - [ ] Save to `results/{run_id}/summary.csv`
- [ ] Implement JSON results export:
  - [ ] Full structured results
  - [ ] Save to `results/{run_id}/results.json`

**Deliverable**: Complete export functionality

---

#### Task 3.3: REPL Mode (Days 27-28)

- [ ] Create `cli/bench_repl.py`:
  - [ ] Load dataset on startup
  - [ ] Implement commands:
    - [ ] `list categories`: Show available categories
    - [ ] `list tasks [category]`: Show tasks
    - [ ] `run task <id>`: Run single task
    - [ ] `show task <id>`: Display task details
    - [ ] `show results <id>`: Display task results
    - [ ] `compare baseline cs-hybrid`: Show comparison
    - [ ] `export sessions`: Export all session data
    - [ ] `exit`: Quit REPL
  - [ ] Add tab completion
  - [ ] Add colorized output

**Deliverable**: Working REPL interface

**Example REPL**:

```python
# cli/bench_repl.py
import cmd

class BenchmarkREPL(cmd.Cmd):
    intro = "SEMCS-Benchmark REPL v2.0\nType 'help' for commands."
    prompt = "> "

    def do_list(self, arg):
        """List categories or tasks"""
        if arg == "categories":
            for cat in self.categories:
                print(f"- {cat} ({self.task_counts[cat]} tasks)")
        elif arg.startswith("tasks"):
            category = arg.split()[1] if len(arg.split()) > 1 else None
            tasks = self.get_tasks(category)
            for task in tasks:
                print(f"{task['id']}: {task['task']}")

    def do_run(self, arg):
        """Run task by ID"""
        parts = arg.split()
        if parts[0] == "task":
            task_id = parts[1]
            task = self.get_task(task_id)
            print(f"Running task {task_id}...")
            result = self.run_task(task)
            self.display_result(result)
```

---

#### Task 3.4: Slash Commands (Day 29)

- [ ] Create `.claude/commands/verify.md`:
  - [ ] Accept task_id, found_files, ground_truth
  - [ ] Calculate and display P/R/F1
- [ ] Create `.claude/commands/ground-truth.md`:
  - [ ] Load ground truth for task
  - [ ] Display in readable format
  - [ ] Don't reveal during search phase
- [ ] Test slash commands in orchestrator

**Deliverable**: 2 working slash commands

---

## Week 4: Testing & Validation

### Goal: Validate all components with pilot study

### Testing & Validation Tasks

#### Task 4.1: Unit Tests (Days 30-31)

- [ ] Write tests for hooks:
  - [ ] Test metrics collection
  - [ ] Test cost tracking
  - [ ] Test verification logic
- [ ] Write tests for subagents:
  - [ ] Test cs-hybrid tool access
  - [ ] Test baseline tool restrictions
- [ ] Write tests for orchestrator:
  - [ ] Test concurrent subagent spawning
  - [ ] Test result collection
  - [ ] Test winner calculation
- [ ] Run all tests: `pytest benchmarks/tests/`

**Deliverable**: 20+ passing unit tests

---

#### Task 4.2: Pilot Study - 10 Tasks (Days 32-33)

- [ ] Select 10 diverse tasks (2 per category)
- [ ] Run full benchmark with CLI
- [ ] Collect all metrics and sessions
- [ ] Manually review results:
  - [ ] Are ground truths accurate?
  - [ ] Are metrics correct?
  - [ ] Did hooks work properly?
  - [ ] Is session data complete?
- [ ] Identify and fix bugs

**Deliverable**: Pilot study report + bug fixes

---

#### Task 4.3: Validation & Refinement (Days 34-35)

- [ ] Validate metrics accuracy:
  - [ ] Manually count tool calls → Compare with hook data
  - [ ] Manually calculate costs → Compare with hook data
  - [ ] Manually verify found files → Compare with verification hook
- [ ] Validate session completeness:
  - [ ] Check all messages captured
  - [ ] Check timestamps present
  - [ ] Check tool uses recorded
- [ ] Refine prompts if needed:
  - [ ] Adjust RL reward weights
  - [ ] Clarify navigation strategy
  - [ ] Fix any confusing instructions
- [ ] Re-run pilot study to confirm fixes

**Deliverable**: Validated system ready for full run

---

## Week 5: Full Evaluation

### Goal: Run complete 70-task benchmark and analyze results

### Full Evaluation Tasks

#### Task 5.1: Full Benchmark Run (Days 36-37)

- [ ] Run complete benchmark: `python cli/bench_runner.py --dataset codex --output results/final_run/`
- [ ] Monitor for errors/crashes
- [ ] Ensure all 70 tasks complete
- [ ] Verify all sessions exported
- [ ] Verify CSV/JSON summaries generated

**Deliverable**: Complete benchmark results

**Expected Output**:

```tree
results/final_run/
├── sessions/
│   ├── comp-001.json
│   ├── comp-002.json
│   └── ... (70 files)
├── summary.csv
├── results.json
└── logs/
    └── benchmark.log
```

---

#### Task 5.2: Statistical Analysis (Days 38-39)'

- [ ] Load summary.csv into pandas
- [ ] Calculate aggregate statistics:
  - [ ] Win rate: cs-hybrid vs baseline
  - [ ] Average improvement: tool calls, cost, F1
  - [ ] Per-category breakdown
  - [ ] Difficulty-stratified results
- [ ] Run statistical tests:
  - [ ] Paired t-test for tool calls
  - [ ] Paired t-test for F1 scores
  - [ ] Calculate effect sizes (Cohen's d)
  - [ ] Check for significance (p < 0.05)
- [ ] Generate visualizations:
  - [ ] Bar charts: win rate by category
  - [ ] Scatter plots: efficiency vs accuracy
  - [ ] Histograms: score distributions

**Deliverable**: Statistical analysis notebook

**Example Analysis**:

```python
import pandas as pd
from scipy import stats

# Load results
df = pd.read_csv("results/final_run/summary.csv")

# Win rate
win_rate = (df["winner"] == "cs_hybrid").mean()
print(f"CS-Hybrid Win Rate: {win_rate:.1%}")

# Efficiency improvement
improvement = (df["baseline_calls"] - df["cs_hybrid_calls"]) / df["baseline_calls"]
print(f"Average Tool Call Reduction: {improvement.mean():.1%}")

# Statistical significance
t_stat, p_value = stats.ttest_rel(df["baseline_calls"], df["cs_hybrid_calls"])
print(f"Paired t-test: t={t_stat:.2f}, p={p_value:.4f}")
```

---

#### Task 5.3: Qualitative Analysis (Day 40)

- [ ] Review 10 interesting session logs:
  - [ ] Find cases where cs-hybrid won dramatically
  - [ ] Find cases where baseline competed well
  - [ ] Find failure modes (both agents failed)
- [ ] Extract insights:
  - [ ] What makes cs effective?
  - [ ] When does cs struggle?
  - [ ] What strategies work best?
- [ ] Document case studies

**Deliverable**: Qualitative analysis report

---

#### Task 5.4: Final Report (Days 41-42)

- [ ] Write comprehensive report: `benchmarks/BENCHMARKS_SUMMARY.md`
- [ ] Sections:
  - [ ] Executive Summary
  - [ ] Dataset Description
  - [ ] Methodology
  - [ ] Quantitative Results
  - [ ] Qualitative Insights
  - [ ] Statistical Analysis
  - [ ] Failure Modes
  - [ ] Future Work
- [ ] Include all visualizations
- [ ] Include key session excerpts
- [ ] Include statistical tables

**Deliverable**: Publication-ready benchmark report

---

## Success Criteria

### Phase 1 (Dataset): []

- 70 tasks created and verified
- Ground truth accuracy confirmed manually
- Balance across categories and difficulties

### Phase 2 (Agents): []

- Main orchestrator spawns concurrent subagents
- CS-hybrid has Bash access for cs
- Baseline restricted to standard tools
- All 4 prompts implemented
- Hooks collect accurate metrics

### Phase 3 (CLI): []

- Non-REPL runs all 70 tasks successfully
- REPL supports interactive exploration
- Sessions exported in parseable JSON
- CSV summary ready for analysis

### Phase 4 (Testing): []

- 20+ unit tests passing
- 10-task pilot study successful
- All metrics validated manually
- No critical bugs remaining

### Phase 5 (Evaluation): []

- Full 70-task run completes without errors
- Statistical significance achieved (p < 0.05)
- CS-hybrid wins >80% of tasks
- Clear efficiency improvement demonstrated
- Comprehensive report published

---

## Risk Mitigation

### Risk 1: Claude SDK API Issues

**Mitigation**: Test SDK integration early (Week 2), have fallback subprocess approach

### Risk 2: Ground Truth Errors

**Mitigation**: Manual verification + pilot study validation

### Risk 3: Poor Task Quality

**Mitigation**: Generate 100+ candidates, select best 70, balance categories

### Risk 4: Agent Failures

**Mitigation**: Implement error handling, logging, graceful degradation

### Risk 5: Timeline Slippage

**Mitigation**: Start with highest-risk items (SDK integration), buffer time in Week 5

---

## Team Allocation (If Applicable)

### Solo Developer

- Follow tasks sequentially
- Focus on MVP first (dataset + basic agents)
- Iterate on prompts and hooks

### 2 Developers

- Dev 1: Dataset + CLI (Weeks 1, 3-4)
- Dev 2: Agents + Hooks (Weeks 2-3)
- Both: Testing + Evaluation (Weeks 4-5)

### 3+ Developers

- Dev 1: Dataset construction (Weeks 1-2)
- Dev 2: Agent implementation (Weeks 2-3)
- Dev 3: CLI + infrastructure (Weeks 3-4)
- All: Testing + evaluation (Weeks 4-5)

---

**Next**: See [DAILY_TASKS.md](./DAILY_TASKS.md) for day-by-day breakdown.
