use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// User-level configuration stored in system config directory
/// Location: ~/.config/cs/config.toml (Linux/macOS) or %APPDATA%\cs\config.toml (Windows)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    // Model configuration - Hybrid Strategy
    /// Model to use for indexing (default: jina-v4 for large file support)
    pub index_model: String,

    /// Model to use for querying (default: jina-code-1.5b for code-optimized search)
    pub query_model: String,

    // Search defaults
    /// Default number of top results to return
    pub default_topk: usize,

    /// Default similarity threshold
    pub default_threshold: f32,

    /// Default search mode: "regex", "sem", "lex", or "hybrid"
    pub default_search_mode: String,

    // Output formatting
    /// Default output format: "text", "json", or "jsonl"
    pub default_output_format: String,

    /// Show similarity scores by default
    pub show_scores_default: bool,

    /// Show line numbers by default
    pub line_numbers_default: bool,

    // Reranking
    /// Enable reranking by default
    pub rerank_enabled: bool,

    /// Reranking model to use ("jina" or "bge")
    pub rerank_model: String,

    // Other preferences
    /// Quiet mode (suppress status messages)
    pub quiet_mode: bool,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            // Hybrid strategy: jina-v4 for indexing (handles large files),
            // jina-code-1.5b for querying (code-optimized semantic search)
            index_model: "jina-v4".to_string(),
            query_model: "jina-code-1.5b".to_string(),

            // Search defaults
            default_topk: 10,
            default_threshold: 0.6,
            default_search_mode: "regex".to_string(),

            // Output defaults
            default_output_format: "text".to_string(),
            show_scores_default: false,
            line_numbers_default: false,

            // Reranking defaults
            rerank_enabled: false,
            rerank_model: "jina".to_string(),

            // Other defaults
            quiet_mode: false,
        }
    }
}

impl UserConfig {
    /// Get the system configuration directory for cc
    pub fn config_dir() -> Result<PathBuf> {
        directories::ProjectDirs::from("", "", "cs")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))
    }

    /// Get the full path to the configuration file
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load configuration from file, or return defaults if file doesn't exist
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Self = toml::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse config.toml: {}", e))?;
            Ok(config)
        } else {
            // No config file exists, return defaults
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;

        // Create config directory if it doesn't exist
        std::fs::create_dir_all(&dir)?;

        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        Ok(())
    }

    /// Get a configuration value by key
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "index-model" | "index_model" => Some(self.index_model.clone()),
            "query-model" | "query_model" => Some(self.query_model.clone()),
            "default-topk" | "default_topk" => Some(self.default_topk.to_string()),
            "default-threshold" | "default_threshold" => Some(self.default_threshold.to_string()),
            "default-search-mode" | "default_search_mode" => Some(self.default_search_mode.clone()),
            "default-output-format" | "default_output_format" => {
                Some(self.default_output_format.clone())
            }
            "show-scores-default" | "show_scores_default" => {
                Some(self.show_scores_default.to_string())
            }
            "line-numbers-default" | "line_numbers_default" => {
                Some(self.line_numbers_default.to_string())
            }
            "rerank-enabled" | "rerank_enabled" => Some(self.rerank_enabled.to_string()),
            "rerank-model" | "rerank_model" => Some(self.rerank_model.clone()),
            "quiet-mode" | "quiet_mode" => Some(self.quiet_mode.to_string()),
            _ => None,
        }
    }

    /// Set a configuration value by key
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "index-model" | "index_model" => {
                self.index_model = value.to_string();
                Ok(())
            }
            "query-model" | "query_model" => {
                self.query_model = value.to_string();
                Ok(())
            }
            "default-topk" | "default_topk" => {
                self.default_topk = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number for default-topk: {}", value))?;
                Ok(())
            }
            "default-threshold" | "default_threshold" => {
                self.default_threshold = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid number for default-threshold: {}", value)
                })?;
                Ok(())
            }
            "default-search-mode" | "default_search_mode" => {
                if !["regex", "sem", "lex", "hybrid"].contains(&value) {
                    return Err(anyhow::anyhow!(
                        "Invalid search mode: {}. Must be one of: regex, sem, lex, hybrid",
                        value
                    ));
                }
                self.default_search_mode = value.to_string();
                Ok(())
            }
            "default-output-format" | "default_output_format" => {
                if !["text", "json", "jsonl"].contains(&value) {
                    return Err(anyhow::anyhow!(
                        "Invalid output format: {}. Must be one of: text, json, jsonl",
                        value
                    ));
                }
                self.default_output_format = value.to_string();
                Ok(())
            }
            "show-scores-default" | "show_scores_default" => {
                self.show_scores_default = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid boolean for show-scores-default: {}", value)
                })?;
                Ok(())
            }
            "line-numbers-default" | "line_numbers_default" => {
                self.line_numbers_default = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid boolean for line-numbers-default: {}", value)
                })?;
                Ok(())
            }
            "rerank-enabled" | "rerank_enabled" => {
                self.rerank_enabled = value.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid boolean for rerank-enabled: {}", value)
                })?;
                Ok(())
            }
            "rerank-model" | "rerank_model" => {
                // Allow both aliases and full model names
                let valid_aliases = ["jina", "jina-v1", "jina-v2", "jina-v3", "bge", "bge-base", "bge-v2-m3"];
                let is_full_name = value.starts_with("jina-reranker-") || value.starts_with("BAAI/") || value.starts_with("rozgo/");

                if !valid_aliases.contains(&value) && !is_full_name {
                    return Err(anyhow::anyhow!(
                        "Invalid rerank model: {}. Must be one of: jina, jina-v1, jina-v2, bge, bge-base, bge-v2-m3, or a full model name",
                        value
                    ));
                }
                self.rerank_model = value.to_string();
                Ok(())
            }
            "quiet-mode" | "quiet_mode" => {
                self.quiet_mode = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean for quiet-mode: {}", value))?;
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Unknown configuration key: {}", key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UserConfig::default();
        assert_eq!(config.index_model, "jina-v4");
        assert_eq!(config.query_model, "jina-code-1.5b");
        assert_eq!(config.default_topk, 10);
        assert_eq!(config.default_threshold, 0.6);
    }

    #[test]
    fn test_get_set() {
        let mut config = UserConfig::default();

        // Test get
        assert_eq!(config.get("index-model"), Some("jina-v4".to_string()));

        // Test set
        config.set("index-model", "bge-small").unwrap();
        assert_eq!(config.index_model, "bge-small");
        assert_eq!(config.get("index-model"), Some("bge-small".to_string()));

        // Test invalid value
        assert!(config.set("default-topk", "not-a-number").is_err());
    }

    #[test]
    fn test_toml_serialization() {
        let config = UserConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();

        // Verify it contains expected keys
        assert!(toml_str.contains("index_model"));
        assert!(toml_str.contains("query_model"));
        assert!(toml_str.contains("jina-v4"));
        assert!(toml_str.contains("jina-code-1.5b"));
    }
}
