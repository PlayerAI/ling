# REL-6604-SEED Implementation Report

## Result

The bounded Seed child of `REL-6604` adds an internal performance-matrix drift
gate. `cargo xtask performance verify` validates the twelve planned
measurements in `docs/testing/PERFORMANCE-BASELINE.md`: two Covered variants,
two Partial rows, and eight Deferred rows. The memory measurement remains
Deferred because no Accepted resource policy exists.

The parent `REL-6604` remains `BlockedSpec`. This child does not run timing
code, freeze a regression threshold, or constitute cross-host or G6
performance evidence.

## Authority and boundary

- Accepted authority: `docs/decisions/0044-seed-performance-matrix-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:368-387`.
- The verifier is inventory-only and emits internal `GOV-PERF-MATRIX-*`
  failures.
- Existing `cargo xtask performance baseline` remains opt-in measurement
  evidence; its JSON observations are not converted into language semantics or
  release thresholds.

## Implementation

- `tools/xtask/src/performance_matrix.rs` extracts the Plan coverage section,
  rejects duplicate, missing, unexpected, or state-drifted rows, and checks
  the measurement-boundary policy phrases.
- `tools/xtask/src/main.rs` exposes `cargo xtask performance verify` with
  truthful usage text.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the check in
  the existing Seed reproducibility gate.
- `docs/testing/PERFORMANCE-BASELINE.md` records memory as Deferred and
  documents the internal command without changing timing evidence.

## Verification

- `cargo xtask performance verify` — twelve measurements (2 Covered,
  2 Partial, 8 Deferred).
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including deterministic and state-drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No benchmark dependency, threshold,
memory/IO claim, unsupported backend/editor harness, or placeholder API is
added. Hardware tiers, warm-up/sample/variance policy, package-build/LSP/
Native/Actor/Replay/device/Kernel/Zed measurements, regression ownership, and
release thresholds remain deferred to later Accepted performance authority.
