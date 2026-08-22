# LSP-2401-LEXICAL-SOURCE Authority Audit

## Outcome

`LSP-2401-LEXICAL-SOURCE` is an authorized bounded child under Accepted
DEC-0084. It records the existing lexer stream as an immutable compiler source
for future semantic-token design. It does not unblock the public
semantic-token taxonomy or any LSP transport.

## Normative traceability

- DEC-0002 keeps original UTF-8 `SourceId + Span` authoritative.
- DEC-0019 keeps source-query revisions and immutable in-process observations
  deterministic.
- DEC-0084 §§Decision 1–4 authorize `CompilerDb::token_source_index`, exact
  original spelling, lexer order, lexical-error visibility, cache identity,
  and the negative protocol boundary.
- The LSP-2401 parent audit and `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` continue
  to block semantic categories, legends, modifiers, negotiation, positions,
  versions, full/delta transport, and publication.

## Implemented boundary

`TokenSourceIndex` retains each existing lexer `TokenKind`, original UTF-8
`Span`, and the exact source spelling. It preserves layout, trivia, error, and
EOF entries in source order and exposes only an `is_valid` lexical-error bit.
`CompilerDb` caches successful indexes by the existing `QueryKey`; a failed
span projection publishes no index. The source name is the existing VFS
logical name and is not converted to a URI or host path.

## Specification gap and deferred work

The public LSP-2401 taxonomy remains blocked. No semantic category, custom
modifier, effect/capability disclosure, declaration/use precedence,
position/version projection, client fallback, limits, cancellation, stale
behavior, JSON-RPC response, or protocol inventory entry is inferred.

## Evidence and compatibility

- `crates/ling-db/src/token_source_index.rs` owns the immutable projection and
  typed span failure.
- `CompilerDb::token_source_index` and its Unicode/BOM/CRLF test cover exact
  source spelling, source identity, stable token kind names, cache reuse, and
  original-byte slicing.
- No diagnostic allocation, schema, Semantic ID, CLI/LSP behavior, runtime,
  bytecode, VM, ABI, dependency, or Unicode table changes are made.
