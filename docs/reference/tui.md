---
layout: default
title: TUI Reference
parent: 📖 Reference
nav_order: 1
---

# TUI (Interactive) Mode

The TUI (Text User Interface) provides a beautiful, interactive search experience with live results and code preview.

## Launch TUI

```bash
# Start interactive search in current directory
ck --tui .

# Start with an initial query
ck --tui --sem "error handling" src/

# Start in regex mode
ck --tui src/
```

## Interface Overview

```
┌─────────────────────────────────────────────────────┐
│ Search: error handling              [Semantic] [●] │  ← Search box
├─────────────────────────────────────────────────────┤
│ Results (234)                                       │
│ ● src/lib.rs:45 (0.92)                            │  ← Results list
│   src/error.rs:12 (0.88)                          │    with scores
│   src/handler.rs:89 (0.85)                        │
├─────────────────────────────────────────────────────┤
│ Preview: src/lib.rs:45-60                          │
│                                                     │
│  pub fn handle_error(e: Error) -> Result<()> {    │  ← Code preview
│      match e {                                     │    with syntax
│          Error::Io(err) => {...}                   │    highlighting
│      }                                              │
│  }                                                  │
└─────────────────────────────────────────────────────┘
```

## Keyboard Shortcuts

### Navigation
- `↑`/`k` - Move up in results
- `↓`/`j` - Move down in results
- `Ctrl+u` - Page up in results
- `Ctrl+d` - Page down in results
- `g` - Jump to top result
- `G` - Jump to bottom result

### Search
- `i` / `/` - Enter search mode (edit query)
- `Esc` - Exit search mode
- `Enter` - Execute search
- `Ctrl+c` - Clear search query

### Preview Controls
- `↑`/`k` - Scroll preview up (in full-file mode)
- `↓`/`j` - Scroll preview down (in full-file mode)
- `f` - Toggle full-file mode (show entire file vs just chunk)
- `m` - Cycle preview modes:
  - **Chunks**: Shows semantic chunks with boundaries
  - **Heatmap**: Heat-colored relevance highlighting
  - **Full File**: Complete file with scroll

### Search Modes
- `s` - Switch to **Semantic** search mode
- `r` - Switch to **Regex** search mode
- `h` - Switch to **Hybrid** search mode

### Actions
- `Enter` - Open file in `$EDITOR` (respects EDITOR env var)
- `y` - Copy file path to clipboard
- `q` / `Esc` - Quit TUI

## Preview Modes

### Chunks Mode
Shows the matched code chunk with semantic boundaries and context:

```rust
┌─ function handle_request • 45 tokens ─┐
│                                        │
│  async fn handle_request(req: Request) {
│      let result = match req.method {
│          Method::GET => handle_get(req),
│          Method::POST => handle_post(req),
│      };
│      result
│  }
└────────────────────────────────────────┘
```

**Features:**
- Shows chunk boundaries with tree-sitter precision
- Displays chunk type (function, class, method)
- Shows token estimates
- Breadcrumbs for nested code

**Best for:** Understanding code structure and finding specific functions/methods

### Heatmap Mode
Colors code lines by semantic relevance to your query:

```
🟢  pub fn process_timeout() {      // 0.95 - Highest relevance
🟡      if elapsed > timeout {      // 0.72 - Medium relevance
⚪          return Err(...);         // 0.45 - Lower relevance
```

**Color Scale:**
- 🟢 **Bright Green** (0.875+): Extremely relevant
- 🟢 **Green** (0.75-0.875): Highly relevant
- 🟡 **Yellow** (0.625-0.75): Moderately relevant
- 🟠 **Orange** (0.5-0.625): Somewhat relevant
- ⚪ **Gray** (<0.5): Low relevance

**Best for:** Finding the most relevant lines within a file

### Full File Mode
Shows the complete file with syntax highlighting:

```rust
// Complete file view with scroll
use std::time::Duration;

pub struct Config {
    timeout: Duration,
}

impl Config {
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            timeout: Duration::from_millis(timeout_ms),
        }
    }
}
// ... rest of file ...
```

**Features:**
- Full syntax highlighting
- Scrollable with `↑`/`↓` or `j`/`k`
- Jump to matched line automatically
- Great for seeing full context

