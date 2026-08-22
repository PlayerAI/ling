# ZED-6803-SEED Implementation Report

## Result

The bounded Seed child of `ZED-6803` adds an internal editor-acceptance
inventory drift gate. `cargo xtask zed-extension verify` validates the exact
thirteen acceptance areas and their `Covered`, `Partial`, `Unsupported`, and
`Future` classifications in `docs/testing/ZED-EXTENSION-ACCEPTANCE.md`, then
checks nine package/query and historical TS/ZQ evidence files.

The parent `ZED-6803` remains `BlockedSpec`. This child does not run npm,
generate a parser, create a Zed manifest, start an LSP process, define editor
behavior, or claim development-install or marketplace support.

## Authority and boundary

- Accepted authority: `docs/decisions/0050-seed-zed-extension-acceptance-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:461-471`.
- The verifier is documentation/evidence-inventory-only and emits internal
  `GOV-ZED-ACCEPTANCE-*` failures.
- The compiler, Accepted specifications, conformance tests, and `ling-syntax`
  remain authoritative over the editor-only Tree-sitter grammar and queries.

## Implementation

- `tools/xtask/src/zed_extension.rs` extracts the thirteen matrix rows,
  rejects duplicate/missing/unexpected areas and state drift, checks policy
  markers, rejects stale legacy names, and validates package/query/TS/ZQ
  evidence markers.
- `tools/xtask/src/main.rs` exposes `cargo xtask zed-extension verify` with
  truthful usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/ZED-EXTENSION-ACCEPTANCE.md` and its authority audit now
  distinguish the Preview lifecycle and grammar/query evidence from absent
  full extension acceptance.

## Verification

- `cargo xtask zed-extension verify` — thirteen areas (4 covered, 4 partial,
  5 unsupported, 2 future classifications) and nine evidence files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including area/state and evidence-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No extension manifest, LSP feature,
formatter, task/runnable, Replay/Evidence UI, install path, marketplace
package, migration promise, network request, system configuration, or
placeholder public API is added. Full Zed/LSP acceptance, clean/offline
installation, provenance, and G6 release evidence remain deferred to later
Accepted authority.
