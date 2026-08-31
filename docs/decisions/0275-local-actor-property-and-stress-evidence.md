# DEC-0275: Local Actor property and bounded stress evidence / 本地 Actor 性质与有界压力证据

> 状态：Proposed<br>
> 提出日期：2026-08-31<br>
> 决定日期：Pending<br>
> Owner role：actor-runtime<br>
> 相关 RFC/缺口：DEC-0010 | DEC-0013 | DEC-0021 | DEC-0266 | DEC-0268 | DEC-0270 | DEC-0271 | DEC-0272 | DEC-0273 | DEC-0274 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | ACT-2306<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the executable property and bounded-stress evidence needed
for ACT-2306 over the internal Experimental local Actor runtime accepted by
DEC-0274. It deliberately distinguishes independent Actor turn execution from
observable Ling scheduling: it does not add source Actor operations, a public
scheduler, fairness, Replay, supervision, remote delivery, bytecode, VM, Native,
or a performance guarantee.

本提案为 DEC-0274 已接受的内部 Experimental 本地 Actor Runtime 定义完成
ACT-2306 所需的可执行性质与有界压力证据。它严格区分独立 Actor turn 的执行与
可观察的 Ling 调度语义：不新增 Actor 源码操作、公开调度器、公平性、Replay、监督、
远程交付、bytecode、VM、Native 或性能保证。

## Question

What exact bounded test contract can prove same-Actor serialization, independent
Actor parallel-turn safety, mailbox/backpressure limits, stop/Fault/shutdown
cleanup, and reproducible interleavings for DEC-0274 without promoting host
thread timing, worker identity, queue acquisition, or an internal trace into
Ling semantics or a public protocol?

## Decision

1. **Accepted authority only.** ACT-2306 consumes only successful
   `CheckedProgram` inputs and the exact checked Actor Core contracts accepted
   by DEC-0270 through DEC-0274. Property cases must construct the real
   `ling-eval` runtime; they must not interpret AST, unresolved HIR, malformed
   Core, source text, Semantic Graph JSON, or observation-only data as
   executable Actor input. Negative frontend tests may fail before checked Core
   publication only.

2. **Internal evidence boundary.** The suite and any parallel-turn test driver
   are internal Experimental Rust evidence in `ling-eval`. They add no Ling
   source spelling, `ActorRef` value, CLI route, public JSON, trace, benchmark,
   scheduling, mailbox, or replay protocol. Every public Actor-bearing route
   continues to reject execution with `L-ACTOR-0002`.

3. **Observable outcome projection.** Equivalent cases compare only the
   explicit command schedule, run-relative Actor identities, per-sender
   accepted-envelope sequence, each Actor's committed state, bounded
   lifecycle/terminal class, canonical Fault facts, cleanup/discard counts, and
   original UTF-8 source evidence. Worker identity, wall time, duration,
   allocation, OS scheduling, thread count, queue acquisition, physical path,
   and Rust panic/debug payloads are excluded from the oracle.

4. **Same-Actor serialization.** A live Actor has at most one reserved or
   executing turn. A turn reads exactly its prior committed state and one FIFO
   envelope, and it publishes exactly once only on normal return. A failed,
   stopped, cancelled, limit-exhausted, or abandoned reservation publishes no
   candidate state. The property suite must generate repeated same-Actor
   sends/turns and prove serial state progression, one-message consumption, and
   no overlap for that Actor.

5. **Independent Actor parallel turns.** The test driver may execute pure,
   non-suspending, non-faulting turns for distinct live Actors concurrently with
   at most four internal workers. Admission, reservation, lifecycle transition,
   state publication, Fault handling, stop, and shutdown remain coordinator
   boundaries. A batch reserves at most one envelope per Actor; after worker
   completion it commits successful candidates in ascending `ActorId` order.
   The test-only overlap probe must demonstrate that two distinct reserved turns
   can reach a common synchronization barrier without host timing as an oracle.
   This proves runtime isolation and parallel-turn safety only; it does not make
   worker count, overlap, progress, fairness, or cross-Actor order observable
   Ling behavior.

6. **Parallel failure boundary.** The parallel-turn cases are limited to the
   pure, normal-return profile. Fault, cancellation, stop, resource exhaustion,
   and host-unwind containment are exercised at the existing single-turn
   coordinator boundary. A future parallel Fault/restart/cancellation protocol
   requires separate Accepted authority; this proposal must not infer it from
   host completion races.

7. **Mailbox and slow-consumer properties.** A slow consumer is modeled by an
   explicit lack of `step`, never sleep or wall time. Generated cases fill each
   exact `Reject` mailbox and the run-wide queue, require `Full` with the
   unchanged payload and sequence counters, then drain one FIFO turn and retry.
   They prove no mailbox or global queued-message counter exceeds its configured
   limit, no `Wait`, drop, coalescing, implicit retry, or clone occurs, and
   per-sender ordering remains contiguous.

