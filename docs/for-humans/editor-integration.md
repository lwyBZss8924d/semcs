---
layout: default
title: Editor Integration
parent: For Humans
nav_order: 6
---

# Editor Integration
{: .no_toc }

Use ck from your favorite editor.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## VS Code

### Command Palette

**Quick search:**

1. Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
2. Run: **Tasks: Run Task**
3. Configure task in `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "CK Semantic Search",
      "type": "shell",
      "command": "ck --sem '${input:searchQuery}' --jsonl src/",
      "presentation": {
        "reveal": "always",
        "panel": "new"
      }
    }
  ],
  "inputs": [
    {
      "id": "searchQuery",
      "type": "promptString",
      "description": "Semantic search query"
    }
  ]
}
```

### Terminal integration

**Keyboard shortcut:**

Add to `keybindings.json`:

```json
{
  "key": "cmd+k cmd+s",
  "command": "workbench.action.terminal.sendSequence",
  "args": {
    "text": "ck --tui .\n"
  }
}
```

Now `Cmd+K Cmd+S` launches ck TUI in terminal.

### Search results as problems

Show ck results in Problems panel:

**.vscode/tasks.json:**
```json
{
  "label": "CK Search",
  "type": "shell",
  "command": "ck --sem '${input:query}' --jsonl . | jq -r '.file + \":\" + (.start_line|tostring) + \": \" + .text'",
  "problemMatcher": {
    "pattern": {
      "regexp": "^(.+):(\\d+):(.+)$",
      "file": 1,
      "line": 2,
      "message": 3
    }
  }
}
```

---

## Vim / Neovim

### Quick grep replacement

Add to `.vimrc` / `init.vim`:

```vim
" Semantic search
command! -nargs=1 CkSem execute 'cgetexpr system("ck --sem <args> . --jsonl | jq -r \".file + \\\":\\\" + (.start_line|tostring) + \\\":\\\" + .text\"")'

" Open TUI
nnoremap <leader>ck :terminal ck --tui .<CR>
```

**Usage:**
```vim
:CkSem error handling
:CkSem authentication
```

Results populate quickfix list.

### FZF integration

**With fzf.vim:**

```vim
function! CkSearch(query)
  let l:results = systemlist('ck --sem "' . a:query . '" . --jsonl')
  let l:formatted = []
  for result in l:results
    let l:data = json_decode(result)
    call add(l:formatted, l:data.file . ':' . l:data.start_line . ': ' . l:data.text)
  endfor
  call fzf#run(fzf#wrap({
    \ 'source': l:formatted,
    \ 'sink': function('s:open_result'),
    \ 'options': '--preview "bat --color=always {1} --highlight-line {2}"'
  \ }))
endfunction

function! s:open_result(line)
  let l:parts = split(a:line, ':')
  execute 'edit +' . l:parts[1] . ' ' . l:parts[0]
endfunction

command! -nargs=1 Ck call CkSearch(<q-args>)
```

**Usage:**
```vim
:Ck error handling
```

Shows FZF popup with preview.

### Telescope integration (Neovim)

**With telescope.nvim:**

```lua
local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local actions = require("telescope.actions")
local action_state = require("telescope.action_state")

local function ck_search(opts)
  opts = opts or {}

  vim.ui.input({ prompt = "Semantic search: " }, function(query)
    if not query then return end

    local results = vim.fn.systemlist('ck --sem "' .. query .. '" . --jsonl')
    local parsed = {}

    for _, line in ipairs(results) do
      local data = vim.fn.json_decode(line)
      table.insert(parsed, {
        filename = data.file,
        lnum = data.start_line,
        text = data.text
      })
    end

    pickers.new(opts, {
      prompt_title = "CK: " .. query,
      finder = finders.new_table {
        results = parsed,
        entry_maker = function(entry)
          return {
            value = entry,
            display = entry.filename .. ":" .. entry.lnum .. ": " .. entry.text,
            ordinal = entry.text,
            filename = entry.filename,
            lnum = entry.lnum,
          }
        end
      },
      sorter = conf.generic_sorter(opts),
      previewer = conf.grep_previewer(opts),
      attach_mappings = function(prompt_bufnr, map)
        actions.select_default:replace(function()
          actions.close(prompt_bufnr)
          local selection = action_state.get_selected_entry()
          vim.cmd("edit +" .. selection.value.lnum .. " " .. selection.value.filename)
        end)
        return true
      end,
    }):find()
  end)
end

vim.keymap.set('n', '<leader>ck', ck_search, { desc = 'CK semantic search' })
```

