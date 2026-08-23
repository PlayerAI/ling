# RC-6901-CURRENT-EVIDENCE Implementation Report

## Result

The bounded child corrects stale RC0 task/protocol facts and makes them
executable evidence. `cargo xtask rc0 verify` now composes the authoritative
status and protocol validators, then requires the RC0 matrix statements to
match their validated summaries.

All eight RC0 criteria remain `BlockedSpec`. The result is not a release
freeze, candidate identity, artifact rehearsal, or Ling 1.0 readiness claim.

## Authority and boundary

- Accepted authority: `docs/decisions/0245-current-rc0-registry-evidence.md`.
- Earlier bounded gate: Accepted `DEC-0052`.
- Parent authority audit: `docs/status/RC-6901-AUTHORITY-AUDIT.md`.
- The implementation is internal, deterministic, read-only, and offline.

## Implementation

- `tools/xtask/src/rc0_freeze.rs` composes the status/protocol validators and
  compares the matrix with both current summaries.
- A focused negative test rejects stale task and protocol statements with
  `GOV-RC0-FREEZE-0011`.
- `tools/xtask/src/main.rs` reports two current-evidence checks.
- `docs/testing/RC0-INTERNAL-FREEZE.md` carries corrected current facts while
  retaining every blocked state and required exit.

## Verification

- `cargo test -p xtask rc0_freeze --locked --offline`
- `cargo xtask rc0 verify`
- `cargo xtask status verify`
- `cargo xtask governance check-protocols`
- Repository-wide locked/offline tests, Clippy, CI, governance, support,
  traceability, formatting, and deterministic diff checks before completion.

## Compatibility and deferrals

No Ling semantics, diagnostics, schema, Semantic ID, package, dependency,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0 data, protocol state, support
state, or public API changes. No migration is required.

RC0 candidate identity/change control, Stable protocols, final support scope,
P0/P1 sign-off, historical corpus, security sign-off, artifacts, complete 1.0
documentation, independent verification, and the freeze itself remain
deferred.
