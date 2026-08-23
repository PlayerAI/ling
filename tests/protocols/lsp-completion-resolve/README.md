# `ling.lsp.completion-resolve/0.1` fixtures

RFC-0043's executable wire evidence lives in
`crates/ling-lsp/tests/completion_resolve.rs`. The suite drives the real
JSON-RPC handler and covers exact capability negotiation, the unchanged
RFC-0042 fallback, opaque deterministic item handles, checked signature,
Effect, and Capability presentation, attached `///` Author Source
documentation, plaintext and escaped Markdown, Unicode and CRLF/BOM edit
preservation, no-op resolution without metadata, notification
silence, and malformed, modified, missing, and stale handle failures.

Compiler metadata selection is covered independently by
`crates/ling-db/src/completion_metadata_index.rs` and the candidate-to-metadata
identity bridge by `crates/ling-db/src/checked_completion_catalog.rs`. The
fixtures use no filesystem lookup, network service, inferred prose, or
AI/model path.

The protocol constructs no `DocumentationId`: it uses only RFC-0043's bounded
snapshot-local CST/Format IR attachment rule for contiguous `///` lines.
