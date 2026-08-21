# INC-1403 Implementation Report: Parse Queries

## Outcome

INC-1403 is complete. The new internal `ling-db` crate provides deterministic
in-memory query entry points for source bytes, normalized line indexes, lexer
tokens, parse trees, and AST lowering. Query values are immutable `Arc`-backed
results keyed by VFS revisions and selected workspace inputs; no host
filesystem, persistent cache, or public query protocol is involved.

## Normative traceability

- Accepted `DEC-0019` Decision §§1–3 authorize immutable query families,
  exact retained UTF-8 source bytes, canonical logical names, source/project
  revisions, and versioned deterministic cache inputs.
- Accepted `DEC-0019` Decision §4 requires the first scheduler to be
  single-threaded and canonical; `CompilerDb::parse_all` uses VFS snapshots in
  canonical logical-name order and `BTreeMap` caches.
- Accepted `DEC-0019` Decision §§6–8 keep persistence, cache migration,
  corruption protocols, parallel scheduling, and third-party query engines
  out of scope; query tracing remains non-compatibility test evidence.
- Existing `ling-source`, `ling-syntax`, and `ling-ast` contracts remain the
  authority for original UTF-8 byte spans, Unicode 17.0.0 identifiers,
  diagnostics, error recovery, and valid-CST AST lowering.

## Implemented boundary

- `CompilerDb` wraps the INC-1402 `VirtualFileSystem` and exposes exact
  `source_bytes`, normalized `line_index`, `tokens`, `parse`, and `ast`
  queries without reading ambient paths or environment state.
- Cache keys carry the repository compiler version, language version,
  query-schema version, pinned Unicode version, canonical logical name, source
  revision, visible layer, and selected package/config/profile/target
  revisions. Old values are immutable and remain process-local only.
- `QueryEvent` records only query kind, source ID, revision, and hit/miss
  outcome. It contains no host path, address, allocation layout, hash-map
  order, wall clock, or unstable debug formatting.
- `parse_all` traverses snapshots in canonical logical-name order. A source
  revision edit creates a new cache key for that file while unrelated source
  keys remain reusable; clean and incremental parse results are compared in
  the test corpus.
- Invalid UTF-8 is surfaced as a bounded `QueryError` before tokens, parse, or
  AST publication. Valid syntax errors remain inside `ParsedSource`, and AST
  lowering delegates to `ling_ast::lower`, which rejects invalid CSTs.

## Evidence

- Seven unit tests cover immutable hit reuse, changed-only invalidation,
  clean/incremental equivalence, workspace revision identity, tokens and AST,
  malformed UTF-8 publication boundaries, BOM/CRLF/Chinese line indexes, and
  canonical multi-file traversal.
- The workspace lockfile records only the new repository-owned crate; no new
  third-party dependency was added.
- The execution backlog and machine status registry mark INC-1403 Done with
  this report and the implementation commit.

## Compatibility and deferred work

- No language syntax or semantics, diagnostic allocation, schema, Semantic ID,
  canonical bytes, CLI/LSP field, or public protocol changed.
- Type/effect queries, checked snapshots, persistent cache serialization/migration,
  cycle diagnostics, parallel scheduling, compiler cancellation, and LSP adapters
  remain later targets. INC-1404 resolve/module queries is now complete; INC-1405
  type/effect queries is the next target.

## Validation

Focused and workspace tests, clippy, formatting, and diff checks passed. The
completion commit and machine-readable evidence are recorded in
`docs/status/implementation-status.toml`.
