# LSP diagnostic-control protocol fixtures

The executable fixtures for `ling.lsp.diagnostic-control/0.1` live in
`crates/ling-lsp/tests/diagnostic_control.rs` and the module unit tests. They
cover default/custom discovery, option bounds, exact root identity, first-wins
deduplication, independent roots, per-document and workspace caps, omission
summary shape/count/range, URI order, push/pull parity, and recovery clearance.

The push and pull suites retain stale-result, transport-limit, result-ID,
temporary-source, and ledger failure-atomicity evidence under their 0.2
markers. The diagnostic adapter suite independently proves original UTF-8 byte
projection under UTF-8, UTF-16, UTF-32, and CRLF. RFC-0034 is normative; this
directory is evidence rather than a second schema.
