# RC-6902-CURRENT-EVIDENCE Implementation Report

## Result

The bounded child corrects the stale RC1 claim that no LSP executable exists.
`cargo xtask rc1 verify` now composes the current RC0 and Zed acceptance gates
and requires explicit source-built Preview LSP, absent-extension, and blocked-
RC0 markers.

All nine RC1 criterion states remain unchanged. No public validation, Zed
extension, acquisition surface, or Ling 1.0 readiness is claimed.

## Authority and boundary

- Accepted authority: `docs/decisions/0246-current-rc1-boundary-evidence.md`.
- Earlier bounded gate: Accepted `DEC-0053`.
- Current prerequisites: Accepted `DEC-0245` and `DEC-0243`.
- Parent authority audit: `docs/status/RC-6902-AUTHORITY-AUDIT.md`.

## Implementation

- `tools/xtask/src/rc1_validation.rs` composes the RC0 and Zed acceptance
  gates and validates three current-boundary markers.
- A focused negative test rejects the former no-LSP statement with
  `GOV-RC1-VALIDATION-0011` errors.
- `tools/xtask/src/main.rs` reports two current-evidence gates.
- `docs/testing/RC1-PUBLIC-VALIDATION.md` distinguishes the existing Preview
  server from the absent Zed extension and public distribution surface.

## Verification

- `cargo test -p xtask rc1_validation --locked --offline`
- `cargo xtask rc1 verify`
- `cargo xtask rc0 verify`
- `cargo xtask zed-extension verify`
- Repository-wide locked/offline tests, Clippy, CI, governance, support,
  status, traceability, formatting, and deterministic diff checks.

## Compatibility and deferrals

No Ling semantics, diagnostics, schema, Semantic ID, package, dependency,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0 data, protocol state, support
state, or public API changes. No migration is required.

RC0, public artifacts/acquisition, security attestations, clean install, Zed
packaging, release samples, migration, issue intake, schema-reset change
control, independent validation, and RC1 approval remain deferred.
