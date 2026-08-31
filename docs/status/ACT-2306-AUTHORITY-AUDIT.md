# ACT-2306 Authority Audit: Actor Properties and Stress Tests

## Outcome

ACT-2306 is Done. Accepted DEC-0274 provides a real internal,
checked-Core-only local Actor runtime in ling-eval; its integration suite already
establishes a bounded set of FIFO, Full, Fault, cancellation, cleanup,
Unicode/BOM/CRLF, and public-boundary facts.

Accepted DEC-0275 defines the property, generated-interleaving,
parallel-turn, slow-consumer, host-unwind, and bounded-stress evidence contract
for that runtime. The completed internal ACT-2306 implementation realizes this
test-only contract without making the resulting host execution model public Ling
behavior.

No new normal-build runtime behavior, scheduler, stress protocol, property
schema, Replay format, diagnostic, or public Actor API is added.

## Normative traceability

- docs/SEMANTICS.md §§19.2--19.5 requires Actor state isolation, a bounded
  mailbox, same-sender order, one active turn, and atomic observation when a
  turn has no await. It does not define a host worker model, a property
  generator, a parallel-commit boundary, stress thresholds, or a public trace.
- Accepted DEC-0270 through DEC-0273 establish the checked-only identity,
  sendability/schema, Reject mailbox, and non-suspending one-message turn
  profile. GAP-ACTOR-AWAIT-REENTRY-001 is Accepted for that profile and is not
  an ACT-2306 blocker.
- Accepted DEC-0274 clauses 4, 7--16 establish explicit runtime bounds,
  failure-atomic admission, FIFO messages, explicit ready/step dispatch,
  publish-on-normal-return state, contained Fault, stop/shutdown cleanup, and
  deterministic internal observations. Its first runtime serializes command
  admission and does not define independent host-parallel turn execution,
  fairness, worker visibility, Replay, or a stress oracle.
- docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md:313--322 is
  non-normative. Its requested serial-state, independent-Actor, bounded
  mailbox, slow-consumer, post-stop, Fault, interleaving, and shutdown evidence
  needs a concrete Accepted interpretation before it can mark a language task
  Done.
- Accepted DEC-0021 defines deterministic scheduling only for compiler queries;
  it cannot supply Actor worker, message, or Replay semantics. DEC-0267/0268
  govern Structured Task scheduling, not Actor dispatch.
- GAP-ACTOR-MAILBOX-SUPERVISOR-001 remains Open for supervisor-visible failure,
  restart, escalation, and alternative mailbox policies. Those unresolved
  surfaces must stay outside the proposed local Reject-only test contract.
  GAP-DETERMINISTIC-REPLAY-001 remains Open; ACT-2306 may use internal
  deterministic test inputs but cannot create a Replay contract.

## Current implementation evidence

- crates/ling-eval/src/actor_runtime.rs implements the DEC-0274 run-owned
  ActorRuntime: it revalidates checked Actor Core, allocates non-reused
  identities, admits typed FIFO envelopes into bounded Reject mailboxes,
  performs an explicit selected step, commits state only after normal return,
  contains evaluator panics, and cleans up through Task-owned cancellation or
  explicit shutdown.
- crates/ling-eval/tests/actor_runtime.rs has twelve integration cases. They
  already cover serial FIFO state progress, mailbox Full with original payload,
  type/cross-run rejection, canonical ready order, turn/initializer Fault,
  cancellation, explicit stop, resource-exhaustion atomicity, and
  Unicode/BOM/CRLF reconstruction.
- crates/ling-eval/src/actor_runtime_properties.rs is a cfg(test)-only
  ACT-2306 driver. It preflights and reserves at most one FIFO envelope per
  distinct Actor, evaluates pure candidates on one to four scoped workers,
  restores reservations on test-driver failure, and commits only successful
  candidates in Actor-ID order after all workers complete.
- The property suite uses a barrier probe rather than timing, deterministic
  SplitMix64 command schedules, one/two-worker projection comparison, bounded
  backpressure, cleanup, resource, panic, and Unicode reconstruction cases.
- There is no public Actor execution, source spawn/send/stop, public scheduler,
  Replay trace, serialization, supervisor, remote delivery, bytecode/VM/native
  Actor path, or runtime differential contract. The CLI Actor boundary remains
  L-ACTOR-0002.

## Accepted authority and completion evidence

An Accepted decision must define, at minimum:

1. the result projection for generated and parallel-turn cases, including
   exactly which lifecycle, state, ordering, Fault, cleanup, and source facts
   compare and which host facts are excluded;
2. same-Actor reservation/serialization and a bounded independent-Actor
   parallel-turn model, including commit ordering and the scope of any
   test-only synchronization probe;
3. deterministic generator inputs, seed retention, command/actor/message/worker
   bounds, slow-consumer modeling without wall time, and failure shrinking;
4. Full, post-stop, cancellation, Fault, panic containment, shutdown,
   cleanup, resource-exhaustion, Unicode/BOM/CRLF, and public-boundary
   expectations; and
5. an explicit non-goal boundary for fairness, liveness, cross-sender global
   order, Replay, supervision, production worker scheduling, and public
   protocols.

Accepted DEC-0275 is the governing authority. ACT-2306 has added the generated
stress harness, bounded parallel-turn driver, and test-only panic injection
seam, while retaining its explicit public-boundary and deferred-work limits.

## Evidence and compatibility

This audit was checked against AGENTS.md, docs/SEMANTICS.md §§19.2--19.5,
docs/LANGUAGE.md, docs/ROADMAP-1.0.md, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, DEC-0266, DEC-0268, DEC-0270 through DEC-0274,
docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md,
docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md,
docs/governance/gap-register.toml, docs/governance/protocol-inventory.toml,
the current Actor runtime, its integration tests, and the ACT-2306 property
suite.

The completed implementation changes no compiler, evaluator public entry,
bytecode, VM, public scheduler, mailbox protocol, diagnostic, schema, Semantic
ID, source-span unit, public CLI, or Unicode 17.0.0 behavior. Its test evidence
retains original UTF-8 byte spans and cites Accepted DEC-0275 clauses.

## Intentionally deferred

ACT-2306 does not use this decision to imply source Actor execution, fair or
live scheduling, cross-sender global ordering, concurrent Fault resolution,
watchdogs, graceful drain, supervision, Replay, serialization, remote delivery,
or bytecode/VM/native execution. Those remain separately governed work. The
accepted parallel-turn evidence is only for the local pure, non-suspending,
normal-return profile; it cannot settle later cancellation, restart, or replay
semantics.
