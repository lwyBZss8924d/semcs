# cs (semcs) - Semantic Code Search for Claude & Codex

semcs [sem-cs] - CODE SEARCH hybrid semantic retrieval and hybrid grep for Claude & Codex.

forked from [BeaconBay/cs](https://github.com/BeaconBay/ck)

**cs (semcs)** finds code by meaning, not just keywords. It's grep that understands what you're looking for — search for "error handling" and find try/catch blocks, error returns, and exception handling code even when those exact words aren't present.

## 🚀 Quick Start

```shell
# Install from crates.io
cargo install cc-search

# Just search — cc builds and updates indexes automatically
cs --sem "error handling" src/
cs --sem "authentication logic" src/
cs --sem "database connection pooling" src/

# Traditional grep-compatible search still works
cs -n "TODO" *.rs
cs -R "TODO|FIXME" .

# Combine both: semantic relevance + keyword filtering
cs --hybrid "connection timeout" src/
```

## ✨ Headline Features

### 🤖 **AI Agent Integration (MCP Server)**

Connect cs directly to Claude Desktop, Cursor, or any MCP-compatible AI client for seamless code search integration:

```shell
# Start MCP server for AI agent integration
cs --serve
```

**Claude Desktop Setup:**

```shell
# Install via Claude Code CLI (recommended)
claude mcp add cs-search -s user -- cs --serve

# Note: You may need to restart Claude Code after installation
# Verify installation with:
claude mcp list  # or use /mcp in Claude Code
```

**Manual Configuration (alternative):**

```json
{
  "mcpServers": {
    "cs": {
      "command": "cs",
      "args": ["--serve"],
      "cwd": "/path/to/your/codebase"
    }
  }
}
```

**Tool Permissions:** When prompted by Claude Code, approve permissions for cs-search tools (semantic_search, regex_search, hybrid_search, etc.)

**Available MCP Tools:**

- `semantic_search` - Find code by meaning using embeddings
- `regex_search` - Traditional grep-style pattern matching
- `hybrid_search` - Combined semantic and keyword search
- `index_status` - Check indexing status and metadata
- `reindex` - Force rebuild of search index
- `health_check` - Server status and diagnostics

**Built-in Pagination:** Handles large result sets gracefully with page_size controls, cursors, and snippet length management.

### 🎨 **Interactive TUI (Terminal User Interface)**

Launch an interactive search interface with real-time results and multiple preview modes:

```shell
# Start TUI for current directory
cs --tui

# Start with initial query
cs --tui "error handling"
```

**Features:**

- **Multiple Search Modes**: Toggle between Semantic, Regex, and Hybrid search with `Tab`
- **Preview Modes**: Switch between Heatmap, Syntax highlighting, and Chunk view with `Ctrl+V`
- **View Options**: Toggle between snippet and full-file view with `Ctrl+F`
- **Multi-select**: Select multiple files with `Ctrl+Space`, open all in editor with `Enter`
- **Search History**: Navigate with `Ctrl+Up/Down`
- **Editor Integration**: Opens files in `$EDITOR` with line numbers (Vim, VS Code, Cursor, etc.)
- **Progress Tracking**: Live indexing progress with file and chunk counts
- **Config Persistence**: Preferences saved to `~/.config/cc/tui.json`

See [TUI.md](TUI.md) for keyboard shortcuts and detailed usage.

### 🔍 **Semantic Search**

Find code by concept, not keywords. Understands synonyms, related terms, and conceptual similarity:

```shell
# These find related code even without exact keywords:
cs --sem "retry logic"           # finds backoff, circuit breakers
cs --sem "user authentication"   # finds login, auth, credentials
cs --sem "data validation"       # finds sanitization, type checking

# Get complete functions/classes containing matches
cs --sem --full-section "error handling"  # returns entire functions
```

### ⚡ **Drop-in grep Compatibility**

All your muscle memory works. Same flags, same behavior, same output format:

```shell
cs -i "warning" *.log              # Case-insensitive
cs -n -A 3 -B 1 "error" src/       # Line numbers + context
cs -l "error" src/                  # List files with matches only
cs -L "TODO" src/                   # List files without matches
cc -R --exclude "*.test.js" "bug"  # Recursive with exclusions
```

### 🎯 **Hybrid Search**

Combine keyword precision with semantic understanding using Reciprocal Rank Fusion:

```shell
cs --hybrid "async timeout" src/    # Best of both worlds
cs --hybrid --scores "cache" src/   # Show relevance scores with color highlighting
cs --hybrid --threshold 0.02 query  # Filter by minimum relevance
```

### ⚙️ **Automatic Delta Indexing**

Semantic and hybrid searches transparently create and refresh their indexes before running. The first search builds what it needs; subsequent searches only touch files that changed.

### 📁 **Smart File Filtering**

Automatically excludes cache directories, build artifacts, and respects `.gitignore` and `.csignore` files:

```shell
# cs respects multiple exclusion layers (all are additive):
cs "pattern" .                           # Uses .gitignore + .csignore + defaults
cs --no-ignore "pattern" .               # Skip .gitignore (still uses .csignore)
cs --no-csignore "pattern" .             # Skip .csignore (still uses .gitignore)
cs --exclude "dist" --exclude "logs" .   # Add custom exclusions

# .csignore file (created automatically on first index):
# - Excludes images, videos, audio, binaries, archives by default
# - Excludes JSON/YAML config files (issue #27)
# - Uses same syntax as .gitignore (glob patterns, ! for negation)
# - Persists across searches (issue #67)
# - Located at repository root, editable for custom patterns

# Exclusion patterns use .gitignore syntax:
cs --exclude "node_modules" .            # Exclude directory and all contents
cs --exclude "*.test.js" .                # Exclude files matching pattern
cs --exclude "build/" --exclude "*.log" . # Multiple exclusions
# Note: Patterns are relative to the search root
```

**Why .csignore?** While `.gitignore` handles version control exclusions, many files that *should* be in your repo aren't ideal for semantic search. Config files (`package.json`, `tsconfig.json`), images, videos, and data files add noise to search results and slow down indexing. `.csignore` lets you focus semantic search on actual code while keeping everything else in git. Think of it as "what should I search" vs "what should I commit".

## 🛠 Advanced Usage

### AI Agent Integration

#### MCP Server (Recommended)

```python
# Example usage in AI agents
response = await client.call_tool("semantic_search", {
    "query": "authentication logic",
    "path": "/path/to/code",
    "page_size": 25,
    "top_k": 50,           # Limit total results (default: 100 for MCP)
    "snippet_length": 200
})

# Handle pagination
if response["pagination"]["next_cursor"]:
    next_response = await client.call_tool("semantic_search", {
        "query": "authentication logic",
        "path": "/path/to/code",
        "cursor": response["pagination"]["next_cursor"]
    })
```

#### JSONL Output (Custom Workflows)

Perfect structured output for LLMs, scripts, and automation:

```shell
# JSONL format - one JSON object per line (recommended for agents)
cs --jsonl --sem "error handling" src/
cs --jsonl --no-snippet "function" .        # Metadata only
cc --jsonl --topk 5 --threshold 0.7 "auth"  # High-confidence results

# Traditional JSON (single array)
cs --json --sem "error handling" src/ | jq '.file'
```

**Why JSONL for AI agents?**

- ✅ **Streaming friendly**: Process results as they arrive
- ✅ **Memory efficient**: Parse one result at a time
- ✅ **Error resilient**: One malformed line doesn't break entire response
- ✅ **Standard format**: Used by OpenAI API, Anthropic API, and modern ML pipelines

### Search & Filter Options

```shell
# Threshold filtering
cs --sem --threshold 0.7 "query"           # Only high-confidence matches
cs --hybrid --threshold 0.01 "concept"     # Low-confidence (exploration)

# Limit results
cs --sem --topk 5 "authentication patterns"

# Complete code sections
cs --sem --full-section "database queries"  # Complete functions
cs --full-section "class.*Error" src/       # Complete classes (works with regex too)

# Relevance scoring
cs --sem --scores "machine learning" docs/
# [0.847] ./ai_guide.txt: Machine learning introduction...
# [0.732] ./statistics.txt: Statistical learning methods...
```

### Language Coverage

| Language | Indexing | Chunking | AST-aware | Notes |
|----------|----------|----------|-----------|-------|
| Zig | ✅ | ✅ | ✅ | contributed by [@Nevon](https://github.com/Nevon) (PR #72) |

### Model Selection

Choose the right embedding model for your needs:

```shell
# Default: BGE-Small (fast, precise chunking)
cs --index .

# Enhanced: Nomic V1.5 (8K context, optimal for large functions)
cs --index --model nomic-v1.5 .

# Code-specialized: Jina Code (local, optimized for programming languages)
cs --index --model jina-code .

# Jina AI API models (cloud-based, state-of-the-art code embeddings)
export JINA_API_KEY="your_api_key"  # Get free key at https://jina.ai/?sui=apikey

# Hybrid strategy (recommended): index with v4, query with code-1.5b
cs --index --model jina-v4 .
cs --sem --model jina-code-1.5b "your query"

# Or index directly with code models
cs --index --model jina-code-1.5b .
cs --index --model jina-code-0.5b .  # Faster, good quality
```

**Model Comparison:**

| Model | Type | Dimensions | Context | Best For |
|-------|------|------------|---------|----------|
| **`bge-small`** (default) | Local | 384 | 512 tokens | Fast indexing, most code |
| **`nomic-v1.5`** | Local | 768 | 8K tokens | Large functions, better quality |
| **`jina-code`** | Local | 768 | 8K tokens | Code-specialized, offline |
| **`jina-v4`** | API | 1536 | 8K tokens | **Indexing** - handles large files |
| **`jina-code-0.5b`** | API | 896 | 8K tokens | Fast cloud search |
| **`jina-code-1.5b`** | API | 1536 | 8K tokens | **Querying** - code-specialized, NL2Code |

**Jina AI API Models** require `JINA_API_KEY` environment variable. Benefits:

- ✨ **No model downloads** - Zero setup, instant start
- 🎯 **State-of-the-art quality** - Advanced code embeddings (0.5B - 3.8B parameters)
- 🌐 **Cross-language search** - Excellent at finding similar code across languages
- 🔍 **Natural language to code** - Superior understanding of intent ("find error handling")
- ⚡ **Generous free tier** - 500 requests/min, 1M tokens/min
- 🔥 **Hybrid strategy** - Index with v4 + query with code-1.5b (dimension-compatible)

**Why Hybrid Works:** jina-v4 and jina-code-1.5b both output 1536 dimensions, enabling cross-model queries. The system automatically detects dimension compatibility. Index once with v4 (optimized for large files, 8K+ tokens), then query with code-1.5b (optimized for code understanding). Best of both worlds!

See [examples/jina_api_usage.md](examples/jina_api_usage.md) for detailed Jina API documentation.

### Index Management

```shell
# Check index status
cs --status .

# Clean up and rebuild / switch models
cs --clean .
cs --switch-model nomic-v1.5 .
cc --switch-model nomic-v1.5 --force .     # Force rebuild

# Add single file to index
cs --add new_file.rs

# File inspection (analyze chunking and token usage)
cs --inspect src/main.rs
cc --inspect --model bge-small src/main.rs  # Test different models
```

**Interrupting Operations:** Indexing can be safely interrupted with Ctrl+C. The partial index is saved, and the next operation will resume from where it stopped, only processing new or changed files.

## 📚 Language Support

| Language | Indexing | Tree-sitter Parsing | Semantic Chunking |
|----------|----------|-------------------|------------------|
| Python | ✅ | ✅ | ✅ Functions, classes |
| JavaScript/TypeScript | ✅ | ✅ | ✅ Functions, classes, methods |
| Rust | ✅ | ✅ | ✅ Functions, structs, traits |
| Go | ✅ | ✅ | ✅ Functions, types, methods |
| Ruby | ✅ | ✅ | ✅ Classes, methods, modules |
| Haskell | ✅ | ✅ | ✅ Functions, types, instances |
| C# | ✅ | ✅ | ✅ Classes, interfaces, methods |

**Text Formats:** Markdown, JSON, YAML, TOML, XML, HTML, CSS, shell scripts, SQL, log files, config files, and any other text format.

**Smart Binary Detection:** Uses ripgrep-style content analysis, automatically indexing any text file while correctly excluding binary files.

**Unsupported File Types:** Text files with unrecognized extensions (like `.org`, `.adoc`, etc.) are automatically indexed as plain text. cc detects text vs binary based on file contents, not extensions.

## 🏗 Installation

### From crates.io

```shell
cargo install cs-search
```

### From Source

```shell
git clone https://github.com/lwyBZss8924d/semcs
cd cc
cargo install --path cs-cli
```

### Package Managers

```shell
# Currently available:
cargo install semcs    # ✅ Available now via crates.io

# Coming soon:
brew install semcs     # 🚧 In development (use cargo for now)
apt install semcs      # 🚧 In development
```

## 💡 Examples

### Finding Code Patterns

```shell
# Find authentication/authorization code
cs --sem "user permissions" src/
cs --sem "access control" src/
cs --sem "login validation" src/

# Find error handling strategies
cs --sem "exception handling" src/
cs --sem "error recovery" src/
cs --sem "fallback mechanisms" src/

# Find performance-related code
cs --sem "caching strategies" src/
cs --sem "database optimization" src/
cs --sem "memory management" src/
```

### Team Workflows

```shell
# Find related test files
cs --sem "unit tests for authentication" tests/
cs -l --sem "test" tests/           # List test files by semantic content

# Identify refactoring candidates
cs --sem "duplicate logic" src/
cs --sem "code complexity" src/
cs -L "test" src/                   # Find source files without tests

# Security audit
cs --hybrid "password|credential|secret" src/
cs --sem "input validation" src/
```

### Integration Examples

```shell
# Git hooks
git diff --name-only | xargs cs --sem "TODO"

# CI/CD pipeline
cs --json --sem "security vulnerability" . | security_scanner.py

# Code review prep
cs --hybrid --scores "performance" src/ > review_notes.txt

# Documentation generation
cs --json --sem "public API" src/ | generate_docs.py
```

## ⚡ Performance

**Field-tested on real codebases:**

- **Indexing:** ~1M LOC in under 2 minutes
- **Search:** Sub-500ms queries on typical codebases
- **Index size:** ~2x source code size with compression
- **Memory:** Efficient streaming for large repositories
- **Token precision:** HuggingFace tokenizers for exact model-specific token counting

## 🔧 Architecture

cs uses a modular Rust workspace:

- **`cs-cli`** - Command-line interface and MCP server
- **`cs-tui`** - Interactive terminal user interface (ratatui-based)
- **`cs-core`** - Shared types, configuration, and utilities
- **`cs-engine`** - Search engine implementations (regex, semantic, hybrid)
- **`cs-index`** - File indexing, hashing, and sidecar management
- **`cs-embed`** - Text embedding providers (FastEmbed, API backends)
- **`cs-ann`** - Approximate nearest neighbor search indices
- **`cs-chunk`** - Text segmentation and language-aware parsing ([query-based chunking](docs/QUERY_BASED_CHUNKING.md))
- **`cs-models`** - Model registry and configuration management

### Index Storage

Example: Indexes are stored in `.cs/` directories alongside your code:

```tree
project/
├── src/
├── docs/
└── .cs/           # Semantic index (can be safely deleted)
    ├── embeddings.json
    ├── ann_index.bin
    └── tantivy_index/
```

The `.cs/` directory is a cache — safe to delete and rebuild anytime.

## 🧪 Testing

```shell
# Run the full test suite
cargo test --workspace

# Test with each feature combination
cargo hack test --each-feature --workspace
```

## 🤝 Contributing

cs is actively developed and welcomes contributions:

1. **Issues:** Report bugs, request features
2. **Code:** Submit PRs for bug fixes, new features
3. **Documentation:** Improve examples, guides, tutorials
4. **Testing:** Help test on different codebases and languages

### Development Setup

```shell
git clone https://github.com/lwyBZss8924d/semcs
cd semcs
cargo build --workspace
cargo test --workspace
./target/debug/cs --index test_files/
./target/debug/cs --sem "test query" test_files/
```

### CI Requirements

Before submitting a PR, ensure your code passes all CI checks:

```shell
# Format code (required)
cargo fmt --all

# Run clippy linter (required - must have no warnings)
cargo clippy --workspace --all-features --all-targets -- -D warnings

# Run tests (required)
cargo test --workspace

# Check minimum supported Rust version (MSRV)
cargo hack check --each-feature --locked --rust-version --workspace
```

The CI pipeline runs on Ubuntu, Windows, and macOS to ensure cross-platform compatibility.

## 🗺 Roadmap

### Current (v0.5+)

- ✅ MCP (Model Context Protocol) server for AI agent integration
- ✅ grep-compatible CLI with semantic search and file listing flags
- ✅ FastEmbed integration with BGE models and enhanced model selection
- ✅ File exclusion patterns and glob support
- ✅ Threshold filtering and relevance scoring with visual highlighting
- ✅ Tree-sitter parsing and intelligent chunking for 7+ languages
- ✅ Complete code section extraction (`--full-section`)
- ✅ Clean stdout/stderr separation for reliable scripting
- ✅ Incremental index updates with hash-based change detection
- ✅ Token-aware chunking with HuggingFace tokenizers
- ✅ Published to crates.io (`cargo install cc-search`)

### Next (v0.6+)

- ✅ [0.6.1] Jina API embeddings model support
- ✅ [0.6.1] Jina API Reranker model support
- ✅ [0.6.1] Configuration file & (cc config) command support
- ✅ [0.6.1] AST search mode support
- ✅ [0.6.1] Hybrid AST search mode support

- 🚧 Package manager distributions (brew, npm, apt)
- 🚧 Enhanced MCP tools (file writing, refactoring assistance)
- 🚧 VS Code extension
- 🚧 JetBrains plugin
- 🚧 Additional more languages chunkers (Java, Swift, etc.)

## ❓ FAQ

**Q: How is this different from grep/ripgrep/silver-searcher?**
A: cs includes all the features of traditional search tools, but adds semantic understanding. Search for "error handling" and find relevant code even when those exact words aren't used.

**Q: Does it work offline?**
A: Yes, completely offline. The embedding model runs locally with no network calls.

**Q: How big are the indexes?**
A: Typically 1-3x the size of your source code. The `.cs/` directory can be safely deleted to reclaim space.

**Q: Is it fast enough for large codebases?**
A: Yes. The first semantic search builds the index automatically; after that only changed files are reprocessed, keeping searches sub-second even on large projects.

**Q: Can I use it in scripts/automation?**
A: Absolutely. The `--json` and `--jsonl` flags provide structured output perfect for automated processing and AI agent integration.

**Q: What about privacy/security?**
A: Everything runs locally. No code or queries are sent to external services. The embedding model is downloaded once and cached locally.

**Q: Where are the embedding models cached?**
A: Models are cached in platform-specific directories:

- Linux/macOS: `~/.cache/cs/models/`
- Windows: `%LOCALAPPDATA%\cs\cache\models\`
- Fallback: `.cs_models/models/` in current directory

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Credits

Built with:

- [ck](https://github.com/BeaconBay/ck) - Original `ck` version 0.5.3
- [Rust](https://rust-lang.org) - Systems programming language
- [FastEmbed](https://github.com/Anush008/fastembed-rs) - Fast text embeddings
- [Tantivy](https://github.com/quickwit-oss/tantivy) - Full-text search engine
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [JinaAI](https://huggingface.co/jinaai) - Text embeddings and reranking models
- [ast-grep](https://github.com/ast-grep/ast-grep) - AST structural search engine

Inspired by the need for better code search tools in the age of AI-assisted development.

---

**Start finding code by what it does, not what it says.**

```shell
cargo install semcs-search
cs --sem "the code you're looking for"
```
