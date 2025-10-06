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
ck --sem "user authentication" src/

# Find login functions
ck --sem "login process" src/

# Find password handling
ck --sem "password validation" src/
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
ck --sem "authorization check" src/

# Find role-based access
ck --sem "role permission" src/

# Find middleware for auth
ck --sem "authentication middleware" src/
```

---

## Error Handling Patterns

### Find Error Handling Code

```bash
# General error handling
ck --sem "error handling" src/

# Exception handling
ck --sem "exception management" src/

# Error logging
ck --sem "error logging" src/
```

### Find Specific Error Types

```bash
# Network errors
ck --sem "network error handling" src/

# Validation errors
ck --sem "validation error" src/

# Database errors
ck --sem "database error" src/
```

### Find Error Recovery

```bash
# Retry logic
ck --sem "retry mechanism" src/

# Fallback handling
ck --sem "fallback strategy" src/

# Circuit breaker
ck --sem "circuit breaker" src/
```

---

## Configuration Management

### Find Configuration Loading

```bash
# Config file loading
ck --sem "configuration loading" src/

# Environment variables
ck --sem "environment configuration" src/

# Settings management
ck --sem "settings management" src/
```

### Find Configuration Validation

```bash
# Config validation
ck --sem "configuration validation" src/

# Default values
ck --sem "default configuration" src/

# Config schema
ck --sem "configuration schema" src/
```

---

## Database Operations

### Find Database Queries

```bash
# SQL queries
ck --sem "database query" src/

# ORM operations
ck --sem "database operations" src/

# Connection management
ck --sem "database connection" src/
```

### Find Database Migrations

```bash
# Migration scripts
ck --sem "database migration" src/

# Schema changes
ck --sem "schema migration" src/

# Version management
ck --sem "database version" src/
```

---

## API Endpoints

### Find REST Endpoints

```bash
# REST API endpoints
ck --sem "REST endpoint" src/

# HTTP handlers
ck --sem "HTTP handler" src/

# Route definitions
ck --sem "route definition" src/
```

### Find API Documentation

```bash
# API documentation
ck --sem "API documentation" src/

# OpenAPI specs
ck --sem "OpenAPI specification" src/

# Swagger documentation
ck --sem "swagger documentation" src/
```

---

## Testing Patterns

### Find Test Functions

```bash
# Test functions (regex)
ck "fn test_" tests/

# Unit tests
ck --sem "unit test" tests/

# Integration tests
ck --sem "integration test" tests/
```

### Find Test Setup

```bash
# Test setup
ck --sem "test setup" tests/

# Mock objects
ck --sem "mock object" tests/

# Test fixtures
ck --sem "test fixture" tests/
```

---

## Performance Patterns

### Find Caching Logic

```bash
# Caching implementation
ck --sem "cache implementation" src/

# Cache invalidation
ck --sem "cache invalidation" src/

# Memory caching
ck --sem "memory cache" src/
```

### Find Optimization Code

```bash
# Performance optimization
ck --sem "performance optimization" src/

# Lazy loading
ck --sem "lazy loading" src/

# Connection pooling
ck --sem "connection pooling" src/
```

---

## Security Patterns

### Find Security Checks

```bash
# Input validation
ck --sem "input validation" src/

# SQL injection prevention
ck --sem "SQL injection" src/

# XSS prevention
ck --sem "XSS prevention" src/
```

### Find Encryption Code

```bash
# Encryption/decryption
ck --sem "encryption" src/

# Hashing functions
ck --sem "hashing" src/

# Key management
ck --sem "key management" src/
```

---

## Logging Patterns

### Find Logging Code

```bash
# Logging implementation
ck --sem "logging" src/

# Structured logging
ck --sem "structured logging" src/

# Log levels
ck --sem "log level" src/
```

### Find Debug Code

```bash
# Debug statements
ck "console.log" src/
ck "println!" src/
ck "print(" src/

# Debug flags
ck --sem "debug mode" src/
```

---

## Async/Concurrency Patterns

### Find Async Code

```bash
# Async functions
ck --sem "async function" src/

# Promise handling
ck --sem "promise handling" src/

# Future handling
ck --sem "future handling" src/
```

### Find Concurrency Patterns

```bash
# Threading
ck --sem "threading" src/

# Parallel processing
ck --sem "parallel processing" src/

# Lock mechanisms
ck --sem "lock mechanism" src/
```

---

## Advanced Search Strategies

### Combining Semantic and Regex

```bash
# Find async test functions
ck --hybrid "async" --sem "test function" tests/

# Find error handling with specific patterns
ck --hybrid "try" --sem "error handling" src/
```

### Using Thresholds for Precision

```bash
# High precision search
ck --sem "authentication" --threshold 0.8 src/

# Broader search
ck --sem "error" --threshold 0.5 src/
```

### Searching Specific File Types

```bash
# Only Rust files
ck --sem "error handling" --glob "*.rs" src/

# Only test files
ck --sem "test setup" --glob "*test*" .
```

---

## Common Use Cases

### Code Review Checklist

```bash
# Find todos and fixmes
ck "TODO|FIXME|HACK" src/

# Find error handling gaps
ck --sem "error handling" --threshold 0.7 src/

# Find security issues
ck --sem "input validation" src/
```

### Refactoring Preparation

```bash
# Find duplicate patterns
ck --sem "similar implementation" src/

# Find deprecated code
ck "deprecated" src/

# Find hardcoded values
ck --sem "hardcoded configuration" src/
```

### Learning a New Codebase

```bash
# Find main entry points
ck --sem "main entry point" src/

# Find core business logic
ck --sem "business logic" src/

# Find data models
ck --sem "data model" src/
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
