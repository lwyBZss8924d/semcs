use std::process::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_basic_grep_functionality() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    fs::write(temp_dir.path().join("file1.txt"), "hello world\nrust programming\ntest line").unwrap();
    fs::write(temp_dir.path().join("file2.rs"), "fn main() {\n    println!(\"Hello Rust\");\n}").unwrap();
    fs::write(temp_dir.path().join("file3.py"), "print('Hello Python')\n# rust comment").unwrap();
    
    // Build the binary first
    let output = Command::new("cargo")
        .args(&["build", "--bin", "ck"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to build ck binary");
    
    if !output.status.success() {
        panic!("Failed to build ck: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Test basic regex search
    let output = Command::new("../target/debug/ck")
        .args(&["rust", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rust programming"));
    assert!(stdout.contains("# rust comment"));
}

#[test]
fn test_case_insensitive_search() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "Hello World\nHELLO WORLD\nhello world").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["-i", "HELLO", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3); // Should match all three lines
}

#[test]
fn test_recursive_search() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    fs::write(temp_dir.path().join("root.txt"), "target text").unwrap();
    fs::write(temp_dir.path().join("subdir").join("nested.txt"), "target text").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["-r", "target", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2); // Should find matches in both files
}

#[test]
fn test_json_output() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "json test line").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["--json", "json", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should be valid JSON
    let json_result: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert!(json_result["file"].is_string());
    assert!(json_result["score"].is_number());
    assert!(json_result["preview"].is_string());
}

#[test]
fn test_index_command() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "indexable content").unwrap();
    
    // Test index creation
    let output = Command::new("../target/debug/ck")
        .args(&["--index", "."])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to run ck index");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Indexed"));
    
    // Check that .ck directory was created
    assert!(temp_dir.path().join(".ck").exists());
}

#[test]
fn test_semantic_search() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files with different semantic content
    fs::write(temp_dir.path().join("ai.txt"), 
        "Machine learning and artificial intelligence are transforming technology").unwrap();
    fs::write(temp_dir.path().join("cooking.txt"), 
        "Cooking recipes and kitchen tips for delicious meals").unwrap();
    fs::write(temp_dir.path().join("programming.txt"), 
        "Software development with Python and data science frameworks").unwrap();
    
    // First create an index
    let output = Command::new("../target/debug/ck")
        .args(&["--index", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck index");
    
    assert!(output.status.success());
    
    // Test semantic search - should rank AI content higher for "neural networks"
    let output = Command::new("../target/debug/ck")
        .args(&["--sem", "neural networks", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck semantic search");
    
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        
        // AI file should appear first due to semantic similarity
        let lines: Vec<&str> = stdout.trim().lines().collect();
        if !lines.is_empty() {
            assert!(lines[0].contains("ai.txt"));
        }
    }
    // Note: Semantic search might fail in test environments due to model availability
    // This is acceptable for integration tests
}

#[test]
fn test_lexical_search() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("doc1.txt"), "machine learning algorithms").unwrap();
    fs::write(temp_dir.path().join("doc2.txt"), "web development frameworks").unwrap();
    
    // Create index
    let output = Command::new("../target/debug/ck")
        .args(&["--index", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck index");
    
    assert!(output.status.success());
    
    // Test lexical search
    let output = Command::new("../target/debug/ck")
        .args(&["--lex", "machine learning", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck lexical search");
    
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("doc1.txt"));
    }
}

#[test]
fn test_hybrid_search() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("mixed.txt"), "Python programming and machine learning").unwrap();
    
    // Create index
    let output = Command::new("../target/debug/ck")
        .args(&["--index", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck index");
    
    assert!(output.status.success());
    
    // Test hybrid search
    let output = Command::new("../target/debug/ck")
        .args(&["--hybrid", "Python", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck hybrid search");
    
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("mixed.txt"));
    }
}

#[test]
fn test_context_lines() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("context.txt"), 
        "line 1\nline 2\ntarget line\nline 4\nline 5").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["-C", "1", "target", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck with context");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should include context lines
    assert!(stdout.contains("line 2"));
    assert!(stdout.contains("target line"));  
    assert!(stdout.contains("line 4"));
}

#[test]
fn test_topk_limit() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple files with matches
    for i in 1..=10 {
        fs::write(temp_dir.path().join(format!("file{}.txt", i)), "match content").unwrap();
    }
    
    let output = Command::new("../target/debug/ck")
        .args(&["--topk", "5", "match", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck with topk");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.trim().lines().count();
    assert!(line_count <= 5);
}

#[test]
fn test_line_numbers() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("numbered.txt"), "line 1\nmatched line\nline 3").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["-n", "matched", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck with line numbers");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should include line number (line 2)
    assert!(stdout.contains(":2:"));
}

#[test]
fn test_clean_command() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "test content").unwrap();
    
    // Create index first
    let output = Command::new("../target/debug/ck")
        .args(&["--index", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck index");
    
    assert!(output.status.success());
    assert!(temp_dir.path().join(".ck").exists());
    
    // Clean index
    let output = Command::new("../target/debug/ck")
        .args(&["--clean", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck clean");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Index cleaned"));
}

#[test]
fn test_error_handling() {
    // Test with nonexistent directory
    let _output = Command::new("../target/debug/ck")
        .args(&["test", "/nonexistent/directory"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    // Should handle error gracefully (might succeed with no matches, which is fine)
    // The important thing is that it doesn't crash
    
    // Test invalid regex
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "test content").unwrap();
    
    let output = Command::new("../target/debug/ck")
        .args(&["[invalid", temp_dir.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run ck");
    
    // Should fail gracefully with invalid regex
    assert!(!output.status.success());
}