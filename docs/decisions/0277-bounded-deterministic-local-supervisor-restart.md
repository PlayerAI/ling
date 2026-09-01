# DEC-0277: Bounded deterministic local Supervisor restart / 有界确定性本地 Supervisor 重启

> 状态：Proposed<br>
> 提出日期：2026-09-01<br>
> 决定日期：Pending<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0010 | DEC-0013 | DEC-0102 | DEC-0268 | DEC-0270 | DEC-0271 | DEC-0272 | DEC-0273 | DEC-0274 | DEC-0275 | DEC-0276 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | GAP-DETERMINISTIC-REPLAY-001 | SUP-2402<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the smallest internal restart-budget and circuit-breaker
profile that can extend DEC-0276 without introducing wall-clock scheduling,
state restoration, public queries, Replay, remote, or backend semantics. While
it remains Proposed, it is not implementation authority for SUP-2402.

本提案定义可扩展 DEC-0276 的最小内部 restart-budget 与 circuit-breaker
profile，同时不引入 wall clock 调度、状态恢复、公开查询、Replay、远程或后端语义。在状态仍为
Proposed 时，它不是 SUP-2402 的实现权威。

## Question

What exact checked-Core-only local Supervisor policy may replace one failed
fixed child with a fresh incarnation, bound every restart attempt in a
deterministic sliding logical window, and stop a restart storm with an explicit
Closed/Open/HalfOpen circuit without changing any public Ling behavior?

## Decision

1. **Proposed authority boundary.** If Accepted, this decision authorizes only
   an internal Experimental SUP-2402 profile in `ling-eval`. It does not close
   either related gap, authorize SUP-2403, expose Supervisor execution, or make
   restart timing a Ling language or compatibility guarantee. No implementation
   may cite this document while its lifecycle state is Proposed.

2. **Checked inputs and fixed ownership.** The restarting profile consumes the
   same successful immutable `CheckedProgram`, run-owned local Task root,
   DEC-0274 Actor runtime, and fixed duplicate-free child slots accepted by
   DEC-0276. AST, unresolved HIR, source text, malformed Core, Semantic Graph,
   and DEC-0102 observation values are not executable inputs. Children cannot
   be added, removed, detached, migrated, duplicated by Actor type, or nested
   under another Supervisor.

3. **Explicit opt-in policy.** DEC-0276 `ContainOne` remains available and
   unchanged. The new policy is exactly `RestartOneBudgeted`: only the faulting
   slot may be replaced; unaffected siblings retain their committed state,
   admitted mailboxes, readiness, and Actor identities. `RestForOne`,
   `OneForAll`, configurable strategy callbacks, escalation classes, and
   parallel recovery are rejected rather than represented as placeholders.

4. **Configuration.** Construction receives one immutable Supervisor-wide
   configuration `(max_restarts, window_ticks, backoff_ticks)`, applied
   independently to every child slot. Each value is a nonzero integer that must
   fit the implementation's checked logical-tick and collection bounds.
   Configuration is validated and complete worst-case bookkeeping is
   preflighted before Supervisor publication. There is no default, environment
   override, runtime mutation, jitter, exponential schedule, or per-child
   exception.

5. **Logical clock only.** Each Supervisor owns a `u64` logical tick starting at
   zero. Time advances only through an explicit serialized coordinator
   `advance_to(tick)` command with `tick >= current_tick`; send, step, host wall
   time, thread scheduling, sleep, CPU time, and I/O do not advance it.
   Regression is a typed command rejection with no mutation. Tick arithmetic
   uses checked operations; overflow is never wrapped, saturated, or derived
   from a host clock.

6. **Per-slot sliding budget.** A slot retains the logical ticks of replacement
   attempts in ascending order. At tick `t`, attempts at ticks `r` satisfying
   `r + window_ticks > t` are inside the half-open window; an attempt whose
   expiry equals `t` is expired. Initial construction is not a restart attempt.
   Both successful replacement publication and initializer Fault consume one
   attempt. The retained history never exceeds `max_restarts`; expired entries
   are removed only at serialized coordinator boundaries.

