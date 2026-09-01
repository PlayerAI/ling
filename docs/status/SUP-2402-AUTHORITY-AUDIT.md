# SUP-2402 Authority Audit: Restart Budgets and Circuit Breakers

## Outcome

`SUP-2402` is Done. SUP-2401 is complete under Accepted DEC-0276, Accepted
DEC-0277 fixes the scoped `RestartOneBudgeted` contract, and implementation
commit `be9b8dd842815cff100267a645b9bef88b13c369` supplies the required private
checked-Core recovery path and executable evidence.

Accepted DEC-0277 supplies the minimal design: an opt-in private
`RestartOneBudgeted` profile with explicit logical ticks, an independent
per-child sliding attempt budget, fixed backoff, Closed/Open/HalfOpen circuit
states, fresh Actor identities, initializer-only state, and bounded last-Fault
provenance. No public restart counter, scheduler, circuit, runtime query,
diagnostic, protocol, or placeholder API was added.

Accepted DEC-0102 authorizes the bounded child `SUP-2402-OBSERVATION`,
which records only immutable budget/circuit observation identities and
structural labels. It does not close the clock, budget, backoff, circuit,
provenance, query, runtime, or replay gaps described below.

## Normative traceability

- The G2 execution package is non-normative. Its field list and query example
  cannot authorize a restart state machine, clock semantics, debug protocol,
  or public runtime schema.
- SUP-2402 depends on SUP-2401 and the absent RFC-C204. ACT-2301 through
  ACT-2306 and SUP-2401 are now Done under scoped Accepted decisions, but no
  Accepted RFC-C204 or replacement public-supervision RFC exists. RFC-0001
  remains a Draft baseline under DEC-0018.
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
  Accepted DEC-0276 authorizes fixed-child `ContainOne` only and explicitly
  requires separate SUP-2402 authority before restart. Accepted DEC-0277 is
  that scoped private authority.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open for broader/public Supervisor
  behavior, SUP-2403, escalation, parallel recovery, Replay, ordering,
  backpressure, and resource evidence. It no longer blocks DEC-0277's narrower
  private SUP-2402 slice.

## Current implementation boundary

- `crates/ling-eval/src/actor_runtime.rs` now supplies the real private
  checked-Core Actor registry, fresh monotonically allocated runtime Actor IDs,
  bounded spawn/send/step/stop, retained Actor Faults, and structured Task-root
  cancellation under Accepted DEC-0274.
- `crates/ling-eval/src/actor_supervisor.rs` supplies the private fixed-child
  DEC-0276 Supervisor and the opt-in DEC-0277 `RestartOneBudgeted` profile. It
  retains `ContainOne`, validates canonical Fault evidence, owns an explicit
  logical clock and per-slot bounded attempt history, performs fixed-backoff and
  Closed/Open/HalfOpen transitions, and publishes only fresh checked
  incarnations in canonical Actor-type order.
- `crates/ling-eval/src/actor_runtime.rs` keeps ordinary spawn behavior
  unchanged and adds one crate-private restart-spawn path that returns an
  initializer Fault to the Supervisor without publishing an Actor or
  cancelling the root solely for that accepted recovery outcome. Aggregate
  restart preflight preserves Actor, command, Fault, shutdown, and event bounds.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no restart/budget/circuit judgments or state. `ling-semantic` has no
  accepted Semantic Graph node for restart counters, windows, provenance, or
  circuit transitions.
- No public protocol inventory entry, diagnostic allocation, fixture, or
  replay record defines restart metrics, budget exhaustion, backoff, circuit
  open/half-open/closed behavior, or cross-process determinism.
- Existing VM resource limits and compiler-query scheduling are not Actor
  restart controls and cannot establish recovery or liveness properties.

## Accepted implementation boundary

DEC-0277 authorizes the following minimum private vertical slice for SUP-2402:

1. one opt-in `RestartOneBudgeted` policy over DEC-0276's existing fixed slots,
   with one immutable `(max_restarts, window_ticks, backoff_ticks)`
   configuration applied independently per slot;
2. a Supervisor-local explicit `u64` logical tick, exact half-open sliding
   window membership, fixed nonzero backoff, and checked overflow/regression
   behavior independent of wall time or host scheduling;
3. Closed/Open/HalfOpen transitions, exactly one canonical due attempt per slot
   at a boundary, fresh Actor identity and initializer state, no old mailbox or
   state transfer, and bounded initializer-Fault retry behavior;
4. canonical last-Fault provenance, failure-atomic transition preflight,
   interaction with existing Actor limits and Task cancellation, exact cleanup,
   and root fallback for invalid or unrecordable recovery; and
5. internal deterministic snapshots and executable boundary/stress evidence,
   while retaining `L-ACTOR-0002` and adding no public query, schema, protocol,
   diagnostic, Replay, remote, or backend surface.

Its explicit non-goals keep public observability, Replay, state restore, group
strategies, remote behavior, fairness/liveness, and Stable compatibility under
their existing gaps.

## Evidence and compatibility

This audit, Accepted DEC-0277, and the completed implementation were checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, DEC-0102, DEC-0274, DEC-0275, DEC-0276, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

Focused `ling-eval` tests and Clippy, the CLI Actor boundary suite, the complete
workspace all-target test gate, formatting, and diff checks passed. The public
compiler, interpreter, VM, bytecode, scheduler, Actor/Supervisor protocol,
diagnostic, schema, Semantic ID, source-span, and Unicode 17.0.0 boundaries did
not change; public Actor execution remains `L-ACTOR-0002`.

## Intentionally deferred

SUP-2402 completes only DEC-0277's scoped private profile. SUP-2403, wall-clock
or jitter scheduling, mutable or persisted configuration, state
snapshot/restore, mailbox transfer, stable
references, dynamic/nested trees, lifetime classes, group restart, escalation,
parallel recovery, public query/metrics/Fault protocols, Replay, remote
delivery, backend Actor execution, migration, fairness/liveness/performance,
and Stable compatibility remain later Accepted work.
