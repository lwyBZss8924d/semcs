# SEMCS-Benchmarks Architecture Design

**Version**: 2.0
**Date**: 2025-10-15
**Status**: Design Approved

## Overview

SEMCS-Benchmarks is a comprehensive evaluation system for measuring the effectiveness of semantic code search tools (specifically `cs`) when integrated with coding agents. The benchmark follows industry best practices from SWE-bench and leverages the Claude Code Agent SDK for real-world agent simulation.

## Core Design Principles

1. **Real Agent Simulation**: Use Claude Code Agent SDK, not subprocess simulations
2. **A/B Testing**: Direct comparison between agents with/without semantic search
3. **Concurrent Execution**: Main agent orchestrates parallel subagent execution
4. **Multi-dimensional Metrics**: Accuracy, efficiency, cost, time
5. **Session-based Analysis**: Export full conversation data for deep insights
6. **CLI-only Interface**: REPL and non-REPL modes, no web visualization

## Architecture: Main Agent + Concurrent Subagents

```text
┌─────────────────────────────────────────────────────────────┐
│                   Main Orchestrator Agent                   │
│  - Reads task from dataset                                  │
│  - Spawns 2 concurrent subagents (A/B)                      │
│  - Collects results and metrics                             │
│  - Verifies against ground truth                            │
│  - Exports session data                                     │
└─────────────────┬───────────────────────────────────────────┘
                  │
          ┌───────┴────────┐
          │                │
          ▼                ▼
┌──────────────────┐ ┌──────────────────┐
│ Subagent A       │ │ Subagent B       │
│ (cs-hybrid)      │ │ (baseline)       │
├──────────────────┤ ├──────────────────┤
│ Tools:           │ │ Tools:           │
│ - Bash (cs)      │ │ - Read           │
│ - Read           │ │ - Grep           │
│ - Grep           │ │ - Glob           │
│ - Glob           │ │                  │
├──────────────────┤ ├──────────────────┤
│ System Prompts:  │ │ System Prompts:  │
│ - CS usage guide │ │ - RL rewards     │
│ - Navigation     │ │                  │
│ - RL rewards     │ │                  │
└──────────────────┘ └──────────────────┘
          │                │
          └────────┬───────┘
                   │
                   ▼
         ┌─────────────────┐
         │  Hooks Layer    │
         ├─────────────────┤
         │ - Metrics Hook  │
         │ - Cost Hook     │
         │ - Verify Hook   │
         └─────────────────┘
                   │
                   ▼
         ┌─────────────────┐
         │  Data Layer     │
         ├─────────────────┤
         │ - Session Export│
         │ - CSV Summary   │
         │ - JSON Results  │
         └─────────────────┘
```

## Component Specifications

### 1. Dataset Layer

**Location**: `benchmarks/datasets/codex/`

**Structure**:

```tree
datasets/
└── codex/
    ├── tasks.yaml                  # 70 benchmark tasks
    ├── ground_truth/               # Verified answers
    │   ├── comp-001.json
    │   ├── comp-002.json
    │   └── ...
    └── metadata.json               # Dataset statistics
```

**Task Format**:

```yaml
- id: comp-001
  category: simple_search
  difficulty: easy
  task: "Find all error handling patterns using Result type"
  query_en: "error handling Result type pattern"
  ground_truth:
    files:
      - "src/agent/codingAgent.ts"
      - "src/core/session.ts"
    snippets:
      - file: "src/agent/codingAgent.ts"
        lines: "145-160"
        content: "export class CodingAgent {...}"
    expected_understanding:
      key_concepts: ["Error propagation", "Result<T,E> pattern"]
  expected_metrics:
    baseline_tool_calls: 6-8
    cs_hybrid_tool_calls: 1-2
    difficulty_weight: 1.0
```

**Dataset Construction Method**:

- Semi-automated AST analysis of codex repository
- Manual ground truth verification by human experts
- 70 tasks across 5 categories (14 tasks each)
- Categories: simple_search, architecture, cross_file, refactoring, multilingual

