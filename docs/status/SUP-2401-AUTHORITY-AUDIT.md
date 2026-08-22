# SUP-2401 Authority Audit: Supervisor Model

## Outcome

`SUP-2401` is correctly recorded as `BlockedSpec`. The G2 plan asks for child
specifications, restart/stop/escalate actions, one-for-one/rest-for-one
strategies, child lifetime classes, restart budgets, state-restore sources,
and a parent Fault channel. No accepted supervision contract defines those
states, transitions, ownership, or their interaction with Actor turns,
mailboxes, Task cancellation, and resource cleanup.

No Supervisor type, child specification, restart strategy, escalation API,
Fault channel, state snapshot protocol, diagnostic, public protocol, or
placeholder G2 API was added.

Accepted `DEC-0101` now authorizes the bounded child `SUP-2401-OBSERVATION`,
which records only immutable supervision observation identities and structural
labels. It does not close the child ownership, strategy, restart budget, state
restore, Fault channel, runtime, or shutdown gaps described below.

## Normative traceability

- The G2 execution package is non-normative. Its supervision checklist cannot
  authorize a runtime state machine, source syntax, query/debug API, or Fault
  schema.
- The plan requires RFC-C204 and ACT-2305 before SUP-2401. No Accepted RFC-C204
  or replacement RFC-0009 exists, and ACT-2301 through ACT-2306 are
  `BlockedSpec`; RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` requires Actors to belong to a supervision tree or an
  explicit runtime root and lists restart strategy, intensity, shutdown order,
  state restore, and Fault escalation as concepts. It does not define the
  child state machine, strategy semantics, budget clocks, snapshot identity,
  failure aggregation, or public observability, and v0.0.1 implements no
  Actor/Task runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit Fault/error
  behavior, cleanup, restart budgets, and queryable Fault provenance, but do
  not define a stable Supervisor syntax, ABI, event schema, or migration
  protocol.
- Accepted DEC-0010/DEC-0013 cover Seed Capability/State and main/runtime
  failures only; DEC-0018 governs RFC lifecycle; RFC-0020 explicitly excludes
  Ling Task/Actor cancellation, scheduling, and replay. None authorizes
  supervision semantics.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open and blocks SUP-2401,
  SUP-2402, and SUP-2403. Its accepted resolution must include stress,
  ordering, backpressure, and resource-limit evidence.

## Current implementation evidence

- The workspace has no Actor runtime, Supervisor, child registry, restart
  budget, state snapshot/restore, escalation channel, or lifecycle event
  schema. `ling-eval` and `ling-vm` expose only Seed runtime Faults and host
  cancellation.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no child specification, supervision policy, Fault provenance, restart
  intensity, or state-restore judgment.
- `ling-semantic` and the protocol inventory contain no accepted Supervisor
  node, event schema, query surface, diagnostic, or public compatibility
  version. Existing compiler-query scheduling decisions are unrelated to
  supervision.
- No fixture covers one child Fault, multi-child Faults, restart/stop/escalate,
  child lifetime classes, state restore failure, parent termination, mailbox
  cleanup, cancellation, or interpreter/VM/runtime equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. supervisor/child ownership, tree and runtime-root identity, child states,
   legal transitions, startup/stop ordering, parent termination, and
   restartability classes;
2. exact restart/stop/escalate and one-for-one/rest-for-one (or other)
   strategy semantics, failure aggregation, sibling effects, mailbox disposal,
   queued-message handling, and backpressure interaction;
3. restart budget units, time/logical windows, backoff, circuit state,
   deterministic clock/replay behavior, budget exhaustion, and escalation
   results;
4. state snapshot/restore identity, versioning, invariant checks, partial
   restore failure, Resource/Managed cleanup, Capability isolation, and
   cancellation/Fault propagation;
5. parent Fault-channel type, provenance fields, diagnostics, Semantic Graph/
   Audit Source projection, public protocol/schema identity, local/remote
   boundary, privacy, and migration rules; and
6. executable positive/negative/migration/stress fixtures for each child
   strategy and lifetime class, single/multiple/repeated Faults, restart
   budget exhaustion, state-restore failure, parent shutdown, mailbox cleanup,
   concurrent failures, Unicode/CRLF/BOM spans, deterministic output, and
   interpreter/VM/runtime differential behavior without unchecked-AST
   execution.

Until these decisions are Accepted, implementing a Supervisor would freeze
failure recovery, liveness, state durability, message-loss, and security
semantics that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
RFC-0001, RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Supervisor, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`SUP-2401` can begin only after ACT-2301 through ACT-2306 and Accepted
RFC-C204 (or replacement RFC-0009) resolve Actor identity, message
sendability, mailbox/backpressure, turn/reentry, runtime lifecycle, and Fault
boundaries. The future implementation must consume accepted types and checked
Core only, publish a versioned Supervisor state machine and Fault channel,
enforce bounded restart/resource behavior, and provide recovery, cleanup, and
interpreter/VM evidence before supervision is exposed.
