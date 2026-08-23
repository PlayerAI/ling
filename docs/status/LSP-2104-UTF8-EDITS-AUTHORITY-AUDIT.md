# LSP-2104-UTF8-EDITS Authority Audit

## Outcome

`LSP-2104-UTF8-EDITS` is a bounded source child authorized by Accepted
`DEC-0069`. It covers only immutable, in-process application of original UTF-8
byte ranges. Accepted `RFC-0029` now supplies the separate public range schema,
negotiated-position adapter, document-version policy, and failure-atomic VFS
publication needed to compose the completed `LSP-2104` parent.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans authoritative.
- Accepted `DEC-0019` keeps source snapshots immutable and separates compiler
  revisions from any editor protocol state.
- Accepted `DEC-0029` defines explicit position projection, but this child
  consumes original byte offsets and does not infer an editor position.
- Accepted `RFC-0023` defines full-text overlays; Accepted `RFC-0029` provides
  the later incremental encoding, bounds, ordering, and publication semantics.
- Accepted `DEC-0069` authorizes the source-only range/batch primitive and
  explicitly excludes public LSP behavior.

## Current interface evidence

The source crate already validates UTF-8, leading BOM placement, normalized
line endings, and original-to-lexical mappings. The new `Utf8Edit` and
`SourceFile::apply_utf8_edits` boundary reuses those checks while returning a
new immutable source. No VFS or `ling-lsp` state is changed by a failed or
successful call.

## Evidence and compatibility

The focused source tests cover Unicode, emoji, combining-compatible source
identity, BOM, CRLF, full replacement equivalence, ordered edits, invalid
boundaries, invalid replacement bytes, and atomic failure. No diagnostic code,
schema, Semantic ID, public protocol, runtime, bytecode, VM, or Unicode table
changed.

## Intentionally deferred

Compiler request identity, stale analysis publication, cancellation,
diagnostics, Workspace Edits, Semantic Transactions, and Stable compatibility
remain in later tasks and open governance gaps. RFC-0029 composes this child
without changing its source-only contract.
