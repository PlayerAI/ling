# Semantic Query and Transaction proposal protocol evidence

Accepted RFC-0027 defines three bounded Preview JSON protocols:

- `ling.semantic-query/0.1` is a deterministic exact-NFC lookup of checked user
  definitions in one import-free file program;
- `ling.semantic-transaction/0.1` describes one full-source replacement
  proposal with an exact base Program ID, sorted authorized target IDs, all
  four preserve constraints, and bounded provenance;
- `ling.semantic-transaction-result/0.1` reports a validated semantic diff with
  `committed: false`.

The transaction reader rejects stale bases before compiling candidate text. A
candidate is compiled through the existing checked pipeline and must preserve
the complete definition set, types, Effects, and Capabilities; every changed
Body ID must be authorized. The command never writes the source or request,
runs Checked Core, or claims `Graph.Commit` or an LSP `WorkspaceEdit`.

Executable evidence is in `crates/ling-cli/src/semantic_commands.rs` and
`crates/ling-cli/tests/semantic_commands.rs`. Independent shape and reader
evidence is in `schemas/semantic-query/0.1/`,
`schemas/semantic-transaction/0.1/`, and
`schemas/semantic-transaction-result/0.1/`.
