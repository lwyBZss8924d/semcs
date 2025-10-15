# Unexpected Behaviors in cc

This file tracks instances where cc behaves unexpectedly during testing or usage.

## Format

**Command:** `command run`  
**Expected:** What should have happened  
**Actual:** What actually happened  
**Date:** YYYY-MM-DD  
**Status:** [Fixed/Open/Investigating]

---

## Issues Found

**Command:** `cc ""`
**Expected:** Error message or no results
**Actual:** Massive output with every empty line in the codebase being matched
**Date:** 2025-09-13
**Status:** Open
**Notes:** Empty pattern matches all empty lines, resulting in overwhelming output. Perhaps should warn or limit results?

---

**Command:** `cc --add /tmp/test.txt`
**Expected:** Add file to index or meaningful error
**Actual:** Error: "No file specified. Usage: cc --add <file>"
**Date:** 2025-09-13
**Status:** Open
**Notes:** The error message is incorrect - a file was specified. Seems like argument parsing issue.

---

**Command:** `cc --sem "🎉🦀✨"`
**Expected:** No results or error about emoji patterns
**Actual:** Returns seemingly random code results
**Date:** 2025-09-13
**Status:** Open
**Notes:** Emoji search returns unrelated results. The semantic embedding seems to handle emojis unpredictably.

---

**Command:** `cc --index --model jina-code-1.5b /path/to/codebase`
**Expected:** Successfully index entire codebase (Jina API docs claim 8,192 token context = ~32KB per input)
**Actual:** Most files failed with "Jina API error (400 Bad Request): Failed to encode text" - API has undocumented ~1KB hard limit
**Date:** 2025-10-13
**Status:** Fixed (with workaround)
**Notes:**
- Jina Code Embeddings API documentation claims 8K token context limit but actually rejects inputs >~1KB
- **ROOT CAUSE**: The `task` parameter is REQUIRED for jina-code models. Without it, limit is ~952 bytes. With `task="nl2code.passage"`, limit increases to ~1012 bytes
- Tested extensively with main.rs (67KB file):
  - WITHOUT task parameter: 32 lines (952 bytes) ✅, 33 lines (1011 bytes) ❌
  - WITH task parameter: 34 lines (1012 bytes) ✅, 35 lines (1065 bytes) ❌
- Solution:
  1. Added `task="nl2code.passage"` parameter for indexing code (required per API spec)
  2. Implemented automatic text splitting at 1KB chunks + embedding averaging in [cc-embed/src/jina_api.rs](cc-embed/src/jina_api.rs)
- After fix: Successfully indexed 67 files with 631 chunks
- **Recommendation**: Use local `jina-code` model (FastEmbed) for production - no byte limits, faster, offline-capable, supports full 8K context
- Despite official 8K token (≈32KB) documentation, actual API limit remains around 1KB regardless of task parameter
- This represents a significant gap between API documentation and actual implementation

---

## Instructions

When you encounter unexpected behavior while using cc:

1. Note the exact command you ran
2. Describe what you expected to happen
3. Describe what actually happened
4. Add the date
5. Set status to "Open"

This helps track and fix edge cases and user experience issues.