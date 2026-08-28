# DEC-0267: Deterministic Task test scheduler and virtual time / 确定性 Task 测试调度器与虚拟时间

> 状态：Proposed<br>
> 提出日期：2026-08-28<br>
> 决定日期：Pending<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0094 | DEC-0266 | GAP-STRUCTURED-TASK-001 | TASK-2204<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal closes only the deterministic, test-only scheduler boundary of
TASK-2204. It does not authorize a production scheduler, public Task entry,
wall-clock API, sleep syntax, worker thread, bytecode/VM Task ABI, Replay
protocol, detach, Actor crossing, or Stable scheduling guarantee.

本提案仅关闭 TASK-2204 的确定性测试调度器边界，不授权生产 scheduler、公开 Task
入口、wall-clock API、sleep 语法、worker thread、bytecode/VM Task ABI、Replay
协议、detach、Actor crossing 或 Stable 调度保证。

## Question

What exact bounded test driver may select among DEC-0266 `TaskRuntime` ready
Tasks, inject logical deadlines, reproduce one run, and explore finite
interleavings without turning its seed mapping, wake order, virtual clock, or
trace representation into production Ling semantics?

## Decision

1. **Authority and input.** TASK-2204 adds an internal Experimental,
   publish-disabled deterministic test scheduler in `ling-eval`. It constructs
   or drives only a DEC-0266 `TaskRuntime`, its exact checked root recipe, and a
   deterministic test host. It never consumes AST, source text, unchecked HIR,
   bytecode, VM state, production host capabilities, or a public Task entry.

2. **Configuration and bounds.** Construction requires explicit non-zero
   limits for scheduling decisions, virtual tick, deadline records, trace
   events, exploration runs, exploration depth, and ready-set width. It also
   receives one `u64` seed; every bit pattern, including zero, is valid. Invalid
   limits, duplicate deadline identities, oversized inputs, and arithmetic
   overflow fail before a scheduler run or trace is published.

3. **Canonical ready input.** At every decision the scheduler obtains
   `TaskRuntime::ready()`, verifies strict canonical Task-path ordering and
   uniqueness, and selects only from that immutable snapshot. An empty set is
   terminal only when the root is terminal. Otherwise the driver advances to
   the next pending logical deadline or reports a bounded quiescence failure;
   it never parks a thread, polls wall time, or invents a wake.

4. **Exact seed mapping.** Seeded selection uses SplitMix64 with wrapping
   `u64` arithmetic. The initial state is the supplied seed. One decision adds
   `0x9E3779B97F4A7C15`, then applies xor-shift/multiply stages
   `(z ^ (z >> 30)) * 0xBF58476D1CE4E5B9`,
   `(z ^ (z >> 27)) * 0x94D049BB133111EB`, and `z ^ (z >> 31)`.
   The selected canonical-ready index is `output mod ready_count`. Exactly one
   draw is consumed per Task selection; deadline processing, trace recording,
   terminal detection, replay validation, and failed preflight consume none.
   This mapping is test evidence only and makes no fairness or production
   ordering promise.

5. **Virtual clock.** Logical time is a `u64` tick starting at zero. One
   successful `TaskRuntime::step` advances it by exactly one. If no Task is
   ready, it may jump only to the smallest pending deadline tick. Time never
   follows CPU duration, wall time, timezone, sleep, Effect latency, or worker
   activity. Tick overflow or the configured maximum is detected before the
   step or jump and ends the run with a bounded scheduler failure.

6. **Deadline injection.** A deadline record is `(tick, canonical TaskPath)`.
   Due records at a tick are processed in Task-path order before seeded
   selection and request DEC-0266 cancellation with cause `Deadline`.
   Unknown-yet-due paths fail without advancing time or executing a Task;
   terminal Tasks remain terminal. Deadline is not source syntax, an Effect,
   a wall-clock timer, or a general wake API.

7. **Wake ordering.** DEC-0266 alone creates lifecycle readiness when a child
   terminates or cancellation/Fault propagation becomes mandatory. TASK-2204
   adds no second wake queue. All simultaneously ready lifecycle continuations
   enter the next canonical ready snapshot, and the seeded or replay driver
   selects among them. “Controlled wake order” therefore means explicit
   control over these ready selections and deadline injections, not fabricated
   I/O readiness or a production wake contract.

8. **Deterministic test host.** Scheduler execution uses an injected bounded
   test host that records canonical Console events and configured failures. A
   completed event remains committed exactly as in DEC-0266. Replay and
   exploration compare event order and payload bytes; they do not suppress,
   roll back, or repeat a production Effect. Host panics become the existing
   checked Runtime Fault boundary and publish no guessed scheduler event.

9. **Typed trace.** A completed or failed run returns an immutable
   `TaskScheduleTrace` containing the seed and limits, initial checked-runtime
   identity, deadline inputs, and ordered events. Each selection event records
   the pre-step tick, complete canonical ready set, selected Task path, and
   resulting `TaskStepKind`; deadline, host-event, terminal-state, and canonical
   Fault summaries are explicit events. Event identities are consecutive from
   one and the trace has exactly one closure event.

10. **Trace boundary.** The trace is returned as a typed internal value and may
    expose deterministic canonical bytes for fixtures. Construction validates
    version, bounds, consecutive event identities, ready membership, monotonic
    time, deadline order, one closure, and path shape before publication.
    Canonical bytes exclude filesystem paths, source IDs, source spans, Rust
    debug text, addresses, allocation/container order, and host timing. Exact
    source spans remain sidecar evidence. No file decoder, CLI flag, schema,
    protocol-inventory entry, or Stable compatibility promise is added.

