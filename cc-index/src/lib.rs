use anyhow::Result;
use cc_core::{FileMetadata, Language, Span, compute_file_hash, get_sidecar_path};
use ignore::{WalkBuilder, overrides::OverrideBuilder};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use tempfile::NamedTempFile;
use walkdir::WalkDir;

pub type ProgressCallback = Box<dyn Fn(&str) + Send + Sync>;

/// Detailed progress information for embedding operations
#[derive(Debug, Clone)]
pub struct EmbeddingProgress {
    pub file_name: String,
    pub file_index: usize,
    pub total_files: usize,
    pub chunk_index: usize,
    pub total_chunks: usize,
    pub chunk_size: usize,
}

pub type DetailedProgressCallback = Box<dyn Fn(EmbeddingProgress) + Send + Sync>;

/// Enhanced progress information for granular indexing feedback
#[derive(Debug, Clone)]
pub enum IndexingProgress {
    /// Starting indexing process
    Starting { total_files: usize },
    /// Processing a specific file
    ProcessingFile {
        file: String,
        file_number: usize,
        total_files: usize,
        file_size: u64,
    },
    /// Chunking a file
    ChunkingFile { file: String, chunks_found: usize },
    /// Processing chunk for embedding
    ProcessingChunk {
        file: String,
        chunk_number: usize,
        total_chunks: usize,
        chunk_size: usize,
    },
    /// Finished processing a file
    FileComplete {
        file: String,
        chunks_processed: usize,
        file_number: usize,
        total_files: usize,
        elapsed_ms: u64,
    },
    /// Overall completion
    Complete {
        total_files: usize,
        total_chunks: usize,
        total_elapsed_ms: u64,
    },
}

pub type EnhancedProgressCallback = Box<dyn Fn(IndexingProgress) + Send + Sync>;

// Global interrupt flag
static INTERRUPTED: AtomicBool = AtomicBool::new(false);
static HANDLER_INIT: Once = Once::new();

pub const INDEX_INTERRUPTED_MSG: &str = "Indexing interrupted by user";

