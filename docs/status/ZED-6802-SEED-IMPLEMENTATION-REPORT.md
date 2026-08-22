# ZED-6802-SEED Implementation Report

## Result

The bounded Seed child of `ZED-6802` adds an internal discovery/acquisition
inventory drift gate. `cargo xtask lsp verify` validates the exact four
priority sources and their `Not established`/`Unavailable` states in
`docs/testing/LSP-DISCOVERY-ACQUISITION.md`, then checks the future security
and no-placeholder boundary markers.

The parent `ZED-6802` remains `BlockedSpec`. This child does not search PATH,
read settings, contact a registry, download or execute a binary, install a
package, or define a public LSP/Zed protocol.

## Authority and boundary

- Accepted authority: `docs/decisions/0049-seed-lsp-discovery-inventory-gate.md`.
- Planning checklist: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:442-459`.
- The verifier is documentation/inventory-only and emits internal
  `GOV-LSP-DISCOVERY-*` failures.
- The existing Preview `ling lsp --stdio` lifecycle and `UNSUP-LSP-EDITOR`
  support record remain authoritative for current editor boundaries.

## Implementation

- `tools/xtask/src/lsp_discovery.rs` extracts the four priority rows, rejects
  duplicate/missing/unexpected sources and state drift, checks non-empty cells,
  enforces future HTTPS/version/integrity/installation/offline/redaction/
  process markers, and rejects stale legacy names.
- `tools/xtask/src/main.rs` exposes `cargo xtask lsp verify` with truthful
  usage text and deterministic summary output.
- `tools/xtask/src/ci.rs` and `.github/workflows/ci.yml` place the command in
  the governance-authority gate.
- `docs/testing/LSP-DISCOVERY-ACQUISITION.md` and its authority audit now
  distinguish the implemented Preview lifecycle from unavailable acquisition
  behavior.

## Verification

- `cargo xtask lsp verify` — four priority sources, two unavailable, and two
  not-established states.
- `cargo xtask ci verify` — CI contract includes the command.
- `cargo test -p xtask --all-features --locked --offline` — focused tests,
  including source/state, security-marker, and stale-name drift cases.
- Workspace governance, traceability, status, formatting, Clippy, and locked
  offline workspace tests are required before completion is recorded.

## Compatibility and deferrals

The child changes no Ling syntax or semantics, Checked Core, runtime, bytecode,
diagnostics, schemas, Semantic IDs, package/lock behavior, dependencies,
Unicode 17.0.0 data, or public protocol. No discovery setting, executable,
release URL, checksum/signature registry, installer, migration promise,
network request, system configuration, or placeholder public API is added.
Executable selection, provenance, acquisition, offline installation, secure
process handling, Zed packaging, and G6 release evidence remain deferred to
later Accepted authority.
