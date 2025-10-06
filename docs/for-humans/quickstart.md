---
layout: default
title: Quick Start
parent: For Humans
nav_order: 1
---

# Quick Start
{: .no_toc }

Get searching in 5 minutes.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Install

```bash
cargo install ck-search
```

{: .note }
Don't have Rust? Install it first: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Verify:
```bash
ck --version
# Should show: ck 0.5.x
```

---

## Your first search

Navigate to any codebase:
```bash
cd ~/projects/your-repo
```

Search for error handling:
```bash
ck --sem "error handling" src/
```

{: .important }
**First run takes ~2 seconds** to build an index. After that, searches are instant.

---

## Understanding results

```
src/lib.rs:45-67 (0.92)
pub fn handle_error(e: Error) -> Result<()> {
    match e {
        Error::Io(err) => log::error!("IO error: {}", err),
        Error::Parse(err) => log::error!("Parse error: {}", err),
    }
}
```

Each result shows:
- **File path** `src/lib.rs:45-67`
- **Relevance score** `(0.92)` - higher is better
- **Code snippet** - the matching function/class

**Score guide:**
- `0.9+` Extremely relevant
- `0.8-0.9` Highly relevant
- `0.7-0.8` Relevant
- `<0.7` Possibly related

---

## Try more searches

```bash
# Find authentication code
ck --sem "user authentication" src/

# Find caching
ck --sem "cache implementation" .

# Find async tasks
ck --sem "spawn async task" src/
```

{: .tip }
Notice: ck finds relevant code even without exact keyword matches!

---

## Traditional grep still works

```bash
# Find todos
ck "TODO" src/

# Case-insensitive
ck -i "fixme" .

# With line numbers
ck -n "fn main" src/
```

All your grep muscle memory works!

---

## What just happened?

1. **Index created** - ck analyzed your code structure
2. **Embeddings generated** - Code converted to semantic vectors
3. **Search executed** - Found matches by meaning, not just text
4. **Results ranked** - Most relevant code shown first

---

## Next steps

**→** [Try the interactive TUI](tui.html) - Visual search interface

**→** [Learn search modes](search-modes.html) - Semantic vs regex vs hybrid

**→** [Find specific patterns](find-patterns.html) - Common use cases
