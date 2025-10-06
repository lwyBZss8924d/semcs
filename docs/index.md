---
layout: default
title: Home
nav_order: 1
---

# ck

**ck** is a command-line tool that searches your code.

Instead of searching for exact words (like grep), ck understands what you mean. Ask for "error handling" and it finds try/catch blocks, Result types, and panic handlers—even when those exact words aren't in the code.

```bash
cargo install ck-search
ck --sem "error handling" src/
```

It's grep, but it gets the concept.

---

## What do you need?

<div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin: 2rem 0;">

<div style="border: 2px solid #5c5c5c; border-radius: 8px; padding: 1.5rem;">
<h3>🎓 I want to learn</h3>
<p style="color: #888;">Step-by-step lessons to get you started</p>
<div style="background: #2d2d2d; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
<pre style="margin: 0; font-size: 0.85em;">
┌─────────────────┐
│ $ cargo install │
│   ck-search     │
│                 │
│ $ ck --sem ...  │
│   ✓ Results!    │
└─────────────────┘
</pre>
</div>
<p><strong><a href="tutorials/">Start learning →</a></strong></p>
<ul style="font-size: 0.9em; color: #aaa;">
<li>Quick Start (5 min)</li>
<li>Interactive TUI</li>
<li>AI Integration</li>
</ul>
</div>

<div style="border: 2px solid #5c5c5c; border-radius: 8px; padding: 1.5rem;">
<h3>🔧 I have a specific problem</h3>
<p style="color: #888;">Practical recipes for common tasks</p>
<div style="background: #2d2d2d; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
<pre style="margin: 0; font-size: 0.85em;">
┌─────────────────┐
│ Find auth code? │
│ Search 100k LOC?│
│ Setup .ckignore?│
│                 │
│ → Solutions     │
└─────────────────┘
</pre>
</div>
<p><strong><a href="how-to/">Browse guides →</a></strong></p>
<ul style="font-size: 0.9em; color: #aaa;">
<li>Find patterns</li>
<li>Editor integration</li>
<li>Performance tuning</li>
</ul>
</div>

<div style="border: 2px solid #5c5c5c; border-radius: 8px; padding: 1.5rem;">
<h3>📖 I need to look something up</h3>
<p style="color: #888;">Complete technical specifications</p>
<div style="background: #2d2d2d; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
<pre style="margin: 0; font-size: 0.85em;">
┌─────────────────┐
│ --threshold ?   │
│ MCP tools ?     │
│ Config vars ?   │
│                 │
│ → Definitions   │
└─────────────────┘
</pre>
</div>
<p><strong><a href="reference/">Open reference →</a></strong></p>
<ul style="font-size: 0.9em; color: #aaa;">
<li>CLI flags</li>
<li>MCP API</li>
<li>Languages</li>
</ul>
</div>

<div style="border: 2px solid #5c5c5c; border-radius: 8px; padding: 1.5rem;">
<h3>💡 I want to understand</h3>
<p style="color: #888;">How it works under the hood</p>
<div style="background: #2d2d2d; padding: 1rem; border-radius: 4px; margin: 1rem 0;">
<pre style="margin: 0; font-size: 0.85em;">
┌─────────────────┐
│ How embeddings? │
│ Why chunks?     │
│ Index design?   │
│                 │
│ → Deep dives    │
└─────────────────┘
</pre>
</div>
<p><strong><a href="explanation/">Read explanations →</a></strong></p>
<ul style="font-size: 0.9em; color: #aaa;">
<li>Semantic search</li>
<li>Search modes</li>
<li>Architecture</li>
</ul>
</div>

</div>

---

## Quick examples

```bash
# Find code by concept, not keywords
ck --sem "retry logic" src/

# Interactive search with live preview
ck --tui .

# Connect to AI agents (Claude, Cursor)
ck --serve
```

---

**New here?** Start with the [5-minute Quick Start →](tutorials/quick-start.html)
