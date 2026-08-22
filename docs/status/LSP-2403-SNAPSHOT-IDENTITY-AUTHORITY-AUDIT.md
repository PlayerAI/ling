# LSP-2403-SNAPSHOT-IDENTITY Authority Audit

## Outcome

`LSP-2403-SNAPSHOT-IDENTITY` is an authorized bounded child under Accepted
DEC-0086. It retains the existing source identity and session-local VFS
revision on the internal checked-token source observation. It does not
implement semantic-token full/delta transport or unblock public LSP-2403.

## Normative traceability

- DEC-0019 governs immutable query keys and deterministic invalidation.
- DEC-0071 governs immutable workspace/source snapshot observations.
- DEC-0084 supplies the lossless lexical token source index.
- DEC-0085 supplies the exact checked definition identity join.
- DEC-0086 authorizes only SourceId/Revision retention, source-order reuse,
  and edit invalidation; it expressly excludes document versions and wire
  protocol fields.

## Implemented boundary

`CheckedTokenSourceIndex` retains the existing `SourceId` and the
`FileSnapshot` `Revision` used to construct it. Repeated queries for the same
`QueryKey` reuse the immutable value. A source edit produces a new revision and
query-key result while the old observation remains unchanged.

The child does not encode token integers, positions, legends, modifiers,
fallback origins, result IDs, full responses, delta edits, URI/version fields,
negotiation, cancellation, limits, or JSON-RPC output.

## Specification gap and deferred work

Public LSP-2403 remains blocked by the missing token taxonomy, position/version
binding, full/delta schema, result-ID lifecycle, stale/base handling,
cancellation/limits, and protocol lifecycle decisions recorded in
`GAP-LSP-TRANSACTION-PROTOCOL-001` and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Evidence and compatibility

- `crates/ling-db/src/checked_token_source_index.rs` owns the source/revision
  retention.
- `CompilerDb::checked_token_source_index` passes the immutable source revision
  already used by the existing query key and cache.
- The focused test covers source identity, revision retention, checked facts,
  source order, and immutable cache reuse.
- No language, diagnostic, schema, Semantic ID, CLI/LSP, runtime, bytecode, VM,
  ABI, dependency, or Unicode 17.0.0 behavior changes.
