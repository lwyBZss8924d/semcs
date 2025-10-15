// Get your Jina AI API key for free: https://jina.ai/?sui=apikey

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::Embedder;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum JinaInput {
    Text(String),
    TextObject { text: String },
}

#[derive(Debug, Serialize)]
struct JinaEmbeddingRequest {
    model: String,
    input: Vec<JinaInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncate: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct JinaEmbeddingResponse {
    data: Vec<JinaEmbedding>,
    #[allow(dead_code)]
    usage: JinaUsage,
}

#[derive(Debug, Deserialize)]
struct JinaEmbedding {
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct JinaUsage {
    #[allow(dead_code)]
    total_tokens: usize,
}

/// Jina AI API embedder supporting code-specific embedding models
#[derive(Debug)]
pub struct JinaApiEmbedder {
    client: reqwest::Client,
    api_key: String,
    model_name: String,
    dimensions: usize,
    task: Option<String>,
    api_url: String,
    use_object_input: bool, // Use {"text": "..."} format for v4 models
}

impl JinaApiEmbedder {
    /// Create a new Jina API embedder
    ///
    /// # Arguments
    /// * `model_name` - The model identifier (e.g., "jina-code-embeddings-1.5b", "jina-code-embeddings-0.5b")
    /// * `dimensions` - Output embedding dimensions
    /// * `task` - Task type for code embeddings (e.g., "nl2code.query", "nl2code.passage", "code2code.query")
    ///
    /// # Environment Variables
    /// * `JINA_API_KEY` - Required API key for authentication
    pub fn new(model_name: &str, dimensions: usize, task: Option<&str>) -> Result<Self> {
        let api_key = std::env::var("JINA_API_KEY")
            .context("JINA_API_KEY environment variable not set. Get your key at: https://jina.ai/?sui=apikey")?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(1800)) // 30 minutes for large indexing operations
            .build()
            .context("Failed to create HTTP client")?;

        // Detect if we should use object input format (for v4 and other multimodal models)
        let use_object_input =
            model_name.contains("embeddings-v") && !model_name.contains("code-embeddings");

        Ok(Self {
            client,
            api_key,
            model_name: model_name.to_string(),
            dimensions,
            task: task.map(|t| t.to_string()),
            api_url: "https://api.jina.ai/v1/embeddings".to_string(),
            use_object_input,
        })
    }

    /// Create embedder with custom API URL (useful for testing or self-hosted endpoints)
    pub fn new_with_url(
        model_name: &str,
        dimensions: usize,
        task: Option<&str>,
        api_url: &str,
    ) -> Result<Self> {
        let mut embedder = Self::new(model_name, dimensions, task)?;
        embedder.api_url = api_url.to_string();
        Ok(embedder)
    }

    // Helper methods for handling API byte limit
    fn split_text(text: &str, max_bytes: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for line in text.lines() {
            let line_with_newline = format!("{}\n", line);

            // If adding this line would exceed limit, start a new chunk
            if !current_chunk.is_empty()
                && (current_chunk.as_bytes().len() + line_with_newline.as_bytes().len()) > max_bytes
            {
                chunks.push(current_chunk);
                current_chunk = String::new();
            }

            current_chunk.push_str(&line_with_newline);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        // If no chunks were created (single long line), split by bytes
        if chunks.is_empty() {
            let bytes = text.as_bytes();
            for chunk_bytes in bytes.chunks(max_bytes) {
                if let Ok(chunk_str) = std::str::from_utf8(chunk_bytes) {
                    chunks.push(chunk_str.to_string());
                }
            }
        }

        chunks
    }

    fn average_embeddings(embeddings: &[Vec<f32>]) -> Result<Vec<f32>> {
        if embeddings.is_empty() {
            anyhow::bail!("Cannot average empty embeddings");
        }

        let dim = embeddings[0].len();
        let mut averaged = vec![0.0; dim];

        for emb in embeddings {
            for (i, &val) in emb.iter().enumerate() {
                averaged[i] += val;
            }
        }

        let count = embeddings.len() as f32;
        for val in &mut averaged {
            *val /= count;
        }

        Ok(averaged)
    }

    fn embed_single(&mut self, text: &str) -> Result<Vec<f32>> {
        // Choose input format based on model type
        let input = if self.use_object_input {
            // Use object format for v4 models (supports larger inputs)
            vec![JinaInput::TextObject {
                text: text.to_string(),
            }]
        } else {
            // Use string format for code models
            vec![JinaInput::Text(text.to_string())]
        };

        let request = JinaEmbeddingRequest {
            model: self.model_name.clone(),
            input,
            task: self.task.clone(),
            dimensions: Some(self.dimensions), // Request specific dimensions (for v4 Matryoshka)
            truncate: Some(true),              // Auto-truncate long inputs
        };

        // Handle both in-runtime and out-of-runtime scenarios
        let response = if tokio::runtime::Handle::try_current().is_ok() {
            // We're already in a tokio runtime, use block_in_place
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let response = self
                        .client
                        .post(&self.api_url)
                        .header("Content-Type", "application/json")
                        .header("Authorization", format!("Bearer {}", self.api_key))
                        .header("Accept", "application/json")
                        .json(&request)
                        .send()
                        .await
                        .context("Failed to send request to Jina API")?;

                    // Check status and provide detailed error
                    let status = response.status();
                    if !status.is_success() {
                        let error_body = response
                            .text()
                            .await
                            .unwrap_or_else(|_| "Could not read error body".to_string());
                        anyhow::bail!(
                            "Jina API error ({}): {} - Model: {}, Input count: {}",
                            status,
                            error_body,
                            self.model_name,
                            request.input.len()
                        );
                    }

                    response
                        .json::<JinaEmbeddingResponse>()
                        .await
                        .context("Failed to parse Jina API response")
                })
            })?
        } else {
            // No runtime, create a new one
            tokio::runtime::Runtime::new()?.block_on(async {
                let response = self
                    .client
                    .post(&self.api_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .header("Accept", "application/json")
                    .json(&request)
                    .send()
                    .await
                    .context("Failed to send request to Jina API")?;

                // Check status and provide detailed error
                let status = response.status();
                if !status.is_success() {
                    let error_body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read error body".to_string());
                    anyhow::bail!(
                        "Jina API error ({}): {} - Model: {}, Input count: {}",
                        status,
                        error_body,
                        self.model_name,
                        request.input.len()
                    );
                }

                response
                    .json::<JinaEmbeddingResponse>()
                    .await
                    .context("Failed to parse Jina API response")
            })?
        };

        // Extract embeddings and truncate to desired dimensions
        let embeddings: Vec<Vec<f32>> = response
            .data
            .into_iter()
            .map(|e| {
                let mut emb = e.embedding;
                // Truncate to desired dimensions if needed (Matryoshka representation)
                if emb.len() > self.dimensions {
                    emb.truncate(self.dimensions);
                }
                emb
            })
            .collect();

        // Verify dimensions match after truncation
        if let Some(first) = embeddings.first()
            && first.len() != self.dimensions
        {
            anyhow::bail!(
                "Dimension mismatch: expected {}, got {}",
                self.dimensions,
                first.len()
            );
        }

        // Return the first (and only) embedding
        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No embedding returned from API"))
    }
}