pub fn request_interrupt() {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// Build override patterns for excluding files during directory traversal
fn build_overrides(
    base_path: &Path,
    exclude_patterns: &[String],
) -> Result<ignore::overrides::Override> {
    let mut builder = OverrideBuilder::new(base_path);

    for pattern in exclude_patterns {
        if pattern.starts_with('!') {
            builder.add(pattern)?;
        } else {
            builder.add(&format!("!{}", pattern))?;
        }
    }

    Ok(builder.build()?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub metadata: FileMetadata,
    pub chunks: Vec<ChunkEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEntry {
    pub span: Span,
    pub embedding: Option<Vec<f32>>,
    pub chunk_type: Option<String>, // "function", "class", "method", or None for generic
    #[serde(default)]
    pub breadcrumb: Option<String>,
    #[serde(default)]
    pub ancestry: Option<Vec<String>>,
    #[serde(default)]
    pub byte_length: Option<usize>,
    #[serde(default)]
    pub estimated_tokens: Option<usize>,
    #[serde(default)]
    pub leading_trivia: Option<Vec<String>>,
    #[serde(default)]
    pub trailing_trivia: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexManifest {
    pub version: String,
    pub created: u64,
    pub updated: u64,
    pub files: HashMap<PathBuf, FileMetadata>,
    /// Embedding model used for this index (added in v0.4.2+)
    pub embedding_model: Option<String>,
    /// Embedding model dimensions (for validation)
    pub embedding_dimensions: Option<usize>,
}

impl Default for IndexManifest {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            version: "0.1.0".to_string(),
            created: now,
            updated: now,
            files: HashMap::new(),
            embedding_model: None, // Default to None for backward compatibility
            embedding_dimensions: None,
        }
    }
}

/// Common filtering logic for directory traversal entries
fn should_include_file(entry: &ignore::DirEntry, index_dir: &Path) -> bool {
    let path = entry.path();
    entry.file_type().is_some_and(|ft| ft.is_file())
        && is_text_file(path)
        && !path.starts_with(index_dir)
}

/// Apply common filtering to a WalkBuilder iterator
fn filter_and_collect_files(walker: ignore::Walk, index_dir: &Path) -> Vec<PathBuf> {
    walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| should_include_file(entry, index_dir))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

pub fn collect_files(
    path: &Path,
    respect_gitignore: bool,
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let index_dir = path.join(".cc");

    if respect_gitignore {
        let overrides = build_overrides(path, exclude_patterns)?;
        let walker = WalkBuilder::new(path)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .hidden(true)
            .overrides(overrides)
            .build();

        Ok(filter_and_collect_files(walker, &index_dir))
    } else {
        // Use WalkBuilder without gitignore support, but still apply overrides
        use cc_core::get_default_exclude_patterns;
        let default_patterns = get_default_exclude_patterns();

        // Combine default patterns with user exclude patterns
        let mut all_patterns = default_patterns;
        all_patterns.extend(exclude_patterns.iter().cloned());
        let combined_overrides = build_overrides(path, &all_patterns)?;

        let walker = WalkBuilder::new(path)
            .git_ignore(false)
            .hidden(true)
            .overrides(combined_overrides)
            .build();

        Ok(filter_and_collect_files(walker, &index_dir))
    }
}

fn collect_files_as_hashset(
    path: &Path,
    respect_gitignore: bool,
    exclude_patterns: &[String],
) -> Result<HashSet<PathBuf>> {
    Ok(collect_files(path, respect_gitignore, exclude_patterns)?
        .into_iter()
        .collect())
}

pub async fn index_directory(
    path: &Path,
    compute_embeddings: bool,
    respect_gitignore: bool,
    exclude_patterns: &[String],
    model: Option<&str>,
) -> Result<()> {
    tracing::info!(
        "index_directory called with compute_embeddings={}",
        compute_embeddings
    );
    let index_dir = path.join(".cc");
    fs::create_dir_all(&index_dir)?;

    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;
    normalize_manifest_paths(&mut manifest, path);

    // Handle model configuration for embeddings
    let resolved_model = if compute_embeddings {
        // Resolve the model name and get its dimensions
        let model_registry = cc_models::ModelRegistry::default();
        let selected_model = if let Some(model_name) = model {
            // User specified a model
            if let Some(model_config) = model_registry.get_model(model_name) {
                model_config.name.clone()
            } else {
                return Err(anyhow::anyhow!(
                    "Unknown model '{}'. Available models: bge-small, nomic-v1.5, jina-code",
                    model_name
                ));
            }
        } else {
            // Use default model
            let default_config = model_registry
                .get_default_model()
                .ok_or_else(|| anyhow::anyhow!("No default model available"))?;
            default_config.name.clone()
        };

        // Set the model info in the manifest for new indexes
        manifest.embedding_model = Some(selected_model.clone());
        if let Some(model_name) = model {
            if let Some(model_config) = model_registry.get_model(model_name) {
                manifest.embedding_dimensions = Some(model_config.dimensions);
            }
        } else if let Some(default_config) = model_registry.get_default_model() {
            manifest.embedding_dimensions = Some(default_config.dimensions);
        }

        Some(selected_model)
    } else {
        None
    };

    let files = collect_files(path, respect_gitignore, exclude_patterns)?;

    if compute_embeddings {
        // Sequential processing with small-batch embeddings for streaming performance
        tracing::info!("Creating embedder for {} files", files.len());
        let mut embedder = cc_embed::create_embedder(resolved_model.as_deref())?;

        for file_path in files.iter() {
            match index_single_file(file_path, path, Some(&mut embedder)) {
                Ok(entry) => {
                    // Write sidecar immediately
                    let sidecar_path = get_sidecar_path(path, file_path);
                    save_index_entry(&sidecar_path, &entry)?;

                    // Update and save manifest immediately
                    let manifest_key = entry.metadata.path.clone();
                    manifest.files.insert(manifest_key, entry.metadata);
                    manifest.updated = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    save_manifest(&manifest_path, &manifest)?;
                }
                Err(e) => {
                    // Suppress warnings for binary files and UTF-8 errors in .git directories
                    let error_msg = e.to_string();
                    let is_binary_skip = error_msg.contains("Binary file, skipping");
                    let is_utf8_error = error_msg.contains("stream did not contain valid UTF-8");
                    let is_git_file = file_path.components().any(|c| c.as_os_str() == ".git");

                    if !(is_binary_skip || is_utf8_error && is_git_file) {
                        tracing::warn!("Failed to index {:?}: {}", file_path, e);
                    }
                }
            }
        }
    } else {
        // Parallel processing with streaming using producer-consumer pattern
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let files_clone = files.clone();
        let path_clone = path.to_path_buf();

        // Spawn worker thread for parallel processing
        let worker_handle = thread::spawn(move || {
            files_clone.par_iter().for_each(|file_path| {
                match index_single_file(file_path, &path_clone, None) {
                    Ok(entry) => {
                        if tx.send((file_path.clone(), entry)).is_err() {
                            // Receiver dropped, stop processing
                        }
                    }
                    Err(e) => {
                        // Suppress warnings for binary files and UTF-8 errors in .git directories
                        let error_msg = e.to_string();
                        let is_binary_skip = error_msg.contains("Binary file, skipping");
                        let is_utf8_error =
                            error_msg.contains("stream did not contain valid UTF-8");
                        let is_git_file = file_path.components().any(|c| c.as_os_str() == ".git");

                        if !(is_binary_skip || is_utf8_error && is_git_file) {
                            tracing::warn!("Failed to index {:?}: {}", file_path, e);
                        }
                    }
                }
            });
        });

        // Main thread: stream results as they arrive
        while let Ok((file_path, entry)) = rx.recv() {
            // Write sidecar immediately
            let sidecar_path = get_sidecar_path(path, &file_path);
            save_index_entry(&sidecar_path, &entry)?;

            // Update and save manifest immediately
            let manifest_key = entry.metadata.path.clone();
            manifest.files.insert(manifest_key, entry.metadata);
            manifest.updated = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            save_manifest(&manifest_path, &manifest)?;
        }

        // Wait for worker to complete
        worker_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Worker thread panicked"))?;
    }

    // Manifest is already updated after each file in streaming mode
    // Only save manifest if using parallel processing (non-embedding case)
    if !compute_embeddings {
        manifest.updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        save_manifest(&manifest_path, &manifest)?;
    }

    Ok(())
}

pub async fn index_file(file_path: &Path, compute_embeddings: bool) -> Result<()> {
    let repo_root = find_repo_root(file_path)?;
    let index_dir = repo_root.join(".cc");
    fs::create_dir_all(&index_dir)?;

    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;

    let entry = if compute_embeddings {
        // Use the model from the existing index, or default if none specified
        let model_name = manifest.embedding_model.as_deref();
        let mut embedder = cc_embed::create_embedder(model_name)?;
        index_single_file(file_path, &repo_root, Some(&mut embedder))?
    } else {
        index_single_file(file_path, &repo_root, None)?
    };
    let sidecar_path = get_sidecar_path(&repo_root, file_path);

    save_index_entry(&sidecar_path, &entry)?;
    let manifest_key = entry.metadata.path.clone();
    manifest.files.insert(manifest_key, entry.metadata);
    manifest.updated = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    save_manifest(&manifest_path, &manifest)?;

    Ok(())
}

pub async fn update_index(
    path: &Path,
    compute_embeddings: bool,
    respect_gitignore: bool,
    exclude_patterns: &[String],
) -> Result<()> {
    let index_dir = path.join(".cc");
    if !index_dir.exists() {
        return index_directory(
            path,
            compute_embeddings,
            respect_gitignore,
            exclude_patterns,
            None, // model - use existing from manifest for update
        )
        .await;
    }

    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;

    let files = collect_files(path, respect_gitignore, exclude_patterns)?;

    let updates: Vec<(PathBuf, IndexEntry)> = if compute_embeddings {
        // Sequential processing when computing embeddings (for memory efficiency)
        let model_name = manifest.embedding_model.as_deref();
        let mut embedder = cc_embed::create_embedder(model_name)?;
        files
            .iter()
            .filter_map(|file_path| {
                let manifest_key =
                    path_utils::to_manifest_path(&path_utils::to_standard_path(file_path, path));

                let needs_update = match manifest.files.get(&manifest_key) {
                    Some(metadata) => match compute_file_hash(file_path) {
                        Ok(hash) => hash != metadata.hash,
                        Err(_) => false,
                    },
                    None => true,
                };
                if needs_update {
                    match index_single_file(file_path, path, Some(&mut embedder)) {
                        Ok(entry) => Some((file_path.clone(), entry)),
                        Err(e) => {
                            // Suppress warnings for binary files and UTF-8 errors in .git directories
                            let error_msg = e.to_string();
                            let is_binary_skip = error_msg.contains("Binary file, skipping");
                            let is_utf8_error =
                                error_msg.contains("stream did not contain valid UTF-8");
                            let is_git_file =
                                file_path.components().any(|c| c.as_os_str() == ".git");

                            if !(is_binary_skip || is_utf8_error && is_git_file) {
                                tracing::warn!("Failed to index {:?}: {}", file_path, e);
                            }
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect()
    } else {
        // Parallel processing when not computing embeddings
        files
            .par_iter()
            .filter_map(|file_path| {
                let manifest_key =
                    path_utils::to_manifest_path(&path_utils::to_standard_path(file_path, path));

                let needs_update = match manifest.files.get(&manifest_key) {
                    Some(metadata) => match compute_file_hash(file_path) {
                        Ok(hash) => hash != metadata.hash,
                        Err(_) => false,
                    },
                    None => true,
                };

                if needs_update {
                    match index_single_file(file_path, path, None) {
                        Ok(entry) => Some((file_path.clone(), entry)),
                        Err(e) => {
                            // Suppress warnings for binary files and UTF-8 errors in .git directories
                            let error_msg = e.to_string();
                            let is_binary_skip = error_msg.contains("Binary file, skipping");
                            let is_utf8_error =
                                error_msg.contains("stream did not contain valid UTF-8");
                            let is_git_file =
                                file_path.components().any(|c| c.as_os_str() == ".git");

                            if !(is_binary_skip || is_utf8_error && is_git_file) {
                                tracing::warn!("Failed to index {:?}: {}", file_path, e);
                            }
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    for (file_path, entry) in updates {
        let sidecar_path = get_sidecar_path(path, &file_path);
        save_index_entry(&sidecar_path, &entry)?;
        let manifest_key = entry.metadata.path.clone();
        manifest.files.insert(manifest_key, entry.metadata);
    }

    if !manifest.files.is_empty() {
        manifest.updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        save_manifest(&manifest_path, &manifest)?;
    }

    Ok(())
}

pub fn clean_index(path: &Path) -> Result<()> {
    let index_dir = path.join(".cc");
    if index_dir.exists() {
        fs::remove_dir_all(&index_dir)?;
    }
    Ok(())
}

pub fn cleanup_index(
    path: &Path,
    respect_gitignore: bool,
    exclude_patterns: &[String],
) -> Result<CleanupStats> {
    let index_dir = path.join(".cc");
    if !index_dir.exists() {
        return Ok(CleanupStats::default());
    }

    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;
    normalize_manifest_paths(&mut manifest, path);

    // Use the new unified cleanup validation
    let stats = cleanup_validation::validate_and_cleanup_index(
        path,
        &index_dir,
        &mut manifest,
        respect_gitignore,
        exclude_patterns,
    )?;

    // Content cache cleanup is now handled by the unified cleanup validation

    // Remove empty directories in .cc
    remove_empty_dirs(&index_dir)?;

    // Update manifest if changes were made
    if stats.orphaned_entries_removed > 0 {
        manifest.updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        save_manifest(&manifest_path, &manifest)?;
    }

    Ok(stats)
}

pub fn get_index_stats(path: &Path) -> Result<IndexStats> {
    let index_dir = path.join(".cc");
    if !index_dir.exists() {
        return Ok(IndexStats::default());
    }

    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;
    normalize_manifest_paths(&mut manifest, path);

    let mut stats = IndexStats {
        total_files: manifest.files.len(),
        index_created: manifest.created,
        index_updated: manifest.updated,
        ..Default::default()
    };

    // Calculate total chunks and size
    for file_path in manifest.files.keys() {
        let standard_path = path_utils::from_manifest_path(file_path);
        let sidecar_path =
            path_utils::get_sidecar_path_for_standard_path(&index_dir, &standard_path);
        if sidecar_path.exists()
            && let Ok(entry) = load_index_entry(&sidecar_path)
        {
            stats.total_chunks += entry.chunks.len();
            stats.total_size_bytes += entry.metadata.size;

            // Count embedded chunks
            let embedded = entry
                .chunks
                .iter()
                .filter(|c| c.embedding.is_some())
                .count();
            stats.embedded_chunks += embedded;
        }
    }

    // Calculate index size on disk
    if let Ok(entries) = WalkDir::new(&index_dir)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
    {
        for entry in entries {
            if entry.file_type().is_file()
                && let Ok(metadata) = entry.metadata()
            {
                stats.index_size_bytes += metadata.len();
            }
        }
    }

    Ok(stats)
}

pub async fn smart_update_index(
    path: &Path,
    compute_embeddings: bool,
    respect_gitignore: bool,
    exclude_patterns: &[String],
) -> Result<UpdateStats> {
    smart_update_index_with_progress(
        path,
        false,
        None,
        compute_embeddings,
        respect_gitignore,
        exclude_patterns,
        None, // model - use default for backward compatibility
    )
    .await
}

pub async fn smart_update_index_with_progress(
    path: &Path,
    force_rebuild: bool,
    progress_callback: Option<ProgressCallback>,
    compute_embeddings: bool,
    respect_gitignore: bool,
    exclude_patterns: &[String],
    model: Option<&str>,
) -> Result<UpdateStats> {
    smart_update_index_with_detailed_progress(
        path,
        force_rebuild,
        progress_callback,
        None, // No detailed progress callback for backward compatibility
        compute_embeddings,
        respect_gitignore,
        exclude_patterns,
        model,
    )
    .await
}

/// Enhanced indexing with detailed embedding progress
#[allow(clippy::too_many_arguments)]
pub async fn smart_update_index_with_detailed_progress(
    path: &Path,
    force_rebuild: bool,
    progress_callback: Option<ProgressCallback>,
    detailed_progress_callback: Option<DetailedProgressCallback>,
    compute_embeddings: bool,
    respect_gitignore: bool,
    exclude_patterns: &[String],
    model: Option<&str>,
) -> Result<UpdateStats> {
    let index_dir = path.join(".cc");
    let mut stats = UpdateStats::default();

    // Set up interrupt handler (only once per process)
    HANDLER_INIT.call_once(|| {
        let _ = ctrlc::set_handler(move || {
            INTERRUPTED.store(true, Ordering::SeqCst);
            eprintln!("\nIndexing interrupted by user. Cleaning up...");
        });
    });

    // Reset interrupt flag for this indexing operation
    INTERRUPTED.store(false, Ordering::SeqCst);

    if force_rebuild {
        clean_index(path)?;
        index_directory(
            path,
            compute_embeddings,
            respect_gitignore,
            exclude_patterns,
            model,
        )
        .await?;
        let index_stats = get_index_stats(path)?;
        stats.files_indexed = index_stats.total_files;
        return Ok(stats);
    }

    // Find repo root for path normalization
    let repo_root = find_repo_root(path)?;

    // Skip cleanup during incremental updates to avoid removing valid entries
    // that may be outside the current search scope or have path normalization issues
    // Cleanup should be done explicitly with --clean-orphans when needed

    // Then perform incremental update
    fs::create_dir_all(&index_dir)?;
    let manifest_path = index_dir.join("manifest.json");
    let mut manifest = load_or_create_manifest(&manifest_path)?;
    normalize_manifest_paths(&mut manifest, &repo_root);

    // Handle model configuration for embeddings
    let (resolved_model, _model_dimensions) = if compute_embeddings {
        // Resolve the model name and get its dimensions
        let model_registry = cc_models::ModelRegistry::default();
        let (selected_model, model_dims) = if let Some(model_name) = model {
            // User specified a model
            if let Some(model_config) = model_registry.get_model(model_name) {
                (model_config.name.clone(), model_config.dimensions)
            } else {
                return Err(anyhow::anyhow!(
                    "Unknown model '{}'. Available models: bge-small, nomic-v1.5, jina-code",
                    model_name
                ));
            }
        } else {
            // Use default model
            let default_config = model_registry
                .get_default_model()
                .ok_or_else(|| anyhow::anyhow!("No default model available"))?;
            (default_config.name.clone(), default_config.dimensions)
        };

        // Check for model compatibility with existing index
        let (final_model, final_dims) = if let Some(existing_model) = &manifest.embedding_model {
            // If we're updating an existing index and no model was specified,
            // use the existing model from the index
            if model.is_none() {
                // Use the existing model - this is an auto-update during search
                (
                    existing_model.clone(),
                    manifest.embedding_dimensions.unwrap_or(384),
                )
            } else if existing_model != &selected_model {
                // User explicitly specified a different model - that's an error
                return Err(anyhow::anyhow!(
                    "Model mismatch: Index was created with '{}', but you're trying to use '{}'. \
                    Please run 'cc --clean .' to remove the old index, then 'cc --index --model {}' to rebuild with the new model.",
                    existing_model,
                    selected_model,
                    model.unwrap_or("default")
                ));
            } else {
                // Model matches, proceed
                (selected_model, model_dims)
            }
        } else {
            // This is either a new index or an old index without model info
            // Set the model info in the manifest
            manifest.embedding_model = Some(selected_model.clone());
            manifest.embedding_dimensions = Some(model_dims);
            (selected_model, model_dims)
        };

        (Some(final_model), Some(final_dims))
    } else {
        (None, None)
    };

    // For incremental updates, only process files in the search scope
    // The cleanup phase already handled removing orphaned files from the entire repo
    let current_files = collect_files(path, respect_gitignore, exclude_patterns)?;

    // First pass: determine which files need updating and collect stats
    let mut files_to_update = Vec::new();
    let mut manifest_changed = false;

    for file_path in current_files {
        // Check for interrupt
        if INTERRUPTED.load(Ordering::SeqCst) {
            eprintln!("Indexing interrupted during file scanning.");
            return Ok(stats);
        }

        let manifest_key =
            path_utils::to_manifest_path(&path_utils::to_standard_path(&file_path, &repo_root));

        if let Some(metadata) = manifest.files.get(&manifest_key) {
            let fs_meta = match fs::metadata(&file_path) {
                Ok(m) => m,
                Err(_) => {
                    stats.files_errored += 1;
                    continue;
                }
            };

            let fs_last_modified = match fs_meta.modified().and_then(|m| {
                m.duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|_| std::io::Error::other("Time error"))
            }) {
                Ok(dur) => dur.as_secs(),
                Err(_) => {
                    stats.files_errored += 1;
                    continue;
                }
            };
            let fs_size = fs_meta.len();

            if fs_last_modified == metadata.last_modified && fs_size == metadata.size {
                stats.files_up_to_date += 1;
                continue;
            }

            let hash = match compute_file_hash(&file_path) {
                Ok(h) => h,
                Err(_) => {
                    stats.files_errored += 1;
                    continue;
                }
            };

            if hash != metadata.hash {
                stats.files_modified += 1;
                files_to_update.push(file_path);
            } else {
                stats.files_up_to_date += 1;
                // Convert to standardized path for manifest storage
                let standard_path = path_utils::to_standard_path(&file_path, &repo_root);
                let manifest_path = path_utils::to_manifest_path(&standard_path);
                let new_metadata = FileMetadata {
                    path: manifest_path.clone(),
                    hash,
                    last_modified: fs_last_modified,
                    size: fs_size,
                };
                manifest.files.insert(manifest_path, new_metadata);
                manifest_changed = true;
            }
        } else {
            stats.files_added += 1;
            files_to_update.push(file_path);
        }
    }

    // Second pass: index the files that need updating
    if compute_embeddings {
        // Sequential processing with streaming - write each file immediately
        let mut embedder = cc_embed::create_embedder(resolved_model.as_deref())?;
        let mut _processed_count = 0;

        for file_path in files_to_update.iter() {
            // Check for interrupt
            if INTERRUPTED.load(Ordering::SeqCst) {
                eprintln!(
                    "Indexing interrupted. {} files processed.",
                    _processed_count
                );
                break;
            }

            if let Some(ref callback) = progress_callback
                && let Some(file_name) = file_path.file_name()
            {
                callback(&file_name.to_string_lossy());
            }

            // Call detailed progress version if callback is provided, otherwise use regular version
            let result = if let Some(ref detailed_callback) = detailed_progress_callback {
                index_single_file_with_progress(
                    file_path,
                    path,
                    Some(&mut embedder),
                    Some(detailed_callback),
                    _processed_count,
                    files_to_update.len(),
                )
            } else {
                index_single_file(file_path, path, Some(&mut embedder))
            };

            match result {
                Ok(entry) => {
                    // Write sidecar immediately
                    let sidecar_path = get_sidecar_path(path, file_path);
                    save_index_entry(&sidecar_path, &entry)?;

                    // Update and save manifest immediately
                    let manifest_key = entry.metadata.path.clone();
                    manifest.files.insert(manifest_key, entry.metadata);
                    manifest.updated = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    save_manifest(&manifest_path, &manifest)?;
                    _processed_count += 1;
                }
                Err(e) => {
                    // Suppress warnings for binary files and UTF-8 errors in .git directories
                    let error_msg = e.to_string();
                    let is_binary_skip = error_msg.contains("Binary file, skipping");
                    let is_utf8_error = error_msg.contains("stream did not contain valid UTF-8");
                    let is_git_file = file_path.components().any(|c| c.as_os_str() == ".git");

                    if !(is_binary_skip || is_utf8_error && is_git_file) {
                        tracing::warn!("Failed to index {:?}: {}", file_path, e);
                    }
                    stats.files_errored += 1;
                }
            }
        }

        stats.files_indexed = _processed_count;
    } else {
        // Parallel processing with streaming using producer-consumer pattern
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let files_clone = files_to_update.clone();
        let path_clone = path.to_path_buf();

        // Spawn worker thread for parallel processing
        let worker_handle = thread::spawn(move || {
            use rayon::prelude::*;

            // Use par_iter with try_for_each to allow early exit on interrupt
            let result = files_clone.par_iter().try_for_each(|file_path| {
                // Check for interrupt
                if INTERRUPTED.load(Ordering::SeqCst) {
                    return Err("interrupted");
                }

                match index_single_file(file_path, &path_clone, None) {
                    Ok(entry) => {
                        if tx.send((file_path.clone(), entry)).is_err() {
                            // Receiver dropped, stop processing
                            return Err("receiver_dropped");
                        }
                    }
                    Err(e) => {
                        // Suppress warnings for binary files and UTF-8 errors in .git directories
                        let error_msg = e.to_string();
                        let is_binary_skip = error_msg.contains("Binary file, skipping");
                        let is_utf8_error =
                            error_msg.contains("stream did not contain valid UTF-8");
                        let is_git_file = file_path.components().any(|c| c.as_os_str() == ".git");

                        if !(is_binary_skip || is_utf8_error && is_git_file) {
                            tracing::warn!("Failed to index {:?}: {}", file_path, e);
                        }
                    }
                }
                Ok(())
            });

            // Log the result for debugging
            if let Err(reason) = result {
                tracing::debug!("Worker thread stopped due to: {}", reason);
            }
        });

        // Main thread: stream results as they arrive
        let mut _processed_count = 0;
        while let Ok((file_path, entry)) = rx.recv() {
            // Check for interrupt
            if INTERRUPTED.load(Ordering::SeqCst) {
                eprintln!(
                    "Indexing interrupted. {} files processed.",
                    _processed_count
                );
                drop(rx); // Drop receiver to signal worker to stop
                break;
            }

            if let Some(ref callback) = progress_callback
                && let Some(file_name) = file_path.file_name()
            {
                callback(&file_name.to_string_lossy());
            }

            // Write sidecar immediately
            let sidecar_path = get_sidecar_path(path, &file_path);
            save_index_entry(&sidecar_path, &entry)?;

            // Update and save manifest immediately
            let manifest_key = entry.metadata.path.clone();
            manifest.files.insert(manifest_key, entry.metadata);
            manifest.updated = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            save_manifest(&manifest_path, &manifest)?;
            _processed_count += 1;
        }

        stats.files_indexed = _processed_count;

        // Wait for worker to complete
        worker_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Worker thread panicked"))?;
    }

    // For sequential processing (embeddings), manifest is already saved after each file
    // Only save manifest for parallel processing or if there were metadata-only changes
    if !compute_embeddings
        && (stats.files_indexed > 0 || stats.orphaned_files_removed > 0 || manifest_changed)
    {
        manifest.updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        save_manifest(&manifest_path, &manifest)?;
    }

    Ok(stats)
}

fn index_single_file(
    file_path: &Path,
    repo_root: &Path,
    embedder: Option<&mut Box<dyn cc_embed::Embedder>>,
) -> Result<IndexEntry> {
    index_single_file_with_progress(file_path, repo_root, embedder, None, 0, 1)
}

fn index_single_file_with_progress(
    file_path: &Path,
    repo_root: &Path,
    embedder: Option<&mut Box<dyn cc_embed::Embedder>>,
    detailed_progress: Option<&DetailedProgressCallback>,
    file_index: usize,
    total_files: usize,
) -> Result<IndexEntry> {
    // Skip binary files to avoid UTF-8 warnings
    if !is_text_file(file_path) {
        return Err(anyhow::anyhow!("Binary file, skipping"));
    }

    // Preprocess file (extracts PDFs to cache, returns path to readable content)
    let content_path = preprocess_file(file_path, repo_root)?;
    let content = fs::read_to_string(&content_path)?;

    // Always use the ORIGINAL file for hash and metadata
    let hash = compute_file_hash(file_path)?;
    let metadata = fs::metadata(file_path)?;

    let standard_path = path_utils::to_standard_path(file_path, repo_root);
    let manifest_path = path_utils::to_manifest_path(&standard_path);

    let file_metadata = FileMetadata {
        path: manifest_path,
        hash,
        last_modified: metadata
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
        size: metadata.len(),
    };

    // Detect language for tree-sitter parsing
    let lang = if cc_core::pdf::is_pdf_file(file_path) {
        Some(Language::Pdf)
    } else {
        cc_core::Language::from_path(file_path)
    };

    let model_name = embedder.as_ref().map(|e| e.model_name());
    let chunks = cc_chunk::chunk_text_with_model(&content, lang, model_name)?;

    let chunk_entries: Vec<ChunkEntry> = if let Some(embedder) = embedder {
        let total_chunks = chunks.len();
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Process chunks with progress reporting
        if let Some(ref callback) = detailed_progress {
            tracing::info!(
                "Computing embeddings for {} chunks in {:?}",
                total_chunks,
                file_path
            );

            let mut chunk_entries = Vec::new();
            for (chunk_index, chunk) in chunks.into_iter().enumerate() {
                if INTERRUPTED.load(Ordering::SeqCst) {
                    return Err(anyhow::anyhow!(INDEX_INTERRUPTED_MSG));
                }
                // Report progress before processing chunk
                callback(EmbeddingProgress {
                    file_name: file_name.clone(),
                    file_index,
                    total_files,
                    chunk_index,
                    total_chunks,
                    chunk_size: chunk.text.len(),
                });

                // Embed single chunk
                let embeddings = embedder.embed(std::slice::from_ref(&chunk.text))?;
                let embedding = embeddings.into_iter().next().ok_or_else(|| {
                    anyhow::anyhow!(
                        "Embedder returned empty results for chunk {} in file {:?}. This may indicate an issue with the embedding model or chunk content.",
                        chunk_index,
                        file_path
                    )
                })?;

                let chunk_type_str = match chunk.chunk_type {
                    cc_chunk::ChunkType::Function => Some("function".to_string()),
                    cc_chunk::ChunkType::Class => Some("class".to_string()),
                    cc_chunk::ChunkType::Method => Some("method".to_string()),
                    cc_chunk::ChunkType::Module => Some("module".to_string()),
                    cc_chunk::ChunkType::Text => None,
                };

                let breadcrumb = chunk.metadata.breadcrumb.clone();
                let ancestry = if chunk.metadata.ancestry.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.ancestry.clone())
                };
                let leading_trivia = if chunk.metadata.leading_trivia.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.leading_trivia.clone())
                };
                let trailing_trivia = if chunk.metadata.trailing_trivia.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.trailing_trivia.clone())
                };

                chunk_entries.push(ChunkEntry {
                    span: chunk.span,
                    embedding: Some(embedding),
                    chunk_type: chunk_type_str,
                    breadcrumb,
                    ancestry,
                    byte_length: Some(chunk.metadata.byte_length),
                    estimated_tokens: Some(chunk.metadata.estimated_tokens),
                    leading_trivia,
                    trailing_trivia,
                });
            }
            chunk_entries
        } else {
            // Fallback to batch processing for backward compatibility
            let chunk_texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
            tracing::info!(
                "Computing embeddings for {} chunks in {:?}",
                chunk_texts.len(),
                file_path
            );
            let embeddings = embedder.embed(&chunk_texts)?;

            // Validate that embedder returned the expected number of embeddings
            if embeddings.len() != chunks.len() {
                return Err(anyhow::anyhow!(
                    "Embedder returned {} embeddings for {} chunks in file {:?}. Expected equal counts.",
                    embeddings.len(),
                    chunks.len(),
                    file_path
                ));
            }

            chunks
                .into_iter()
                .zip(embeddings)
                .map(|(chunk, embedding)| {
                    let chunk_type_str = match chunk.chunk_type {
                        cc_chunk::ChunkType::Function => Some("function".to_string()),
                        cc_chunk::ChunkType::Class => Some("class".to_string()),
                        cc_chunk::ChunkType::Method => Some("method".to_string()),
                        cc_chunk::ChunkType::Module => Some("module".to_string()),
                        cc_chunk::ChunkType::Text => None,
                    };
                    let breadcrumb = chunk.metadata.breadcrumb.clone();
                    let ancestry = if chunk.metadata.ancestry.is_empty() {
                        None
                    } else {
                        Some(chunk.metadata.ancestry.clone())
                    };
                    let leading_trivia = if chunk.metadata.leading_trivia.is_empty() {
                        None
                    } else {
                        Some(chunk.metadata.leading_trivia.clone())
                    };
                    let trailing_trivia = if chunk.metadata.trailing_trivia.is_empty() {
                        None
                    } else {
                        Some(chunk.metadata.trailing_trivia.clone())
                    };
                    ChunkEntry {
                        span: chunk.span,
                        embedding: Some(embedding),
                        chunk_type: chunk_type_str,
                        breadcrumb,
                        ancestry,
                        byte_length: Some(chunk.metadata.byte_length),
                        estimated_tokens: Some(chunk.metadata.estimated_tokens),
                        leading_trivia,
                        trailing_trivia,
                    }
                })
                .collect()
        }
    } else {
        // No embedder, just store spans without embeddings
        chunks
            .into_iter()
            .map(|chunk| {
                let chunk_type_str = match chunk.chunk_type {
                    cc_chunk::ChunkType::Function => Some("function".to_string()),
                    cc_chunk::ChunkType::Class => Some("class".to_string()),
                    cc_chunk::ChunkType::Method => Some("method".to_string()),
                    cc_chunk::ChunkType::Module => Some("module".to_string()),
                    cc_chunk::ChunkType::Text => None,
                };
                let breadcrumb = chunk.metadata.breadcrumb.clone();
                let ancestry = if chunk.metadata.ancestry.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.ancestry.clone())
                };
                let leading_trivia = if chunk.metadata.leading_trivia.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.leading_trivia.clone())
                };
                let trailing_trivia = if chunk.metadata.trailing_trivia.is_empty() {
                    None
                } else {
                    Some(chunk.metadata.trailing_trivia.clone())
                };
                ChunkEntry {
                    span: chunk.span,
                    embedding: None,
                    chunk_type: chunk_type_str,
                    breadcrumb,
                    ancestry,
                    byte_length: Some(chunk.metadata.byte_length),
                    estimated_tokens: Some(chunk.metadata.estimated_tokens),
                    leading_trivia,
                    trailing_trivia,
                }
            })
            .collect()
    };

    Ok(IndexEntry {
        metadata: file_metadata,
        chunks: chunk_entries,
    })
}

fn load_or_create_manifest(path: &Path) -> Result<IndexManifest> {
    if path.exists() {
        let data = fs::read(path)?;
        Ok(serde_json::from_slice(&data)?)
    } else {
        Ok(IndexManifest::default())
    }
}

fn normalize_manifest_paths(manifest: &mut IndexManifest, repo_root: &Path) {
    let original_entries = std::mem::take(&mut manifest.files);
    let mut normalized = HashMap::with_capacity(original_entries.len());

    for (key, mut metadata) in original_entries {
        let standard_key = if key.is_absolute() {
            path_utils::to_standard_path(&key, repo_root)
        } else {
            path_utils::from_manifest_path(&key)
        };
        let manifest_key = path_utils::to_manifest_path(&standard_key);

        let metadata_standard = if metadata.path.is_absolute() {
            path_utils::to_standard_path(&metadata.path, repo_root)
        } else {
            path_utils::from_manifest_path(&metadata.path)
        };
        metadata.path = path_utils::to_manifest_path(&metadata_standard);

        normalized.insert(manifest_key, metadata);
    }

    manifest.files = normalized;
}

fn save_manifest(path: &Path, manifest: &IndexManifest) -> Result<()> {
    let data = serde_json::to_vec_pretty(manifest)?;
    atomic_write(path, &data)
}

fn save_index_entry(path: &Path, entry: &IndexEntry) -> Result<()> {
    let data = bincode::serialize(entry)?;
    atomic_write(path, &data)
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    let mut tmp = NamedTempFile::new_in(parent)?;
    tmp.write_all(data)?;
    tmp.as_file().sync_all()?;

    if path.exists() {
        fs::remove_file(path)?;
    }

    tmp.persist(path)?;
    Ok(())
}

pub fn load_index_entry(path: &Path) -> Result<IndexEntry> {
    let data = fs::read(path)?;
    Ok(bincode::deserialize(&data)?)
}

fn find_repo_root(path: &Path) -> Result<PathBuf> {
    let mut current = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };

    loop {
        if current.join(".cc").exists() || current.join(".git").exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return Ok(path.to_path_buf()),
        }
    }
}

