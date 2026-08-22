# PRJ-1107 Authority Audit: Project API and CLI Integration

## Outcome

PRJ-1107 remains `BlockedSpec` for the full project API and CLI surface. The
accepted RFC-0024 decision authorizes and the completed child
`PRJ-1107-CHECK` implements one bounded Preview slice: an explicit, locked,
offline project graph check. Accepted DEC-0058 and the completed child
`PRJ-1107-LOAD` add a read-only locked-project snapshot boundary around the
same graph/lock APIs. These children do not promote the parent task to `Done`
because semantic project checking, run/test/build behavior, workspace
selection, and artifact policy still lack accepted contracts.

The children reuse the accepted RFC-0002 `ling-project` manifest, module
discovery, lockfile, and package-graph APIs. They do not duplicate package
resolution or add a placeholder `CompilerHost` API. Existing file-oriented
`ling` commands remain unchanged outside `ling project check`.

## Normative traceability

- Accepted RFC-0002 §1 and §§2–14 fix explicit `ling.toml` selection, the
  deterministic local package graph, package identities, and `ling.lock/1`
  library protocols. They do not define the complete project CLI surface.
- Accepted RFC-0024 §§1–9 authorize only `ling project check --manifest-path
  PATH --locked [--format human|json]`: explicit-root selection, local locked
  graph validation, path-free bilingual human output, the Experimental
  `ling.project.check/0.1` JSON report, and no lock/network/artifact writes.
- Accepted DEC-0003 fixes the current M0 CLI parser and command baseline;
  RFC-0024 is the additional authority for the nested project-check command.
- Accepted DEC-0013 supplies existing main/runtime failure categories and
  exit semantics; RFC-0024 deliberately reuses existing diagnostics and does
  not allocate a new code range.
- Accepted DEC-0058 authorizes only the in-process `LockedProject` snapshot and
  `load_locked_project` read-only locked boundary; it does not define a
  compiler host, workspace, run/test/build, artifact, CLI, or protocol API.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the executable name to
  `ling`; stale `zero` spellings in lower-authority execution inputs are not
  implementation authority.

## Specification gap and remaining parent scope

`GAP-PROJECT-CLI-INTERFACE-001` remains open for the parent task's unresolved
surface:

- semantic project `check` and its relationship to the checked compiler;
- project `run`, `test`, and `build` semantics and artifact scope;
- workspace/member selection, manifest discovery beyond an explicit root, and
  `--locked`/`--offline` behavior outside RFC-0024;
- process-exit mapping and machine-readable result contracts for those
  commands; and
- any `CompilerHost` or build-artifact API.

The RFC-0024 child does not choose among those alternatives through code.

## Evidence and compatibility

The child implementations are recorded in
`docs/status/PRJ-1107-CHECK-IMPLEMENTATION-REPORT.md` and
`docs/status/PRJ-1107-LOAD-IMPLEMENTATION-REPORT.md`. The existing command is
covered by `crates/ling-cli/tests/project_check.rs` and the protocol evidence
under `tests/protocols/project-check/`; the snapshot is covered by
`crates/ling-project/tests/locked_project.rs`. The command still requires
exactly one explicit `ling.toml` path and exactly one `--locked`, while the
snapshot is read-only, path-free, and deterministic. The protocol inventory
and support matrix register `ling.project.check/0.1` as Experimental only.

No language semantics, source spans, Semantic IDs, bytecode, VM behavior,
Unicode tables, existing diagnostic allocations, or stable public schemas
changed. The JSON report is current-writer-only and intentionally not
canonical. The implementation is local/offline and does not claim network,
workspace, artifact, or performance behavior.

## Intentionally deferred

The parent PRJ-1107 task remains blocked until accepted decisions define the
remaining project CLI/API surface. FMT-1507 and the LSP/transaction work have
separate registered authority gaps; this child does not combine their
protocol choices or create a shared placeholder service.
