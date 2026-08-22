# LSP-2401-LEXICAL-SOURCE implementation report

## Result

The bounded lexical-token source child is implemented under Accepted DEC-0084.
It gives future semantic-token design a lossless compiler-owned source without
inventing a taxonomy or publishing an LSP surface.

## Implementation

- `TokenSourceIndex` stores source identity, logical source name, lexical
  validity, and source-order `TokenSource` entries.
- Each `TokenSource` retains the lexer `TokenKind`, original UTF-8 span, and
  exact original source spelling, including synthetic layout/EOF zero-width
  entries.
- `CompilerDb::token_source_index` reuses the existing token/source queries,
  caches by the immutable query key, and maps projection failures to a typed
  internal `QueryError` without publishing partial state.

## Verification

- `cargo fmt --all -- --check`
- `cargo test -p ling-db --all-targets --locked --offline` — 44 tests passed.
- The focused test covers Chinese identifiers, BOM, CRLF, original-byte
  slicing, stable token-kind names, exact source identity, and cache reuse.

## Boundaries

This child is not semantic-token generation, a taxonomy/legend, a modifier
model, position conversion, client negotiation, full/delta transport,
cancellation, stale-result handling, or an LSP/JSON-RPC response. Public
`LSP-2401` remains `BlockedSpec`.
