---
layout: default
title: Copy Test
nav_order: 999
---

# Code Copy Test Page

This page demonstrates the enhanced code copy functionality.

## Basic Code Block

```bash
cargo install ck-search
ck --sem "error handling" src/
```

## Multi-line Code Block

```rust
fn handle_error(e: Error) -> Result<()> {
    match e {
        Error::Io(err) => log::error!("IO error: {}", err),
        Error::Parse(err) => log::error!("Parse error: {}", err),
        _ => {}
    }
    Ok(())
}
```

## JSON Example

```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "/home/user/project",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

## Long Code Block

```bash
# Install ck
cargo install ck-search

# Verify installation
ck --version

# Navigate to your project
cd ~/projects/your-repo

# Run your first semantic search
ck --sem "error handling" src/

# Try different searches
ck --sem "user authentication" src/
ck --sem "cache implementation" .
ck --sem "spawn async task" src/

# Use traditional grep
ck "TODO" src/
ck -i "fixme" .
ck -n "fn main" src/
```

## Inline Code

You can also copy `inline code` like this: `ck --sem "pattern" src/`

## Instructions

1. **Hover** over any code block to see the copy button
2. **Click** the copy button to copy the code
3. **Watch** for visual feedback (button changes to "Copied!")
4. **Try** the keyboard shortcut: Ctrl+C when focused on a code block
5. **Test** on mobile - copy buttons are always visible

## Features

- ✅ Single-click copy
- ✅ Visual feedback (button changes color and text)
- ✅ Keyboard shortcut support (Ctrl+C)
- ✅ Mobile-friendly (always visible on mobile)
- ✅ Accessibility support (ARIA labels, tooltips)
- ✅ Fallback for older browsers
- ✅ Error handling
- ✅ Smooth animations
