#!/bin/bash

# Demo script showing the --full-section feature

echo "============================================"
echo "  ck --full-section Feature Demo"
echo "============================================"
echo ""

DEMO_FILE="examples/full_section_demo.py"

echo "1. Regular search for 'error handling' (shows only matching line):"
echo "   $ ck 'error handling' $DEMO_FILE"
echo ""
ck 'error handling' $DEMO_FILE | head -5
echo ""

echo "----------------------------------------"
echo ""

echo "2. With --full-section flag (shows complete function):"
echo "   $ ck --full-section 'error handling' $DEMO_FILE"
echo ""
ck --full-section 'error handling' $DEMO_FILE | head -30
echo ""

echo "----------------------------------------"
echo ""

echo "3. Semantic search without --full-section:"
echo "   $ ck --sem 'retry logic' examples/"
echo ""
ck --sem 'retry logic' examples/ | head -10
echo ""

echo "----------------------------------------"
echo ""

echo "4. Semantic search WITH --full-section:"
echo "   $ ck --sem --full-section 'retry logic' examples/"
echo ""
ck --sem --full-section 'retry logic' examples/ | head -40
echo ""

echo "============================================"
echo "  Key Benefits:"
echo "============================================"
echo ""
echo "• Regular search: Shows just the matching line with optional context"
echo "• --full-section: Returns the entire function/class containing the match"
echo "• Works with both regex and semantic search"
echo "• Especially useful for understanding complete code logic"
echo "• Perfect for AI agents that need full context"
echo ""
echo "Note: --full-section uses tree-sitter to identify code boundaries"
echo "Supported languages: Python, JavaScript, TypeScript (more coming soon)"