# DEC-0021: Deterministic parallel internal query scheduling

> 状态：Accepted  
> 提出日期：2026-08-21  
> 决定日期：2026-08-21  
> Owner role：compiler-architecture  
> 相关 RFC/缺口：`DEC-0019`, `GAP-INCREMENTAL-CACHE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

DEC-0019 establishes a deterministic single-threaded query boundary and
defers parallel scheduling until clean/incremental equivalence and repeated
scheduling evidence exist. The compiler now has that equivalence evidence, so
the remaining question is how to parallelize independent internal work without
making worker timing, host parallelism, or map order observable.

## Decision

1. The internal query implementation may execute independent, pure query jobs
   over immutable source snapshots in bounded worker scopes. A job may not read
   host paths, ambient process state, mutable compiler caches, or unchecked AST
   values, and it may not perform host effects.
2. Ready jobs are identified by canonical logical task order. A scheduling
   seed may change worker assignment and completion order for stress evidence,
   but it never changes the dependency graph, result order, cache key, trace
   order, or selected diagnostic.
3. Workers return immutable results only. The owning database publishes cache
   entries and query trace events serially in canonical order after all workers
   join. A worker failure prevents publication of the affected batch; bounded
   diagnostics are selected by canonical task order rather than completion
   timing.
4. `ling-db` adopts this boundary first for independent source parse misses in
   `CompilerDb::parse_all`. Dependent workspace traversal, type/effect
   checking, semantic snapshot construction, persistent caches, and corruption
   recovery remain under their existing deterministic boundaries until separate
   evidence or an accepted decision expands the scope.
5. This decision is internal implementation authority only. It adds no source
   syntax, language semantics, CLI/LSP field, diagnostic allocation, Semantic
   ID rule, JSON schema, cache file, or public protocol.

## Conformance plan

- Run multiple scheduling seeds over the same multi-file source set, including
  Unicode names and malformed syntax, and compare canonical logical names,
  parsed trees/errors, cache publication, and query traces byte-for-byte or by
  their existing structural equality contracts.
- Verify canonical `parse_all` output remains stable when source insertion order
  differs, and repeated calls reuse immutable cached results without spawning
  work for hits.
- Combine the parallel source-query path with the INC-1407 clean/incremental
  equivalence sequence and verify diagnostics, checked projections, semantic
  JSON, formatter output, and interpreter results remain equal.
- Keep invalid UTF-8, worker panic containment, oversized source limits, query
  cycles, and persistence/corruption behavior bounded and outside publication;
  future workspace-wide parallelism requires additional evidence.

## Compatibility impact

- Source, Typed Core, effects, interpreter, VM, CLI, LSP, diagnostics, schemas,
  Semantic IDs, Unicode 17.0.0 behavior, and original UTF-8 spans: none.
- Internal execution may use more than one worker, but canonical publication
  prevents host CPU count, timing, allocation order, and map iteration from
  changing observable results.
- No migration or persistent data format is introduced; normal builds remain
  offline and locked.

## Unresolved alternatives

- Parallelizing dependent HIR, resolve, type/effect, and semantic queries needs
  a separate benchmark and invalidation review; it is not implied by this
  source-parse slice.
- Persistent query serialization, version migration, and corruption recovery
  remain open under `GAP-INCREMENTAL-CACHE-001` and require a later accepted
  cache protocol.
- A third-party scheduler remains rejected for this slice pending independent
  dependency, license, offline, and trace-determinism review.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
