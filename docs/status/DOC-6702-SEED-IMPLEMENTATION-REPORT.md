# DOC-6702-SEED Implementation Report

## Result

The bounded Seed child of `DOC-6702` adds an internal example-matrix drift
gate. `cargo xtask examples verify` validates the seven two-layer requirement
rows and seven `FTR-SEED-0001` through `FTR-SEED-0007` traceability rows in
`docs/testing/EXAMPLE-COVERAGE.md`, including non-empty evidence cells.

The parent `DOC-6702` remains `BlockedSpec`. This child does not run examples,
add syntax, define APIs, or promote Experimental/Preview output to Stable.

## Authority and boundary

- Accepted authority: `docs/decisions/0046-seed-example-matrix-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:406-415`.
- The verifier is inventory-only and emits internal `GOV-EXAMPLES-MATRIX-*`
  failures.
- Existing examples, Semantic/Audit output, and conformance fixtures remain
  evidence within their accepted Seed boundaries.

## Implementation

- `tools/xtask/src/examples_matrix.rs` extracts both matrix sections, rejects
  duplicate, missing, unexpected, or malformed rows, checks evidence cells,
  and preserves anti-placeholder and experimental-boundary policy text.
- `tools/xtask/src/main.rs` exposes `cargo xtask examples verify` with truthful
  usage text.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the check in
  the governance-authority gate.
- `docs/testing/EXAMPLE-COVERAGE.md` documents the command without changing
  example source or observed outputs.

## Verification

- `cargo xtask examples verify` — seven two-layer requirements and seven
  feature traceability rows.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including deterministic and requirement-drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No example, profile, ownership rule,
migration promise, security claim, or placeholder API is added. Stable feature
support, future capabilities, broader profiles, package/editor tooling, and
release example policy remain deferred to later Accepted authority.
