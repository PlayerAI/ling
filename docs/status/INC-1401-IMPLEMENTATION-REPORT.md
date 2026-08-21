# INC-1401 Implementation Report: Incremental Query Boundary

## Outcome

INC-1401 is complete as an accepted architecture decision. `DEC-0019` fixes
the internal, in-memory query boundary required before VFS/revision and query
implementation work. It deliberately does not create a placeholder crate,
cache file, LSP API, or public protocol.

## Normative traceability

- `DEC-0019` Question and Decision §§1–3 define immutable query families,
  exact source revisions, dependency invalidation, and deterministic cache-key
  inputs without changing language semantics or Semantic ID domains.
- `DEC-0019` Decision §§4–8 define the initial single-threaded scheduler,
  internal cancellation boundary, no-persistence rule, test-only query traces,
  and dependency/license review requirements.
- `DEC-0019` Compatibility impact preserves source behavior, diagnostics,
  schemas, protocols, canonical bytes, Unicode 17.0.0, and original UTF-8
  spans; no new public diagnostic allocation is authorized.

## Evidence

- `docs/decisions/0019-incremental-query-boundary.md` is Accepted and indexed
  in the authority and lifecycle registries.
- `GAP-INCREMENTAL-CACHE-001` no longer blocks INC-1401; it remains Open for
  persistent cache schema/migration, parallel scheduling, and corruption
  recovery that DEC-0019 explicitly defers.
- The execution backlog marks INC-1401 Done. Governance, lifecycle, gap,
  generated-report, formatting, and status checks provide deterministic
  registry evidence.

## Compatibility and deferred work

- No compiler/runtime source code, CLI command, LSP field, JSON schema,
  diagnostic code, Semantic ID version, cache artifact, or public protocol was
  added.
- INC-1402 (VFS/revision) is the next implementation target and must consume
  this boundary. Persistent cache, parallel query execution, compiler-facing
  cancellation, LSP cancellation, and structured Task cancellation remain
  separately governed.

## Validation

The final validation commands and completion commit are recorded in
`docs/status/implementation-status.toml` after the status registry update.
