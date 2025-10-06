---
layout: default
title: Examples
parent: AI Integration
nav_order: 3
---

# Agent Examples

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** See real-world examples of how AI agents use ck for code search and analysis.

**You'll learn:**
- Code exploration workflows
- Refactoring assistance patterns
- Code review automation
- Bug finding strategies
- Documentation generation

---

## Code Exploration

### Understanding Authentication Flow

**Agent task:** "Help me understand how authentication works in this codebase"

**Workflow:**
```json
// 1. Broad semantic search for authentication
{
  "name": "semantic_search",
  "arguments": {
    "query": "user authentication",
    "path": "./src",
    "top_k": 10,
    "threshold": 0.7
  }
}

// 2. Find specific authentication methods
{
  "name": "semantic_search",
  "arguments": {
    "query": "JWT token validation",
    "path": "./src/auth",
    "top_k": 5
  }
}

// 3. Find login/logout flows
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(login|logout|signin|signout)",
    "path": "./src",
    "ignore_case": true
  }
}
```

**Agent response:**
```
I found the authentication system in your codebase. Here's how it works:

1. **JWT Token Validation** (src/auth/jwt.rs:45-67, score: 0.92)
   - Validates JWT tokens using HMAC-SHA256
   - Extracts user claims from token payload
   - Handles token expiration and refresh

2. **Login Flow** (src/auth/login.rs:12-34, score: 0.88)
   - Validates user credentials against database
   - Generates JWT tokens on successful login
   - Stores refresh tokens for session management

3. **Middleware Integration** (src/middleware/auth.rs:23-45, score: 0.85)
   - Protects routes requiring authentication
   - Extracts user context from JWT tokens
   - Handles authentication errors

The system uses a standard JWT-based approach with refresh tokens for security.
```

### Discovering API Patterns

**Agent task:** "Show me all the REST API endpoints in this project"

**Workflow:**
```json
// 1. Find API route definitions
{
  "name": "semantic_search",
  "arguments": {
    "query": "REST API endpoints",
    "path": "./src",
    "top_k": 20
  }
}

// 2. Find HTTP handlers
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(GET|POST|PUT|DELETE|PATCH)\\s+/",
    "path": "./src",
    "context": 2
  }
}

// 3. Find route registration
{
  "name": "semantic_search",
  "arguments": {
    "query": "route registration",
    "path": "./src",
    "threshold": 0.8
  }
}
```

---

## Refactoring Assistance

### Finding Duplicate Code

**Agent task:** "Find potential code duplication for refactoring"

**Workflow:**
```json
// 1. Find similar implementations
{
  "name": "semantic_search",
  "arguments": {
    "query": "database connection",
    "path": "./src",
    "top_k": 15,
    "threshold": 0.6
  }
}

// 2. Find configuration loading patterns
{
  "name": "semantic_search",
  "arguments": {
    "query": "configuration loading",
    "path": "./src",
    "top_k": 10
  }
}

// 3. Find error handling patterns
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "./src",
    "top_k": 20,
    "threshold": 0.7
  }
}
```

**Agent analysis:**
```
I found several areas with potential duplication:

1. **Database Connections** (3 similar implementations)
   - src/db/connection.rs:45-67 (score: 0.91)
   - src/models/user.rs:23-45 (score: 0.89)
   - src/services/auth.rs:12-34 (score: 0.87)
   
   Recommendation: Extract to a shared database connection pool.

2. **Configuration Loading** (4 similar patterns)
   - All use similar env var parsing
   - Consider creating a unified config module

3. **Error Handling** (5 different approaches)
   - Mix of Result<T, E> and panic! usage
   - Standardize on Result types for better error handling
```

### Identifying Technical Debt

**Agent task:** "Find technical debt and areas for improvement"

**Workflow:**
```json
// 1. Find TODO comments
{
  "name": "regex_search",
  "arguments": {
    "pattern": "TODO|FIXME|HACK|XXX",
    "path": "./src",
    "context": 3
  }
}

// 2. Find deprecated code
{
  "name": "regex_search",
  "arguments": {
    "pattern": "deprecated|obsolete|legacy",
    "path": "./src",
    "ignore_case": true
  }
}

// 3. Find hardcoded values
{
  "name": "semantic_search",
  "arguments": {
    "query": "hardcoded configuration",
    "path": "./src",
    "threshold": 0.7
  }
}
```

