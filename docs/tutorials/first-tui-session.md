---
layout: default
title: Your First TUI Session
parent: Tutorials
nav_order: 2
---

# Your First TUI Session

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Master ck's interactive search interface in 10 minutes.

**You'll learn:**
- Launch and navigate the TUI
- Switch search modes
- Use different preview modes
- Open files in your editor

**Prerequisites:** Complete [Quick Start](quick-start.html) first

---

## Step 1: Launch TUI

```bash
cd ~/projects/your-repo
ck --tui .
```

**You'll see:**
```
┌─ ck Search ──────────────────────────────┐
│ Query: _                    [Semantic ●] │
├──────────────────────────────────────────┤
│ No results yet. Start typing...          │
├──────────────────────────────────────────┤
│ Preview                                   │
│                                           │
└──────────────────────────────────────────┘
```

---

## Step 2: Your First Search

1. **Start typing:** The cursor is already in the search box
2. **Type:** `error handling`
3. **Press Enter**

**What happens:**
- Index builds if this is first TUI search (~1-2 sec)
- Results appear instantly
- Top result is automatically previewed

**You'll see:**
```
┌─ ck Search ──────────────────────────────┐
│ Query: error handling       [Semantic ●] │
├──────────────────────────────────────────┤
│ Results (23)                              │
│ ● src/lib.rs:45 (0.92)                   │← Selected
│   src/error.rs:12 (0.88)                  │
│   src/handler.rs:89 (0.85)                │
├──────────────────────────────────────────┤
│ src/lib.rs:45-67                          │
│                                           │
│  pub fn handle_error(e: Error) {          │← Preview
│      match e { ... }                      │
│  }                                        │
└──────────────────────────────────────────┘
```

---

## Step 3: Navigate Results

**Try these keys:**

- **`j` or `↓`** - Move to next result
- **`k` or `↑`** - Move to previous result
- **`g`** - Jump to first result
- **`G`** - Jump to last result

{: .tip }
**Notice:** Preview updates as you navigate!

---

## Step 4: Cycle Preview Modes

Press **`m`** to cycle through preview modes:

**Mode 1: Chunks (default)**
```
┌─ function handle_error • 45 tokens ─┐
│                                      │
│  pub fn handle_error(e: Error) {     │
│      match e {                       │
│          Error::Io(err) => {...}     │
│      }                                │
│  }                                    │
└──────────────────────────────────────┘
```
Shows semantic boundaries of functions/classes.

**Mode 2: Heatmap** (press `m` again)
```
│  🟢 pub fn handle_error(e: Error) {  │← 0.92 relevance
│  🟡     match e {                    │← 0.75 relevance
│  ⚪         _ => {}                   │← 0.45 relevance
│  }                                    │
```
Colors show line-by-line relevance.

**Mode 3: Full File** (press `m` again)
```
│  use std::error::Error;              │← Full file
│                                       │   scrollable
│  pub fn handle_error(e: Error) {     │
│      match e {                       │
│          ...                          │
│  }                                    │
│                                       │
│  pub fn other_function() { ... }     │
```
Complete file with syntax highlighting.

---

## Step 5: Switch Search Modes

Try different search modes:

**Press `s`** - Semantic mode (default)
- Searches by meaning
- Best for concepts

**Press `r`** - Regex mode
- Traditional grep
- Best for exact patterns

**Press `h`** - Hybrid mode
- Combines both
- Best for keyword + concept

**Try it:**
1. Press `r` to switch to regex
2. Type: `fn test_`
3. Press Enter
4. See all test functions

---

## Step 6: Modify Your Search

**Edit the query:**
1. Press `/` or `i` to enter search mode
2. Type a new query
3. Press Enter to search
4. Press Esc to cancel editing

**Clear and start over:**
- Press `Ctrl+c` to clear the search box

---

## Step 7: Open in Editor

1. Navigate to an interesting result
2. **Press Enter**

**What happens:**
- File opens in `$EDITOR` (vim, nvim, nano, etc.)
- Jumps to the matched line
- TUI continues running in background

**Set your editor:**
```bash
export EDITOR=nvim
# Or: code, vim, emacs, nano, etc.
```

---

## Step 8: Full-File Mode Deep Dive

1. Find an interesting result
2. Press `f` to toggle **full-file mode**
3. Use `j`/`k` or `↓`/`↑` to scroll
4. Press `f` again to go back to chunk mode

**Great for:**
- Understanding context
- Seeing how code fits together
- Reading surrounding functions

---

## Step 9: Practice Workflow

Try this realistic workflow:

1. **Search broadly:** `authentication`
2. **Press `s`** - Ensure semantic mode
3. **Press `m`** until heatmap mode
4. **Navigate** with `j`/`k` to see different files
5. **Press `f`** for full-file on interesting result
6. **Press `Enter`** to open in editor
7. **Press `q`** to quit TUI when done

---

## Common Workflows

### Learning a Codebase

```
1. Search: "database queries"
2. Mode: Semantic
3. Preview: Chunks mode
4. Navigate through results to learn patterns
```

### Finding Bugs

```
1. Search: "error handling"
2. Mode: Heatmap
3. Look for low scores (potential gaps)
4. Open files to investigate
```

### Code Review

```
1. Search: "TODO|FIXME"
2. Mode: Regex
3. Preview: Full file
4. Check context around each TODO
```

---

## Keyboard Reference

### Navigation
- `j`/`↓` - Next result
- `k`/`↑` - Previous result
- `g` - First result
- `G` - Last result
- `Ctrl+d` - Page down
- `Ctrl+u` - Page up

### Search
- `/` or `i` - Edit query
- `Enter` - Execute search
- `Esc` - Cancel editing
- `Ctrl+c` - Clear query

### Modes
- `s` - Semantic mode
- `r` - Regex mode
- `h` - Hybrid mode
- `m` - Cycle preview modes
- `f` - Toggle full-file mode

### Actions
- `Enter` - Open in $EDITOR
- `y` - Copy file path
- `q` - Quit

---

## Tips & Tricks

{: .tip }
**💡 Quick iteration:**
- Don't overthink queries - just start typing
- Use `i` to quickly modify search
- Try different modes (`s`/`r`/`h`) on same query

{: .tip }
**💡 Preview modes:**
- Chunks: Best for understanding structure
- Heatmap: Best for finding most relevant code
- Full-file: Best for context

{: .tip }
**💡 Efficient navigation:**
- Use `g`/`G` to jump to extremes
- Use `Ctrl+d`/`Ctrl+u` for fast scrolling
- Let preview load before moving (smoother experience)

---

## What You've Learned

✅ Launch and navigate TUI
✅ Search in different modes
✅ Use all three preview modes
✅ Open files in your editor
✅ Practical workflows

---

## Next Steps

**→ Connect AI agents:** [AI Integration](../ai-integration/mcp-quickstart.html)

**→ Deep dive TUI features:** [TUI Reference](../reference/tui.html) (full reference)

**→ Understand search modes:** [Search Modes Explained](../explanation/search-modes.html)

**→ Solve specific problems:** Browse [How-To Guides](../how-to/)

---

**Time spent:** ~10 minutes
**Skills gained:** Interactive search, preview modes, editor integration
**Next:** [AI Integration](../ai-integration/mcp-quickstart.html)