/// Check if content needs re-extraction
fn should_reextract(source_path: &Path, cache_path: &Path) -> Result<bool> {
    if !cache_path.exists() {
        return Ok(true);
    }

    let source_modified = fs::metadata(source_path)?.modified()?;
    let cache_modified = fs::metadata(cache_path)?.modified()?;

    Ok(source_modified > cache_modified)
}

/// Extract text content from a PDF file
fn extract_pdf_text(path: &Path) -> Result<String> {
    pdf_extract::extract_text(path)
        .map_err(|e| anyhow::anyhow!("Failed to extract text from PDF {}: {}", path.display(), e))
}

/// Preprocess a file if needed, returning path to readable content
/// For regular files: returns the original path (no preprocessing)
/// For PDFs: extracts text to cache, returns cache path
fn preprocess_file(file_path: &Path, repo_root: &Path) -> Result<PathBuf> {
    if cc_core::pdf::is_pdf_file(file_path) {
        let cache_path = cc_core::pdf::get_content_cache_path(repo_root, file_path);

        // Check if re-extraction needed
        if should_reextract(file_path, &cache_path)? {
            tracing::debug!(
                "Extracting PDF content from {:?} to {:?}",
                file_path,
                cache_path
            );
            let extracted_text = extract_pdf_text(file_path)?;

            // Ensure cache directory exists
            if let Some(parent) = cache_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write extracted text
            fs::write(&cache_path, extracted_text)?;
        }

        Ok(cache_path) // Return path to extracted text
    } else {
        Ok(file_path.to_path_buf()) // Return original path for regular files
    }
}

