# Index Behavior Specification

This document defines the expected behavior for ck's indexing system across all scenarios.

## Core Principles

1. **One index per repository** (like git) - `.ck/` directory at repository root
2. **Incremental updates** - preserve existing index entries, only update what changed
3. **Automatic maintenance** - cleanup orphaned files during normal operations
4. **Path-aware search** - search scope can be subdirectories but uses repo-wide index

## Index Structure

```
repo/
├── src/file1.rs
├── docs/manual.pdf
├── subdir/file2.py
└── .ck/                    # Repository index
    ├── manifest.json       # All indexed files metadata
    ├── content/            # PDF cache (only for PDFs)
    │   └── docs/manual.pdf.txt
    ├── src/file1.rs.ck     # Binary index entries
    ├── docs/manual.pdf.ck
    └── subdir/file2.py.ck
```

## Indexing Scenarios

### 1. Initial Index Creation

**Command**: `ck --index .` (on fresh repo)

**Expected Behavior**:
- Create `.ck/` directory
- Scan all files respecting gitignore and default exclusions
- Create manifest.json with all found files
- Generate embeddings for all files
- Create .ck sidecar files for each indexed file
- Extract PDF content to `.ck/content/` for PDF files

**Status**: ✅ Working

### 2. Explicit Index Update

**Command**: `ck --index .` (on existing index)

**Expected Behavior**:
- Load existing manifest.json
- Scan filesystem for all files
- **Preserve existing entries** for unchanged files
- **Add new entries** for newly created files
- **Update entries** for modified files (based on hash/mtime)
- **Remove entries** for deleted files + cleanup sidecars + PDF cache
- Update manifest.json with final state

**Status**: ✅ Working

### 3. Automatic Index Update During Search

**Command**: `ck --sem "query" .` or `ck --sem "query" subdir/`

**Expected Behavior**:
- Load existing index if present, create if missing
- **Incrementally update** index for any changed files in repository
- **Preserve all existing** index entries for unchanged files
- **Add/update only changed** files since last index update
- **Clean up orphaned** entries for deleted files
- **Never wipe existing index** - always incremental
- Perform search using updated index

**Current Status**: ❌ **BROKEN** - Wiping existing index entries

### 4. Subdirectory Search with Auto-Index

**Command**: `ck --sem "query" subdir/`

**Expected Behavior**:
- Use repository-wide index (find nearest .ck up the tree)
- Incrementally update index if any files changed
- Search results **limited to subdir/ scope** even though using repo-wide index
- Index contains all repo files, search filters by path

**Current Status**: ❌ **BROKEN** - Auto-indexing wipes repo index

## File Lifecycle Scenarios

### 5. New File Added

**Action**: Create `new_file.txt`
**Command**: `ck --sem "query" .`

**Expected Behavior**:
- Detect new file during auto-indexing
- Add new file to manifest
- Generate embedding and create sidecar
- **Preserve all existing entries**

**Status**: ❌ **BROKEN**

### 6. File Modified

**Action**: Edit existing file
**Command**: `ck --sem "query" .`

**Expected Behavior**:
- Detect file change (hash/mtime)
- Update manifest entry
- Regenerate embedding and update sidecar
- **Preserve all other entries**

**Status**: ❌ **BROKEN**

### 7. File Deleted

**Action**: Delete existing file
**Command**: `ck --sem "query" .`

**Expected Behavior**:
- Detect missing file during auto-indexing
- Remove from manifest
- Delete corresponding .ck sidecar
- If PDF: delete cached content from `.ck/content/`
- **Preserve all other entries**

**Status**: ❌ **BROKEN** - Files stay in results until explicit cleanup

### 8. PDF File Deleted (Current Bug)

**Action**: Delete `docs/manual.pdf`
**Command**: `ck --sem "content" .`

**Expected Behavior**:
- Auto-indexing detects missing PDF
- Remove from manifest
- Delete `docs/manual.pdf.ck` sidecar
- Delete `docs/manual.pdf.txt` from cache
- **No search results** from deleted PDF

**Current Status**: ❌ **BROKEN** - Returns stale results from cache

## Search Behavior

### 9. Repository Root Search

**Command**: `ck --sem "query" .`

**Expected Behavior**:
- Auto-update index if needed
- Search all files in repository
- Return results from entire repo

**Status**: ❌ **BROKEN** - Auto-indexing broken

### 10. Subdirectory Search

**Command**: `ck --sem "query" subdir/`

**Expected Behavior**:
- Use repository index (don't create subdir-specific index)
- Auto-update repository index if needed
- **Filter results to subdir/ only**
- Should find files like `subdir/file.txt`

**Status**: ❌ **BROKEN** - Auto-indexing broken

### 11. Unindexed Repository

**Command**: `ck --sem "query" .` (no existing .ck)

**Expected Behavior**:
- **Automatically create full index** with embeddings
- Perform search on newly created index
- Should "just work" without explicit --index

**Status**: ❌ **BROKEN** - Gets "No embeddings found" error

## Clean-up Operations

### 12. Explicit Cleanup

**Command**: `ck --clean-orphans .`

**Expected Behavior**:
- Remove manifest entries for deleted files
- Remove orphaned .ck sidecars
- Remove orphaned PDF cache files
- **Preserve entries for existing files**

**Status**: ✅ Working

### 13. Full Clean

**Command**: `ck --clean .`

**Expected Behavior**:
- Remove entire `.ck/` directory
- Start fresh

**Status**: ✅ Working

## Error Cases

### 14. Index Corruption

**Scenario**: Corrupted manifest.json or missing sidecars

**Expected Behavior**:
- Gracefully rebuild affected parts
- Continue working with valid entries
- Log warnings for corrupted parts

**Status**: ❓ Unknown

### 15. Permission Errors

**Scenario**: Cannot write to `.ck/` directory

**Expected Behavior**:
- Fail gracefully with clear error message
- Don't crash or corrupt existing data

**Status**: ❓ Unknown

## Configuration

### 16. Gitignore Respect

**Expected Behavior**:
- Respect `.gitignore` by default
- `--no-ignore` flag overrides
- Never index `.git/` itself

**Status**: ✅ Working

### 17. Default Exclusions

**Expected Behavior**:
- Exclude `node_modules/`, `target/`, `.cache/`, etc. by default
- `--no-default-excludes` flag overrides
- `--exclude` adds custom patterns

**Status**: ✅ Working (except in auto-indexing)

## Summary of Issues

### Critical Bugs
1. **Auto-indexing wipes existing index** - Makes incremental updates impossible
2. **Deleted file cleanup disabled** - Stale results from deleted files
3. **Fresh repo auto-indexing fails** - "No embeddings found" error

### Root Cause
The `smart_update_index` function called during automatic indexing is not working incrementally. It appears to be:
- Not preserving existing manifest entries
- Not properly handling file collection and updates
- Using incorrect parameters (empty exclude patterns vs default patterns)

### Next Steps
1. Fix `smart_update_index` to work incrementally
2. Re-enable automatic cleanup during indexing
3. Test all scenarios in this specification
4. Add automated tests for these behaviors