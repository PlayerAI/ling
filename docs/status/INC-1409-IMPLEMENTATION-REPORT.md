# INC-1409 Implementation Report: Disposable Persistent Query Cache

## Outcome

INC-1409 is complete for the bounded slice authorized by Accepted `DEC-0022`.
Ling now has an opt-in, disposable persistent cache envelope and uses it only
for the derived `ling-db` line-index query. Cache data is versioned, key-bound,
checksummed, atomically published, and treated as a safe miss whenever it is
missing, unreadable, incompatible, corrupt, or outside the limits.

## Normative traceability

- Accepted `DEC-0012` supplies the deterministic BLAKE3 identity and canonical
  byte boundary used for cache keys; no cache path becomes a Semantic ID.
- Accepted `DEC-0019` fixes the source-revision and query invalidation boundary.
- Accepted `DEC-0021` keeps persistent work outside the parallel scheduling
  slice and requires separate cache authority.
- Accepted `DEC-0022` authorizes the bounded envelope, key dimensions,
  corruption-safe fallback, checked line-index reconstruction, and explicit
  deferral of persistent dependent-query graphs and migrations.
- `ling-source` remains authoritative for immutable UTF-8 snapshots, lexical
  normalization, original spans, and Unicode 17.0.0 behavior.

## Implemented boundary

- New internal `ling-cache` provides a length-delimited v1 envelope containing
  compiler/toolchain, language, Unicode, schema, profile, target, query,
  logical-name, source-byte, and workspace-input dimensions through
  `CacheKey`.
- The envelope enforces bounded key/payload sizes, a fixed magic/version, a
  BLAKE3 checksum, and create-new temporary write → sync → rename publication.
  Equal keys retain the first complete entry; concurrent or failed writes do
  not change query behavior.
- `CompilerDb::with_persistent_cache` is explicit opt-in. `line_index` first
  checks the existing in-memory query, then validates a persistent line-index
  payload against the current `SourceFile`, otherwise recomputes and publishes
  the derived result. The payload never contains a source identity or unchecked
  compiler value.
- Cache reads are deliberately best-effort. Unknown versions, malformed
  lengths, key mismatches, checksum failures, truncated files, and invalid line
  starts all fall back to the existing deterministic computation.

## Evidence

- `ling-cache` tests cover round-trip/foreign-key rejection, checksum
  corruption, version dimensions, and profile/target key separation.
- `ling-db` tests cover reuse across database lifetimes, source-byte
  invalidation, corruption-safe misses, profile/target invalidation, BOM/CRLF
  and Unicode line boundaries, and current-source identity reconstruction.
- `cargo test -p ling-cache --all-targets --locked --offline` and
  `cargo test -p ling-db --all-targets --locked --offline` passed.
- `cargo clippy -p ling-cache -p ling-db --all-targets --locked --offline -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` passed for the
  implementation slice; repository-wide gates are recorded after integration.

## Compatibility and deferred work

- Source syntax, Typed Core semantics, effects, interpreter/VM behavior,
  diagnostics, Semantic IDs, JSON schemas, CLI/LSP fields, and Unicode tables
  are unchanged. Normal cache absence, corruption, and write failure are
  behaviorally identical to a cache miss.
- The on-disk artifact is private and disposable, not a stable public protocol
  or cross-release migration promise. Persistent parse/HIR/resolve/type/effect/
  semantic/bytecode graphs, migration rules, eviction, locking, compression,
  encryption, and shared roots remain open under `GAP-INCREMENTAL-CACHE-001`.

## Validation and next target

Governance and status records include Accepted `DEC-0022`, generated authority,
lifecycle, and gap reports, and the executable cache evidence. INC-1410
incremental performance baseline is the next execution-plan target.
