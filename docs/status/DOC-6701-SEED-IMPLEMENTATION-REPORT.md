# DOC-6701-SEED Implementation Report

## Result

The bounded Seed child of `DOC-6701` adds an internal documentation-inventory
drift gate. `cargo xtask docs verify` validates the twelve formal-set manuals
in `docs/testing/DOCUMENTATION-INVENTORY.md`, including four explicit
`Future / Unsupported` rows and the remaining bounded Seed/Preview/Partial
states.

The parent `DOC-6701` remains `BlockedSpec`. This child does not generate
manuals, add examples, define syntax or protocols, or claim future support.

## Authority and boundary

- Accepted authority: `docs/decisions/0045-seed-documentation-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:389-404`.
- The verifier is inventory-only and emits internal `GOV-DOCS-MATRIX-*`
  failures.
- Future manuals remain planning/status evidence until their specifications,
  implementations, support, and release evidence are Accepted.

## Implementation

- `tools/xtask/src/documentation_matrix.rs` extracts the Formal set section,
  rejects duplicate, missing, unexpected, or state-drifted rows, and checks
  anti-promotion and stale-name policy text across Markdown line wrapping.
- `tools/xtask/src/main.rs` exposes `cargo xtask docs verify` with truthful
  usage text.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the check in
  the governance-authority gate.
- `docs/testing/DOCUMENTATION-INVENTORY.md` documents the command without
  changing the formal manual set.

## Verification

- `cargo xtask docs verify` — twelve manuals (4 Future / Unsupported).
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including deterministic and state-drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No documentation generator, example,
command, protocol, migration promise, security claim, or placeholder API is
added. Future Task/Actor/Replay, Native/FFI, Kernel/Device, Critical/Contract/
Evidence, LSP/Zed, migration, and security/disclosure manuals remain deferred.
