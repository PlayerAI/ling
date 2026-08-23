# LSP-2404-CHECKED-SOURCE-FIXTURES Authority Audit

## Outcome

`LSP-2404-CHECKED-SOURCE-FIXTURES` is an authorized bounded child under
Accepted DEC-0087. It adds compiler-owned regression fixtures for exact source
bytes, spans, order, Unicode/BOM/CRLF handling, and VFS revision invalidation.
It does not itself define semantic-token expected output. Public LSP-2404 was
subsequently authorized by Accepted RFC-0046/RFC-0047/RFC-0048 and completed
through a separate client-visible conformance corpus.

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

## Subsequent integration

Accepted RFC-0046/RFC-0047/RFC-0048 later supplied the taxonomy, generation,
position, full/delta, result identity, cancellation, limits, privacy, and
Preview lifecycle authority. Completed LSP-2404 consumes this child's exact
source-boundary evidence without broadening DEC-0087.

## Evidence and compatibility

- The two integration fixtures cover Unicode/BOM/CRLF original bytes and VFS
  revision invalidation.
- No language, diagnostic, schema, Semantic ID, CLI/LSP, runtime, bytecode, VM,
  ABI, dependency, or Unicode 17.0.0 behavior changes.
