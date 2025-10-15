# Benchmarks 优化计划

基于你的三个优秀观察，这里是优化建议和实施计划。

---

## 📋 问题分析

### 1. ✅ requirements.txt 冗余
- **现状**: 有 requirements.txt 和 pyproject.toml 两套依赖管理
- **问题**: 容易不同步，维护成本高
- **解决**: ✅ 已将 requirements.txt 改为 deprecation notice

### 2. ⚠️ Shell 脚本混杂在 Python 体系中
- **现状**: 7 个 .sh 脚本（1 个工具 + 6 个演示）
- **问题**:
  - `test_scenarios/*.sh` 是演示脚本，主要用于生成人类可读的对比输出
  - `automation/quick_test.sh` 是用户入口，便于一键运行
- **是否需要改**: 取决于使用场景

### 3. 🔧 空目录问题
- **现状**: 12 个空目录
- **问题**: 有些是预留的未实现功能，有些是缺少数据
- **需要**: 清理 + 添加 .gitkeep + 补充实现

---

## 🎯 优化方案

### Option A: 激进方案（完全 Python 化）

**优点**: 统一技术栈，易于维护
**缺点**: 失去 Shell 脚本的简洁性和演示价值

```
benchmarks/
├── pyproject.toml              # 唯一依赖来源
├── automation/
│   ├── cli.py                  # 替代 quick_test.sh
│   └── scenario_runner.py      # 替代 test_scenarios/*.sh
└── 删除所有 .sh 文件
```

### Option B: 混合方案（推荐）✅

**优点**: 保留各自优势
**缺点**: 需要维护两种语言

```
benchmarks/
├── pyproject.toml              # Python 依赖
├── requirements.txt            # Deprecated notice
├── automation/
│   ├── quick_test.sh          # 保留：用户友好的入口
│   ├── test_runner.py         # Python 核心
│   └── cli.py                 # 新增：Python CLI 接口
└── test_scenarios/             # 保留：演示和文档价值
    ├── *.sh                   # 保留：生成可读对比输出
    └── README.md              # 新增：说明这些是演示脚本
```

### Option C: 清理方案（最小改动）

只清理空目录，保持现有结构。

---

## 📊 详细分析

### 1. Shell 脚本的角色

#### `automation/quick_test.sh` - 保留 ✅
**用途**: 用户入口，提供友好的 onboarding
**价值**:
- 自动检查依赖（cs, uv）
- 友好的错误提示
- 适合非技术用户

**替代方案**: 创建 Python CLI
```python
# benchmarks/automation/cli.py
import click

@click.command()
def quick_test():
    """Run quick benchmark test"""
    # 检查依赖
    # 运行测试
    # 显示结果
```

**建议**: 两者都保留
- `quick_test.sh` 用于快速上手
- Python CLI 用于程序化调用

#### `test_scenarios/*.sh` - 可选保留 🤔
**用途**: 演示脚本，生成对比输出
**价值**:
- 用于文档和博客
- 生成 markdown 友好的输出
- 展示实际使用场景

**问题**:
- 不是自动化测试的一部分
- 与 `automation/test_runner.py` 功能重复

**替代方案**:
```python
# automation/scenario_runner.py
def run_scenario_01_error_handling():
    """演示：错误处理审计"""
    print_header("Error Handling Audit")

    # Baseline
    print_section("Traditional grep (8 calls)")
    run_grep_baseline()

    # CS Hybrid
    print_section("cs --hybrid (1 call)")
    run_cs_hybrid()

    # Comparison
    print_comparison()
```

**建议**:
- **短期**: 保留 .sh 脚本作为演示
- **长期**: 迁移到 Python + rich 库美化输出

### 2. 空目录分析

| 目录 | 状态 | 用途 | 建议 |
|------|------|------|------|
| `automation/results/` | ✅ 必要 | test_runner.py 输出 | 添加 .gitkeep |
| `real_world/results/` | ✅ 必要 | agent 输出 | 添加 .gitkeep |
| `quantitative/results/` | ✅ 必要 | 量化评估输出 | 添加 .gitkeep |
| `automation/agents/` | ❌ 冗余 | 重复 real_world/agents | **删除** |
| `automation/datasets/` | ❌ 冗余 | 重复其他 datasets | **删除** |
| `automation/eval/` | ❌ 冗余 | 重复 quantitative/eval | **删除** |
| `automation/tasks/` | ❌ 冗余 | 重复 real_world/tasks | **删除** |
| `quantitative/agents/` | ⏸️ 未实现 | 预留量化测试 agent | 添加 README 说明 |
| `quantitative/datasets/` | ⏸️ 未实现 | 预留 CodeSearchNet 等数据集 | 添加 README 说明 |
| `quantitative/tasks/` | ⏸️ 未实现 | 预留量化任务定义 | 添加 README 说明 |
| `real_world/datasets/` | ⏸️ 未实现 | 预留外部代码库 | 添加 README 说明 |
| `real_world/eval/` | ❌ 冗余 | 评估在 test_runner 中 | **删除** |

