---
layout: default
title: Editor Integration
parent: How-To Guides
nav_order: 4
---

# Editor Integration

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Set up cc with your favorite code editor for seamless semantic search.

**You'll learn:**
- VS Code integration
- Vim/Neovim setup
- Emacs configuration
- Other editor integrations
- Custom workflows

---

## VS Code Integration

### Method 1: Command Palette

**Quick search:**
1. Press `Ctrl+Shift+P` (Windows/Linux) or `Cmd+Shift+P` (macOS)
2. Type "Terminal: Create New Terminal"
3. Run cc commands directly

**Example workflow:**
```bash
# In VS Code terminal
cs --tui .
cs --sem "error handling" src/
```

### Method 2: Tasks Integration

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "cc: Semantic Search",
      "type": "shell",
      "command": "cc",
      "args": ["--tui", "${workspaceFolder}"],
      "group": "build",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "new"
      }
    },
    {
      "label": "cc: Search Current File",
      "type": "shell",
      "command": "cc",
      "args": ["--sem", "${input:searchQuery}", "${fileDirname}"],
      "group": "build"
    }
  ],
  "inputs": [
    {
      "id": "searchQuery",
      "description": "Search query",
      "default": "error handling",
      "type": "promptString"
    }
  ]
}
```

**Usage:**
1. Press `Ctrl+Shift+P`
2. Type "Tasks: Run Task"
3. Select "cc: Semantic Search"

### Method 3: Extension (Future)

**Planned VS Code extension:**
- Integrated search panel
- Inline results
- Quick actions
- Keyboard shortcuts

---

## Vim/Neovim Integration

### Basic Integration

**Command mode:**
```vim
" Search current directory
:!cc --tui .

" Search with query
:!cc --sem "error handling" .

" Search current file's directory
:!cc --sem "pattern" %:p:h
```

### Custom Commands

Add to `.vimrc` or `init.vim`:

```vim
" cc semantic search commands
command! -nargs=1 CcSem :!cc --sem "<args>" .
command! -nargs=1 CcRegex :!cc "<args>" .
command! CcTui :!cc --tui .

" Search current file's directory
command! -nargs=1 CcSemFile :!cc --sem "<args>" %:p:h
```

**Usage:**
```vim
:CcSem "error handling"
:CcRegex "TODO"
:CcTui
```

### Advanced Vim Integration

**With fzf.vim:**
```vim
function! CcSemanticSearch(query)
  let results = system('cc --sem "' . a:query . '" --jsonl .')
  let files = []
  for line in split(results, '\n')
    if line != ''
      let data = json_decode(line)
      call add(files, data.file . ':' . data.line_start)
    endif
  endfor
  call fzf#run(fzf#wrap({'source': files}))
endfunction

command! -nargs=1 CcSemFzf call CcSemanticSearch(<q-args>)
```

### Neovim with Lua

**For Neovim users:**

```lua
-- ~/.config/nvim/lua/cc.lua
local M = {}

function M.semantic_search(query)
  local cmd = string.format('cc --sem "%s" .', query)
  vim.fn.jobstart(cmd, {
    on_stdout = function(_, data)
      -- Process results
      for _, line in ipairs(data) do
        if line ~= "" then
          print(line)
        end
      end
    end
  })
end

function M.tui()
  vim.fn.jobstart('cc --tui .', {
    detach = true
  })
end

-- Commands
vim.api.nvim_create_user_command('CcSem', function(opts)
  M.semantic_search(opts.args)
end, { nargs = 1 })

vim.api.nvim_create_user_command('CcTui', M.tui, {})

return M
```

---

## Emacs Integration

### Basic Setup

**M-x shell integration:**
```elisp
;; cc.el - cc integration for Emacs
(defun cc-semantic-search (query)
  "Run cc semantic search with QUERY."
  (interactive "sSearch query: ")
  (let ((default-directory (project-root (project-current))))
    (shell-command (format "cc --sem '%s' ." query))))

(defun cs-tui ()
  "Launch cc TUI."
  (interactive)
  (let ((default-directory (project-root (project-current))))
    (async-shell-command "cc --tui .")))

(defun cc-regex-search (pattern)
  "Run cc regex search with PATTERN."
  (interactive "sRegex pattern: ")
  (let ((default-directory (project-root (project-current))))
    (shell-command (format "cc '%s' ." pattern))))

