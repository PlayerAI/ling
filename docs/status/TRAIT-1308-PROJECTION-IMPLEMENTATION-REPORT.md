# TRAIT-1308-PROJECTION Implementation Report

## Status and scope

**Status: Done (bounded projection slice).**

This task implements the data-only Trait Semantic Graph projection authorized
by Accepted RFC-0022. It is deliberately narrower than the parent
`TRAIT-1308` item: the parent remains `BlockedSpec` for the full IDE/LSP
surface and does not gain a v0.0.1 or Stable 1.0 support claim.

## Normative clauses covered

- RFC-0022 §§1–3: optional `x-ling-trait-ide` extension on
  `ling.semantic/0.1` and `/0.2`, checked dictionary-witness input, and
  explicit non-goals.
- RFC-0022 §§4–6: deterministic path-free Trait and implementation IDs,
  ordered witness/member records, implementation definition mapping, and
  original UTF-8 byte spans.
- RFC-0022 §§7–8: structural reader validation, forward-compatible optional
  extension behavior, and compatibility boundaries.
- RFC-0005 §§4.1–4.2, DEC-0027, and RFC-0021: the projection consumes the
  already selected immutable witness and never re-runs Trait selection.

## Implemented slice

- `ling-semantic` now exposes an optional `SemanticTraitIdeProjection` on both
  file and project snapshots. Graphs without witnesses keep their existing
  JSON shape.
- Each witness records the Trait identity, receiver, selected implementation,
  obligation order, member definition identities, source names, and original
  byte spans. IDs use the RFC-0022 length-prefixed identity domains and omit
  host paths, allocation details, and map iteration order.
- The reader validates extension version, identities, ordered/unique ordinals,
  non-empty member data, source spans, and that projected member IDs exist in
  the graph definitions.
- Implementation-member parameter and expression nodes are emitted so the
  projected graph remains internally reference-complete for checked Trait
  programs.
- `DictionaryWitness` provides the minimal checked accessors needed by the
  projection without exposing backend selection or mutable solver state.

## Evidence

- `cargo test -p ling-semantic --locked --offline`: 15 unit tests and 5 project
  snapshot tests passed, including projection identity/span preservation,
  JSON round-trip, alternate source provenance, and malformed extension
  rejection.
- `cargo check -p ling-semantic --locked --offline` passed after the bounded
  implementation was added.
- `cargo test --workspace --locked --offline --quiet`: passed (including the
  15 semantic unit tests, 5 project snapshot tests, and 92 xtask tests; one
  repository test remains explicitly ignored by its existing fixture policy).
- `cargo clippy --workspace --all-targets --all-features --locked --offline --
  -D warnings`, `cargo fmt --all -- --check`, and `git diff --check` passed.
- `target\\debug\\xtask.exe governance check-all`, `status verify`, and `ci
  verify` passed: 66 governance documents, 28 gaps, 41 lifecycle records, 23
  protocols, 82 diagnostic codes, and 274 implementation tasks (68 Done).

## Compatibility and traceability

- No diagnostic code, LSP JSON-RPC method, Workspace Edit, transaction,
  bytecode instruction, VM behavior, CLI contract, or Unicode 17.0.0 table
  changed.
- The existing Semantic Graph protocol remains the authority; the new field is
  an Experimental `x-*` extension registered through
  `PROTO-SEMANTIC-GRAPH-JSON`.
- `docs/status/TRAIT-1308-AUTHORITY-AUDIT.md` records why the parent task stays
  blocked and which authority is still missing.

## Deferred work

Hover rendering, completion ranking, definition/implementation requests,
identity-preserving rename, constraint diagnostics and repairs, document
position/version preconditions, cancellation, stale-result handling, atomic
Semantic Transactions, and Stable 1.0 lifecycle promotion remain explicitly
deferred to the parent `TRAIT-1308` task and its unresolved governance gaps.
