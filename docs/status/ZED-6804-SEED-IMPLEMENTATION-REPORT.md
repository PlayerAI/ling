# ZED-6804-SEED Implementation Report

## Result

The bounded Seed child of `ZED-6804` adds an internal DAP-status inventory
drift gate. `cargo xtask dap verify` validates the exact nine DAP surfaces and
their `Unavailable`, `Future`, `Partial foundation only`, and `Unsupported`
states in `docs/testing/DAP-STATUS.md`, then checks three DAP authority-audit
marker files.

The parent `ZED-6804` remains `BlockedSpec`. This child does not run a
debugger, register DAP, expose controls, define wire fields, read settings,
contact a registry, or change runtime behavior. DAP remains intentionally
non-blocking for language and basic editor support.

## Authority and boundary

- Accepted authority: `docs/decisions/0051-seed-dap-status-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:473-475`.
- The verifier is documentation/audit-inventory-only and emits internal
  `GOV-DAP-STATUS-*` failures.
- VM control, Runtime Fault, and source-map evidence remains experimental
  library foundation, not DAP protocol or debugger semantics.

## Implementation

- `tools/xtask/src/dap_status.rs` extracts the nine matrix rows, rejects
  duplicate/missing/unexpected surfaces and state drift, checks non-blocking
  and no-control policy markers, rejects stale legacy names, and validates
  DAP-3601/3602/3603 audit markers.
- `tools/xtask/src/main.rs` exposes `cargo xtask dap verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/DAP-STATUS.md` and its authority audit now record the
  inventory gate without presenting a debugger implementation.

## Verification

- `cargo xtask dap verify` — nine surfaces (4 unavailable, 3 future, 1
  partial foundation, 1 unsupported) and three audit files.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including state and audit-marker drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No DAP adapter, debugger command,
extension registration, launch/attach API, migration promise, network request,
system configuration, or placeholder public API is added. DAP wire/lifecycle,
debug metadata, capabilities, breakpoints, values, Fault mapping, Task/Actor
views, security, platforms, cancellation, installation, and release evidence
remain deferred to later Accepted authority.
