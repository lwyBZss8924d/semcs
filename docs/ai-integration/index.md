---
layout: default
title: AI Integration
nav_order: 6
has_children: true
---

# AI Integration

**Connect ck to AI coding tools via MCP**

Integrate ck's semantic search into your AI workflow. Connect to Claude Desktop, Cursor, Windsurf, and other AI coding assistants.

<div class="ai-hero">
<div class="ai-hero-content">
<h2>🤖 Supercharge Your AI Coding</h2>
<p>Give AI agents the power to search your codebase semantically. They can find patterns, understand context, and provide better assistance.</p>
<div class="ai-hero-actions">
<a href="mcp-quickstart" class="ai-button primary">Get Started (5 min)</a>
<a href="mcp-api" class="ai-button secondary">API Reference</a>
</div>
</div>
<div class="ai-hero-visual">
<div class="ai-flow">
<div class="flow-step">You</div>
<div class="flow-arrow">→</div>
<div class="flow-step">AI Agent</div>
<div class="flow-arrow">→</div>
<div class="flow-step">ck Search</div>
<div class="flow-arrow">→</div>
<div class="flow-step">Your Code</div>
</div>
</div>
</div>

---

## Supported AI Tools

<div class="ai-tools-grid">

<div class="ai-tool-card featured">
<div class="tool-logo">🧠</div>
<div class="tool-info">
<h3>Claude Desktop</h3>
<p>Official Anthropic desktop app with MCP support</p>
<div class="tool-status">✅ Fully Supported</div>
</div>
<div class="tool-actions">
<a href="setup-guides#claude-desktop" class="tool-button">Setup Guide</a>
</div>
</div>

<div class="ai-tool-card">
<div class="tool-logo">🎯</div>
<div class="tool-info">
<h3>Cursor</h3>
<p>AI-powered code editor with MCP integration</p>
<div class="tool-status">✅ Fully Supported</div>
</div>
<div class="tool-actions">
<a href="setup-guides#cursor" class="tool-button">Setup Guide</a>
</div>
</div>

<div class="ai-tool-card">
<div class="tool-logo">🌊</div>
<div class="tool-info">
<h3>Windsurf</h3>
<p>AI coding assistant with MCP capabilities</p>
<div class="tool-status">✅ Fully Supported</div>
</div>
<div class="tool-actions">
<a href="setup-guides#windsurf" class="tool-button">Setup Guide</a>
</div>
</div>

<div class="ai-tool-card">
<div class="tool-logo">🔗</div>
<div class="tool-info">
<h3>LangChain</h3>
<p>Build custom AI applications with ck</p>
<div class="tool-status">✅ Supported</div>
</div>
<div class="tool-actions">
<a href="setup-guides#langchain" class="tool-button">Setup Guide</a>
</div>
</div>

<div class="ai-tool-card">
<div class="tool-logo">🤖</div>
<div class="tool-info">
<h3>AutoGPT</h3>
<p>Autonomous AI agents with code search</p>
<div class="tool-status">✅ Supported</div>
</div>
<div class="tool-actions">
<a href="setup-guides#autogpt" class="tool-button">Setup Guide</a>
</div>
</div>

<div class="ai-tool-card">
<div class="tool-logo">⚙️</div>
<div class="tool-info">
<h3>Custom MCP Client</h3>
<p>Build your own integration</p>
<div class="tool-status">🔧 Advanced</div>
</div>
<div class="tool-actions">
<a href="setup-guides#custom" class="tool-button">Guide</a>
</div>
</div>

</div>

---

## Available Tools

<div class="tools-overview">

<div class="tool-section">
<div class="tool-header">
<div class="tool-icon">🧠</div>
<div class="tool-info">
<h3>semantic_search</h3>
<p>Find code by meaning, not just text</p>
</div>
</div>
<div class="tool-details">
<div class="tool-params">
<strong>Parameters:</strong>
<ul>
<li><code>query</code> - What you're looking for</li>
<li><code>path</code> - Directory to search</li>
<li><code>threshold</code> - Min relevance (0.0-1.0)</li>
<li><code>top_k</code> - Max results</li>
</ul>
</div>
<div class="tool-example">
<strong>Example:</strong>
<pre><code>{
  "query": "error handling",
  "path": "/home/user/project",
  "threshold": 0.7,
  "top_k": 10
}</code></pre>
</div>
</div>
</div>

<div class="tool-section">
<div class="tool-header">
<div class="tool-icon">🔍</div>
<div class="tool-info">
<h3>regex_search</h3>
<p>Traditional pattern matching with regex</p>
</div>
</div>
<div class="tool-details">
<div class="tool-params">
<strong>Parameters:</strong>
<ul>
<li><code>pattern</code> - Regex pattern</li>
<li><code>path</code> - Directory to search</li>
<li><code>ignore_case</code> - Case-insensitive</li>
<li><code>context</code> - Context lines</li>
</ul>
</div>
<div class="tool-example">
<strong>Example:</strong>
<pre><code>{
  "pattern": "fn test_\\w+",
  "path": "/home/user/project/tests",
  "ignore_case": true
}</code></pre>
</div>
</div>
</div>

