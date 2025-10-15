# SEMCS-Benchmarks Specifications

This directory contains the complete design specifications for the SEMCS benchmarking system.

**Version**: 2.0
**Date**: 2025-10-15
**Status**: Design Complete, Ready for Implementation

---

## 📁 Directory Structure

```tree
specs/
├── README.md                    # This file
├── plan/                        # Design documents
│   ├── ARCHITECTURE.md          # System architecture & components
│   └── SYSTEM_PROMPTS.md        # All 4 system prompt categories
└── tasks/                       # Implementation plan
    └── WEEK_BY_WEEK.md          # 5-week task breakdown
```

---

## 📖 Document Overview

### Plan Documents

#### [ARCHITECTURE.md](./plan/ARCHITECTURE.md)

**Purpose**: Define the complete system architecture

**Contents**:

- Core design principles
- Main agent + subagent architecture diagram
- Component specifications (dataset, agents, hooks, CLI)
- Data flow diagram
- Technology stack
- Success criteria
- Timeline overview

**Read this first** to understand the overall system design.

---

#### [SYSTEM_PROMPTS.md](./plan/SYSTEM_PROMPTS.md)

**Purpose**: Provide all 4 categories of system prompts

**Contents**:

- **(I) Main Workflow Prompt**: Orchestration logic for main agent
- **(II) CS Usage Guide**: How to use `cs` semantic search effectively
- **(III) Gradient Descent Navigation**: Iterative search refinement strategy
- **(IV) RL Reward Model**: Performance incentives and scoring

**Read this second** to understand how agents will behave.

---

### Task Documents

#### [WEEK_BY_WEEK.md](./tasks/WEEK_BY_WEEK.md)

**Purpose**: Detailed 5-week implementation plan

**Contents**:

- Week 1-2: Dataset Construction (70 tasks + ground truth)
- Week 2-3: Agent Implementation (main + subagents + hooks)
- Week 3-4: CLI Development (runner + REPL + session export)
- Week 4: Testing & Validation (pilot study)
- Week 5: Full Evaluation (70-task run + analysis)

**Read this third** to plan implementation work.

---

## 🎯 Quick Start Guide

### For Project Managers

1. Read [ARCHITECTURE.md](./plan/ARCHITECTURE.md) → Understand scope
2. Read [WEEK_BY_WEEK.md](./tasks/WEEK_BY_WEEK.md) → Understand timeline
3. Review success criteria in ARCHITECTURE.md
4. Allocate resources based on task breakdown

### For Developers

1. Read [ARCHITECTURE.md](./plan/ARCHITECTURE.md) → Understand system design
2. Read [SYSTEM_PROMPTS.md](./plan/SYSTEM_PROMPTS.md) → Understand agent behavior
3. Read [WEEK_BY_WEEK.md](./tasks/WEEK_BY_WEEK.md) → Pick tasks to implement
4. Start with Week 1 tasks (dataset construction)

### For Researchers

1. Read [ARCHITECTURE.md](./plan/ARCHITECTURE.md) → Understand methodology
2. Read [SYSTEM_PROMPTS.md](./plan/SYSTEM_PROMPTS.md) → Understand prompts
3. Focus on RL reward model design
4. Review statistical analysis plan in Week 5 tasks

---

## 🔑 Key Design Decisions

### 1. Main Agent + Concurrent Subagents

**Rationale**: Follows SWE-agent philosophy while leveraging Claude SDK's subagent feature for true concurrent A/B testing.

### 2. Four System Prompt Categories

**Rationale**: Separation of concerns - workflow, tool usage, strategy, and incentives are independent dimensions.

### 3. CLI-Only Interface (No Web)

**Rationale**: Simplifies implementation, faster iteration, aligns with developer workflow.

### 4. Session Export as Raw Data

**Rationale**: Claude SDK sessions provide complete conversation history for deep analysis.

### 5. Hooks for Metrics Collection

**Rationale**: Real-time metrics collection without manual parsing of agent output.

---

## 📊 Expected Outcomes

### Dataset

- 70 high-quality benchmark tasks
- 5 categories × 14 tasks each
- Manual ground truth verification
- ~30k LOC coverage of codex repository

### Agents

- Main orchestrator spawning concurrent subagents
- CS-hybrid with semantic search access
- Baseline with standard tools only
- Real-time metrics via hooks

### Results

- Statistical significance (p < 0.05)
- CS-hybrid wins >80% of tasks
- -60% tool calls on average
- +25% F1 score improvement
- Publication-ready benchmark report

---

## 🔗 Related Documents

### In Benchmarks Root

- [README.md](../README.md): Overview of benchmarks system
- [QUICK_START.md](../QUICK_START.md): How to run benchmarks

### External References

- **SWE-bench**: `/Users/arthur/dev-space/SWE-bench/`
- **SWE-agent**: `/Users/arthur/dev-space/SWE-agent/`
- **semtools**: `/Users/arthur/dev-space/semtools/benchmarks/`
- **Claude SDK**: `/Users/arthur/dev-space/claude-code-sdk-python/`
- **codex**: `/Users/arthur/dev-space/codex/` (dataset source)

---

## ✅ Implementation Checklist

### Phase 1: Dataset (Week 1-2)

- [ ] Setup dataset infrastructure
- [ ] AST-based task generation
- [ ] Manual task curation (70 tasks)
- [ ] Dataset validation

### Phase 2: Agents (Week 2-3)

- [ ] Create agent infrastructure
- [ ] Write 4 system prompts
- [ ] Implement 3 hooks
- [ ] Implement 2 subagents
- [ ] Implement main orchestrator

### Phase 3: CLI (Week 3-4)

- [ ] Non-REPL runner
- [ ] Session export
- [ ] REPL mode
- [ ] Slash commands

### Phase 4: Testing (Week 4)

- [ ] Unit tests
- [ ] Pilot study (10 tasks)
- [ ] Validation & refinement

### Phase 5: Evaluation (Week 5)

- [ ] Full benchmark run (70 tasks)
- [ ] Statistical analysis
- [ ] Qualitative analysis
- [ ] Final report

---

## 🚀 Getting Started

```shell
# Install dependencies
cd /Users/arthur/dev-space/semcs/benchmarks
uv sync

# Week 1: Start with dataset construction
python automation/dataset_builder.py --source /Users/arthur/dev-space/codex --output datasets/codex/

# Week 2: Test agent SDK integration
python agents/test_sdk_integration.py

# Week 3: Build and test CLI
python cli/bench_runner.py --dataset datasets/codex/ --max-tasks 3 --output results/test/

# Week 4: Run pilot study
python cli/bench_runner.py --dataset datasets/codex/ --max-tasks 10 --output results/pilot/

# Week 5: Full evaluation
python cli/bench_runner.py --dataset datasets/codex/ --output results/final_run/
```

---

## 📞 Questions?

For implementation questions, refer to:

1. [ARCHITECTURE.md](./plan/ARCHITECTURE.md) for design decisions
2. [SYSTEM_PROMPTS.md](./plan/SYSTEM_PROMPTS.md) for prompt content
3. [WEEK_BY_WEEK.md](./tasks/WEEK_BY_WEEK.md) for task details

For research questions, refer to:

- SWE-bench paper: <https://arxiv.org/abs/2310.06770>
- SWE-agent paper: <https://arxiv.org/abs/2405.15793>
- Claude SDK docs: <https://docs.claude.com/en/api/agent-sdk/>

---

**Status**: Design complete, ready for implementation 🎉

**Next Step**: Begin Week 1 Task 1.1 - Setup Dataset Infrastructure
