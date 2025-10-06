---
layout: default
title: Setup Guides
parent: For Humans Using AI Tools
nav_order: 5
---

# Setup Guides
{: .no_toc }

Integrate ck with popular AI tools and frameworks.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Claude Desktop

### Installation

**1. Install ck:**
```bash
cargo install ck-search
```

**2. Configure MCP server:**

**macOS/Linux:**
```bash
nano ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

**Windows:**
```cmd
notepad %APPDATA%\Claude\claude_desktop_config.json
```

**Add configuration:**
```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"],
      "env": {
        "CK_MODEL": "default"
      }
    }
  }
}
```

**3. Restart Claude Desktop**

### Verification

Look for MCP indicator (🔌) in bottom-left corner.

**Test command:**
```
Search for error handling in ~/projects/myapp
```

Claude should respond with code search results.

---

## Claude Code (VS Code Extension)

### Setup

**1. Install Claude Code extension** from VS Code marketplace

**2. Configure MCP in settings:**

Open Command Palette (`Cmd+Shift+P`) → **Claude Code: Configure MCP Servers**

**Add:**
```json
{
  "ck-search": {
    "command": "ck",
    "args": ["--serve"]
  }
}
```

**3. Reload VS Code**

### Usage

Use @ mention:
```
@ck-search find authentication code in current workspace
```

Or natural commands:
```
Use ck to search for database queries
```

---

## LangChain

### Python implementation

```python
from langchain.tools import BaseTool
from typing import Optional
import subprocess
import json