11. **Replay equivalence.** Replay reconstructs a fresh runtime from the same
    checked root recipe and deterministic test-host script. Before every step
    it requires the recorded ready set and tick exactly, selects the recorded
    path, and compares step kind, host events, terminal state, cleanup counts,
    and canonical Fault set. The first mismatch is a structured internal replay
    error with event identity; replay never falls back to seeded choice. Source
    paths, source IDs, BOM/CRLF encoding, and spans may differ only where the
    trace explicitly excludes them from logical equivalence.

12. **Bounded exploration.** Exploration is canonical breadth-first over
    explicit ready-choice prefixes. Alternatives use canonical Task-path order;
    each prefix reconstructs a fresh runtime and deterministic host. The
    configured run, depth, ready-width, decision, deadline, tick, and trace
    limits are checked before enqueue or execution. Duplicate prefixes are
    rejected by canonical identity. Exhaustion returns an explicit incomplete
    result rather than a coverage claim. The first failing trace is therefore
    shortest by decision count and then lexicographically smallest; no hidden
    nondeterministic shrinker is used.

13. **Failure precedence and atomicity.** A DEC-0266 Task Fault remains a Task
    outcome and retains its existing Fault/cancellation/cleanup precedence.
    Scheduler configuration, trace, replay, quiescence, or exploration failures
    are internal driver errors and never become catchable Ling data. A failure
    is detected before its scheduling mutation; already completed runtime
    steps and test-host Effects remain committed. No new public diagnostic code
    is allocated.

14. **No production semantics.** Seed selection, logical ticks, deadline
    injection, breadth-first exploration, trace bytes, and replay checks are
    test tools, not observable guarantees of TASK-2205. They cannot be selected
    from Ling source, CLI/project commands, artifacts, LSP, bytecode, VM, or a
    package manifest. Existing public paths continue to reject checked Tasks
    with `L-TASK-0004`.

15. **Completion boundary.** TASK-2204 is complete only with positive,
    negative, boundary, determinism, replay, exploration, virtual-time,
    cancellation, cleanup, Fault, host-event, Unicode/span, and reconstruction
    tests over DEC-0266. TASK-2205 retains production scheduling and public
    integration; TASK-2206 retains stress, shutdown, race, and final
    conformance evidence.

## Conformance plan

- Freeze SplitMix64 vectors for zero, maximum, and representative seeds;
  compare repeated runs and equivalent checked reconstruction across physical
  paths, source IDs, insertion order, BOM/CRLF, and Chinese identifiers.
- Drive empty, single-ready, multiple-ready, nested, fused `let!`, multi-child,
  cancellation, cleanup, owner/child/transitive Fault, and committed/failed
  Console cases; require exact ready snapshots, ticks, choices, states, paths,
  host events, cleanup counts, and Fault sets.
- Cover deadlines before start, while ready, while suspended, during Fault
  drain, after completion, at equal ticks, at the configured maximum, and at
  overflow; require canonical path order and no wall-clock dependency.
- Round-trip typed traces and canonical fixture bytes; reject bad versions,
  duplicate or skipped event IDs, noncanonical ready/path/deadline order,
  selected paths outside the ready set, time regression, missing/multiple
  closures, truncation, and every explicit size bound before publication.
- Replay successful, cancelled, Faulted, host-failed, and deadline runs; mutate
  each decision/ready/tick/step/host/terminal field and require the first exact
  mismatch without fallback or partial success.
- Explore bounded two- and three-child programs; compare the canonical set and
  order of prefixes/failures across reconstruction, prove shortest-then-
  lexicographic failure selection, and report limit exhaustion as incomplete.
- Retain all public `L-TASK-0004` file/project run/test/build, REPL, artifact,
  bytecode 1.0–1.4, and VM rejection evidence and ordinary Seed/Handler
  interpreter/VM differentials.

## Compatibility impact

- Source/CLI/runtime: no source syntax, public entry point, production
  scheduler, wall clock, worker, package, CLI/LSP, artifact, bytecode, VM, or
  ABI behavior changes.
- Diagnostics: no code or existing meaning changes. Internal scheduler/replay
  errors are typed Rust errors; Task Runtime Faults retain `L-RUNTIME-0001` and
  public Task execution retains `L-TASK-0004`.
- Schemas/protocols/Semantic IDs: none. Typed traces and canonical bytes are
  internal fixtures, not a public protocol, Semantic Graph, Audit, or Replay
  revision.
- Determinism/Unicode: exact seed vectors, canonical Task paths, logical ticks,
  ordered inputs, and bounded reconstruction make test results host-independent.
  Original UTF-8 spans and Unicode 17.0.0 remain authoritative sidecar evidence.

## Unresolved alternatives

- Random crates with unstated algorithms, entropy seeds, wall-clock timers,
  eager child priority, FIFO as language semantics, implicit fairness,
  occurrence-time Fault precedence, thread parking, production Effects during
  replay, and unbounded state-space search are rejected.
- A public trace file, decoder, CLI replay command, source Clock/sleep Effect,
  I/O wake injection, production scheduler, worker pool, work stealing,
  metrics, shutdown, Task bytecode/VM/native ABI, detach, user Resource
  finalizers, Replay integration, Actor crossing, migration, and Stable
  compatibility remain unresolved later work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
