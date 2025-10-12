---
layout: default
title: Explanation
nav_order: 5
has_children: true
---

# Explanation

**Understanding-oriented • Conceptual deep-dives**

Background, design decisions, and how ck works under the hood. Understand the "why" behind ck's design and behavior.

<div class="explanation-grid">

<div class="explanation-card core">
<div class="explanation-icon">🧠</div>
<h3><a href="semantic-search">How Semantic Search Works</a></h3>
<p>Deep dive into embeddings, chunking, ranking, and the technology behind semantic code search.</p>
<div class="explanation-topics">
<span class="topic">Embeddings</span>
<span class="topic">Chunking</span>
<span class="topic">Ranking</span>
</div>
</div>

<div class="explanation-card modes">
<div class="explanation-icon">🔍</div>
<h3><a href="search-modes">Search Modes Compared</a></h3>
<p>When to use semantic, regex, and hybrid search modes, and how they work differently.</p>
<div class="explanation-topics">
<span class="topic">Semantic</span>
<span class="topic">Regex</span>
<span class="topic">Hybrid</span>
</div>
</div>

<div class="explanation-card architecture">
<div class="explanation-icon">🏗️</div>
<h3><a href="architecture">Architecture Deep Dive</a></h3>
<p>How ck's components work together: indexing, search, and result ranking systems.</p>
<div class="explanation-topics">
<span class="topic">Indexing</span>
<span class="topic">Search</span>
<span class="topic">Ranking</span>
</div>
</div>

<div class="explanation-card performance">
<div class="explanation-icon">⚡</div>
<h3><a href="performance">Performance Characteristics</a></h3>
<p>Speed, memory usage, scaling behavior, and optimization strategies.</p>
<div class="explanation-topics">
<span class="topic">Speed</span>
<span class="topic">Memory</span>
<span class="topic">Scaling</span>
</div>
</div>

<div class="explanation-card chunking">
<div class="explanation-icon">🧩</div>
<h3><a href="chunking">Chunking Strategy</a></h3>
<p>How tree-sitter parses code structure and creates meaningful chunks for search.</p>
<div class="explanation-topics">
<span class="topic">Tree-sitter</span>
<span class="topic">AST</span>
<span class="topic">Boundaries</span>
</div>
</div>

<div class="explanation-card models">
<div class="explanation-icon">🤖</div>
<h3><a href="embedding-models">Embedding Models</a></h3>
<p>Local embedding models, their trade-offs, and how they understand code semantics.</p>
<div class="explanation-topics">
<span class="topic">Models</span>
<span class="topic">Trade-offs</span>
<span class="topic">Semantics</span>
</div>
</div>

</div>

---

## Understanding ck's Design

<div class="design-principles">

<div class="principle">
<div class="principle-icon">🎯</div>
<div class="principle-content">
<h3>Concept-First Search</h3>
<p>ck searches for <em>what code does</em>, not just what it says. This fundamental shift enables finding patterns across different implementations, languages, and coding styles.</p>
</div>
</div>

<div class="principle">
<div class="principle-icon">🏠</div>
<div class="principle-content">
<h3>Local-First Architecture</h3>
<p>Everything runs locally on your machine. No cloud APIs, no data leaving your system. Your code stays private while you get powerful semantic search.</p>
</div>
</div>

<div class="principle">
<div class="principle-icon">⚡</div>
<div class="principle-content">
<h3>Speed Through Indexing</h3>
<p>One-time indexing enables instant searches. The upfront cost pays dividends in search speed and accuracy.</p>
</div>
</div>

<div class="principle">
<div class="principle-icon">🔧</div>
<div class="principle-content">
<h3>Tool Integration</h3>
<p>Designed to work with existing workflows: command-line, editors, AI agents. Fits into how you already work.</p>
</div>
</div>

</div>

---

## How It All Works

<div class="workflow-diagram">
<div class="workflow-step">
<div class="step-number">1</div>
<div class="step-content">
<h4>Code Analysis</h4>
<p>Tree-sitter parses your code into an Abstract Syntax Tree (AST)</p>
</div>
</div>

