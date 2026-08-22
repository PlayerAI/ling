# RC-6902-SEED Implementation Report

## Result

The bounded Seed child of `RC-6902` adds an internal RC1 inventory-drift gate.
`cargo xtask rc1 verify` validates the exact nine criteria in
`docs/testing/RC1-PUBLIC-VALIDATION.md`, their documented blocked,
unsupported, and partial states, the no-publication boundary, and eight linked
release-audit marker files.

The parent `RC-6902` remains `BlockedSpec`. This child does not execute public
validation, publish or download an artifact, install a package, verify a
signature, run migration, manage issues, or promote any feature or protocol.

## Authority and boundary

- Accepted authority: `docs/decisions/0053-seed-rc1-public-validation-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:490-500`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-RC1-VALIDATION-*` failures.
- Seed release, RC0, support, security, compatibility, and editor evidence
  remain evidence of their existing boundaries, not public RC1 readiness.

## Implementation

- `tools/xtask/src/rc1_validation.rs` extracts the nine RC1 rows, rejects
  duplicate/missing/unexpected criteria and state drift, checks the explicit
  no-publication policy, rejects stale legacy names, and validates eight linked
  audit-marker files.
- `tools/xtask/src/main.rs` exposes `cargo xtask rc1 verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/RC1-PUBLIC-VALIDATION.md` records the inventory command without
  presenting it as public release validation.

## Verification

- `cargo xtask rc1 verify` — nine criteria (4 BlockedSpec, 2 Unsupported, 3
  partial) and eight audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including criterion/state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No public artifact, package,
installer, extension, signature, SBOM, provenance record, issue form,
migration executable, network request, system configuration, or placeholder
release API is created. Public RC1 validation, acquisition, migration,
schema-reset, issue ownership, and Stable-support evidence remain deferred to
later Accepted authorities.
