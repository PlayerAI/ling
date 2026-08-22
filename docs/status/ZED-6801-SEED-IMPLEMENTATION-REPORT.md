# ZED-6801-SEED Implementation Report

## Result

The bounded Seed child of `ZED-6801` adds an internal compatibility-matrix
drift gate. `cargo xtask zed verify` validates the exact ten compatibility
surfaces and states in `docs/testing/ZED-COMPATIBILITY-MATRIX.md`, plus five
locked Tree-sitter package evidence files and their markers.

The parent `ZED-6801` remains `BlockedSpec`. This child does not run npm or
Tree-sitter, add an extension, define LSP/Zed behavior, or claim Stable editor
support.

## Authority and boundary

- Accepted authority: `docs/decisions/0048-seed-zed-compatibility-matrix-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:430-440`.
- The verifier is documentation/package-inventory-only and emits internal
  `GOV-ZED-MATRIX-*` failures.
- Compiler validity, diagnostics, source spans, and Ling semantics remain
  authoritative over the editor-only Tree-sitter grammar and queries.

## Implementation

- `tools/xtask/src/zed_matrix.rs` extracts the ten matrix rows, rejects
  duplicate/missing/unexpected rows and state drift, preserves unsupported and
  no-placeholder policy text, and checks package/lock/tree-sitter/README/
  known-difference markers.
- `tools/xtask/src/main.rs` exposes `cargo xtask zed verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/ZED-COMPATIBILITY-MATRIX.md` records original UTF-8 byte-span
  ownership and keeps the Windows cache-lock limitation explicit.

## Verification

- `cargo xtask zed verify` — ten surfaces and five package evidence files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including surface/state and package-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No Zed extension, LSP executable,
binary acquisition, installer, migration promise, security claim, or
placeholder public API is added. Stable editor support and G6 release
evidence remain deferred to later Accepted authority.
