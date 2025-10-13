/**
 * Webview panel provider for the cc search interface
 */

import * as vscode from 'vscode';
import * as path from 'path';
import { CcCliAdapter } from './cliAdapter';
import { CcMcpAdapter } from './mcpAdapter';
import { SearchMode, SearchOptions, SearchResult, CcConfig, SearchResponse, IndexStatus, IndexProgressUpdate } from './types';

interface SearchBackend {
  search(options: SearchOptions): Promise<SearchResponse>;
  reindex(path: string, progress?: vscode.Progress<{ message?: string; increment?: number }>): Promise<void>;
  getIndexStatus(path: string): Promise<IndexStatus>;
  dispose(): void;
  onIndexProgress?: vscode.Event<IndexProgressUpdate>;
  getDefaultCcignoreContent?(indexRoot: string): Promise<string>;
}

export class CcSearchPanel implements vscode.WebviewViewProvider {
  public static readonly viewType = 'cc.searchView';

  private _view?: vscode.WebviewView;
  private adapter: SearchBackend | undefined;
  private config: CcConfig;
  private currentIndexRoot: string | undefined;
  private adapterDisposables: vscode.Disposable[] = [];

  // Fallback copy of cc-core defaults; extension fetches live content when possible.
  private static readonly FALLBACK_CCIGNORE_CONTENT = `# .ccignore - Default patterns for cc semantic search
# Created automatically during first index
# Syntax: same as .gitignore (glob patterns, ! for negation)

# Images
*.png
*.jpg
*.jpeg
*.gif
*.bmp
*.svg
*.ico
*.webp
*.tiff

# Video
*.mp4
*.avi
*.mov
*.mkv
*.wmv
*.flv
*.webm

# Audio
*.mp3
*.wav
*.flac
*.aac
*.ogg
*.m4a

# Binary/Compiled
*.exe
*.dll
*.so
*.dylib
*.a
*.lib
*.obj
*.o

# Archives
*.zip
*.tar
*.tar.gz
*.tgz
*.rar
*.7z
*.bz2
*.gz

# Data files
*.db
*.sqlite
*.sqlite3
*.parquet
*.arrow

# Config formats (issue #27)
*.json
*.yaml
*.yml

# Add your custom patterns below this line
`;

