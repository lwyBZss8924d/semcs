---
layout: default
title: Quick Start
parent: Tutorials
nav_order: 1
---

# Quick Start Tutorial

**Goal:** Install ck and run your first semantic search in 5 minutes.

**You'll learn:**
- How to install ck
- Run a basic semantic search
- Understand search results

---

## Step 1: Install ck

```bash
cargo install ck-search
```

**Verification:**
```bash
ck --version
# Should output: ck 0.5.x
```

> **Don't have Rust?** Install it first: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## Step 2: Navigate to a Codebase

```bash
# Use any codebase - here's an example
cd ~/projects/your-repo

# Or clone a sample project
git clone https://github.com/BeaconBay/ck
cd ck
```

---

## Step 3: Your First Semantic Search

Let's find error handling code:

```bash
ck --sem "error handling" src/
```

**What happens:**
1. First run creates an index (~1-2 seconds)
2. Search completes and shows results
3. Each result has a relevance score (0.0 - 1.0)

**Example output:**
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

## Step 4: Understanding Results

Each result shows:
- **File path and line numbers:** `src/lib.rs:45-67`
- **Relevance score:** `(0.92)` - higher is more relevant
- **Code snippet:** The actual matching code

**Score guide:**
- **0.9+**: Extremely relevant
- **0.8-0.9**: Highly relevant
- **0.7-0.8**: Relevant
- **< 0.7**: May be tangentially related

---

## Step 5: Try Different Searches

```bash
# Find authentication code
ck --sem "user authentication" src/

# Find caching logic
ck --sem "cache implementation" .

# Find async task spawning
ck --sem "spawn async task" src/
```

**Notice:** ck finds relevant code even without exact keyword matches!

---

## Step 6: Traditional Grep Still Works

```bash
# Find todos
ck "TODO" src/

# Case-insensitive search
ck -i "fixme" .

# Show line numbers
ck -n "fn main" src/
```

ck is grep-compatible, so all your muscle memory works!

---

## What Just Happened?

1. **Index created:** On first search, ck analyzed your code and created embeddings
2. **Semantic search:** Found code by meaning, not just text matching
3. **Ranked results:** Showed most relevant matches first
4. **Chunking:** Results are complete functions/classes, not just lines

---

## Next Steps

✅ **You've completed the quick start!**

**→ Learn interactive search:** [First TUI Session](first-tui-session.html)

**→ Set up AI integration:** [Setup AI Integration](setup-ai-integration.html)

**→ Understand search modes:** [Search Modes Explained](../explanation/search-modes.html)

**→ Need help?** Check [How-To Guides](../how-to/) for specific tasks

---

## Troubleshooting

**❌ Command not found**
```bash
# Add cargo to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**❌ No results**
```bash
# Check index was created
ls .ck/

# Try regex to verify files exist
ck "fn " src/

# Lower threshold for more results
ck --sem "query" --threshold 0.5 src/
```

**❌ Slow indexing**
- Normal on first run (1-2 seconds for medium repos)
- Subsequent searches are instant
- Large repos (>10k files) may take 10-30 seconds

---

**Time spent:** ~5 minutes
**Skills gained:** Basic semantic search, understanding results
**Next:** [Interactive TUI](first-tui-session.html)
