# LSP lifecycle fixtures

This directory names the executable evidence for `RFC-0004` and
`PROTO-LSP-LIFECYCLE`. The Rust integration suite in
`crates/ling-lsp/tests/lifecycle.rs` is the source of truth for framed
initialize/initialized/shutdown/exit transcripts, encoding negotiation,
Unicode workspace metadata, malformed transport, and protocol errors.
The CLI integration test in `crates/ling-cli/tests/lsp.rs` additionally starts
the real `ling lsp --stdio` binary and proves stdout framing and stderr purity.

The fixture deliberately does not cover document synchronization, diagnostics,
Workspace Edits, Semantic Transactions, snapshots, or cancellation; those
surfaces remain unresolved in the execution-plan gap register.
