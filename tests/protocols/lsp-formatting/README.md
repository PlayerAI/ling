# LSP document-formatting protocol fixtures

Authority: Accepted `RFC-0026`, `RFC-0023`, `DEC-0029`, `DEC-0023`, and
`DEC-0057`.

Executable coverage lives in `crates/ling-lsp/tests/formatting.rs`. It verifies:

- `ling.lsp.formatting/0.1` capability and method selection;
- exact UTF-8, UTF-16, and UTF-32 whole-document `TextEdit` ranges;
- Unicode, BOM, CRLF, latest-overlay, immutability, and deterministic output;
- empty results for unchanged or invalid Author Source;
- fail-closed preinitialize, post-shutdown, missing, read-only,
  malformed-option, and notification behavior.

`IDE-2310` completion and the reconciliation of its historical authority audit
are recorded in `docs/status/IDE-2310-IMPLEMENTATION-REPORT.md`.

The fixture does not claim range formatting, format-on-save, filesystem access,
`WorkspaceEdit`, Semantic Transaction, cancellation, or Stable compatibility.
