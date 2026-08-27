# TASK-2203 Authority Audit: Structured Task Lifecycle Runtime

## Outcome

`TASK-2203` is `Ready` under Accepted DEC-0266. The G2 plan requires a
runtime that creates and closes scopes, registers children, joins by default,
propagates parent cancellation, aggregates child Faults, models timeout as an
explicit Clock plus cancellation combination, and exits without orphan Tasks.
DEC-0266 authorizes the scheduler-neutral lifecycle kernel, including explicit
driver choices without defining a scheduler policy. Timeout injection, virtual
time, deterministic scheduling, production scheduling, detach, and user
Resource finalizers remain later work.

No Task runtime, scope registry, child cancellation propagation, join policy,
Fault aggregator, timeout API, orphan detector, scheduler interface,
diagnostic allocation, or placeholder G2 API was added.

Accepted `DEC-0093` now authorizes the bounded child
`TASK-2203-LIFECYCLE-OBSERVATION`, which records only immutable structural
observations and deterministic identities. It does not close any of the
runtime authority gaps listed below.

The audit also found that DEC-0264's current ban on another Task handle crossing
a suspension prevents multiple live same-scope children. Accepted DEC-0266
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
- `GAP-STRUCTURED-TASK-001` remains open for TASK-2204 through TASK-2206
  scheduling, virtual-time, production-runtime, detach, Resource cleanup, and
  final conformance decisions. RFC-0008 and planning RFC-C202 are not Accepted;
  RFC-0001 remains a Draft baseline under DEC-0018.
- RFC-0020 defines explicit host-owned cancellation for existing VM execution
  only. It does not define source Task cancellation, child registration, join,
  timeout/Clock races, Fault aggregation, cleanup, or orphan semantics.
- Accepted DEC-0266 defines checked runtime identities, lifecycle states,
  same-scope handle retention, registration, join, cancellation, deterministic
  Fault aggregation, exactly-once cleanup, explicit bounds, and a scheduler-
  neutral ready-set/step boundary.

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

## Accepted implementation contract

DEC-0266 defines the following minimum TASK-2203 contract:

1. construction from one exact checked Task definition, its matching Checked
   Task Core and state machine, checked arguments, and explicit non-zero bounds;
2. canonical runtime Task/scope identities, atomic child registration,
   same-scope linear handle ownership, suspension/wake, default join, and no
   orphan escape;
3. monotonic cancellation with explicit checkpoints, committed-Effect
   preservation, canonical Fault aggregation, and children-first exactly-once
   structural cleanup;
4. a canonical ready set plus caller-driven `step(id)` boundary that makes no
   scheduling, wall-clock, thread, bytecode, VM, CLI, or public protocol claim;
   and
5. positive, negative, bounded, cancellation, cleanup, Fault, source-span, and
   explicit-schedule differential evidence consuming Checked Core only.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0018, RFC-0001,
RFC-0020, Accepted DEC-0266,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, and VM
crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, scheduler, or Unicode 17.0.0 behavior changed.

The child implementation report and authority audit provide focused evidence
for the observation boundary; TASK-2203 implementation may now proceed, while
public Task execution remains rejected by `L-TASK-0004`.

## Intentionally deferred

TASK-2201 and TASK-2202 are complete dependencies and TASK-2203 may proceed
under Accepted DEC-0266. TASK-2204 deterministic scheduling and virtual time,
TASK-2205 production scheduling and public execution integration, TASK-2206
final conformance/stress evidence, Task bytecode/VM/native ABI, detach,
user Resource finalizers, Replay, Actor crossing, migration, and Stable
compatibility remain intentionally deferred.
