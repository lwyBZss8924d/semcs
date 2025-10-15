# Quick Test Commands Reference

快速测试命令参考 - 适合日常使用

## 🚀 最快开始

```bash
cd /Users/arthur/dev-space/semcs/benchmarks
./automation/quick_test.sh
```

---

## 📦 环境管理

### 首次设置

```bash
# 安装 uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# 创建环境并安装依赖
cd /Users/arthur/dev-space/semcs/benchmarks
uv sync
```

### 查看环境信息

```shell
uv --version              # uv 版本
uv run python --version   # Python 版本
uv pip list               # 已安装包
```

### 更新依赖

```shell
uv sync --upgrade         # 更新所有依赖
uv add pyyaml             # 添加新依赖
```

---

## 🧪 运行测试

### 快速测试（推荐入门）

```shell
# 自动化快速测试（3 个简单任务）
./automation/quick_test.sh

# 或手动运行
uv run python automation/test_runner.py --difficulty easy --max-tasks 3 --verbose
```

### 完整测试（25 个任务）

```shell
uv run python automation/test_runner.py --verbose
```

### 按类别测试

```shell
# 简单搜索（5 tasks）
uv run python automation/test_runner.py --category simple_search -v

# 跨文件关系（5 tasks）
uv run python automation/test_runner.py --category cross_file -v

# 架构理解（5 tasks，梯度下降）
uv run python automation/test_runner.py --category architecture -v

# 重构准备（5 tasks）
uv run python automation/test_runner.py --category refactoring -v

# 多语言理解（2 tasks）
uv run python automation/test_runner.py --category multilingual -v
```

### 按难度测试

```shell
# 简单任务
uv run python automation/test_runner.py --difficulty easy -v

# 中等难度
uv run python automation/test_runner.py --difficulty medium -v

# 困难任务
uv run python automation/test_runner.py --difficulty hard -v

# 非常困难（梯度下降导航）
uv run python automation/test_runner.py --difficulty very_hard -v
```

### 限制任务数量

```shell
# 前 5 个任务
uv run python automation/test_runner.py --max-tasks 5 -v

# 前 10 个任务
uv run python automation/test_runner.py --max-tasks 10 -v
```

---

## 📊 查看结果

### 查看摘要

```shell
# JSON 格式
cat automation/results/summary_report.json | python -m json.tool

# 或使用 jq（如果安装了）
cat automation/results/summary_report.json | jq .
```

### 查看详细结果

```shell
# 所有任务详细结果
cat automation/results/detailed_results.json | python -m json.tool | less

# 只看某个任务
cat automation/results/detailed_results.json | jq '.[] | select(.task_id == "comp-001")'
```

### 提取关键指标

```shell
# 平均调用减少百分比
cat automation/results/summary_report.json | jq '.overall_improvements.avg_call_reduction_pct'

# 平均 token 减少百分比
cat automation/results/summary_report.json | jq '.overall_improvements.avg_token_reduction_pct'

# 成功率对比
cat automation/results/summary_report.json | jq '.success_rates'
```

---

## 🔧 常用组合

### 开发测试（快速验证）

```shell
# 只测试简单任务，限制 3 个
uv run python automation/test_runner.py \
    --difficulty easy \
    --max-tasks 3 \
    --verbose
```

### 完整评估（发布前）

```shell
# 所有任务，详细输出
uv run python automation/test_runner.py --verbose > benchmark_run.log 2>&1
```

### 特定类别深度测试

```shell
# 测试架构理解任务（最能体现梯度下降优势）
uv run python automation/test_runner.py \
    --category architecture \
    --verbose
```

### 性能压力测试

```shell
# 测试最难的任务
uv run python automation/test_runner.py \
    --difficulty very_hard \
    --verbose
```

---

## 🎯 目标验证命令

检查是否达到设计目标：

```shell
# 目标 1: 调用减少 ≥70%
cat automation/results/summary_report.json | \
  jq '.overall_improvements.avg_call_reduction_pct >= 70'

# 目标 2: Token 减少 ≥85%
cat automation/results/summary_report.json | \
  jq '.overall_improvements.avg_token_reduction_pct >= 85'

# 目标 3: CS Hybrid 成功率 ≥85%
cat automation/results/summary_report.json | \
  jq '.success_rates.cs_hybrid >= 0.85'

# 全部检查
cat automation/results/summary_report.json | jq '{
  call_reduction_ok: (.overall_improvements.avg_call_reduction_pct >= 70),
  token_reduction_ok: (.overall_improvements.avg_token_reduction_pct >= 85),
  success_rate_ok: (.success_rates.cs_hybrid >= 0.85)
}'
```

---

## 🐛 故障排除命令

### 检查环境

```shell
# 检查 cs 是否可用
cs --version

# 检查索引是否存在
ls -la /Users/arthur/dev-space/semcs/.cs/

# 检查 Python 环境
uv run python -c "import yaml; print('PyYAML OK')"
```

### 清理和重建

```shell
# 清理 Python 环境
rm -rf .venv uv.lock

# 重新创建
uv sync

# 清理测试结果
rm -rf automation/results/*.json
```

### 重建索引

```shell
cd /Users/arthur/dev-space/semcs
cs --index --model jina-v4 .
```

---

## 📝 快捷别名（可选）

添加到 `~/.bashrc` 或 `~/.zshrc`：

```shell
# 进入 benchmark 目录
alias cdb='cd /Users/arthur/dev-space/semcs/benchmarks'

# 快速测试
alias cstest='cd /Users/arthur/dev-space/semcs/benchmarks && ./automation/quick_test.sh'

# 完整测试
alias csfull='cd /Users/arthur/dev-space/semcs/benchmarks && uv run python automation/test_runner.py --verbose'

# 查看结果
alias csresults='cat /Users/arthur/dev-space/semcs/benchmarks/automation/results/summary_report.json | python -m json.tool'
```

使用：

```shell
cdb          # 进入目录
cstest       # 运行快速测试
csresults    # 查看结果
```

---

## 🔄 CI/CD 命令

适合在 GitHub Actions 或其他 CI 中使用：

```shell
# 完整流程（无交互）
cd benchmarks && \
  uv sync && \
  uv run python automation/test_runner.py --max-tasks 10 && \
  cat automation/results/summary_report.json
```

---

## ⏱️ 预计运行时间

| 命令 | 任务数 | 预计时间 |
|------|--------|---------|
| `quick_test.sh` | 3 | 1-2 分钟 |
| `--max-tasks 5` | 5 | 2-3 分钟 |
| `--difficulty easy` | ~8 | 3-4 分钟 |
| `--category simple_search` | 5 | 2-3 分钟 |
| `--category architecture` | 5 | 3-5 分钟 |
| 完整测试（25 tasks） | 25 | 8-12 分钟 |

---

## 📖 相关文档

- 📘 完整文档: [README.md](README.md)
- 🚀 中文快速开始: [QUICK_START.md](QUICK_START.md)
- 📐 设计文档: [ENHANCED_BENCHMARK_DESIGN.md](ENHANCED_BENCHMARK_DESIGN.md)
- 📊 实现总结: [BENCHMARK_IMPLEMENTATION_SUMMARY.md](BENCHMARK_IMPLEMENTATION_SUMMARY.md)

---

**最常用命令**:

```shell
./automation/quick_test.sh                                    # 快速测试
uv run python automation/test_runner.py --verbose             # 完整测试
cat automation/results/summary_report.json | python -m json.tool  # 查看结果
```