**Usage:** Press `<leader>ck`, type query, browse with telescope.

---

## Emacs

### Basic integration

Add to `.emacs` or `init.el`:

```elisp
(defun ck-semantic-search (query)
  "Search codebase semantically with ck."
  (interactive "sSemantic search: ")
  (grep (concat "ck --sem \"" query "\" .")))

(global-set-key (kbd "C-c s") 'ck-semantic-search)
```

### Helm integration

**With helm:**

```elisp
(defun helm-ck-search ()
  "Helm interface for ck semantic search."
  (interactive)
  (helm :sources
        (helm-build-async-source "CK Search"
          :candidates-process
          (lambda ()
            (let ((query (read-string "Query: ")))
              (start-process "ck" nil "ck" "--sem" query "." "--jsonl")))
          :action '(("Open file" . (lambda (candidate)
                                      (let* ((data (json-read-from-string candidate))
                                             (file (cdr (assoc 'file data)))
                                             (line (cdr (assoc 'start_line data))))
                                        (find-file file)
                                        (goto-line line))))))
        :buffer "*helm ck*"))

(global-set-key (kbd "C-c C-s") 'helm-ck-search)
```

---

## JetBrains IDEs

### External Tool

**Configure external tool:**

1. Open **Settings** → **Tools** → **External Tools**
2. Click **+** to add new tool
3. Configure:
   - **Name:** CK Semantic Search
   - **Program:** `ck`
   - **Arguments:** `--sem "$Prompt$" --jsonl $ProjectFileDir$`
   - **Working directory:** `$ProjectFileDir$`

4. Assign keyboard shortcut in **Keymap**

### Terminal integration

Add to terminal:

**File** → **Settings** → **Tools** → **Terminal**

Add alias to shell profile:
```bash
alias cks='ck --sem'
alias ckt='ck --tui .'
```

---

## Sublime Text

### Build System

Create `CK.sublime-build`:

```json
{
  "shell_cmd": "ck --sem \"$prompt\" --jsonl . | jq -r '.file + \":\" + (.start_line|tostring) + \": \" + .text'",
  "file_regex": "^(.+):(\\d+):(.+)$",
  "selector": "source",
  "variants": [
    {
      "name": "TUI",
      "shell_cmd": "osascript -e 'tell app \"Terminal\" to do script \"cd $project_path && ck --tui .\"'"
    }
  ]
}
```

**Usage:** `Cmd+B`, enter query, results in output panel.

---

## Shell aliases

Add to `.bashrc` / `.zshrc`:

```bash
# Quick semantic search
cks() {
  ck --sem "$*" .
}

# Interactive TUI
ckt() {
  ck --tui "${1:-.}"
}

# Search and edit first result
cke() {
  local result=$(ck --sem "$*" --jsonl . | head -1)
  local file=$(echo "$result" | jq -r .file)
  local line=$(echo "$result" | jq -r .start_line)
  ${EDITOR:-vim} "+$line" "$file"
}
```

**Usage:**
```bash
cks error handling
ckt src/
cke authentication logic
```

---

## Tips

{: .tip }
**Use --jsonl for programmatic parsing:**
```bash
ck --sem "query" --jsonl . | jq -r '.file + ":" + (.start_line|tostring)'
```
Perfect for editor integrations.

{: .tip }
**Launch TUI from editor:** Most flexible - full keyboard navigation and preview

{: .tip }
**Quickfix/Problems integration:** Treat search results like compiler errors for easy navigation

---

## Next steps

**→** [Large codebases](large-codebases.html) - Performance tips for big projects

**→** [CLI reference](cli-reference.html) - All output formats and flags

**→** [TUI guide](tui.html) - Master the interactive interface
