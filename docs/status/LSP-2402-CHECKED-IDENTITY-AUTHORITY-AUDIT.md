# LSP-2402-CHECKED-IDENTITY Authority Audit

## Outcome

`LSP-2402-CHECKED-IDENTITY` is an authorized bounded child under Accepted
DEC-0085. It joins existing lexical source entries to exact checked definition
facts for internal compiler analysis. It does not implement typed semantic
tokens or unblock public LSP-2402.

## Normative traceability

- DEC-0002 keeps original UTF-8 spans authoritative.
- DEC-0012 keeps existing Definition/Semantic identity and canonical bytes
  authoritative.
- DEC-0019 governs immutable query keys and deterministic invalidation.
- DEC-0084 supplies the lexical token source index.
- DEC-0085 §§Decision 1–4 authorize exact span/name joining, existing type/
  effect/capability facts, source order, caching, and the negative presentation
  boundary.

## Implemented boundary

`CheckedTokenSourceIndex` joins one `TokenSource` with a
`TypedDefinitionSymbol` only when the logical source name and original span are
equal. Joined entries retain the existing definition ID, canonical type text,
effect names, and capability names; all other tokens retain empty optional
facts. Source order and query-key identity remain unchanged.

The query does not classify references, assign semantic-token categories or
modifiers, select precedence, create fallback origins, project positions or
versions, negotiate clients, or publish protocol data.

## Specification gap and deferred work

The parent LSP-2402 remains blocked by the missing taxonomy, source-origin and
fallback schema, position/version, redaction, negotiation, transport, and
lifecycle decisions recorded in `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and
`GAP-LSP-TRANSACTION-PROTOCOL-001`.

## Evidence and compatibility

- `crates/ling-db/src/checked_token_source_index.rs` owns the exact join.
- `CompilerDb::checked_token_source_index` caches immutable results and reuses
  the lexical and typed-definition indexes.
- The focused test covers source identity, checked type/effect facts, exact
  definition-token matching, and cache reuse.
- No language, diagnostic, schema, Semantic ID, CLI/LSP, runtime, bytecode,
  VM, ABI, dependency, or Unicode 17.0.0 behavior changes.
