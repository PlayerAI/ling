# RC-6903-SEED Implementation Report

## Result

The bounded Seed child of `RC-6903` adds an internal independent-verification
readiness gate. `cargo xtask rc3 verify` validates the exact seven checks in
`docs/testing/RC3-INDEPENDENT-VERIFICATION.md`, their documented `BlockedSpec`
or partial Seed states, the no-independent-sign-off boundary, and seven linked
release-audit marker files.

The parent `RC-6903` remains `BlockedSpec`. This child does not build a tag,
verify an artifact, contact an independent reviewer, create an evidence bundle,
or make a Go/No-Go decision.

## Authority and boundary

- Accepted authority: `docs/decisions/0054-seed-rc3-independent-verification-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:502-512`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-RC3-VERIFICATION-*` failures.
- Seed self-validation, RC0/RC1 audits, security controls, and support data
  remain evidence of their existing boundaries, not independent sign-off.

## Implementation

- `tools/xtask/src/rc3_verification.rs` extracts the seven RC3 rows, rejects
  duplicate/missing/unexpected checks and state drift, checks the explicit
  no-independent-sign-off policy, rejects stale legacy names, and validates
  seven linked audit-marker files.
- `tools/xtask/src/main.rs` exposes `cargo xtask rc3 verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/RC3-INDEPENDENT-VERIFICATION.md` records the readiness command
  without presenting it as independent verification.

## Verification

- `cargo xtask rc3 verify` — seven checks (3 BlockedSpec, 4 partial) and seven
  audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including check/state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No tag, artifact, evidence bundle,
reviewer identity, signature, issue status, network request, system
configuration, or placeholder release API is created. Independent verification,
candidate comparison, reviewer sign-off, and all Stable-release claims remain
deferred to later Accepted authorities.
