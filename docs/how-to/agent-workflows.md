---
layout: default
title: Agent Workflows
parent: How-To Guides
nav_order: 2
---

# Agent Workflows
{: .no_toc }

Practical workflows showing how AI agents use ck to solve common development tasks.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Introduction

This guide provides task-oriented workflows for AI agents using ck through the MCP server. Each workflow demonstrates how to combine semantic search, regex search, and hybrid search to solve real development challenges.

**What you'll learn:**
- How to explore unfamiliar codebases efficiently
- How to perform automated code reviews
- How to plan refactoring projects
- How to find bugs and security issues
- How to generate documentation from code

**Prerequisites:**
- ck MCP server running and connected to your AI tool
- Indexed codebase (see [Setup MCP Server](setup-mcp-server.html))

---

## Code Exploration Workflows

### Understanding Authentication Flow

**Task:** "Help me understand how authentication works in this codebase"

**Why this workflow:** Authentication is often spread across multiple files. Semantic search helps you discover the complete flow without knowing specific implementation details.

**Step 1: Find authentication entry points**
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

**Step 2: Understand token validation**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "JWT token validation",
    "path": "/project/src/auth",
    "threshold": 0.8,
    "top_k": 5
  }
}
```

**Step 3: Find authentication middleware**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(middleware|auth|authenticate)",
    "path": "/project/src",
    "ignore_case": true,
    "context": 3
  }
}
```

**Expected outcome:**
- Complete picture of authentication flow
- Identification of entry points, validation logic, and middleware
- Understanding of security mechanisms in use

---

### Discovering API Patterns

**Task:** "Show me all the REST API endpoints and their patterns"

**Why this workflow:** API endpoints may use different frameworks or registration patterns. This approach finds them all regardless of implementation style.

**Step 1: Find API route definitions**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "REST API endpoints routes",
    "path": "/project/src",
    "top_k": 20,
    "threshold": 0.7
  }
}
```

**Step 2: Find HTTP method handlers**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(GET|POST|PUT|DELETE|PATCH)\\s+[\"'`]/",
    "path": "/project/src",
    "context": 2
  }
}
```

**Step 3: Find route registration patterns**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "route registration app.use",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 10
  }
}
```

**Expected outcome:**
- Comprehensive list of all API endpoints
- Understanding of routing patterns used
- Identification of API versioning approach

---

### Tracing Data Flow

**Task:** "Trace how user data flows through the application"

**Why this workflow:** Understanding data flow requires connecting multiple components. Semantic search excels at finding conceptually related code.

**Step 1: Find data models**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "user data model schema",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 5
  }
}
```

**Step 2: Find database operations**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "user database queries CRUD",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 15
  }
}
```

**Step 3: Find API handlers that use the data**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "user API handler controller",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

**Expected outcome:**
- Map of data flow from API to database
- Identification of transformation layers
- Understanding of validation points

---

## Code Review Workflows

### Error Handling Review

**Task:** "Review this codebase for error handling best practices"

**Why this workflow:** Error handling inconsistencies are common quality issues. This workflow finds both good and bad patterns.

**Step 1: Find error handling patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling exception management",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**Step 2: Find problematic unwrap() usage (Rust)**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "\\.unwrap\\(\\)|\\.expect\\(",
    "path": "/project/src",
    "context": 3
  }
}
```

**Step 3: Find proper error propagation**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error propagation with ? operator Result",
    "path": "/project/src",
    "threshold": 0.8,
    "top_k": 5
  }
}
```

**Step 4: Check error logging**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(log::|logger\\.|console\\.(error|warn))",
    "path": "/project/src",
    "context": 2
  }
}
```

**Expected outcome:**
- List of error handling anti-patterns with locations
- Examples of proper error handling from the codebase
- Specific recommendations for improvement

---

### Security Audit

**Task:** "Perform a security audit focusing on common vulnerabilities"

**Why this workflow:** Security issues often follow patterns. This systematic approach covers major vulnerability classes.

**Step 1: Find input validation**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "input validation sanitization",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 15
  }
}
```

