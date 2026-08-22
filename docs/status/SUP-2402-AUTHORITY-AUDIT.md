# SUP-2402 Authority Audit: Restart Budgets and Circuit Breakers

## Outcome

`SUP-2402` is correctly recorded as `BlockedSpec`. The G2 plan names
`restart_count`, `window`, `backoff`, `max_restarts`, `last_fault_provenance`,
and `circuit_state`, and forbids unlimited rapid restarts. It does not define
the time/logical-clock model, budget scope, state transitions, backoff
algorithm, persistence, deterministic/replay behavior, or public observability
needed to implement those fields.

No restart counter, budget, backoff scheduler, circuit breaker, Fault
provenance store, runtime query, diagnostic, protocol, or placeholder G2 API
was added. The plan's stale `zero query runtime` example was not propagated to
the current `ling` CLI or any implementation surface.

Accepted `DEC-0102` now authorizes the bounded child `SUP-2402-OBSERVATION`,
which records only immutable budget/circuit observation identities and
structural labels. It does not close the clock, budget, backoff, circuit,
provenance, query, runtime, or replay gaps described below.

## Normative traceability

- The G2 execution package is non-normative. Its field list and query example
  cannot authorize a restart state machine, clock semantics, debug protocol,
  or public runtime schema.
- SUP-2402 depends on the missing Supervisor model and RFC-C204. No Accepted
  RFC-C204 or replacement RFC-0009 exists; ACT-2301 through ACT-2306 and
  SUP-2401 are `BlockedSpec`, and RFC-0001 remains a Draft baseline under
  DEC-0018.
- `docs/SEMANTICS.md` requires restart intensity, shutdown order, state
  restore, and Fault escalation to be explicit, but it does not define budget
  units, time windows, backoff, circuit transitions, persistence, or replay.
  v0.0.1 has no Actor/Task/Supervisor runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` prohibit unlimited fast restart
  and require queryable Fault provenance, but do not define a stable query
  command, schema, clock, migration, or diagnostic contract. References to the
  plan's `zero` command are stale and cannot enter implementation.
- Accepted DEC-0010/DEC-0013 cover Seed Capability/State and main/runtime
  Faults, DEC-0018 governs RFC lifecycle, DEC-0021 covers only compiler-query
  scheduling, and RFC-0020 excludes Ling Task/Actor scheduling and replay.
  None fixes Supervisor budget/circuit behavior.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open and blocks SUP-2402; its
  required evidence includes stress, ordering, backpressure, and resource
  limits.

## Current implementation evidence

- The workspace has no Actor runtime, Supervisor, restart loop, budget store,
  logical clock, backoff scheduler, circuit state, provenance index, or runtime
  query protocol. `ling-eval` and `ling-vm` only report Seed runtime Faults and
  host cancellation.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no restart/budget/circuit judgments or state. `ling-semantic` has no
  accepted Semantic Graph node for restart counters, windows, provenance, or
  circuit transitions.
- No public protocol inventory entry, diagnostic allocation, fixture, or
  replay record defines restart metrics, budget exhaustion, backoff, circuit
  open/half-open/closed behavior, or cross-process determinism.
- Existing VM resource limits and compiler-query scheduling are not Actor
  restart controls and cannot establish recovery or liveness properties.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. budget scope and units (per child, supervisor, tree, or runtime), counter
   increment/reset rules, logical/monotonic time windows, persistence, and
   deterministic behavior under replay and clock failure;
2. restart/stop/escalate state transitions, backoff and jitter bounds,
   max-restart behavior, circuit closed/open/half-open transitions, concurrent
   Fault handling, and interaction with mailbox, Task cancellation, and
   supervision strategies;
3. Fault provenance identity, deduplication/aggregation, privacy, state
   snapshot/restore, Resource/Managed cleanup, queued-message disposition,
   and parent escalation results;
4. observability fields, metrics/events, diagnostics, Semantic Graph/Audit
   Source projection, query/administration protocol, authorization, versioning,
   migration, local/remote boundary, and replay schema; and
5. executable positive/negative/migration/stress fixtures covering burst and
   window boundaries, backoff, budget exhaustion, circuit transitions,
   repeated/concurrent Faults, restart during cancellation, restore failure,
   mailbox cleanup, clock/replay determinism, resource limits,
   Unicode/CRLF/BOM spans, and interpreter/VM/runtime differential behavior
   without unchecked-AST execution.

Until these decisions are Accepted, implementing counters or a circuit breaker
  would freeze liveness, failure-recovery, timing, observability, and security
  semantics that the language authority intentionally leaves open.

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

`SUP-2402` can begin only after SUP-2401, ACT-2301 through ACT-2306, and
Accepted RFC-C204 (or replacement RFC-0009) resolve Supervisor state,
Actor/mailbox/turn behavior, Fault provenance, clock/replay, and observability
boundaries. The future implementation must consume accepted types and checked
Core only, expose a versioned budget/circuit state machine, enforce bounded
restart/resource behavior, retain deterministic provenance, and publish
recovery, cleanup, and replay evidence before restart control is exposed.
