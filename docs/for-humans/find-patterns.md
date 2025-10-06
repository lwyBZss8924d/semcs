---
layout: default
title: Find Patterns
parent: For Humans
nav_order: 4
---

# Find Common Patterns
{: .no_toc }

Ready-to-use searches for common tasks.

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Error handling

```bash
# Find all error handling
ck --sem "error handling" src/

# Find specific types
ck --sem "panic handling" src/
ck --sem "error propagation" src/
ck --sem "error recovery" src/
```

Finds: try/catch, Result types, panic handlers, error propagation, fallbacks

---

## Authentication & Authorization

```bash
# General auth
ck --sem "user authentication" src/

# Specific patterns
ck --sem "login validation" src/
ck --sem "access control" src/
ck --sem "permission checks" src/
ck --sem "token verification" src/
```

Finds: Login flows, JWT validation, permission systems, session management

---

## Database queries

```bash
# All database code
ck --sem "database queries" src/

# Specific operations
ck --sem "insert into database" src/
ck --sem "update records" src/
ck --sem "transaction handling" src/
ck --sem "connection pooling" src/
```

Finds: SQL queries, ORM calls, migrations, connection management

---

## Configuration

```bash
# Config loading
ck --sem "load configuration" src/

# Environment variables
ck --sem "environment variables" src/

# Settings management
ck --sem "application settings" src/
```

Finds: Config parsers, env var loading, settings validation

---

## Async & Concurrency

```bash
# Async operations
ck --sem "async task spawning" src/

# Concurrency patterns
ck --sem "thread management" src/
ck --sem "mutex locks" src/
ck --sem "channel communication" src/
```

Finds: async/await, tokio tasks, threads, mutexes, channels

---

## Caching

```bash
# General caching
ck --sem "cache implementation" src/

# Specific patterns
ck --sem "cache invalidation" src/
ck --sem "memoization" src/
ck --sem "cache warming" src/
```

Finds: Cache layers, memoization, TTL logic, invalidation strategies

---

## HTTP & API

```bash
# HTTP handlers
ck --sem "http request handling" src/

# API endpoints
ck --sem "REST API endpoints" src/
ck --sem "GraphQL resolvers" src/

# HTTP clients
ck --sem "http client requests" src/
```

Finds: Route handlers, middleware, HTTP clients, API logic

---

## Testing

```bash
# Test files
ck --sem "unit tests" tests/
ck --sem "integration tests" tests/

# Test patterns
ck --sem "mock implementations" tests/
ck --sem "test fixtures" tests/

# Or use regex for precision
ck "fn test_\w+" tests/
```

---

## Validation

```bash
# Input validation
ck --sem "input validation" src/
ck --sem "data sanitization" src/

# Type validation
ck --sem "type checking" src/
ck --sem "schema validation" src/
```

Finds: Validators, sanitizers, type guards, schema checks

---

## Logging

```bash
# All logging
ck --sem "logging" src/

# Specific patterns
ck --sem "structured logging" src/
ck --sem "error logging" src/
ck --sem "audit logging" src/
```

Finds: Log statements, loggers, log formatting

---

## Performance

```bash
# Optimization
ck --sem "performance optimization" src/
ck --sem "caching strategies" src/

# Profiling
ck --sem "performance metrics" src/
ck --sem "benchmarking" src/
```

Finds: Optimizations, caches, metrics, benchmarks

---

## Security

```bash
# Security patterns
ck --sem "input sanitization" src/
ck --sem "SQL injection prevention" src/
ck --sem "XSS protection" src/
ck --sem "CSRF protection" src/
```

Finds: Sanitizers, security checks, protection mechanisms

---

## Tips

{: .tip }
**Start broad, then narrow:**
```bash
# Broad search first
ck --sem "authentication" src/

# Then get specific
ck --sem "JWT token validation" src/
```

{: .tip }
**Combine with hybrid for filtering:**
```bash
# Find retry logic mentioning "backoff"
ck --hybrid "backoff" src/
```

{: .tip }
**Use regex for exact matches:**
```bash
# After semantic search, find all uses
ck "authenticate_user" src/
```

---

## Your patterns

Don't see your use case? Try:

1. Describe what you're looking for conceptually
2. Start with `ck --sem "your concept" src/`
3. Adjust threshold if needed: `--threshold 0.6`
4. Combine with hybrid if you know keywords

Examples:
```bash
ck --sem "rate limiting" src/
ck --sem "circuit breaker" src/
ck --sem "event sourcing" src/
ck --sem "dependency injection" src/
```

---

## Next steps

**→** [Search modes explained](search-modes.html) - When to use each mode

**→** [Interactive TUI](tui.html) - Visual exploration

**→** [Large codebases](large-codebases.html) - Performance tips
