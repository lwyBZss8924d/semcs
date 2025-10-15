# Test Scenarios - Demonstration Scripts

这些 Shell 脚本是**演示工具**，不是自动化测试。

## 🎯 目的

- 生成人类可读的对比输出（适合文档和博客）
- 展示真实使用场景
- 对比 cs --hybrid vs 传统 grep/glob 工具
- 提供可复制的示例

## 📋 场景列表

| 脚本 | 场景 | 对比 | 改进 |
|------|------|------|------|
| `01_error_handling_audit.sh` | 错误处理审计 | 1 call vs 8 calls | 87.5% ↓ |
| `02_config_system_trace.sh` | 配置系统追踪 | 3 calls vs 12 calls | 75% ↓ |
| `03_api_integration_locate.sh` | API 集成定位 | 2 calls vs 10 calls | 80% ↓ |
| `04_cross_language_refactor.sh` | 跨语言重构 | 4 calls vs 15 calls | 73% ↓ |
| `05_recursive_navigation.sh` | 递归导航 | 6 calls vs 20+ calls | 70% ↓ |

## 🚀 使用方法

### 运行单个场景

```bash
cd /Users/arthur/dev-space/semcs/benchmarks/test_scenarios

# 场景 1: 错误处理审计
./01_error_handling_audit.sh

# 场景 2: 配置系统追踪
./02_config_system_trace.sh

# 场景 5: 递归导航（梯度下降）
./05_recursive_navigation.sh
```

### 运行所有场景

```bash
./run_all_scenarios.sh
```

输出保存到: `../results/scenario_*.txt`

## 📊 输出示例

```
╔════════════════════════════════════════════════════════════════════════╗
║  Test Scenario 1: Error Handling Audit                                 ║
╚════════════════════════════════════════════════════════════════════════╝

📍 Objective: Coding Agent needs to audit all error handling patterns

─────────────────────────────────────────────────────────────────────────
🔹 Traditional Approach (grep/glob)
─────────────────────────────────────────────────────────────────────────

❌ Requires 8 tool calls:
   1. grep -r 'Result<' . --include='*.rs'      → 1,234 matches
   2. grep -r '?' . --include='*.rs'            → 8,456 matches
   ...

─────────────────────────────────────────────────────────────────────────
✨ CS Hybrid Approach
─────────────────────────────────────────────────────────────────────────

✓ Single semantic + lexical + AST query:
   cs --hybrid "error handling Result anyhow patterns" . \
      --topk 15 --rerank --scores

Results: 15 highly relevant matches
Precision: ~90%
```

## 🤖 自动化测试

这些演示脚本**不是**自动化测试系统的一部分。

对于自动化基准测试，请使用 Python 测试运行器：

```bash
cd /Users/arthur/dev-space/semcs/benchmarks

# 快速测试（3 个简单任务）
./automation/quick_test.sh

# 完整测试（25 个任务）
uv run python automation/test_runner.py --verbose

# 按类别测试
uv run python automation/test_runner.py --category architecture -v
```

## 🔄 与自动化测试的区别

| 方面 | 演示脚本 (test_scenarios/) | 自动化测试 (automation/) |
|------|---------------------------|-------------------------|
| **目的** | 人类可读的演示 | 机器可执行的评估 |
| **输出** | 美化的文本输出 | JSON 结果 + 统计指标 |
| **运行** | 手动运行查看 | CI/CD 自动运行 |
| **指标** | 定性对比 | 定量指标（P@k, R@k, MRR） |
| **用途** | 文档、博客、演示 | 性能验证、回归测试 |

## 📚 使用场景

### 1. 文档和博客
```bash
# 生成演示输出用于文档
./01_error_handling_audit.sh > demo_output.txt

# 截图或复制输出到 README/博客
```

### 2. 用户演示
```bash
# 向新用户展示 cs --hybrid 的优势
./run_all_scenarios.sh

# 观看实时对比
```

### 3. 快速验证
```bash
# 快速验证 cs --hybrid 是否正常工作
./03_api_integration_locate.sh
```

## 🔮 未来计划

这些 Shell 脚本可能会迁移到 Python + rich 库：

**优点**:
- 统一技术栈（纯 Python）
- 更好的可维护性
- 更丰富的输出格式（颜色、表格、进度条）
- 更容易集成到自动化系统

**示例（未来）**:
```python
from rich.console import Console
from rich.table import Table

console = Console()

def run_scenario_01():
    console.print("[bold]Scenario 1: Error Handling Audit[/bold]")

    # Traditional approach
    with console.status("Running grep baseline..."):
        baseline_results = run_grep_baseline()

    # CS Hybrid
    with console.status("Running cs --hybrid..."):
        cs_results = run_cs_hybrid()

    # Comparison table
    table = Table(title="Results Comparison")
    table.add_row("Baseline", str(baseline_results))
    table.add_row("CS Hybrid", str(cs_results))
    console.print(table)
```

## 📖 相关文档

- **自动化测试**: [../automation/README.md](../automation/) - Python 自动化测试系统
- **快速开始**: [../QUICK_START.md](../QUICK_START.md) - 完整中文指南
- **实现总结**: [../BENCHMARK_IMPLEMENTATION_SUMMARY.md](../BENCHMARK_IMPLEMENTATION_SUMMARY.md)

---

**总结**: 这些是演示工具，不是测试工具。用于展示和文档，不用于自动化评估。
