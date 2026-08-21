# SUP-2403 Authority Audit: Supervision Tests

## Outcome

`SUP-2403` is correctly recorded as `BlockedSpec`. The G2 plan requires tests
for single and repeated child Faults, Faults during restart, budget-exceeded
escalation, parent termination, state-snapshot restore failure, and explicit
cleanup of unprocessed mailbox messages. Without Accepted Supervisor,
restart-budget, Actor, mailbox, Task, and replay contracts, those tests would
only ratify an implementation-specific policy.

No supervision test corpus, fixture schema, runtime harness, restart policy,
diagnostic, protocol, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its test checklist does not
  authorize a Supervisor state machine, recovery outcome, trace schema, or
  cross-backend equivalence relation.
- SUP-2403 depends on RFC-C204, ACT-2305, SUP-2401, and SUP-2402. No Accepted
  RFC-C204 or replacement RFC-0009 exists; ACT-2301 through ACT-2306,
  SUP-2401, and SUP-2402 are `BlockedSpec`; RFC-0001 remains Draft under
  DEC-0018.
- `docs/SEMANTICS.md` requires explicit supervision strategy, shutdown order,
  state restore, and Fault escalation, but does not fix the transitions,
  budgets, snapshot identity, mailbox cleanup, or test equivalence relation;
  v0.0.1 implements no Actor/Task/Supervisor runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require Fault provenance,
  restart budgets, cleanup, and stress evidence, but do not define fixture
  metadata, deterministic seeds, platform scope, migration, or public test
  protocols.
- Accepted DEC-0010/DEC-0013 cover Seed State/Capability and main/runtime
  Faults only; DEC-0021 covers compiler-query scheduling only; RFC-0020
  excludes Ling Task/Actor cancellation, scheduling, and replay. None
  authorizes supervision test semantics.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open and blocks SUP-2403; its
  resolution requires positive/negative/migration/stress, ordering,
  backpressure, and resource-limit evidence.

## Current implementation evidence

- The workspace has no Actor runtime, Supervisor, restart budget, state
  snapshot/restore, mailbox cleanup, Fault provenance, deterministic scheduler,
  or replay harness. `ling-eval` and `ling-vm` only test Seed execution and
  host VM cancellation/resource faults.
- There is no accepted source/Core/runtime representation for child Faults,
  restart-in-progress, budget exhaustion, escalation, parent shutdown, or
  unprocessed-message disposition. No schema or diagnostic can identify these
  events.
- Existing conformance and VM differential fixtures cannot exercise Actor
  supervision, and compiler-query scheduling evidence is not runtime
  interleaving evidence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the supervision state machine and strategy/lifetime classes under test,
   legal Fault/restart/stop/escalate transitions, parent/child ordering, and
   concurrent-failure aggregation;
2. restart budget/window/backoff/circuit semantics, deterministic clock/replay
   inputs, budget-exceeded result, and the exact escalation channel;
3. state snapshot/restore identity, versioning and invariant checks, restore
   failure behavior, Resource/Managed cleanup, mailbox drain/drop policy,
   cancellation, and parent termination semantics;
4. fixture/test metadata, stable event/provenance schema, deterministic seeds,
   random-interleaving model, platform/resource bounds, privacy, migration,
   diagnostics, Semantic Graph/Audit Source projection, and interpreter/VM/
   runtime comparison rules; and
5. executable positive/negative/migration/stress fixtures for single/multiple/
   repeated Faults, Fault during restart, budget exhaustion/escalation, parent
   termination, restore failure, unprocessed-mailbox cleanup, concurrent
   failures, cancellation, Unicode/CRLF/BOM spans, deterministic reruns, and
   cross-backend behavior without unchecked-AST execution.

Until these decisions are Accepted, adding tests would make an unapproved
recovery policy look normative and could hide data loss, leaked resources,
non-determinism, or unsafe Fault handling.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Supervisor, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`SUP-2403` can begin only after SUP-2401/SUP-2402, ACT-2301 through ACT-2306,
and Accepted RFC-C204 (or replacement RFC-0009) resolve supervision,
restart-budget, Actor, mailbox, turn, Fault, cleanup, and replay boundaries.
The future tests must consume accepted fixtures and checked Core/runtime traces
only, exercise each accepted recovery transition, and publish bounded cleanup,
ordering, determinism, and interpreter/VM evidence before claiming
supervision conformance.
