# LSP-2104-POSITION-EDITS Authority Audit

## Outcome

`LSP-2104-POSITION-EDITS` is a bounded source child authorized by Accepted
`DEC-0070`. It composes the accepted source position projection with the
immutable UTF-8 byte-edit primitive. Accepted `RFC-0029` now supplies the
separate public `didChange`, URI/version, ordering, and failure-atomic VFS
composition needed by the completed `LSP-2104` parent.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans authoritative.
- Accepted `DEC-0029` defines explicit UTF-8/UTF-16/UTF-32 counting, SourceMap
  projection, strict boundary validation, and no-clamping behavior.
- Accepted `DEC-0069` defines ordered immutable original-byte edit application,
  replacement revalidation, and atomic failure.
- Accepted `RFC-0023` defines full-text overlays; Accepted `RFC-0029` provides
  the later incremental encoding, bounds, ordering, and publication semantics.
- Accepted `DEC-0070` authorizes only the source-layer composition of those
  boundaries.

## Current interface evidence

`ling-source` already exposes `SourceFile::original_offset` and the explicit
`PositionEncoding` projection. The child adds `LspPositionEdit` and ordered
application that converts positions before delegating to `Utf8Edit`; failed
projection or replacement validation never mutates the input source or VFS.

## Evidence and compatibility

Focused tests cover all supported encodings, Chinese/emoji text, BOM/CRLF,
full-replacement equivalence, ordered edits, invalid positions, and atomic
failure. No diagnostic code, schema, Semantic ID, public protocol, runtime,
bytecode, VM, or Unicode table changed.

## Intentionally deferred

Compiler request identity, stale analysis publication, cancellation,
diagnostics, Workspace Edits, Semantic Transactions, and Stable compatibility
remain in later tasks and open governance gaps. RFC-0029 composes this child
without changing its source-only contract.
