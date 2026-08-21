# LSP-2101-LIFECYCLE implementation report

> Status: In Progress / 实施中
> Task: `LSP-2101-LIFECYCLE`
> Authority: Accepted `RFC-0004` and `DEC-0029`

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