<div class="workflow-arrow">→</div>

<div class="workflow-step">
<div class="step-number">2</div>
<div class="step-content">
<h4>Chunking</h4>
<p>Meaningful code units (functions, classes, methods) are extracted</p>
</div>
</div>

<div class="workflow-arrow">→</div>

<div class="workflow-step">
<div class="step-number">3</div>
<div class="step-content">
<h4>Embedding</h4>
<p>Each chunk is converted to a semantic vector using local models</p>
</div>
</div>

<div class="workflow-arrow">→</div>

<div class="workflow-step">
<div class="step-number">4</div>
<div class="step-content">
<h4>Indexing</h4>
<p>Vectors are stored in a local index for fast similarity search</p>
</div>
</div>

<div class="workflow-arrow">→</div>

<div class="workflow-step">
<div class="step-number">5</div>
<div class="step-content">
<h4>Search</h4>
<p>Query is embedded and matched against indexed chunks</p>
</div>
</div>

<div class="workflow-arrow">→</div>

<div class="workflow-step">
<div class="step-number">6</div>
<div class="step-content">
<h4>Ranking</h4>
<p>Results are ranked by semantic similarity and returned</p>
</div>
</div>
</div>

---

## Key Concepts

<div class="concepts-grid">

<div class="concept-card">
<div class="concept-header">
<div class="concept-icon">🧠</div>
<h3>Semantic Vectors</h3>
</div>
<p>Code chunks are converted to high-dimensional vectors that capture semantic meaning. Similar concepts cluster together in vector space.</p>
<div class="concept-example">
<strong>Example:</strong> "error handling" and "exception management" have similar vectors
</div>
</div>

<div class="concept-card">
<div class="concept-header">
<div class="concept-icon">🧩</div>
<h3>Code Chunking</h3>
</div>
<p>Tree-sitter identifies meaningful code boundaries: functions, classes, methods. This preserves context better than line-by-line search.</p>
<div class="concept-example">
<strong>Example:</strong> A complete function is one chunk, not individual lines
</div>
</div>

<div class="concept-card">
<div class="concept-header">
<div class="concept-icon">📊</div>
<h3>Similarity Search</h3>
</div>
<p>Vector similarity (cosine similarity) measures how "close" concepts are. Higher scores mean more relevant matches.</p>
<div class="concept-example">
<strong>Example:</strong> 0.9+ = extremely relevant, 0.7-0.9 = highly relevant
</div>
</div>

<div class="concept-card">
<div class="concept-header">
<div class="concept-icon">🏠</div>
<h3>Local Processing</h3>
</div>
<p>All computation happens on your machine. No data leaves your system, ensuring privacy and eliminating network dependencies.</p>
<div class="concept-example">
<strong>Example:</strong> Embeddings generated locally, no cloud APIs
</div>
</div>

</div>

---

## Design Decisions

<div class="decisions">

<div class="decision">
<h3>Why Tree-sitter for Chunking?</h3>
<p>Tree-sitter provides language-aware parsing that understands code structure. This creates more meaningful chunks than simple text-based approaches.</p>
<div class="decision-pros">
<div class="pro">✅ Language-aware boundaries</div>
<div class="pro">✅ Handles syntax variations</div>
<div class="pro">✅ Preserves code context</div>
</div>
</div>

<div class="decision">
<h3>Why Local Embedding Models?</h3>
<p>Privacy, speed, and reliability. Your code never leaves your machine, and you're not dependent on external services.</p>
<div class="decision-pros">
<div class="pro">✅ Complete privacy</div>
<div class="pro">✅ No network dependency</div>
<div class="pro">✅ Consistent performance</div>
</div>
</div>

<div class="decision">
<h3>Why Index-Based Search?</h3>
<p>Pre-computed indexes enable instant search results. The upfront indexing cost is amortized across many searches.</p>
<div class="decision-pros">
<div class="pro">✅ Instant search results</div>
<div class="pro">✅ Efficient for large codebases</div>
<div class="pro">✅ Incremental updates</div>
</div>
</div>

