---
layout: default
title: Examples
parent: For Humans Using AI Tools
nav_order: 4
---

# Agent Examples
{: .no_toc }

Real-world workflows showing how AI agents use ck.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Code review assistant

**Goal:** Review pull request for error handling best practices.

### Workflow

**1. Find all error handling:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**2. Find unwrap() usage:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "\\.unwrap\\(\\)|unwrap_or|expect\\(",
    "path": "/project/src",
    "context": 3
  }
}
```

**3. Find proper error propagation:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error propagation with ? operator",
    "path": "/project/src",
    "threshold": 0.8,
    "top_k": 5
  }
}
```

**4. Synthesize report:**
- Compare unwrap() usage with proper error handling examples
- Suggest improvements based on idiomatic patterns found
- Reference specific files and line numbers

---

## Documentation generator

**Goal:** Generate documentation by understanding code structure.

### Workflow

**1. Find public API:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "pub (fn|struct|enum|trait)",
    "path": "/project/src",
    "context": 5
  }
}
```

**2. Understand authentication flow:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication entry point",
    "path": "/project/src",
    "threshold": 0.8,
    "top_k": 3
  }
}
```

**3. Find related functionality:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "token validation",
    "path": "/project/src/auth",
    "threshold": 0.75,
    "top_k": 10
  }
}
```

**4. Generate docs:**
- Map out API surface from public items
- Explain flows based on semantic understanding
- Link related components discovered through search

---

## Refactoring assistant

**Goal:** Help refactor database layer to use connection pooling.

### Workflow

**1. Find all database connections:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "database connection creation",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**2. Find existing pool usage:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "pool",
    "path": "/project/src/db",
    "threshold": 0.6,
    "top_k": 5
  }
}
```

**3. Find connection.execute() calls:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "connection\\.execute|conn\\.query",
    "path": "/project/src",
    "context": 2
  }
}
```

**4. Propose refactoring:**
- Show which files need updates
- Provide example of pooled connection usage from codebase
- Estimate scope (X files, Y functions)

---

## Security audit

**Goal:** Find potential security issues in authentication code.

### Workflow

**1. Find authentication logic:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "user authentication login",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 10
  }
}
```

**2. Check for hardcoded secrets:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(password|secret|key)\\s*=\\s*[\"'][^\"']+[\"']",
    "path": "/project/src",
    "ignore_case": true
  }
}
```

**3. Find SQL queries (injection risk):**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "SQL query string concatenation",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**4. Find input validation:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "input sanitization validation",
    "path": "/project/src/auth",
    "threshold": 0.75
  }
}
```

**5. Generate report:**
- List potential vulnerabilities with file:line references
- Compare against secure patterns found in codebase
- Prioritize by risk level

---

## Onboarding assistant

**Goal:** Help new developer understand codebase architecture.

### Workflow

**1. Find entry points:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "fn main\\(|#\\[actix_web::main\\]|#\\[tokio::main\\]",
    "path": "/project"
  }
}
```

**2. Map out modules:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "module documentation overview",
    "path": "/project/src",
    "threshold": 0.6,
    "top_k": 20
  }
}
```

**3. Find configuration:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "application configuration loading",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 3
  }
}
```

**4. Find key abstractions:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "trait \\w+|struct \\w+.*\\{",
    "path": "/project/src",
    "context": 5
  }
}
```

**5. Create onboarding guide:**
- Start here (main function)
- Key concepts (traits/structs found)
- Configuration guide
- Module breakdown with descriptions

---

## Bug investigation

**Goal:** Find root cause of timeout errors in production.

### Workflow

**1. Find timeout configuration:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "timeout",
    "path": "/project/src",
    "threshold": 0.6,
    "top_k": 15
  }
}
```

**2. Find network operations:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "HTTP request external API",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

**3. Find retry logic:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "retry backoff exponential",
    "path": "/project/src",
    "threshold": 0.65,
    "top_k": 5
  }
}
```

**4. Check error logging:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "log::(error|warn).*timeout",
    "path": "/project/src",
    "ignore_case": true,
    "context": 5
  }
}
```

**5. Analyze findings:**
- Identify where timeouts are configured
- Check if retry logic exists
- Verify error handling and logging
- Suggest improvements

---

## Migration planner

**Goal:** Plan migration from sync to async code.

### Workflow

**1. Find blocking operations:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "blocking file IO network operation",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**2. Find existing async code:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "async fn|await|tokio::|async_std::",
    "path": "/project/src",
    "context": 3
  }
}
```

**3. Find thread spawning:**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "thread spawn",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**4. Create migration plan:**
- List functions to convert (blocking → async)
- Show async patterns already in use
- Estimate effort per module
- Suggest migration order (dependencies first)

---

## Test coverage analyzer

**Goal:** Identify untested code areas.

### Workflow

**1. Find all test functions:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "#\\[test\\]|#\\[tokio::test\\]|fn test_",
    "path": "/project/tests",
    "context": 10
  }
}
```

**2. Find authentication code:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication logic",
    "path": "/project/src/auth",
    "threshold": 0.75
  }
}
```

**3. Find authentication tests:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication tests",
    "path": "/project/tests",
    "threshold": 0.7
  }
}
```

**4. Compare coverage:**
- List functions in src/auth
- Cross-reference with tests found
- Identify untested functions
- Suggest test cases based on function semantics

---

## API design reviewer

**Goal:** Review API consistency and design patterns.

### Workflow

**1. Find public API surface:**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "pub fn \\w+|pub struct \\w+",
    "path": "/project/src/api",
    "context": 5
  }
}
```

**2. Find error handling patterns:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "API error response",
    "path": "/project/src/api",
    "threshold": 0.75,
    "top_k": 10
  }
}
```

**3. Find validation patterns:**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "input validation request",
    "path": "/project/src/api",
    "threshold": 0.7
  }
}
```

**4. Review consistency:**
- Check naming conventions across endpoints
- Verify error handling is consistent
- Ensure validation is applied uniformly
- Suggest improvements for outliers

---

## Tips for agents

{: .tip }
**Start broad, then narrow:** Begin with semantic search on large scope, then drill down with regex or hybrid

{: .tip }
**Combine search modes:** Use semantic to find concepts, regex to find all usages

{: .tip }
**Adjust threshold dynamically:** If too few results, lower threshold; if too many, raise it

{: .tip }
**Use context wisely:** Context lines are expensive; only request when needed for clarity

{: .tip }
**Cache index status:** Don't check on every search; cache during session

---

## Next steps

**→** [MCP API Reference](mcp-api.html) - Complete protocol specification

**→** [Setup Guides](setup-guides.html) - Integration with AI frameworks

**→** [MCP Quick Start](mcp-quickstart.html) - Get started in 5 minutes
