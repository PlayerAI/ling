# LSP-2201-DIAGNOSTIC-POSITION Authority Audit

## Outcome

`LSP-2201-DIAGNOSTIC-POSITION` is a bounded child of `LSP-2201`, authorized by
Accepted `DEC-0072`. It projects one existing compiler diagnostic span through
an explicit source position encoding. Accepted RFC-0031 now consumes this
primitive in the public adapter without changing the child's contract.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans authoritative and
  requires explicitly labeled source position units.
- Accepted `DEC-0029` defines strict UTF-8/UTF-16/UTF-32 SourceMap projection,
  exact boundaries, BOM/CRLF handling, and no-clamping behavior.
- Accepted `DEC-0034` preserves path-free diagnostic ordering and does not
  authorize field mapping or publication.
- `PROTO-DIAGNOSTIC-JSON` remains a separate Preview writer; it does not
  define an LSP diagnostic schema.
- Accepted `DEC-0072` authorizes the private source-layer projection; Accepted
  RFC-0031 separately authorizes its use by the public adapter.

## Current interface evidence

`ling-diagnostics::DiagnosticSpan` retains a logical name and original `u64`
byte offsets. The child adds a private `ling-lsp` range and typed failure that
requires exact logical-name identity, validates the offset domain and ordering,
and delegates both endpoints to `SourceFile::lsp_position`. It is not wired to
`LspServer`, transport, publication, or diagnostic serialization.

## Evidence and compatibility

Focused tests cover all explicit encodings, Chinese/emoji text, BOM/CRLF,
final-line boundaries, mismatched names, reversed spans, u32 overflow, source
overflow, and no-clamping failures. No diagnostic code, schema, Semantic ID,
source span, CLI, runtime, bytecode, VM, ABI, or Unicode table changed.

## Intentionally deferred

URI/version and snapshot association, stale results, publication/clearance,
cancellation, suppression, root-cause grouping, caps, tags, repair application,
and JSON-RPC publication fixtures remain in LSP-2202 through LSP-2205.
