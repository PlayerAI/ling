# TASK-2204 Authority Audit: Deterministic Task Test Scheduler

## Outcome

`TASK-2204` is `Ready` under Accepted DEC-0267. TASK-2201 through TASK-2203
are Done: Accepted DEC-0266 and
implementation commit `e8765790c421f3437049562d69c9aa6d487b5464` provide the
checked, scheduler-neutral `TaskRuntime`, canonical ready set, explicit
`step(id)`, lifecycle cancellation, cleanup, bounds, and Fault aggregation that
TASK-2204 requires.

Accepted DEC-0267 defines the remaining test-only scheduler choices: exact seed
mapping, logical ticks, deadline cancellation, deterministic host events,
typed trace validation, replay equivalence, and bounded interleaving
exploration. It authorizes only the internal Experimental test boundary and
makes no production scheduler or public execution promise.

No scheduler, virtual clock, deadline adapter, seeded selector, replay engine,
interleaving explorer, public trace schema, scheduler diagnostic, production
API, or placeholder G2 surface is added by this audit.

## Normative traceability

- Accepted DEC-0264, DEC-0265, and DEC-0266 now provide the source/Core,
  checked state machine, and scheduler-neutral runtime dependencies. Public
  Task execution remains excluded and rejected with `L-TASK-0004`.
- Accepted DEC-0094 authorizes only immutable, non-executable scheduler
  observation identities. It deliberately does not choose a seed algorithm,
  queue order, clock, deadline, exploration, replay, or production behavior.
- Accepted DEC-0019 and DEC-0021 govern immutable compiler-query scheduling,
  not Ling Task execution. Their priority/FIFO details cannot be copied into a
  Task scheduler without separate authority.
- RFC-0020 governs host-owned VM cancellation only. DEC-0266 governs source
  Task cancellation, but its `Deadline` cause requires a future explicit
  logical Clock adapter and it assigns no scheduler policy.
- `docs/ROADMAP-1.0.md` requires deterministic scheduling evidence for v0.2.
  Accepted DEC-0267 closes the TASK-2204 portion of
  `GAP-STRUCTURED-TASK-001`; the gap remains open for TASK-2205 and TASK-2206.

## Current implementation evidence

- `ling-eval::TaskRuntime` consumes checked Task Core/machine evidence, retains
  canonical lexical Task paths, exposes a sorted ready set, and accepts only an
  explicit ready Task selection. It never selects among Tasks or reads time.
- Runtime tests already prove registration, suspension/wake, multiple live
  handles, cancellation, cleanup, Fault precedence/aggregation, explicit
  bounds, opposite caller-selected schedules, deterministic identities, and
  original Unicode/BOM/CRLF spans.
- The existing `ling-concurrency::SchedulerObservationTrace` is data-only. It
  cannot drive `TaskRuntime`, inject a deadline, reproduce a run, or explore an
  interleaving.
- Compiler-query scheduling, bytecode/VM execution, and public CLI/project
  routes remain separate. No Task scheduler, Clock, trace replay, or public
  Task execution path exists.

## Accepted implementation contract

Under Accepted DEC-0267, TASK-2204 must implement:

1. an internal publish-disabled scheduler over DEC-0266 only, with exact
   SplitMix64 selection from canonical ready snapshots and explicit non-zero
   decision/time/deadline/trace/exploration bounds;
2. `u64` logical ticks independent of wall time, canonical due-deadline
   injection as `Deadline` cancellation, and no fabricated I/O wake queue;
3. a bounded deterministic test host plus validated typed traces that record
   ready sets, choices, step outcomes, deadlines, host events, terminal state,
   cleanup counts, and canonical Faults without paths or host timing;
4. strict replay against a freshly reconstructed checked runtime, with the
   first exact mismatch and no seeded fallback; and
5. canonical breadth-first exploration of explicit schedule prefixes, with
   shortest-then-lexicographic failures and explicit incomplete results at
   every bound.

## Evidence and compatibility

The refreshed audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, RFC-0020, Accepted DEC-0019,
DEC-0021, DEC-0094, and Accepted DEC-0264 through DEC-0267,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`, the backlog, gap and
protocol registries, the current Task runtime, and public Task rejection tests.

The audit and proposal change no source, CLI, diagnostic, schema, Semantic ID,
Audit, bytecode, VM, artifact, production runtime, source span, or Unicode
17.0.0 behavior. Proposed typed trace bytes remain internal fixtures and are
not a public protocol or compatibility promise.

## Intentionally deferred

TASK-2204 may now proceed under Accepted DEC-0267. TASK-2205 retains
production workers, queues, wakes, fairness, metrics, shutdown, and public
execution integration. TASK-2206 retains stress, million-short-task, race,
shutdown, and final conformance evidence. Public trace/replay, source Clock/sleep, I/O wake
injection, Task bytecode/VM/native ABI, detach, Resource finalizers, Replay,
Actor crossing, migration, and Stable compatibility remain separately
governed.
