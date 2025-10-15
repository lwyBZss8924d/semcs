# cs - Semantic Code Search for VS Code & Cursor

Brings the power of `cs` semantic code search directly into Visual Studio Code and Cursor.

## Features

### Search Capabilities

- **Semantic Search**: Find code by meaning, not just keywords
- **Hybrid Search** (Default): Combine semantic understanding with keyword precision
- **Regex Search**: Traditional pattern matching when you need it
- **Automatic Reranking**: Cross-encoder reranking enabled by default for best relevance (⚡ RERANK badge)
- **Smart Context**: 2 lines of context before/after matches for better understanding

### UI & UX

- **Crisp, Clean Interface**: Terminal-inspired monospace design
- **Live Results**: Search as you type with 300ms debouncing (TUI-style)
- **Visual Score Indicators**: Color-coded relevance scores (cyan/blue/yellow/orange) with visual bars
- **Line-by-Line Previews**: Context-aware preview with accurate line numbering
- **Relative Paths**: Clean file paths relative to workspace root
- **Keyboard Navigation**: ↑/↓ to navigate, Enter to open, Esc to reset
- **Match Highlighting**: Visual distinction for matching lines in previews

### Integration

- **Direct File Navigation**: Click any result to jump to exact location with brief highlight
- **Real-time Index Status**: Green dot when indexed, yellow when needs indexing
- **One-Click Reindexing**: Rebuild workspace index with progress notifications
- **Editor Integration**: "Search Selection" command for quick searches

## Requirements

- [cs](https://github.com/lwyBZss8924d/semcs) must be installed and available in your PATH
- Install with: `cargo install cs-search`

## Usage

### Commands

- `cs: Search` (`Ctrl+Shift+;` / `Cmd+Shift+;`) - Open search panel
- `cs: Search Selection` (`Ctrl+Shift+'` / `Cmd+Shift+'`) - Search selected text
- `cs: Reindex` - Force rebuild of search index

### Search Modes

- **Hybrid** - Combines semantic and keyword search (default)
- **Semantic** - Find code by concept and meaning
- **Regex** - Traditional grep-style pattern matching

### Keyboard Navigation

- `↑/↓` - Navigate results
- `Enter` - Open selected result or trigger search
- `Esc` - Return focus to search input

## Extension Settings

- `cs.mode` - Integration mode: `cli` (default) or `mcp`
- `cs.cliPath` - Path to cs binary (default: `cs`)
- `cs.defaultMode` - Default search mode: `hybrid`, `semantic`, or `regex`
- `cs.topK` - Maximum number of results (default: 100)
- `cs.threshold` - Minimum relevance threshold (default: 0.02)
- `cs.pageSize` - Results per page (default: 50)

## Installation

### For Cursor

```bash
cd cs-vscode
./install-cursor.sh
```

Then restart Cursor.

### For VS Code

```bash
cd cs-vscode
npm install
npm run compile
code --install-extension . --force
```

## Development

### Building

```bash
cd cs-vscode
npm install
npm run compile
```

### Testing

1. Open in VS Code
2. Press F5 to launch Extension Development Host
3. Test search functionality

### Packaging

```bash
npm run package
```

This creates a `.vsix` file you can install locally or publish to the marketplace.

## Roadmap

- [x] Phase 1: CLI mode + sidebar UI
- [x] Automatic reranking for better relevance
- [x] Visual score indicators and crisp UI
- [x] Line numbers and match highlighting
- [x] Relative path display
- [ ] Phase 2: MCP server integration for persistent connections
- [ ] Phase 3: Full syntax highlighting in previews
- [ ] Phase 4: Streaming results for large codebases
- [ ] Phase 5: Multi-workspace support

## License

Same as cs - Apache 2.0 or MIT

## More Information

- [cs on GitHub](https://github.com/lwyBZss8924d/semcs)
- [Report Issues](https://github.com/lwyBZss8924d/semcs/issues)