fn is_text_file(path: &Path) -> bool {
    // PDFs are considered indexable even though they're binary
    if cc_core::pdf::is_pdf_file(path) {
        return true;
    }

    // Use NUL byte heuristic like ripgrep - read first 8KB and check for NUL bytes
    const BUFFER_SIZE: usize = 8192;

    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buffer = vec![0; BUFFER_SIZE];
            match file.read(&mut buffer) {
                Ok(bytes_read) => {
                    // If file is empty, consider it text
                    if bytes_read == 0 {
                        return true;
                    }

                    // Check for NUL bytes in the read portion
                    !buffer[..bytes_read].contains(&0)
                }
                Err(_) => false, // If we can't read, assume binary
            }
        }
        Err(_) => false, // If we can't open, assume binary
    }
}

#[cfg(test)]
fn sidecar_to_original_path(
    sidecar_path: &Path,
    index_dir: &Path,
    _repo_root: &Path,
) -> Option<PathBuf> {
    let relative_path = sidecar_path.strip_prefix(index_dir).ok()?;
    let original_path = relative_path.with_extension("");

    // Handle the .cc extension removal
    if let Some(name) = original_path.file_name() {
        let name_str = name.to_string_lossy();
        if let Some(original_name) = name_str.strip_suffix(".cc") {
            let mut result = original_path.clone();
            result.set_file_name(original_name);
            return Some(result);
        }
    }

    Some(original_path)
}

