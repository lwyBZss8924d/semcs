use anyhow::Result;
use clap::Parser;
use ck_core::{SearchOptions, SearchMode};
use console::style;
use regex::Regex;
use std::path::{Path, PathBuf};

mod progress;
use progress::StatusReporter;

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
    ck --index .                       # First, create search index
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
    ck --status .                     # Check index status
    ck --status-verbose .              # Detailed index statistics
    ck --clean-orphans .               # Clean up orphaned files
    ck --clean .                       # Remove entire index
    ck --add file.rs                   # Add single file to index

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
    pattern: Option<String>,
    
    #[arg(help = "Files or directories to search")]
    files: Vec<PathBuf>,
    
    #[arg(short = 'n', long = "line-number", help = "Show line numbers")]
    line_numbers: bool,
    
    #[arg(long = "no-filename", help = "Suppress filenames in output")]
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
    
    #[arg(long = "full-section", help = "Return complete code sections (functions/classes) instead of just matching lines. Uses tree-sitter to identify semantic boundaries. Supported: Python, JavaScript, TypeScript")]
    full_section: bool,
    
    #[arg(short = 'q', long = "quiet", help = "Suppress status messages and progress indicators")]
    quiet: bool,
    
    // Command flags (replacing subcommands)
    #[arg(long = "index", help = "Create or update search index for the specified path")]
    index: bool,
    
    #[arg(long = "clean", help = "Clean up search index")]
    clean: bool,
    
    #[arg(long = "clean-orphans", help = "Clean only orphaned index files")]
    clean_orphans: bool,
    
    #[arg(long = "add", help = "Add a single file to the index")]
    add: bool,
    
    #[arg(long = "status", help = "Show index status and statistics")]
    status: bool,
    
    #[arg(long = "status-verbose", help = "Show detailed index statistics")]
    status_verbose: bool,
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
    let status = StatusReporter::new(cli.quiet);
    
    // Handle command flags first (these take precedence over search)
    if cli.index {
        // Handle --index flag
        let path = cli.files.first().cloned().unwrap_or_else(|| PathBuf::from("."));
            
        status.section_header("Indexing Repository");
        status.info(&format!("Scanning files in {}", path.display()));
        
        let indexing_progress = status.create_spinner("Building index...");
        
        // Create progress callback to show current file being processed
        let progress_callback = if !cli.quiet {
            let pb_clone = indexing_progress.clone();
            Some(Box::new(move |file_name: &str| {
                if let Some(pb) = &pb_clone {
                    pb.set_message(format!("Processing {}", file_name));
                }
            }) as ck_index::ProgressCallback)
        } else {
            None
        };
        
        let stats = ck_index::smart_update_index_with_progress(&path, true, progress_callback).await?;
        status.finish_progress(indexing_progress, "Index built successfully");
        
        status.success(&format!("Indexed {} files", stats.files_indexed));
        if stats.files_added > 0 {
            status.info(&format!("  {} new files added", stats.files_added));
        }
        if stats.files_modified > 0 {
            status.info(&format!("  {} files updated", stats.files_modified));
        }
        if stats.files_up_to_date > 0 {
            status.info(&format!("  {} files already current", stats.files_up_to_date));
        }
        if stats.orphaned_files_removed > 0 {
            status.info(&format!("  {} orphaned entries cleaned", stats.orphaned_files_removed));
        }
        return Ok(());
    }
    
    if cli.clean || cli.clean_orphans {
        // Handle --clean and --clean-orphans flags
        let clean_path = cli.files.first().cloned().unwrap_or_else(|| PathBuf::from("."));
        let orphans_only = cli.clean_orphans;
        
        if orphans_only {
            status.section_header("Cleaning Orphaned Files");
            status.info(&format!("Scanning for orphans in {}", clean_path.display()));
            
            let cleanup_spinner = status.create_spinner("Removing orphaned entries...");
            let cleanup_stats = ck_index::cleanup_index(&clean_path)?;
            status.finish_progress(cleanup_spinner, "Cleanup complete");
            
            if cleanup_stats.orphaned_entries_removed > 0 || cleanup_stats.orphaned_sidecars_removed > 0 {
                status.success(&format!("Removed {} orphaned entries and {} orphaned sidecars",
                        cleanup_stats.orphaned_entries_removed,
                        cleanup_stats.orphaned_sidecars_removed));
            } else {
                status.info("No orphaned files found");
            }
        } else {
            status.section_header("Cleaning Index");
            status.warn(&format!("Removing entire index for {}", clean_path.display()));
            
            let clean_spinner = status.create_spinner("Removing index files...");
            ck_index::clean_index(&clean_path)?;
            status.finish_progress(clean_spinner, "Index removed");
            
            status.success("Index cleaned successfully");
        }
        return Ok(());
    }
    
    if cli.add {
        // Handle --add flag
        let file = cli.files.first().cloned().ok_or_else(|| {
            anyhow::anyhow!("No file specified. Usage: ck --add <file>")
        })?;
        status.section_header("Adding File to Index");
        status.info(&format!("Processing {}", file.display()));
        
        let add_spinner = status.create_spinner("Updating index...");
        ck_index::index_file(&file).await?;
        status.finish_progress(add_spinner, "File indexed");
        
        status.success(&format!("Added {} to index", file.display()));
        return Ok(());
    }
    
    if cli.status || cli.status_verbose {
        // Handle --status and --status-verbose flags
        let status_path = cli.files.first().cloned().unwrap_or_else(|| PathBuf::from("."));
        let verbose = cli.status_verbose;
            
        status.section_header("Index Status");
        let check_spinner = status.create_spinner("Reading index...");
        let stats = ck_index::get_index_stats(&status_path)?;
        status.finish_progress(check_spinner, "Status retrieved");
        
        if stats.total_files == 0 {
            status.warn(&format!("No index found at {}", status_path.display()));
            status.info("Run 'ck --index .' to create an index");
        } else {
            status.info(&format!("Index location: {}", status_path.display()));
            status.success(&format!("Files indexed: {}", stats.total_files));
            status.info(&format!("  Total chunks: {}", stats.total_chunks));
            status.info(&format!("  Embedded chunks: {}", stats.embedded_chunks));
            
            if verbose {
                let size_mb = stats.total_size_bytes as f64 / (1024.0 * 1024.0);
                let index_size_mb = stats.index_size_bytes as f64 / (1024.0 * 1024.0);
                status.info(&format!("  Source size: {:.1} MB", size_mb));
                status.info(&format!("  Index size: {:.1} MB", index_size_mb));
                
                use std::time::UNIX_EPOCH;
                if stats.index_created > 0 {
                    if let Some(created) = UNIX_EPOCH.checked_add(std::time::Duration::from_secs(stats.index_created)) {
                        if let Ok(datetime) = created.elapsed() {
                            status.info(&format!("  Created: {:.1} hours ago", datetime.as_secs() as f64 / 3600.0));
                        }
                    }
                }
                if stats.index_updated > 0 {
                    if let Some(updated) = UNIX_EPOCH.checked_add(std::time::Duration::from_secs(stats.index_updated)) {
                        if let Ok(datetime) = updated.elapsed() {
                            status.info(&format!("  Updated: {:.1} hours ago", datetime.as_secs() as f64 / 3600.0));
                        }
                    }
                }
                
                // Show compression ratio
                if stats.total_size_bytes > 0 {
                    let compression_ratio = stats.index_size_bytes as f64 / stats.total_size_bytes as f64;
                    status.info(&format!("  Compression: {:.1}x ({:.1}%)", 1.0/compression_ratio, compression_ratio * 100.0));
                }
            }
        }
        return Ok(());
    }
    
    // Default behavior: search with pattern
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
            run_search(pattern.clone(), file_path, options, &status).await?;
        }
    } else {
        eprintln!("Error: No pattern specified");
        std::process::exit(1);
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
        full_section: cli.full_section,
    }
}

