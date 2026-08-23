# LSP rename protocol evidence

This directory records the executable evidence boundary for Accepted
`RFC-0041` and Preview marker `ling.lsp.rename/0.1`.

The executable corpus is `crates/ling-lsp/tests/rename.rs`. It covers exact
transactional capability discovery and validation, checked definition,
binding, reference, and explicit import-alias selection, multi-document
versioned edits, Unicode 17.0.0 name policy, normalized and confusable
collisions, visibility/coherence simulation, DefinitionId migration, relation
topology, read-only dependencies, source-less builtins, temporary sources,
UTF-8/16/32 projection, BOM/CRLF/Unicode, deterministic null and failure
results, invalid parameters, notifications, and checked-pipeline failure.
Exact initialize responses are also locked by
`tests/fixtures/lsp-diagnostics-v1/`.

The server returns a standard proposed Workspace Edit and never applies it.
This protocol does not authorize general Semantic Transactions, language Alias
syntax, localized Author Source rewriting, generated/dependency mutation,
module/file rename, type-only identity inference, cancellation, annotations,
or Stable compatibility.
