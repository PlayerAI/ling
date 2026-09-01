# DEC-0278: Private supervision executable evidence matrix / 私有监督可执行证据矩阵

> 状态：Accepted<br>
> 提出日期：2026-09-01<br>
> 决定日期：2026-09-01<br>
> Owner role：actor-supervision<br>
> 相关 RFC/缺口：DEC-0103 | DEC-0274 | DEC-0275 | DEC-0276 | DEC-0277 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | SUP-2403<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable evidence package that can close
SUP-2403 over the already Accepted private local Actor/Supervisor behavior. It
does not authorize a new recovery transition, state restore, escalation,
Replay, public fixture protocol, or backend execution surface.

本决定定义可基于既有 Accepted 私有本地 Actor/Supervisor 行为完成 SUP-2403
的最小可执行证据包。它不授权新的恢复转换、state restore、escalation、Replay、公开
fixture 协议或后端执行面。

## Question

What exact private test matrix may execute DEC-0274 through DEC-0277 against
the real checked-Core runtime, map the stale G2 scenario list to supported or
explicitly unsupported outcomes, and complete SUP-2403 without turning test
fixtures into new language semantics or a compatibility protocol?

## Decision

1. **Scoped authority.** This decision authorizes only a crate-private,
   `cfg(test)` SUP-2403 executable evidence matrix in `ling-eval`. It may test
   behavior already fixed by Accepted DEC-0274 through DEC-0277 and may record
   explicit negative evidence for unimplemented plan scenarios. It adds no new
   runtime behavior and does not close either related specification gap.

2. **Executable input boundary.** Every positive case constructs a successful
   immutable `CheckedProgram` and drives the real DEC-0274 Actor runtime plus
   DEC-0276/DEC-0277 `LocalActorSupervisor`. AST, unresolved HIR, unchecked or
   malformed Core, source text interpretation, the DEC-0103 structural
   observation model, bytecode, VM, Native, Wasm, remote Actors, and public CLI
   entry points are not executable substitutes.

3. **Evidence case set.** The matrix contains exactly these bounded case
   families:
   `contain-one-single-fault`, `contain-one-sequential-faults`,
   `restart-fresh-incarnation`, `restart-initializer-fault`,
   `budget-open-half-open`, `parent-stop-cancel-mailbox-cleanup`,
   `invalid-or-resource-root-fallback`, and
   `unicode-reconstruction-determinism`. New semantic case families require
   separate Accepted authority.

4. **Plan-scenario mapping.** The non-normative G2 checklist is interpreted
   only through Accepted behavior:
   - single and multiple sequential child Faults use DEC-0276 `ContainOne`;
   - "Fault during restart" means the DEC-0277 initializer Fault returned by
     the one serialized replacement attempt, not a concurrent child turn;
   - budget exhaustion means DEC-0277 circuit `Open`, not escalation;
   - parent termination means explicit stop or owner Task cancellation;
   - state restore failure is unsupported because no restore operation exists;
   - unprocessed mailbox cleanup means the accepted discard count and
     exactly-once cleanup evidence, never drain, transfer, or replay.

5. **Explicit unsupported outcomes.** The matrix must prove that escalation,
   state snapshot/restore, concurrent recovery, group strategies, dynamic or
   nested supervision, mailbox transfer, and public recovery queries have no
   callable private production path or public surface. Tests must not fabricate
   placeholder methods, fake Faults, unreachable enum variants, or snapshots
   merely to claim coverage of those plan labels.

6. **Serialized scripts.** Each executable case uses one finite explicit
   command script consisting only of accepted construction, typed send,
   explicit `step(ActorId)`, explicit `advance_to(u64)`, stop, or owner
   cancellation boundaries. No worker race, sleep, wall clock, random runtime
   choice, implicit dispatch, liveness wait, or host scheduling observation may
   determine an expected result.

7. **Bounded test resources.** Each case declares small finite Actor/runtime
   limits, child count, mailbox capacity, command count, logical ticks, and
   restart budget. The complete case table and all retained projections must
   fit those preflighted bounds. Resource exhaustion is tested only at an exact
   accepted boundary and may not be bypassed to finish a scenario.

8. **In-memory projection.** Assertions may compare only the private fields
   already authorized for DEC-0274 through DEC-0277 evidence: runtime and Actor
   identities, canonical Actor-type order, Supervisor/child/Actor lifecycle,
   logical tick, circuit and attempt history, eligible/open deadlines, ready
   identities, queued/discarded-message counts, cleanup counts, bounded runtime
   metrics/events, and canonical Fault phase/category plus original UTF-8 span.
   The projection is in-memory test data, not a serialized fixture or API.

9. **Forbidden observations.** Paths, physical source names or IDs, wall time,
   duration, thread/worker identity, addresses, allocation/layout, hash-map
   order, panic text, Rust debug output, console diagnostics, and host locale
   cannot select or appear in the expected projection.

