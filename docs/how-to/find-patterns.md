---
layout: default
title: Find Specific Patterns
parent: How-To Guides
nav_order: 3
---

# Find Specific Patterns

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

**Goal:** Learn ready-to-use searches for common code patterns and use cases.

**You'll learn:**
- Authentication patterns
- Error handling approaches
- Configuration management
- Database operations
- API endpoints
- Testing patterns

---

## Authentication Patterns

### Find Login/Authentication Logic

```bash
# Semantic search for authentication
cc --sem "user authentication" src/

# Find login functions
cc --sem "login process" src/

# Find password handling
cc --sem "password validation" src/
```

**What you'll find:**
- Login functions and methods
- Password hashing/validation
- Session management
- JWT token handling
- OAuth implementations

### Find Authorization Checks

```bash
# Find permission checks
cc --sem "authorization check" src/

# Find role-based access
cc --sem "role permission" src/

# Find middleware for auth
cc --sem "authentication middleware" src/
```

---

## Error Handling Patterns

### Find Error Handling Code

```bash
# General error handling
cc --sem "error handling" src/

# Exception handling
cc --sem "exception management" src/

# Error logging
cc --sem "error logging" src/
```

### Find Specific Error Types

```bash
# Network errors
cc --sem "network error handling" src/

# Validation errors
cc --sem "validation error" src/

# Database errors
cc --sem "database error" src/
```

### Find Error Recovery

```bash
# Retry logic
cc --sem "retry mechanism" src/

# Fallback handling
cc --sem "fallback strategy" src/

# Circuit breaker
cc --sem "circuit breaker" src/
```

---

## Configuration Management

### Find Configuration Loading

```bash
# Config file loading
cc --sem "configuration loading" src/

# Environment variables
cc --sem "environment configuration" src/

# Settings management
cc --sem "settings management" src/
```

### Find Configuration Validation

```bash
# Config validation
cc --sem "configuration validation" src/

# Default values
cc --sem "default configuration" src/

# Config schema
cc --sem "configuration schema" src/
```

---

## Database Operations

### Find Database Queries

```bash
# SQL queries
cc --sem "database query" src/

# ORM operations
cc --sem "database operations" src/

# Connection management
cc --sem "database connection" src/
```

### Find Database Migrations

```bash
# Migration scripts
cc --sem "database migration" src/

# Schema changes
cc --sem "schema migration" src/

# Version management
cc --sem "database version" src/
```

---

## API Endpoints

### Find REST Endpoints

```bash
# REST API endpoints
cc --sem "REST endpoint" src/

# HTTP handlers
cc --sem "HTTP handler" src/

# Route definitions
cc --sem "route definition" src/
```

### Find API Documentation

```bash
# API documentation
cc --sem "API documentation" src/

# OpenAPI specs
cc --sem "OpenAPI specification" src/

# Swagger documentation
cc --sem "swagger documentation" src/
```

---

## Testing Patterns

### Find Test Functions

```bash
# Test functions (regex)
cc "fn test_" tests/

# Unit tests
cc --sem "unit test" tests/

# Integration tests
cc --sem "integration test" tests/
```

### Find Test Setup

```bash
# Test setup
cc --sem "test setup" tests/

# Mock objects
cc --sem "mock object" tests/

# Test fixtures
cc --sem "test fixture" tests/
```

---

## Performance Patterns

### Find Caching Logic

```bash
# Caching implementation
cc --sem "cache implementation" src/

# Cache invalidation
cc --sem "cache invalidation" src/

# Memory caching
cc --sem "memory cache" src/
```

### Find Optimization Code

```bash
# Performance optimization
cc --sem "performance optimization" src/

# Lazy loading
cc --sem "lazy loading" src/

# Connection pooling
cc --sem "connection pooling" src/
```

---

## Security Patterns

### Find Security Checks

```bash
# Input validation
cc --sem "input validation" src/

# SQL injection prevention
cc --sem "SQL injection" src/

# XSS prevention
cc --sem "XSS prevention" src/
```

### Find Encryption Code

```bash
# Encryption/decryption
cc --sem "encryption" src/

# Hashing functions
cc --sem "hashing" src/

# Key management
cc --sem "key management" src/
```

---

## Logging Patterns

### Find Logging Code

```bash
# Logging implementation
cc --sem "logging" src/

# Structured logging
cc --sem "structured logging" src/

# Log levels
cc --sem "log level" src/
```

### Find Debug Code

```bash
# Debug statements
cc "console.log" src/
cc "println!" src/
cc "print(" src/

# Debug flags
cc --sem "debug mode" src/
```

---

## Async/Concurrency Patterns

### Find Async Code

```bash
# Async functions
cc --sem "async function" src/

# Promise handling
cc --sem "promise handling" src/

# Future handling
cc --sem "future handling" src/
```

### Find Concurrency Patterns

```bash
# Threading
cc --sem "threading" src/

# Parallel processing
cc --sem "parallel processing" src/

# Lock mechanisms
cc --sem "lock mechanism" src/
```

---

## Advanced Search Strategies

### Combining Semantic and Regex

```bash
# Find async test functions
cc --hybrid "async" --sem "test function" tests/

# Find error handling with specific patterns
cc --hybrid "try" --sem "error handling" src/
```

### Using Thresholds for Precision

```bash
# High precision search
cc --sem "authentication" --threshold 0.8 src/

# Broader search
cc --sem "error" --threshold 0.5 src/
```

### Searching Specific File Types

```bash
# Only Rust files
cc --sem "error handling" --glob "*.rs" src/

# Only test files
cc --sem "test setup" --glob "*test*" .
```

---

## Common Use Cases

### Code Review Checklist

```bash
# Find todos and fixmes
cc "TODO|FIXME|HACK" src/

# Find error handling gaps
cc --sem "error handling" --threshold 0.7 src/

# Find security issues
cc --sem "input validation" src/
```

### Refactoring Preparation

```bash
# Find duplicate patterns
cc --sem "similar implementation" src/

# Find deprecated code
cc "deprecated" src/

# Find hardcoded values
cc --sem "hardcoded configuration" src/
```

### Learning a New Codebase

```bash
# Find main entry points
cc --sem "main entry point" src/

# Find core business logic
cc --sem "business logic" src/

# Find data models
cc --sem "data model" src/
```

---

## Tips for Effective Pattern Search

{: .tip }
**💡 Start broad, then narrow:**
1. Use semantic search for concepts
2. Add keywords with hybrid search
3. Use regex for exact patterns

{: .tip }
**💡 Use descriptive queries:**
- "user authentication" instead of "auth"
- "error handling" instead of "error"
- "database connection" instead of "db"

{: .tip }
**💡 Combine search modes:**
- Semantic for discovery
- Hybrid for precision
- Regex for exact matches

{: .tip }
**💡 Adjust thresholds:**
- 0.8+ for high precision
- 0.6-0.8 for balanced results
- 0.5- for broad discovery

---

## Next Steps

**→ Learn advanced search:** [Search Modes Explained](../explanation/search-modes.html)

**→ Optimize performance:** [Large Codebases](large-codebases.html)

**→ Integrate with editors:** [Editor Integration](editor-integration.html)

**→ Connect AI tools:** [AI Integration](../ai-integration/mcp-quickstart.html)