fn remove_empty_dirs(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_empty_dirs(&path)?;
            // Try to remove if now empty
            if fs::read_dir(&path)?.next().is_none() {
                let _ = fs::remove_dir(&path);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanupStats {
    pub orphaned_entries_removed: usize,
    pub orphaned_sidecars_removed: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_chunks: usize,
    pub embedded_chunks: usize,
    pub total_size_bytes: u64,
    pub index_size_bytes: u64,
    pub index_created: u64,
    pub index_updated: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStats {
    pub files_indexed: usize,
    pub files_added: usize,
    pub files_modified: usize,
    pub files_up_to_date: usize,
    pub files_errored: usize,
    pub orphaned_files_removed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Test embedder that can return empty results to test error handling
    struct EmptyResultsEmbedder;

    impl cc_embed::Embedder for EmptyResultsEmbedder {
        fn id(&self) -> &'static str {
            "empty-results-test"
        }

        fn dim(&self) -> usize {
            384
        }

        fn model_name(&self) -> &str {
            "test-empty-results"
        }

        fn embed(&mut self, _texts: &[String]) -> Result<Vec<Vec<f32>>> {
            // Always return empty vector to trigger the panic scenario
            Ok(Vec::new())
        }
    }

    /// Test embedder that returns mismatched count of embeddings
    struct MismatchedCountEmbedder;

    impl cc_embed::Embedder for MismatchedCountEmbedder {
        fn id(&self) -> &'static str {
            "mismatched-count-test"
        }

        fn dim(&self) -> usize {
            384
        }

        fn model_name(&self) -> &str {
            "test-mismatched-count"
        }

        fn embed(&mut self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
            // Always return one less embedding than requested
            if texts.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(vec![vec![0.0; self.dim()]; texts.len() - 1])
            }
        }
    }

    #[test]
    fn test_index_single_file_handles_empty_embedding_results() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create a simple test file
        let test_file = test_path.join("test.txt");
        fs::write(&test_file, "hello world").unwrap();

        // Create an embedder that returns empty results
        let mut empty_embedder: Box<dyn cc_embed::Embedder> = Box::new(EmptyResultsEmbedder);

        // This should return an error, not panic
        let result = index_single_file(&test_file, test_path, Some(&mut empty_embedder));

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // The empty embedder triggers the count mismatch error (0 embeddings for 1 chunk)
        assert!(error_msg.contains("Embedder returned 0 embeddings for 1 chunks"));
        assert!(error_msg.contains("Expected equal counts"));
        assert!(error_msg.contains("test.txt"));
    }

    #[test]
    fn test_index_single_file_with_progress_handles_empty_embedding_results() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create a simple test file
        let test_file = test_path.join("test.txt");
        fs::write(&test_file, "hello world").unwrap();

        // Create an embedder that returns empty results
        let mut empty_embedder: Box<dyn cc_embed::Embedder> = Box::new(EmptyResultsEmbedder);

        // Use the detailed progress callback to trigger the single-chunk processing path
        let dummy_callback: DetailedProgressCallback = Box::new(|_progress: EmbeddingProgress| {});
        let result = index_single_file_with_progress(
            &test_file,
            test_path,
            Some(&mut empty_embedder),
            Some(&dummy_callback),
            0,
            1,
        );

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // This should hit the single-chunk path and get the specific error
        assert!(error_msg.contains("Embedder returned empty results"));
        assert!(error_msg.contains("chunk 0"));
        assert!(error_msg.contains("test.txt"));
    }

    #[test]
    fn test_index_single_file_handles_mismatched_embedding_count() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create a test file with multiple chunks (use some code content)
        let test_file = test_path.join("test.rs");
        fs::write(
            &test_file,
            "fn main() {\n    println!(\"hello\");\n}\n\nfn other() {\n    println!(\"world\");\n}",
        )
        .unwrap();

        // Create an embedder that returns mismatched count
        let mut mismatched_embedder: Box<dyn cc_embed::Embedder> =
            Box::new(MismatchedCountEmbedder);

        // This should return an error, not silently mismatch
        let result = index_single_file(&test_file, test_path, Some(&mut mismatched_embedder));

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Embedder returned"));
        assert!(error_msg.contains("embeddings for"));
        assert!(error_msg.contains("chunks"));
        assert!(error_msg.contains("Expected equal counts"));
    }

    #[test]
    fn test_index_single_file_with_valid_embedder_still_works() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create a simple test file
        let test_file = test_path.join("test.txt");
        fs::write(&test_file, "hello world").unwrap();

        // Create a dummy embedder that returns proper results
        let dummy_embedder = cc_embed::DummyEmbedder::new();
        let mut boxed_embedder: Box<dyn cc_embed::Embedder> = Box::new(dummy_embedder);

        // This should work fine
        let result = index_single_file(&test_file, test_path, Some(&mut boxed_embedder));

        assert!(result.is_ok());
        let entry = result.unwrap();
        assert!(!entry.chunks.is_empty());
        // Verify that embeddings are present
        for chunk in &entry.chunks {
            assert!(chunk.embedding.is_some());
            assert_eq!(chunk.embedding.as_ref().unwrap().len(), 384); // DummyEmbedder dimension
        }
    }

    #[tokio::test]
    async fn test_smart_update_index() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create initial file
        fs::write(test_path.join("file1.txt"), "initial content").unwrap();

        // First index
        let stats1 = smart_update_index(test_path, false, true, &[])
            .await
            .unwrap();
        assert_eq!(stats1.files_added, 1);
        assert_eq!(stats1.files_indexed, 1);

        // No changes, should be up to date
        let stats2 = smart_update_index(test_path, false, true, &[])
            .await
            .unwrap();
        assert_eq!(stats2.files_up_to_date, 1);
        assert_eq!(stats2.files_indexed, 0);

        // Modify file
        fs::write(test_path.join("file1.txt"), "modified content").unwrap();
        let stats3 = smart_update_index(test_path, false, true, &[])
            .await
            .unwrap();
        assert_eq!(stats3.files_modified, 1);
        assert_eq!(stats3.files_indexed, 1);

        // Add new file
        fs::write(test_path.join("file2.txt"), "new file content").unwrap();
        let stats4 = smart_update_index(test_path, false, true, &[])
            .await
            .unwrap();
        assert_eq!(stats4.files_added, 1);
        assert_eq!(stats4.files_up_to_date, 1);
        assert_eq!(stats4.files_indexed, 1);
    }

    #[test]
    fn test_cleanup_index() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create index directory and manifest
        let index_dir = test_path.join(".cc");
        fs::create_dir_all(&index_dir).unwrap();

        let mut manifest = IndexManifest::default();
        manifest.files.insert(
            test_path.join("deleted_file.txt"),
            FileMetadata {
                path: test_path.join("deleted_file.txt"),
                hash: "fake_hash".to_string(),
                last_modified: 0,
                size: 0,
            },
        );

        let manifest_path = index_dir.join("manifest.json");
        save_manifest(&manifest_path, &manifest).unwrap();

        // Cleanup should remove orphaned entry
        let stats = cleanup_index(test_path, true, &[]).unwrap();
        assert_eq!(stats.orphaned_entries_removed, 1);

        // Check that manifest was updated
        let updated_manifest = load_or_create_manifest(&manifest_path).unwrap();
        assert_eq!(updated_manifest.files.len(), 0);
    }

    #[test]
    fn test_get_index_stats() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // No index exists
        let stats = get_index_stats(test_path).unwrap();
        assert_eq!(stats.total_files, 0);

        // Create index
        let index_dir = test_path.join(".cc");
        fs::create_dir_all(&index_dir).unwrap();

        let mut manifest = IndexManifest::default();
        manifest.files.insert(
            test_path.join("test.txt"),
            FileMetadata {
                path: test_path.join("test.txt"),
                hash: "test_hash".to_string(),
                last_modified: 1234567890,
                size: 100,
            },
        );

        let manifest_path = index_dir.join("manifest.json");
        save_manifest(&manifest_path, &manifest).unwrap();

        let stats = get_index_stats(test_path).unwrap();
        assert_eq!(stats.total_files, 1);
    }

    #[test]
    fn test_sidecar_to_original_path() {
        let temp_dir = TempDir::new().unwrap();
        let index_dir = temp_dir.path().join(".cc");

        // Test normal file
        let sidecar = index_dir.join("test.txt.cc");
        let original = sidecar_to_original_path(&sidecar, &index_dir, temp_dir.path());
        assert_eq!(original, Some(PathBuf::from("test.txt")));

        // Test nested file
        let nested_sidecar = index_dir.join("src").join("main.rs.cc");
        let nested_original =
            sidecar_to_original_path(&nested_sidecar, &index_dir, temp_dir.path());
        assert_eq!(nested_original, Some(PathBuf::from("src/main.rs")));
    }

    #[test]
    fn test_is_text_file() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a text file (no NUL bytes)
        let text_file = temp_path.join("test.txt");
        let mut file = File::create(&text_file).unwrap();
        file.write_all(b"Hello world\nThis is text content")
            .unwrap();
        assert!(is_text_file(&text_file));

        // Create a text file with unusual extension
        let log_file = temp_path.join("app.log");
        let mut file = File::create(&log_file).unwrap();
        file.write_all(b"2024-01-15 ERROR: Failed to connect")
            .unwrap();
        assert!(is_text_file(&log_file));

        // Create a file without extension but with text content
        let no_ext_file = temp_path.join("README");
        let mut file = File::create(&no_ext_file).unwrap();
        file.write_all(b"This is a README file").unwrap();
        assert!(is_text_file(&no_ext_file));

        // Create a binary file with NUL bytes
        let binary_file = temp_path.join("test.bin");
        let mut file = File::create(&binary_file).unwrap();
        file.write_all(&[
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x57, 0x6F, 0x72, 0x6C, 0x64,
        ])
        .unwrap(); // "Hello\0World"
        assert!(!is_text_file(&binary_file));

        // Create an empty file (should be considered text)
        let empty_file = temp_path.join("empty.txt");
        File::create(&empty_file).unwrap();
        assert!(is_text_file(&empty_file));

        // Test non-existent file (should return false)
        let nonexistent = temp_path.join("nonexistent.txt");
        assert!(!is_text_file(&nonexistent));
    }

    #[test]
    fn test_remove_empty_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // Create nested empty directories
        let nested_dir = test_path.join("level1").join("level2").join("level3");
        fs::create_dir_all(&nested_dir).unwrap();

        // Remove empty dirs
        remove_empty_dirs(test_path).unwrap();

        // Check that empty dirs were removed
        assert!(!nested_dir.exists());
        assert!(!test_path.join("level1").join("level2").exists());
        assert!(!test_path.join("level1").exists());
    }
}