---

## 🚀 实施建议

### Phase 1: 立即清理（5分钟）

```bash
cd /Users/arthur/dev-space/semcs/benchmarks

# 1. 删除冗余空目录
rm -rf automation/agents automation/datasets automation/eval automation/tasks
rm -rf real_world/eval

# 2. 为必要的空目录添加 .gitkeep
touch automation/results/.gitkeep
touch real_world/results/.gitkeep
touch quantitative/results/.gitkeep

# 3. 为未实现的目录添加 README
echo "# Quantitative Datasets (Future)" > quantitative/datasets/README.md
echo "# Quantitative Tasks (Future)" > quantitative/tasks/README.md
echo "# Real-world Test Repositories (Future)" > real_world/datasets/README.md
echo "# Quantitative Test Agents (Future)" > quantitative/agents/README.md
```

### Phase 2: 添加 Python CLI（30分钟）

创建 `automation/cli.py`:

```python
#!/usr/bin/env python3
"""
CS Benchmark CLI - Python interface for running benchmarks
"""
import click
import subprocess
import sys
from pathlib import Path

@click.group()
def cli():
    """CS Benchmark Suite - Automated testing for cs --hybrid"""
    pass

@cli.command()
@click.option('--max-tasks', type=int, default=3, help='Max tasks to run')
@click.option('--verbose', '-v', is_flag=True, help='Verbose output')
def quick(max_tasks, verbose):
    """Quick test (3 easy tasks by default)"""
    from test_runner import TestRunner

    # Check prerequisites
    if not check_cs_installed():
        click.echo("❌ cs not installed", err=True)
        sys.exit(1)

    # Run test
    repo = Path(__file__).parent.parent.parent
    runner = TestRunner(repo, verbose=verbose)
    # ... run tests

@cli.command()
@click.option('--category', help='Filter by category')
@click.option('--difficulty', help='Filter by difficulty')
@click.option('--max-tasks', type=int, help='Max tasks to run')
@click.option('--verbose', '-v', is_flag=True)
def run(category, difficulty, max_tasks, verbose):
    """Run full benchmark"""
    # ... implementation

@cli.command()
def scenarios():
    """Run demonstration scenarios (replaces .sh scripts)"""
    click.echo("Running demonstration scenarios...")
    # Implementation

if __name__ == '__main__':
    cli()
```

添加到 pyproject.toml:
```toml
[project.scripts]
cs-benchmark = "automation.cli:cli"
```

使用:
```bash
uv run cs-benchmark quick
uv run cs-benchmark run --category architecture
uv run cs-benchmark scenarios
```

### Phase 3: 迁移演示脚本（可选，1-2小时）

使用 `rich` 库美化输出:

```python
from rich.console import Console
from rich.table import Table
from rich.panel import Panel

console = Console()

def run_scenario_01():
    console.print(Panel("[bold]Test Scenario 1: Error Handling Audit[/bold]"))

    # Traditional approach
    table = Table(title="Traditional Approach (8 grep calls)")
    table.add_column("Step", style="cyan")
    table.add_column("Command", style="yellow")
    table.add_column("Results", style="green")

    table.add_row("1", "grep -r 'Result<'", "1,234 matches")
    # ...

    console.print(table)

    # CS Hybrid approach
    console.print("\n[bold green]✓ CS Hybrid (1 call)[/bold green]")
    # ...
```

---

## 🎯 推荐方案

### 立即执行（今天）

1. ✅ **已完成**: requirements.txt → deprecation notice
2. 🧹 **清理空目录**: 删除冗余，添加 .gitkeep
3. 📝 **添加说明**: 为未实现目录添加 README

### 短期（本周）

4. 🐍 **添加 Python CLI**: automation/cli.py
5. 📦 **更新 pyproject.toml**: 添加 CLI 入口点
6. 📖 **更新文档**: 说明 Shell vs Python 使用场景

### 长期（可选）

