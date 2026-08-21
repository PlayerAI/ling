# ACT-2306 Authority Audit: Actor Properties and Stress Tests

## Outcome

`ACT-2306` is correctly recorded as `BlockedSpec`. The G2 plan asks for
property and stress evidence for serialized state mutation, actor parallelism,
bounded full mailboxes, slow-consumer backpressure, post-stop sends, Fault
cleanup, declared message ordering, and process-shutdown cleanup. These are
acceptance properties of the still-unresolved Actor runtime; they cannot be
made meaningful by adding an independent test harness or by treating host
thread behavior as Ling semantics.

No Actor runtime, scheduler, stress protocol, property schema, replay format,
fixture corpus, diagnostic, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its property list does not
  authorize runtime behavior, a determinism class, a stress-result schema, or
  cross-backend equivalence claims.
- ACT-2306 depends on ACT-2305 and therefore on the missing RFC-C203/C204
  Actor identity, mailbox, turn, and supervision contracts. The interleaving
  and replay claims also require RFC-C205/RFC-0010. None of RFC-C203/C204/C205
  or replacement RFC-0009/RFC-0010 is Accepted; RFC-0001 remains a Draft
  baseline under DEC-0018.
- `docs/SEMANTICS.md` states high-level Actor constraints (one turn at a time,
  bounded mailbox, same-sender ordering, and cleanup/Fault obligations), but
  v0.0.1 implements no Actor/Task Core forms or runtime. It does not define
  the scheduler model, allowed interleaving equivalence, resource budgets,
  shutdown marker, or replay comparison relation needed to interpret these
  properties.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require concurrency stress,
  deterministic observable behavior, and cleanup evidence, but do not define
  the executable fixture schema, platform scope, random-seed contract, or
  migration rules.
- Accepted DEC-0021 defines deterministic scheduling only for independent
  internal compiler queries. It explicitly does not define Actor scheduling,
  message order, runtime parallelism, or replay. Accepted DEC-0010/DEC-0013
  cover current Seed State/Capability and main/runtime Fault boundaries only;
  RFC-0020 covers host-VM cancellation and excludes Task/Actor scheduling.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-DETERMINISTIC-REPLAY-001` remain Open and leave the tested semantics
  unresolved.

## Current implementation evidence

- The workspace has no Actor, Task, scheduler, mailbox, supervision, or replay
  runtime. `ling-eval` and `ling-vm` execute the Seed checked subset only;
  existing VM resource/cancellation tests cannot establish Actor properties.
- There is no accepted Actor source/Core form, typed envelope, queue policy,
  turn state, scheduler seed, interleaving model, process-shutdown event,
  Fault provenance schema, or runtime resource counter for a property harness
  to observe.
- Existing compiler-query scheduling tests are deliberately internal and
  deterministic. They cannot be reused as evidence that different Actors may
  run in parallel or that mailbox order is preserved under random runtime
  interleavings.
- No conformance/property corpus covers serialized state, actor parallelism,
  full/slow mailboxes, post-stop sends, turn Fault cleanup, declared ordering,
  shutdown cleanup, Unicode/CRLF/BOM source spans, or interpreter/VM/runtime
  differential equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the semantic property relation: what counts as serial state isolation,
   allowed Actor parallelism, declared per-sender/global ordering, and
   equivalent versus divergent interleavings;
2. mailbox capacity/backpressure and post-stop send outcomes, resource limits,
   slow-consumer progress, fairness, starvation, drop/coalesce observability,
   and shutdown/drain/discard behavior;
3. turn/await/reentry, cancellation, Fault, supervision, and cleanup
   invariants, including host-unwind containment and process termination
   markers;
4. deterministic scheduler/replay inputs, random-seed and logical-time
   handling, platform scope, trace/event schema, privacy, version migration,
   and comparison tolerances for Interpreter/VM/runtime backends;
5. evidence status and acceptance thresholds for stress bounds, memory/resource
   accounting, liveness/watchdog limits, and failure recovery; and
6. executable positive/negative/interleaving/stress fixtures covering serial
   state updates, independent Actor parallelism, full and slow mailboxes,
   post-stop sends, Fault and cancellation cleanup, random ordering, process
   shutdown, resource exhaustion, Unicode/CRLF/BOM spans, deterministic
   reruns, and cross-backend equivalence without unchecked-AST execution.

Until these decisions are Accepted, a property test could certify an
implementation-defined schedule or silently turn host timing, dropped data,
or cleanup behavior into language semantics.

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
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`ACT-2306` can begin only after ACT-2301 through ACT-2305 and Accepted
RFC-C203/C204/C205 (or replacement RFC-0009/RFC-0010) resolve Actor identity,
message ownership, mailbox/backpressure, turn/reentry, supervision, runtime
ABI, and determinism/replay boundaries. The future test suite must consume
accepted fixtures and checked Core/runtime traces only, distinguish language
properties from implementation performance, and publish bounded stress,
cleanup, ordering, and cross-backend evidence before claiming Actor support.