;; Key bindings
(global-set-key (kbd "C-c c s") 'cc-semantic-search)
(global-set-key (kbd "C-c c t") 'cs-tui)
(global-set-key (kbd "C-c c r") 'cc-regex-search)
```

### Advanced Emacs Integration

**With helm:**
```elisp
(defun cc-helm-semantic-search ()
  "Helm interface for cc semantic search."
  (interactive)
  (helm :sources (helm-build-sync-source "cc semantic search"
                   :candidates (lambda ()
                                 (let ((query (helm-read-string "Search: ")))
                                   (split-string (shell-command-to-string
                                                  (format "cc --sem '%s' ." query))
                                                 "\n" t)))
                   :action (lambda (candidate)
                             (find-file (car (split-string candidate ":")))))))
```

---

## Other Editors

### Sublime Text

**Build system** (save as `cc.sublime-build`):
```json
{
  "cmd": ["cc", "--tui", "$file_path"],
  "selector": "source",
  "shell": true
}
```

### Atom

**Package integration:**
```javascript
// In your Atom package
const { exec } = require('child_process');

function ccSemanticSearch(query) {
  exec(`cs --sem "${query}" .`, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error: ${error}`);
      return;
    }
    console.log(stdout);
  });
}
```

### JetBrains IDEs

**External tool configuration:**
1. Go to Settings → Tools → External Tools
2. Add new tool:
   - **Name:** cc Semantic Search
   - **Program:** cs
   - **Arguments:** `--sem $Prompt$ .`
   - **Working directory:** `$ProjectFileDir$`

### Helix

**Commands in `config.toml`:**
```toml
[keys.normal]
"<space>c" = ":sh cc --tui ."
"<space>s" = ":sh cc --sem"
```

---

## Custom Workflows

### Git Integration

**Pre-commit hooks:**
```bash
#!/bin/sh
# .git/hooks/pre-commit

# Search for TODOs in staged files
if cc "TODO" --glob "*.rs" --files-with-matches . | grep -q .; then
  echo "Warning: TODOs found in staged files"
  cc "TODO" --glob "*.rs" .
fi
```

### Project-specific Scripts

**Create `scripts/search.sh`:**
```bash
#!/bin/bash

case "$1" in
  "auth")
    cc --sem "authentication" src/
    ;;
  "error")
    cc --sem "error handling" src/
    ;;
  "test")
    cc "fn test_" tests/
    ;;
  "tui")
    cc --tui .
    ;;
  *)
    echo "Usage: $0 {auth|error|test|tui}"
    exit 1
    ;;
esac
```

**Make executable:**
```bash
chmod +x scripts/search.sh
./scripts/search.sh auth
```

### Shell Aliases

**Add to `.bashrc` or `.zshrc`:**
```bash
# cc aliases
alias ccs='cc --sem'
alias cct='cc --tui'
alias ccr='cc'
alias cch='cc --hybrid'

# Project-specific searches
alias ccauth='cc --sem "authentication" src/'
alias ccerror='cc --sem "error handling" src/'
alias cctest='cc "fn test_" tests/'
```

---

## Editor-specific Tips

### VS Code Tips

**Integrated terminal:**
- Use `Ctrl+`` to toggle terminal
- Split terminal for multiple cc sessions
- Use terminal tabs for different searches

**Tasks integration:**
- Create tasks for common searches
- Use input variables for dynamic queries
- Bind tasks to keyboard shortcuts

### Vim Tips

**Quick navigation:**
```vim
" Jump to search results
:grep "pattern" | copen

" Use quickfix list
:cnext
:cprev
```

**Custom mappings:**
```vim
" Search word under cursor
nnoremap <leader>cs :CcSem <C-R><C-W><CR>
nnoremap <leader>cr :CcRegex <C-R><C-W><CR>
```

### Emacs Tips

**Project integration:**
- Use `project.el` for project-aware searches
- Integrate with `dired` for file navigation
- Use `compilation-mode` for search results

---

## Troubleshooting

### Common Issues

**Command not found:**
```bash
# Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

**Editor can't find cc:**
- Ensure cc is in PATH
- Restart editor after installing cs
- Check editor's PATH configuration

**Slow performance:**
- Use specific directories instead of `.`
- Exclude large directories with .ccignore
- Use regex search for exact patterns

### Editor-specific Issues

**VS Code:**
- Check terminal PATH vs editor PATH
- Use integrated terminal for consistency

**Vim/Neovim:**
- Ensure shell integration works
- Check `:!` command functionality

**Emacs:**
- Verify `shell-command` works
- Check `default-directory` setting

---

## Best Practices

### General Tips

**Use appropriate search modes:**
- Semantic for concept discovery
- Regex for exact patterns
- Hybrid for keyword + concept

**Optimize for your workflow:**
- Create aliases for common searches
- Use editor-specific integrations
- Set up project-specific scripts

**Performance considerations:**
- Search specific directories when possible
- Use .ccignore to exclude irrelevant files
- Consider indexing time for large repos

### Editor-specific Best Practices

**VS Code:**
- Use tasks for repeatable searches
- Leverage integrated terminal
- Create workspace-specific settings

**Vim/Neovim:**
- Create custom commands for common patterns
- Use fzf or similar for result navigation
- Integrate with existing plugins

**Emacs:**
- Use project-aware searches
- Integrate with existing packages
- Create custom key bindings

---

## Next Steps

**→ Learn advanced search:** [Find Specific Patterns](find-patterns.html)

**→ Optimize performance:** [Large Codebases](large-codebases.html)

**→ Connect AI tools:** [AI Integration](../ai-integration/mcp-quickstart.html)

**→ Configure filtering:** [Configuration](configuration.html)
