# Benchmarks 文档清理方案

## 📋 当前文档 (16 个 .md)

### 根目录 (8 个)
1. `README.md` - 主文档
2. `QUICK_START.md` - 快速开始
3. `QUICK_TEST_COMMANDS.md` - 命令参考
4. `ENHANCED_BENCHMARK_DESIGN.md` - 设计文档（详细）
5. `BENCHMARK_IMPLEMENTATION_SUMMARY.md` - 实现总结（详细）
6. `FILE_MANIFEST.md` - 文件清单
7. `OPTIMIZATION_PLAN.md` - 优化计划（开发文档）
8. `CLEANUP_SUMMARY.md` - 清理总结（开发文档）
9. `UV_MIGRATION_SUMMARY.md` - uv 迁移总结（开发文档）

### docs/ (3 个)
10. `docs/CODING_AGENT_INTEGRATION.md` - Agent 集成指南
11. `docs/CS_VS_GREP_ANALYSIS.md` - 对比分析
12. `docs/HUMAN_FRIENDLY_GUIDE.md` - 人类友好指南

### 子目录 (5 个)
13. `test_scenarios/README.md` - 演示脚本说明
14. `quantitative/agents/README.md` - 未实现说明
15. `quantitative/datasets/README.md` - 未实现说明
16. `quantitative/tasks/README.md` - 未实现说明
17. `real_world/datasets/README.md` - 未实现说明

---

## 🎯 清理方案

### 保留 (5 个核心文档)

#### 1. `README.md` ✅
**用途**: 主文档，测评系统总览
**内容**:
- 快速开始
- 测评方法说明
- 使用示例
- 结果解读

#### 2. `QUICK_START.md` ✅
**用途**: 中文快速开始指南
**内容**:
- 环境安装
- 运行测试
- 查看结果

#### 3-7. 子目录 README.md ✅
**用途**: 说明目录用途
- `test_scenarios/README.md`
- `quantitative/agents/README.md`
- `quantitative/datasets/README.md`
- `quantitative/tasks/README.md`
- `real_world/datasets/README.md`

### 归档到 docs/archive/ (9 个开发文档)

移动到 `docs/archive/` 保留历史记录：

- `ENHANCED_BENCHMARK_DESIGN.md` → `docs/archive/`
- `BENCHMARK_IMPLEMENTATION_SUMMARY.md` → `docs/archive/`
- `FILE_MANIFEST.md` → `docs/archive/`
- `OPTIMIZATION_PLAN.md` → `docs/archive/`
- `CLEANUP_SUMMARY.md` → `docs/archive/`
- `UV_MIGRATION_SUMMARY.md` → `docs/archive/`
- `QUICK_TEST_COMMANDS.md` → 合并到 README.md
- `docs/CODING_AGENT_INTEGRATION.md` → `docs/archive/`
- `docs/CS_VS_GREP_ANALYSIS.md` → `docs/archive/`
- `docs/HUMAN_FRIENDLY_GUIDE.md` → `docs/archive/`

### 删除 docs/ 目录
将 docs/ 合并到 docs/archive/，简化结构

---

## ✨ 清理后结构

```
benchmarks/
├── README.md                      # ✅ 主文档（简化，重点突出）
├── QUICK_START.md                 # ✅ 中文快速开始
├── pyproject.toml
├── .python-version
├── .gitignore
│
├── automation/
│   ├── test_runner.py
│   ├── quick_test.sh
│   └── results/.gitkeep
│
├── real_world/
│   ├── tasks/code_comprehension_tasks.yaml
│   ├── agents/{baseline,cs_hybrid}_agent.py
│   ├── datasets/README.md         # ✅ 简短说明
│   └── results/.gitkeep
│
├── quantitative/
│   ├── eval/metrics.py
│   ├── agents/README.md           # ✅ 简短说明
│   ├── tasks/README.md            # ✅ 简短说明
│   ├── datasets/README.md         # ✅ 简短说明
│   └── results/.gitkeep
│
├── test_scenarios/
│   ├── README.md                  # ✅ 简短说明
│   └── *.sh
│
└── docs/
    └── archive/                   # 🗄️ 历史文档归档
        ├── ENHANCED_BENCHMARK_DESIGN.md
        ├── BENCHMARK_IMPLEMENTATION_SUMMARY.md
        ├── FILE_MANIFEST.md
        ├── OPTIMIZATION_PLAN.md
        ├── CLEANUP_SUMMARY.md
        ├── UV_MIGRATION_SUMMARY.md
        ├── CODING_AGENT_INTEGRATION.md
        ├── CS_VS_GREP_ANALYSIS.md
        └── HUMAN_FRIENDLY_GUIDE.md
```

