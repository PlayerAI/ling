# LSP-2101-LIFECYCLE implementation report

> Status: Done / 已完成
> Task: `LSP-2101-LIFECYCLE`
> Authority: Accepted `RFC-0004` and `DEC-0029`
> Verified commit: `38d95fb7b91c2035bd2b1b4ebf864c1693050925`

## Scope

This milestone implements only the bounded Preview lifecycle for
`ling lsp --stdio`: CRLF `Content-Length` framing, JSON-RPC 2.0 request/error
handling, initialize/initialized/shutdown/exit state transitions, position
encoding negotiation, bounded opaque workspace folders, and stdio purity.

The parent `LSP-2101` task remains `BlockedSpec` because its broader CLI command
model and the remaining editor protocol surfaces are not yet accepted as a
single 1.0 contract. Document synchronization, diagnostics, edits, semantic
transactions, snapshots, cancellation, and filesystem resolution are not
implemented here.

## Normative clauses covered

- `RFC-0004` §§1–5: command spelling, framing limits, lifecycle state machine,
  initialization fields/result, deterministic JSON-RPC errors, and channel
  separation.
- `DEC-0029`: first-supported position encoding with UTF-16 fallback; source
  projection remains owned by `ling-source`.

## Evidence

- `crates/ling-lsp/tests/lifecycle.rs` exercises framed transcripts, UTF-8 and
  fallback negotiation, Unicode workspace metadata, malformed JSON/headers,
  pre-initialize requests, and early exit.
- `crates/ling-cli/tests/lsp.rs` starts the real `ling lsp --stdio` binary and
  proves stdout framing, shutdown status, and stderr purity; CLI parser tests
  require exactly `lsp --stdio` and reject formatter options.
- `PROTO-LSP-LIFECYCLE` records the Preview/current-writer-only compatibility
  boundary; no JSON schema or Ling diagnostic allocation is introduced.

## Compatibility and determinism

The slice changes no `.ling` syntax, compiler semantics, diagnostics, Semantic
IDs, bytecode, runtime, ABI, or Unicode tables. Headers are CRLF-only, frames
and workspace counts are bounded, response keys are serialized deterministically,
request IDs are preserved, and URI values never become host paths or identity.

## Verification

The implementation commit passed `cargo test --workspace --all-targets --locked
--offline`, workspace clippy with `-D warnings`, `cargo fmt --all -- --check`,
and the governance, support, schema, traceability, status, and diff checks. The
parent `LSP-2101` remains `BlockedSpec`; this report closes only the independently
authorized lifecycle/transport slice.
