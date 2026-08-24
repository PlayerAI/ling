# TASK-2201 Authority Audit: Structured Task Syntax and Checked Core

## Outcome

`TASK-2201` remains correctly recorded as `BlockedSpec`. Accepted `DEC-0089`
closes the bounded `TASK-2201-TASK-SYNTAX-REJECTION` evidence child without
adding a Task grammar, and Accepted `DEC-0091` closes the separate
publish-disabled `TASK-2201-CORE-MODEL` identity-graph child. The G2 plan
proposes a minimal `task`/`scope`/`let!`/`return` surface and requires Checked
Core fields for scope identity, parent/child relation, spawn/join, suspension points,
cancellation, cleanup, and optionally capability-gated detach. EFF-2103 and
its checked Handler Core are now complete. Proposed `DEC-0264` defines a
checked-only, non-executable Task frontend boundary, but it is not
implementation authority until Accepted; runtime lifecycle, Fault aggregation,
scheduling, and detach remain later decisions.

The rejection child proves only that a Task-shaped top-level declaration is
rejected by the existing bilingual syntax diagnostic before checked snapshot
publication. The Core-model child validates only nonzero checked identities,
parent/child acyclicity, suspension identity uniqueness, optional detach
evidence, and path-free canonical bytes. No Task grammar, AST/HIR/typed-program
integration, task type checker, runtime cancellation or cleanup semantics,
diagnostic allocation, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its sample syntax and Core field
  list do not authorize a new source construct or lifecycle semantics.
- `docs/SEMANTICS.md` describes Task as a future Core/runtime form and states
  the high-level structured-lifetime intent, but v0.0.1 implements only the
  first twelve Core forms and explicitly excludes Task/Actor/Node/Kernel.
  It does not fix syntax, type/effect rules, suspension ownership, cleanup
  order, or Fault aggregation.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` place Structured Task in the
  v0.2 Concurrent scope and require a specification gate before implementation.
- The plan dependency `RFC-C202` is only a planning placeholder. No Accepted
  RFC-0008 (or replacement) defines Task syntax or Checked Core, and RFC-0001
  remains a Draft design baseline under DEC-0018.
- `GAP-STRUCTURED-TASK-001` leaves parent/child runtime lifetime, cancellation
  propagation, Fault aggregation, detach authority, suspension points, and
  cleanup ordering open. EFF-2103 now provides the accepted checked Handler
  foundation needed to define Task suspension interaction.
- RFC-0020 accepts only host-owned VM cancellation for the existing Seed
  bytecode entry point. It does not define source Task cancellation,
  structured cleanup, suspension, or child Fault propagation.
- Accepted `DEC-0089` reuses `L-SYNTAX-0010` for negative Task-syntax evidence
  only; it does not reserve a lexer keyword or authorize any positive Task
  semantics.
- Accepted `DEC-0091` authorizes only the publish-disabled `ling-concurrency`
  identity graph; source spans are evidence and no identity graph field grants
  detach, cancellation, scheduling, or runtime authority.

## Current implementation evidence

- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no `task`, `scope`, `let!`, `spawn`, `join`, `await`, or Task lifecycle node;
  the current grammar and type pipeline implement the Seed subset only.
- `ling-concurrency::TaskCore` is an internal checked-data graph only; it is not
  connected to `TypedProgram`, `ProgramSnapshot`, the parser, or the evaluator.
- `ling-types::TypedProgram` and the current checked Core boundary contain no
  scope/parent identity, child registration, suspension continuation,
  cancellation token, cleanup region, or detach capability.
- `ling-eval`, `ling-bytecode`, and `ling-vm` execute only accepted Seed
  constructs. They have no Task state machine, structured scope runtime,
  cancellation propagation, cleanup stack, or Task Fault aggregation.
- Existing VM cancellation is an explicit host-control boundary and cannot be
  reused as source-level Task semantics. No Task conformance, differential,
  scheduler, resource-cleanup, or migration fixture exists.
- `crates/ling-cli/tests/task_boundary.rs` is the bounded negative fixture; it
  asserts the original `task` byte span and no checked snapshot.

## Required authority before implementation

Before TASK-2201 implementation, an Accepted RFC or decision must define:

1. source grammar, AST/HIR/Checked Core representation, type/effect rules for
   `task`, `scope`, `let!`, `return`, `await`, spawn, join, and result
   observation, including original UTF-8 source spans and stable identities;
2. lexical scope/handle ownership, path-complete observation, initial detach
   rejection, suspension live-value restrictions, and interaction with
   Effect/Capability handlers;
3. diagnostics, Semantic IDs, Audit Source, deterministic identities, and the
   checked-only rejection boundary before runtime authority; and
4. positive/negative/migration fixtures for nested scopes, multiple suspension
   points, conditional/match paths, exact handle observation, invalid live
   values, detach rejection, Unicode/CRLF/BOM spans, deterministic identities,
   checked-only execution rejection, and no unchecked-AST execution.

Proposed DEC-0264 specifies these TASK-2201 frontend choices. Until it is
Accepted, implementing them would prematurely freeze public syntax and checked
identity. Runtime join/cancel/cleanup, Fault aggregation, scheduling,
interpreter/VM execution, and resource limits remain TASK-2202 through
TASK-2206 authority even after TASK-2201 is unblocked.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
Proposed DEC-0264, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, and VM
crates.

No compiler, interpreter, VM, bytecode, diagnostic registry, schema,
Semantic ID, public source-span contract, runtime, scheduler, or Unicode 17.0.0
behavior changed. The two bounded children add only an offline negative
fixture and an internal checked-data model.

## Intentionally deferred

The bounded `TASK-2201-TASK-SYNTAX-REJECTION` child is complete under
`DEC-0089`, and `TASK-2201-CORE-MODEL` is complete under `DEC-0091`. Public
`TASK-2201` can begin after Proposed DEC-0264 is reviewed and Accepted; EFF-2103
is already complete. The broader `GAP-STRUCTURED-TASK-001` remains open for
TASK-2202 through TASK-2206 runtime work. The implementation must lower only
accepted Task syntax to checked Typed Core, make every suspension and cleanup
identity explicit, and preserve source identity. Later tasks must prove
interpreter/VM and cancellation/cleanup equivalence before exposing Task
execution.
