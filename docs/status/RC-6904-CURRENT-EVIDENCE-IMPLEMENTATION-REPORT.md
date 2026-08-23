# RC-6904-CURRENT-EVIDENCE Implementation Report

## Result

The bounded child corrects the RC2 protocol total and composes the current
predecessor inventory chain. `cargo xtask rc2 verify` now executes RC3→RC1→RC0
and requires current upstream/protocol boundary markers.

All six RC2 evidence-class states remain unchanged. No blocker fix, candidate,
source freeze, or Final decision is authorized.

## Authority and boundary

- Accepted authority: `docs/decisions/0248-current-rc2-boundary-evidence.md`.
- Earlier bounded gate: Accepted `DEC-0055`.
- Current upstream authority: Accepted `DEC-0247`.
- Parent authority audit: `docs/status/RC-6904-AUTHORITY-AUDIT.md`.

## Implementation

- `tools/xtask/src/rc2_change_control.rs` composes RC3→RC1→RC0 and validates
  three current-boundary markers.
- A focused negative test rejects stale upstream/protocol evidence with
  `GOV-RC2-CHANGE-CONTROL-0011`.
- `tools/xtask/src/main.rs` reports the composed upstream gate.
- `docs/testing/RC2-FINAL-CHANGE-CONTROL.md` reports 27 protocols and preserves
  the blocker-only/no-claim boundary.

## Verification

- `cargo test -p xtask rc2_change_control --locked --offline`
- `cargo xtask rc2 verify`
- `cargo xtask rc3 verify`
- `cargo xtask rc1 verify`
- `cargo xtask rc0 verify`
- `cargo xtask governance check-protocols`
- Repository-wide locked/offline tests, Clippy, CI, governance, support,
  status, traceability, formatting, and deterministic diff checks.

## Compatibility and deferrals

No Ling semantics, diagnostics, schema, Semantic ID, package, dependency,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0 data, protocol state, support
state, or public API changes. No migration is required.

Blocker taxonomy/disposition, candidate baseline, regression/risk/impact
records, matrix rerun, immutable candidate, reviewer approval, regeneration,
and Final/Go remain deferred.