---

## 📝 README.md 简化内容

### 精简为 3 个核心部分

#### 1. 快速开始 (Quick Start)
```markdown
## 🚀 Quick Start

### One-command test
\`\`\`bash
cd benchmarks
./automation/quick_test.sh
\`\`\`

### Full benchmark
\`\`\`bash
uv sync
uv run python automation/test_runner.py --verbose
\`\`\`
```

#### 2. 测评方法 (Benchmark Methodology)
```markdown
## 📊 Benchmark Methodology

### A/B Testing Approach
- **Baseline**: grep/glob only
- **Treatment**: cs --hybrid

### 25 Real-World Tasks
- Simple search (5)
- Cross-file relationships (5)
- Architecture understanding (5)
- Refactoring prep (5)
- Multilingual (2)

### Metrics
- Tool call reduction: Target 70-75%
- Context token savings: Target 85%+
- Success rate, Precision, Recall
```

#### 3. 使用说明 (Usage Guide)
```markdown
## 📖 Usage

### Run by category
\`\`\`bash
uv run python automation/test_runner.py --category architecture -v
\`\`\`

### Run by difficulty
\`\`\`bash
uv run python automation/test_runner.py --difficulty easy -v
\`\`\`

### View results
\`\`\`bash
cat automation/results/summary_report.json | python -m json.tool
\`\`\`
```

---

## 🎯 执行步骤

1. **创建归档目录**
   ```bash
   mkdir -p docs/archive
   ```

2. **移动开发文档到归档**
   ```bash
   mv ENHANCED_BENCHMARK_DESIGN.md docs/archive/
   mv BENCHMARK_IMPLEMENTATION_SUMMARY.md docs/archive/
   mv FILE_MANIFEST.md docs/archive/
   mv OPTIMIZATION_PLAN.md docs/archive/
   mv CLEANUP_SUMMARY.md docs/archive/
   mv UV_MIGRATION_SUMMARY.md docs/archive/
   mv QUICK_TEST_COMMANDS.md docs/archive/
   mv docs/CODING_AGENT_INTEGRATION.md docs/archive/
   mv docs/CS_VS_GREP_ANALYSIS.md docs/archive/
   mv docs/HUMAN_FRIENDLY_GUIDE.md docs/archive/
   ```

3. **删除空的 docs/ 目录**
   ```bash
   rmdir docs  # 如果为空
   ```

4. **简化 README.md**
   - 删除过于详细的部分
   - 保留核心使用说明
   - 突出快速开始

5. **简化 QUICK_START.md**
   - 删除重复内容
   - 保留最常用命令
   - 添加故障排除

---

## 📊 清理前后对比

| 项目 | 清理前 | 清理后 |
|------|--------|--------|
| **根目录 .md** | 9 | 2 (README + QUICK_START) |
| **docs/ .md** | 3 | 0 |
| **子目录 .md** | 5 | 5 (保持不变) |
| **总文档数** | 17 | 7 (用户文档) + 10 (归档) |

---

## ✅ 优点

1. **用户友好**: 只看到需要的文档
2. **减少混淆**: 没有开发过程文档
3. **保留历史**: 归档在 docs/archive/
4. **快速上手**: README 更简洁明了

---

**准备执行?** 我可以立即执行这个清理方案。