8. **Stop, Fault, and shutdown properties.** Explicit stop, owner cancellation,
   initializer Fault, turn Fault, and explicit shutdown must reject later sends
   as `Closed`, retain no committed candidate from a failed turn, drain
   runtime-owned queued envelopes, and execute cleanup exactly once in canonical
   Actor-ID order. A test-only evaluator panic injection must be contained as
   the existing bounded internal Actor Fault category; it cannot cross the test
   API as an unwind, a Ling catchable value, or host diagnostic text.

9. **Interleaving generator.** The corpus uses a test-local deterministic
   SplitMix64 command generator with explicit source constants for seeds,
   actor bound, mailbox capacity, operation bound, and worker bound. It emits
   only `spawn`, `send`, selected `step`, parallel normal-turn batch, `stop`,
   owner-cancellation observation, and explicit shutdown commands. Every
   failure is retained as a smallest checked source plus its full explicit seed
   and command sequence. Generator bytes, probes, and snapshots are internal
   test artifacts, not Replay input or a compatibility format.

10. **Bounded normal stress.** Normal offline tests execute at least four
    distinct seeds, no more than four Actors, no more than eight queued messages
    per Actor, no more than 256 commands per run, and one- and two-worker
    parallel-turn cases. The same projection must hold for both worker counts.
    These are evidence bounds rather than a throughput, latency, memory, or
    universal-host-capacity promise. Larger soak cases may be ignored or manual
    only and cannot replace the bounded corpus.

11. **Determinism and source reconstruction.** Reconstructed equivalent checked
    programs with differing source names/IDs, insertion order, Unicode names,
    BOM, and LF/CRLF must produce identical outcome projections for the same
    explicit command schedule and seed. Cross-sender admission and completed
    worker order are implementation inputs and are compared only through the
    admitted command order; this decision creates neither Replay nor an
    interpreter/VM/native differential obligation.

12. **Resource and configuration boundaries.** The suite covers zero, one, and
    maximum valid limits plus each relevant overflow/exhaustion boundary for
    created/live Actors, mailbox/global queue, commands, turns, events, Fault
    retention, shutdown work, generator commands, and worker count. Exhaustion
    is failure-atomic and uses existing exact runtime classifications; invalid
    test-driver configuration and test-only injection failure remain typed
    internal errors. No diagnostic allocation is added.

13. **Compatibility and completion boundary.** ACT-2306 is complete only when
    clauses 1 through 12 have executable positive, negative, boundary,
    generated-interleaving, parallel-turn, cleanup, Fault, Unicode/BOM/CRLF,
    and public-boundary evidence; the targeted and relevant offline workspace
    gates pass; and status/traceability identify all intentionally deferred
    runtime surfaces. The completion claim is limited to the Experimental local
    profile defined by DEC-0274 and this decision.

## Conformance plan

- Add an ACT-2306 integration/property suite around the real checked frontend
  and `ActorRuntime`, with a deterministic command generator and retained seed
  fixtures.
- Add one- and two-worker parallel-turn driver cases with a barrier probe for
  two distinct Actors; compare canonical committed state, envelope order,
  lifecycle, terminal record, and cleanup projection rather than worker timing.
- Cover serial same-Actor updates, independent parallel turns, `Full` and
  global queue pressure, slow-consumer backpressure, retry after drain,
  post-stop/terminal sends, initializer/turn/panic Fault containment, owner
  cancellation, shutdown, exact limits, and failure atomicity.
- Reconstruct Unicode, BOM, LF/CRLF, source-name, and insertion-order variants;
  retain `L-ACTOR-0002` on every public execution surface.
- Run targeted `ling-eval`, Actor-boundary, Clippy, formatting, documentation,
  status, and governance gates offline and record only commands actually run.

## Compatibility impact

- Source/CLI: none; public Actor execution remains unavailable through
  `L-ACTOR-0002`.
- Runtime: adds only internal Experimental property/stress and bounded
  parallel-turn test evidence for distinct pure Actors; it creates no public
  scheduler, fairness, liveness, or performance guarantee.
- Diagnostics/schemas/identity: no new diagnostic, Semantic ID, source span
  unit, public schema, trace, protocol, bytecode/VM/native ABI, or migration.
- Determinism/Unicode: uses explicit deterministic test inputs and canonical
  result projection; Unicode remains 17.0.0 and original UTF-8 byte spans are
  preserved.

## Unresolved alternatives

- Source-level spawn/send/stop, ActorRef values, priorities, fairness,
  starvation/liveness, cross-sender global ordering, concurrent Fault handling,
  worker-pool production scheduling, watchdogs, graceful drain, supervision,
  replay, serialization, remote delivery, backend Actor ABI, and Stable
  compatibility require later Accepted authority.
- Raising the four-worker/eight-message/256-command evidence bounds or turning
  them into an SLO requires measured and portable resource evidence. This
  proposal makes no promise that every host can execute all bounded cases in
  parallel.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