7. 🎨 **迁移演示脚本**: test_scenarios/*.sh → Python + rich
8. 🧪 **实现量化测试**: quantitative/ 完整实现
9. 📊 **数据集准备**: 下载 CodeSearchNet 等数据集

---

## 📝 文件结构优化后

```
benchmarks/
├── pyproject.toml                    # 唯一依赖定义 ✅
├── requirements.txt                  # Deprecation notice ✅
├── .python-version                   # Python 3.11
├── .gitignore                        # Python artifacts
│
├── automation/
│   ├── cli.py                       # NEW: Python CLI 入口
│   ├── test_runner.py               # A/B 测试核心
│   ├── quick_test.sh                # 保留：用户友好入口
│   └── results/                     # 测试结果
│       └── .gitkeep                 # NEW
│
├── real_world/
│   ├── tasks/
│   │   └── code_comprehension_tasks.yaml
│   ├── agents/
│   │   ├── baseline_agent.py
│   │   └── cs_hybrid_agent.py
│   ├── results/
│   │   └── .gitkeep                 # NEW
│   └── datasets/
│       └── README.md                # NEW: 说明未来用途
│
├── quantitative/
│   ├── eval/
│   │   └── metrics.py
│   ├── agents/
│   │   └── README.md                # NEW: 说明未来用途
│   ├── tasks/
│   │   └── README.md                # NEW
│   ├── datasets/
│   │   └── README.md                # NEW
│   └── results/
│       └── .gitkeep                 # NEW
│
├── test_scenarios/                   # 可选保留
│   ├── README.md                    # NEW: 说明这些是演示脚本
│   ├── 01_error_handling_audit.sh   # 演示：错误处理
│   ├── 02_config_system_trace.sh    # 演示：配置追踪
│   ├── 03_api_integration_locate.sh # 演示：API 定位
│   ├── 04_cross_language_refactor.sh # 演示：跨语言
│   ├── 05_recursive_navigation.sh   # 演示：递归导航
│   └── run_all_scenarios.sh         # 运行所有演示
│
└── docs/                            # 保持不变
    ├── README.md
    ├── QUICK_START.md
    ├── QUICK_TEST_COMMANDS.md
    └── ...
```

---

## ❓ 决策点

需要你确认：

### Q1: Shell 演示脚本怎么处理？

- [ ] **A**: 全部保留（简单，但技术栈混杂）
- [ ] **B**: 迁移到 Python + rich（统一，但需要时间）
- [ ] **C**: 只保留 quick_test.sh，删除演示脚本（最激进）

**我的建议**: B（长期）或 A（短期过渡）

### Q2: 空目录清理策略？

- [x] **推荐**: 删除冗余 + 为必要目录添加 .gitkeep + 为未来目录添加 README
- [ ] **保守**: 全部保留，只添加 README
- [ ] **激进**: 删除所有空目录，需要时再创建

### Q3: CLI 接口需要吗？

- [ ] **Yes**: 创建 automation/cli.py，统一 Python 入口
- [ ] **No**: 保持现状，只用 quick_test.sh + test_runner.py

**我的建议**: Yes，提供更好的编程接口

---

## 🎬 立即执行的命令

如果你同意上述建议，运行：

```bash
cd /Users/arthur/dev-space/semcs/benchmarks

# 清理冗余空目录
rm -rf automation/agents automation/datasets automation/eval automation/tasks real_world/eval

# 添加 .gitkeep
touch automation/results/.gitkeep real_world/results/.gitkeep quantitative/results/.gitkeep

# 添加未来目录的说明
cat > quantitative/datasets/README.md << 'EOF'
# Quantitative Datasets (Future Implementation)

This directory is reserved for standard code search datasets:

- CodeSearchNet extended format
- Custom code retrieval benchmarks
- Ground truth data for quantitative evaluation

**Status**: Not yet implemented
**Priority**: Medium (Phase 2 expansion)
EOF

cat > quantitative/tasks/README.md << 'EOF'
# Quantitative Tasks (Future Implementation)

This directory will contain:

- nl2code task definitions (natural language → code)
- code2code task definitions (code → related code)
- QA task definitions (question answering on code)

**Status**: Not yet implemented
**Priority**: Medium (Phase 2 expansion)
EOF

cat > real_world/datasets/README.md << 'EOF'
# Real-World Test Repositories (Future)

This directory is reserved for external codebases used in testing:

- rust-analyzer (large Rust codebase)
- tokio (async Rust runtime)
- typescript (TypeScript compiler)
- etc.

**Status**: Not yet implemented
**Priority**: Low (Phase 3 expansion)
EOF

cat > quantitative/agents/README.md << 'EOF'
# Quantitative Test Agents (Future)

Agent implementations for quantitative benchmarks:

- Baseline agents for standard datasets
- Comparison with ast-grep, ripgrep, etc.

**Status**: Not yet implemented
**Priority**: Medium (Phase 2 expansion)
EOF

cat > test_scenarios/README.md << 'EOF'
# Test Scenarios - Demonstration Scripts

These shell scripts are **demonstration tools**, not automated tests.

## Purpose

- Generate human-readable comparison output
- Used in documentation and blog posts
- Show real-world usage examples
- Compare cs --hybrid vs traditional grep/glob

## Usage

```bash
# Run individual scenario
./01_error_handling_audit.sh

# Run all scenarios
./run_all_scenarios.sh
```

## Automated Testing

For automated benchmarking, use the Python test runner:

```bash
cd ../automation
uv run python test_runner.py --verbose
```

## Future

These may be migrated to Python + rich library for better integration.
EOF

echo "✅ Cleanup complete!"
```

---

**你的选择？** 我可以立即执行上述清理，或者根据你的偏好调整方案。
