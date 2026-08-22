# LSP-2102-NEGOTIATION implementation report

> Status: Done / 已完成 (bounded Preview slice)
> Task: `LSP-2102-NEGOTIATION`
> Authority: Accepted `RFC-0004` and `DEC-0029`

## Scope

This slice records and verifies the initialize-time position-encoding boundary
already implemented by the Preview LSP lifecycle. It selects the first client
label supported by Ling (`utf-8`, `utf-16`, or `utf-32`), falls back to
`utf-16` for an absent or empty list, and returns the selected wire label in
`capabilities.positionEncoding`.

The parent `LSP-2102` remains `BlockedSpec` for document/snapshot versions,
stale results, diagnostics and Workspace Edit projection, cancellation, and
Stable versus Experimental transaction compatibility.

## Normative clauses covered

- RFC-0004 §§2–4: initialize capability parsing, deterministic response
  fields, lifecycle gating, and JSON-RPC invalid-parameter behavior.
- DEC-0029 §§1–3: supported encoding labels, first-supported negotiation,
  UTF-16 fallback, and source-layer ownership of positions.
- DEC-0002: original UTF-8 byte spans remain authoritative and are not
  replaced by editor positions.

## Implementation and evidence

- `crates/ling-lsp/src/lib.rs` stores the negotiated `PositionEncoding`, parses
  `capabilities.general.positionEncodings`, and emits the selected wire label.
- `crates/ling-source/src/position.rs` owns the encoding enum, deterministic
  negotiation helper, and strict SourceMap projection; it does not expose
  host paths, revisions, or document identity.
- `crates/ling-lsp/tests/position_encoding.rs` covers first-supported
  selection, unknown-label filtering, absent/empty fallback, malformed
  metadata rejection before lifecycle transition, and response capability
  projection.
- Existing lifecycle and source-map fixtures cover Unicode, BOM, CRLF,
  combining marks, supplementary characters, and invalid character boundaries.

## Compatibility and determinism

No language syntax, compiler semantics, diagnostics registry, Semantic IDs,
bytecode, VM, CLI command set, package identity, or Unicode 17.0.0 data
changed. Negotiation is process-local and deterministic; client order is the
only selection input, and unknown labels never change the UTF-16 fallback.

## Verification

The focused negotiation tests and repository gates passed after implementation
commit `39755afad13db66b429967fe61f20f66a4aea699`:

- `cargo test -p ling-lsp --test position_encoding --locked --offline`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings`
- `cargo test --workspace --locked --offline --quiet`
- `xtask governance check-all`, `xtask status verify`, `xtask ci verify`, and
  `xtask support verify`
- `git diff --check`

## Deferred work

Incremental/range edits, document URI mapping, version and snapshot
preconditions, stale-result and cancellation policy, diagnostics, Workspace
Edits, and Semantic Transactions remain governed by the parent LSP/semantic
protocol gaps. No Stable 1.0 editor claim is made.