7. **Fault acknowledgement and backoff.** A matching DEC-0276 turn-Fault report
   first terminates and cleans the current Actor. The Supervisor prunes the
   slot's attempt history and preflights the full transition. If fewer than
   `max_restarts` attempts remain, it records the canonical Fault provenance,
   enters `Backoff`, and sets `eligible_tick = t + backoff_ticks`. Only this
   complete record acknowledges the Fault and suppresses root cancellation.
   The old Actor reference is permanently closed and no restart occurs before
   the eligible tick.

8. **Circuit opening.** If a matching turn Fault arrives while the retained
   history already contains `max_restarts` attempts, the slot records the Fault,
   enters circuit `Open`, and sets `open_until` to the earliest retained
   attempt's checked expiry. The report may be acknowledged only after that
   state is recordable. An Open slot owns no live Actor, accepts no send or
   step, and performs no replacement before `open_until`; siblings continue.

9. **Canonical replacement.** `advance_to` processes due slots in ascending
   accepted `ActorTypeId` order. A due Backoff slot enters `Restarting`, records
   one attempt at the command tick, and spawns exactly one fresh incarnation
   from the same checked Actor definition. A successful initializer publishes
   a new monotonically allocated `ActorId`, empty mailbox, and initializer
   state, then returns the slot to `Running` with circuit `Closed`. Previous
   state, messages, references, and Actor ID are never restored, transferred,
   reused, cloned, or replayed.

10. **Half-open probe.** At or after `open_until`, `advance_to` prunes the
    expired history, moves the slot to `HalfOpen`, and permits exactly one
    replacement attempt. Successful publication returns the circuit to
    `Closed` and appends the probe tick to any still-active attempt history.
    An initializer Fault during the probe keeps no Actor or candidate state,
    records that attempt and provenance, and reopens the circuit until the
    earliest retained attempt's checked expiry. No second probe occurs at the
    same coordinator boundary.

11. **Initializer Fault outside HalfOpen.** An initializer Fault during an
    ordinary Backoff replacement retires the reserved Actor ID, releases all
    candidate state and resources, records the attempt and canonical
    initializer-Fault provenance, then either schedules another fixed backoff
    if budget remains or opens the circuit at the earliest retained expiry.
    It neither cancels the root nor becomes a Ling-catchable value solely
    because initialization failed. Invalid or unrecordable initializer evidence
    follows the terminal fallback in clause 14.

12. **Fault provenance.** Each slot retains at most one `last_fault` projection:
    run identity, Actor ID and type, checked definition/expression identity,
    initializer or turn phase, original UTF-8 span, registered Fault category,
    discarded-message count, and cleanup count. A newer accepted Fault replaces
    the previous projection after its transition is fully preflighted. Payloads,
    paths, source IDs, wall time, duration, threads, addresses, allocation,
    panic text, and Rust debug output are forbidden.

13. **Serialized interaction.** Fault acknowledgement, clock advancement,
    circuit transition, replacement attempt, explicit stop, and owner
    cancellation are mutually serialized coordinator boundaries. A command
    never observes a partially published replacement. Concurrent Fault or
    recovery, implicit dispatch, fairness, liveness, worker-order, and real-time
    deadlines remain rejected. Existing explicit Actor `step(ActorId)` remains
    the only turn-dispatch operation.

14. **Terminal fallback.** A stale, duplicate, cross-run, wrong-slot,
    wrong-incarnation, inconsistent, overflowed, or resource-unrecordable Fault
    or restart transition fails the Supervisor, closes admission, cancels all
    pending restart/circuit work, stops live siblings in ascending Actor ID
    order, and requests root Task cancellation. Exhausted Actor created/live,
    Fault-retention, command, event, or shutdown-work limits use the same
    terminal fallback; limits are never bypassed to keep restarting.

15. **Stop and cancellation.** Explicit stop or owner Task cancellation closes
    admission before processing due restarts, cancels Backoff/Open/HalfOpen
    work, and never creates another Actor. Live children are stopped in
    ascending Actor ID order with exactly-once cleanup; already failed or
    replaced incarnations and empty slots are not cleaned again. Repeated stop
    remains `AlreadyStopped` and produces no additional attempt, event, or
    cleanup.

16. **Bounded deterministic observation.** Internal tests may observe current
    logical tick, immutable configuration, canonical slot and current Actor
    identity, lifecycle/circuit state, attempts in the active window,
    `eligible_tick` or `open_until`, last Fault provenance, discarded messages,
    and cleanup count. The projection is ordered by `ActorTypeId`/`ActorId` and
    is test evidence only—not a metric, Semantic Graph, Audit, Replay, JSON,
    administration command, or compatibility protocol.

