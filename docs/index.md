---
layout: default
title: Home
nav_order: 1
---

# ck

**Semantic code search.** Find code by what it does, not just what it says.

```bash
cargo install ck-search
ck --sem "error handling" src/
```

Ask for "error handling" and find try/catch blocks, Result types, panic handlers—even when those exact words aren't there.

---

## Who are you?

<div style="display: grid; grid-template-columns: 1fr 1fr; gap: 3rem; margin: 3rem 0;">

<div style="background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%); border: 1px solid #30363d; border-radius: 12px; padding: 2rem; box-shadow: 0 8px 16px rgba(0,0,0,0.4);">
<div style="font-size: 3rem; margin-bottom: 1rem;">👤</div>
<h3 style="color: #58a6ff; margin-top: 0;">I'm a human developer</h3>
<p style="color: #8b949e; font-size: 0.95rem;">I want to search my codebase, understand code, and find patterns quickly.</p>
<a href="for-humans/" style="display: inline-block; margin-top: 1.5rem; padding: 0.75rem 1.5rem; background: #238636; color: white; text-decoration: none; border-radius: 6px; font-weight: 500;">Get Started →</a>

<div style="margin-top: 2rem; padding-top: 1.5rem; border-top: 1px solid #30363d; font-size: 0.85rem; color: #6e7681;">
<strong>You'll learn:</strong>
<ul style="margin: 0.5rem 0 0 0; padding-left: 1.2rem;">
<li>Install and search (5 min)</li>
<li>Use the interactive TUI</li>
<li>Find code patterns</li>
</ul>
</div>
</div>

<div style="background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%); border: 1px solid #30363d; border-radius: 12px; padding: 2rem; box-shadow: 0 8px 16px rgba(0,0,0,0.4);">
<div style="font-size: 3rem; margin-bottom: 1rem;">🤖</div>
<h3 style="color: #58a6ff; margin-top: 0;">I'm an AI agent</h3>
<p style="color: #8b949e; font-size: 0.95rem;">I need programmatic access to semantic code search for my users.</p>
<a href="for-agents/" style="display: inline-block; margin-top: 1.5rem; padding: 0.75rem 1.5rem; background: #238636; color: white; text-decoration: none; border-radius: 6px; font-weight: 500;">MCP Integration →</a>

<div style="margin-top: 2rem; padding-top: 1.5rem; border-top: 1px solid #30363d; font-size: 0.85rem; color: #6e7681;">
<strong>You'll integrate:</strong>
<ul style="margin: 0.5rem 0 0 0; padding-left: 1.2rem;">
<li>MCP server setup</li>
<li>API reference</li>
<li>Code examples</li>
</ul>
</div>
</div>

</div>

---

## What it does

<div style="background: #161b22; border-left: 4px solid #58a6ff; padding: 1.5rem; margin: 2rem 0; border-radius: 6px;">

**Traditional search** (grep): Match exact text
```bash
grep "try.*catch" src/
```
Finds: Lines with "try" and "catch"

**Semantic search** (ck): Understand concepts
```bash
ck --sem "error handling" src/
```
Finds: try/catch, Result types, panic handling, error returns, validation—regardless of exact wording

</div>

---

## Quick example

```bash
# Search by meaning
ck --sem "retry logic" src/

# Interactive mode
ck --tui .

# For AI agents
ck --serve
```

**One minute to install.** Choose your path above.