class CKSemanticSearchTool(BaseTool):
    name = "ck_semantic_search"
    description = """
    Search codebase semantically by meaning, not exact text.
    Input should be a JSON string with 'query' and 'path' fields.
    Example: {"query": "error handling", "path": "/home/user/project"}
    """

    def _run(self, tool_input: str) -> str:
        params = json.loads(tool_input)
        query = params["query"]
        path = params["path"]

        result = subprocess.run(
            ["ck", "--sem", query, path, "--jsonl"],
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            return f"Error: {result.stderr}"

        # Parse and format results
        lines = result.stdout.strip().split('\n')
        results = [json.loads(line) for line in lines if line]

        output = []
        for r in results[:5]:  # Top 5
            output.append(
                f"{r['file']}:{r['start_line']}-{r['end_line']} "
                f"(score: {r['score']:.2f})\n{r['text']}\n"
            )

        return "\n".join(output)

    async def _arun(self, tool_input: str) -> str:
        raise NotImplementedError("Async not supported")

class CKRegexSearchTool(BaseTool):
    name = "ck_regex_search"
    description = """
    Search codebase with regex patterns.
    Input should be JSON with 'pattern' and 'path'.
    Example: {"pattern": "fn test_\\w+", "path": "/home/user/project"}
    """

    def _run(self, tool_input: str) -> str:
        params = json.loads(tool_input)
        pattern = params["pattern"]
        path = params["path"]

        result = subprocess.run(
            ["ck", pattern, path, "-n"],
            capture_output=True,
            text=True
        )

        return result.stdout if result.returncode == 0 else result.stderr

    async def _arun(self, tool_input: str) -> str:
        raise NotImplementedError("Async not supported")
```

### Usage with agent

```python
from langchain.agents import initialize_agent, AgentType
from langchain.llms import OpenAI

tools = [CKSemanticSearchTool(), CKRegexSearchTool()]

agent = initialize_agent(
    tools,
    OpenAI(temperature=0),
    agent=AgentType.ZERO_SHOT_REACT_DESCRIPTION,
    verbose=True
)

agent.run(
    "Find all error handling code in /home/user/myapp/src "
    "and check if it follows best practices"
)
```

---

## AutoGPT

### Plugin setup

**1. Create plugin file:**

`plugins/ck_search_plugin.py`:
```python
from auto_gpt_plugin_template import AutoGPTPluginTemplate
import subprocess
import json

class CKSearchPlugin(AutoGPTPluginTemplate):
    def __init__(self):
        super().__init__()
        self._name = "CK Search Plugin"
        self._version = "0.1.0"
        self._description = "Semantic code search"

    def can_handle_post_prompt(self) -> bool:
        return True

    def post_prompt(self, prompt: str) -> str:
        return prompt

    def can_handle_on_instruction(self) -> bool:
        return True

    def on_instruction(self, instructions: list) -> None:
        pass

    def can_handle_text_embedding(self) -> bool:
        return False

    def handle_text_embedding(self, text: str) -> list:
        return []

    def can_handle_user_input(self) -> bool:
        return False

    def user_input(self, user_input: str) -> str:
        return user_input

    def can_handle_report(self) -> bool:
        return False

    def report(self, report: str) -> None:
        pass

    def semantic_search(self, query: str, path: str) -> str:
        """Search code semantically"""
        result = subprocess.run(
            ["ck", "--sem", query, path, "--jsonl"],
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            return f"Error: {result.stderr}"

        lines = result.stdout.strip().split('\n')
        results = [json.loads(line) for line in lines if line]

        return json.dumps(results[:10], indent=2)
```

**2. Register commands:**

Add to `commands` in plugin:
```python
def _generate_commands(self):
    return [
        {
            "name": "ck_search",
            "description": "Search code semantically",
            "args": {
                "query": "search query",
                "path": "directory path"
            },
            "function": self.semantic_search
        }
    ]
```

---

## GPT Engineer

### Integration

**1. Create custom tool:**

`gpt_engineer/tools/ck_search.py`:
```python
import subprocess
import json
from pathlib import Path

def semantic_search(query: str, path: str = ".") -> list[dict]:
    """Search codebase semantically with ck"""
    result = subprocess.run(
        ["ck", "--sem", query, path, "--jsonl"],
        capture_output=True,
        text=True,
        cwd=Path.cwd()
    )

    if result.returncode != 0:
        return []

    lines = result.stdout.strip().split('\n')
    return [json.loads(line) for line in lines if line]

def find_pattern(pattern: str, path: str = ".") -> str:
    """Find code with regex pattern"""
    result = subprocess.run(
        ["ck", pattern, path, "-n"],
        capture_output=True,
        text=True,
        cwd=Path.cwd()
    )

    return result.stdout
```

**2. Add to toolset:**

In `gpt_engineer/core/ai.py`:
```python
from gpt_engineer.tools.ck_search import semantic_search, find_pattern

TOOLS = [
    # ... existing tools
    {
        "type": "function",
        "function": {
            "name": "semantic_search",
            "description": "Search code by meaning, not exact text",
            "parameters": {
                "type": "object",
                "properties": {
                    "query": {"type": "string"},
                    "path": {"type": "string"}
                },
                "required": ["query"]
            }
        }
    }
]
```

---

## Custom MCP client

### Minimal Python client

```python
import json
import subprocess
import sys

class CKMCPClient:
    def __init__(self):
        self.process = subprocess.Popen(
            ["ck", "--serve"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        self.request_id = 0

    def send(self, method: str, params: dict = None) -> dict:
        self.request_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params or {}
        }

        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()

        response = self.process.stdout.readline()
        return json.loads(response)

    def initialize(self):
        return self.send("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "custom-client", "version": "1.0"}
        })

    def list_tools(self):
        return self.send("tools/list")

    def call_tool(self, name: str, arguments: dict):
        return self.send("tools/call", {
            "name": name,
            "arguments": arguments
        })

    def close(self):
        self.process.terminate()
        self.process.wait()

# Usage
client = CKMCPClient()
client.initialize()

result = client.call_tool("semantic_search", {
    "query": "error handling",
    "path": "/home/user/project",
    "threshold": 0.7
})

print(result["result"]["content"][0]["text"])

client.close()
```

---

## Cursor IDE

### Setup

**1. Install ck:**
```bash
cargo install ck-search
```

**2. Configure as MCP server:**

Cursor uses similar config to Claude Desktop.

**Create/edit:** `~/.cursor/mcp_config.json`

```json
{
  "mcpServers": {
    "ck-search": {
      "command": "ck",
      "args": ["--serve"]
    }
  }
}
```

**3. Restart Cursor**

### Usage

Use Cursor's AI chat:
```
@ck search for authentication code
```

Or natural language:
```
Use ck to find all database queries
```

---

## Tips

{: .tip }
**Test command-line first:** Before MCP integration, verify `ck --sem "test" .` works

{: .tip }
**Use absolute paths in MCP:** Relative paths may not work as expected

{: .tip }
**Set environment variables:** Configure `CK_MODEL`, `CK_WORKERS` in server config

{: .tip }
**Check logs:** Most AI tools log MCP server output for debugging

---

## Next steps

**→** [MCP API Reference](mcp-api.html) - Complete protocol specification

**→** [Examples](examples.html) - Real-world integration examples

**→** [MCP Quick Start](mcp-quickstart.html) - Get started in 5 minutes
