use ratatui::style::Color;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

pub fn theme_set() -> &'static ThemeSet {
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

pub fn score_to_color(score: f32) -> Color {
    if score >= 0.875 {
        Color::Rgb(0, 255, 100) // Bright green
    } else if score >= 0.75 {
        Color::Rgb(0, 200, 80)
    } else if score >= 0.625 {
        Color::Rgb(0, 160, 70)
    } else if score >= 0.5 {
        Color::Rgb(100, 140, 60)
    } else {
        Color::Rgb(140, 140, 140) // Gray
    }
}

// Token-level similarity calculation for heatmap highlighting
pub fn split_into_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in text.chars() {
        match ch {
            ' ' | '\t' | '\n' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
            }
            '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':' | '.' | '!' | '?' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

pub fn calculate_token_similarity(token: &str, pattern: &str) -> f32 {
    // Skip whitespace and punctuation
    if token.trim().is_empty() || token.chars().all(|c| !c.is_alphanumeric()) {
        return 0.0;
    }

    let token_lower = token.to_lowercase();
    let pattern_lower = pattern.to_lowercase();

    // Exact match gets highest score
    if token_lower == pattern_lower {
        return 1.0;
    }

    // Check if token contains any pattern words or vice versa
    let pattern_words: Vec<&str> = pattern_lower.split_whitespace().collect();
    let mut max_score: f32 = 0.0;

    for pattern_word in &pattern_words {
        if pattern_word.len() < 3 {
            continue; // Skip short words
        }

        // Exact word match
        if token_lower == *pattern_word {
            max_score = max_score.max(0.9);
        }
        // Substring match
        else if token_lower.contains(pattern_word) {
            let ratio = pattern_word.len() as f32 / token_lower.len() as f32;
            max_score = max_score.max(0.6 * ratio);
        }
        // Pattern word contains token
        else if pattern_word.contains(&token_lower) && token_lower.len() >= 3 {
            let ratio = token_lower.len() as f32 / pattern_word.len() as f32;
            max_score = max_score.max(0.5 * ratio);
        }
        // Fuzzy similarity for related terms
        else {
            let similarity = calculate_fuzzy_similarity(&token_lower, pattern_word);
            max_score = max_score.max(similarity * 0.4);
        }
    }

    max_score
}

pub fn calculate_fuzzy_similarity(s1: &str, s2: &str) -> f32 {
    // Simple edit distance-based similarity
    if s1.is_empty() || s2.is_empty() || s1.len() < 3 || s2.len() < 3 {
        return 0.0;
    }

    let len1 = s1.len();
    let len2 = s2.len();
    let max_len = len1.max(len2);

    // Count common characters
    let s1_chars: HashSet<char> = s1.chars().collect();
    let s2_chars: HashSet<char> = s2.chars().collect();
    let common_chars = s1_chars.intersection(&s2_chars).count();

    // Similarity based on common characters
    common_chars as f32 / max_len as f32
}

pub fn apply_heatmap_color_to_token(token: &str, score: f32) -> Color {
    // Skip coloring whitespace and punctuation
    if token.trim().is_empty() || token.chars().all(|c| !c.is_alphanumeric()) {
        return Color::Reset;
    }

    // 8-step linear gradient: grey to green with bright final step
    match score {
        s if s >= 0.875 => Color::Rgb(0, 255, 100), // Extra bright green
        s if s >= 0.75 => Color::Rgb(0, 180, 80),
        s if s >= 0.625 => Color::Rgb(0, 160, 70),
        s if s >= 0.5 => Color::Rgb(0, 140, 60),
        s if s >= 0.375 => Color::Rgb(50, 120, 80),
        s if s >= 0.25 => Color::Rgb(100, 130, 100),
        s if s >= 0.125 => Color::Rgb(140, 140, 140),
        s if s > 0.0 => Color::Rgb(180, 180, 180),
        _ => Color::Reset,
    }
}

pub fn find_repo_root(path: &Path) -> Option<PathBuf> {
    let mut current = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };

    loop {
        if current.join(".ck").exists() {
            return Some(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}
