use anyhow::Result;
use ck_core::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub span: Span,
    pub text: String,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkType {
    Text,
    Function,
    Class,
    Method,
    Module,
}

pub fn chunk_text(text: &str, language: Option<&str>) -> Result<Vec<Chunk>> {
    tracing::debug!("Chunking text with language: {:?}, length: {} chars", language, text.len());
    
    let result = match language {
        Some("python") => {
            tracing::debug!("Using Python tree-sitter parser");
            chunk_python(text)
        },
        Some("typescript") | Some("javascript") => {
            tracing::debug!("Using TypeScript/JavaScript tree-sitter parser");
            chunk_typescript(text)
        },
        _ => {
            tracing::debug!("Using generic chunking strategy");
            chunk_generic(text)
        },
    };
    
    match &result {
        Ok(chunks) => tracing::debug!("Successfully created {} chunks", chunks.len()),
        Err(e) => tracing::warn!("Chunking failed: {}", e),
    }
    
    result
}

fn chunk_generic(text: &str) -> Result<Vec<Chunk>> {
    let mut chunks = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let chunk_size = 20;
    let overlap = 5;
    
    let mut i = 0;
    while i < lines.len() {
        let end = (i + chunk_size).min(lines.len());
        let chunk_lines = &lines[i..end];
        let chunk_text = chunk_lines.join("\n");
        
        let byte_start = lines[0..i].iter().map(|l| l.len() + 1).sum::<usize>();
        let byte_end = byte_start + chunk_text.len();
        
        chunks.push(Chunk {
            span: Span {
                byte_start,
                byte_end,
                line_start: i + 1,
                line_end: end,
            },
            text: chunk_text,
            chunk_type: ChunkType::Text,
        });
        
        i += chunk_size - overlap;
        if i >= lines.len() {
            break;
        }
    }
    
    Ok(chunks)
}

fn chunk_python(text: &str) -> Result<Vec<Chunk>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_python::language())?;
    
    let tree = parser.parse(text, None).ok_or_else(|| {
        anyhow::anyhow!("Failed to parse Python code")
    })?;
    
    let mut chunks = Vec::new();
    let mut cursor = tree.root_node().walk();
    
    extract_code_chunks(&mut cursor, text, &mut chunks, "python");
    
    if chunks.is_empty() {
        return chunk_generic(text);
    }
    
    Ok(chunks)
}

fn chunk_typescript(text: &str) -> Result<Vec<Chunk>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_typescript::language_typescript())?;
    
    let tree = parser.parse(text, None).ok_or_else(|| {
        anyhow::anyhow!("Failed to parse TypeScript code")
    })?;
    
    let mut chunks = Vec::new();
    let mut cursor = tree.root_node().walk();
    
    extract_code_chunks(&mut cursor, text, &mut chunks, "typescript");
    
    if chunks.is_empty() {
        return chunk_generic(text);
    }
    
    Ok(chunks)
}

fn extract_code_chunks(
    cursor: &mut tree_sitter::TreeCursor,
    source: &str,
    chunks: &mut Vec<Chunk>,
    language: &str,
) {
    let node = cursor.node();
    let node_kind = node.kind();
    
    let is_chunk = match language {
        "python" => matches!(node_kind, "function_definition" | "class_definition"),
        "typescript" | "javascript" => matches!(
            node_kind,
            "function_declaration" | "class_declaration" | "method_definition" | "arrow_function"
        ),
        _ => false,
    };
    
    if is_chunk {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let start_pos = node.start_position();
        let end_pos = node.end_position();
        
        let text = &source[start_byte..end_byte];
        
        let chunk_type = match node_kind {
            "function_definition" | "function_declaration" | "arrow_function" => ChunkType::Function,
            "class_definition" | "class_declaration" => ChunkType::Class,
            "method_definition" => ChunkType::Method,
            _ => ChunkType::Text,
        };
        
        chunks.push(Chunk {
            span: Span {
                byte_start: start_byte,
                byte_end: end_byte,
                line_start: start_pos.row + 1,
                line_end: end_pos.row + 1,
            },
            text: text.to_string(),
            chunk_type,
        });
    }
    
    if cursor.goto_first_child() {
        loop {
            extract_code_chunks(cursor, source, chunks, language);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}