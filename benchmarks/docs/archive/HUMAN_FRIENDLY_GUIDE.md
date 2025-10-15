# cs --hybrid: Human-Friendly Guide

## 🎯 Why cs is Easier Than grep

**Problem with grep/glob:**
```bash
# Complex, hard to remember
grep -r "pattern" . --include="*.rs" --exclude-dir={target,node_modules} -i -n | head -20

# Need to learn regex
grep -r "impl.*Config.*{" . --include="*.rs"

# Different tools, different syntax
rg "pattern" -t rust -g '!target/*'
fd -e rs -x grep -H "pattern"
```

**With cs --hybrid:**
```bash
# Simple, natural language
cs --hybrid "what you want to find" .

# That's it!
```

---

## 🚀 Quick Start

### Installation

```bash
# Install cs
curl -sSL https://get-cs.sh | bash

# Or via cargo
cargo install cs-cli

# Verify installation
cs --version
```

### Your First Search

```bash
# Search for error handling code
cs --hybrid "error handling" .

# Search in Chinese
cs --hybrid "错误处理" .

# Mix languages
cs --hybrid "error handling 错误处理" .
```

That's all you need to get started!

---

## 💡 Common Use Cases

### 1. Find Functions

**Old way (grep):**
```bash
grep -r "fn .*error" . --include="*.rs"  # Misses many patterns
grep -r "function.*error" . --include="*.js"  # Different per language
```

**New way (cs):**
```bash
# Works in any language!
cs --hybrid "error handling function" .

# Or with AST pattern
cs --hybrid "function handles errors" .
```

### 2. Find Configuration

**Old way:**
```bash
grep -r "config" . --include="*.rs" | wc -l
# Output: 2,341 matches (too many!)

grep -r "struct.*Config" . --include="*.rs"
grep -r "load.*config" . -i --include="*.rs"
# ... manual filtering ...
```

**New way:**
```bash
cs --hybrid "configuration system" . --topk 10
# Returns 10 most relevant results, sorted by relevance
```

### 3. Find API Calls

**Old way:**
```bash
grep -r "api.jina.ai" .
grep -r "JINA_API_KEY" .
grep -r "reqwest::Client" . --include="*.rs"
# ... combine results manually ...
```

**New way:**
```bash
cs --hybrid "Jina API calls" .
# Understands API, calls, HTTP, requests, etc.
```

---

## 🎨 Using Results Effectively

### Basic Search (Quick Overview)

```bash
# Simple search - returns top 10
cs --hybrid "error handling" .
```

### With Line Numbers (For Precise Reading)

```bash
# Add -n flag
cs --hybrid "error handling" . -n

# Output shows:
# /path/to/file.rs:42:
#   pub fn handle_error() -> Result<()> {
```

### With Scores (Know What's Most Relevant)

```bash
# Add --scores flag
cs --hybrid "error handling" . --scores

# Output shows:
# [0.89] /path/to/file.rs:42:
#        ^ higher score = more relevant
```

### Control Result Count

```bash
# Get more results
cs --hybrid "error handling" . --topk 20

# Get fewer results
cs --hybrid "error handling" . --topk 5
```

### Filter by Quality

```bash
# Only show high-quality matches
cs --hybrid "error handling" . --threshold 0.8

# More relaxed filtering
cs --hybrid "error handling" . --threshold 0.6
```

---

## 🎯 Power Features (When You Need Them)

### Reranking (Best Results First)

```bash
# Enable reranking for better relevance
cs --hybrid "error handling" . --rerank

# Specify reranker model
cs --hybrid "error handling" . --rerank --rerank-model jina
```

### Search Specific Directory

```bash
# Search only in src/
cs --hybrid "error handling" src/

# Search only in a specific file
cs --hybrid "error handling" src/main.rs
```

### Combine Everything

```bash
# The "ultimate" search command
cs --hybrid "error handling 错误处理" . \
   --topk 15 \
   --rerank \
   --scores \
   -n \
   --threshold 0.7

# Returns 15 high-quality, reranked results
# with scores and line numbers
```

---

## 📚 Cheat Sheet

### Common Patterns

```bash
# Find functions
cs --hybrid "function name" .

# Find types/classes
cs --hybrid "type definition" .
cs --hybrid "class definition" .

# Find specific feature
cs --hybrid "authentication code" .
cs --hybrid "database connection" .
cs --hybrid "API integration" .

# Find bugs/issues
cs --hybrid "error handling" .
cs --hybrid "null pointer" .
cs --hybrid "memory leak" .

# Find tests
cs --hybrid "test for feature X" .
cs --hybrid "unit tests" .

# Find documentation
cs --hybrid "how to use feature X" .
```

### Flags Reference

| Flag | What It Does | Example |
|------|--------------|---------|
| `--topk N` | Return top N results | `--topk 15` |
| `--threshold X` | Only show score ≥ X | `--threshold 0.8` |
| `--rerank` | Improve relevance | `--rerank` |
| `--scores` | Show similarity scores | `--scores` |
| `-n` | Show line numbers | `-n` |
| `--help` | Show all options | `--help` |

---

## 🤝 Comparing with grep

