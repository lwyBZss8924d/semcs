---
layout: default
title: Interactive TUI
parent: For Humans
nav_order: 2
---

# Interactive TUI
{: .no_toc }

Visual search interface with live results.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Launch

```bash
ck --tui .
```

You'll see:
```
┌─ ck ────────────────────────────┐
│ Query: _              [Semantic]│
├─────────────────────────────────┤
│ Start typing to search...       │
├─────────────────────────────────┤
│ Preview                         │
└─────────────────────────────────┘
```

---

## Basic navigation

**Search:**
- Type your query
- Press `Enter` to search
- Press `Esc` to stop editing

**Navigate results:**
- `j` or `↓` - Next result
- `k` or `↑` - Previous result
- `g` - Jump to top
- `G` - Jump to bottom

**Actions:**
- `Enter` - Open file in `$EDITOR`
- `y` - Copy file path
- `q` - Quit

---

## Search modes

Switch modes with a single key:

**`s` - Semantic mode**
```
Query: error handling
Finds: try/catch, Result<>, panic!, etc.
```

**`r` - Regex mode**
```
Query: fn test_\w+
Finds: fn test_parse, fn test_auth, etc.
```

**`h` - Hybrid mode**
```
Query: timeout
Finds: Code with "timeout" ranked by relevance
```

---

## Preview modes

Press `m` to cycle through:

**1. Chunks** (default)
```
┌─ function handle_error • 45 tokens ─┐
│ pub fn handle_error(e: Error) {     │
│     match e { ... }                  │
│ }                                    │
└─────────────────────────────────────┘
```
Shows function/class boundaries.

**2. Heatmap**
```
│ 🟢 pub fn handle_error...  (0.92)   │
│ 🟡     match e {           (0.75)   │
│ ⚪         _ => {}          (0.45)   │
```
Colors show line-by-line relevance.

**3. Full File**
```
│ use std::error::Error;              │
│                                      │
│ pub fn handle_error(e: Error) {     │
│     match e { ... }                  │
│ }                                    │
```
Complete file with scroll.

---

## Full-file mode

Press `f` to toggle:
- Shows entire file
- Scroll with `j`/`k` or `↓`/`↑`
- Great for context
- Press `f` again to return to chunks

---

## Quick workflows

**Find and open:**
1. Type query → Enter
2. Navigate with `j`/`k`
3. Press `Enter` to open

**Explore different implementations:**
1. Search concept (e.g., "authentication")
2. Press `m` for heatmap
3. See which files are most relevant
4. Navigate and compare

**Learn a codebase:**
1. Search "database queries"
2. Chunks mode to see function structure
3. Full-file mode for context
4. Open in editor to dive deep

---

## Tips

{: .tip }
**Set your editor:** `export EDITOR=nvim` (or code, vim, emacs)

{: .tip }
**Quick iteration:** Use `i` or `/` to edit your search without leaving TUI

{: .tip }
**Best preview mode:**
- **Chunks** - Understanding structure
- **Heatmap** - Finding most relevant lines
- **Full-file** - Seeing context

---

## All keyboard shortcuts

| Key | Action |
|-----|--------|
| `j` / `↓` | Next result |
| `k` / `↑` | Previous result |
| `g` | First result |
| `G` | Last result |
| `Ctrl+d` | Page down |
| `Ctrl+u` | Page up |
| `i` / `/` | Edit query |
| `Enter` | Execute search / Open file |
| `Esc` | Cancel editing |
| `Ctrl+c` | Clear query |
| `s` | Semantic mode |
| `r` | Regex mode |
| `h` | Hybrid mode |
| `m` | Cycle preview modes |
| `f` | Toggle full-file |
| `y` | Copy path |
| `q` | Quit |

---

## Next steps

**→** [Learn search modes](search-modes.html) - When to use each mode

**→** [Find patterns](find-patterns.html) - Common searches

**→** [Configure ck](configuration.html) - Settings and .ckignore
