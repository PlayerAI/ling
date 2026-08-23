# CLI-1704 implementation report

## Result

CLI-1704 is complete under DEC-0039, RFC-0025, and DEC-0256. `ling test` has two
explicit, mutually exclusive current modes: deterministic standalone
file/directory execution and one locked/offline project-entry smoke test.

## Evidence

- `crates/ling-cli/src/test_runner.rs` implements sorted standalone discovery,
  independent checked compilation/execution, capture, aggregation, and exit
  precedence.
- `crates/ling-cli/src/project.rs` and `main.rs` implement explicit project
  loading and the single root-entry smoke test through checked Typed Core.
- `crates/ling-cli/tests/test.rs`, `project_commands.rs`, and
  `output_policy.rs` cover both modes, schemas, capture, failures, selection,
  output policy, determinism, and nonmutation.
- `schemas/test/0.1/` and the project-command schema corpus retain exact
  machine contracts.

No test annotation or hidden convention is implemented. This is deliberate:
the execution plan requires the accepted convention when source test syntax is
not specified, and DEC-0256 defines the two explicit program-level conventions
as complete current behavior.

## Compatibility and deferred work

No observable implementation byte changes in this closure. Test declarations,
manifest test targets, workspaces, filtering, assertions, snapshots, property
tests, parallelism, cancellation, coverage, benchmarks, and Stable compatibility
remain deferred.