| Task | grep/glob | cs --hybrid |
|------|-----------|-------------|
| **Search for "error"** | `grep -r "error" .` | `cs --hybrid "error" .` |
| **Case insensitive** | `grep -ri "error" .` | Auto-handled |
| **Specific file type** | `grep -r "error" . --include="*.rs"` | Auto-detected |
| **Exclude directories** | `grep -r "error" . --exclude-dir=target` | Auto-excluded (.gitignore) |
| **With line numbers** | `grep -rn "error" .` | `cs --hybrid "error" . -n` |
| **Semantic search** | Not possible | `cs --hybrid "error handling" .` |
| **Multilingual** | Not possible | `cs --hybrid "error 错误" .` |

---

## 💡 Tips & Tricks

### Tip 1: Use Natural Language

❌ Don't think in regex:
```bash
# Hard to remember
grep -r "fn\s+\w+\s*<.*Result.*>" . --include="*.rs"
```

✅ Think in natural language:
```bash
# Easy to remember
cs --hybrid "functions that return Result" .
```

### Tip 2: Mix Languages

```bash
# Use both English and Chinese
cs --hybrid "configuration 配置" .

# Increases recall without noise
```

### Tip 3: Use Scores to Prioritize

```bash
# Search with scores
cs --hybrid "feature X" . --scores

# Read high-score files first (>0.8)
# Skim medium-score files (0.6-0.8)
# Skip low-score files (<0.6)
```

### Tip 4: Iterative Refinement

```bash
# First search - broad
cs --hybrid "authentication" . --topk 20

# Look at results, then narrow down
cs --hybrid "authentication token validation" . --topk 10

# Even more specific
cs --hybrid "JWT token validation middleware" . --topk 5
```

### Tip 5: Save Common Searches

```bash
# Add to your shell aliases
alias search-errors='cs --hybrid "error handling" . --topk 15 --rerank --scores -n'
alias search-tests='cs --hybrid "unit tests" . --topk 10 --scores'
alias search-config='cs --hybrid "configuration" . --topk 10 --rerank'

# Now just run:
search-errors
```

---

## 🎓 Learning Path

### Beginner (5 minutes)

```bash
# 1. Basic search
cs --hybrid "error" .

# 2. With line numbers
cs --hybrid "error" . -n

# 3. With scores
cs --hybrid "error" . --scores

# You now know 80% of what you need!
```

### Intermediate (10 minutes)

```bash
# 4. Control results
cs --hybrid "error" . --topk 15

# 5. Filter quality
cs --hybrid "error" . --threshold 0.7

# 6. Enable reranking
cs --hybrid "error" . --rerank
```

### Advanced (15 minutes)

```bash
# 7. Combine everything
cs --hybrid "error handling 错误处理" . \
   --topk 15 --rerank --scores -n --threshold 0.7

# 8. Search specific paths
cs --hybrid "error" src/core/

# 9. Save as aliases
alias my-search='cs --hybrid "..." . --rerank --scores'
```

**Total learning time: 30 minutes to full proficiency!**

Compare to grep: ~20-35 hours to master regex and all the flags.

---

## ❓ FAQ

### Q: Do I need to know regex?

**A:** No! Natural language works great:
```bash
# No regex needed
cs --hybrid "functions that handle errors" .
```

### Q: Do I need to specify file types?

**A:** No! cs automatically detects file types:
```bash
# Automatically searches .rs, .py, .js, etc.
cs --hybrid "error" .
```

### Q: Do I need to exclude build directories?

**A:** No! cs respects `.gitignore` automatically:
```bash
# Automatically excludes target/, node_modules/, etc.
cs --hybrid "error" .
```

### Q: Can I search in Chinese?

**A:** Yes! Full multilingual support:
```bash
cs --hybrid "错误处理" .
cs --hybrid "error 错误" .  # Mix languages
```

### Q: Is it fast?

**A:** Yes! Pre-built semantic index makes it very fast:
```bash
# First run: builds index (~1-2 min for 1000 files)
cs --index .

# Subsequent searches: instant
cs --hybrid "error" .  # <1 second
```

### Q: Can I use it without internet?

**A:** Yes! Default models work offline:
```bash
# Use local models (no API key needed)
cs --hybrid "error" .

# Or use Jina API for better quality (requires key)
export JINA_API_KEY=your_key
cs --hybrid "error" .
```

---

## 🎯 Summary

### Why cs is Better for Humans

| Aspect | grep/glob | cs --hybrid |
|--------|-----------|-------------|
| **Learning curve** | ~20-35 hours | ~30 minutes |
| **Syntax** | Complex regex | Natural language |
| **Multilingual** | No | Yes (English, Chinese, etc.) |
| **File type handling** | Manual | Automatic |
| **Exclude patterns** | Manual | Automatic (.gitignore) |
| **Relevance** | No ranking | Scored + ranked |
| **Semantic search** | No | Yes |

### Remember

1. **No regex needed** - use natural language
2. **No file type flags** - automatic detection
3. **No exclude patterns** - respects .gitignore
4. **Multilingual** - mix English and Chinese
5. **Scored results** - know what's relevant

### Your First Command

```bash
cs --hybrid "what you want to find" .
```

That's all you need to know to get started!

---

## 📚 Next Steps

1. **Try it now**: `cs --hybrid "error handling" .`
2. **Read examples**: See `test_scenarios/` for real-world examples
3. **Check advanced docs**: See `CODING_AGENT_INTEGRATION.md` for power features

**Happy searching! 🚀**
