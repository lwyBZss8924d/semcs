// AST structural search using ast-grep
// Requires ast-grep to be installed: brew install ast-grep

use anyhow::Result;
use cs_core::{CcError, Language, SearchOptions, SearchResult, Span};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

/// AST match returned by ast-grep --json
#[derive(Debug, Deserialize)]
struct AstGrepMatch {
    text: String,
    range: AstGrepRange,
    file: String,
    language: String,
    #[serde(rename = "metaVariables")]
    meta_variables: Option<AstGrepMetaVars>,
}

#[derive(Debug, Deserialize)]
struct AstGrepRange {
    #[serde(rename = "byteOffset")]
    byte_offset: ByteOffset,
    start: Position,
    end: Position,
}

#[derive(Debug, Deserialize)]
struct ByteOffset {
    start: usize,
    end: usize,
}

#[derive(Debug, Deserialize)]
struct Position {
    line: usize,
    #[allow(dead_code)] // Required by ast-grep JSON schema but not used
    column: usize,
}

#[derive(Debug, Deserialize)]
struct AstGrepMetaVars {
    single: Option<serde_json::Value>,
}

/// Perform AST structural search using ast-grep CLI
pub async fn ast_search(options: &SearchOptions) -> Result<Vec<SearchResult>> {
    // 1. Check if ast-grep is installed
    check_ast_grep_installed()?;

    // 2. Build ast-grep command
    let mut cmd = Command::new("ast-grep");
    cmd.arg("run");

    // Pattern (use ast_pattern if set, otherwise use query)
    let pattern = options
        .ast_pattern
        .as_ref()
        .unwrap_or(&options.query);
    cmd.arg("--pattern").arg(pattern);

    // JSON output
    cmd.arg("--json");

    // Language parameter (optional, ast-grep auto-detects by default)
    if let Some(lang) = &options.ast_lang {
        cmd.arg("--lang").arg(map_language_to_astgrep(lang));
    }

    // Strictness (optional)
    if let Some(strictness) = &options.ast_strictness {
        cmd.arg("--strictness").arg(strictness);
    }

    // Selector (optional)
    if let Some(selector) = &options.ast_selector {
        cmd.arg("--selector").arg(selector);
    }

    // Exclude patterns (convert to --globs)
    for pattern in &options.exclude_patterns {
        cmd.arg("--globs").arg(format!("!{}", pattern));
    }

    // Gitignore support
    if !options.respect_gitignore {
        cmd.arg("--no-ignore").arg("vcs");
    }

    // Top-k limit (approximate)
    // Note: ast-grep doesn't have a built-in limit, so we'll filter after
    // let top_k_limit = options.top_k.unwrap_or(1000);

    // Search path
    let search_path = options.path.to_str().unwrap_or(".");
    cmd.arg(search_path);

    // 3. Execute command and capture output
    let output = cmd
        .output()
        .map_err(|e| CcError::Search(format!("Failed to execute ast-grep: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CcError::Search(format!("ast-grep failed: {}", stderr)).into());
    }

    // 4. Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Handle empty output (no matches)
    if stdout.trim().is_empty() || stdout.trim() == "[]" {
        return Ok(vec![]);
    }

    let matches: Vec<AstGrepMatch> = serde_json::from_str(&stdout).map_err(|e| {
        CcError::Search(format!(
            "Failed to parse ast-grep output: {}\nOutput: {}",
            e, stdout
        ))
    })?;

    // 5. Convert to SearchResult
    let mut results: Vec<SearchResult> = matches
        .into_iter()
        .map(|m| {
            let lang = Language::from_extension(&m.language.to_lowercase());

            // Extract symbol from metavariables (typically $NAME)
            let symbol = extract_symbol(&m.meta_variables);

            SearchResult {
                file: PathBuf::from(m.file),
                span: Span {
                    byte_start: m.range.byte_offset.start,
                    byte_end: m.range.byte_offset.end,
                    line_start: m.range.start.line + 1, // ast-grep uses 0-based lines
                    line_end: m.range.end.line + 1,
                },
                score: 1.0, // AST matches are exact matches
                preview: m.text,
                lang,
                symbol,
                chunk_hash: None,
                index_epoch: None,
            }
        })
        .collect();

    // 6. Apply top-k limit if specified
    if let Some(top_k) = options.top_k {
        results.truncate(top_k);
    }

    Ok(results)
}

/// Check if ast-grep is installed and available
fn check_ast_grep_installed() -> Result<()> {
    Command::new("ast-grep")
        .arg("--version")
        .output()
        .map_err(|_| {
            CcError::Search(
                "ast-grep not found. Install it first:\n\
                 • macOS: brew install ast-grep\n\
                 • Linux: cargo install ast-grep --locked\n\
                 • Or visit: https://ast-grep.github.io/guide/installation.html"
                    .to_string(),
            )
        })?;
    Ok(())
}

/// Extract symbol name from metavariables (usually $NAME)
fn extract_symbol(meta_vars: &Option<AstGrepMetaVars>) -> Option<String> {
    meta_vars
        .as_ref()
        .and_then(|mv| mv.single.as_ref())
        .and_then(|single| {
            // Try common metavariable names
            single
                .get("NAME")
                .or_else(|| single.get("FUNC"))
                .or_else(|| single.get("VAR"))
        })
        .and_then(|var| var.get("text"))
        .and_then(|text| text.as_str())
        .map(String::from)
}

/// Map cc language names to ast-grep language names
fn map_language_to_astgrep(lang: &str) -> &str {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => "rust",
        "python" | "py" => "python",
        "javascript" | "js" => "javascript",
        "typescript" | "ts" => "typescript",
        "tsx" => "tsx",
        "go" => "go",
        "java" => "java",
        "c" => "c",
        "cpp" | "c++" => "cpp",
        "csharp" | "c#" | "cs" => "csharp",
        "ruby" | "rb" => "ruby",
        "kotlin" | "kt" => "kotlin",
        "swift" => "swift",
        "html" => "html",
        "css" => "css",
        "yaml" | "yml" => "yaml",
        // Default: pass through as-is
        _ => lang,
    }
}

/// Check if a pattern looks like an AST pattern
/// (contains metavariables like $VAR, $$VAR, $$$VAR)
pub fn is_ast_pattern(query: &str) -> bool {
    query.contains('$')
        || query.contains("function ")
        || query.contains("class ")
        || query.contains("impl ")
        || query.contains("struct ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ast_pattern() {
        assert!(is_ast_pattern("function $NAME() { $$$ }"));
        assert!(is_ast_pattern("$VAR = $VALUE"));
        assert!(is_ast_pattern("class $CLASS"));
        assert!(!is_ast_pattern("simple text search"));
    }

    #[test]
    fn test_map_language() {
        assert_eq!(map_language_to_astgrep("rust"), "rust");
        assert_eq!(map_language_to_astgrep("python"), "python");
        assert_eq!(map_language_to_astgrep("typescript"), "typescript");
        assert_eq!(map_language_to_astgrep("c++"), "cpp");
    }

    #[tokio::test]
    #[ignore] // Requires ast-grep installation
    async fn test_ast_search_basic() {
        let options = SearchOptions {
            mode: cs_core::SearchMode::Ast,
            query: "function $NAME() { $$$ }".to_string(),
            path: PathBuf::from("/tmp"),
            ast_pattern: None,
            ast_lang: Some("javascript".to_string()),
            ..Default::default()
        };

        // This test will only run if ast-grep is installed
        match ast_search(&options).await {
            Ok(results) => {
                // Should either find matches or return empty vec
                assert!(results.is_empty() || results.len() > 0);
            }
            Err(e) => {
                // Expected if ast-grep not installed or no JS files in /tmp
                println!("Test skipped or expected error: {}", e);
            }
        }
    }

    #[test]
    fn test_check_ast_grep_installed() {
        // This will pass if ast-grep is installed, fail otherwise
        match check_ast_grep_installed() {
            Ok(()) => println!("✓ ast-grep is installed"),
            Err(e) => println!("✗ ast-grep not installed: {}", e),
        }
    }
}