**Step 2: Check for SQL injection risks**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(SELECT|INSERT|UPDATE|DELETE).*\\+.*\\w+",
    "path": "/project/src",
    "context": 3
  }
}
```

**Step 3: Find hardcoded secrets**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(password|secret|key|token)\\s*=\\s*[\"'][^\"']+[\"']",
    "path": "/project/src",
    "ignore_case": true,
    "context": 2
  }
}
```

**Step 4: Check authentication bypass risks**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication bypass admin override",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

**Step 5: Find sensitive data handling**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "password hashing encryption",
    "path": "/project/src/auth",
    "threshold": 0.75
  }
}
```

**Expected outcome:**
- Categorized list of security issues (high/medium/low risk)
- Specific file and line number references
- Recommendations with examples from secure code in the codebase

---

### Performance Analysis

**Task:** "Analyze code for performance bottlenecks"

**Why this workflow:** Performance issues often hide in database queries, loops, and inefficient algorithms.

**Step 1: Find database query patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "database query execution",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**Step 2: Find N+1 query patterns**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "for\\s+\\w+\\s+in\\s+.*\\{[^}]*query",
    "path": "/project/src",
    "context": 5
  }
}
```

**Step 3: Find caching mechanisms**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "caching memoization",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 10
  }
}
```

**Step 4: Check for inefficient loops**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "nested loops iteration",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**Expected outcome:**
- Identification of N+1 query problems
- Areas where caching could help
- Inefficient algorithms with suggested alternatives

---

## Refactoring Workflows

### Finding Duplicate Code

**Task:** "Find duplicate code that should be refactored into shared utilities"

**Why this workflow:** Semantic search finds functionally similar code even when variable names differ.

**Step 1: Find similar database connection patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "database connection creation",
    "path": "/project/src",
    "threshold": 0.65,
    "top_k": 20
  }
}
```

**Step 2: Find configuration loading patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "configuration loading environment variables",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 15
  }
}
```

**Step 3: Find error handling patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling validation",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**Expected outcome:**
- Groups of similar code with semantic scores
- Recommendations for extraction into shared modules
- Estimated refactoring scope (files affected, functions to change)

---

### Technical Debt Identification

**Task:** "Find and categorize technical debt"

**Why this workflow:** Technical debt appears in comments, deprecated code, and hardcoded values.

**Step 1: Find TODO/FIXME comments**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "TODO|FIXME|HACK|XXX|BUG",
    "path": "/project/src",
    "context": 3
  }
}
```

**Step 2: Find deprecated code**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "#\\[deprecated\\]|@deprecated|obsolete|legacy",
    "path": "/project/src",
    "ignore_case": true,
    "context": 3
  }
}
```

**Step 3: Find hardcoded configuration**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "hardcoded values magic numbers",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 15
  }
}
```

**Step 4: Find code complexity indicators**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "complex nested conditions",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**Expected outcome:**
- Categorized technical debt inventory
- Priority ranking based on impact
- Actionable refactoring suggestions

---

### Migration Planning

**Task:** "Plan migration from synchronous to asynchronous code"

**Why this workflow:** Understanding current patterns helps plan migration strategy.

**Step 1: Find blocking I/O operations**
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

**Step 2: Find existing async code patterns**
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

**Step 3: Find thread spawning**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "thread spawn concurrent",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

**Step 4: Find connection pool usage**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "connection pool",
    "path": "/project/src",
    "threshold": 0.75
  }
}
```

**Expected outcome:**
- List of functions to convert (blocking → async)
- Examples of async patterns already in use
- Migration order based on dependencies
- Estimated effort per module

---

## Bug Finding Workflows

### Finding Error Handling Gaps

**Task:** "Find places where errors might not be handled properly"

**Why this workflow:** Unhandled errors are a common source of production bugs.

**Step 1: Find error handling patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling try catch",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 25
  }
}
```