10. **Exact positive evidence.** The case table must prove sibling preservation,
    closed stale references, fresh monotonically allocated Actor IDs,
    initializer-only replacement state, empty replacement mailboxes, attempt
    consumption, exact half-open window expiry, fixed backoff, circuit
    Open/HalfOpen/Closed transitions, canonical simultaneous due-slot order,
    and no same-boundary retry after initializer Fault.

11. **Exact termination evidence.** Stop and owner cancellation cases must prove
    closed admission, no due replacement after termination, canonical live-child
    shutdown, exact discard counts for admitted unprocessed messages,
    exactly-once cleanup, and idempotent repeated stop. Invalid evidence,
    overflow, and unrecordable resource cases must prove the DEC-0276/DEC-0277
    terminal root fallback without partial restart publication.

12. **Determinism reconstruction.** At least one containment case and one
    restart case are reconstructed from equivalent checked inputs with Unicode
    identifiers/text, BOM, LF/CRLF, different logical source names/IDs, and
    definition insertion order. Their clause 8 projections must be identical
    except for original UTF-8 spans, whose authoritative byte offsets must match
    the corresponding original source bytes.

13. **No cross-process or backend claim.** Repeated in-process execution may
    prove deterministic equality under the explicit scripts, but it does not
    constitute a Replay log, deterministic seed protocol, cross-process
    fixture, interpreter/VM differential, remote result, platform guarantee,
    or performance measurement.

14. **DEC-0103 relationship.** The existing DEC-0103
    `SUP-2403-OBSERVATION` corpus remains a separate vocabulary-only artifact.
    Its opaque labels may be checked for inventory drift, but they are not
    runtime inputs or expected outcomes and cannot override this decision or
    DEC-0274 through DEC-0277.

15. **Public boundary.** No Ling syntax, value, Effect, Capability, Actor or
    Supervisor operation, CLI/REPL/LSP route, public Rust API, diagnostic,
    schema, Semantic ID, Audit/Graph projection, protocol, package/ABI,
    bytecode, VM, Native, Wasm, editor, or migration behavior is added. Public
    Actor-bearing execution continues to stop at `L-ACTOR-0002`.

16. **Completion boundary.** SUP-2403 is Done only when the exact case families
    and negative boundaries above execute against the real private runtime,
    task-specific and full repository gates pass, completion evidence is bound
    to a commit, and the status/backlog/gap records are synchronized. Existing
    tests may be reused only when they visibly assert the required case outcome;
    a list of test names is not executable evidence.

17. **Deferred plan items.** State restore, escalation, concurrent/multiple
    recovery, stable/public fixture schemas, Replay, cross-process/backend
    differential evidence, public Fault queries, migration, fairness, liveness,
    stress/performance guarantees, and Stable compatibility remain blocked.
    Their absence does not prevent this scoped private evidence task from
    completing and must be reported explicitly rather than silently omitted.

## Conformance plan

- Add one dedicated private `ling-eval` supervision evidence module whose case
  table covers all eight clause 3 families and directly drives the real
  checked-Core Supervisor/Actor implementation.
- Require exact snapshots and runtime evidence for single/sequential Faults,
  fresh restart, initializer Fault, circuit exhaustion/probe, stop/cancel with
  queued messages, terminal fallback, and canonical simultaneous recovery.
- Add explicit compile/module-boundary or inventory assertions showing there is
  no restore, escalation, concurrent recovery, public query, serialization, or
  public execution path; do not create placeholder APIs to make negatives pass.
- Reconstruct containment and restart cases across Unicode/BOM/LF/CRLF/source
  identities/insertion order and compare only the allowed bounded projections
  plus corresponding original UTF-8 span evidence.
- Run focused `ling-eval` tests and strict Clippy, the CLI Actor boundary,
  the full locked/offline workspace suite, governance/status/documentation
  gates, formatting, and diff checks before marking SUP-2403 Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, Semantic
  Graph/Audit, protocols, packages/ABI, stored data, bytecode/VM/backends,
  dependencies, and migration: none; the matrix is private `cfg(test)` evidence.
- Runtime: no production transition or public API is added. Tests exercise only
  Accepted private behavior and explicit unsupported boundaries.
- Determinism and Unicode: no new compatibility class is claimed. Unicode
  remains 17.0.0 and original UTF-8 byte spans remain authoritative.

## Unresolved alternatives

- A versioned public supervision fixture/trace schema, deterministic seed and
  Replay protocol, cross-process/platform/backend differential corpus, and
  public query/diagnostic surface require their own Accepted authority.
- State restore and migration, escalation channels, concurrent/group recovery,
  dynamic/nested supervisors, lifetime classes, mailbox drain/transfer, remote
  delivery, fairness/liveness, performance/stress thresholds, and Stable
  support remain outside this decision.
- Broadening SUP-2403 to match every stale plan label is rejected until those
  semantics exist; negative evidence is the truthful outcome for unsupported
  labels in this scoped matrix.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