// ============================================================================
// Cleanup Validation Module
// ============================================================================

/// Comprehensive cleanup and validation for the index
mod cleanup_validation {
    use super::*;
    // IndexManifest is defined in this module

    /// Validates and cleans up the index to ensure consistency
    pub fn validate_and_cleanup_index(
        repo_root: &Path,
        index_dir: &Path,
        manifest: &mut IndexManifest,
        respect_gitignore: bool,
        exclude_patterns: &[String],
    ) -> Result<CleanupStats> {
        let mut stats = CleanupStats::default();

        // Step 1: Get all files that actually exist in the repository
        let existing_files =
            collect_files_as_hashset(repo_root, respect_gitignore, exclude_patterns)?;
        let standard_existing_files: HashSet<PathBuf> = existing_files
            .into_iter()
            .map(|path| path_utils::to_standard_path(&path, repo_root))
            .collect();

        // Step 2: Validate manifest entries
        let manifest_entries: Vec<PathBuf> =
            manifest.files.keys().map(|k| k.to_path_buf()).collect();
        for manifest_path in manifest_entries {
            let standard_path = path_utils::from_manifest_path(&manifest_path);

            // Check if file exists in reality
            if !standard_existing_files.contains(&standard_path) {
                remove_manifest_entry(manifest, &manifest_path, repo_root, index_dir, &mut stats)?;
                continue;
            }

            // Check if sidecar file exists
            let sidecar_path =
                path_utils::get_sidecar_path_for_standard_path(index_dir, &standard_path);
            if !sidecar_path.exists() {
                remove_manifest_entry(manifest, &manifest_path, repo_root, index_dir, &mut stats)?;
                continue;
            }
        }

        // Step 3: Clean up orphaned sidecar files
        cleanup_orphaned_sidecars(index_dir, &standard_existing_files, manifest, &mut stats)?;

        Ok(stats)
    }