impl Embedder for JinaApiEmbedder {
    fn id(&self) -> &'static str {
        "jina-api"
    }

    fn dim(&self) -> usize {
        self.dimensions
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn embed(&mut self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        let mut all_embeddings = Vec::new();

        for text in texts {
            // Clean text: remove null bytes and other control characters
            let cleaned: String = text
                .chars()
                .filter(|c| *c == '\n' || *c == '\t' || *c == '\r' || (!c.is_control()))
                .collect();

            if self.use_object_input {
                // v4 models support large inputs (8K+ tokens) - no need to split
                let embedding = self.embed_single(&cleaned)?;
                all_embeddings.push(embedding);
            } else {
                // jina-code models have ~1KB limit - need to split and average
                const MAX_BYTES: usize = 1000; // Conservative limit (~1012 bytes actual)

                if cleaned.as_bytes().len() <= MAX_BYTES {
                    let embedding = self.embed_single(&cleaned)?;
                    all_embeddings.push(embedding);
                } else {
                    // Split into chunks and average embeddings
                    let chunks = Self::split_text(&cleaned, MAX_BYTES);
                    let chunk_embeddings: Vec<Vec<f32>> = chunks
                        .iter()
                        .map(|chunk| self.embed_single(chunk))
                        .collect::<Result<Vec<_>>>()?;

                    // Average the embeddings
                    let averaged = Self::average_embeddings(&chunk_embeddings)?;
                    all_embeddings.push(averaged);
                }
            }
        }

        Ok(all_embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_jina_api_embedder_requires_api_key() {
        // Clear environment variable to test error case
        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }

        let result = JinaApiEmbedder::new("jina-code-embeddings-1.5b", 768, Some("nl2code.query"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JINA_API_KEY"));
    }

    #[test]
    #[serial]
    fn test_jina_api_embedder_creation_with_api_key() {
        // Set a dummy API key for testing
        unsafe {
            std::env::set_var("JINA_API_KEY", "test_key_123");
        }

        let embedder =
            JinaApiEmbedder::new("jina-code-embeddings-1.5b", 768, Some("nl2code.query"));

        assert!(embedder.is_ok());
        let embedder = embedder.unwrap();
        assert_eq!(embedder.id(), "jina-api");
        assert_eq!(embedder.dim(), 768);
        assert_eq!(embedder.model_name(), "jina-code-embeddings-1.5b");

        // Cleanup
        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }
    }

    #[test]
    #[serial]
    fn test_jina_api_embedder_empty_input() {
        unsafe {
            std::env::set_var("JINA_API_KEY", "test_key_123");
        }

        let mut embedder =
            JinaApiEmbedder::new("jina-code-embeddings-0.5b", 512, Some("code2code.query"))
                .unwrap();

        let texts: Vec<String> = vec![];
        let result = embedder.embed(&texts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }
    }

    #[tokio::test]
    #[ignore] // Requires actual API key and network access
    async fn test_jina_api_real_request() {
        // This test requires JINA_API_KEY to be set in environment
        if std::env::var("JINA_API_KEY").is_err() {
            return;
        }

        let mut embedder =
            JinaApiEmbedder::new("jina-code-embeddings-1.5b", 768, Some("nl2code.query")).unwrap();

        let texts = vec![
            "Calculates the square of a number. Parameters: number (int or float) - The number to square. Returns: int or float - The square of the number.".to_string(),
        ];

        let result = embedder.embed(&texts);
        assert!(result.is_ok());

        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), 1);
        assert_eq!(embeddings[0].len(), 768);

        // Real embeddings should not be all zeros
        assert!(!embeddings[0].iter().all(|&x| x == 0.0));
    }
}
