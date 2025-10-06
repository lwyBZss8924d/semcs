# Documentation Inventory Report

**Date:** 2025-10-06
**Total Files Analyzed:** 38 markdown files

---

## Executive Summary

This report provides a comprehensive inventory of all documentation in `/Users/mikerenwick/Documents/GitHub/ck/docs/`, including section-by-section listings, duplicate content identification, Divio framework categorization, and recommendations.

### Key Findings

1. **Significant Content Duplication**: Multiple files contain overlapping information about search modes, CLI usage, and MCP setup
2. **Organization Structure**: Documentation follows Divio framework with dedicated directories, but some content is misplaced
3. **For Humans vs For Agents Separation**: Good high-level separation exists, but content duplicates between these sections
4. **Design Documents**: Three technical design documents exist that should potentially be archived or moved

---

## File-by-File Analysis

### Root Level

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/index.md`
**Sections:**
- # ck
- ## What it does
- ## Quick start
- ## Documentation (with Divio grid)
- ## AI Integration

**Content Type:** Overview/Landing page
**Divio Category:** Reference (landing page)
**Issues:** None - well-structured landing page
**Duplicates:** Links to other pages, no duplication

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/copy-test.md`
**Sections:**
- # Code Copy Test Page
- ## Basic Code Block
- ## Multi-line Code Block
- ## JSON Example
- ## Long Code Block
- ## Inline Code
- ## Instructions
- ## Features

**Content Type:** Test/Demo page
**Divio Category:** N/A (Test file)
**Issues:** Should be removed - test file not needed in production docs
**Duplicates:** None

---

### Design Documents (Root Level)

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/MCP_DEAD_CODE_INTENT.md`
**Sections:**
- # MCP Dead Code Intent Documentation
- ## Session Management
- ### SearchSession Fields
- ### Session Cleanup
- ### Session Statistics
- ## Implementation Timeline
- ## When These Will Be Activated
- ## Why Not Remove Them?
- ## Related Files
- ## Activation Checklist

**Content Type:** Technical design/architecture
**Divio Category:** Explanation (design decisions)
**Issues:** Should be in `/docs/explanation/` or archived
**Duplicates:** None - unique technical content

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/MCP_PAGINATION_DESIGN.md`
**Sections:**
- # MCP Pagination Design
- ## Phase 0: Design & Request/Response Shapes
- ### Enhanced Request Parameters
- ### Enhanced Response Format
- ### Cursor Format
- ## Phase 1: SearchSession Infrastructure
- ## Phase 2: Handler Updates
- ## Phase 3: CLI Separation
- ## Phase 4: Documentation Updates
- ## Phase 5: Testing Strategy

**Content Type:** Technical design
**Divio Category:** Explanation (architecture)
**Issues:** Design doc - should be in `/docs/explanation/` or archived
**Duplicates:** Pagination content duplicated in MCP API reference

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/QUERY_BASED_CHUNKING.md`
**Sections:**
- # Query-Based Chunking Overview
- ## Capture Conventions
- ### Current Query Coverage
- ## Runtime Overrides
- ## Chunk Metadata
- ## Testing
- ## Next Steps

**Content Type:** Technical design
**Divio Category:** Explanation (architecture)
**Issues:** Design doc - should be in `/docs/explanation/` or moved to reference
**Duplicates:** Chunking content also in explanation/semantic-search.md

---

### /for-humans/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/index.md`
**Sections:**
- # For Human Developers
- ## Start here
- ## Common tasks
- ## Reference

**Content Type:** Navigation index
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/quickstart.md`
**Sections:**
- # Quick Start
- ## Install
- ## Your first search
- ## Understanding results
- ## Try more searches
- ## Traditional grep still works
- ## What just happened?
- ## Next steps

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅ (Learning-oriented)
**Issues:** None
**Duplicates:**
- Overlaps with `/tutorials/quick-start.md` (MAJOR DUPLICATE)
- Installation instructions duplicated in `/tutorials/installation.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/tui.md`
**Sections:**
- # Interactive TUI
- ## Launch
- ## Basic navigation
- ## Search modes
- ## Preview modes
- ## Full-file mode
- ## Quick workflows
- ## Tips
- ## All keyboard shortcuts
- ## Next steps

**Content Type:** Tutorial/Reference hybrid
**Divio Category:** **How-To** (Task-oriented)
**Issues:** Mix of tutorial and reference content
**Duplicates:**
- Overlaps with `/tutorials/first-tui-session.md` (MAJOR DUPLICATE)
- Overlaps with `/reference/tui.md` (MAJOR DUPLICATE)
- TUI keyboard shortcuts duplicated across 3 files

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/search-modes.md`
**Sections:**
- # Search Modes
- ## Quick guide
- ## Semantic search
- ## Regex search
- ## Hybrid search
- ## Choosing the right mode

