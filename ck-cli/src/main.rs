use anyhow::Result;
use clap::{Parser, Subcommand};
use ck_core::{SearchOptions, SearchMode};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "ck")]
#[command(about = "Semantic grep by embedding - seek code, semantically")]
#[command(long_about = r#"
ck (seek) - A drop-in replacement for grep with semantic search capabilities

QUICK START EXAMPLES:

  Basic grep-style search (no indexing required):
    ck "error" src/                    # Find text matches
    ck -i "TODO" .                     # Case-insensitive search  
    ck -r "fn main" .                  # Recursive search
    ck -n "import" lib.py              # Show line numbers

  Semantic search (finds conceptually similar code):
    ck index .                         # First, create search index
    ck --sem "error handling" src/     # Find error handling patterns
    ck --sem "database connection"     # Find DB-related code
    ck --sem "authentication"         # Find auth-related code

  Lexical search (BM25 full-text search):
    ck --lex "user authentication"    # Full-text search with ranking
    ck --lex "http client request"    # Better than regex for phrases

  Hybrid search (combines regex + semantic):  
    ck --hybrid "async function"      # Best of both worlds
    ck --hybrid "error" --topk 10     # Top 10 most relevant results
    ck --hybrid "bug" --threshold 0.02 # Only results with RRF score >= 0.02
    ck --sem "auth" --scores           # Show similarity scores in output

  Index management:
    ck status .                       # Check index status
    ck status . --verbose             # Detailed index statistics
    ck clean . --orphans              # Clean up orphaned files
    ck clean .                        # Remove entire index

  JSON output for tools/scripts:
    ck --json --sem "bug fix" src/    # Machine-readable output
    ck --json --topk 5 "TODO"        # Limit results

  Advanced grep features:
    ck -C 2 "error" src/              # Show 2 lines of context  
    ck -A 3 -B 1 "TODO"              # 3 lines after, 1 before
    ck -w "test" .                    # Match whole words only
    ck -F "log.Error()" .             # Fixed string (no regex)

SEARCH MODES:
  --regex   : Classic grep behavior (default, no index needed)
  --lex     : BM25 lexical search (requires index)  
  --sem     : Semantic/embedding search (requires index)
  --hybrid  : Combines regex and semantic (requires index)

THRESHOLD AND SCORING:
  --threshold SCORE : Filter results by minimum score (0.0-1.0 semantic/lexical, 0.01-0.05 hybrid)
  --scores         : Show scores in output [0.950] file:line:match

The semantic search understands meaning - searching for "error handling" 
will find try/catch blocks, error returns, exception handling, etc.
"#)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    pattern: Option<String>,
    
    #[arg(help = "Files or directories to search")]
    files: Vec<PathBuf>,
    
    #[arg(short = 'n', long = "line-number", help = "Show line numbers")]
    line_numbers: bool,
    
    #[arg(short = 'h', help = "Suppress filenames in output")]
    no_filenames: bool,
    
    #[arg(short = 'H', help = "Always print filenames")]
    with_filenames: bool,
    
    #[arg(short = 'i', long = "ignore-case", help = "Case insensitive search")]
    ignore_case: bool,
    
    #[arg(short = 'w', long = "word-regexp", help = "Match whole words only")]
    word_regexp: bool,
    
    #[arg(short = 'F', long = "fixed-strings", help = "Interpret pattern as fixed string")]
    fixed_strings: bool,
    
    #[arg(short = 'R', short_alias = 'r', long = "recursive", help = "Recursively search directories")]
    recursive: bool,
    
    #[arg(short = 'C', long = "context", value_name = "NUM", help = "Show NUM lines of context before and after")]
    context: Option<usize>,
    
    #[arg(short = 'A', long = "after-context", value_name = "NUM", help = "Show NUM lines after match")]
    after_context: Option<usize>,
    
    #[arg(short = 'B', long = "before-context", value_name = "NUM", help = "Show NUM lines before match")]
    before_context: Option<usize>,
    
    #[arg(long = "sem", help = "Semantic search - finds conceptually similar code")]
    semantic: bool,
    
    #[arg(long = "lex", help = "Lexical search - BM25 full-text search with ranking")]
    lexical: bool,
    
    #[arg(long = "hybrid", help = "Hybrid search - combines regex and semantic results")]
    hybrid: bool,
    
    #[arg(long = "regex", help = "Regex search mode (default, grep-compatible)")]
    regex: bool,
    
    #[arg(long = "topk", value_name = "N", help = "Limit results to top N matches (useful with --sem/--lex)")]
    top_k: Option<usize>,
    
    #[arg(long = "threshold", value_name = "SCORE", help = "Minimum score threshold (0.0-1.0 for semantic/lexical, 0.01-0.05 for hybrid RRF)")]
    threshold: Option<f32>,
    
    #[arg(long = "scores", help = "Show similarity scores in output")]
    show_scores: bool,
    
    #[arg(long = "json", help = "Output results as JSON for tools/scripts")]
    json: bool,
    
    #[arg(long = "json-v1", help = "Output results as JSON v1 schema")]
    json_v1: bool,
    
    #[arg(long = "reindex", help = "Force index update before searching")]
    reindex: bool,
    
    #[arg(long = "exclude", value_name = "PATTERN", help = "Exclude directories matching pattern (can be used multiple times)")]
    exclude: Vec<String>,
    
    #[arg(long = "no-default-excludes", help = "Disable default directory exclusions (like .git, node_modules, etc.)")]
    no_default_excludes: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create or update search index")]
    #[command(long_about = r#"