---

## Code Review Automation

### Security Audit

**Agent task:** "Perform a security audit of the codebase"

**Workflow:**
```json
// 1. Find input validation
{
  "name": "semantic_search",
  "arguments": {
    "query": "input validation",
    "path": "./src",
    "top_k": 15
  }
}

// 2. Find SQL injection risks
{
  "name": "regex_search",
  "arguments": {
    "pattern": "SELECT.*\\+.*FROM|INSERT.*\\+.*INTO",
    "path": "./src"
  }
}

// 3. Find authentication bypasses
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication bypass",
    "path": "./src",
    "threshold": 0.8
  }
}

// 4. Find sensitive data handling
{
  "name": "semantic_search",
  "arguments": {
    "query": "password handling",
    "path": "./src"
  }
}
```

**Agent report:**
```
Security Audit Results:

🔴 **High Risk Issues:**
1. **SQL Injection Risk** (src/db/queries.rs:45)
   - Raw SQL concatenation found
   - Recommendation: Use parameterized queries

2. **Hardcoded Secrets** (src/config.rs:23)
   - API keys in source code
   - Recommendation: Move to environment variables

🟡 **Medium Risk Issues:**
1. **Missing Input Validation** (src/api/users.rs:67)
   - User input not validated
   - Recommendation: Add validation middleware

2. **Weak Error Handling** (src/auth/login.rs:34)
   - Sensitive error messages exposed
   - Recommendation: Sanitize error responses
```

### Performance Analysis

**Agent task:** "Analyze code for performance issues"

**Workflow:**
```json
// 1. Find database queries
{
  "name": "semantic_search",
  "arguments": {
    "query": "database query",
    "path": "./src",
    "top_k": 20
  }
}

// 2. Find N+1 query patterns
{
  "name": "regex_search",
  "arguments": {
    "pattern": "for.*in.*\\{.*query",
    "path": "./src"
  }
}

// 3. Find inefficient algorithms
{
  "name": "semantic_search",
  "arguments": {
    "query": "algorithm complexity",
    "path": "./src",
    "threshold": 0.7
  }
}
```

---

## Bug Finding

### Finding Error Handling Gaps

**Agent task:** "Find places where error handling might be missing"

**Workflow:**
```json
// 1. Find error handling patterns
{
  "name": "semantic_search",
  "arguments": {
    "query": "error handling",
    "path": "./src",
    "top_k": 25
  }
}

// 2. Find unwrap() usage (Rust)
{
  "name": "regex_search",
  "arguments": {
    "pattern": "\\.unwrap\\(\\)|\\.expect\\(",
    "path": "./src"
  }
}

// 3. Find try-catch blocks (JavaScript)
{
  "name": "regex_search",
  "arguments": {
    "pattern": "try\\s*\\{",
    "path": "./src"
  }
}

// 4. Find functions without error handling
{
  "name": "semantic_search",
  "arguments": {
    "query": "functions without error handling",
    "path": "./src",
    "threshold": 0.6
  }
}
```

### Finding Race Conditions

**Agent task:** "Find potential race conditions in concurrent code"

**Workflow:**
```json
// 1. Find concurrent code
{
  "name": "semantic_search",
  "arguments": {
    "query": "concurrent programming",
    "path": "./src",
    "top_k": 15
  }
}

// 2. Find shared state access
{
  "name": "semantic_search",
  "arguments": {
    "query": "shared state access",
    "path": "./src",
    "threshold": 0.7
  }
}

// 3. Find locking mechanisms
{
  "name": "regex_search",
  "arguments": {
    "pattern": "(lock|mutex|semaphore|atomic)",
    "path": "./src",
    "ignore_case": true
  }
}
```

---

## Documentation Generation

### API Documentation

**Agent task:** "Generate API documentation from code"

