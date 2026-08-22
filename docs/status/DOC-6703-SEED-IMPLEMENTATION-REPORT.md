# DOC-6703-SEED Implementation Report

## Result

The bounded Seed child of `DOC-6703` adds an internal bilingual-tutorial
coverage drift gate. `cargo xtask tutorial verify` validates the exact two
tutorial sources and eight requirement rows in
`docs/testing/TUTORIAL-COVERAGE.md`, then checks the bilingual tutorial and
source markers without executing programs.

The parent `DOC-6703` remains `BlockedSpec`. This child does not add syntax,
localized aliases, APIs, protocols, or Stable support claims.

## Authority and boundary

- Accepted authority: `docs/decisions/0047-seed-bilingual-tutorial-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:417-426`.
- The verifier is documentation/inventory-only and emits internal
  `GOV-TUTORIAL-MATRIX-*` failures.
- Existing process-level tests, conformance fixtures, and Semantic/Audit
  output remain the authority for observed behavior and protocol evidence.

## Implementation

- `tools/xtask/src/tutorial_matrix.rs` validates source/output labels,
  non-empty evidence cells, tutorial headings and boundary markers, source
  markers, and anti-promotion policy text.
- `tools/xtask/src/main.rs` exposes `cargo xtask tutorial verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/TUTORIAL-COVERAGE.md` records the bilingual source and
  requirement inventory; `docs/TUTORIAL.md` links the verification command.

## Verification

- `cargo xtask tutorial verify` — two bilingual sources and eight requirements.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including source, boundary, and requirement-drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. The existing Chinese and English
sources remain Seed evidence; no localization policy, profile, ownership rule,
migration promise, security claim, or placeholder public API is added. Stable
tutorial content and G6 release evidence remain deferred to later Accepted
authority.
