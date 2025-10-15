use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

mod user_config;
pub use user_config::UserConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub provider: String,
    pub dimensions: usize,
    pub max_tokens: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        let mut models = HashMap::new();

        models.insert(
            "bge-small".to_string(),
            ModelConfig {
                name: "BAAI/bge-small-en-v1.5".to_string(),
                provider: "fastembed".to_string(),
                dimensions: 384,
                max_tokens: 512,
                description: "Small, fast English embedding model".to_string(),
            },
        );

        models.insert(
            "minilm".to_string(),
            ModelConfig {
                name: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
                provider: "fastembed".to_string(),
                dimensions: 384,
                max_tokens: 256,
                description: "Lightweight English embedding model".to_string(),
            },
        );

        // Add enhanced models
        models.insert(
            "nomic-v1.5".to_string(),
            ModelConfig {
                name: "nomic-embed-text-v1.5".to_string(),
                provider: "fastembed".to_string(),
                dimensions: 768,
                max_tokens: 8192,
                description: "High-quality English embedding model with large context window"
                    .to_string(),
            },
        );

        models.insert(
            "jina-code".to_string(),
            ModelConfig {
                name: "jina-embeddings-v2-base-code".to_string(),
                provider: "fastembed".to_string(),
                dimensions: 768,
                max_tokens: 8192,
                description: "Code-specific embedding model optimized for programming tasks"
                    .to_string(),
            },
        );

        // Jina AI API models - use native dimensions (Matryoshka truncation happens client-side)
        models.insert(
            "jina-code-0.5b".to_string(),
            ModelConfig {
                name: "jina-code-embeddings-0.5b".to_string(),
                provider: "jina-api".to_string(),
                dimensions: 896,  // Native: 896d (Matryoshka: 512, 256, 128, 64)
                max_tokens: 32768,
                description: "Jina AI API: 494M parameter code embedding model for NL2Code, code similarity, and cross-language retrieval (requires JINA_API_KEY)"
                    .to_string(),
            },
        );

        models.insert(
            "jina-code-1.5b".to_string(),
            ModelConfig {
                name: "jina-code-embeddings-1.5b".to_string(),
                provider: "jina-api".to_string(),
                dimensions: 1536,  // Native: 1536d (Matryoshka: 1024, 512, 256, 128)
                max_tokens: 32768,
                description: "Jina AI API: 1.54B parameter advanced code embedding model with enhanced retrieval capabilities (requires JINA_API_KEY)"
                    .to_string(),
            },
        );

        models.insert(
            "jina-v3".to_string(),
            ModelConfig {
                name: "jina-embeddings-v3".to_string(),
                provider: "jina-api".to_string(),
                dimensions: 1024,
                max_tokens: 8192,
                description: "Jina AI API: 570M parameter multilingual text embedding model (requires JINA_API_KEY)"
                    .to_string(),
            },
        );

        models.insert(
            "jina-v4".to_string(),
            ModelConfig {
                name: "jina-embeddings-v4".to_string(),
                provider: "jina-api".to_string(),
                dimensions: 1536,  // Using 1536d (Matryoshka from native 2048d) for compatibility with jina-code-1.5b
                max_tokens: 8192,
                description: "Jina AI API: 3.8B parameter multimodal embedding model - BEST for indexing large code files (supports 8K+ tokens, outputs 1536d for jina-code-1.5b compatibility) (requires JINA_API_KEY)"
                    .to_string(),
            },
        );

        Self {
            models,
            default_model: "bge-small".to_string(), // Keep BGE as default for backward compatibility
        }
    }
}

impl ModelRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn get_model(&self, name: &str) -> Option<&ModelConfig> {
        self.models.get(name)
    }

    pub fn get_default_model(&self) -> Option<&ModelConfig> {
        self.models.get(&self.default_model)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub model: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub index_backend: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            model: "bge-small".to_string(),
            chunk_size: 512,
            chunk_overlap: 128,
            index_backend: "hnsw".to_string(),
        }
    }
}

impl ProjectConfig {
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
