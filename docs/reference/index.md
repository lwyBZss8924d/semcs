---
layout: default
title: Reference
nav_order: 4
has_children: true
---

# Reference

**Information-oriented • Technical specifications**

Complete API documentation and reference materials. Everything you need to know about ck's commands, APIs, and configuration options.

<div class="reference-grid">

<div class="reference-card cli">
<div class="reference-icon">💻</div>
<h3><a href="cli">CLI Reference</a></h3>
<p>Complete command-line interface documentation with all flags, options, and examples.</p>
<div class="reference-stats">
<span class="stat">50+ commands</span>
<span class="stat">All flags</span>
<span class="stat">Examples</span>
</div>
</div>

<div class="reference-card api">
<div class="reference-icon">🔌</div>
<h3><a href="mcp-api.html">MCP API</a></h3>
<p>Model Context Protocol API specification for AI agent integration.</p>
<div class="reference-stats">
<span class="stat">6 tools</span>
<span class="stat">JSON-RPC</span>
<span class="stat">Examples</span>
</div>
</div>

<div class="reference-card config">
<div class="reference-icon">⚙️</div>
<h3><a href="configuration">Configuration</a></h3>
<p>Environment variables, settings, and configuration options.</p>
<div class="reference-stats">
<span class="stat">20+ options</span>
<span class="stat">Environment</span>
<span class="stat">Files</span>
</div>
</div>

<div class="reference-card languages">
<div class="reference-icon">🌐</div>
<h3><a href="languages">Language Support</a></h3>
<p>Supported programming languages and chunking strategies.</p>
<div class="reference-stats">
<span class="stat">15+ languages</span>
<span class="stat">Tree-sitter</span>
<span class="stat">Chunking</span>
</div>
</div>

<div class="reference-card formats">
<div class="reference-icon">📄</div>
<h3><a href="formats">Output Formats</a></h3>
<p>JSONL, JSON, and other output format specifications.</p>
<div class="reference-stats">
<span class="stat">3 formats</span>
<span class="stat">Structured</span>
<span class="stat">Parsable</span>
</div>
</div>

<div class="reference-card tui">
<div class="reference-icon">🖥️</div>
<h3><a href="tui">TUI Reference</a></h3>
<p>Interactive terminal interface commands and shortcuts.</p>
<div class="reference-stats">
<span class="stat">30+ keys</span>
<span class="stat">Modes</span>
<span class="stat">Navigation</span>
</div>
</div>

</div>

---

## Quick Reference

<div class="quick-ref-grid">

<div class="quick-ref-section">
<h3>🔍 Search Commands</h3>
<div class="command-list">
<div class="command">
<code>ck --sem "query" path/</code>
<span class="command-desc">Semantic search</span>
</div>
<div class="command">
<code>ck "pattern" path/</code>
<span class="command-desc">Regex search</span>
</div>
<div class="command">
<code>ck --hybrid "query" path/</code>
<span class="command-desc">Hybrid search</span>
</div>
<div class="command">
<code>ck --tui path/</code>
<span class="command-desc">Interactive mode</span>
</div>
</div>
</div>

<div class="quick-ref-section">
<h3>⚙️ Common Options</h3>
<div class="command-list">
<div class="command">
<code>--threshold 0.7</code>
<span class="command-desc">Min relevance score</span>
</div>
<div class="command">
<code>--topk 50</code>
<span class="command-desc">Max results</span>
</div>
<div class="command">
<code>--jsonl</code>
<span class="command-desc">JSONL output</span>
</div>
<div class="command">
<code>--serve</code>
<span class="command-desc">MCP server mode</span>
</div>
</div>
</div>

<div class="quick-ref-section">
<h3>🎯 MCP Tools</h3>
<div class="command-list">
<div class="command">
<code>semantic_search</code>
<span class="command-desc">Find by meaning</span>
</div>
<div class="command">
<code>regex_search</code>
<span class="command-desc">Pattern matching</span>
</div>
<div class="command">
<code>hybrid_search</code>
<span class="command-desc">Semantic + keywords</span>
</div>
<div class="command">
<code>index_status</code>
<span class="command-desc">Check index health</span>
</div>
</div>
</div>

</div>

---

## Reference Categories

<div class="ref-categories">

<div class="ref-category">
<div class="category-header">
<div class="category-icon">💻</div>
<div class="category-info">
<h3>Command Line</h3>
<p>CLI interface and commands</p>
</div>
</div>
<div class="category-links">
<a href="cli">CLI Reference</a>
<a href="tui">TUI Reference</a>
<a href="configuration">Configuration</a>
</div>
</div>