**Step 2: Find panic/unwrap usage (Rust)**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "\\.unwrap\\(\\)|\\.expect\\(|panic!",
    "path": "/project/src",
    "context": 3
  }
}
```

**Step 3: Find uncaught exceptions (JavaScript)**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "catch\\s*\\(|try\\s*\\{",
    "path": "/project/src",
    "context": 5
  }
}
```

**Step 4: Find I/O operations without error handling**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "file system operations without error handling",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**Expected outcome:**
- List of risky error handling locations
- Functions that should return Result/Error types
- Recommendations for defensive programming

---

### Finding Race Conditions

**Task:** "Find potential race conditions in concurrent code"

**Why this workflow:** Race conditions are subtle bugs that require understanding shared state access.

**Step 1: Find concurrent code patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "concurrent programming threads async",
    "path": "/project/src",
    "threshold": 0.75,
    "top_k": 15
  }
}
```

**Step 2: Find shared state access**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "shared mutable state",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 10
  }
}
```

**Step 3: Find synchronization mechanisms**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(Mutex|RwLock|Semaphore|Atomic|lock\\(|acquire)",
    "path": "/project/src",
    "ignore_case": true,
    "context": 5
  }
}
```

**Step 4: Find async state modifications**
```json
{
  "name": "hybrid_search",
  "arguments": {
    "query": "async state modification concurrent access",
    "path": "/project/src",
    "threshold": 0.7
  }
}
```

**Expected outcome:**
- Identification of unsynchronized shared state
- Places where locks might be missing
- Suggestions for synchronization strategies

---

### Finding Resource Leaks

**Task:** "Find potential memory or file handle leaks"

**Why this workflow:** Resource leaks often occur when cleanup code is missing or incorrect.

**Step 1: Find resource allocation**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "file handle connection allocation",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 20
  }
}
```

**Step 2: Find cleanup/close patterns**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(close\\(|drop\\(|defer|finally|__del__|dispose)",
    "path": "/project/src",
    "context": 3
  }
}
```

**Step 3: Check for RAII patterns**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "RAII Drop destructor cleanup",
    "path": "/project/src",
    "threshold": 0.75
  }
}
```

**Expected outcome:**
- Resources that may not be properly cleaned up
- Missing Drop implementations or finally blocks
- Recommendations for RAII patterns

---

## Documentation Generation Workflows

### API Documentation

**Task:** "Generate comprehensive API documentation from code"

**Why this workflow:** Documentation should reflect actual implementation, not assumptions.

**Step 1: Find public API surface**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "pub fn|pub struct|pub enum|export function|export class",
    "path": "/project/src",
    "context": 5
  }
}
```

**Step 2: Find API endpoints**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "API endpoint route handler",
    "path": "/project/src/api",
    "threshold": 0.75,
    "top_k": 30
  }
}
```

**Step 3: Find request/response models**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "request response model schema",
    "path": "/project/src",
    "threshold": 0.8,
    "top_k": 20
  }
}
```

**Step 4: Find authentication requirements**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication required middleware",
    "path": "/project/src",
    "threshold": 0.75
  }
}
```

**Expected outcome:**
- Complete API endpoint listing with HTTP methods
- Request/response schemas
- Authentication requirements per endpoint
- Auto-generated API documentation

---

### Architecture Documentation

**Task:** "Document system architecture from code structure"

**Why this workflow:** Architecture documentation should reflect actual code organization.

**Step 1: Find entry points**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "fn main\\(|#\\[tokio::main\\]|#\\[actix_web::main\\]|if __name__",
    "path": "/project",
    "context": 5
  }
}
```

**Step 2: Map out modules**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "module documentation overview",
    "path": "/project/src",
    "threshold": 0.65,
    "top_k": 25
  }
}
```

