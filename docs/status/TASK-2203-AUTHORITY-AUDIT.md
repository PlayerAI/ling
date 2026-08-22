# TASK-2203 Authority Audit: Structured Task Lifecycle Runtime

## Outcome

`TASK-2203` is correctly recorded as `BlockedSpec`. The G2 plan requires a
runtime that creates and closes scopes, registers children, joins by default,
propagates parent cancellation, aggregates child Faults, models timeout as an
explicit Clock plus cancellation combination, and exits without orphan Tasks.
The lifecycle, failure, cleanup, and scheduler contracts required to implement
that runtime are not accepted.

No Task runtime, scope registry, child cancellation propagation, join policy,
Fault aggregator, timeout API, orphan detector, scheduler interface,
diagnostic allocation, or placeholder G2 API was added.

Accepted `DEC-0093` now authorizes the bounded child
`TASK-2203-LIFECYCLE-OBSERVATION`, which records only immutable structural
observations and deterministic identities. It does not close any of the
runtime authority gaps listed below.

## Normative traceability

- The G2 execution package is non-normative; its runtime checklist does not
  authorize a scheduler, lifecycle state machine, or public timeout protocol.
- `TASK-2201` and `TASK-2202` are `BlockedSpec`, so no accepted Task Core or
  state-machine input exists. `docs/SEMANTICS.md` gives only future structured
  lifetime intent and explicitly excludes Task from v0.0.1 execution.
- `docs/ROADMAP-1.0.md` makes Task cancellation, cleanup, suspension, and
  deterministic scheduling v0.2 specification gates. It does not define
  parent/child failure precedence, join timing, timeout races, or orphan
  observability.
- `GAP-STRUCTURED-TASK-001` leaves parent/child lifetime, cancellation
  propagation, Fault aggregation, detach, suspension, and cleanup ordering
  open. RFC-0008 and planning RFC-C202 are not Accepted; RFC-0001 remains a
  Draft baseline under DEC-0018.
- RFC-0020 defines explicit host-owned cancellation for existing VM execution
  only. It does not define source Task cancellation, child registration, join,
  timeout/Clock races, Fault aggregation, cleanup, or orphan semantics.

## Current implementation evidence

- The syntax, AST, HIR, type, effect, and checked Core crates contain no Task
  scope, child handle, join, cancellation propagation, or structured-runtime
  representation.
- `ling-eval` executes checked Seed `ProgramSnapshot` values synchronously;
  it has no scope tree, child registry, scheduler, join barrier, timeout
  clock, cleanup stack, or Fault aggregation.
- `ling-vm` has bounded instruction/frame limits and explicit host cancellation,
  but no Task scope runtime, parent-to-child token graph, orphan policy, or
  structured cleanup/failure state. A host cancellation token cannot serve as
  source Task lifecycle semantics.
- No fixture or public protocol covers scope close, default join, cancellation
  races, child Fault aggregation, timeout, cleanup ordering, orphan rejection,
  resource exhaustion, or interpreter/VM lifecycle equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. scope and child identity, registration and ownership, default join and
   scope-exit obligations, result observation, transfer/detach authority, and
   orphan detection/reporting;
2. cancellation token topology, idempotence, propagation checkpoints, timeout
   and Clock interaction, race precedence against normal return/Join/Fault,
   and committed external-effect behavior;
3. child Fault representation, aggregation order, propagation/recovery policy,
   cleanup/finalization guarantees on success, cancellation, and Fault,
   resource limits, and no-leak/no-partial-publication rules;
4. interpreter reference semantics, VM/scheduler ABI, deterministic versus
   production scheduling boundaries, source-map and diagnostic projection,
   Semantic IDs, Audit Source, protocol/schema versioning, and migration; and
5. executable positive/negative/migration/differential fixtures for nested and
   empty scopes, multiple children, default join, cancellation before/after
   effects, timeout races, child Fault combinations, cleanup success/cancel/
   Fault paths, unobserved results, detach/orphan rejection, resource limits,
   Unicode/CRLF/BOM spans, deterministic traces, and no unchecked-AST
   execution.

Until these decisions are Accepted, the runtime could leak children, run after
parent cancellation, lose or reorder Faults, race timeout against effects, or
silently leave an orphan Task.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0018, RFC-0001,
RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, and VM
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, scheduler, or Unicode 17.0.0 behavior changed.

The child implementation report and authority audit provide focused evidence
for the observation boundary; the public lifecycle runtime remains blocked.

## Intentionally deferred

`TASK-2203` can begin only after TASK-2201/TASK-2202 and an Accepted RFC-0008
(or replacement) resolve `GAP-STRUCTURED-TASK-001`. The future runtime must
consume checked Task state machines only, make scope/cancellation/cleanup/Fault
transitions explicit, separate deterministic test scheduling from production
behavior, and publish lifecycle differential evidence before exposing Task
execution.
