# TASK-2203 Authority Audit: Structured Task Lifecycle Runtime

## Outcome

`TASK-2203` is correctly recorded as `BlockedSpec`. The G2 plan requires a
runtime that creates and closes scopes, registers children, joins by default,
propagates parent cancellation, aggregates child Faults, models timeout as an
explicit Clock plus cancellation combination, and exits without orphan Tasks.
The lifecycle, failure, cleanup, and scheduler contracts required to implement
that runtime are not accepted.

Proposed DEC-0266 now supplies a reviewable scheduler-neutral lifecycle-kernel
contract, but a Proposed decision is not implementation authority. TASK-2203
therefore remains `BlockedSpec` until that decision is Accepted.

No Task runtime, scope registry, child cancellation propagation, join policy,
Fault aggregator, timeout API, orphan detector, scheduler interface,
diagnostic allocation, or placeholder G2 API was added.

Accepted `DEC-0093` now authorizes the bounded child
`TASK-2203-LIFECYCLE-OBSERVATION`, which records only immutable structural
observations and deterministic identities. It does not close any of the
runtime authority gaps listed below.

The audit also found that DEC-0264's current ban on another Task handle crossing
a suspension prevents multiple live same-scope children. Proposed DEC-0266
addresses that conflict explicitly: linear handles remain in the runtime scope
registry rather than DEC-0265 value frames, preserving existing machine bytes.

## Normative traceability

- The G2 execution package is non-normative; its runtime checklist does not
  authorize a scheduler, lifecycle state machine, or public timeout protocol.
- `TASK-2201` and `TASK-2202` are complete under Accepted DEC-0264 and
  DEC-0265, so checked Task Core and non-executable `ling.task-machine/0.1`
  inputs now exist. `docs/SEMANTICS.md` still gives only future structured
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
- Proposed DEC-0266 defines checked runtime identities, lifecycle states,
  same-scope handle retention, registration, join, cancellation, deterministic
  Fault aggregation, exactly-once cleanup, explicit bounds, and a scheduler-
  neutral ready-set/step boundary. It remains non-authoritative until Accepted.

## Current implementation evidence

- The frontend, type, Effect, and Checked Core pipeline now represents Task
  scopes, handles, suspension liveness, and validated non-executable state
  machines. It contains no child registry, join barrier, cancellation
  propagation, cleanup executor, Fault aggregator, or structured runtime.
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

## Proposed authority contract

DEC-0266 proposes the following minimum contract; implementation still requires
its acceptance:

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

Until this decision is Accepted, the runtime could leak children, run after
parent cancellation, lose or reorder Faults, race timeout against effects, or
silently leave an orphan Task.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0018, RFC-0001,
RFC-0020, Proposed DEC-0266,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, and VM
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, scheduler, or Unicode 17.0.0 behavior changed.

The child implementation report and authority audit provide focused evidence
for the observation boundary; the public lifecycle runtime remains blocked.

## Intentionally deferred

`TASK-2203` remains blocked on acceptance of DEC-0266 (or an Accepted
replacement that resolves the same portion of `GAP-STRUCTURED-TASK-001`);
TASK-2201 and TASK-2202 are complete dependencies. The future runtime must
consume checked Task state machines only, make scope/cancellation/cleanup/Fault
transitions explicit, separate deterministic test scheduling from production
behavior, and publish lifecycle differential evidence before exposing Task
execution.
