---
layout: default
title: Quick Start
parent: Tutorials
nav_order: 1
---

# Quick Start

Install ck and run your first semantic search in 5 minutes.

## What You'll Learn

- Install ck from crates.io
- Run semantic search to find code by meaning
- Understand search results and relevance scores
- Use traditional grep-style search

---

## Install ck

```bash
cargo install ck-search
```

Verify installation:
```bash
ck --version
```

If you don't have Rust installed, install it first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Your First Semantic Search

Navigate to any codebase and run your first semantic search:

```bash
cd ~/projects/your-repo
ck --sem "error handling" src/
```

This command:
1. Automatically builds an index on first run (1-2 seconds)
2. Finds code related to error handling by meaning, not just text
3. Returns the top 10 most relevant results with scores

Example output:
```
src/lib.rs:45-67 (0.92)
pub fn handle_error(e: Error) -> Result<()> {
    match e {
        Error::Io(err) => log::error!("IO error: {}", err),
        Error::Parse(err) => log::error!("Parse error: {}", err),
    }
}

src/error.rs:12-34 (0.88)
#[derive(Debug)]
pub enum AppError {
    Network(String),
    Timeout,
    InvalidInput(String),
}
```

---

## Understanding Results

Each result shows:
- **File path and line numbers:** `src/lib.rs:45-67`
- **Relevance score:** `(0.92)` - higher is more relevant
- **Code snippet:** The actual matching code

Relevance scores:
- **0.9+**: Extremely relevant
- **0.8-0.9**: Highly relevant
- **0.7-0.8**: Relevant
- **< 0.7**: May be tangentially related

---

## Try Different Searches

```bash
# Find authentication code
ck --sem "user authentication" src/

# Find caching logic
ck --sem "cache implementation" .

# Find async task spawning
ck --sem "spawn async task" src/
```

Notice: ck finds relevant code even without exact keyword matches!

---

## Traditional Grep Still Works

ck is fully grep-compatible:

```bash
# Find todos
ck "TODO" src/

# Case-insensitive search
ck -i "fixme" .

# Show line numbers
ck -n "fn main" src/
```

All your grep muscle memory works!

---

## How It Works

1. **Automatic indexing:** ck analyzed your code and created semantic embeddings
2. **Semantic matching:** Found code by meaning, not just text matching
3. **Ranked results:** Returned the most relevant matches first
4. **Smart chunking:** Results show complete functions/classes, not just lines

---

## Next Steps

- **Interactive search:** [First TUI Session](first-tui-session.html)
- **Installation details:** [Full Installation Guide](installation.html)
- **Search modes:** [Understanding Search Modes](../explanation/search-modes.html)
- **Common tasks:** [How-To Guides](../how-to/)

---

## Troubleshooting

**Command not found:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**No results:**
```bash
# Check if index exists
ls .ck/

# Try regex search to verify files exist
ck "fn " src/

# Lower threshold for more results
ck --sem "query" --threshold 0.5 src/
```

**Slow first search:**
- Normal behavior - indexing takes 1-2 seconds for medium repos
- Subsequent searches are instant
- Large repos may take longer on first run
