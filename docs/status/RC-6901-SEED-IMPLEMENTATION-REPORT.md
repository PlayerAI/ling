# RC-6901-SEED Implementation Report

## Result

The bounded Seed child of `RC-6901` adds an internal RC0 inventory-drift gate.
`cargo xtask rc0 verify` validates the exact eight criteria in
`docs/testing/RC0-INTERNAL-FREEZE.md`, their `BlockedSpec` states, the
no-freeze/no-publication boundary, and ten linked release-audit marker files.

The parent `RC-6901` remains `BlockedSpec`. This child does not execute a
release freeze, create a candidate identity, publish an artifact, scan
dependencies, disposition issues, or promote any feature or protocol.

## Authority and boundary

- Accepted authority: `docs/decisions/0052-seed-rc0-internal-freeze-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:479-488`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-RC0-FREEZE-*` failures.
- The v0.0.1 Seed release report, support-matrix draft, and G6 audits remain
  evidence of their existing boundaries, not a v1.0 release candidate.

## Implementation

- `tools/xtask/src/rc0_freeze.rs` extracts the eight RC0 rows, rejects
  duplicate/missing/unexpected criteria and state drift, checks the explicit
  no-freeze/no-publication policy, rejects stale legacy names, and validates
  ten linked audit-marker files.
- `tools/xtask/src/main.rs` exposes `cargo xtask rc0 verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/RC0-INTERNAL-FREEZE.md` records the inventory command without
  presenting it as a release operation.

## Verification

- `cargo xtask rc0 verify` — eight `BlockedSpec` criteria and ten audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including criterion/state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No release tag, artifact, signature,
SBOM, issue status, network request, system configuration, or placeholder
release API is created. Feature/protocol freeze, P0/P1 sign-off, historical
corpus execution, security sign-off, artifact rehearsal, and complete 1.0
documentation remain deferred to later Accepted authorities.