fn highlight_matches(text: &str, pattern: &str, options: &SearchOptions) -> String {
    // Don't highlight if this is JSON output
    if options.json_output {
        return text.to_string();
    }
    
    match options.mode {
        SearchMode::Regex => {
            highlight_regex_matches(text, pattern, options)
        }
        SearchMode::Semantic | SearchMode::Hybrid => {
            // For semantic/hybrid search, use subchunk similarity highlighting
            highlight_semantic_chunks(text, pattern, options)
        }
        _ => {
            text.to_string()
        }
    }
}

fn highlight_regex_matches(text: &str, pattern: &str, options: &SearchOptions) -> String {
    // Build regex from pattern with same flags as main search
    let regex_flags = if options.case_insensitive { "(?i)" } else { "" };
    let regex_pattern = if options.fixed_string {
        // For fixed strings, escape regex special characters
        format!("{}{}", regex_flags, regex::escape(pattern))
    } else if options.whole_word {
        // For word regexp, wrap in word boundaries
        format!(r"{}\b{}\b", regex_flags, pattern)
    } else {
        format!("{}{}", regex_flags, pattern)
    };
    
    match Regex::new(&regex_pattern) {
        Ok(re) => {
            // Replace matches with highlighted versions
            re.replace_all(text, |caps: &regex::Captures| {
                style(&caps[0]).red().bold().to_string()
            }).to_string()
        },
        Err(_) => {
            // If regex is invalid, return original text
            text.to_string()
        }
    }
}