**Content Type:** Explanation
**Divio Category:** **Explanation** (Understanding-oriented)
**Issues:** Should be in `/explanation/`
**Duplicates:**
- Duplicates content in `/explanation/search-modes.md` (COMPLETE OVERLAP - different versions)
- Search mode comparison also in CLI reference

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/find-patterns.md`
**Sections:**
- # Find Common Patterns
- ## Error handling
- ## Authentication & Authorization
- ## Database queries
- ## Configuration
- ## Async & Concurrency
- ## Caching
- ## HTTP & API
- ## Testing
- ## Performance
- ## Security
- ## Logging
- ## Tips

**Content Type:** How-to recipes
**Divio Category:** **How-To** ✅ (Problem-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/how-to/find-patterns.md` (COMPLETE DUPLICATE)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/cli-reference.md`
**Sections:**
- # CLI Reference
- ## Basic syntax
- ## Search modes
- ## Interactive mode
- ## Output formats
- ## Context control
- ## File filtering
- ## Index management
- ## Model selection
- ## MCP server
- ## Help & version
- ## Environment variables
- ## Exit codes
- ## Examples

**Content Type:** Reference
**Divio Category:** **Reference** ✅ (Information-oriented)
**Issues:** None - appropriate location
**Duplicates:**
- Duplicates `/reference/cli.md` (COMPLETE DUPLICATE - different formatting)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/configuration.md`
**Sections:**
- # Configuration
- ## .ckignore
- ## Include exceptions
- ## How exclusions work
- ## Common patterns
- ## Environment variables
- ## Configuration precedence
- ## Tuning search
- ## Performance tuning
- ## Tips

