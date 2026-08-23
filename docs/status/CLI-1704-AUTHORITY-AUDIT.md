# CLI-1704 Authority Audit: Test command

## Outcome

CLI-1704 now has sufficient Accepted authority and complete current behavior.
DEC-0256 composes DEC-0039's standalone file/directory runner and RFC-0025's
explicit locked/offline project-entry smoke test. This matches the execution
plan's requirement to use an accepted convention while test syntax is absent.

## Normative traceability

- DEC-0039 defines standalone selection, sorted discovery, independent checked
  execution, captured Console output, `ling.test/0.1`, and exit precedence.
- RFC-0025 defines explicit manifest-mode selection and exactly one isolated
  root-package entry smoke test using `ling.project.command/0.1`.
- DEC-0253 owns the single parser/dispatcher and mode selection.
- DEC-0254 governs rendering without changing selection, schemas, or exits.
- DEC-0256 accepts those two modes as the complete CLI-1704 surface and forbids
  inferred syntax or hidden conventions.

## Implementation evidence

The standalone runner recursively selects `.ling` files without following
symlinks, sorts UTF-8 logical names, runs every case sequentially through the
checked pipeline, captures output, aggregates diagnostics, and emits its report
even after case failures. Project mode performs a fresh locked/offline project
load, validates the root entry, and executes it once in an isolated console.

Parser and integration tests cover positive and negative mode selection,
deterministic ordering, continued execution, capture, compile/runtime exits,
empty discovery, schemas, path-free project identity, output policy, and input
nonmutation.

## Compatibility and deferred work

No schema, diagnostic code, exit, source syntax, type, Effect, Checked Core,
Semantic ID, span, runtime, bytecode, VM, ABI, or Unicode 17.0.0 behavior
changes. Source test declarations, manifest targets, workspaces, filtering,
assertions, snapshots, property testing, parallelism, cancellation, coverage,
benchmarks, and Stable compatibility remain intentionally deferred.