### 2. Agent Layer

**Location**: `benchmarks/agents/`

**Components**:

#### Main Orchestrator (`main_orchestrator.py`)

- **Role**: Coordinates benchmark execution
- **Responsibilities**:
  - Load task from dataset
  - Spawn concurrent subagents
  - Collect results from both subagents
  - Run verification hooks
  - Export session data
  - Calculate winner

#### CS-Hybrid Subagent (`cs_hybrid_subagent.py`)

- **Tools**: `["Bash", "Read", "Grep", "Glob"]`
- **Special Access**: Can run `cs` via Bash tool
- **System Prompts**: (II) CS Usage + (III) Navigation + (IV) RL Rewards
- **Strategy**: Semantic-first with gradient descent navigation

#### Baseline Subagent (`baseline_subagent.py`)

- **Tools**: `["Read", "Grep", "Glob"]` (NO Bash)
- **Special Access**: None (standard tools only)
- **System Prompts**: (IV) RL Rewards only
- **Strategy**: Traditional grep/glob search patterns

### 3. Prompt System (4 Categories)

**Location**: `benchmarks/agents/prompts/`

#### (I) Main Workflow Prompt (`main_workflow.md`)

- **Purpose**: Orchestration logic for main agent
- **Content**: Task workflow, subagent spawning, result collection
- **Length**: ~50 lines

#### (II) CS Usage Guide (`cs_usage_guide.md`)

- **Purpose**: Teach agent how to use `cs` CLI effectively
- **Content**: CLI syntax, usage patterns, when to use cs vs grep
- **Length**: ~80 lines

#### (III) Gradient Descent Navigation (`gradient_descent.md`)

- **Purpose**: Define search strategy (iterative refinement)
- **Content**: Algorithm steps, convergence criteria, examples
- **Length**: ~100 lines

#### (IV) RL Reward Model (`rl_rewards.md`)

- **Purpose**: Incentivize efficient, accurate behavior
- **Content**: Reward formula, scoring examples, optimization tips
- **Length**: ~70 lines

### 4. Hooks Layer

**Location**: `benchmarks/agents/hooks/`

#### Metrics Hook (`metrics_hook.py`)

- **Trigger**: `@Hook("PreToolUse")`
- **Collects**: Tool name, agent name, timestamp
- **Output**: `{tool_calls: N, tools_used: [...]}`

#### Cost Hook (`cost_tracker_hook.py`)

- **Trigger**: `@Hook("PostToolUse")`
- **Collects**: Token usage, USD cost per message
- **Output**: `{cost_usd: 0.123, tokens: 1500}`

#### Verification Hook (`verification_hook.py`)

- **Trigger**: `@Hook("PostToolUse")`
- **Collects**: Found files, matches with ground truth
- **Output**: `{precision: 0.9, recall: 0.8, f1: 0.85}`

### 5. CLI Layer

**Location**: `benchmarks/cli/`

#### Non-REPL Runner (`bench_runner.py`)

```shell
# Run full benchmark
python cli/bench_runner.py --dataset codex --output results/run_001/

# Run specific category
python cli/bench_runner.py --dataset codex --category architecture

# Run with verbose logging
python cli/bench_runner.py --dataset codex --verbose
```

**Features**:

- Sequential task execution
- Real-time progress display
- Immediate session export
- CSV/JSON summary generation

#### REPL Mode (`bench_repl.py`)

```shell
$ python cli/bench_repl.py --dataset codex

SEMCS-Benchmark REPL v2.0
> list categories
> run task comp-001
> show results comp-001
> compare baseline cs-hybrid
> export sessions
```

**Features**:

- Interactive task selection
- Live result display
- Manual verification override
- Session replay

### 6. Slash Commands

**Location**: `.claude/commands/`

#### `/verify` Command