17. **Public boundary.** No Ling syntax, value, Effect, Capability, Actor or
    Supervisor operation, CLI/REPL route, public Rust API, diagnostic, schema,
    Semantic ID, protocol, package/ABI, bytecode, VM, Native, Wasm, LSP, editor,
    or migration behavior is added. Every public Actor-bearing execution route
    continues to stop with `L-ACTOR-0002`. Unicode remains 17.0.0 and original
    UTF-8 byte spans remain authoritative.

18. **Completion boundary.** Acceptance would authorize only the private
    `RestartOneBudgeted` vertical slice and its executable evidence. SUP-2402 is
    Done only after the real DEC-0276 Supervisor/Actor runtime implements these
    transitions and all repository gates pass. SUP-2403, public observability,
    Replay, remote/backend execution, and the open gaps are not promoted by
    accepting or implementing this decision.

## Conformance plan

- Reject zero/overflowing configuration, incompatible resource bounds,
  non-monotonic ticks, unchecked inputs, dynamic/duplicate children, and every
  policy other than existing `ContainOne` or proposed `RestartOneBudgeted`
  before publication.
- Fault a child at known ticks and prove exact half-open window boundaries,
  fixed backoff eligibility, attempt counting for successful and initializer-
  Fault replacements, new Actor IDs, initializer-only state, empty mailboxes,
  closed old references, and byte-identical unaffected siblings.
- Drive `max_restarts` and one additional Fault; prove Open blocks replacement,
  exactly one HalfOpen probe occurs at/after expiry, success closes the circuit,
  initializer Fault reopens it, and no same-boundary retry loop is possible.
- Exercise multiple due slots, insertion-order permutations, repeated Faults,
  stop/cancellation at every Backoff/Open/HalfOpen/Restarting boundary, Actor
  resource exhaustion, tick overflow, malformed reports, and initializer Fault;
  require canonical ordering, atomic publication, exact cleanup, and root
  fallback only for clause 14 failures.
- Reconstruct equivalent checked programs with Unicode identifiers/text, BOM,
  LF/CRLF, different source names/IDs, and physical paths; compare only the
  clause 16 projection and original UTF-8 spans without leaking host facts.
- Keep CLI/interpreter/bytecode/VM/Native/Wasm/LSP/editor Actor routes at
  `L-ACTOR-0002`. Differential, Replay, migration, public-query, timing,
  fairness, liveness, and performance fixtures remain required before their
  respective surfaces can be promoted.

## Compatibility impact

- Source and CLI: none; no syntax, value, operation, clock, query, entry point,
  or public execution route is added.
- Diagnostics, schemas, Semantic IDs, protocols, packages, ABI, and stored data:
  none; configuration, logical ticks, attempts, circuits, and provenance remain
  private Experimental Rust state with no serialization or migration contract.
- Runtime: if Accepted and implemented, adds one explicit private restart
  policy while preserving DEC-0276 `ContainOne` and DEC-0274 no-Supervisor
  behavior. Restarted children receive new runtime identities and initializer
  state; existing public APIs remain unchanged.
- Determinism and Unicode: decisions use explicit logical ticks, checked
  identities, and serialized canonical commands—not host time or scheduling;
  Unicode 17.0.0 and original UTF-8 byte spans are unchanged.

## Unresolved alternatives

- Per-Supervisor/tree/global budgets, distinct per-child configuration,
  exponential or adaptive backoff, jitter, wall/monotonic time, automatic clock
  advancement, persisted circuits, manual reset, and administrative override
  remain unresolved.
- State snapshots/restoration, mailbox transfer or replay, stable references,
  duplicate Actor-type children, dynamic/nested trees, child lifetime classes,
  `RestForOne`, `OneForAll`, escalation, graceful drain, concurrent recovery,
  watchdogs, and user callbacks remain later authority work.
- Public Fault/budget queries, diagnostics, metrics, Semantic Graph/Audit/Replay
  schemas, authorization, versioning, migration, remote supervision, backend
  ABIs, fairness/liveness/performance guarantees, and Stable compatibility are
  not selected by this proposal.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
