# LSP-2201 implementation report

> Status: implementation complete; registry binding pending
> Task: `LSP-2201`
> Authority: Accepted `RFC-0031`, `DEC-0001`, `DEC-0002`, `DEC-0029`,
> `DEC-0034`, and `DEC-0072`

## Scope

This milestone implements the deterministic, pure in-process conversion from
registered Ling compiler diagnostics to LSP `Diagnostic` JSON values. It does
not publish diagnostics, associate results with mutable document versions, or
apply repairs.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2201: map existing
  compiler diagnostic identity, messages, severity, primary/related positions,
  and structured repair metadata for editor consumption.
- `RFC-0031` §§1–5: exact source identity, complete input validation, field
  mapping, negotiated position projection, canonical order, determinism, typed
  failures, and explicit publication exclusions.
- `DEC-0001`, `DEC-0002`, `DEC-0029`, `DEC-0034`, and `DEC-0072`: registered
  codes, original-byte authority, strict position conversion, and path-free
  deterministic ordering.

## Implementation

- `ling-diagnostics` exposes read-only Chinese/English message and Semantic ID
  accessors; its existing `ling.diagnostic/0.1` serialization is unchanged.
- `ling-lsp` adds explicit source, related-label, adapter-input, output, and
  typed-error values plus `adapt_diagnostics`.
- The adapter validates the whole source/input set before returning output,
  projects through the caller-selected encoding, preserves structured fields,
  and sorts by the Accepted DEC-0034 key.
- `PROTO-LSP-DIAGNOSTIC` registers Experimental
  `ling.lsp.diagnostic/0.1` in the protocol inventory and support matrix.

## Tests and evidence

`crates/ling-lsp/tests/diagnostic_adapter.rs` covers exact JSON shape,
Error/Warning/Note mapping, multiple repairs, Semantic IDs, related sources,
UTF-8/16/32 Chinese/emoji/combining-mark/BOM/CRLF projection, canonical order,
repeatable serialization, source-set failures, missing/unknown spans, reversed
and out-of-range offsets, scalar/surrogate/CRLF interiors, and invalid-later-
input atomicity.

## Compatibility and determinism

- Adds current-writer-only Experimental `ling.lsp.diagnostic/0.1`; no previous
  adapter version or migration exists.
- No diagnostic code, severity, message, repair, core diagnostic JSON schema,
  Ling syntax/semantics, Typed Core, runtime, bytecode, VM, ABI, or Unicode
  17.0.0 table changes.
- Output exposes only logical Ling URIs and deterministic JSON values; host
  paths, allocation, hash-map order, timestamps, and debug text are absent.

## Verification

The exact implementation commit will be bound here and in
`docs/status/implementation-status.toml` after the focused and full locked
offline repository gates pass.

## Intentionally deferred

`publishDiagnostics`, pull diagnostics, document/snapshot versions, debounce,
cancellation, stale-result handling, replacement/clearance, deduplication,
root-cause/error-storm caps, suppression, tags, immutable code-description
URLs, Workspace Edits, Semantic Transactions, and Stable compatibility remain
owned by LSP-2202 through LSP-2205 and future Accepted authorities.
