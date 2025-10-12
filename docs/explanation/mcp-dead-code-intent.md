# MCP Dead Code Intent Documentation

This document explains the purpose of code marked with `#[allow(dead_code)]` in the MCP implementation. These are not obsolete remnants but planned infrastructure for future features.

## Session Management (`ck-cli/src/mcp/session.rs`)

### SearchSession Fields

```rust
#[allow(dead_code)]
pub id: Uuid,                    // Session identifier
#[allow(dead_code)]
pub search_options: SearchOptions,  // Original search parameters
#[allow(dead_code)]
pub created_at: SystemTime,         // Creation timestamp
#[allow(dead_code)]
pub search_completed: bool,         // Completion flag
```

**Future Intent:**
- **Session ID**: Will be used for debugging logs and session tracking in production
- **Search Options**: Enables session replay and parameter validation for cursor reuse
- **Created At**: For analytics on session lifetime and usage patterns
- **Search Completed**: Will support incremental/streaming search results

### Session Cleanup

```rust
#[allow(dead_code)]
pub async fn cleanup_expired_sessions(&self) -> usize
```

**Future Intent:**
- Background task to periodically clean expired sessions
- Prevents unbounded memory growth in long-running servers
- Will be called by a tokio interval timer in production

### Session Statistics

```rust
#[allow(dead_code)]
pub async fn get_stats(&self) -> SessionStats

#[allow(dead_code)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub expired_sessions: usize,
    pub total_cached_results: usize,
    pub memory_usage_estimate: usize,
}
```

**Future Intent:**
- Monitoring endpoint for production deployments
- Memory usage tracking and alerting
- Performance metrics collection
- Health check endpoints for orchestration systems

## Implementation Timeline

These features are intentionally left as dead code because:

1. **Core First**: The pagination infrastructure needed to be stable before adding auxiliary features
2. **Production Readiness**: These features become critical only in production deployments
3. **Backward Compatibility**: Adding them later won't break existing clients
4. **Testing Infrastructure**: They enable comprehensive testing without production overhead

## When These Will Be Activated

- **Session cleanup**: When MCP server runs as a long-lived daemon process
- **Statistics**: When monitoring/observability is added to the MCP server
- **Session replay**: When debugging tools are added for MCP interactions
- **Streaming results**: When real-time search capabilities are implemented

## Why Not Remove Them?

1. **Design Documentation**: The fields document the complete intended design
2. **Type Safety**: Ensures future additions don't break existing structure
3. **Testing**: Used in unit tests to verify session behavior
4. **Minimal Cost**: No runtime overhead, only compile-time presence
5. **Future-Proofing**: Avoids breaking changes when features are activated

## Related Files

- `ck-cli/src/mcp/session.rs`: Core session management with dead code
- `ck-cli/src/mcp_server.rs`: Will eventually use session statistics
- Future: `ck-cli/src/mcp/monitor.rs` (not yet created) will use stats
- Future: `ck-cli/src/mcp/cleanup.rs` (not yet created) will run cleanup tasks

## Activation Checklist

When ready to activate these features:

1. Remove `#[allow(dead_code)]` annotations
2. Add background task spawning in `mcp_server.rs`
3. Create `/stats` endpoint for monitoring
4. Add configurable cleanup interval
5. Update documentation with new capabilities