# Quick Start Guide - CS Benchmark Suite

One-command setup and benchmark execution.

## 🚀 Quick Start (Recommended)

### 1. Install uv (if not already installed)

```shell
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or use Homebrew
brew install uv

# Or use Cargo
cargo install --git https://github.com/astral-sh/uv uv
```

### 2. Run Quick Test (One Command)

```shell
cd /Users/arthur/dev-space/semcs/benchmarks
./automation/quick_test.sh
```

This script will automatically:

- ✅ Check if `cs` is installed
- ✅ Check if index exists (build if needed)
- ✅ Check if `uv` is installed
- ✅ Create Python virtual environment (`.venv/`)
- ✅ Install all dependencies (`PyYAML`)
- ✅ Run 3 easy tasks
- ✅ Generate JSON result reports

**Expected time**: 2-3 minutes (first run)

---

## 📦 Environment Management (Using uv)

### Create Environment

```shell
cd /Users/arthur/dev-space/semcs/benchmarks

# Create virtual environment and install dependencies
uv sync
```

This will:

1. Create `.venv/` virtual environment
2. Install all dependencies from `pyproject.toml`
3. Generate `uv.lock` lockfile (ensures reproducibility)

### Activate Environment

```bash
# Activate virtual environment
source .venv/bin/activate

# Now you can use python directly
python automation/test_runner.py --help
```

### Run Without Activation

```bash
# Use uv run directly (recommended)
uv run python automation/test_runner.py --verbose
```

---

## 🔬 Running Benchmarks

### Quick Test (3 easy tasks)

```shell
# Use quick test script (recommended for beginners)
./automation/quick_test.sh

# Or run manually
uv run python automation/test_runner.py --difficulty easy --max-tasks 3 --verbose
```

### Full Benchmark (25 tasks)

```shell
cd automation
uv run python test_runner.py --verbose
```

**Expected time**: 5-10 minutes

### Run by Category

```shell
# Simple search tasks
uv run python automation/test_runner.py --category simple_search --verbose

# Architecture understanding tasks (gradient descent navigation)
uv run python automation/test_runner.py --category architecture --verbose

# Cross-file relationship tracing
uv run python automation/test_runner.py --category cross_file --verbose

# Refactoring preparation tasks
uv run python automation/test_runner.py --category refactoring --verbose

# Multilingual understanding tasks
uv run python automation/test_runner.py --category multilingual --verbose
```

### Run by Difficulty

```shell
# Easy tasks (quick validation)
uv run python automation/test_runner.py --difficulty easy --verbose

# Medium difficulty
uv run python automation/test_runner.py --difficulty medium --verbose

# Hard tasks
uv run python automation/test_runner.py --difficulty hard --verbose

# Very hard (gradient descent)
uv run python automation/test_runner.py --difficulty very_hard --verbose
```

---

## 📊 Viewing Results

After testing, results are saved in `automation/results/`:

### View Summary Report

```shell
cat automation/results/summary_report.json | python -m json.tool
```

**Example output**:

```json
{
  "total_tasks": 25,
  "overall_improvements": {
    "avg_call_reduction_pct": 73.5,
    "avg_token_reduction_pct": 86.2
  },
  "success_rates": {
    "baseline": 0.64,
    "cs_hybrid": 0.92,
    "improvement": 0.28
  }
}
```

### View Detailed Results

```shell
cat automation/results/detailed_results.json | python -m json.tool | less
```

**Each task includes**:

- Baseline metrics (grep/glob): calls, tokens, duration, precision, recall
- CS Hybrid metrics (cs --hybrid): calls, tokens, duration, precision, recall
- Improvement percentages: call reduction, token reduction, time reduction

---

## 🛠️ Dependency Management

### View Installed Packages

```shell
uv pip list
```

### Add New Dependencies

```shell
# Add to pyproject.toml
uv add numpy pandas matplotlib

# Or add dev dependencies
uv add --dev pytest pytest-cov
```

### Update Dependencies

```shell
# Update all dependencies
uv sync --upgrade

# Update specific package
uv pip install --upgrade pyyaml
```

---

## 🧪 Development Workflow

### 1. Install Development Dependencies

```shell
# Install including dev tools (pytest, ruff)
uv sync --all-extras
```

### 2. Code Formatting and Linting

```shell
# Run linter
uv run ruff check .

# Auto-fix issues
uv run ruff check --fix .

# Format code
uv run ruff format .
```

### 3. Run Tests (Future)

```shell
uv run pytest
```

---

## 📝 Common Tasks

### Create Clean Environment

```shell
# Remove old environment
rm -rf .venv uv.lock

# Recreate
uv sync
```

### Use in CI/CD

```yaml
# .github/workflows/benchmark.yml
- name: Set up uv
  run: curl -LsSf https://astral.sh/uv/install.sh | sh

- name: Install dependencies
  run: uv sync

- name: Run benchmark
  run: uv run python automation/test_runner.py --max-tasks 10
```

### Check Environment Info

```shell
# uv version
uv --version

# Python version
uv run python --version

# Installed packages
uv pip list

# Project info
uv run python -c "import sys; print(sys.executable)"
```

---

## 🎯 Expected Results

After successful run, you should see:

```text
==================================================
BENCHMARK SUMMARY REPORT
==================================================

Total tasks evaluated: 25

--- Overall Improvements (cs --hybrid vs grep/glob) ---
  Average tool call reduction: 73.5%
  Median tool call reduction: 75.2%
  Average token reduction: 86.2%
  Median token reduction: 87.1%
  Average time reduction: 62.3%

--- Success Rates ---
  Baseline (grep/glob): 64.0%
  CS Hybrid: 92.0%
  Improvement: +28.0%

--- Precision & Recall ---
  Baseline precision: 52.3%
  CS Hybrid precision: 76.8%
  Precision improvement: +24.5%
  Baseline recall: 61.2%
  CS Hybrid recall: 86.4%
  Recall improvement: +25.2%
```

---

## ⚠️ Troubleshooting

### uv command not found

```shell
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Reload shell
source ~/.bashrc  # or ~/.zshrc
```

### cs command not found

```shell
# Install cs from project root
cd /Users/arthur/dev-space/semcs
cargo install --path cs-cli
```

### Index does not exist

```shell
# Build index
cd /Users/arthur/dev-space/semcs
cs --index --model jina-v4 .
```

### Python version incompatible

```shell
# Check Python version (requires >= 3.9)
python3 --version

# Use specific Python version with uv
uv venv --python 3.11
```

### Dependency installation failed

```shell
# Clear cache and retry
uv sync --no-cache

# Or install manually
uv pip install pyyaml
```

---

## 📚 Documentation

- **Main README**: [README.md](README.md)
- **Archived docs**: [docs/archive/](docs/archive/)

---

## 🎓 Next Steps

1. ✅ Run quick test to verify setup
2. 📊 Review results to understand improvements
3. 🔬 Run full benchmark
4. 📈 Analyze results by category and difficulty
5. 🚀 Integrate cs --hybrid into your Coding Agent

---

**Quick start**: `./automation/quick_test.sh`

**Full benchmark**: `uv run python automation/test_runner.py --verbose`

**Report issues**: <https://github.com/lwyBZss8924d/semcs/issues>
