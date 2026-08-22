# RC-6905-SEED Implementation Report

## Result

The bounded Seed child of `RC-6905` adds an internal v1.0 release-artifact
inventory gate. `cargo xtask v1 verify` validates the exact fourteen items in
`docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md`, their documented partial,
unavailable, unsupported, draft, experimental, preview, and blocked states,
the immutable-Seed/no-publication boundary, and nine linked audit-marker files.

The parent `RC-6905` remains `BlockedSpec`. This child does not build, sign,
upload, download, install, or advertise a v1.0 artifact and does not promote
any capability to Stable.

## Authority and boundary

- Accepted authority: `docs/decisions/0056-seed-v1-release-artifact-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:524-542`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-V1-ARTIFACT-*` failures.
- The v0.0.1 Seed tag/report and RC0–RC4 audits remain historical or blocked
  evidence, not a v1.0 artifact manifest.

## Implementation

- `tools/xtask/src/v1_artifact_inventory.rs` extracts the fourteen release
  rows, rejects duplicate/missing/unexpected items and state drift, checks the
  immutable-Seed/no-publication policy, rejects stale legacy names, and
  validates nine linked audit-marker files.
- `tools/xtask/src/main.rs` exposes `cargo xtask v1 verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md` records the inventory command
  without presenting it as a release manifest or publication operation.

## Verification

- `cargo xtask v1 verify` — fourteen items (5 partial, 2 unavailable,
  2 unsupported, 1 preview/not packaged, 1 draft, 1 experimental/preview/
  future, 2 BlockedSpec) and nine audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No v1.0 tag, artifact, installer,
extension, language server, signature, SBOM, provenance record, migration
executable, evidence bundle, network request, system configuration, or
placeholder release API is created. Publication, Stable support, and all
artifact/acquisition evidence remain deferred to later Accepted authorities.