    /// Remove a manifest entry and its associated files
    fn remove_manifest_entry(
        manifest: &mut IndexManifest,
        manifest_path: &Path,
        repo_root: &Path,
        index_dir: &Path,
        stats: &mut CleanupStats,
    ) -> Result<()> {
        manifest.files.remove(manifest_path);

        // Remove sidecar file
        let standard_path = path_utils::from_manifest_path(manifest_path);
        let sidecar_path =
            path_utils::get_sidecar_path_for_standard_path(index_dir, &standard_path);
        if sidecar_path.exists() {
            fs::remove_file(&sidecar_path)?;
            stats.orphaned_sidecars_removed += 1;
        }

        // Remove content cache for PDFs
        if cc_core::pdf::is_pdf_file(&standard_path) {
            let absolute_path = repo_root.join(&standard_path);
            let cache_path = cc_core::pdf::get_content_cache_path(repo_root, &absolute_path);
            if cache_path.exists() {
                fs::remove_file(&cache_path)?;
                tracing::debug!("Removed orphaned content cache: {:?}", cache_path);
            }
        }

        stats.orphaned_entries_removed += 1;
        tracing::warn!("Removed manifest entry: {:?}", manifest_path);
        Ok(())
    }

    /// Clean up sidecar files that don't have corresponding manifest entries
    fn cleanup_orphaned_sidecars(
        index_dir: &Path,
        standard_existing_files: &HashSet<PathBuf>,
        manifest: &IndexManifest,
        stats: &mut CleanupStats,
    ) -> Result<()> {
        if !index_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(index_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let sidecar_path = entry.path();
                if sidecar_path.extension().and_then(|s| s.to_str()) == Some("cc")
                    && let Some(standard_path) =
                        path_utils::sidecar_to_standard_path(sidecar_path, index_dir)
                {
                    let manifest_path = path_utils::to_manifest_path(&standard_path);

                    // Remove if file doesn't exist in reality or isn't in manifest
                    if !standard_existing_files.contains(&standard_path)
                        || !manifest.files.contains_key(&manifest_path)
                    {
                        fs::remove_file(sidecar_path)?;
                        stats.orphaned_sidecars_removed += 1;
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Path Utilities Module
// ============================================================================

/// Standardized path format for the indexing system.
/// All paths are stored as relative paths from the repository root without "./" prefix.
/// Example: "examples/code/api_client.js" instead of "./examples/code/api_client.js"
mod path_utils {
    use super::*;

    /// Convert an absolute path to a standardized relative path from repo root
    pub fn to_standard_path(absolute_path: &Path, repo_root: &Path) -> PathBuf {
        if let Ok(relative) = absolute_path.strip_prefix(repo_root) {
            relative.to_path_buf()
        } else {
            absolute_path.to_path_buf()
        }
    }

    /// Convert a standardized path to a manifest path (with "./" prefix for compatibility)
    pub fn to_manifest_path(standard_path: &Path) -> PathBuf {
        PathBuf::from(".").join(standard_path)
    }

    /// Convert a manifest path (with "./" prefix) to a standardized path
    pub fn from_manifest_path(manifest_path: &Path) -> PathBuf {
        if let Ok(relative) = manifest_path.strip_prefix(".") {
            relative.to_path_buf()
        } else {
            manifest_path.to_path_buf()
        }
    }

    /// Get the sidecar path for a standardized file path
    pub fn get_sidecar_path_for_standard_path(index_dir: &Path, standard_path: &Path) -> PathBuf {
        let sidecar_name = format!("{}.cc", standard_path.display());
        index_dir.join(sidecar_name)
    }

    /// Convert a sidecar path back to a standardized original path
    pub fn sidecar_to_standard_path(sidecar_path: &Path, index_dir: &Path) -> Option<PathBuf> {
        let relative_path = sidecar_path.strip_prefix(index_dir).ok()?;
        let original_path = relative_path.with_extension("");

        // Handle the .cc extension removal
        if let Some(name) = original_path.file_name() {
            let name_str = name.to_string_lossy();
            if let Some(original_name) = name_str.strip_suffix(".cc") {
                let mut result = original_path.clone();
                result.set_file_name(original_name);
                return Some(result);
            }
        }

        Some(original_path)
    }
}
