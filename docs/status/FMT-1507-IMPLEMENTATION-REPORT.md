# FMT-1507 implementation report

## Result

FMT-1507 now provides both accepted integration surfaces:

- Preview `ling fmt` under DEC-0028;
- Experimental synchronous `textDocument/formatting` under RFC-0026.

The LSP adapter formats only the current open writable RFC-0023 overlay. It
constructs the compiler `SourceFile`, uses the compiler parser and existing
Format IR, calls `format_core_edit`, and projects zero or one whole-document
`TextEdit` through the negotiated DEC-0029 position encoding.

## Implementation

- `crates/ling-lsp` depends directly on `ling-format` and `ling-syntax`; no
  duplicate parser, regex formatter, or transport-independent style engine was
  introduced.
- Initialize advertises `documentFormattingProvider: true`.
- Requests require exact `textDocument.uri` plus fixed `tabSize=4` and
  `insertSpaces=true` options.
- Missing/closed documents fail with `-32602`, dependency documents with
  `-32005`, invalid URIs with `-32006`, and impossible internal projection with
  bilingual `-32603`.
- Safe changed source returns one edit; unchanged or invalid source returns an
  empty array. The server never applies the edit or mutates text/version/VFS.
- A leading BOM remains outside LSP position zero and is removed exactly once
  from replacement text, preventing loss or duplication. CRLF end positions are
  derived from original bytes while formatter output remains LF.

## Tests

The focused suite covers:

- UTF-8/UTF-16/UTF-32 exact end positions for Unicode source;
- BOM, CRLF, latest accepted overlay version, and immutable response behavior;
- changed, unchanged, invalid, missing, read-only, malformed-option, and
  notification cases;
- existing lifecycle, overlay, position, formatter, and CLI compatibility.

Repository-wide test, lint, governance, support, status, traceability,
formatting, and deterministic-diff evidence is recorded in the task registry
when the implementation commit is bound.

## Compatibility impact

Added: Experimental `ling.lsp.formatting/0.1`, one standard LSP capability, one
request method, and JSON-RPC internal error `-32603` for fail-closed formatting.

Unchanged: Ling language semantics, diagnostics and error-code registry,
Semantic Graph/IDs, Audit Source, project and package protocols, bytecode/VM,
ABI, source-span units, and Unicode 17.0.0.

## Intentionally deferred

Configurable style, range/on-type formatting, format-on-save, minimal diffs,
filesystem/closed-file formatting, Workspace Edit, Semantic Transaction,
asynchronous cancellation/publication, multi-document edits, and Stable editor
compatibility remain unimplemented and unclaimed.
