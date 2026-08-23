# RC-6903-CURRENT-EVIDENCE Implementation Report

## Result

The bounded child makes RC3 readiness execute the current upstream inventory
chain. `cargo xtask rc3 verify` now composes RC1, which composes RC0, and
requires explicit pass/block/non-independence markers.

All seven RC3 states remain unchanged. This is repository self-validation, not
independent verification or a Go/No-Go decision.

## Authority and boundary

- Accepted authority: `docs/decisions/0247-current-rc3-upstream-evidence.md`.
- Earlier bounded gate: Accepted `DEC-0054`.
- Current upstream authority: Accepted `DEC-0246`.
- Parent authority audit: `docs/status/RC-6903-AUTHORITY-AUDIT.md`.

## Implementation

- `tools/xtask/src/rc3_verification.rs` composes the RC1→RC0 chain and checks
  three current-boundary markers.
- A focused negative test rejects missing markers with
  `GOV-RC3-VERIFICATION-0011`.
- `tools/xtask/src/main.rs` reports the composed upstream gate.
- `docs/testing/RC3-INDEPENDENT-VERIFICATION.md` explicitly separates passing
  bounded inventories from independent review.

## Verification

- `cargo test -p xtask rc3_verification --locked --offline`
- `cargo xtask rc3 verify`
- `cargo xtask rc1 verify`
- `cargo xtask rc0 verify`
- Repository-wide locked/offline tests, Clippy, CI, governance, support,
  status, traceability, formatting, and deterministic diff checks.

## Compatibility and deferrals

No Ling semantics, diagnostics, schema, Semantic ID, package, dependency,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0 data, protocol state, support
state, or public API changes. No migration is required.

Candidate immutability, reviewer independence, clean build, artifact/security
verification, reproduction, evidence retention, signed comparison, and
Go/No-Go remain deferred.
