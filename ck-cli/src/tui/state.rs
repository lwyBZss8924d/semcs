use ck_core::{SearchMode, SearchResult, Span};
use ratatui::{
    style::Color,
    text::Line,
};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_chunks: usize,
}

#[derive(Debug, Clone)]
pub struct IndexedChunkMeta {
    pub span: Span,
    pub chunk_type: Option<String>,
    pub breadcrumb: Option<String>,
    pub ancestry: Vec<String>,
    pub estimated_tokens: Option<usize>,
    pub byte_length: Option<usize>,
    pub leading_trivia: Option<Vec<String>>,
    pub trailing_trivia: Option<Vec<String>>,
}

// Enhanced chunk colors for better visualization
pub const COLOR_CHUNK_HIGHLIGHT: Color = Color::Rgb(255, 165, 0); // Orange - highlighted chunk
pub const COLOR_CHUNK_BOUNDARY: Color = Color::Rgb(0, 255, 127); // Spring green - chunk boundaries
pub const COLOR_CHUNK_TEXT: Color = Color::Rgb(255, 255, 255); // Bright white - highlighted chunk text
pub const COLOR_CHUNK_LINE_NUM: Color = Color::Rgb(255, 215, 0); // Gold - highlighted chunk line numbers

pub const SPINNER_FRAMES: [char; 4] = ['|', '/', '-', '\\'];

#[derive(Debug)]
pub enum UiEvent {
    Indexing {
        generation: u64,
        message: String,
        progress: Option<f32>,
    },
    IndexingDone {
        generation: u64,
    },
    SearchProgress {
        generation: u64,
        message: String,
    },
    SearchCompleted {
        generation: u64,
        results: Vec<SearchResult>,
        summary: String,
        query: String,
    },
    SearchFailed {
        generation: u64,
        error: String,
    },
}

#[derive(Clone)]
pub struct ChunkColumnChar {
    pub ch: char,
    pub is_match: bool,
}

pub enum ChunkDisplayLine {
    Label {
        prefix: usize,
        text: String,
    },
    Content {
        columns: Vec<ChunkColumnChar>,
        line_num: usize,
        text: String,
        is_match_line: bool,
        in_matched_chunk: bool,
        has_any_chunk: bool,
    },
    Message(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewMode {
    Snippet,
    Chunks,
}

#[derive(Debug, Clone)]
pub struct TuiConfig {
    pub max_results: usize,
    pub preview_lines: usize,
    pub enable_heatmap: bool,
    pub enable_syntax_highlighting: bool,
    pub chunk_preview_mode: PreviewMode,
    pub colors: ColorScheme,
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub query: Color,
    pub results: Color,
    pub preview: Color,
    pub status: Color,
    pub selected: Color,
    pub match_line: Color,
    pub chunk_boundary: Color,
    pub chunk_text: Color,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            max_results: 100,
            preview_lines: 10,
            enable_heatmap: true,
            enable_syntax_highlighting: true,
            chunk_preview_mode: PreviewMode::Chunks,
            colors: ColorScheme::default(),
        }
    }
}

impl TuiConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }

    pub fn with_preview_lines(mut self, preview_lines: usize) -> Self {
        self.preview_lines = preview_lines;
        self
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            query: Color::Yellow,
            results: Color::White,
            preview: Color::Cyan,
            status: Color::Green,
            selected: Color::Magenta,
            match_line: Color::Yellow,
            chunk_boundary: COLOR_CHUNK_BOUNDARY,
            chunk_text: COLOR_CHUNK_TEXT,
        }
    }
}

pub struct TuiState {
    pub query: String,
    pub mode: SearchMode,
    pub results: Vec<SearchResult>,
    pub selected_idx: usize,
    pub preview_content: String,
    pub preview_lines: Vec<Line<'static>>, // Colored preview
    pub preview_mode: PreviewMode,
    pub full_file_mode: bool, // false = snippet (±5 lines), true = full file
    pub scroll_offset: usize, // For scrolling in full file mode
    pub status_message: String,
    pub search_path: PathBuf,
    pub selected_files: HashSet<PathBuf>, // For multi-select
    pub search_history: Vec<String>,      // Search history
    pub history_index: usize,             // Current position in history
    pub command_mode: bool,               // true when query starts with /
    pub index_stats: Option<IndexStats>,
    pub last_index_stats_refresh: Option<Instant>,
    pub index_stats_error: Option<String>,
    pub preview_cache: Option<PreviewCache>,
    pub indexing_message: Option<String>,
    pub indexing_progress: Option<f32>,
    pub indexing_active: bool,
    pub indexing_started_at: Option<Instant>,
    pub last_indexing_update: Option<Instant>,
    pub search_in_progress: bool,
}

pub struct PreviewCache {
    pub file: PathBuf,
    pub lines: Vec<String>,
    pub is_pdf: bool,
    pub chunks: Vec<IndexedChunkMeta>,
}

impl TuiState {
    pub fn new(search_path: PathBuf) -> Self {
        Self {
            query: String::new(),
            mode: SearchMode::Semantic,
            results: Vec::new(),
            selected_idx: 0,
            preview_content: String::new(),
            preview_lines: Vec::new(),
            preview_mode: PreviewMode::Snippet,
            full_file_mode: false,
            scroll_offset: 0,
            status_message: "Type to search...".to_string(),
            search_path,
            selected_files: HashSet::new(),
            search_history: Vec::new(),
            history_index: 0,
            command_mode: false,
            index_stats: None,
            last_index_stats_refresh: None,
            index_stats_error: None,
            preview_cache: None,
            indexing_message: None,
            indexing_progress: None,
            indexing_active: false,
            indexing_started_at: None,
            last_indexing_update: None,
            search_in_progress: false,
        }
    }
}

/// Calculate the global depth for each chunk across the entire file
pub fn calculate_chunk_depths(
    all_chunks: &[IndexedChunkMeta],
) -> HashMap<usize, usize> {
    let mut depths = HashMap::new();
    
    for chunk in all_chunks {
        let span = &chunk.span;
        let depth = calculate_nesting_depth(chunk, all_chunks);
        
        // Store depth for each line in the chunk
        for line in span.line_start..=span.line_end {
            depths.insert(line, depth);
        }
    }
    
    depths
}

fn calculate_nesting_depth(
    target_chunk: &IndexedChunkMeta,
    all_chunks: &[IndexedChunkMeta],
) -> usize {
    let mut max_depth = 0;
    let target_span = &target_chunk.span;
    
    for chunk in all_chunks {
        let span = &chunk.span;
        
        // Check if this chunk contains our target chunk
        if span.line_start < target_span.line_start
            && span.line_end > target_span.line_end
        {
            let depth = calculate_nesting_depth(chunk, all_chunks) + 1;
            max_depth = max_depth.max(depth);
        }
    }
    
    max_depth
}

pub fn calculate_max_depth(all_chunks: &[IndexedChunkMeta]) -> usize {
    let mut max_depth = 0;
    
    for chunk in all_chunks {
        let depth = calculate_nesting_depth(chunk, all_chunks);
        max_depth = max_depth.max(depth);
    }
    
    max_depth
}