<div class="tool-section">
<div class="tool-header">
<div class="tool-icon">🔗</div>
<div class="tool-info">
<h3>hybrid_search</h3>
<p>Semantic ranking + keyword filtering</p>
</div>
</div>
<div class="tool-details">
<div class="tool-params">
<strong>Parameters:</strong>
<ul>
<li><code>query</code> - Search query</li>
<li><code>path</code> - Directory to search</li>
<li><code>threshold</code> - Min relevance</li>
<li><code>top_k</code> - Max results</li>
</ul>
</div>
<div class="tool-example">
<strong>Example:</strong>
<pre><code>{
  "query": "timeout",
  "path": "/home/user/project/src",
  "threshold": 0.7
}</code></pre>
</div>
</div>
</div>

<div class="tool-section">
<div class="tool-header">
<div class="tool-icon">📊</div>
<div class="tool-info">
<h3>index_status</h3>
<p>Check index health and statistics</p>
</div>
</div>
<div class="tool-details">
<div class="tool-params">
<strong>Parameters:</strong>
<ul>
<li><code>path</code> - Directory to check</li>
</ul>
</div>
<div class="tool-example">
<strong>Returns:</strong>
<pre><code>{
  "indexed": true,
  "total_chunks": 15234,
  "total_files": 342,
  "index_size_bytes": 45678901
}</code></pre>
</div>
</div>
</div>

<div class="tool-section">
<div class="tool-header">
<div class="tool-icon">🔄</div>
<div class="tool-info">
<h3>reindex</h3>
<p>Force rebuild of semantic index</p>
</div>
</div>
<div class="tool-details">
<div class="tool-params">
<strong>Parameters:</strong>
<ul>
<li><code>path</code> - Directory to reindex</li>
<li><code>force</code> - Force reindex</li>
</ul>
</div>
<div class="tool-example">
<strong>Use when:</strong>
<ul>
<li>Files changed outside ck</li>
<li>Index corruption</li>
<li>Major refactoring</li>
</ul>
</div>
</div>
</div>

</div>

---

## Quick Start

<div class="quick-start-steps">

<div class="step">
<div class="step-number">1</div>
<div class="step-content">
<h4>Install ck</h4>
<pre><code>cargo install ck-search</code></pre>
</div>
</div>

<div class="step">
<div class="step-number">2</div>
<div class="step-content">
<h4>Configure AI Tool</h4>
<p>Add ck to your AI tool's MCP configuration</p>
</div>
</div>

<div class="step">
<div class="step-number">3</div>
<div class="step-content">
<h4>Test Integration</h4>
<p>Ask your AI agent to search your codebase</p>
</div>
</div>

</div>

---

## Example Workflows

<div class="workflow-examples">

<div class="workflow-example">
<h4>🔍 Code Exploration</h4>
<p><strong>Agent task:</strong> "Understand how authentication works in this codebase"</p>
<div class="workflow-steps">
<div class="workflow-step">1. <code>semantic_search("authentication", "./src")</code></div>
<div class="workflow-step">2. Analyze returned files</div>
<div class="workflow-step">3. <code>semantic_search("token validation", "./src/auth")</code></div>
<div class="workflow-step">4. Synthesize understanding</div>
</div>
</div>

<div class="workflow-example">
<h4>🔧 Refactoring Assistance</h4>
<p><strong>Agent task:</strong> "Find all database queries for refactoring"</p>
<div class="workflow-steps">
<div class="workflow-step">1. <code>hybrid_search("database query", "./src")</code></div>
<div class="workflow-step">2. <code>regex_search("SELECT .* FROM", "./src")</code></div>
<div class="workflow-step">3. Combine results</div>
<div class="workflow-step">4. Propose refactoring</div>
</div>
</div>

<div class="workflow-example">
<h4>🐛 Code Review</h4>
<p><strong>Agent task:</strong> "Find error handling issues"</p>
<div class="workflow-steps">
<div class="workflow-step">1. <code>semantic_search("error handling", "./src")</code></div>
<div class="workflow-step">2. Check each result for best practices</div>
<div class="workflow-step">3. <code>regex_search("unwrap\\(\\)|expect\\(", "./src")</code></div>
<div class="workflow-step">4. Report findings</div>
</div>
</div>

</div>

---

## Get Started

<div class="get-started">
<div class="get-started-card">
<h3>🚀 Quick Setup</h3>
<p>Get up and running in 5 minutes</p>
<a href="mcp-quickstart" class="get-started-button">MCP Quick Start</a>
</div>