Create or update the search index for semantic and lexical search.

EXAMPLES:
  ck index .                          # Index current directory
  ck index /path/to/project           # Index specific project
  ck index . && ck --sem "auth"      # Index then search

The index enables fast semantic and lexical search. It's stored in a .ck/ 
directory and automatically updates when files change during search.

Supported file types: rs, py, js, ts, go, java, c, cpp, rb, php, swift, 
kt, scala, txt, md, json, yaml, xml, html, css, sh, sql, and more.
"#)]
    Index {
        #[arg(help = "Path to directory to index")]
        path: PathBuf,
    },
    
    #[command(about = "Search with explicit pattern and path")]
    #[command(long_about = r#"
Explicit search command with pattern and path arguments.

EXAMPLES:
  ck search "error" src/              # Basic text search
  ck search "authentication" . --reindex  # Force index update
  
This is equivalent to: ck "pattern" path
Most users prefer the shorter syntax: ck --sem "pattern" path
"#)]
    Search {
        #[arg(help = "Search pattern or query")]
        pattern: String,
        
        #[arg(help = "Path to search in (default: current directory)")]
        path: Option<PathBuf>,
        
        #[arg(long = "reindex", help = "Force index update before searching")]
        reindex: bool,
    },
    
    #[command(about = "Clean up search index and orphaned files")]
    #[command(long_about = r#"
Clean up the search index and remove orphaned files.

EXAMPLES:
  ck clean .                          # Remove entire .ck index directory
  ck clean . --orphans                # Only remove orphaned/stale files
  ck clean /path/to/project           # Clean specific project
  
ORPHAN CLEANUP:
Removes index entries for deleted files and cleans up empty directories.
This is automatically done during indexing, but can be run manually.

FULL CLEANUP:  
Completely removes the .ck directory. You'll need to run 'ck index' again
to use semantic or lexical search.
"#)]
    Clean {
        #[arg(help = "Path to clean (default: current directory)")]
        path: Option<PathBuf>,
        
        #[arg(long = "orphans", help = "Only clean orphaned files, keep index")]
        orphans_only: bool,
    },
    
    #[command(about = "Add single file to index")]
    #[command(long_about = r#"
Add or update a single file in the search index.

EXAMPLES:
  ck add src/main.rs                  # Add specific file
  ck add lib.py                       # Update file after changes
  
This is useful for quickly updating the index after editing a file,
without reprocessing the entire directory.
"#)]
    Add {
        #[arg(help = "File path to add to index")]
        file: PathBuf,
    },
    
    #[command(about = "Show index status and statistics")]
    #[command(long_about = r#"
Display information about the current search index.

EXAMPLES:
  ck status .                         # Basic index info
  ck status . --verbose               # Detailed statistics
  ck status /path/to/project -v       # Check remote project
  
OUTPUT INCLUDES:
- Number of files indexed
- Total chunks and embeddings
- Disk usage and compression ratios
- Creation and update timestamps
- Index health and optimization suggestions

Use this to verify indexing worked and monitor index size growth.
"#)]
    Status {
        #[arg(help = "Path to check (default: current directory)")]
        path: Option<PathBuf>,
        
        #[arg(long = "verbose", short = 'v', help = "Show detailed statistics and timestamps")]
        verbose: bool,
    },
}