  constructor(
    private readonly _extensionUri: vscode.Uri,
    config: CcConfig
  ) {
    this.config = config;
  }

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken
  ) {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [
        vscode.Uri.joinPath(this._extensionUri, 'webview')
      ]
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    // Handle messages from the webview
    webviewView.webview.onDidReceiveMessage(async (data) => {
      switch (data.type) {
        case 'search':
          await this.handleSearch(data.query, data.mode, data.includePatterns, data.excludePatterns);
          break;
        case 'openFile':
          await this.openFile(data.file, data.line);
          break;
        case 'reindex':
          await this.handleReindex();
          break;
        case 'getIndexStatus':
          await this.handleIndexStatus();
          break;
        case 'openCcignore':
          await this.openCcignore();
          break;
      }
    });

    // Send initial configuration
    this.sendConfig();
  }

  /**
   * Perform a search
   */
  public async search(query: string, mode?: SearchMode, includePatterns?: string[], excludePatterns?: string[]) {
    if (!this._view) {
      return;
    }

    const searchMode = mode || this.config.defaultMode;

    // Show searching state
    this._view.webview.postMessage({
      type: 'searchStarted',
      query,
      mode: searchMode
    });

    try {
      const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
      if (!workspaceFolder) {
        throw new Error('No workspace folder open');
      }

      const indexRoot = this.resolveIndexRoot(workspaceFolder);
      const options: SearchOptions = {
        query,
        mode: searchMode,
        path: indexRoot,
        topK: this.config.topK,
        threshold: this.config.threshold,
        pageSize: this.config.pageSize,
        contextLines: this.config.contextLines,
        includePatterns,
        excludePatterns,
        rerank: searchMode === 'semantic' || searchMode === 'hybrid',
        includeSnippet: true,
        respectGitignore: true,
        useDefaultExcludes: true,
        beforeContextLines: this.config.contextLines,
        afterContextLines: this.config.contextLines
      };

      const response = await this.getAdapter(indexRoot).search(options);

      // Make file paths relative to workspace
      const workspaceRoot = workspaceFolder.uri.fsPath;
      const relativeResults = response.results.map(r => ({
        ...r,
        file: r.file.startsWith(workspaceRoot)
          ? r.file.substring(workspaceRoot.length + 1)
          : r.file,
        absolutePath: r.file // Keep absolute for opening
      }));

      // Send results to webview
      this._view.webview.postMessage({
        type: 'searchResults',
        results: relativeResults,
        count: response.count,
        totalCount: response.totalCount,
        hasMore: response.hasMore,
        searchTimeMs: response.searchTimeMs ?? response.searchTime,
        nextCursor: response.nextCursor
      });
    } catch (error) {
      this._view.webview.postMessage({
        type: 'searchError',
        error: error instanceof Error ? error.message : String(error)
      });
    }
  }

  /**
   * Reindex the workspace
   */
  public async reindex() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
      vscode.window.showErrorMessage('No workspace folder open');
      return;
    }

    const indexRoot = this.resolveIndexRoot(workspaceFolder);
    const adapter = this.getAdapter(indexRoot);
    const source: IndexProgressUpdate['source'] = this.config.mode === 'mcp' ? 'mcp' : 'cli';

    if (this._view) {
      this._view.webview.postMessage({
        type: 'indexProgress',
        update: {
          message: 'Starting reindex…',
          source,
          timestamp: Date.now()
        }
      });
    }

    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Notification,
        title: 'Reindexing with cc',
        cancellable: false
      },
      async (progress) => {
        try {
          await adapter.reindex(indexRoot, progress);
          vscode.window.showInformationMessage('cc: Reindexing complete');

          // Update index status in UI
          await this.handleIndexStatus();
        } catch (error) {
          vscode.window.showErrorMessage(
            `cc: Reindexing failed: ${error instanceof Error ? error.message : String(error)}`
          );
        }
      }
    );
  }

  /**
   * Update configuration
   */
  public updateConfig(config: CcConfig) {
    this.config = config;
    this.resetAdapter();
    this.sendConfig();
  }

  private async handleSearch(query: string, mode: SearchMode, includePatterns?: string[], excludePatterns?: string[]) {
    await this.search(query, mode, includePatterns, excludePatterns);
  }

  private async handleReindex() {
    await this.reindex();
  }

  private async handleIndexStatus() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder || !this._view) {
      return;
    }

    try {
      const indexRoot = this.resolveIndexRoot(workspaceFolder);
      const status = await this.getAdapter(indexRoot).getIndexStatus(indexRoot);

      this._view.webview.postMessage({
        type: 'indexStatus',
        status
      });
    } catch (error) {
      console.error('Failed to get index status:', error);
    }
  }

  private async openFile(file: string, line: number) {
    try {
      // If file is relative, make it absolute
      const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
      const absolutePath = path.isAbsolute(file)
        ? file
        : workspaceFolder
          ? path.join(workspaceFolder.uri.fsPath, file)
          : file;

      const uri = vscode.Uri.file(absolutePath);
      const document = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(document);

      // Jump to line
      const position = new vscode.Position(Math.max(0, line - 1), 0);
      editor.selection = new vscode.Selection(position, position);
      editor.revealRange(
        new vscode.Range(position, position),
        vscode.TextEditorRevealType.InCenter
      );

      // Highlight the line briefly
      const decoration = vscode.window.createTextEditorDecorationType({
        backgroundColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
        isWholeLine: true
      });

      editor.setDecorations(decoration, [new vscode.Range(position, position)]);

      setTimeout(() => {
        decoration.dispose();
      }, 2000);
    } catch (error) {
      vscode.window.showErrorMessage(
        `Failed to open file: ${error instanceof Error ? error.message : String(error)}`
      );
    }
  }

  private sendConfig() {
    if (!this._view) {
      return;
    }

    this._view.webview.postMessage({
      type: 'config',
      config: this.config
    });
  }

  private _getHtmlForWebview(webview: vscode.Webview): string {
    // Get URIs for webview resources
    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, 'webview', 'main.js')
    );
    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, 'webview', 'styles.css')
    );

    // Use a nonce for CSP
    const nonce = getNonce();

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
  <link href="${styleUri}" rel="stylesheet">
  <title>cc Search</title>
