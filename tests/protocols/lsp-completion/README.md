# `ling.lsp.completion/0.1` and `ling.lsp.completion/0.2` fixtures

RFC-0042's executable wire evidence lives in
`crates/ling-lsp/tests/completion.rs`. The suite drives the real JSON-RPC
handler and covers exact initialize discovery, all six checked contexts,
prefix filtering, deterministic item order, exact text edits, malformed
request contexts, notification silence, and rejection of incomplete source.

The compiler-side candidate inventory is covered independently by
`crates/ling-db/src/checked_completion_catalog.rs`. Neither suite calls a
network service or an AI/model ranking path.

RFC-0043's negotiated 0.2 extension and exact 0.1 fallback are covered by
`crates/ling-lsp/tests/completion_resolve.rs`; its lazy-presentation evidence is
indexed separately in `tests/protocols/lsp-completion-resolve/README.md`.
