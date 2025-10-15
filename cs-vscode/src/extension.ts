/**
 * cc VS Code Extension
 *
 * Brings semantic code search powered by cc directly into VS Code
 */

import * as vscode from 'vscode';
import { CcSearchPanel } from './searchPanel';
import { CcConfig } from './types';

let searchPanel: CcSearchPanel | undefined;

export function activate(context: vscode.ExtensionContext) {
  console.log('cc extension is now active');

  // Load configuration
  const config = loadConfig();

  // Register the webview view provider
  const provider = new CcSearchPanel(context.extensionUri, config);
  searchPanel = provider;

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(
      CcSearchPanel.viewType,
      provider,
      {
        webviewOptions: {
          retainContextWhenHidden: true
        }
      }
    )
  );

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand('cc.search', async () => {
      // Focus the search view
      await vscode.commands.executeCommand('cc.searchView.focus');
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('cc.searchSelection', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return;
      }

      const selection = editor.selection;
      const text = editor.document.getText(selection);

      if (!text) {
        vscode.window.showWarningMessage('No text selected');
        return;
      }

      // Focus the search view and perform search
      await vscode.commands.executeCommand('cc.searchView.focus');

      if (searchPanel) {
        await searchPanel.search(text, config.defaultMode);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('cc.reindex', async () => {
      if (searchPanel) {
        await searchPanel.reindex();
      }
    })
  );

  // Listen for configuration changes
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('cc')) {
        const newConfig = loadConfig();
        if (searchPanel) {
          searchPanel.updateConfig(newConfig);
        }
      }
    })
  );

  // Check if cc binary is available
  checkCcAvailability(config.cliPath);
}

export function deactivate() {
  if (searchPanel) {
    searchPanel.dispose();
    searchPanel = undefined;
  }
}

/**
 * Load configuration from VS Code settings
 */
function loadConfig(): CcConfig {
  const config = vscode.workspace.getConfiguration('cc');

  return {
    mode: config.get('mode', 'cli') as 'cli' | 'mcp',
    cliPath: config.get('cliPath', 'cc'),
    mcpCommand: config.get('mcp.command', 'cc'),
    mcpArgs: config.get('mcp.args', ['--serve']),
    indexRoot: config.get('index.root', '${workspaceFolder}'),
    defaultMode: config.get('defaultMode', 'semantic') as 'hybrid' | 'semantic' | 'regex',
    pageSize: config.get('pageSize', 25),
    threshold: config.get('threshold', 0.6),
    topK: config.get('topK', 50),
    contextLines: config.get('contextLines', 4)
  };
}

/**
 * Check if cc binary is available on PATH
 */
async function checkCcAvailability(cliPath: string) {
  const { spawn } = require('child_process');

  const child = spawn(cliPath, ['--version'], { shell: false });

  let available = false;

  child.on('close', (code: number) => {
    if (code === 0) {
      available = true;
    } else {
      vscode.window.showWarningMessage(
        `cc binary not found at "${cliPath}". Please install cc or update the "cc.cliPath" setting.`,
        'Install cc',
        'Open Settings'
      ).then((selection) => {
        if (selection === 'Install cc') {
          vscode.env.openExternal(vscode.Uri.parse('https://github.com/lwyBZss8924d/semcs#installation'));
        } else if (selection === 'Open Settings') {
          vscode.commands.executeCommand('workbench.action.openSettings', 'cc.cliPath');
        }
      });
    }
  });

  child.on('error', () => {
    vscode.window.showWarningMessage(
      `cc binary not found at "${cliPath}". Please install cc or update the "cc.cliPath" setting.`,
      'Install cc',
      'Open Settings'
    ).then((selection) => {
      if (selection === 'Install cc') {
        vscode.env.openExternal(vscode.Uri.parse('https://github.com/lwyBZss8924d/semcs#installation'));
      } else if (selection === 'Open Settings') {
        vscode.commands.executeCommand('workbench.action.openSettings', 'cc.cliPath');
      }
    });
  });
}