</div>

---

## Further Reading

<div class="further-reading">
<div class="reading-item">
<h4>🧠 <a href="semantic-search">How Semantic Search Works</a></h4>
<p>Deep technical dive into embeddings and vector search</p>
</div>

<div class="reading-item">
<h4>🔍 <a href="search-modes">Search Modes Compared</a></h4>
<p>When and why to use different search approaches</p>
</div>

<div class="reading-item">
<h4>🏗️ <a href="architecture">Architecture Deep Dive</a></h4>
<p>System design and component interactions</p>
</div>

<div class="reading-item">
<h4>⚡ <a href="performance">Performance Characteristics</a></h4>
<p>Speed, memory, and scaling behavior</p>
</div>
</div>

<style>
.explanation-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  margin: 2rem 0;
}

.explanation-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.explanation-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.3);
}

.explanation-card.core {
  border-left: 4px solid #58a6ff;
}

.explanation-card.modes {
  border-left: 4px solid #238636;
}

.explanation-card.architecture {
  border-left: 4px solid #f85149;
}

.explanation-card.performance {
  border-left: 4px solid #d29922;
}

.explanation-card.chunking {
  border-left: 4px solid #db61a2;
}

.explanation-card.models {
  border-left: 4px solid #a5a5a5;
}

.explanation-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.explanation-card h3 {
  margin-top: 0;
  margin-bottom: 1rem;
}

.explanation-card h3 a {
  color: #f0f6fc;
  text-decoration: none;
}

.explanation-card h3 a:hover {
  color: #58a6ff;
}

.explanation-card p {
  color: #8b949e;
  margin-bottom: 1rem;
}

.explanation-topics {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.topic {
  background: #21262d;
  color: #8b949e;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
}

.design-principles {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.principle {
  display: flex;
  gap: 1rem;
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.principle-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.principle-content h3 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1rem;
}

.principle-content p {
  color: #8b949e;
  margin: 0;
}

.principle-content em {
  color: #58a6ff;
  font-style: normal;
}

.workflow-diagram {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  margin: 3rem 0;
  flex-wrap: wrap;
}

.workflow-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 1.5rem;
  min-width: 150px;
  text-align: center;
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
}

.step-content h4 {
  color: #f0f6fc;
  margin: 0 0 0.5rem 0;
}

.step-content p {
  color: #8b949e;
  margin: 0;
  font-size: 0.9rem;
}

.workflow-arrow {
  color: #58a6ff;
  font-size: 2rem;
  font-weight: bold;
}

.concepts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.concept-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.concept-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.concept-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.concept-header h3 {
  color: #f0f6fc;
  margin: 0;
}

.concept-card p {
  color: #8b949e;
  margin-bottom: 1rem;
}

.concept-example {
  background: #21262d;
  border: 1px solid #30363d;
  border-radius: 6px;
  padding: 1rem;
  font-size: 0.9rem;
}

.concept-example strong {
  color: #f0f6fc;
}

.decisions {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  margin: 3rem 0;
}

.decision {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
}

.decision h3 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 1rem;
}

.decision p {
  color: #8b949e;
  margin-bottom: 1.5rem;
}

.decision-pros {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.pro {
  background: #238636;
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.8rem;
}

.further-reading {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
  margin: 3rem 0;
}

.reading-item {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1.5rem;
}

.reading-item h4 {
  color: #f0f6fc;
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.reading-item h4 a {
  color: #f0f6fc;
  text-decoration: none;
}

.reading-item h4 a:hover {
  color: #58a6ff;
}

.reading-item p {
  color: #8b949e;
  margin: 0;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .explanation-grid {
    grid-template-columns: 1fr;
  }
  
  .design-principles {
    grid-template-columns: 1fr;
  }
  
  .workflow-diagram {
    flex-direction: column;
  }
  
  .workflow-arrow {
    transform: rotate(90deg);
  }
  
  .concepts-grid {
    grid-template-columns: 1fr;
  }
  
  .principle {
    flex-direction: column;
    text-align: center;
  }
}
</style>