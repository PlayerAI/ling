# LSP-2105 implementation report

> Status: Done / 已完成
> Task: `LSP-2105`
> Authority: Accepted `RFC-0030`, `RFC-0002`, `RFC-0004`, `RFC-0023`,
> `RFC-0025`, `DEC-0019`, and `DEC-0071`

## Scope

This milestone implements bounded client-published workspace reload without
host filesystem access. Unique logical source and project-input deltas are
validated against an exact session revision, canonicalized, applied to a
private server/VFS candidate, and published only after the whole batch succeeds.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2105: respond to
  source/manifest/lock/config/dependency changes, advance project input
  revisions, and avoid eager full rebuilds for individual file events.
- `RFC-0030` §§1–7: capability, request/result schema, exact revision identity,
  source/input deltas, limits, canonical order, overlay behavior, atomicity,
  errors, and explicit host-watcher non-claims.
- `DEC-0019` and `DEC-0071`: revision-aware invalidation and immutable
  workspace observation.

## Implementation

- `ling-source` adds the `PackageLock` input plus revisioned, overlay-safe disk
  source and workspace-input removal primitives.
- `ling-db` includes package-lock bytes in disposable query/cache identity and
  clean-database reconstruction.
- `ling-lsp` advertises `ling.lsp.workspace/0.1`, parses canonical bounded
  reload batches, applies them in canonical order to a clone, preserves open
  overlays and client-version history, and returns the final decimal revision.
- Request snapshots now include canonical immutable project-input snapshots.

## Tests and evidence

- Source tests cover revisioned removals, absent-input no-ops, source-ID
  non-reuse, and open-overlay preservation.
- LSP tests cover exact capability fields, array-order equivalence, source and
  all project-input kinds, exact Unicode/BOM/CRLF bytes, no-op revisions,
  overlay-hidden disk updates, close behavior, removal, stale/malformed/
  duplicate/unsupported/oversized batches, notifications, and failure atomicity.

## Compatibility and determinism

- Adds Experimental current-writer-only `ling.lsp.workspace/0.1`; there is no
  predecessor or migration requirement.
- Package-lock bytes affect only disposable internal query identity. No public
  cache format or cross-process revision is introduced.
- No Ling syntax, diagnostics, public schema, Semantic ID, runtime, bytecode,
  VM, ABI, network/filesystem, or Unicode 17.0.0 behavior changes.

## Verification

The milestone is accepted only after focused source/database/LSP tests and the
full locked offline workspace, strict Clippy, CI, governance, LSP, support,
status, RC0, traceability, formatting, and deterministic-diff gates pass. The
exact implementation commit is
`49994b9132ff22ae3fd17ab172476d020a79febe`, bound in the task registry.

## Intentionally deferred

Host watcher adapters, file URI/path/symlink mapping, multi-root projects,
network dependencies, diagnostics refresh, compiler-result cancellation and
staleness, Workspace Edits, Semantic Transactions, and Stable compatibility
remain owned by later tasks and Accepted authorities.