```markdown
# .claude/commands/verify.md
Check if found files match ground truth for task $1.
Report precision, recall, F1.
```

#### `/ground-truth` Command

```markdown
# .claude/commands/ground-truth.md
Inject ground truth for current task without revealing during search phase.
```

### 7. Session Export

**Format**: JSON per task
**Location**: `results/{run_id}/sessions/`

**Structure**:

```json
{
  "task_id": "comp-001",
  "timestamp": "2025-10-15T23:45:00Z",
  "messages": [
    {
      "role": "assistant",
      "content": "I'll search for error handling patterns...",
      "tool_uses": [{"name": "Bash", "input": {"command": "cs --hybrid 'error handling'"}}]
    }
  ],
  "baseline_result": {
    "tool_calls": 8,
    "cost_usd": 0.12,
    "found_files": ["file1.ts", "file2.ts"]
  },
  "cs_hybrid_result": {
    "tool_calls": 2,
    "cost_usd": 0.05,
    "found_files": ["file1.ts", "file2.ts", "file3.ts"]
  },
  "winner": "cs_hybrid",
  "improvement_pct": 75
}
```

## Data Flow

```text
1. Load Task from Dataset
   ↓
2. Main Agent reads task YAML
   ↓
3. Spawn 2 Concurrent Subagents
   ├─ Subagent A (cs-hybrid)
   └─ Subagent B (baseline)
   ↓
4. Both Subagents Execute Task
   ├─ Hook: PreToolUse → Log metrics
   ├─ Tool Execution
   └─ Hook: PostToolUse → Verify & track cost
   ↓
5. Collect Results from Both
   ↓
6. Run Verification
   ├─ Compare found_files vs ground_truth
   └─ Calculate P/R/F1
   ↓
7. Determine Winner
   ├─ Score = accuracy × efficiency
   └─ Winner = highest score
   ↓
8. Export Session Data
   ├─ JSON: Full conversation
   ├─ CSV: Summary row
   └─ Logs: Detailed trace
```

## Technology Stack

- **Agent Framework**: Claude Code Agent SDK (Python)
- **Language Model**: Claude Sonnet 4.5
- **Dataset Format**: YAML + JSON
- **CLI Framework**: Python asyncio + argparse
- **Session Storage**: JSON files
- **Summary Format**: CSV + JSON
- **Dependency Management**: uv + pyproject.toml

## Success Criteria

### For Dataset (Phase 1)

- [] 70 high-quality tasks created
- [] All ground truth manually verified
- [] Expected metrics defined per task
- [] Balance across 5 categories

### For Agents (Phase 2)

- [] Main agent spawns concurrent subagents
- [] CS-hybrid can access cs via Bash
- [] Baseline restricted to standard tools
- [] All 4 prompts written and tested
- [] Hooks collect accurate metrics

### For CLI (Phase 3)

- [] Non-REPL runs 70 tasks successfully
- [] REPL supports interactive exploration
- [] Sessions exported in parseable format
- [] CSV summary ready for analysis

### For Results (Phase 4)

- [] Statistical significance (p < 0.05)
- [] cs-hybrid wins >80% of tasks
- [] Clear efficiency improvement shown
- [] Qualitative insights extracted

## Timeline

- **Week 1-2**: Dataset construction (70 tasks + ground truth)
- **Week 2-3**: Agent implementation (main + subagents + hooks)
- **Week 3-4**: CLI development (runner + REPL + session export)
- **Week 4**: Testing and validation (10-task pilot)
- **Week 5**: Full evaluation (70 tasks + analysis)

**Total Duration**: 5 weeks

## References

- **SWE-bench**: Docker-based evaluation, functional verification
- **SWE-agent**: Agent-Computer Interface, template system, YAML configs
- **semtools**: A/B testing methodology with system prompt variants
- **Claude Code Agent SDK**: Hooks, subagents, sessions, slash-commands, cost-tracking

---

**Next Document**: See [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md) for detailed task breakdown.
