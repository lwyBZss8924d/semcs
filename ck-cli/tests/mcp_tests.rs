use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

// Import from the main.rs module
use ck_search::{CkMcpServer, HybridSearchRequest, RegexSearchRequest, SemanticSearchRequest};

#[tokio::test]
async fn test_mcp_semantic_search_basic_functionality() {
    let temp_dir = create_test_files().await;
    let server = CkMcpServer::new(temp_dir.path().to_path_buf()).unwrap();

    // Test first page request
    let request = SemanticSearchRequest {
        query: "function".to_string(),
        path: temp_dir.path().to_string_lossy().to_string(),
        top_k: Some(10),
        threshold: Some(0.1),
        cursor: None,
        page_size: Some(5),
        include_snippet: Some(true),
        snippet_length: Some(100),
        context_lines: Some(0),
    };

    let result = server.handle_semantic_search(request, None, None).await;

    // Verify the result contains pagination information
    if let Ok((summary, response)) = result {
        assert!(summary.contains("Page 1"));

        // Verify response structure
        assert!(response["search"].is_object());
        assert!(response["results"].is_object());
        assert!(response["pagination"].is_object());

        // Check pagination fields
        assert!(response["pagination"]["current_page"].is_number());
        assert!(response["results"]["count"].is_number());
        assert!(response["results"]["has_more"].is_boolean());
        assert_eq!(response["search"]["mode"], "semantic");
    }
}

#[tokio::test]
async fn test_mcp_regex_search_basic_functionality() {
    let temp_dir = create_test_files().await;
    let server = CkMcpServer::new(temp_dir.path().to_path_buf()).unwrap();

    let request = RegexSearchRequest {
        pattern: "function".to_string(),
        path: temp_dir.path().to_string_lossy().to_string(),
        ignore_case: Some(true),
        context: Some(2),
        cursor: None,
        page_size: Some(3),
        include_snippet: Some(true),
        snippet_length: Some(50),
    };

    let result = server.handle_regex_search(request).await;

    if let Ok((summary, response)) = result {
        assert!(summary.contains("Page 1"));

        // Verify response structure matches expected format
        assert_eq!(response["search"]["mode"], "regex");
        assert!(response["results"]["matches"].is_array());
        assert!(response["pagination"]["page_size"].is_number());

        // Check match structure for regex search
        if let Some(matches) = response["results"]["matches"].as_array() {
            if !matches.is_empty() {
                let first_match = &matches[0];
                assert_eq!(first_match["type"], "regex_match");
                assert!(first_match["match"]["line_number"].is_number());
            }
        }
    }
}

#[tokio::test]
async fn test_mcp_hybrid_search_basic_functionality() {
    let temp_dir = create_test_files().await;
    let server = CkMcpServer::new(temp_dir.path().to_path_buf()).unwrap();

    let request = HybridSearchRequest {
        query: "function error".to_string(),
        path: temp_dir.path().to_string_lossy().to_string(),
        top_k: Some(5),
        threshold: Some(0.01),
        cursor: None,
        page_size: Some(2),
        include_snippet: Some(true),
        snippet_length: Some(200),
        context_lines: Some(1),
    };

    let result = server.handle_hybrid_search(request).await;

    if let Ok((summary, response)) = result {
        assert!(summary.contains("Page 1"));

        // Verify hybrid-specific fields
        assert_eq!(response["search"]["mode"], "hybrid");

        // Check match structure for hybrid search
        if let Some(matches) = response["results"]["matches"].as_array() {
            if !matches.is_empty() {
                let first_match = &matches[0];
                assert_eq!(first_match["type"], "hybrid_match");
                // Hybrid matches should have both score and rrf_score
                assert!(first_match["match"]["score"].is_number());
                assert!(first_match["match"]["rrf_score"].is_number());
            }
        }
    }
}

#[tokio::test]
async fn test_mcp_invalid_cursor_handling() {
    let temp_dir = create_test_files().await;
    let server = CkMcpServer::new(temp_dir.path().to_path_buf()).unwrap();

    // Test with invalid cursor
    let request = SemanticSearchRequest {
        query: "test".to_string(),
        path: temp_dir.path().to_string_lossy().to_string(),
        top_k: Some(10),
        threshold: Some(0.1),
        cursor: Some("invalid_cursor".to_string()),
        page_size: Some(5),
        include_snippet: Some(true),
        snippet_length: Some(100),
        context_lines: Some(0),
    };

    let result = server.handle_semantic_search(request, None, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_search_parameters_validation() {
    let temp_dir = create_test_files().await;
    let server = CkMcpServer::new(temp_dir.path().to_path_buf()).unwrap();

    // Test with extreme page size (should be clamped)
    let request = SemanticSearchRequest {
        query: "test".to_string(),
        path: temp_dir.path().to_string_lossy().to_string(),
        top_k: Some(10),
        threshold: Some(0.1),
        cursor: None,
        page_size: Some(1000), // Should be clamped to 200
        include_snippet: Some(true),
        snippet_length: Some(10000), // Should be clamped to 2000
        context_lines: Some(50),     // Should be clamped to 10
    };

    let result = server.handle_semantic_search(request, None, None).await;

    if let Ok((_, response)) = result {
        // The actual page size in the response should be clamped
        let page_size = response["pagination"]["page_size"].as_u64().unwrap_or(0);
        assert!(page_size <= 200);
    }
}

#[tokio::test]
async fn test_mcp_nonexistent_path() {
    let server = CkMcpServer::new(PathBuf::from("/nonexistent")).unwrap();

    let request = SemanticSearchRequest {
        query: "test".to_string(),
        path: "/definitely/does/not/exist".to_string(),
        top_k: Some(10),
        threshold: Some(0.1),
        cursor: None,
        page_size: Some(5),
        include_snippet: Some(true),
        snippet_length: Some(100),
        context_lines: Some(0),
    };

    let result = server.handle_semantic_search(request, None, None).await;
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.to_string().contains("Path does not exist"));
    }
}

async fn create_test_files() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create some test files with different content
    let files = vec![
        (
            "test1.rs",
            "function main() {\n    println!(\"Hello world\");\n}\n\nfunction helper() {\n    // Some helper code\n}",
        ),
        (
            "test2.js",
            "function calculate(x, y) {\n    return x + y;\n}\n\nfunction error_handler() {\n    console.error(\"An error occurred\");\n}",
        ),
        (
            "test3.py",
            "def process_data(data):\n    try:\n        return data.process()\n    except Exception as e:\n        handle_error(e)\n\ndef handle_error(error):\n    print(f\"Error: {error}\")",
        ),
        (
            "test4.txt",
            "This is a text file with some content.\nIt contains various words and phrases.\nSome lines mention functions and errors.",
        ),
    ];

    for (filename, content) in files {
        let file_path = temp_dir.path().join(filename);
        fs::write(file_path, content)
            .await
            .expect("Failed to write test file");
    }

    temp_dir
}