<div class="ref-category">
<div class="category-header">
<div class="category-icon">🔌</div>
<div class="category-info">
<h3>API & Integration</h3>
<p>Programmatic access and APIs</p>
</div>
</div>
<div class="category-links">
<a href="mcp-api">MCP API</a>
<a href="formats">Output Formats</a>
<a href="jsonl">JSONL Format</a>
</div>
</div>

<div class="ref-category">
<div class="category-header">
<div class="category-icon">🌐</div>
<div class="category-info">
<h3>Language Support</h3>
<p>Programming languages and parsing</p>
</div>
</div>
<div class="category-links">
<a href="languages">Supported Languages</a>
<a href="chunking">Chunking Strategies</a>
<a href="tree-sitter">Tree-sitter Queries</a>
</div>
</div>

</div>

---

## Search This Reference

<div class="search-box">
<input type="text" placeholder="Search reference documentation..." class="search-input">
<button class="search-button">🔍</button>
</div>

<div class="search-suggestions">
<div class="suggestion">
<strong>Looking for a specific command?</strong>
<p>Try searching for: <code>--sem</code>, <code>--tui</code>, <code>--threshold</code></p>
</div>
<div class="suggestion">
<strong>Need API documentation?</strong>
<p>Check out: <a href="mcp-api">MCP API Reference</a></p>
</div>
<div class="suggestion">
<strong>Configuration questions?</strong>
<p>See: <a href="configuration">Configuration Reference</a></p>
</div>
</div>

<style>
.reference-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  margin: 2rem 0;
}

.reference-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.reference-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.3);
}

.reference-card.cli {
  border-left: 4px solid #58a6ff;
}

.reference-card.api {
  border-left: 4px solid #238636;
}

.reference-card.config {
  border-left: 4px solid #f85149;
}

.reference-card.languages {
  border-left: 4px solid #a5a5a5;
}

.reference-card.formats {
  border-left: 4px solid #d29922;
}

.reference-card.tui {
  border-left: 4px solid #db61a2;
}

.reference-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.reference-card h3 {
  margin-top: 0;
  margin-bottom: 1rem;
}

.reference-card h3 a {
  color: #f0f6fc;
  text-decoration: none;
}

.reference-card h3 a:hover {
  color: #58a6ff;
}

.reference-card p {
  color: #8b949e;
  margin-bottom: 1rem;
}

.reference-stats {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.stat {
  background: #21262d;
  color: #8b949e;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
}

.quick-ref-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.quick-ref-section {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.quick-ref-section h3 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1.5rem;
}

.command-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.command {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 0.75rem 1rem;
}

.command code {
  background: #0d1117;
  color: #58a6ff;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
}

.command-desc {
  color: #8b949e;
  font-size: 0.9rem;
}

.ref-categories {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.ref-category {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.category-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.category-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.category-info h3 {
  color: #f0f6fc;
  margin: 0 0 0.25rem 0;
}

.category-info p {
  color: #8b949e;
  margin: 0;
  font-size: 0.9rem;
}

.category-links {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.category-links a {
  color: #58a6ff;
  text-decoration: none;
  padding: 0.5rem 0;
  border-bottom: 1px solid transparent;
  transition: border-color 0.2s ease;
}

.category-links a:hover {
  border-bottom-color: #58a6ff;
}

.search-box {
  display: flex;
  gap: 1rem;
  margin: 2rem 0;
  max-width: 500px;
}

.search-input {
  flex: 1;
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 0.75rem 1rem;
  color: #f0f6fc;
  font-size: 1rem;
}

.search-input:focus {
  outline: none;
  border-color: #58a6ff;
}

.search-button {
  background: #58a6ff;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 0.75rem 1rem;
  cursor: pointer;
  font-size: 1rem;
}

.search-suggestions {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
  margin: 2rem 0;
}

.suggestion {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1.5rem;
}

.suggestion strong {
  color: #f0f6fc;
}

.suggestion p {
  color: #8b949e;
  margin: 0.5rem 0 0 0;
}

.suggestion code {
  background: #21262d;
  color: #58a6ff;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
}

.suggestion a {
  color: #58a6ff;
  text-decoration: none;
}

.suggestion a:hover {
  text-decoration: underline;
}

@media (max-width: 768px) {
  .reference-grid {
    grid-template-columns: 1fr;
  }
  
  .quick-ref-grid {
    grid-template-columns: 1fr;
  }
  
  .ref-categories {
    grid-template-columns: 1fr;
  }
  
  .command {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .search-box {
    flex-direction: column;
  }
}
</style>