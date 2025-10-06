---
layout: default
title: Home
---

# ck - Semantic Code Search

**ck (seek)** finds code by meaning, not just keywords. Search for "error handling" and find try/catch blocks, error returns, and exception handling—even when those exact words aren't present.

```bash
# Install
cargo install ck-search

# Search by meaning
ck --sem "authentication logic" src/

# Interactive TUI
ck --tui .

# AI agent integration
ck --serve
```

---

## Documentation

Following the [Divio documentation system](https://docs.divio.com/documentation-system/), our docs are organized by your needs:

### 📚 [Tutorials](tutorials/)
*Learning-oriented • Get started fast*

Perfect for newcomers. Step-by-step lessons to get you productive quickly.

- **[Quick Start](tutorials/quick-start.html)** - Install and run your first search (5 min)
- **[Your First TUI Session](tutorials/first-tui-session.html)** - Interactive search walkthrough (10 min)
- **[Setting Up AI Integration](tutorials/setup-ai-integration.html)** - Connect Claude Desktop (15 min)

### 🔧 [How-To Guides](how-to/)
*Problem-oriented • Solve specific tasks*

Recipes for common tasks and workflows. Goal-oriented instructions.

- **[Find Specific Patterns](how-to/find-patterns.html)** - Authentication, errors, configs, etc.
- **[Integrate with Your Editor](how-to/editor-integration.html)** - VS Code, Vim, Emacs
- **[Search Large Codebases](how-to/large-codebases.html)** - Performance tips and tricks
- **[Customize File Filtering](how-to/file-filtering.html)** - .ckignore and exclusion patterns
- **[Use in CI/CD](how-to/ci-cd.html)** - Automated code analysis

### 📖 [Reference](reference/)
*Information-oriented • Look up details*

Technical specifications and complete API documentation.

- **[CLI Reference](reference/cli.html)** - All command-line flags and options
- **[MCP API](reference/mcp-api.html)** - Complete MCP tool specifications
- **[Configuration](reference/configuration.html)** - Environment variables and settings
- **[Language Support](reference/languages.html)** - Supported languages and chunking

### 💡 [Explanation](explanation/)
*Understanding-oriented • Conceptual background*

Deep dives into how ck works and why it's designed this way.

- **[How Semantic Search Works](explanation/semantic-search.html)** - Embeddings, chunking, ranking
- **[Search Modes Compared](explanation/search-modes.html)** - When to use each mode
- **[Chunking Strategy](explanation/chunking.html)** - Tree-sitter and code structure
- **[Index Architecture](explanation/index-architecture.html)** - How indexes are built and updated

---

## Quick Links

- [GitHub Repository](https://github.com/BeaconBay/ck)
- [Crates.io](https://crates.io/crates/ck-search)
- [Report Issues](https://github.com/BeaconBay/ck/issues)
- [Changelog](https://github.com/BeaconBay/ck/blob/main/CHANGELOG.md)

---

## Not Sure Where to Start?

**→ New to ck?** Start with [Quick Start Tutorial](tutorials/quick-start.html)

**→ Need to solve a problem?** Browse [How-To Guides](how-to/)

**→ Looking for specific info?** Check [Reference](reference/)

**→ Want to understand deeply?** Read [Explanations](explanation/)