**Best for:** Understanding how the matched code fits in the larger file

## Search Modes

### Semantic Mode (`s`)
Searches by meaning, not exact text:

```
Query: "error handling"
Finds: try/catch, Result<>, match arms, error returns
```

**When to use:**
- Finding concepts across different implementations
- Discovering similar patterns
- Learning how something is done in the codebase

### Regex Mode (`r`)
Classic grep-style pattern matching:

```
Query: "fn \w+_test"
Finds: fn test_parse, fn integration_test, etc.
```

**When to use:**
- Finding exact patterns
- Searching for specific syntax
- Performance (no indexing required)

### Hybrid Mode (`h`)
Combines semantic ranking with keyword filtering:

```
Query: "timeout"
Ranks: Semantic relevance but requires "timeout" keyword
```

**When to use:**
- You know a keyword but want semantic ranking
- Filtering broad semantic searches
- Best of both worlds

## Tips & Tricks

### Effective Querying

**Good semantic queries:**
```bash
"authentication logic"      # Concept-based
"database connection pool"  # Specific pattern
"retry mechanism"          # Implementation detail
```

**Less effective:**
```bash
"the code that handles"    # Too vague
"stuff"                    # Not specific enough
```

### Performance Tips

1. **Index once, search many**: The first search in a directory creates an index (~1-2 sec for medium repos)
2. **Use regex for exact matches**: Faster for simple string matching
3. **Narrow your scope**: Search `src/` instead of `.` when possible
4. **Full-file mode**: Use sparingly on large files (can be slow to render)

### Workflow Examples

**Finding a function:**
1. Start TUI: `ck --tui src/`
2. Press `s` for semantic mode
3. Type: "parse configuration"
4. Press `m` to cycle to chunks mode
5. Navigate with `j`/`k`
6. Press `Enter` to open in editor

**Exploring error handling:**
1. Search: `ck --tui --sem "error handling" .`
2. Press `m` for heatmap mode
3. See which lines are most relevant
4. Press `f` for full-file context
5. Press `y` to copy path for later

**Finding todos with context:**
1. Press `r` for regex mode
2. Type: `TODO|FIXME`
3. Press `m` for chunks mode
4. See each TODO in its surrounding code context

## Configuration

### Environment Variables

```bash
# Editor for 'Enter' key
export EDITOR=nvim

# Custom theme (if supported)
export CK_THEME=dark
```

### Color Scheme

TUI uses your terminal's color scheme by default. For best results:
- Use a terminal with 24-bit (true color) support
- Dark mode terminals work best
- Recommended: iTerm2, Alacritty, WezTerm, Windows Terminal

## Troubleshooting

### TUI not launching

```bash
# Check terminal compatibility
echo $TERM
# Should be: xterm-256color, screen-256color, etc.

# Try explicit mode
TERM=xterm-256color ck --tui .
```

### Slow scrolling in full-file mode

- Switch to chunks or heatmap mode (`m`)
- Use regex mode for very large files
- Narrow your search scope

### Search not finding anything

1. Check mode indicator (top-right)
2. Verify index exists (first search creates it)
3. Try regex mode to confirm file exists
4. Check .gitignore/.ckignore isn't excluding files

### Preview not showing

- File might be binary (TUI only shows text)
- File might be too large (>10MB)
- Try opening in editor with `Enter`

## Advanced Features

### Multi-file Navigation

While browsing results:
1. Note interesting results with mental markers
2. Use `y` to copy paths as you go
3. Open each in new editor tab/split

### Comparing Implementations

1. Search for a pattern (e.g., "rate limiting")
2. Use heatmap to see different approaches
3. Compare implementations across files
4. Learn from the highest-scored examples

### Code Discovery

Use semantic search for learning:
```bash
# How is logging done?
ck --tui --sem "logging"

# How are configs loaded?
ck --tui --sem "configuration loading"

# How are errors handled?
ck --tui --sem "error handling patterns"
```

## See Also

- [Search Modes](search-modes.html) - Deep dive into semantic/regex/hybrid
- [CLI Reference](cli-reference.html) - All command-line options
- [Advanced Usage](advanced-usage.html) - Index management and tuning