<div class="get-started-card">
<h3>📖 Complete Reference</h3>
<p>Full API documentation and examples</p>
<a href="mcp-api" class="get-started-button">MCP API Reference</a>
</div>

<div class="get-started-card">
<h3>💡 Real Examples</h3>
<p>See how AI agents use ck in practice</p>
<a href="examples" class="get-started-button">Examples</a>
</div>
</div>

<style>
.ai-hero {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 3rem;
  align-items: center;
  background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%);
  border: 1px solid #30363d;
  border-radius: 16px;
  padding: 3rem;
  margin: 3rem 0;
}

.ai-hero-content h2 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1rem;
}

.ai-hero-content p {
  color: #8b949e;
  font-size: 1.1rem;
  margin-bottom: 2rem;
}

.ai-hero-actions {
  display: flex;
  gap: 1rem;
}

.ai-button {
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  text-decoration: none;
  font-weight: 600;
  transition: all 0.2s ease;
}

.ai-button.primary {
  background: #238636;
  color: white;
}

.ai-button.primary:hover {
  background: #2ea043;
}

.ai-button.secondary {
  background: transparent;
  color: #58a6ff;
  border: 1px solid #58a6ff;
}

.ai-button.secondary:hover {
  background: #58a6ff;
  color: white;
}

.ai-flow {
  display: flex;
  align-items: center;
  gap: 1rem;
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.flow-step {
  background: #58a6ff;
  color: white;
  padding: 0.75rem 1rem;
  border-radius: 8px;
  font-weight: 600;
  text-align: center;
  min-width: 80px;
}

.flow-arrow {
  color: #58a6ff;
  font-size: 1.5rem;
  font-weight: bold;
}

.ai-tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.ai-tool-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.ai-tool-card.featured {
  border-color: #58a6ff;
  background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%);
}

.tool-logo {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
}

.tool-info h3 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.tool-info p {
  color: #8b949e;
  margin-bottom: 0.5rem;
}

.tool-status {
  font-size: 0.9rem;
  font-weight: 600;
}

.tool-status:contains("✅") {
  color: #238636;
}

.tool-actions {
  margin-top: auto;
}

.tool-button {
  background: #58a6ff;
  color: white;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: background-color 0.2s ease;
}

.tool-button:hover {
  background: #4493f8;
}

.tools-overview {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  margin: 3rem 0;
}

.tool-section {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.tool-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.tool-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.tool-header h3 {
  color: #f0f6fc;
  margin: 0;
}

.tool-header p {
  color: #8b949e;
  margin: 0;
}

.tool-details {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.tool-params ul {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
}

.tool-params li {
  color: #8b949e;
  margin-bottom: 0.25rem;
}

.tool-params code {
  background: #21262d;
  color: #58a6ff;
  padding: 0.125rem 0.25rem;
  border-radius: 3px;
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
}

.tool-example pre {
  background: #0d1117;
  border: 1px solid #21262d;
  border-radius: 6px;
  padding: 1rem;
  overflow-x: auto;
  font-size: 0.9rem;
}

.quick-start-steps {
  display: flex;
  gap: 2rem;
  margin: 3rem 0;
  justify-content: center;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  min-width: 200px;
}

.step-number {
  background: #58a6ff;
  color: white;
  width: 3rem;
  height: 3rem;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 1.2rem;
  margin-bottom: 1rem;
}

.step-content h4 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1rem;
}

.step-content p {
  color: #8b949e;
  margin: 0;
}

.step-content pre {
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 0.75rem;
  font-size: 0.9rem;
  margin: 0;
}

.workflow-examples {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  margin: 3rem 0;
}

.workflow-example {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.workflow-example h4 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.workflow-example p {
  color: #8b949e;
  margin-bottom: 1.5rem;
}

.workflow-steps {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.workflow-step {
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 0.75rem 1rem;
  color: #8b949e;
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
  font-size: 0.9rem;
}

.workflow-step code {
  color: #58a6ff;
}

.get-started {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.get-started-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  text-align: center;
}

.get-started-card h3 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1rem;
}

.get-started-card p {
  color: #8b949e;
  margin-bottom: 1.5rem;
}

.get-started-button {
  background: #238636;
  color: white;
  text-decoration: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  transition: background-color 0.2s ease;
}

.get-started-button:hover {
  background: #2ea043;
}

@media (max-width: 768px) {
  .ai-hero {
    grid-template-columns: 1fr;
    text-align: center;
  }
  
  .ai-flow {
    flex-direction: column;
  }
  
  .flow-arrow {
    transform: rotate(90deg);
  }
  
  .ai-tools-grid {
    grid-template-columns: 1fr;
  }
  
  .tool-details {
    grid-template-columns: 1fr;
  }
  
  .quick-start-steps {
    flex-direction: column;
  }
  
  .get-started {
    grid-template-columns: 1fr;
  }
}
</style>
