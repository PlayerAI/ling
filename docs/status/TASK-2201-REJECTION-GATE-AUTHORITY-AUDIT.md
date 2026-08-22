# TASK-2201 Rejection Gate Authority Audit

## Outcome

`TASK-2201-TASK-SYNTAX-REJECTION` is a bounded negative-evidence child
authorized by Accepted `DEC-0089`. It proves only that a Task-shaped top-level
declaration cannot reach the checked compiler pipeline under the current Seed
profile. The public `TASK-2201` target remains `BlockedSpec`.

## Normative basis

- `docs/LANGUAGE.md` §19 excludes Task from v0.0.1 Seed.
- `docs/SEMANTICS.md` §18 marks Task as pre-v0.2 and leaves the complete
  lifecycle contract to the concurrent specification gate.
- `docs/ROADMAP-1.0.md` §6.2 requires accepted Task lifecycle,
  cancellation, detach, and suspension authority before implementation.
- `DEC-0001` and `DEC-0002` fix the existing diagnostic registry and original
  UTF-8 byte-span units.
- `DEC-0089` limits this child to the existing parser/CLI rejection boundary.

## Evidence boundary

The fixture invokes `ling_cli::compile_source` with a source-shaped `task`
declaration and checks `L-SYNTAX-0010`, bilingual JSON fields, and the exact
original byte span. It proves that no `Compiled` value or checked
`ProgramSnapshot` is returned.

It does not reserve a Task keyword, add AST/HIR/Typed Core nodes, or define
scope, suspension, cancellation, cleanup, Fault, detach, scheduler, Effect,
Capability, bytecode, VM, LSP, schema, Semantic ID, or migration behavior.

## Intentionally deferred

Task grammar, scope identity, parent/child ownership, suspension frames,
cancellation propagation, cleanup ordering, Fault aggregation, detach
authority, deterministic scheduling, runtime lowering, and positive/
differential fixtures remain blocked by `GAP-STRUCTURED-TASK-001` and the
missing Accepted Task RFC.