**Step 3: Find key abstractions**
```json
{
  "name": "regex_search",
  "arguments": {
    "pattern": "trait \\w+|interface \\w+|abstract class",
    "path": "/project/src",
    "context": 5
  }
}
```

**Step 4: Find data flow**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "data flow pipeline processing",
    "path": "/project/src",
    "threshold": 0.7,
    "top_k": 15
  }
}
```

**Step 5: Find external dependencies**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "external service integration API client",
    "path": "/project/src",
    "threshold": 0.75
  }
}
```

**Expected outcome:**
- System architecture diagram data
- Module dependency graph
- Key abstractions and their purposes
- Integration points with external systems

---

## Best Practices for Agent Workflows

### Search Strategy

**Start broad, then narrow:**
1. Begin with semantic search on large scope (threshold 0.6-0.7)
2. Analyze results to understand code organization
3. Drill down with regex or hybrid search on specific areas
4. Use higher thresholds (0.8+) for precise matching

**Example progression:**
```json
// 1. Broad discovery
{"name": "semantic_search", "arguments": {"query": "authentication", "path": "/project/src", "threshold": 0.6}}

// 2. Focused area
{"name": "semantic_search", "arguments": {"query": "JWT validation", "path": "/project/src/auth", "threshold": 0.75}}

// 3. Specific patterns
{"name": "regex_search", "arguments": {"pattern": "verify_token", "path": "/project/src/auth"}}
```

---

### Threshold Selection

**Choose thresholds based on task:**

- **0.5-0.6**: Exploratory analysis, discovering related concepts
- **0.7-0.8**: Focused investigation, finding specific implementations
- **0.8-0.9**: Precise matching, finding exact patterns
- **0.9+**: Nearly exact semantic matches only

**Adjust dynamically:**
- If too few results (< 3), lower threshold by 0.1
- If too many results (> 50), raise threshold by 0.1
- If results aren't relevant, rephrase query instead

---

### Context Lines Usage

**Use context wisely:**
- **0 lines**: File listing only, minimal data transfer
- **2-3 lines**: Quick pattern verification
- **5-10 lines**: Understanding function structure
- **10+ lines**: Deep code comprehension

**Remember:** Context lines multiply data transfer. Use sparingly for large-scale searches.

```json
// Efficient for large-scale discovery
{"name": "semantic_search", "arguments": {"query": "auth", "path": "/project", "top_k": 50}}

// Then get context for specific files
{"name": "regex_search", "arguments": {"pattern": "verify", "path": "/project/src/auth/jwt.rs", "context": 5}}
```

---

### Combining Search Modes

**Semantic + Regex (most common):**
```json
// 1. Find concept semantically
{"name": "semantic_search", "arguments": {"query": "database connection pooling", "threshold": 0.7}}

// 2. Find all usages with regex
{"name": "regex_search", "arguments": {"pattern": "pool\\.get|getConnection", "context": 2}}
```

**Hybrid search (convenience):**
```json
// Automatically combines both approaches
{"name": "hybrid_search", "arguments": {"query": "connection pool", "threshold": 0.7}}
```

**Regex + Semantic (verification):**
```json
// 1. Find pattern with regex
{"name": "regex_search", "arguments": {"pattern": "\\.unwrap\\(\\)"}}

// 2. Understand context semantically
{"name": "semantic_search", "arguments": {"query": "error handling best practices", "threshold": 0.8}}
```

---

### Pagination Patterns

**Interactive analysis (small pages):**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "user authentication",
    "path": "/project/src",
    "page_size": 5,
    "top_k": 20
  }
}
// Check results, then fetch next page if needed
```

**Comprehensive reports (large pages):**
```json
{
  "name": "semantic_search",
  "arguments": {
    "query": "security issues",
    "path": "/project/src",
    "page_size": 50,
    "top_k": 100
  }
}
// Process all results at once
```

**Iterative refinement:**
```json
// First page to understand results
{"page_size": 10}

