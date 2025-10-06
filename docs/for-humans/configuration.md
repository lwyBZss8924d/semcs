---
layout: default
title: Configuration
parent: For Humans
nav_order: 5
---

# Configuration
{: .no_toc }

Customize ck's behavior with settings and ignore files.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## .ckignore

Control which files to exclude from search.

### Basic usage

Create `.ckignore` in your project root:

```gitignore
# Dependencies
node_modules/
vendor/
.cargo/

# Build outputs
target/
dist/
build/
*.o
*.so

# Generated files
*.min.js
*.bundle.js
package-lock.json

# Docs
docs/
*.md
```

{: .note }
`.ckignore` uses the same syntax as `.gitignore`

---

## Include exceptions

Use `!` to include files that would otherwise be excluded:

```gitignore
# Exclude all JSON
*.json

# But keep configs
!config.json
!settings.json

# Exclude docs
docs/

# But keep API reference
!docs/api/
```

---

## How exclusions work

By default, ck respects:
1. **`.gitignore`** - Your repo's ignore rules
2. **`.ckignore`** - Search-specific exclusions
3. **Built-in ignores** - Common binaries, images, etc.

### Skip .gitignore

```bash
ck --no-ignore --sem "config" .
```

Search all files, even those in `.gitignore`.

### Skip .ckignore

```bash
ck --no-ckignore --sem "test" .
```

Only respect `.gitignore`, not `.ckignore`.

### Skip both

```bash
ck --no-ignore --no-ckignore --sem "error" .
```

Search absolutely everything.

---

## Common patterns

### Exclude by file type

```gitignore
# Data files
*.json
*.yaml
*.xml
*.csv

# Compiled
*.pyc
*.class
*.o

# Minified
*.min.js
*.min.css
```

### Exclude by directory

```gitignore
# Test data
test_fixtures/
__fixtures__/

# Third-party
third_party/
external/

# Generated
.generated/
auto/
```

### Language-specific

**JavaScript/TypeScript:**
```gitignore
node_modules/
.next/
.nuxt/
dist/
coverage/
*.map
```

**Rust:**
```gitignore
target/
Cargo.lock
```

**Python:**
```gitignore
__pycache__/
*.pyc
.venv/
venv/
.pytest_cache/
```

**Go:**
```gitignore
vendor/
*.test
*.out
```

---

## Environment variables

Configure ck globally via environment:

### CK_MODEL

Default embedding model for semantic search.

```bash
export CK_MODEL=large
```

**Options:**
- `default` - Fast, good accuracy (default)
- `large` - Slower, better accuracy

### CK_WORKERS

Number of parallel workers for indexing.

```bash
export CK_WORKERS=8
```

Defaults to number of CPU cores.

### CK_INDEX_PATH

Custom location for `.ck/` index directory.

```bash
export CK_INDEX_PATH=/tmp/ck-indices
```

Useful for shared indices or custom cache locations.

### EDITOR

Editor for TUI's "open file" action.

```bash
export EDITOR=nvim
```

Falls back to `$VISUAL`, then `vi`.

---

## Configuration precedence

From highest to lowest priority:

1. **Command-line flags** - `ck --model large`
2. **Environment variables** - `export CK_MODEL=large`
3. **Defaults** - Built-in sensible defaults

---

## Tuning search

### Semantic search threshold

Lower = more results, higher = fewer but more relevant.

```bash
# Cast wide net (default: 0.6)
ck --sem "auth" --threshold 0.5 .

# Only best matches
ck --sem "auth" --threshold 0.8 .
```

### Result limits

Control how many results to show:

```bash
# Top 10 only
ck --sem "cache" --topk 10 src/

# All matches (default: 100)
ck --sem "cache" --topk 999999 src/
```

---

## Performance tuning

### Faster indexing

```bash
# Use all CPU cores
export CK_WORKERS=16

# Or set per-command (future feature)
# ck --workers 16 --reindex .
```

### Smaller indices

Add more patterns to `.ckignore`:

```gitignore
# Skip test fixtures (usually large)
tests/fixtures/
test_data/

# Skip vendored code
vendor/
third_party/

# Skip generated files
*.generated.*
auto_*
```

Smaller index = faster searches + less disk space.

---

## Tips

{: .tip }
**Start with .gitignore:** Most projects don't need a `.ckignore` - `.gitignore` already excludes the right things

{: .tip }
**Use .ckignore for search-specific exclusions:** Things you want in git but not in search (docs, configs, test data)

{: .tip }
**Test your exclusions:**
```bash
# See what would be indexed
ck --reindex . --verbose

# Or search to check
ck --sem "test" . | grep "should_be_excluded"
```

---

## Next steps

**→** [Editor integration](editor-integration.html) - Use ck from VS Code, Vim, Neovim

**→** [Large codebases](large-codebases.html) - Performance tips for big projects

**→** [CLI reference](cli-reference.html) - All flags and options
