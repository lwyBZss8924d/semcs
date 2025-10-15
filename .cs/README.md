# SEMCS Demo Index

This `.cs/` directory contains a **pre-built semantic search index** for the SEMCS repository.

## 🎯 Purpose

This index is included in the repository so that users can:

1. **Experience cs immediately** after cloning - no need to build index first
2. **See what an index looks like** - understand the structure and size
3. **Test semantic search** without waiting for indexing
4. **Learn by example** - see how cs organizes embeddings and metadata

## 📊 Index Details

- **Model**: jina-embeddings-v4 (API-based, 1536 dimensions)
- **Size**: ~14 MB for ~50k LOC codebase
- **Files Indexed**: All Rust source files (.rs) and documentation (.md)
- **Created**: 2025-10-15
- **Repository Version**: v0.6.1

## 🚀 Quick Test

After cloning this repository, you can immediately test semantic search:

```bash
# Semantic search (uses this pre-built index)
cs --semantic "error handling patterns" .

# Hybrid search (BM25 + semantic + reranking)
cs --hybrid "configuration system" .

# AST structural search
cs --ast 'function $NAME($$)' .
```

No `cs --index` needed - the index is already built!

## 🔄 Rebuilding the Index

If you want to rebuild the index yourself:

```bash
# Remove existing index
rm -rf .cs/

# Build new index (requires JINA_API_KEY for jina-v4)
export JINA_API_KEY="your_key_here"
cs --index --model jina-v4 .

# Or use local model (no API key needed)
cs --index --model nomic-v1.5 .
```

## 📁 Index Structure

```
.cs/
├── *.cs               # Individual file embeddings (binary format)
├── index_metadata.json  # Index configuration and stats
└── file_map.json      # File path to embedding mapping
```

## 🎓 Learning Resources

- **Documentation**: [docs/](../docs/)
- **Examples**: [EXAMPLES.md](../EXAMPLES.md)
- **Quick Start**: [docs/tutorials/quick-start.md](../docs/tutorials/quick-start.md)
- **Benchmarks**: [benchmarks/](../benchmarks/)

## ⚠️ Important Notes

1. **Index is version-specific**: This index matches repository state at v0.6.1
2. **Model must match**: Use `jina-v4` for queries, or rebuild with your preferred model
3. **API key required**: jina-v4 requires `JINA_API_KEY` environment variable
4. **Size consideration**: 14MB is typical for this codebase size (~50k LOC)

## 🤝 Contributing

If you modify the codebase significantly, consider regenerating the index before committing:

```bash
cs --index --model jina-v4 .
git add .cs/
git commit -m "Update .cs index for new code changes"
```

---

**Why include the index in git?**

Most code search tools require users to build indexes themselves, which can take minutes for large codebases. By including a pre-built index, we let users experience instant semantic search, making it easier to evaluate and adopt the tool.

**For more information**: See [README.md](../README.md)
