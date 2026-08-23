# LSP references protocol fixtures

Authority: Accepted `docs/RFC-0039.md`.

The executable corpus is `crates/ling-lsp/tests/references.rs`, supported by
the compiler-index tests in `crates/ling-db/src/reference_search_index.rs` and
exact initialize transcript fixtures under `tests/fixtures/lsp-diagnostics-v1/`.

The corpus covers exact provider discovery and malformed capabilities;
declaration and reference selection; `includeDeclaration`; checked
read/write/call grouping; source-less builtin behavior; workspace and tracked
read-only dependency URI reuse; UTF-8/16/32, BOM, CRLF, Chinese, emoji, and
combining text; empty selection; invalid params; notification silence; and
atomic checked failure.

Version 0.1 exposes standard `Location[]` only. The complete internal relation
vocabulary is `read`, `write`, `call`, `type`, and `implementation`, while the
exact discovery marker records that only the first three are emitted until
resolver-owned type/implementation occurrence identities exist.
