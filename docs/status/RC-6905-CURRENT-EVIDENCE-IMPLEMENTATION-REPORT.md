# RC-6905-CURRENT-EVIDENCE Implementation Report

## Result

The bounded child corrects stale LSP/protocol facts and composes the current
predecessor inventory chain. `cargo xtask v1 verify` now executes
RC2→RC3→RC1→RC0 and validates four current-boundary markers.

All fourteen artifact states remain unchanged. No v1 artifact, distribution,
Stable support, or publication is authorized.

## Authority and boundary

- Accepted authority:
  `docs/decisions/0249-current-v1-artifact-boundary-evidence.md`.
- Earlier bounded gate: Accepted `DEC-0056`.
- Current upstream authority: Accepted `DEC-0248`.
- Current LSP boundary: Accepted `DEC-0242`.

## Implementation

- `tools/xtask/src/v1_artifact_inventory.rs` composes RC2→RC0 and validates
  current LSP, protocol, upstream, and parent-blocked markers.
- A focused negative test rejects no-LSP/21-protocol/stale-upstream facts with
  `GOV-V1-ARTIFACT-0011`.
- `tools/xtask/src/main.rs` reports the composed upstream gate.
- `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md` records the source-built
  Preview server and 27 non-Stable protocols without a distribution claim.

## Verification

- `cargo test -p xtask v1_artifact_inventory --locked --offline`
- `cargo xtask v1 verify`
- `cargo xtask rc2 verify`, `rc3 verify`, `rc1 verify`, and `rc0 verify`
- `cargo xtask governance check-protocols`
- `cargo xtask lsp verify` and `cargo xtask zed-extension verify`
- Repository-wide locked/offline tests, Clippy, CI, governance, support,
  status, traceability, formatting, and deterministic diff checks.

## Compatibility and deferrals

No Ling semantics, diagnostics, schema, Semantic ID, package, dependency,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0 data, protocol state, support
state, or public API changes. No migration is required.

Every actual v1 publication item and all Stable/promotion evidence remain
deferred to their Accepted authorities and executable release gates.
