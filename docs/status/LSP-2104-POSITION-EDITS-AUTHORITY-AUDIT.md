# LSP-2104-POSITION-EDITS Authority Audit

## Outcome

`LSP-2104-POSITION-EDITS` is a bounded child of the blocked `LSP-2104` target,
authorized by Accepted `DEC-0070`. It composes the accepted source position
projection with the immutable UTF-8 byte-edit primitive. It does not accept or
implement public LSP `didChange`, URI/version, VFS, or transaction semantics.

## Normative traceability

- Accepted `DEC-0002` keeps original UTF-8 byte spans authoritative.
- Accepted `DEC-0029` defines explicit UTF-8/UTF-16/UTF-32 counting, SourceMap
  projection, strict boundary validation, and no-clamping behavior.
- Accepted `DEC-0069` defines ordered immutable original-byte edit application,
  replacement revalidation, and atomic failure.
- Accepted `RFC-0023` defines only full-text Preview overlays and defers range
  edits until encoding, bounds, and transaction semantics are accepted.
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

Encoding negotiation, LSP JSON fields, URI/document versions, overlay/VFS
publication, stale-result rejection, cancellation, diagnostics, Workspace
Edits, and Semantic Transactions remain in the `LSP-2104` parent and open
governance gaps.

