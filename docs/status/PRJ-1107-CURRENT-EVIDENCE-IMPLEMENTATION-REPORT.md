# PRJ-1107-CURRENT-EVIDENCE implementation report

## Result

The bounded internal project-surface evidence gate is implemented under
Accepted DEC-0250. `cargo xtask project verify` now proves the current
three-slice implementation boundary and fails closed if the blocked public
surface is silently promoted. The parent `PRJ-1107` remains `BlockedSpec`.

Implementation commit: `0822280f4a2ea2c9e4205e8fbff104a00b522ef3`.

## Normative clauses covered

- Accepted RFC-0024 §§1–9 authorize only the explicit locked graph-check CLI
  and its Experimental `ling.project.check/0.1` result.
- Accepted DEC-0058 authorizes only the read-only `LockedProject` snapshot.
- Accepted DEC-0083 authorizes only the internal locked-project semantic
  snapshot query.
- Accepted DEC-0250 authorizes this evidence composition and requires the five
  unresolved public surfaces to remain `BlockedSpec`.

## Implementation and tests

- `docs/testing/PROJECT-CLI-STATUS.md` records exactly one Experimental, two
  Internal, and five `BlockedSpec` surfaces.
- `tools/xtask/src/project_status.rs` validates the exact matrix, twelve
  implementation/test/report files, and four parent/child task states.
- Focused unit tests reject matrix-state drift, parent promotion, missing
  child status, and missing implementation evidence.
- The command is wired into the CLI usage and the always-on
  `governance-authority` CI contract.

## Compatibility, determinism, and Unicode

The verifier is deterministic, read-only, path-independent, and offline. It
adds no language behavior, public diagnostic, schema, Semantic ID, package
protocol, dependency, CLI/LSP/DAP/runtime surface, bytecode, VM, ABI, artifact,
network behavior, or Unicode 17.0.0 change. It does not execute project code.

## Intentionally deferred

Public semantic project checking, compiler-host lifecycle, run/test/build,
entry and capability policy, workspace/member selection, implicit discovery,
artifact production, and public result schemas remain blocked by
`GAP-PROJECT-CLI-INTERFACE-001`.
