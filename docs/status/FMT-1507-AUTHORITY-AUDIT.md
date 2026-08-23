# FMT-1507 Authority Audit: Formatter CLI/LSP Integration

## Outcome

FMT-1507 has sufficient Accepted authority and is implemented as a bounded
Preview/Experimental integration. DEC-0028 governs `ling fmt`; DEC-0057 governs
the in-process whole-document edit; Accepted RFC-0026 governs the synchronous
`textDocument/formatting` adapter over the RFC-0023 open-document overlay.

The parent task does not require or claim range formatting, format-on-save,
`WorkspaceEdit`, Semantic Transaction, asynchronous request publication, or a
Stable editor contract. Those distinct behaviors remain under the open LSP and
Semantic Transaction gaps.

## Normative traceability

- DEC-0023 §§1, 5–9 requires compiler-CST formatting, safe publication,
  idempotence, preservation, and separation from canonical Audit Source.
- DEC-0028 defines the exact Preview `ling fmt` file/stdin/check/JSON contract.
- DEC-0057 defines zero or one original-byte whole-document edit without wire
  semantics.
- RFC-0004 defines the synchronous `ling lsp --stdio` lifecycle, framing,
  request IDs, response ordering, and channel purity.
- RFC-0023 defines path-free workspace/dependency/untitled URIs, immutable VFS
  snapshots, open state, monotonic client versions, and dependency writability.
- DEC-0029 defines UTF-8/UTF-16/UTF-32 projection from original UTF-8 bytes.
- RFC-0026 defines the exact `textDocument/formatting` request, fixed Ling
  options, capability, snapshot rule, TextEdit cardinality, BOM/CRLF behavior,
  errors, compatibility boundary, and explicit non-claims.

No lower-authority `zero` or `.zero` spelling enters the implementation.

## Gap disposition

- `GAP-FORMATTER-CLI-PROTOCOL-001` remains resolved by DEC-0028; incompatible
  CLI extensions still require new authority.
- `GAP-FORMATTER-AUTHOR-SOURCE-001` remains Open for broader style and localized
  Author Source alternatives. RFC-0026 reuses, but does not broaden, DEC-0023.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` remains Open for general snapshot identity,
  concurrent freshness, Workspace Edit, cancellation, range edits, and
  transactions. RFC-0026 closes only the synchronous formatting response.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` remains Open. Formatting does not read or
  mutate Semantic Graph/Transaction data and therefore does not depend on a
  guessed Semantic Transaction protocol.

This scoping removes the former false requirement that a read-only formatting
response first define every future semantic mutation protocol.

## Evidence

`crates/ling-lsp/tests/formatting.rs` exercises the accepted wire contract:

- exact advertised capability and method marker;
- hard-coded UTF-8, UTF-16, and UTF-32 whole-document end positions;
- Unicode, BOM, CRLF, latest-overlay, and response immutability;
- no edits for unchanged or invalid source;
- fail-closed missing, read-only, malformed-option, and notification behavior.

Existing formatter property suites continue to prove idempotence, compiler
token/CST preservation, checked semantic equivalence, comment preservation,
and canonical Audit byte equality. Existing lifecycle, overlay, position, and
stdio tests protect their protocol boundaries.

## Compatibility and deferred work

The change adds Experimental `ling.lsp.formatting/0.1` and
`documentFormattingProvider: true`. It allocates no Ling diagnostic and changes
no syntax, type, Effect, runtime, Semantic ID, Audit bytes, package identity,
bytecode, ABI, source-span unit, or Unicode 17.0.0 data.

Range/on-type formatting, minimal diffs, configurable style, format-on-save,
closed-file or filesystem formatting, `WorkspaceEdit`, multi-document edits,
Semantic Transaction, cancellation, parallel request scheduling, and Stable
compatibility are intentionally deferred.