// Adjust query based on what you find
{"query": "refined query", "page_size": 10}

// Final comprehensive fetch
{"page_size": 50}
```

---

### Error Recovery

**Handle index status:**
```json
// Always check before large operations
{
  "name": "index_status",
  "arguments": {
    "path": "/project"
  }
}

// If not indexed, trigger reindex
{
  "name": "reindex",
  "arguments": {
    "path": "/project",
    "force": false
  }
}
```

**Handle empty results:**
```javascript
if (results.length === 0) {
  // Try broader query
  threshold -= 0.1;
  // Or try different search mode
  // Or expand search path
}
```

**Handle too many results:**
```javascript
if (results.length > 100) {
  // Narrow the path
  path = "/project/src/specific-module";
  // Or increase threshold
  threshold += 0.1;
  // Or use more specific query
  query = "more specific terms";
}
```

---

## Common Patterns

### Code Exploration Pattern

**Template for understanding unfamiliar code:**

```json
// 1. Get overview (entry points)
{"name": "regex_search", "arguments": {"pattern": "fn main|class Main|export default", "path": "/project"}}

// 2. Find main modules
{"name": "semantic_search", "arguments": {"query": "main components modules", "path": "/project/src", "top_k": 20}}

// 3. Understand specific area
{"name": "semantic_search", "arguments": {"query": "[specific feature]", "path": "/project/src/[module]", "threshold": 0.75}}

// 4. Find implementation details
{"name": "regex_search", "arguments": {"pattern": "[specific pattern]", "path": "/project/src/[module]", "context": 5}}
```

---

### Quality Analysis Pattern

**Template for code quality review:**

```json
// 1. Find quality issues semantically
{"name": "semantic_search", "arguments": {"query": "error handling", "path": "/project/src", "threshold": 0.7}}

// 2. Find anti-patterns with regex
{"name": "regex_search", "arguments": {"pattern": "\\.unwrap\\(\\)|panic!|TODO", "path": "/project/src"}}

// 3. Find best practices
{"name": "semantic_search", "arguments": {"query": "proper error handling Result", "threshold": 0.8}}

// 4. Compare and generate recommendations
```

---

### Refactoring Pattern

**Template for planning refactoring:**

```json
// 1. Find current implementation
{"name": "semantic_search", "arguments": {"query": "[current pattern]", "path": "/project/src", "threshold": 0.7}}

// 2. Find duplicate code
{"name": "semantic_search", "arguments": {"query": "[similar functionality]", "threshold": 0.65, "top_k": 20}}

// 3. Find target pattern examples
{"name": "semantic_search", "arguments": {"query": "[desired pattern]", "threshold": 0.8}}

// 4. List all files needing changes
{"name": "regex_search", "arguments": {"pattern": "[specific code to change]", "path": "/project/src"}}
```

---

### Security Audit Pattern

**Template for security review:**

```json
// 1. Find authentication/authorization
{"name": "semantic_search", "arguments": {"query": "authentication authorization", "threshold": 0.75}}

// 2. Check for common vulnerabilities
{"name": "regex_search", "arguments": {"pattern": "(SELECT|INSERT).*\\+|eval\\(|exec\\(", "context": 3}}

// 3. Find input validation
{"name": "semantic_search", "arguments": {"query": "input validation sanitization", "threshold": 0.75}}

// 4. Check sensitive data handling
{"name": "hybrid_search", "arguments": {"query": "password secret key", "threshold": 0.7}}
```

---

## Next Steps

**Set up the MCP server:**
- [Setup MCP Server](setup-mcp-server.html) - Installation and configuration guide

**Learn the complete API:**
- [MCP API Reference](../reference/mcp-api.html) - Full protocol specification

**Advanced integration:**
- [Integration Guide](../for-humans/integration.html) - Connect with AI tools
- [MCP Quickstart](../for-humans/mcp-quickstart.html) - Get started in 5 minutes
