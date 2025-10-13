#!/bin/bash

# Quick demo script for cc (semantic grep)
# This script demonstrates different search modes and their capabilities

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR"

echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}    cc (Semantic Grep) Quick Demo${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Check if cc is available
if ! command -v cc &> /dev/null; then
    echo -e "${RED}Error: 'cc' command not found. Please install it first.${NC}"
    echo "Run: cargo install cc"
    exit 1
fi

echo -e "${YELLOW}First, let's index the examples for semantic/lexical/hybrid search:${NC}"
echo -e "${BLUE}$ cc --index $EXAMPLES_DIR${NC}"
cc --index "$EXAMPLES_DIR"
echo ""

sleep 1

echo -e "${PURPLE}=== 1. REGEX SEARCH (Default) ===${NC}"
echo -e "${YELLOW}Find 'error' using regex pattern matching:${NC}"
echo -e "${BLUE}$ cc 'error' $EXAMPLES_DIR${NC}"
cc "error" "$EXAMPLES_DIR" | head -10
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 2. SEMANTIC SEARCH ===${NC}"
echo -e "${YELLOW}Find 'error handling' using semantic understanding:${NC}"
echo -e "${BLUE}$ cc --sem 'error handling' $EXAMPLES_DIR${NC}"
cc --sem "error handling" "$EXAMPLES_DIR" | head -10
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 3. LEXICAL SEARCH (BM25) ===${NC}"
echo -e "${YELLOW}Find 'machine learning' using full-text search with ranking:${NC}"
echo -e "${BLUE}$ cc --lex 'machine learning' $EXAMPLES_DIR/text_samples${NC}"
cc --lex "machine learning" "$EXAMPLES_DIR/text_samples" | head -5
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 4. HYBRID SEARCH ===${NC}"
echo -e "${YELLOW}Find 'database connection' using combined approach:${NC}"
echo -e "${BLUE}$ cc --hybrid 'database connection' $EXAMPLES_DIR/code${NC}"
cc --hybrid "database connection" "$EXAMPLES_DIR/code" | head -8
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 5. FULL SECTION MODE ===${NC}"
echo -e "${YELLOW}Get entire functions/classes containing 'async':${NC}"
echo -e "${BLUE}$ cc --sem --full-section 'async operations' $EXAMPLES_DIR/code${NC}"
cc --sem --full-section "async operations" "$EXAMPLES_DIR/code" | head -15
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 6. SEMANTIC SEARCH WITH SCORES ===${NC}"
echo -e "${YELLOW}Show relevance scores for 'artificial intelligence':${NC}"
echo -e "${BLUE}$ cc --sem --scores --limit 5 'artificial intelligence' $EXAMPLES_DIR/text_samples${NC}"
cc --sem --scores --limit 5 "artificial intelligence" "$EXAMPLES_DIR/text_samples"
echo ""

sleep 2

echo -e "${PURPLE}=== 7. CONTEXT LINES ===${NC}"
echo -e "${YELLOW}Show surrounding context for 'class' matches:${NC}"
echo -e "${BLUE}$ cc -C 2 'class' $EXAMPLES_DIR/code${NC}"
cc -C 2 "class" "$EXAMPLES_DIR/code" | head -15
echo -e "${GREEN}...${NC}"
echo ""

sleep 2

echo -e "${PURPLE}=== 8. JSON OUTPUT ===${NC}"
echo -e "${YELLOW}Get machine-readable JSON output:${NC}"
echo -e "${BLUE}$ cc --json --sem --limit 3 'async function' $EXAMPLES_DIR/code${NC}"
cc --json --sem --limit 3 "async function" "$EXAMPLES_DIR/code" | head -20
echo -e "${GREEN}...${NC}"
echo ""

echo -e "${CYAN}========================================${NC}"
echo -e "${GREEN}Demo complete! Try these modes yourself:${NC}"
echo ""
echo -e "${YELLOW}Regex (exact patterns):${NC}     cc 'pattern' examples/"
echo -e "${YELLOW}Semantic (meaning):${NC}         cc --sem 'concept' examples/"
echo -e "${YELLOW}Lexical (full-text):${NC}        cc --lex 'phrase' examples/"
echo -e "${YELLOW}Hybrid (comprehensive):${NC}     cc --hybrid 'term' examples/"
echo ""
echo -e "${BLUE}Search in specific areas:${NC}"
echo -e "${YELLOW}Code examples:${NC}              cc --sem 'error handling' examples/code/"
echo -e "${YELLOW}Text samples:${NC}               cc --sem 'machine learning' examples/text_samples/"
echo ""
echo -e "${BLUE}Add --scores to see relevance, --full-section for complete blocks${NC}"
echo -e "${BLUE}Use -C N for context lines, --json for machine-readable output${NC}"
echo ""
echo -e "${CYAN}Happy searching! 🔍${NC}"