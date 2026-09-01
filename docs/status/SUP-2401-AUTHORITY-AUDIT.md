# SUP-2401 Authority Audit: Minimal Local Supervisor Containment

## Outcome

SUP-2401 is Ready for one bounded internal implementation slice under Accepted
DEC-0276. That decision authorizes exactly one optional, run-owned,
non-nested local Supervisor with a fixed duplicate-free child set and the
`ContainOne` policy. A contained child is terminal; unaffected siblings keep
running; no child is restarted or restored.

The authority does not expose Supervisor behavior to Ling source, CLI, REPL,
bytecode, VM, Native, Wasm, LSP, editor, schema, protocol, or package surfaces.
Every public Actor-bearing execution route must continue to stop with
`L-ACTOR-0002`.

## Normative traceability

- Accepted DEC-0274 clauses 1--17 define the checked-Core-only local Actor
  runtime, bounded identities and queues, explicit dispatch, atomic state
  publication, Fault provenance, cleanup, and the no-Supervisor root
  cancellation fallback.
- Accepted DEC-0275 clauses 3--8 define the canonical Actor outcome evidence,
  keep Fault handling at a serialized coordinator boundary, and require
  exactly-once cleanup without promoting host scheduling into Ling semantics.
- Accepted DEC-0268 clauses 5, 9, and 10 define the structured local Task root,
  cancellation token, bounded shutdown, and failure containment boundary that
  owns the Actor runtime and optional Supervisor.
- Accepted DEC-0276 clauses 1--17 define the only executable SUP-2401 profile:
  one optional root, canonical fixed child slots, failure-atomic construction,
  `ContainOne`, synchronous child-Fault acknowledgement, root fallback,
  explicit stop, bounded resources, deterministic evidence, and no public
  execution.
- `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md:326--338` is
  non-normative. DEC-0276 intentionally defers its restart, escalation,
  lifetime, budget, state-restore, and public parent-Fault alternatives rather
  than creating placeholder variants.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open for SUP-2402/SUP-2403,
  restart, escalation, parallel recovery, and public Supervisor behavior. It no
  longer blocks the narrower internal SUP-2401 slice.

## Current implementation boundary

- `crates/ling-eval/src/actor_runtime.rs` already owns the real checked Actor
  registry and performs deterministic spawn, send, step, stop, Fault cleanup,
  and Task-root cancellation.
- Its current turn-Fault path immediately cancels the root and closes every
  sibling. Therefore a correct implementation must modify that coordinator
  boundary so only a valid synchronous Supervisor acknowledgement suppresses
  the existing fallback. A wrapper that observes the result after cancellation
  cannot satisfy DEC-0276.
- Initializer Fault already retires the reserved Actor identity, cleans live
  siblings, fails the runtime, and cancels the owner. This is the required
  failure-atomic Supervisor-construction outcome; no recovery path is needed.
- The existing no-Supervisor constructor and behavior must remain unchanged.
  The new Supervisor surface remains internal to `ling-eval` and must not be
  re-exported as a public Rust embedding API.

## Required implementation evidence

The SUP-2401 implementation must prove:

1. empty, duplicate, unknown, wrong-program, malformed, and over-limit child
   sets fail before Supervisor publication;
2. child definitions and cleanup use canonical checked identity/Actor-ID
   order, and partial construction leaves no live child or visible Supervisor;
3. one turn Fault produces exactly one matching synchronous report, seals only
   that slot, retains no candidate state, closes later operations on that
   Actor, and preserves sibling state, mailboxes, readiness, and later results;
4. invalid, cross-run, wrong-type, stale, duplicate, out-of-order, malformed,
   or unrecordable reports fail the Supervisor, stop all live children, and
   request root Task cancellation;
5. explicit stop and owner cancellation close admission, clean each live child
   exactly once in ascending Actor-ID order, do not clean contained children
   twice, and produce one terminal Supervisor result;
6. command, event, Fault-retention, child-count, queue, and shutdown-work
   bounds are preflighted without partial publication or unsupported recovery;
7. deterministic projections exclude paths, source IDs, insertion/hash order,
   allocation, wall time, threads, worker identity, and Rust debug text while
   preserving original UTF-8 byte spans and Unicode 17.0.0; and
8. all public Actor routes retain `L-ACTOR-0002`, with no restart, restore,
   dynamic/nested tree, public protocol, diagnostic, Semantic ID, schema,
   backend, Replay, remote, or Stable support claim.

## Compatibility impact

The accepted scope changes only an internal Experimental runtime path once it
is implemented. It changes no Ling syntax, public execution route, diagnostic,
schema, protocol, Semantic ID, package/ABI, source-span unit, migration rule,
or Unicode version. Deterministic outcomes remain driven by checked identities
and explicit coordinator commands rather than host timing or container order.

## Intentionally deferred

Automatic restart, replacement identity, restart budgets/windows, backoff,
jitter, circuit breaking, snapshots, state restore, mailbox transfer, nested
or dynamic Supervisor trees, lifetime classes, `OneForOne`, `RestForOne`,
`OneForAll`, escalation, parallel Fault/recovery, source/public Fault channels,
Replay, remote delivery, backend Actor execution, fairness/liveness, and Stable
compatibility remain outside SUP-2401.