**Content Type:** How-to/Reference hybrid
**Divio Category:** **How-To** (Task-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/how-to/configuration.md` (COMPLETE DUPLICATE)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/editor-integration.md`
**Sections:**
- # Editor Integration
- ## VS Code
- ## Vim / Neovim
- ## Emacs
- ## JetBrains IDEs
- ## Sublime Text
- ## Shell aliases
- ## Tips

**Content Type:** How-to
**Divio Category:** **How-To** ✅ (Task-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/how-to/editor-integration.md` (COMPLETE DUPLICATE)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-humans/large-codebases.md`
**Sections:**
- # Large Codebases
- ## What is "large"?
- ## First-time indexing
- ## Exclude unnecessary files
- ## Search performance
- ## Index management
- ## Memory usage
- ## Real-world examples
- ## Benchmarking
- ## Tips
- ## Troubleshooting

**Content Type:** How-to
**Divio Category:** **How-To** ✅ (Task-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/how-to/large-codebases.md` (COMPLETE DUPLICATE)

---

### /for-agents/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-agents/index.md`
**Sections:**
- # For Humans Using AI Tools
- ## Start here
- ## Integration guides
- ## Tools available

**Content Type:** Navigation index
**Divio Category:** N/A (Navigation)
**Issues:** Title says "For Humans Using AI Tools" which is confusing
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-agents/mcp-quickstart.md`
**Sections:**
- # MCP Quick Start
- ## What is MCP?
- ## Claude Desktop setup
- ## Command-line testing
- ## Available tools
- ## Integration patterns
- ## Best practices
- ## Debugging
- ## Tips

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅ (Learning-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/ai-integration/mcp-quickstart.md` (COMPLETE DUPLICATE)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-agents/examples.md`
**Sections:**
- # Agent Examples
- ## Code review assistant
- ## Documentation generator
- ## Refactoring assistant
- ## Security audit
- ## Onboarding assistant
- ## Bug investigation
- ## Migration planner
- ## Test coverage analyzer
- ## API design reviewer
- ## Tips for agents

**Content Type:** How-to examples
**Divio Category:** **How-To** (Task-oriented)
**Issues:** None
**Duplicates:**
- Duplicates `/ai-integration/examples.md` (COMPLETE DUPLICATE)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/for-agents/setup-guides.md`
**Sections:**
- # Setup Guides
- ## Claude Desktop
- ## Claude Code (VS Code Extension)
- ## LangChain
- ## AutoGPT
- ## GPT Engineer
- ## Custom MCP client
- ## Cursor IDE
- ## Tips

**Content Type:** How-to
**Divio Category:** **How-To** ✅ (Task-oriented)
**Issues:** None
**Duplicates:** None (unique content)

---

### /tutorials/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/tutorials/index.md`
**Sections:**
- # Tutorials
- ## Tutorial Path
- ## What You'll Learn
- ## Next Steps

**Content Type:** Navigation index with styled cards
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/tutorials/installation.md`
**Sections:**
- # Installation
- ## Quick Install
- ## Installation Methods
- ## Platform-Specific Instructions
- ## Verify Installation
- ## Updating
- ## Uninstalling
- ## System Requirements
- ## Configuration
- ## Troubleshooting

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅ (Learning-oriented)
**Issues:** None
**Duplicates:**
- Installation instructions overlap with `/for-humans/quickstart.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/tutorials/quick-start.md`
**Sections:**
- # Quick Start
- ## Install ck
- ## Your First Semantic Search
- ## Understanding Results
- ## Try Different Searches
- ## Traditional Grep Still Works
- ## How It Works
- ## Next Steps
- ## Troubleshooting

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅ (Learning-oriented)
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/quickstart.md` (same content, slightly different formatting)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/tutorials/first-tui-session.md`
**Sections:**
- # Your First TUI Session
- ## Step 1: Launch TUI
- ## Step 2: Your First Search
- ## Step 3: Navigate Results
- ## Step 4: Cycle Preview Modes
- ## Step 5: Switch Search Modes
- ## Step 6: Modify Your Search
- ## Step 7: Open in Editor
- ## Step 8: Full-File Mode Deep Dive
- ## Step 9: Practice Workflow
- ## Common Workflows
- ## Keyboard Reference
- ## Tips & Tricks
- ## What You've Learned

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅ (Learning-oriented)
**Issues:** None
**Duplicates:**
- Overlaps significantly with `/for-humans/tui.md`

---

### /how-to/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/index.md`
**Sections:**
- # How-To Guides
- ## Browse by Category
- ## Quick Problem Solver
- ## Coming Soon

**Content Type:** Navigation index with styled cards
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/ai-integration.md`
**Sections:**
- # AI Integration
- ## MCP Server
- ## Claude Desktop Setup
- ## Cursor Setup
- ## Windsurf Setup
- ## Other MCP Clients
- ## Available Tools (semantic_search, regex_search, hybrid_search, index_status, reindex, health_check)
- ## Pagination
- ## Example Agent Usage
- ## JSONL Output (Custom Workflows)
- ## Configuration
- ## Troubleshooting
- ## Security Considerations

**Content Type:** How-to
**Divio Category:** **How-To** ✅ (Task-oriented)
**Issues:** Very comprehensive, overlaps with reference material
**Duplicates:**
- MCP setup duplicated in `/for-agents/mcp-quickstart.md`
- Tool descriptions duplicate `/reference/mcp-api.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/find-patterns.md`
**Sections:**
(Same as for-humans/find-patterns.md)

**Content Type:** How-to
**Divio Category:** **How-To** ✅
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/find-patterns.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/large-codebases.md`
**Sections:**
(Same as for-humans/large-codebases.md)

**Content Type:** How-to
**Divio Category:** **How-To** ✅
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/large-codebases.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/editor-integration.md`
**Sections:**
(Same as for-humans/editor-integration.md)

**Content Type:** How-to
**Divio Category:** **How-To** ✅
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/editor-integration.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/how-to/configuration.md`
**Sections:**
(Same as for-humans/configuration.md)

**Content Type:** How-to
**Divio Category:** **How-To** ✅
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/configuration.md`

---

### /explanation/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/explanation/index.md`
**Sections:**
- # Explanation
- ## Understanding ck's Design
- ## How It All Works (workflow diagram)
- ## Key Concepts
- ## Design Decisions
- ## Further Reading

**Content Type:** Navigation index with explanatory content
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/explanation/semantic-search.md`
**Sections:**
- # How Semantic Search Works
- ## Overview
- ## The Technology Stack (Tree-sitter, Chunking, Embedding Generation, Vector Storage, Similarity Search)
- ## Embedding Models
- ## Similarity Search Process
- ## Code Chunking Deep Dive
- ## Performance Characteristics
- ## Accuracy and Relevance
- ## Local Processing Architecture
- ## Comparison with Other Approaches
- ## Advanced Topics
- ## Limitations and Considerations
- ## Future Developments
- ## Summary

**Content Type:** Explanation
**Divio Category:** **Explanation** ✅ (Understanding-oriented)
**Issues:** None - excellent deep-dive
**Duplicates:**
- Chunking content overlaps with `QUERY_BASED_CHUNKING.md`
- Some overlap with index.md workflow description

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/explanation/search-modes.md`
**Sections:**
- # Search Modes
- ## Semantic Search (`--sem`)
- ## Lexical Search (`--lex`)
- ## Regex Search (default)
- ## Hybrid Search (`--hybrid`)
- ## Choosing the Right Mode
- ## Performance Characteristics
- ## Advanced Options
- ## Tips & Tricks

**Content Type:** Explanation
**Divio Category:** **Explanation** ✅ (Understanding-oriented)
**Issues:** None
**Duplicates:**
- MAJOR overlap with `/for-humans/search-modes.md` (different content versions!)
- Search mode comparison table duplicated in CLI reference

---

### /reference/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/reference/index.md`
**Sections:**
- # Reference
- ## Quick Reference
- ## Reference Categories
- ## Search This Reference

**Content Type:** Navigation index
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/reference/cli.md`
**Sections:**
- # CLI Reference
- ## Search Modes (Semantic, Lexical, Hybrid)
- ## Result Filtering
- ## Output Formats
- ## Interactive Mode
- ## MCP Server Mode
- ## Index Management
- ## Grep-Compatible Options
- ## Advanced Features
- ## Common Usage Examples
- ## Troubleshooting
- ## Related Documentation

**Content Type:** Reference
**Divio Category:** **Reference** ✅ (Information-oriented)
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-humans/cli-reference.md` (slightly different formatting)

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/reference/tui.md`
**Sections:**
- # TUI (Interactive) Mode
- ## Launch TUI
- ## Interface Overview
- ## Keyboard Shortcuts
- ## Preview Modes (Chunks, Heatmap, Full File)
- ## Search Modes
- ## Tips & Tricks
- ## Configuration
- ## Troubleshooting
- ## Advanced Features

**Content Type:** Reference
**Divio Category:** **Reference** ✅ (Information-oriented)
**Issues:** None
**Duplicates:**
- Overlaps with `/for-humans/tui.md` and `/tutorials/first-tui-session.md`
- Keyboard shortcuts table duplicated across files

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/reference/mcp-api.md`
**Sections:**
- # MCP API Reference
- ## Overview
- ## Server Setup
- ## Available Tools (semantic_search, regex_search, hybrid_search, index_status, reindex, health_check)
- ## Pagination
- ## Error Handling
- ## Best Practices
- ## Example Workflows
- ## Security Considerations
- ## Troubleshooting

**Content Type:** Reference
**Divio Category:** **Reference** ✅ (Information-oriented)
**Issues:** None - comprehensive API reference
**Duplicates:**
- Tool descriptions overlap with `/how-to/ai-integration.md`
- Pagination documentation duplicates design in `MCP_PAGINATION_DESIGN.md`

---

### /ai-integration/

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/ai-integration/index.md`
**Sections:**
- # AI Integration
- ## Supported AI Tools
- ## Available Tools
- ## Quick Start
- ## Example Workflows
- ## Get Started

**Content Type:** Navigation/landing page
**Divio Category:** N/A (Navigation)
**Issues:** None
**Duplicates:** None

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/ai-integration/mcp-quickstart.md`
**Sections:**
(Same as for-agents/mcp-quickstart.md)

**Content Type:** Tutorial
**Divio Category:** **Tutorial** ✅
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-agents/mcp-quickstart.md`

---

#### `/Users/mikerenwick/Documents/GitHub/ck/docs/ai-integration/examples.md`
**Sections:**
(Same as for-agents/examples.md)

**Content Type:** How-to examples
**Divio Category:** **How-To**
**Issues:** None
**Duplicates:**
- COMPLETE DUPLICATE of `/for-agents/examples.md`

---

## Duplicate Content Analysis

### Complete Duplicates (Same Content, Different Locations)

1. **Quick Start Guide**
   - `/for-humans/quickstart.md`
   - `/tutorials/quick-start.md`
   - **Recommendation:** Keep only `/tutorials/quick-start.md` (proper Divio location)

2. **Find Patterns**
   - `/for-humans/find-patterns.md`
   - `/how-to/find-patterns.md`
   - **Recommendation:** Keep only `/how-to/find-patterns.md` (proper Divio location)

3. **Configuration**
   - `/for-humans/configuration.md`
   - `/how-to/configuration.md`
   - **Recommendation:** Keep only `/how-to/configuration.md` (proper Divio location)

4. **Editor Integration**
   - `/for-humans/editor-integration.md`
   - `/how-to/editor-integration.md`
   - **Recommendation:** Keep only `/how-to/editor-integration.md` (proper Divio location)

5. **Large Codebases**
   - `/for-humans/large-codebases.md`
   - `/how-to/large-codebases.md`
   - **Recommendation:** Keep only `/how-to/large-codebases.md` (proper Divio location)

6. **CLI Reference**
   - `/for-humans/cli-reference.md`
   - `/reference/cli.md`
   - **Recommendation:** Keep only `/reference/cli.md` (proper Divio location)

7. **MCP Quick Start**
   - `/for-agents/mcp-quickstart.md`
   - `/ai-integration/mcp-quickstart.md`
   - **Recommendation:** Keep `/ai-integration/mcp-quickstart.md`, create symlink or redirect from for-agents

8. **Examples (AI Agent)**
   - `/for-agents/examples.md`
   - `/ai-integration/examples.md`
   - **Recommendation:** Keep `/ai-integration/examples.md`, create symlink or redirect from for-agents

### Partial Duplicates (Overlapping Content)

1. **Search Modes**
   - `/for-humans/search-modes.md`
   - `/explanation/search-modes.md`
   - **Issue:** Different versions with different content! One has lexical search, other doesn't
   - **Recommendation:** Consolidate into `/explanation/search-modes.md` (Divio: Explanation)

2. **TUI Documentation**
   - `/for-humans/tui.md`
   - `/tutorials/first-tui-session.md`
   - `/reference/tui.md`
   - **Issue:** Three different treatments of same topic
   - **Recommendation:**
     - Keep `/tutorials/first-tui-session.md` for learning
     - Keep `/reference/tui.md` for keyboard shortcuts
     - Remove `/for-humans/tui.md`

3. **Chunking**
   - `/QUERY_BASED_CHUNKING.md` (design doc)
   - `/explanation/semantic-search.md` (has chunking section)
   - **Recommendation:** Move design doc to `/docs/explanation/chunking.md` or archive it

4. **Pagination**
   - `/MCP_PAGINATION_DESIGN.md` (design doc)
   - `/reference/mcp-api.md` (has pagination section)
   - **Recommendation:** Archive design doc, keep only implemented version in reference

5. **MCP Setup/Tools**
   - `/how-to/ai-integration.md`
   - `/reference/mcp-api.md`
   - `/for-agents/mcp-quickstart.md`
   - **Issue:** Tool descriptions repeated across multiple files
   - **Recommendation:**
     - Reference material in `/reference/mcp-api.md`
     - How-to guide in `/how-to/ai-integration.md`
     - Tutorial in `/ai-integration/mcp-quickstart.md`
     - Ensure they reference each other appropriately

---

## Divio Framework Categorization

### Tutorials (Learning-oriented)
✅ **Correctly Placed:**
- `/tutorials/quick-start.md`
- `/tutorials/installation.md`
- `/tutorials/first-tui-session.md`
- `/ai-integration/mcp-quickstart.md`

❌ **Misplaced (in for-humans or for-agents):**
- `/for-humans/quickstart.md` → DELETE (duplicate)
- `/for-agents/mcp-quickstart.md` → DELETE (duplicate)

### How-To Guides (Task-oriented)
✅ **Correctly Placed:**
- `/how-to/find-patterns.md`
- `/how-to/large-codebases.md`
- `/how-to/editor-integration.md`
- `/how-to/configuration.md`
- `/how-to/ai-integration.md`
- `/ai-integration/examples.md`

❌ **Misplaced (in for-humans):**
- `/for-humans/find-patterns.md` → DELETE (duplicate)
- `/for-humans/configuration.md` → DELETE (duplicate)
- `/for-humans/editor-integration.md` → DELETE (duplicate)
- `/for-humans/large-codebases.md` → DELETE (duplicate)
- `/for-humans/tui.md` → DELETE (covered by tutorial + reference)
- `/for-agents/examples.md` → DELETE (duplicate)

### Reference (Information-oriented)
✅ **Correctly Placed:**
- `/reference/cli.md`
- `/reference/tui.md`
- `/reference/mcp-api.md`

❌ **Misplaced:**
- `/for-humans/cli-reference.md` → DELETE (duplicate)

### Explanation (Understanding-oriented)
✅ **Correctly Placed:**
- `/explanation/semantic-search.md`
- `/explanation/search-modes.md`

❌ **Misplaced:**
- `/for-humans/search-modes.md` → DELETE (move unique content to explanation version)
- `/MCP_DEAD_CODE_INTENT.md` → MOVE to `/explanation/` or ARCHIVE
- `/MCP_PAGINATION_DESIGN.md` → ARCHIVE (already implemented)
- `/QUERY_BASED_CHUNKING.md` → MOVE to `/explanation/chunking.md` or ARCHIVE

---

## Recommendations

### Immediate Actions (High Priority)

1. **Delete Complete Duplicates in /for-humans/:**
   - `/for-humans/quickstart.md` (keep `/tutorials/quick-start.md`)
   - `/for-humans/find-patterns.md` (keep `/how-to/find-patterns.md`)
   - `/for-humans/configuration.md` (keep `/how-to/configuration.md`)
   - `/for-humans/editor-integration.md` (keep `/how-to/editor-integration.md`)
   - `/for-humans/large-codebases.md` (keep `/how-to/large-codebases.md`)
   - `/for-humans/cli-reference.md` (keep `/reference/cli.md`)
   - `/for-humans/tui.md` (covered by tutorial + reference)

2. **Delete Complete Duplicates in /for-agents/:**
   - `/for-agents/mcp-quickstart.md` (keep `/ai-integration/mcp-quickstart.md`)
   - `/for-agents/examples.md` (keep `/ai-integration/examples.md`)

3. **Delete Test File:**
   - `/docs/copy-test.md`

4. **Consolidate Search Modes:**
   - Compare `/for-humans/search-modes.md` and `/explanation/search-modes.md`
   - Merge any unique content into `/explanation/search-modes.md`
   - Delete `/for-humans/search-modes.md`

### Medium Priority Actions

5. **Handle Design Documents:**
   - Archive or move `/MCP_DEAD_CODE_INTENT.md` to `/explanation/` if still relevant
   - Archive `/MCP_PAGINATION_DESIGN.md` (feature is implemented)
   - Move `/QUERY_BASED_CHUNKING.md` to `/explanation/chunking.md` or archive

6. **Update /for-humans/ Index:**
   - Revise `/for-humans/index.md` to point to canonical locations
   - Consider whether `/for-humans/` should exist at all or just be a landing page with links

7. **Update /for-agents/ Index:**
   - Revise to point to `/ai-integration/` content
   - Fix confusing title "For Humans Using AI Tools"

### Low Priority Actions

8. **Cross-reference Optimization:**
   - Ensure tutorials link to how-to guides for next steps
   - Ensure how-to guides link to reference for detailed specs
   - Ensure explanation pages link to tutorials for getting started

9. **Navigation Improvement:**
   - Consider whether `/for-humans/` and `/for-agents/` directories add value or create confusion
   - Alternative: Use categories/tags instead of separate directory structures

---

## Statistics

- **Total Files:** 38 markdown files
- **Complete Duplicates:** 8 pairs (16 files)
- **Partial Duplicates:** 5 instances
- **Test/Temporary Files:** 1
- **Design Documents:** 3
- **Index/Navigation Files:** 7

**After Cleanup:**
- **Files to Delete:** 9 complete duplicates + 1 test file = **10 files**
- **Files to Archive:** 3 design docs (optional)
- **Files to Consolidate:** 1 (search-modes.md)
- **Remaining Files:** ~24-25 core documentation files

---

## Content Quality Notes

### Strengths
1. Comprehensive coverage of all ck features
2. Good separation of human vs agent use cases conceptually
3. Well-written tutorials with step-by-step instructions
4. Excellent reference documentation (MCP API, CLI)
5. Strong how-to guides with practical examples

### Weaknesses
1. Massive duplication between `/for-humans/` and Divio structure
2. Two different versions of search-modes.md with conflicting content
3. Design documents in root should be archived or moved
4. Unclear value of `/for-humans/` and `/for-agents/` directories vs. proper Divio structure
5. Test file (`copy-test.md`) should not be in production docs

---

## Final Recommendation

**Primary Action:** Consolidate documentation by:
1. Deleting all duplicates in `/for-humans/` and `/for-agents/`
2. Keeping only the properly categorized Divio structure (tutorials, how-to, reference, explanation)
3. Converting `/for-humans/` and `/for-agents/` to simple landing pages with links to appropriate sections
4. Archiving or properly placing design documents

**Rationale:** The current structure with both `/for-humans/` + Divio directories creates unnecessary duplication and maintenance burden. The Divio framework alone provides sufficient organization.
