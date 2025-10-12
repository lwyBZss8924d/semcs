---
layout: default
title: Tutorials
nav_order: 2
has_children: true
---

# Tutorials

**Learning-oriented • Step-by-step lessons**

Perfect for newcomers. Follow these tutorials to get productive with ck quickly.

<div class="tutorial-grid">

<div class="tutorial-card primary">
<div class="tutorial-badge">⭐ Essential</div>
<h3><a href="quick-start">Quick Start</a></h3>
<p><strong>5 minutes</strong> • Install ck and run your first semantic search</p>
<p>Get up and running with ck in minutes. Learn the basics of semantic search and understand your first results.</p>
<div class="tutorial-progress">
<span class="progress-step">Install</span> → <span class="progress-step">Search</span> → <span class="progress-step">Understand</span>
</div>
</div>

<div class="tutorial-card">
<h3><a href="first-tui-session">Your First TUI Session</a></h3>
<p><strong>10 minutes</strong> • Master the interactive search interface</p>
<p>Learn to use ck's beautiful interactive interface with live results, code preview, and multiple search modes.</p>
<div class="tutorial-progress">
<span class="progress-step">Launch</span> → <span class="progress-step">Navigate</span> → <span class="progress-step">Master</span>
</div>
</div>

<div class="tutorial-card">
<h3><a href="installation">Installation Guide</a></h3>
<p><strong>15 minutes</strong> • Complete installation and configuration</p>
<p>Detailed installation instructions for all platforms, troubleshooting, and optional configuration.</p>
<div class="tutorial-progress">
<span class="progress-step">Choose Platform</span> → <span class="progress-step">Install</span> → <span class="progress-step">Configure</span>
</div>
</div>

</div>

---

## Tutorial Path

<div class="tutorial-path">
<div class="path-step">
<div class="step-number">1</div>
<div class="step-content">
<h4>Quick Start (Required)</h4>
<p>Essential first steps to get ck working</p>
</div>
</div>

<div class="path-arrow">→</div>

<div class="path-step">
<div class="step-number">2</div>
<div class="step-content">
<h4>First TUI Session (Recommended)</h4>
<p>Learn the interactive interface</p>
</div>
</div>

<div class="path-arrow">→</div>

<div class="path-step">
<div class="step-number">3</div>
<div class="step-content">
<h4>Installation Guide (Optional)</h4>
<p>Advanced setup and configuration</p>
</div>
</div>
</div>

---

## What You'll Learn

<div class="learning-outcomes">
<div class="outcome">
<div class="outcome-icon">✅</div>
<div class="outcome-text">
<h4>Install and configure ck</h4>
<p>Get ck running on your system</p>
</div>
</div>

<div class="outcome">
<div class="outcome-icon">✅</div>
<div class="outcome-text">
<h4>Run semantic searches</h4>
<p>Find code by meaning, not just text</p>
</div>
</div>

<div class="outcome">
<div class="outcome-icon">✅</div>
<div class="outcome-text">
<h4>Use the interactive TUI</h4>
<p>Master the visual search interface</p>
</div>
</div>

<div class="outcome">
<div class="outcome-icon">✅</div>
<div class="outcome-text">
<h4>Understand search results</h4>
<p>Interpret relevance scores and code chunks</p>
</div>
</div>
</div>

---

## Next Steps

After completing the tutorials, explore:

- **[How-To Guides](../how-to/)** - Solve specific problems
- **[Reference](../reference/)** - Complete technical documentation  
- **[Explanation](../explanation/)** - Understand how ck works

<style>
.tutorial-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  margin: 2rem 0;
}

.tutorial-card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 12px;
  padding: 2rem;
  position: relative;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.tutorial-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.3);
}

.tutorial-card.primary {
  border-color: #58a6ff;
  background: linear-gradient(135deg, #1a1f2e 0%, #161b22 100%);
}

.tutorial-badge {
  position: absolute;
  top: -8px;
  right: 1rem;
  background: #58a6ff;
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 600;
}

.tutorial-card h3 {
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.tutorial-card h3 a {
  color: #f0f6fc;
  text-decoration: none;
}

.tutorial-card h3 a:hover {
  color: #58a6ff;
}

.tutorial-card p {
  color: #8b949e;
  margin-bottom: 1rem;
}

.tutorial-card p strong {
  color: #f0f6fc;
}

.tutorial-progress {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  color: #6e7681;
}

.progress-step {
  background: #21262d;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
}

.tutorial-path {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  margin: 2rem 0;
  flex-wrap: wrap;
}

.path-step {
  display: flex;
  align-items: center;
  gap: 1rem;
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1rem;
  min-width: 200px;
}

.step-number {
  background: #58a6ff;
  color: white;
  width: 2rem;
  height: 2rem;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  flex-shrink: 0;
}

.step-content h4 {
  margin: 0 0 0.25rem 0;
  color: #f0f6fc;
}

.step-content p {
  margin: 0;
  color: #8b949e;
  font-size: 0.9rem;
}

.path-arrow {
  color: #58a6ff;
  font-size: 1.5rem;
  font-weight: bold;
}

.learning-outcomes {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin: 2rem 0;
}

.outcome {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1.5rem;
}

.outcome-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.outcome-text h4 {
  margin: 0 0 0.5rem 0;
  color: #f0f6fc;
}

.outcome-text p {
  margin: 0;
  color: #8b949e;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .tutorial-path {
    flex-direction: column;
  }
  
  .path-arrow {
    transform: rotate(90deg);
  }
  
  .tutorial-grid {
    grid-template-columns: 1fr;
  }
}
</style>