#!/bin/bash
#
# Quick test script for cs --hybrid benchmark system
# Runs a minimal test (first 3 easy tasks) to verify setup
#

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

echo "=================================================="
echo "CS --hybrid Benchmark Quick Test"
echo "=================================================="
echo ""
echo "This will run a quick test of the automated benchmark system"
echo "Testing: First 3 easy tasks (comp-001, comp-002, comp-004)"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

# Check if cs is installed
if ! command -v cs &> /dev/null; then
    echo "ERROR: cs command not found. Please install cs first."
    echo "  cd $REPO_ROOT && cargo install --path cs-cli"
    exit 1
fi

echo "✓ cs found: $(cs --version | head -1)"

# Check if index exists
if [ ! -d "$REPO_ROOT/.cs" ]; then
    echo "WARNING: .cs index not found. Building index..."
    cd "$REPO_ROOT"
    cs --index --model jina-v4 .
    echo "✓ Index built"
else
    echo "✓ Index exists"
fi

# Check if uv is available
if ! command -v uv &> /dev/null; then
    echo "ERROR: uv not found. Please install uv first:"
    echo "  curl -LsSf https://astral.sh/uv/install.sh | sh"
    echo ""
    echo "Or use Homebrew:"
    echo "  brew install uv"
    exit 1
fi

echo "✓ uv found: $(uv --version)"

# Setup Python environment with uv
echo ""
echo "Setting up Python environment..."
BENCHMARK_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
cd "$BENCHMARK_DIR"

# Sync dependencies (creates .venv if needed)
uv sync --quiet || {
    echo "WARNING: uv sync failed, trying with --no-cache..."
    uv sync --no-cache
}
echo "✓ Dependencies installed"

echo ""
echo "Running quick test (3 easy tasks)..."
echo "=================================================="
echo ""

cd "$SCRIPT_DIR"

# Run test with uv
uv run python test_runner.py \
    --repo "$REPO_ROOT" \
    --difficulty easy \
    --max-tasks 3 \
    --verbose

echo ""
echo "=================================================="
echo "Quick test complete!"
echo "=================================================="
echo ""
echo "Results saved to: $SCRIPT_DIR/results/"
echo ""
echo "View detailed results:"
echo "  cat $SCRIPT_DIR/results/detailed_results.json | python3 -m json.tool"
echo ""
echo "View summary:"
echo "  cat $SCRIPT_DIR/results/summary_report.json | python3 -m json.tool"
echo ""
echo "Next steps:"
echo "  - Run full benchmark: uv run python test_runner.py --verbose"
echo "  - Run by category: uv run python test_runner.py --category architecture --verbose"
echo "  - Run by difficulty: uv run python test_runner.py --difficulty hard --verbose"
echo ""
echo "Or activate the virtual environment:"
echo "  source $BENCHMARK_DIR/.venv/bin/activate"
echo "  python test_runner.py --verbose"
echo ""
