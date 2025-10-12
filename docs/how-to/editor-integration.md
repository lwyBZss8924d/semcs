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

**Goal:** Set up ck with your favorite code editor for seamless semantic search.

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
3. Run ck commands directly

**Example workflow:**
```bash
# In VS Code terminal
ck --tui .
ck --sem "error handling" src/
```

### Method 2: Tasks Integration

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "ck: Semantic Search",
      "type": "shell",
      "command": "ck",
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
      "label": "ck: Search Current File",
      "type": "shell",
      "command": "ck",
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
3. Select "ck: Semantic Search"

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
:!ck --tui .

" Search with query
:!ck --sem "error handling" .

" Search current file's directory
:!ck --sem "pattern" %:p:h
```

### Custom Commands

Add to `.vimrc` or `init.vim`:

```vim
" ck semantic search commands
command! -nargs=1 CkSem :!ck --sem "<args>" .
command! -nargs=1 CkRegex :!ck "<args>" .
command! CkTui :!ck --tui .

" Search current file's directory
command! -nargs=1 CkSemFile :!ck --sem "<args>" %:p:h
```

**Usage:**
```vim
:CkSem "error handling"
:CkRegex "TODO"
:CkTui
```

### Advanced Vim Integration

**With fzf.vim:**
```vim
function! CkSemanticSearch(query)
  let results = system('ck --sem "' . a:query . '" --jsonl .')
  let files = []
  for line in split(results, '\n')
    if line != ''
      let data = json_decode(line)
      call add(files, data.file . ':' . data.line_start)
    endif
  endfor
  call fzf#run(fzf#wrap({'source': files}))
endfunction

command! -nargs=1 CkSemFzf call CkSemanticSearch(<q-args>)
```

### Neovim with Lua

**For Neovim users:**

```lua
-- ~/.config/nvim/lua/ck.lua
local M = {}

function M.semantic_search(query)
  local cmd = string.format('ck --sem "%s" .', query)
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
  vim.fn.jobstart('ck --tui .', {
    detach = true
  })
end

-- Commands
vim.api.nvim_create_user_command('CkSem', function(opts)
  M.semantic_search(opts.args)
end, { nargs = 1 })

vim.api.nvim_create_user_command('CkTui', M.tui, {})

return M
```

---

## Emacs Integration

### Basic Setup

**M-x shell integration:**
```elisp
;; ck.el - ck integration for Emacs
(defun ck-semantic-search (query)
  "Run ck semantic search with QUERY."
  (interactive "sSearch query: ")
  (let ((default-directory (project-root (project-current))))
    (shell-command (format "ck --sem '%s' ." query))))

(defun ck-tui ()
  "Launch ck TUI."
  (interactive)
  (let ((default-directory (project-root (project-current))))
    (async-shell-command "ck --tui .")))

(defun ck-regex-search (pattern)
  "Run ck regex search with PATTERN."
  (interactive "sRegex pattern: ")
  (let ((default-directory (project-root (project-current))))
    (shell-command (format "ck '%s' ." pattern))))

;; Key bindings
(global-set-key (kbd "C-c c s") 'ck-semantic-search)
(global-set-key (kbd "C-c c t") 'ck-tui)
(global-set-key (kbd "C-c c r") 'ck-regex-search)
```

### Advanced Emacs Integration

**With helm:**
```elisp
(defun ck-helm-semantic-search ()
  "Helm interface for ck semantic search."
  (interactive)
  (helm :sources (helm-build-sync-source "ck semantic search"
                   :candidates (lambda ()
                                 (let ((query (helm-read-string "Search: ")))
                                   (split-string (shell-command-to-string
                                                  (format "ck --sem '%s' ." query))
                                                 "\n" t)))
                   :action (lambda (candidate)
                             (find-file (car (split-string candidate ":")))))))
```

---

## Other Editors

### Sublime Text

**Build system** (save as `ck.sublime-build`):
```json
{
  "cmd": ["ck", "--tui", "$file_path"],
  "selector": "source",
  "shell": true
}
```

### Atom

**Package integration:**
```javascript
// In your Atom package
const { exec } = require('child_process');

function ckSemanticSearch(query) {
  exec(`ck --sem "${query}" .`, (error, stdout, stderr) => {
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
   - **Name:** ck Semantic Search
   - **Program:** ck
   - **Arguments:** `--sem $Prompt$ .`
   - **Working directory:** `$ProjectFileDir$`

### Helix

**Commands in `config.toml`:**
```toml
[keys.normal]
"<space>c" = ":sh ck --tui ."
"<space>s" = ":sh ck --sem"
```

---

## Custom Workflows

### Git Integration

**Pre-commit hooks:**
```bash
#!/bin/sh
# .git/hooks/pre-commit

# Search for TODOs in staged files
if ck "TODO" --glob "*.rs" --files-with-matches . | grep -q .; then
  echo "Warning: TODOs found in staged files"
  ck "TODO" --glob "*.rs" .
fi
```

### Project-specific Scripts

**Create `scripts/search.sh`:**
```bash
#!/bin/bash

case "$1" in
  "auth")
    ck --sem "authentication" src/
    ;;
  "error")
    ck --sem "error handling" src/
    ;;
  "test")
    ck "fn test_" tests/
    ;;
  "tui")
    ck --tui .
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
# ck aliases
alias cks='ck --sem'
alias ckt='ck --tui'
alias ckr='ck'
alias ckh='ck --hybrid'

# Project-specific searches
alias ckauth='ck --sem "authentication" src/'
alias ckerror='ck --sem "error handling" src/'
alias cktest='ck "fn test_" tests/'
```

---

## Editor-specific Tips

### VS Code Tips

**Integrated terminal:**
- Use `Ctrl+`` to toggle terminal
- Split terminal for multiple ck sessions
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
nnoremap <leader>cs :CkSem <C-R><C-W><CR>
nnoremap <leader>cr :CkRegex <C-R><C-W><CR>
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

**Editor can't find ck:**
- Ensure ck is in PATH
- Restart editor after installing ck
- Check editor's PATH configuration

**Slow performance:**
- Use specific directories instead of `.`
- Exclude large directories with .ckignore
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
- Use .ckignore to exclude irrelevant files
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
