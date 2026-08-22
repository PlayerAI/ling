# REL-6603-SEED Implementation Report

## Result

The bounded Seed child of `REL-6603` adds an internal security-audit matrix
drift gate. `cargo xtask security verify` validates the nine rows in
`docs/testing/SECURITY-AUDIT.md`: three Covered variants, two Partial rows,
and four Deferred rows. It also protects the policy text that keeps absent
threat-model and release evidence explicit.

The parent `REL-6603` remains `BlockedSpec`. This child does not constitute a
vulnerability assessment, penetration test, advisory scan, or G6 security
sign-off.

## Authority and boundary

- Accepted authority: `docs/decisions/0043-seed-security-matrix-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:354-366`.
- The verifier is inventory-only and emits internal `GOV-SECURITY-*` failures.
- No threat model, security protocol, FFI, sandbox, remote, replay/evidence,
  device, editor updater, SBOM schema, advisory result, or public API is added.

## Implementation

- `tools/xtask/src/security.rs` parses the Markdown table, rejects duplicate,
  missing, unexpected, or state-drifted rows, and checks the required policy
  phrases.
- `tools/xtask/src/main.rs` exposes the internal command and truthful usage
  text.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the check in
  the existing Seed reproducibility gate.
- `docs/testing/SECURITY-AUDIT.md` documents the command and preserves the
  explicit Future/Unsupported boundary.

## Verification

- `cargo xtask security verify` — nine surfaces (3 Covered, 2 Partial,
  4 Deferred).
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including deterministic and state-drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. Existing security controls remain
evidence for implemented Seed boundaries only. FFI/TCB, archive/build
sandboxing, remote authentication/provenance, replay/evidence privacy,
editor-binary trust, advisory/license/SBOM/provenance, threat-model, and
incident/disclosure work remain deferred to later Accepted authority.
