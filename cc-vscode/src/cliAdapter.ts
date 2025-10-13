/**
 * CLI adapter for spawning cc binary and parsing results
 */

import { spawn } from 'child_process';
import * as fs from 'fs';
import * as pathModule from 'path';
import * as vscode from 'vscode';
import { SearchOptions, SearchResponse, SearchResult, IndexStatus, IndexProgressUpdate } from './types';

export class CcCliAdapter {
  private cliPath: string;
  private progressEmitter = new vscode.EventEmitter<IndexProgressUpdate>();

  public readonly onIndexProgress = this.progressEmitter.event;

  constructor(cliPath: string) {
    this.cliPath = cliPath;
  }

  /**
   * Execute a search using the cc CLI
   */
  async search(options: SearchOptions): Promise<SearchResponse> {
    const args = this.buildSearchArgs(options);

    return new Promise((resolve, reject) => {
      const results: SearchResult[] = [];
      const errors: string[] = [];

      const child = spawn(this.cliPath, args, {
        cwd: options.path,
        shell: false
      });

      let stdout = '';
      let stderr = '';

      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      child.on('close', (code) => {
        if (code !== 0 && code !== 1) {
          // Code 1 is acceptable (no matches found)
          reject(new Error(`cc exited with code ${code}: ${stderr}`));
          return;
        }

        try {
          // Parse JSONL output
          const lines = stdout.split('\n').filter(line => line.trim());

          for (const line of lines) {
            try {
              const result = this.parseJsonlResult(line);
              if (result) {
                // Apply client-side threshold filtering for semantic/hybrid searches
                if (options.mode === 'semantic' || options.mode === 'hybrid') {
                  if (result.score !== undefined && options.threshold !== undefined) {
                    if (result.score >= options.threshold) {
                      results.push(result);
                    }
                  } else {
                    // If no score or no threshold, include it
                    results.push(result);
                  }
                } else {
                  // For regex searches, include all results
                  results.push(result);
                }
              }
            } catch (e) {
              console.warn('Failed to parse result line:', line, e);
            }
          }

          resolve({
            results,
            count: results.length,
            totalCount: results.length,
            hasMore: false
          });
        } catch (e) {
          reject(e);
        }
      });

      child.on('error', (err) => {
        reject(new Error(`Failed to spawn cc: ${err.message}`));
      });
    });
  }

  /**
   * Get index status for a path
   */
  async getIndexStatus(path: string): Promise<IndexStatus> {
    return new Promise((resolve, reject) => {
      const child = spawn(this.cliPath, ['--status', path], {
        shell: false
      });

      let stdout = '';
      let stderr = '';

      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      child.on('close', (code) => {
        // Parse status output
        // This is a simplified version - actual parsing depends on cc's status output format
        const exists = !stdout.includes('No index found');

        const indexDir = pathModule.join(path, '.cc');
        let lastModified: number | undefined;
        try {
          const stats = fs.statSync(indexDir);
          lastModified = Math.floor(stats.mtimeMs / 1000);
        } catch {
          // ignore missing directory or access issues
        }

        resolve({
          exists,
          path,
          totalFiles: this.extractNumber(stdout, /(\d+)\s+files/),
          totalChunks: this.extractNumber(stdout, /(\d+)\s+chunks/),
          indexPath: indexDir,
          lastModified
        });
      });

      child.on('error', (err) => {
        reject(new Error(`Failed to get index status: ${err.message}`));
      });
    });
  }

