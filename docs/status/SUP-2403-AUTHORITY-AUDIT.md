# SUP-2403 Authority Audit: Supervision Tests

## Outcome

`SUP-2403` remains correctly recorded as `BlockedSpec`, but the original audit
is obsolete about its prerequisites. ACT-2301 through ACT-2306 and SUP-2401/
SUP-2402 are now Done under scoped Accepted DEC-0270 through DEC-0277. The real
private checked-Core Actor runtime and local Supervisor implement bounded
mailboxes, turns, containment, restart budgets, fixed backoff, circuits,
termination, and cleanup.

The former blocker was narrower: DEC-0277 clause 18 explicitly did not
authorize SUP-2403, and the non-normative G2 checklist mixes already Accepted
outcomes with unaccepted state restore, escalation, concurrency, Replay, and
cross-backend expectations. Accepted DEC-0278 now defines a private
executable evidence matrix that tests only DEC-0274 through DEC-0277 behavior
and records unsupported plan labels honestly. SUP-2403 is Ready for that exact
bounded matrix; placeholder APIs remain prohibited.

## Normative traceability

- The G2 execution package is non-normative. Its test checklist does not
  authorize a Supervisor state machine, recovery outcome, trace schema, or
  cross-backend equivalence relation.
- SUP-2403's registered dependency SUP-2402 is Done. No Accepted RFC-C204 or
  replacement public RFC-0009 exists, but scoped DEC-0274 through DEC-0277 now
  authorize the private local runtime behavior needed for a bounded test-only
  slice. RFC-0001 remains Draft under DEC-0018 and cannot broaden that slice.
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
  excludes Ling Task/Actor cancellation, scheduling, and replay. DEC-0274
  through DEC-0277 now authorize private Actor/Supervisor execution and
  focused evidence, but each completion boundary excludes SUP-2403.
- Accepted DEC-0103 authorizes only the structural
  `SUP-2403-OBSERVATION` vocabulary. It cannot be used as a runtime harness or
  expected-outcome source.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open and blocks SUP-2403; its
  resolution requires positive/negative/migration/stress, ordering,
  backpressure, and resource-limit evidence.

## Current implementation evidence

- `crates/ling-eval/src/actor_runtime.rs` implements the DEC-0274 bounded local
  checked-Core Actor registry, typed send/FIFO admission, explicit turn,
  lifecycle, Fault provenance, shutdown, discard, and exactly-once cleanup.
- `crates/ling-eval/src/actor_supervisor.rs` implements DEC-0276 `ContainOne`
  and DEC-0277 `RestartOneBudgeted`, including logical ticks, exact attempt
  windows, fixed backoff, Closed/Open/HalfOpen circuits, fresh identities,
  initializer Fault handling, canonical recovery order, stop/cancellation,
  resource fallback, and private snapshots.
- Existing focused tests already prove many individual DEC-0276/DEC-0277
  clauses. They are not a distinct accepted SUP-2403 matrix and do not define
  state restore, escalation, concurrent recovery, serialized fixtures, Replay,
  or cross-backend equivalence.
- Every public Actor-bearing route remains unavailable with `L-ACTOR-0002`;
  there is no public Actor/Supervisor query, diagnostic, schema, or protocol.

## Required authority before implementation

Accepted DEC-0278 supplies the minimum missing boundary:

1. exactly eight private case families execute only the Accepted ContainOne,
   restart, circuit, termination, cleanup, invalid-evidence, resource, and
   Unicode reconstruction behavior;
2. the stale checklist maps budget exhaustion to circuit Open rather than
   escalation, restart-time Fault to serialized initializer Fault, parent
   termination to stop/cancellation, and mailbox cleanup to accepted discard
   and cleanup evidence;
3. state restore, escalation, concurrent/group recovery, public fixtures,
   Replay, and backend differential behavior are explicit negative/deferred
   boundaries and must not gain placeholders;
4. finite explicit command scripts consume successful `CheckedProgram` and the
   real private runtime, then compare only bounded in-memory DEC-0274 through
   DEC-0277 projections; and
5. SUP-2403 completes only after dedicated executable evidence, public
   `L-ACTOR-0002` boundary checks, full repository gates, commit binding, and
   synchronized status evidence.

DEC-0278 authorizes only this private evidence task and no new runtime behavior.

## Evidence and compatibility

This refreshed audit and Accepted DEC-0278 were checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, DEC-0103, DEC-0274, DEC-0275, DEC-0276, DEC-0277, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

The re-audit and proposal change no compiler, interpreter, VM, bytecode,
scheduler, mailbox, Actor protocol, Supervisor runtime, diagnostic, schema,
Semantic ID, source-span, public protocol, or Unicode 17.0.0 behavior.

## Intentionally deferred

`SUP-2403` may now begin, but remains limited to private in-memory evidence over
DEC-0274 through DEC-0277. State
restore, escalation, concurrent/group recovery, public fixture/query/Fault
protocols, Replay, remote delivery, interpreter/VM/backend differential claims,
migration, fairness, liveness, performance/stress guarantees, and Stable
compatibility require later Accepted authority and keep the broader gap Open.
