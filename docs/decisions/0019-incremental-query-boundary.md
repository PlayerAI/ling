# DEC-0019: Incremental query boundary and invalidation policy / 增量查询边界与失效策略

> 状态：Accepted  
> 提出日期：2026-08-21  
> 决定日期：2026-08-21  
> Owner role：compiler-architecture  
> 相关 RFC/缺口：GAP-INCREMENTAL-CACHE-001  
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

The v0.1 execution plan requires incremental source, parse, resolve, and
type/effect queries before LSP integration, but the repository has no accepted
boundary for query inputs, revisions, dependency invalidation, cancellation,
parallel scheduling, persistence, cycles, or debugging. The decision must keep
incremental work observationally equivalent to a clean checked compilation and
must not create a public cache protocol before its schema and migration rules
are accepted.

当前 v0.1 计划要求在接入 LSP 前支持 source、parse、resolve、type/effect 的
增量查询，但仓库尚未接受查询输入、revision、依赖失效、取消、并行调度、
持久化、循环和调试边界。本决定必须保证增量结果与 clean checked 编译在
可观察行为上等价，并且在 schema 与迁移规则被接受前不得形成公共缓存协议。

## Decision

1. The compiler may introduce an internal, in-memory query graph whose nodes
   are immutable values. The initial node families are `source_bytes`,
   `line_index`, `parse`, `ast`, `hir`, `module_graph`, `resolve`,
   `type_effect`, and `checked_snapshot`. A query consumes only declared
   inputs and immutable dependency results; evaluation continues to consume
   checked Typed Core, never unresolved AST or HIR.
2. A source revision is identified by the exact retained UTF-8 byte sequence,
   its canonical logical source name, and the project/package revision that
   selected it. Original bytes and byte spans remain authoritative; line
   indexes and normalized views are derived data and never replace the source
   snapshot. A source edit invalidates that source query and every transitive
   dependent query, while unrelated modules remain reusable.
3. Internal cache keys MUST include the query kind, compiler/language version,
   pinned Unicode version, relevant schema/identity version, logical source or
   package identity, source/project revision, and explicitly selected
   profile/target inputs when those inputs exist. Host paths, allocation
   addresses, hash-map iteration order, wall-clock time, and debug formatting
   MUST NOT affect a key or a checked result.
4. The first implementation uses a deterministic single-threaded scheduler.
   Query dependencies are traversed in canonical order and cycles are rejected
   at the query boundary with bounded internal diagnostics. Parallel scheduling
   is a later optimization and may graduate only after repeated scheduling and
   clean/incremental equivalence evidence.
5. Query cancellation is an internal cooperative control point only. It may
   stop an in-flight request before publishing a new query result, but it does
   not roll back host effects, alter Ling semantics, or reuse the Experimental
   `ling.vm.control/0.1` API. A compiler-facing cancellation protocol requires
   its own accepted decision.
6. Persistent disk caches, serialized query graphs, cache migration, and cache
   corruption recovery are explicitly out of scope for this decision. Until a
   separate accepted cache protocol exists, normal builds remain offline and
   cache misses/failures fall back to clean recomputation without changing
   diagnostics, Semantic IDs, or program output.
7. Query tracing is test-only evidence. It may record logical query names,
   dependency edges, hit/miss outcomes, and revisions, but it MUST omit host
   paths, addresses, allocation layout, hash-map order, and unstable debug
   strings from compatibility claims.
8. The implementation should prefer repository-owned query primitives and the
   locked dependency set. A third-party query engine requires an independent
   license/offline review and may not be exposed as a Ling public API merely by
   being adopted internally.

This decision authorizes the `INC-1401` architecture boundary only. It does
not authorize a new source construct, CLI command, LSP field, JSON schema,
Semantic ID version, cache file, or public protocol.

本决定仅授权 `INC-1401` 的架构边界，不授权新的源码结构、CLI 命令、LSP 字段、
JSON schema、Semantic ID 版本、缓存文件或公共协议。

## Conformance plan

- Build the same checked source graph through clean and incremental paths and
  compare diagnostics, checked signatures, effects/capabilities, semantic
  graph bytes, Semantic IDs, and VM logical outcomes.
- Change one source byte, whitespace/comment region, private body, public
  signature, imported module, and package revision; assert the documented
  invalidation frontier and reuse of unrelated query nodes.
- Exercise Unicode 17.0.0 identifiers, decomposed/NFC source, BOM/CRLF, exact
  UTF-8 byte spans, logical-name changes, and package-aware identity inputs.
- Run canonical-order and repeated-process tests with different filesystem
  enumeration and hash seeds; assert deterministic query traces and results.
- Reject query cycles, oversized revision inputs, malformed source snapshots,
  and cancellation at publication boundaries without publishing partial
  checked results.
- Keep persistence/corruption and parallel-scheduler suites explicitly
  deferred until their own accepted protocol/decision exists.

## Compatibility impact

- **Source and runtime:** none. Existing Ling syntax, Typed Core rules,
  effects, capabilities, interpreter behavior, and VM behavior are unchanged.
- **CLI/LSP:** none. No command, LSP request, position encoding, or editor
  field is added; the query graph is an internal compiler service boundary.
- **Diagnostics:** no new public diagnostic allocation. Future query-cycle or
  cancellation diagnostics must use registered `L-<DOMAIN>-<NUMBER>` codes
  only after their behavior is specified and tested.
- **Schemas, protocols, and Semantic IDs:** none. No cache or query wire
  schema is published; existing Semantic ID domains and canonical bytes remain
  unchanged and are merely checked as equivalence evidence.
- **Determinism and Unicode:** query keys and traces exclude host-specific
  ordering and retain the repository-wide Unicode 17.0.0 and original UTF-8
  span rules.
- **Migration:** none for current users. Persistent cache formats and their
  migrations require a later accepted protocol.

## Unresolved alternatives

- A persistent public cache protocol remains open under
  `GAP-INCREMENTAL-CACHE-001`; DEC-0019 deliberately does not resolve its
  schema, version range, corruption, or migration policy.
- Parallel query execution is deferred until deterministic scheduling and
  clean/incremental equivalence evidence can be independently reproduced.
- A compiler-facing cancellation API, LSP request cancellation, and structured
  Task cancellation remain separate decisions; VM cancellation cannot be
  silently generalized to them.
- A third-party query engine remains possible after license, dependency-lock,
  offline, memory, and trace-determinism review; no dependency is selected by
  this decision.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
