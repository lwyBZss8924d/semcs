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

## What it does

<div class="comparison-box">
<div class="comparison-item">
<h3>🔍 Traditional search (grep)</h3>
<p>Match exact text</p>
<pre><code>grep "try.*catch" src/</code></pre>
<p><strong>Finds:</strong> Lines with "try" and "catch"</p>
</div>

<div class="comparison-item highlight">
<h3>🧠 Semantic search (ck)</h3>
<p>Understand concepts</p>
<pre><code>ck --sem "error handling" src/</code></pre>
<p><strong>Finds:</strong> try/catch, Result types, panic handling, error returns, validation—regardless of exact wording</p>
</div>
</div>

---

## Quick start

```bash
# Search by meaning
ck --sem "retry logic" src/

# Interactive mode
ck --tui .

# For AI tools
ck --serve
```

**One minute to install.** Choose your path below.

---

## Documentation

<div class="divio-grid">

<div class="divio-card tutorial">
<div class="divio-icon">🎓</div>
<h3><a href="tutorials/">Tutorials</a></h3>
<p><strong>Learning-oriented</strong> • Step-by-step lessons</p>
<p>Perfect for newcomers. Follow these tutorials to get productive with ck quickly.</p>
<div class="divio-links">
<a href="tutorials/quick-start.html">Quick Start (5 min)</a>
<a href="tutorials/first-tui-session.html">Interactive TUI</a>
<a href="tutorials/installation.html">Installation Guide</a>
</div>
</div>

<div class="divio-card how-to">
<div class="divio-icon">🔧</div>
<h3><a href="how-to/">How-To Guides</a></h3>
<p><strong>Problem-oriented</strong> • Step-by-step recipes</p>
<p>Practical guides for specific tasks and common workflows.</p>
<div class="divio-links">
<a href="how-to/find-patterns.html">Find Specific Patterns</a>
<a href="how-to/editor-integration.html">Editor Integration</a>
<a href="how-to/large-codebases.html">Large Codebases</a>
</div>
</div>

<div class="divio-card reference">
<div class="divio-icon">📖</div>
<h3><a href="reference/">Reference</a></h3>
<p><strong>Information-oriented</strong> • Technical specifications</p>
<p>Complete API documentation and reference materials.</p>
<div class="divio-links">
<a href="reference/cli.html">CLI Reference</a>
<a href="reference/mcp-api.html">MCP API</a>
<a href="how-to/configuration.html">Configuration</a>
</div>
</div>

<div class="divio-card explanation">
<div class="divio-icon">💡</div>
<h3><a href="explanation/">Explanation</a></h3>
<p><strong>Understanding-oriented</strong> • Conceptual deep-dives</p>
<p>Background, design decisions, and how ck works under the hood.</p>
<div class="divio-links">
<a href="explanation/semantic-search.html">How Semantic Search Works</a>
<a href="explanation/search-modes.html">Search Modes Compared</a>
<a href="explanation/architecture.html">Architecture</a>
</div>
</div>

</div>

---

## AI Integration

<div class="ai-integration">
<div class="ai-card">
<h3>🤖 For Humans Using AI Tools</h3>
<p>Connect ck to Claude Desktop, Cursor, and other AI coding assistants via MCP.</p>
<div class="ai-links">
<a href="ai-integration/mcp-quickstart.html">MCP Quick Start</a>
<a href="reference/mcp-api.html">MCP API Reference</a>
<a href="ai-integration/examples.html">Examples</a>
</div>
</div>
</div>

<style>
.comparison-box {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  margin: 2rem 0;
}

.comparison-item {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1.5rem;
}

.comparison-item.highlight {
  border-color: #58a6ff;
  background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%);
}

.comparison-item h3 {
  color: #58a6ff;
  margin-top: 0;
}

.comparison-item pre {
  background: #0d1117;
  border: 1px solid #21262d;
  border-radius: 6px;
  padding: 1rem;
  margin: 1rem 0;
  overflow-x: auto;
}

.divio-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.divio-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  position: relative;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.divio-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.3);
}

.divio-card.tutorial {
  border-left: 4px solid #58a6ff;
}

.divio-card.how-to {
  border-left: 4px solid #238636;
}

.divio-card.reference {
  border-left: 4px solid #f85149;
}

.divio-card.explanation {
  border-left: 4px solid #a5a5a5;
}

.divio-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.divio-card h3 {
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.divio-card h3 a {
  color: #f0f6fc;
  text-decoration: none;
}

.divio-card h3 a:hover {
  color: #58a6ff;
}

.divio-card p {
  color: #8b949e;
  margin-bottom: 1rem;
}

.divio-card p strong {
  color: #f0f6fc;
}

.divio-links {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.divio-links a {
  color: #58a6ff;
  text-decoration: none;
  font-size: 0.9rem;
  padding: 0.25rem 0;
  border-bottom: 1px solid transparent;
  transition: border-color 0.2s ease;
}

.divio-links a:hover {
  border-bottom-color: #58a6ff;
}

.ai-integration {
  margin: 3rem 0;
}

.ai-card {
  background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%);
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  text-align: center;
}

.ai-card h3 {
  color: #58a6ff;
  margin-top: 0;
}

.ai-card p {
  color: #8b949e;
  margin-bottom: 1.5rem;
}

.ai-links {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.ai-links a {
  background: #238636;
  color: white;
  text-decoration: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-weight: 500;
  transition: background-color 0.2s ease;
}

.ai-links a:hover {
  background: #2ea043;
}

@media (max-width: 768px) {
  .comparison-box {
    grid-template-columns: 1fr;
  }
  
  .divio-grid {
    grid-template-columns: 1fr;
  }
  
  .ai-links {
    flex-direction: column;
    align-items: center;
  }
}
</style>