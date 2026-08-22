# RC-6904-SEED Implementation Report

## Result

The bounded Seed child of `RC-6904` adds an internal RC2/final change-control
inventory gate. `cargo xtask rc2 verify` validates the exact six evidence
classes in `docs/testing/RC2-FINAL-CHANGE-CONTROL.md`, their documented
`BlockedSpec` or partial Seed states, the blocker-only/no-claim boundary, and
seven linked release-audit marker files.

The parent `RC-6904` remains `BlockedSpec`. This child does not approve a
blocker, classify a change, create a candidate, run a matrix, or make a Final
or Go decision.

## Authority and boundary

- Accepted authority: `docs/decisions/0055-seed-rc2-final-change-control-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:514-522`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-RC2-CHANGE-CONTROL-*` failures.
- RC0/RC1/RC3 audits, support and protocol registries, and Seed tests remain
  evidence of their existing boundaries, not RC2 approval.

## Implementation

- `tools/xtask/src/rc2_change_control.rs` extracts the six RC2 rows, rejects
  duplicate/missing/unexpected evidence classes and state drift, checks the
  explicit blocker-only/no-claim policy, rejects stale legacy names, and
  validates seven linked audit-marker files.
- `tools/xtask/src/main.rs` exposes `cargo xtask rc2 verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/RC2-FINAL-CHANGE-CONTROL.md` records the inventory command
  without presenting it as a change approval or release operation.

## Verification

- `cargo xtask rc2 verify` — six evidence classes (5 BlockedSpec, 1 partial)
  and seven audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No source fix, blocker status, risk
acceptance, candidate, tag, artifact, reviewer decision, network request,
system configuration, or placeholder release API is created. Blocker approval,
candidate regeneration, RC2 change acceptance, and Final/Go claims remain
deferred to later Accepted authorities.
