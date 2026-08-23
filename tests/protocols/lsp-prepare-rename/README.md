# LSP prepare-rename protocol evidence

This directory records the executable evidence boundary for Accepted
`RFC-0040` and Preview marker `ling.lsp.prepare-rename/0.1`.

The executable corpus is `crates/ling-lsp/tests/prepare_rename.rs`. It covers
exact capability discovery and validation, declaration/reference selection,
unreferenced declarations, writable temporary and workspace sources,
read-only dependencies, source-less builtins, UTF-8/16/32 projection,
BOM/CRLF/Unicode, null results, invalid parameters, notifications, and checked
pipeline failure. Exact initialize responses are also locked by
`tests/fixtures/lsp-diagnostics-v1/`.

This protocol is read-only. It does not validate a new name, return edits,
preserve DefinitionId, or authorize `textDocument/rename`.