</head>
<body>
  <div class="search-container">
    <div class="search-header">
      <div class="search-input-wrapper">
        <input type="text" id="searchInput" placeholder="Search code..." autofocus>
      </div>
      <div class="search-controls">
        <select id="modeSelector">
          <option value="hybrid">Hybrid</option>
          <option value="semantic">Semantic</option>
          <option value="regex">Regex</option>
        </select>
        <button id="reindexButton" title="Reindex workspace">
          <span class="codicon codicon-refresh"></span>
        </button>
      </div>
      <div class="filter-controls">
        <input type="text" id="includeInput" placeholder="files to include: *.ts *.js (comma or space separated)" class="filter-input">
        <input type="text" id="excludeInput" placeholder="files to exclude: *.html *.md (comma or space separated)" class="filter-input">
      </div>
      <div class="index-meta">
        <div class="index-status" id="indexStatus" role="status">
          <span class="status-indicator"></span>
          <span class="status-text">Checking index...</span>
        </div>
        <div class="index-actions">
          <button id="refreshStatusButton" class="icon-button" title="Refresh index status">
            <span class="codicon codicon-refresh"></span>
          </button>
          <button id="ccignoreButton" class="link-button" title="Open .ccignore rules">.ccignore</button>
        </div>
      </div>
      <div class="index-progress hidden" id="indexProgress">
        <span class="codicon codicon-sync"></span>
        <span class="progress-text">Indexing...</span>
      </div>
    </div>

    <div class="search-body">
      <div id="loadingIndicator" class="loading hidden">
        <div class="spinner"></div>
        <span>Searching...</span>
      </div>

      <div id="resultsContainer" class="results-container">
        <div class="empty-state">
          <p>Enter a search query to find code</p>
          <p class="hint">Try searching for concepts like "error handling" or "database connection"</p>
        </div>
      </div>

      <div id="errorContainer" class="error-container hidden"></div>
    </div>

    <div class="search-footer">
      <div id="resultCount" class="result-count"></div>
    </div>
  </div>

  <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
  }

  public dispose() {
    if (this.adapter) {
      this.adapter.dispose();
      this.adapter = undefined;
    }
    this.disposeAdapterEvents();
  }

  private resetAdapter() {
    if (this.adapter) {
      this.adapter.dispose();
    }
    this.adapter = undefined;
    this.currentIndexRoot = undefined;
    this.disposeAdapterEvents();
  }

  private getAdapter(indexRoot: string): SearchBackend {
    if (!this.adapter || this.currentIndexRoot !== indexRoot) {
      if (this.adapter) {
        this.adapter.dispose();
      }
      this.disposeAdapterEvents();
      this.adapter = this.createAdapter(this.config, indexRoot);
      this.currentIndexRoot = indexRoot;
      this.registerAdapterEvents(this.adapter);
    }
    return this.adapter;
  }

  private createAdapter(config: CcConfig, indexRoot: string): SearchBackend {
    if (config.mode === 'mcp') {
      return new CcMcpAdapter(config, indexRoot);
    }
    return new CcCliAdapter(config.cliPath);
  }

  private registerAdapterEvents(adapter: SearchBackend) {
    if (adapter.onIndexProgress) {
      const disposable = adapter.onIndexProgress((update) => {
        if (this._view) {
          this._view.webview.postMessage({
            type: 'indexProgress',
            update
          });
        }
      });
      this.adapterDisposables.push(disposable);
    }
  }

  private disposeAdapterEvents() {
    this.adapterDisposables.forEach((disposable) => disposable.dispose());
    this.adapterDisposables = [];
  }

  private resolveIndexRoot(workspaceFolder?: vscode.WorkspaceFolder): string {
    const workspacePath = workspaceFolder?.uri.fsPath ?? process.cwd();
    const template = this.config.indexRoot ?? '${workspaceFolder}';
    const substituted = template.replace('${workspaceFolder}', workspacePath);
    return path.isAbsolute(substituted)
      ? substituted
      : path.resolve(workspacePath, substituted);
  }

  private async openCcignore() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
      vscode.window.showErrorMessage('No workspace folder open');
      return;
    }

    const ccignoreUri = vscode.Uri.joinPath(workspaceFolder.uri, '.ccignore');
    let exists = true;
    try {
      await vscode.workspace.fs.stat(ccignoreUri);
    } catch (error) {
      exists = false;
    }

    if (!exists) {
      const selection = await vscode.window.showInformationMessage(
        'No .ccignore file found for this workspace. Create one with the default cc patterns?',
        'Create .ccignore',
        'Cancel'
      );

      if (selection !== 'Create .ccignore') {
        return;
      }

      const indexRoot = this.resolveIndexRoot(workspaceFolder);
      const adapter = this.getAdapter(indexRoot);
      let defaultContent = CcSearchPanel.FALLBACK_CCIGNORE_CONTENT;

      if (adapter.getDefaultCcignoreContent) {
        try {
          const fetched = await adapter.getDefaultCcignoreContent(indexRoot);
          if (typeof fetched === 'string' && fetched.trim().length > 0) {
            defaultContent = fetched;
          }
        } catch (error) {
          console.warn('Failed to fetch default .ccignore content from cc backend:', error);
        }
      }

      if (!defaultContent.endsWith('\n')) {
        defaultContent = `${defaultContent}\n`;
      }

      await vscode.workspace.fs.writeFile(
        ccignoreUri,
        Buffer.from(defaultContent, 'utf8')
      );
      vscode.window.showInformationMessage('.ccignore created with default cc patterns');
    }

    const document = await vscode.workspace.openTextDocument(ccignoreUri);
    await vscode.window.showTextDocument(document, { preview: false });
  }
}

function getNonce(): string {
  let text = '';
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
