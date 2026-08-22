# LSP-2402-CHECKED-IDENTITY implementation report

## Result

The bounded checked-token identity child is implemented under Accepted
DEC-0085. It supplies exact checked definition facts to future semantic-token
work without inventing a taxonomy or transport.

## Implementation

- `CheckedTokenSource` retains the lexical token and optional existing
  Definition ID, type display, effects, and capabilities.
- `CheckedTokenSourceIndex` preserves lexical source order and attaches facts
  only for exact source-name/span matches against `TypedDefinitionIndex`.
- `CompilerDb::checked_token_source_index` reuses the lexical and checked
  indexes, caches successful immutable results by the existing `QueryKey`, and
  publishes no fabricated facts for unmatched tokens.

## Verification

- `cargo fmt --all -- --check`
- `cargo test -p ling-db --all-targets --locked --offline` — 45 tests passed.
- `cargo clippy -p ling-db --all-targets --locked --offline -- -D warnings`
- The focused test covers a checked Unicode-capable source, exact definition
  token matching, canonical `Int` type/effect facts, source name, and cache
  reuse.

## Boundaries

This child is not semantic-token generation, a taxonomy/legend, a modifier or
fallback-origin model, reference classification, position/version projection,
client negotiation, full/delta transport, cancellation, stale handling, or an
LSP/JSON-RPC response. Public `LSP-2402` remains `BlockedSpec`.