fn expand_glob_patterns(paths: &[PathBuf], exclude_patterns: &[String]) -> Result<Vec<PathBuf>> {
    let mut expanded = Vec::new();
    
    for path in paths {
        let path_str = path.to_string_lossy();
        
        // Check if this looks like a glob pattern
        if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') {
            // Use glob to expand the pattern
            match glob::glob(&path_str) {
                Ok(glob_paths) => {
                    let mut found_matches = false;
                    for glob_result in glob_paths {
                        match glob_result {
                            Ok(matched_path) => {
                                // Apply exclusion patterns to glob results
                                if !should_exclude_path(&matched_path, exclude_patterns) {
                                    expanded.push(matched_path);
                                }
                                found_matches = true;
                            }
                            Err(e) => {
                                eprintln!("Warning: glob error for pattern '{}': {}", path_str, e);
                            }
                        }
                    }
                    
                    // If no matches found, treat as literal path (grep behavior)
                    if !found_matches {
                        expanded.push(path.clone());
                    }
                }
                Err(e) => {
                    eprintln!("Warning: invalid glob pattern '{}': {}", path_str, e);
                    // Treat as literal path if glob pattern is invalid
                    expanded.push(path.clone());
                }
            }
        } else {
            // Not a glob pattern, use as-is
            expanded.push(path.clone());
        }
    }
    
    Ok(expanded)
}

fn should_exclude_path(path: &Path, exclude_patterns: &[String]) -> bool {
    // Check if any component in the path matches an exclusion pattern
    for component in path.components() {
        if let std::path::Component::Normal(name) = component {
            let name_str = name.to_string_lossy();
            for pattern in exclude_patterns {
                if name_str == pattern.as_str() {
                    return true;
                }
            }
        }
    }
    false
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into())
        )
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Index { path }) => {
            println!("Indexing {:?}...", path);
            let stats = ck_index::smart_update_index(&path, true).await?;
            println!("Indexing complete: {} files indexed", stats.files_indexed);
        }
        
        Some(Commands::Search { ref pattern, ref path, reindex }) => {
            let search_path = path.clone().unwrap_or_else(|| PathBuf::from("."));
            let options = build_options(&cli, reindex);
            run_search(pattern.clone(), search_path, options).await?;
        }
        
        Some(Commands::Clean { path, orphans_only }) => {
            let clean_path = path.unwrap_or_else(|| PathBuf::from("."));
            
            if orphans_only {
                println!("Cleaning orphaned files from {:?}...", clean_path);
                let cleanup_stats = ck_index::cleanup_index(&clean_path)?;
                println!("Cleanup complete: {} orphaned entries removed, {} orphaned sidecars removed",
                        cleanup_stats.orphaned_entries_removed,
                        cleanup_stats.orphaned_sidecars_removed);
            } else {
                println!("Cleaning entire index for {:?}...", clean_path);
                ck_index::clean_index(&clean_path)?;
                println!("Index cleaned");
            }
        }
        
        Some(Commands::Add { file }) => {
            println!("Adding {:?} to index...", file);
            ck_index::index_file(&file).await?;
            println!("File added to index");
        }
        
        Some(Commands::Status { path, verbose }) => {
            let status_path = path.unwrap_or_else(|| PathBuf::from("."));
            let stats = ck_index::get_index_stats(&status_path)?;
            
            if stats.total_files == 0 {
                println!("No index found at {:?}", status_path);
            } else {
                println!("Index status for {:?}:", status_path);
                println!("  Files indexed: {}", stats.total_files);
                println!("  Total chunks: {}", stats.total_chunks);
                println!("  Embedded chunks: {}", stats.embedded_chunks);
                
                if verbose {
                    println!("  Total file size: {} bytes", stats.total_size_bytes);
                    println!("  Index size on disk: {} bytes", stats.index_size_bytes);
                    
                    use std::time::UNIX_EPOCH;
                    if stats.index_created > 0 {
                        if let Some(created) = UNIX_EPOCH.checked_add(std::time::Duration::from_secs(stats.index_created)) {
                            println!("  Created: {:?}", created);
                        }
                    }
                    if stats.index_updated > 0 {
                        if let Some(updated) = UNIX_EPOCH.checked_add(std::time::Duration::from_secs(stats.index_updated)) {
                            println!("  Last updated: {:?}", updated);
                        }
                    }
                    
                    // Show compression ratio
                    if stats.total_size_bytes > 0 {
                        let compression_ratio = stats.index_size_bytes as f64 / stats.total_size_bytes as f64;
                        println!("  Compression ratio: {:.2}%", compression_ratio * 100.0);
                    }
                }
            }
        }
        
        None => {
            if let Some(ref pattern) = cli.pattern {
                let reindex = cli.reindex;
                
                // Build options to get exclusion patterns  
                let temp_options = build_options(&cli, reindex);
                
                let files = if cli.files.is_empty() {
                    vec![PathBuf::from(".")]
                } else {
                    expand_glob_patterns(&cli.files, &temp_options.exclude_patterns)?
                };
                
                // Handle multiple files like grep; allow -h/-H overrides
                let mut show_filenames = files.len() > 1 || files.iter().any(|p| p.is_dir());
                if cli.no_filenames { show_filenames = false; }
                if cli.with_filenames { show_filenames = true; }
                for file_path in files {
                    let mut options = build_options(&cli, reindex);
                    options.show_filenames = show_filenames;
                    run_search(pattern.clone(), file_path, options).await?;
                }
            } else {
                eprintln!("Error: No pattern specified");
                std::process::exit(1);
            }
        }
    }
    
    Ok(())
}

