# DEC-0022: Disposable persistent query-cache envelope

> 状态：Accepted
> 提出日期：2026-08-21
> 决定日期：2026-08-21
> Owner role：compiler-architecture
> 相关 RFC/缺口：`DEC-0012`, `DEC-0019`, `DEC-0021`, `GAP-INCREMENTAL-CACHE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

`DEC-0019` and `DEC-0021` define deterministic in-memory query boundaries,
while `GAP-INCREMENTAL-CACHE-001` leaves persistent keys, corruption recovery,
and compatibility unresolved. INC-1409 needs a bounded persistent slice without
serializing unchecked compiler state or creating a public cache protocol.

## Decision

1. Ling may use a persistent cache only when a caller explicitly supplies a
   disposable root. The cache is internal implementation state; it is not a
   source-language feature, CLI/LSP field, stable schema, or public protocol.
2. A cache key contains the compiler/toolchain version, language version,
   Unicode version, query-schema version, profile and target dimensions,
   canonical digests of package/config/profile/target workspace inputs, query
   kind, canonical logical source name, and the exact source-byte digest. Key
   construction is length-delimited and deterministic; host paths, addresses,
   allocation order, and map order are never semantic inputs.
3. The v1 envelope uses a fixed magic and version, bounded key and payload
   lengths, the canonical key bytes, and a BLAKE3 checksum over the envelope
   header, key, and payload. Publication uses a create-new temporary file,
   complete write and sync, then rename. An already published entry is kept;
   equal keys therefore remain immutable and deterministic.
4. Missing, unreadable, truncated, malformed, unknown-version, checksum-
   mismatched, foreign-key, or over-limit entries are safe cache misses. Cache
   writes are best effort and may not change query behavior. No unchecked AST,
   HIR, Typed Core, bytecode, or diagnostic object is deserialized from disk.
5. INC-1409 persists only the derived `ling-db` line-index payload. The reader
   validates its version, length, ordering, lexical length, and line-start
   bounds against the current `SourceFile`, then reconstructs a fresh checked
   `LineIndex` with the current source identity. Any rejection recomputes the
   result through the existing source boundary.
6. Cross-version migrations, persistent dependent-query graphs, eviction and
   locking policy, cache sharing between projects, and serialization of
   compiler IR require a later accepted decision. This slice does not close
   those broader gaps.

## Conformance plan

- Round-trip a valid line-index entry and compare it with a clean in-memory
  computation, including CRLF, BOM, and Unicode source boundaries.
- Change source bytes, profile, or target inputs and require a miss followed by
  the same result as a clean database; verify source IDs are reconstructed from
  the current snapshot rather than read from the payload.
- Flip checksum bytes and exercise missing, truncated, foreign-key,
  over-limit, and unknown-version entries; every case must be a bounded miss
  with no partial publication or panic.
- Repeat publication and reads across process-like database lifetimes and
  compare cache IDs, payloads, query traces, and clean/incremental behavior
  without depending on host paths or enumeration order.
- Reject future envelope versions rather than guessing a migration; a future
  migration must be separately specified and evidenced.

## Compatibility impact

- Source syntax, Typed Core semantics, effects, interpreter and VM behavior,
  diagnostics, Semantic IDs, JSON schemas, CLI/LSP fields, and Unicode
  17.0.0 behavior: unchanged.
- Adds only the private `ling-cache` crate and an explicitly opt-in disposable
  on-disk artifact. Cache absence, corruption, or write failure is equivalent
  to a normal cache miss.
- The envelope is versioned for safe rejection, not a compatibility guarantee;
  normal builds and tests remain locked and offline.

## Unresolved alternatives

- Persistent serialization for parse, HIR, resolve, type/effect, semantic, or
  bytecode results remains deferred until checked-value schemas and migrations
  are accepted.
- Multi-process locking, eviction, compression, encryption, and shared cache
  roots remain out of scope for this disposable slice.
- A public cache directory layout or command-line cache management interface
  requires separate protocol governance.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