**Workflow:**
```json
// 1. Find API endpoints
{
  "name": "semantic_search",
  "arguments": {
    "query": "API endpoint",
    "path": "./src",
    "top_k": 30
  }
}

// 2. Find request/response models
{
  "name": "semantic_search",
  "arguments": {
    "query": "request response model",
    "path": "./src"
  }
}

// 3. Find authentication requirements
{
  "name": "semantic_search",
  "arguments": {
    "query": "authentication required",
    "path": "./src"
  }
}
```

**Generated documentation:**
```markdown
# API Documentation

## Authentication Endpoints

### POST /auth/login
- **Description:** Authenticate user and return JWT token
- **Request:** `{ "email": "string", "password": "string" }`
- **Response:** `{ "token": "string", "user": {...} }`
- **Auth:** None required

### POST /auth/logout
- **Description:** Invalidate user session
- **Request:** `{ "token": "string" }`
- **Response:** `{ "success": true }`
- **Auth:** JWT token required
```

### Architecture Documentation

**Agent task:** "Document the system architecture"

**Workflow:**
```json
// 1. Find main components
{
  "name": "semantic_search",
  "arguments": {
    "query": "main application components",
    "path": "./src",
    "top_k": 20
  }
}

// 2. Find data flow
{
  "name": "semantic_search",
  "arguments": {
    "query": "data flow",
    "path": "./src"
  }
}

// 3. Find external dependencies
{
  "name": "semantic_search",
  "arguments": {
    "query": "external dependencies",
    "path": "./src"
  }
}
```

---

## Advanced Workflows

### Multi-step Analysis

**Agent task:** "Analyze the entire codebase for maintainability"

**Workflow:**
```json
// Step 1: Get overview
{
  "name": "index_status",
  "arguments": {
    "path": "./"
  }
}

// Step 2: Find main entry points
{
  "name": "semantic_search",
  "arguments": {
    "query": "main entry point",
    "path": "./src",
    "top_k": 5
  }
}

// Step 3: Analyze each major component
{
  "name": "semantic_search",
  "arguments": {
    "query": "business logic",
    "path": "./src",
    "top_k": 10
  }
}

// Step 4: Find integration points
{
  "name": "semantic_search",
  "arguments": {
    "query": "external integration",
    "path": "./src"
  }
}
```

### Continuous Monitoring

**Agent task:** "Set up continuous code quality monitoring"

**Workflow:**
```json
// Daily checks
{
  "name": "regex_search",
  "arguments": {
    "pattern": "TODO|FIXME",
    "path": "./src"
  }
}

// Weekly analysis
{
  "name": "semantic_search",
  "arguments": {
    "query": "code quality issues",
    "path": "./src",
    "threshold": 0.7
  }
}

// Monthly review
{
  "name": "semantic_search",
  "arguments": {
    "query": "technical debt",
    "path": "./src"
  }
}
```

---

## Best Practices

### Effective Agent Workflows

**Start broad, then narrow:**
1. Use semantic search for concept discovery
2. Use regex search for specific patterns
3. Combine results for comprehensive analysis

**Use appropriate thresholds:**
- 0.5-0.6: Exploratory analysis
- 0.7-0.8: Focused investigation
- 0.8+: Specific issue identification

**Leverage pagination:**
- Use small page sizes for interactive analysis
- Use larger page sizes for comprehensive reports
- Check `has_next` for complete coverage

### Common Patterns

**Code exploration:**
```json
// 1. Broad search
{"name": "semantic_search", "arguments": {"query": "concept", "path": "./"}}

// 2. Specific area
{"name": "semantic_search", "arguments": {"query": "specific", "path": "./src/area"}}

// 3. Pattern matching
{"name": "regex_search", "arguments": {"pattern": "pattern", "path": "./src"}}
```

**Quality analysis:**
```json
// 1. Find issues
{"name": "semantic_search", "arguments": {"query": "issue type", "path": "./src"}}

// 2. Verify patterns
{"name": "regex_search", "arguments": {"pattern": "specific", "path": "./src"}}

// 3. Generate report
// Combine and analyze results
```

---

## Next Steps

**→** [MCP API Reference](../reference/mcp-api.html) - Complete API documentation

**→** [Setup Guides](setup-guides.html) - Integration with specific tools

**→** [MCP Quick Start](mcp-quickstart.html) - Getting started guide
