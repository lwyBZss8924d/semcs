# Query-Based Chunking Overview

`ck-chunk` now drives language-aware chunk boundaries through Tree-sitter queries instead of handwritten matchers. Each language opts in by providing `queries/<language>/tags.scm` with captures that describe the constructs we care about (functions, struct/enum definitions, modules, etc.).

## Capture Conventions

- Tag definitions with hierarchical capture names such as `@definition.function`, `@definition.struct`, or `@module.impl`.
- The last segment of the capture name maps to a `ChunkType`:
  - `function`, `fn` → `ChunkType::Function`
  - `method` → `ChunkType::Method`
  - `class`, `struct`, `enum`, `trait` → `ChunkType::Class`
  - `module`, `impl`, `mod`, `namespace` → `ChunkType::Module`
- Non-matching captures are ignored by the refinement pipeline.

See `ck-chunk/queries/rust/tags.scm` for an example.

### Current Query Coverage

| Language | Status |
| --- | --- |
| Rust | Query-based (`queries/rust/tags.scm`) |
| Python | Query-based (`queries/python/tags.scm`) |
| TypeScript / JavaScript | Query-based (`queries/typescript` & `queries/javascript`) |
| Ruby | Query-based (`queries/ruby/tags.scm`) |
| Go | Query-based (`queries/go/tags.scm`) |
| C# | Query-based (`queries/csharp/tags.scm`) |
| Zig | Query-based (`queries/zig/tags.scm`) |
| Haskell | Query-based (`queries/haskell/tags.scm`) |

## Runtime Overrides

Embedded queries can be overridden without recompiling by setting `CK_CHUNK_QUERY_DIR` to a directory containing `<language>/tags.scm`. The loader falls back to the compiled queries when no override is present.

## Chunk Metadata

Every chunk now carries contextual metadata alongside the text:

- `ancestry`: lexical container names collected from parent nodes (e.g., `module::Class`).
- `breadcrumb`: a pre-joined `::` string form of the ancestry.
- `leading_trivia` / `trailing_trivia`: attached comments or attributes kept out of the main chunk text.
- `byte_length` and `estimated_tokens`: size hints for downstream embedding selection.

Metadata is populated for both query-driven and legacy chunkers, so consumers always see consistent fields even during incremental migration.

## Testing

- Language-specific Tree-sitter fixtures should validate query captures with `tree-sitter test`.
- Rust/Python/TypeScript/Haskell/Ruby/Go/C#/**Zig** unit tests (`tests::test_*_query_matches_legacy`) ensure query and legacy matchers stay in sync across the migrated languages.

## Next Steps

- Port remaining languages by adding `tags.scm` files and fixtures.
- Extend metadata assertions (CLI snapshots, index dumps) so UI surfaces breadcrumbs and trivia.
- Remove the legacy matcher once all languages rely on queries.