fn build_options(cli: &Cli, reindex: bool) -> SearchOptions {
    let mode = if cli.semantic {
        SearchMode::Semantic
    } else if cli.lexical {
        SearchMode::Lexical
    } else if cli.hybrid {
        SearchMode::Hybrid
    } else {
        SearchMode::Regex
    };
    
    let context = cli.context.unwrap_or(0);
    let before_context = cli.before_context.unwrap_or(context);
    let after_context = cli.after_context.unwrap_or(context);
    
    // Build exclusion patterns (as provided; glob semantics applied in ck-search)
    let mut exclude_patterns = Vec::new();
    
    // Add default exclusions unless disabled
    if !cli.no_default_excludes {
        exclude_patterns.extend(ck_core::get_default_exclude_patterns());
    }
    
    // Add user-specified exclusions
    exclude_patterns.extend(cli.exclude.clone());
    
    SearchOptions {
        mode,
        query: String::new(),
        path: PathBuf::from("."),
        top_k: cli.top_k,
        threshold: cli.threshold,
        case_insensitive: cli.ignore_case,
        whole_word: cli.word_regexp,
        fixed_string: cli.fixed_strings,
        line_numbers: cli.line_numbers,
        context_lines: context,
        before_context_lines: before_context,
        after_context_lines: after_context,
        recursive: cli.recursive,
        json_output: cli.json || cli.json_v1,
        reindex,
        show_scores: cli.show_scores,
        show_filenames: false, // Will be set by caller
        exclude_patterns,
    }
}

async fn run_search(pattern: String, path: PathBuf, mut options: SearchOptions) -> Result<()> {
    options.query = pattern;
    options.path = path;
    
    if options.reindex {
        ck_index::update_index(&options.path).await?;
    }
    
    let results = ck_search::search(&options).await?;
    
    let mut has_matches = false;
    if options.json_output {
        for result in results {
            has_matches = true;
            let json_result = ck_core::JsonSearchResult {
                file: result.file.display().to_string(),
                span: result.span,
                lang: result.lang,
                symbol: result.symbol,
                score: result.score,
                signals: ck_core::SearchSignals {
                    lex_rank: None,
                    vec_rank: None,
                    rrf_score: result.score,
                },
                preview: result.preview,
                model: "none".to_string(),
            };
            println!("{}", serde_json::to_string(&json_result)?);
        }
    } else {
        for result in results {
            has_matches = true;
            let score_text = if options.show_scores {
                format!("[{:.3}] ", result.score)
            } else {
                String::new()
            };
            
            let file_text = if options.show_filenames {
                format!("{}:", result.file.display())
            } else {
                String::new()
            };
            
            if options.line_numbers && options.show_filenames {
                println!("{}{}{}:{}", 
                    score_text,
                    file_text,
                    result.span.line_start,
                    result.preview
                );
            } else if options.line_numbers {
                println!("{}{}:{}", 
                    score_text,
                    result.span.line_start,
                    result.preview
                );
            } else {
                println!("{}{}{}", 
                    score_text,
                    file_text,
                    result.preview
                );
            }
        }
    }
    
    // grep-like exit codes: 0 if matches, 1 if none
    if !has_matches { 
        std::process::exit(1);
    }
    Ok(())
}