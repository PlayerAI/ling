# LSP-2404-CHECKED-SOURCE-FIXTURES Authority Audit

## Outcome

`LSP-2404-CHECKED-SOURCE-FIXTURES` is an authorized bounded child under
Accepted DEC-0087. It adds compiler-owned regression fixtures for exact source
bytes, spans, order, Unicode/BOM/CRLF handling, and VFS revision invalidation.
It does not define semantic-token expected output or unblock public LSP-2404.

## Normative traceability

- DEC-0002 keeps original UTF-8 spans authoritative.
- DEC-0019 keeps incremental query identity and invalidation deterministic.
- DEC-0071 supplies immutable source/workspace snapshot observations.
- DEC-0084 supplies lexical token spelling and source order.
- DEC-0085 supplies exact checked definition facts.
- DEC-0086 supplies SourceId/Revision snapshot identity.
- DEC-0087 authorizes only internal source fixtures and excludes all protocol
  presentation fields.

## Implemented boundary

`crates/ling-db/tests/checked_token_source.rs` verifies a leading BOM, CRLF,
Chinese identifiers, emoji literal bytes, monotonic source spans, exact source
slices, unchanged cache reuse, and new-revision invalidation after an edit.
The previous observation remains immutable and is never partially replaced.

The fixtures do not assert token categories, modifiers, positions, document
versions, fallback origins, legends, result IDs, full responses, delta edits,
negotiation, cancellation, limits, or JSON-RPC output.

## Specification gap and deferred work

Public LSP-2404 remains blocked by the missing versioned fixture schema,
semantic-token taxonomy, typed/fallback provenance, position/version binding,
full/delta equivalence, result-ID/base handling, stale/cancellation/limits, and
protocol lifecycle decisions recorded in the registered LSP/semantic gaps.

## Evidence and compatibility

- The two integration fixtures cover Unicode/BOM/CRLF original bytes and VFS
  revision invalidation.
- No language, diagnostic, schema, Semantic ID, CLI/LSP, runtime, bytecode, VM,
  ABI, dependency, or Unicode 17.0.0 behavior changes.
