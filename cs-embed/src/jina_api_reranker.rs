// Jina AI API reranker supporting the latest Jina reranker models
// Get your Jina AI API key for free: https://jina.ai/?sui=apikey

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{Reranker, RerankResult};

#[derive(Debug, Serialize)]
struct JinaRerankRequest {
    model: String,
    query: String,
    documents: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_n: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct JinaRerankResponse {
    #[allow(dead_code)]
    model: String,
    results: Vec<JinaRerankResultItem>,
    #[allow(dead_code)]
    usage: JinaUsage,
}

#[derive(Debug, Deserialize)]
struct JinaRerankResultItem {
    index: usize,
    relevance_score: f32,
    #[allow(dead_code)]
    document: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JinaUsage {
    #[allow(dead_code)]
    total_tokens: usize,
}

/// Jina AI API reranker supporting reranker models
#[derive(Debug)]
pub struct JinaApiReranker {
    client: reqwest::Client,
    api_key: String,
    model_name: String,
    api_url: String,
}

impl JinaApiReranker {
    /// Create a new Jina API reranker
    ///
    /// # Arguments
    /// * `model_name` - The model identifier (e.g., "jina-reranker-v2-base-multilingual", "jina-reranker-v1-turbo-en")
    ///
    /// # Environment Variables
    /// * `JINA_API_KEY` - Required API key for authentication
    pub fn new(model_name: &str) -> Result<Self> {
        let api_key = std::env::var("JINA_API_KEY")
            .context("JINA_API_KEY environment variable not set. Get your key at: https://jina.ai/?sui=apikey")?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60)) // 1 minute for reranking
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key,
            model_name: model_name.to_string(),
            api_url: "https://api.jina.ai/v1/rerank".to_string(),
        })
    }

    /// Create reranker with custom API URL (useful for testing or self-hosted endpoints)
    #[allow(dead_code)]
    pub fn new_with_url(model_name: &str, api_url: &str) -> Result<Self> {
        let mut reranker = Self::new(model_name)?;
        reranker.api_url = api_url.to_string();
        Ok(reranker)
    }
}

impl Reranker for JinaApiReranker {
    fn id(&self) -> &'static str {
        "jina-api-reranker"
    }

    fn rerank(&mut self, query: &str, documents: &[String]) -> Result<Vec<RerankResult>> {
        if documents.is_empty() {
            return Ok(vec![]);
        }

        let request = JinaRerankRequest {
            model: self.model_name.clone(),
            query: query.to_string(),
            documents: documents.to_vec(),
            top_n: None, // Return all results, let caller decide filtering
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
                        .context("Failed to send request to Jina Rerank API")?;

                    // Check status and provide detailed error
                    let status = response.status();
                    if !status.is_success() {
                        let error_body = response
                            .text()
                            .await
                            .unwrap_or_else(|_| "Could not read error body".to_string());
                        anyhow::bail!(
                            "Jina Rerank API error ({}): {} - Model: {}, Documents: {}",
                            status,
                            error_body,
                            self.model_name,
                            documents.len()
                        );
                    }

                    response
                        .json::<JinaRerankResponse>()
                        .await
                        .context("Failed to parse Jina Rerank API response")
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
                    .context("Failed to send request to Jina Rerank API")?;

                // Check status and provide detailed error
                let status = response.status();
                if !status.is_success() {
                    let error_body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not read error body".to_string());
                    anyhow::bail!(
                        "Jina Rerank API error ({}): {} - Model: {}, Documents: {}",
                        status,
                        error_body,
                        self.model_name,
                        documents.len()
                    );
                }

                response
                    .json::<JinaRerankResponse>()
                    .await
                    .context("Failed to parse Jina Rerank API response")
            })?
        };

        // Convert API response to RerankResult format
        // Note: Jina API returns results sorted by relevance score (highest first)
        let results: Vec<RerankResult> = response
            .results
            .into_iter()
            .map(|item| RerankResult {
                query: query.to_string(),
                document: documents[item.index].clone(),
                score: item.relevance_score,
            })
            .collect();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_jina_api_reranker_requires_api_key() {
        // Clear environment variable to test error case
        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }

        let result = JinaApiReranker::new("jina-reranker-v2-base-multilingual");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JINA_API_KEY"));
    }

    #[test]
    #[serial]
    fn test_jina_api_reranker_creation_with_api_key() {
        // Set a dummy API key for testing
        unsafe {
            std::env::set_var("JINA_API_KEY", "test_key_123");
        }

        let reranker = JinaApiReranker::new("jina-reranker-v2-base-multilingual");

        assert!(reranker.is_ok());
        let reranker = reranker.unwrap();
        assert_eq!(reranker.id(), "jina-api-reranker");
        assert_eq!(reranker.model_name, "jina-reranker-v2-base-multilingual");

        // Cleanup
        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }
    }

    #[test]
    #[serial]
    fn test_jina_api_reranker_empty_input() {
        unsafe {
            std::env::set_var("JINA_API_KEY", "test_key_123");
        }

        let mut reranker = JinaApiReranker::new("jina-reranker-v2-base-multilingual").unwrap();

        let documents: Vec<String> = vec![];
        let result = reranker.rerank("test query", &documents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        unsafe {
            std::env::remove_var("JINA_API_KEY");
        }
    }

    #[tokio::test]
    #[ignore] // Requires actual API key and network access
    async fn test_jina_api_real_rerank_request() {
        // This test requires JINA_API_KEY to be set in environment
        if std::env::var("JINA_API_KEY").is_err() {
            return;
        }

        let mut reranker =
            JinaApiReranker::new("jina-reranker-v2-base-multilingual").unwrap();

        let query = "programming in Rust";
        let documents = vec![
            "Rust is a systems programming language".to_string(),
            "Python is great for data science".to_string(),
            "Rust has memory safety guarantees".to_string(),
        ];

        let result = reranker.rerank(query, &documents);
        assert!(result.is_ok());

        let results = result.unwrap();
        assert_eq!(results.len(), 3);

        // First result should be most relevant to Rust programming
        assert!(results[0].document.contains("Rust"));
        assert!(results[0].score > results[2].score); // Rust docs should score higher than Python
    }
}