  async getDefaultCcignoreContent(indexRoot: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const child = spawn(this.cliPath, ['--print-default-ccignore'], {
        cwd: indexRoot,
        shell: false
      });

      let stdout = '';
      let stderr = '';

      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      child.on('close', (code) => {
        if (code === 0) {
          resolve(stdout);
        } else {
          reject(new Error(`Failed to fetch default .ccignore (exit code ${code}): ${stderr}`));
        }
      });

      child.on('error', (err) => {
        reject(new Error(`Failed to spawn cc: ${err.message}`));
      });
    });
  }

  /**
   * Trigger reindexing
   */
  async reindex(path: string, progress?: vscode.Progress<{ message?: string; increment?: number }>): Promise<void> {
    return new Promise((resolve, reject) => {
      const child = spawn(this.cliPath, ['--index', path], {
        shell: false
      });

      let stderr = '';

      child.stderr.on('data', (data) => {
        const message = data.toString();
        stderr += message;

        // Report progress if available
        if (progress) {
          progress.report({ message: message.trim() });
        }

        const trimmed = message.trim();
        if (trimmed) {
          this.progressEmitter.fire({
            message: trimmed,
            source: 'cli',
            timestamp: Date.now()
          });
        }
      });

      child.on('close', (code) => {
        if (code === 0) {
          this.progressEmitter.fire({
            message: 'Reindexing complete',
            source: 'cli',
            timestamp: Date.now()
          });
          resolve();
        } else {
          this.progressEmitter.fire({
            message: `Reindexing failed (code ${code})`,
            source: 'cli',
            timestamp: Date.now()
          });
          reject(new Error(`Reindexing failed with code ${code}: ${stderr}`));
        }
      });

      child.on('error', (err) => {
        this.progressEmitter.fire({
          message: `Failed to reindex: ${err.message}`,
          source: 'cli',
          timestamp: Date.now()
        });
        reject(new Error(`Failed to reindex: ${err.message}`));
      });
    });
  }

  /**
   * Build command-line arguments for search
   */
  private buildSearchArgs(options: SearchOptions): string[] {
    const args: string[] = ['--jsonl'];

    // Search mode
    switch (options.mode) {
      case 'semantic':
        args.push('--sem');
        break;
      case 'hybrid':
        args.push('--hybrid');
        break;
      case 'regex':
        // Default mode, no flag needed
        break;
    }

    const shouldRerank =
      (options.mode === 'semantic' || options.mode === 'hybrid') && (options.rerank ?? true);
    if (shouldRerank) {
      args.push('--rerank');
    }

    if (options.rerankModel) {
      args.push('--rerank-model', options.rerankModel);
    }

    // Always show scores for semantic/hybrid
    if (options.mode === 'semantic' || options.mode === 'hybrid') {
      args.push('--scores');
    }

    // Always show line numbers
    args.push('-n');

    if (options.includeSnippet === false) {
      args.push('--no-snippet');
    }

    if (options.respectGitignore === false) {
      args.push('--no-ignore');
    }

    if (options.useDefaultExcludes === false) {
      args.push('--no-default-excludes');
    }

    // Query
    args.push(options.query);

    // File targets (include patterns)
    if (options.includePatterns && options.includePatterns.length > 0) {
      options.includePatterns
        .filter(pattern => pattern.trim().length > 0)
        .forEach(pattern => args.push(pattern.trim()));
    } else {
      // Default to searching the current workspace directory
      args.push('.');
    }

    // Optional parameters
    if (options.topK !== undefined) {
      args.push('--topk', options.topK.toString());
    }

    if (options.threshold !== undefined) {
      args.push('--threshold', options.threshold.toString());
    }

    if (options.caseInsensitive) {
      args.push('-i');
    }

    // Default to 2 lines of context for better snippets
    const contextLines = options.contextLines !== undefined ? options.contextLines : 2;
    if (contextLines > 0) {
      args.push('-C', contextLines.toString());
    }

    // Exclude patterns
    if (options.excludePatterns && options.excludePatterns.length > 0) {
      options.excludePatterns.forEach(pattern => {
        if (pattern.trim()) {
          args.push('--exclude', pattern.trim());
        }
      });
    }

    return args;
  }

  /**
   * Parse a JSONL result line from cc output
   */
  private parseJsonlResult(line: string): SearchResult | null {
    if (!line.trim()) {
      return null;
    }

    const data = JSON.parse(line);

    return {
      file: data.file || data.path,
      lineStart: data.line_start || data.span?.line_start || 1,
      lineEnd: data.line_end || data.span?.line_end || 1,
      byteStart: data.byte_start || data.span?.byte_start || 0,
      byteEnd: data.byte_end || data.span?.byte_end || 0,
      preview: data.snippet || data.preview || data.content || '',
      score: data.score,
      language: data.language || data.lang
    };
  }

  /**
   * Extract a number from text using regex
   */
  private extractNumber(text: string, regex: RegExp): number | undefined {
    const match = text.match(regex);
    return match ? parseInt(match[1], 10) : undefined;
  }

  dispose(): void {
    // CLI adapter does not maintain persistent resources
    this.progressEmitter.dispose();
  }
}
