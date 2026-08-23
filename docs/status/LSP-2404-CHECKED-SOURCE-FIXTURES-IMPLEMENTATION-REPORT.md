# LSP-2404-CHECKED-SOURCE-FIXTURES implementation report

## Result

The bounded checked-token source fixture child is implemented under Accepted
DEC-0087. It provides deterministic source-boundary evidence without freezing
semantic-token or transport behavior.

## Implementation

- `crates/ling-db/tests/checked_token_source.rs` covers leading BOM, CRLF,
  Chinese identifiers, emoji literal bytes, source order, exact original-byte
  slices, cache reuse, and revision invalidation.
- Fixtures consume only the existing `CompilerDb` checked-token observation and
  `FileSnapshot` bytes; they emit no serialized semantic-token output.

## Verification

- `cargo fmt --all -- --check`
- `cargo test -p ling-db --all-targets --locked --offline` — 45 unit tests and
  2 checked-token-source fixture tests passed.
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`

## Boundaries and subsequent integration

This child is not semantic-token generation, a versioned fixture schema,
position projection, fallback classification, full/delta equivalence,
result-ID/base handling, cancellation, stale handling, or an LSP/JSON-RPC
response. Accepted RFC-0046/RFC-0047/RFC-0048 subsequently authorized the
separate public conformance corpus completed by `LSP-2404`; this child remains
only compiler-owned source-boundary evidence.
