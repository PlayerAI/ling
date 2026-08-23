# LSP pull-diagnostics protocol fixtures

The executable fixtures for `ling.lsp.pull-diagnostics/0.1` live in
`crates/ling-lsp/tests/pull_diagnostics.rs`. They cover capability-gated method
availability, exact provider fields, document and workspace full/unchanged
reports, stateless result identity, URI ordering, open/closed versions, removed
URI clearance, push-value parity, temporary syntax diagnostics, validation,
the 1024-entry previous-result bound, and oversized-response failure atomicity.

`crates/ling-lsp/tests/diagnostic_adapter.rs` independently covers original
UTF-8 byte-span projection under UTF-8, UTF-16, and UTF-32. The pull provider
reuses those exact adapted JSON values without transformation. RFC-0033 is the
normative authority; this directory contains evidence, not another schema.