fn highlight_semantic_chunks(text: &str, pattern: &str, _options: &SearchOptions) -> String {
    // Split text into sentences/phrases for semantic analysis
    let sentences = split_into_sentences(text);
    
    // For now, implement a simple keyword-based approximation
    // TODO: Replace with actual semantic similarity once embedding infrastructure is available
    let highlighted_sentences: Vec<String> = sentences.into_iter().map(|sentence| {
        let similarity_score = calculate_keyword_similarity(&sentence, pattern);
        
        if similarity_score > 0.7 {
            style(sentence).green().bold().to_string()
        } else if similarity_score > 0.4 {
            style(sentence).green().to_string()
        } else {
            sentence
        }
    }).collect();
    
    highlighted_sentences.join(" ")
}

fn split_into_sentences(text: &str) -> Vec<String> {
    // Split on natural boundaries for both text and code
    // For code: split on semicolons, newlines, braces
    // For text: split on sentence endings
    let boundary_regex = Regex::new(r"[.!?;\n}]+\s*|\s*\{\s*").unwrap();
    
    let parts: Vec<String> = boundary_regex.split(text)
        .filter(|s| !s.trim().is_empty() && s.trim().len() > 5) // Filter very short fragments
        .map(|s| s.trim().to_string())
        .collect();
    
    // If we got too few parts, fall back to splitting on whitespace for longer phrases
    if parts.len() <= 1 && text.len() > 100 {
        let words: Vec<&str> = text.split_whitespace().collect();
        return words.chunks(10) // Group every 10 words
            .map(|chunk| chunk.join(" "))
            .collect();
    }
    
    parts
}

fn calculate_keyword_similarity(text: &str, pattern: &str) -> f32 {
    let text_lower = text.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    
    // Extract words from both text and pattern
    let text_words: std::collections::HashSet<String> = text_lower
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty() && w.len() > 2) // Filter out short words
        .map(String::from)
        .collect();
    
    let pattern_words: std::collections::HashSet<String> = pattern_lower
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty() && w.len() > 2)
        .map(String::from)
        .collect();
    
    if pattern_words.is_empty() {
        return 0.0;
    }
    
    // Check for exact substring matches (higher weight)
    let mut score = 0.0;
    for pattern_word in &pattern_words {
        if text_lower.contains(pattern_word) {
            score += 1.0;
        }
    }
    
    // Add partial word overlap
    let intersection_count = text_words.intersection(&pattern_words).count();
    score += intersection_count as f32 * 0.5;
    
    // Normalize by pattern length
    score / pattern_words.len() as f32
}

async fn run_search(pattern: String, path: PathBuf, mut options: SearchOptions, status: &StatusReporter) -> Result<()> {
    options.query = pattern;
    options.path = path;
    
    if options.reindex {
        let reindex_spinner = status.create_spinner("Updating index...");
        ck_index::update_index(&options.path).await?;
        status.finish_progress(reindex_spinner, "Index updated");
    }
    
    // Show search progress for non-regex searches or when explicitly enabled
    let search_spinner = if !matches!(options.mode, ck_core::SearchMode::Regex) {
        status.create_spinner(&format!("Searching with {} mode...", 
            match options.mode {
                ck_core::SearchMode::Semantic => "semantic",
                ck_core::SearchMode::Lexical => "lexical", 
                ck_core::SearchMode::Hybrid => "hybrid",
                _ => "regex"
            }))
    } else {
        None
    };
    
    // Create progress callback for search operations
    let search_progress_callback = if !status.quiet && search_spinner.is_some() {
        let spinner_clone = search_spinner.clone();
        Some(Box::new(move |msg: &str| {
            if let Some(ref pb) = spinner_clone {
                pb.set_message(msg.to_string());
            }
        }) as ck_search::SearchProgressCallback)
    } else {
        None
    };
    
    let results = ck_search::search_with_progress(&options, search_progress_callback).await?;
    
    if let Some(spinner) = search_spinner {
        status.finish_progress(Some(spinner), &format!("Found {} results", results.len()));
    }
    
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
                format!("{}:", style(result.file.display()).cyan().bold())
            } else {
                String::new()
            };
            
            let highlighted_preview = highlight_matches(&result.preview, &options.query, &options);
            
            if options.line_numbers && options.show_filenames {
                println!("{}{}{}:{}", 
                    score_text,
                    file_text,
                    style(result.span.line_start).yellow(),
                    highlighted_preview
                );
            } else if options.line_numbers {
                println!("{}{}:{}", 
                    score_text,
                    style(result.span.line_start).yellow(),
                    highlighted_preview
                );
            } else {
                println!("{}{}{}", 
                    score_text,
                    file_text,
                    highlighted_preview
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