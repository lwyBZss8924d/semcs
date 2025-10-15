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
cs --sem "user authentication" src/

# Find login functions
cs --sem "login process" src/

# Find password handling
cs --sem "password validation" src/
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
cs --sem "authorization check" src/

# Find role-based access
cs --sem "role permission" src/

# Find middleware for auth
cs --sem "authentication middleware" src/
```

---

## Error Handling Patterns

### Find Error Handling Code

```bash
# General error handling
cs --sem "error handling" src/

# Exception handling
cs --sem "exception management" src/

# Error logging
cs --sem "error logging" src/
```

### Find Specific Error Types

```bash
# Network errors
cs --sem "network error handling" src/

# Validation errors
cs --sem "validation error" src/

# Database errors
cs --sem "database error" src/
```

### Find Error Recovery

```bash
# Retry logic
cs --sem "retry mechanism" src/

# Fallback handling
cs --sem "fallback strategy" src/

# Circuit breaker
cs --sem "circuit breaker" src/
```

---

## Configuration Management

### Find Configuration Loading

```bash
# Config file loading
cs --sem "configuration loading" src/

# Environment variables
cs --sem "environment configuration" src/

# Settings management
cs --sem "settings management" src/
```

### Find Configuration Validation

```bash
# Config validation
cs --sem "configuration validation" src/

# Default values
cs --sem "default configuration" src/

# Config schema
cs --sem "configuration schema" src/
```

---

## Database Operations

### Find Database Queries

```bash
# SQL queries
cs --sem "database query" src/

# ORM operations
cs --sem "database operations" src/

# Connection management
cs --sem "database connection" src/
```

### Find Database Migrations

```bash
# Migration scripts
cs --sem "database migration" src/

# Schema changes
cs --sem "schema migration" src/

# Version management
cs --sem "database version" src/
```

---

## API Endpoints

### Find REST Endpoints

```bash
# REST API endpoints
cs --sem "REST endpoint" src/

# HTTP handlers
cs --sem "HTTP handler" src/

# Route definitions
cs --sem "route definition" src/
```

### Find API Documentation

```bash
# API documentation
cs --sem "API documentation" src/

# OpenAPI specs
cs --sem "OpenAPI specification" src/

# Swagger documentation
cs --sem "swagger documentation" src/
```

---

## Testing Patterns

### Find Test Functions

```bash
# Test functions (regex)
cs "fn test_" tests/

# Unit tests
cs --sem "unit test" tests/

# Integration tests
cs --sem "integration test" tests/
```

### Find Test Setup

```bash
# Test setup
cs --sem "test setup" tests/

# Mock objects
cs --sem "mock object" tests/

# Test fixtures
cs --sem "test fixture" tests/
```

---

## Performance Patterns

### Find Caching Logic

```bash
# Caching implementation
cs --sem "cache implementation" src/

# Cache invalidation
cs --sem "cache invalidation" src/

# Memory caching
cs --sem "memory cache" src/
```

### Find Optimization Code

```bash
# Performance optimization
cs --sem "performance optimization" src/

# Lazy loading
cs --sem "lazy loading" src/

# Connection pooling
cs --sem "connection pooling" src/
```

---

## Security Patterns

### Find Security Checks

```bash
# Input validation
cs --sem "input validation" src/

# SQL injection prevention
cs --sem "SQL injection" src/

# XSS prevention
cs --sem "XSS prevention" src/
```

### Find Encryption Code

```bash
# Encryption/decryption
cs --sem "encryption" src/

# Hashing functions
cs --sem "hashing" src/

# Key management
cs --sem "key management" src/
```

---

## Logging Patterns

### Find Logging Code

```bash
# Logging implementation
cs --sem "logging" src/

# Structured logging
cs --sem "structured logging" src/

# Log levels
cs --sem "log level" src/
```

### Find Debug Code

```bash
# Debug statements
cs "console.log" src/
cs "println!" src/
cs "print(" src/

# Debug flags
cs --sem "debug mode" src/
```

---

## Async/Concurrency Patterns

### Find Async Code

```bash
# Async functions
cs --sem "async function" src/

# Promise handling
cs --sem "promise handling" src/

# Future handling
cs --sem "future handling" src/
```

### Find Concurrency Patterns

```bash
# Threading
cs --sem "threading" src/

# Parallel processing
cs --sem "parallel processing" src/

# Lock mechanisms
cs --sem "lock mechanism" src/
```

---

## Advanced Search Strategies

### Combining Semantic and Regex

```bash
# Find async test functions
cs --hybrid "async" --sem "test function" tests/

# Find error handling with specific patterns
cs --hybrid "try" --sem "error handling" src/
```

### Using Thresholds for Precision

```bash
# High precision search
cs --sem "authentication" --threshold 0.8 src/

# Broader search
cs --sem "error" --threshold 0.5 src/
```

### Searching Specific File Types

```bash
# Only Rust files
cs --sem "error handling" --glob "*.rs" src/

# Only test files
cs --sem "test setup" --glob "*test*" .
```

---

## Common Use Cases

### Code Review Checklist

```bash
# Find todos and fixmes
cs "TODO|FIXME|HACK" src/

# Find error handling gaps
cs --sem "error handling" --threshold 0.7 src/

# Find security issues
cs --sem "input validation" src/
```

### Refactoring Preparation

```bash
# Find duplicate patterns
cs --sem "similar implementation" src/

# Find deprecated code
cs "deprecated" src/

# Find hardcoded values
cs --sem "hardcoded configuration" src/
```

### Learning a New Codebase

```bash
# Find main entry points
cs --sem "main entry point" src/

# Find core business logic
cs --sem "business logic" src/

# Find data models
cs --sem "data model" src/
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
