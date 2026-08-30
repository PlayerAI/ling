# DEC-0269: Task conformance and stress evidence / Task 一致性与压力证据

> 状态：Proposed<br>
> 提出日期：2026-08-30<br>
> 决定日期：Pending<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0264 | DEC-0265 | DEC-0266 | DEC-0267 | DEC-0268 | GAP-STRUCTURED-TASK-001 | TASK-2206<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes only the conformance and bounded stress evidence for the
Experimental Structured Task profile accepted by DEC-0264 through DEC-0268. It
does not add Task syntax, user Resource finalizers, detach, wall-clock time,
Task bytecode/VM/native execution, a public trace or benchmark protocol, or a
Stable performance claim.

本决定仅关闭 DEC-0264 至 DEC-0268 已接受的 Experimental Structured Task
profile 的一致性与有界压力证据；不新增 Task 语法、用户 Resource finalizer、
detach、墙钟时间、Task bytecode/VM/native 执行、公开 trace/benchmark 协议或
Stable 性能承诺。

## Question

What exact executable evidence is sufficient to complete TASK-2206 without
mistaking lower-authority planning examples for new language semantics,
exposing production scheduling order, or turning machine timing and allocation
accidents into Ling compatibility promises?

## Decision

1. **Authority boundary.** The oracle is exactly the accepted behavior in
   DEC-0264 through DEC-0268. Every executable case begins with a successful
   `CheckedProgram` and its Checked Task Core/machine, except negative frontend
   cases that must fail before checked publication. Tests never interpret AST,
   unchecked HIR, malformed Core, or scheduler output as new semantics.

2. **Outcome projection.** Equivalent runs compare only terminal class and
   value, canonical Fault facts and ordering, canonical Task paths, cleanup
   multiplicity, committed host events subject to the admitted partial order,
   registered diagnostics, and original UTF-8 source evidence. Worker identity,
   queue acquisition order, park/wake counts, duration, addresses, allocation,
   physical paths, and Rust debug text are excluded.

3. **Parent exit and structured drain.** A parent normal return remains blocked
   by checked handle observation and lexical join. Parent cancellation or Fault
   before a child completes must monotonically cancel and drain descendants,
   retain already committed Effects, enter every accepted cleanup identity once,
   and leave no nonterminal Task in the final snapshot.

4. **Cleanup terminology.** The G2 plan's `Resource` example is satisfied in
   this profile only by the accepted runtime-owned handle, frame, scope-registry,
   and cleanup-identity obligations. `ROADMAP-1.0` assigns user `Resource`,
   move/drop, finalizers, and allocator behavior to G3. Tests must not fabricate
   a user cleanup callback or claim Resource finalizer conformance before that
   separate authority exists.

5. **Fault races.** Before owner propagation, independently faulting siblings
   may both reach a checked boundary. The final set is schedule-independent:
   owner Fault is primary when present; otherwise the smallest canonical child
   path is primary, with remaining causes in canonical order. Fault dominates
   cancellation, which dominates normal return. Production scheduling may
   choose an admitted race but may not change this projection.

6. **Logical deadlines.** Timeout-versus-completion cases use only DEC-0267
   logical ticks and explicit `TaskDeadline` input. A deadline applied before a
   terminal transition produces `Deadline` cancellation; a deadline observed
   after terminal completion is recorded as unapplied and cannot change the
   value. Equal-tick deadlines are ordered by canonical Task path. No wall clock,
   sleep, polling interval, timezone, or host timer participates.

7. **Nested scopes.** Evidence includes at least two lexical scope levels and
   descendant Tasks. Inner scopes close and drain before outer scopes; each Task
   reaches one reason-preserving cleanup edge, and the root cannot publish a
   terminal result while a scope or descendant remains live.

8. **Detach rejection.** `detach` remains unavailable: it has no accepted
   syntax, Capability, Checked Task command, runtime transition, or scheduler
   operation. A source spelling that attempts to call or reference `detach`
   must fail through the existing registered frontend diagnostic before Checked
   Task publication. Internal graph types must not be exposed as language
   authority. No valid-detach fixture is required until a later Accepted RFC.

9. **Million-Task ceiling.** Production local scheduling accepts a configured
   run-wide Task limit no greater than exactly `1_000_000`. A value of
   `1_000_000` is a valid bound; `1_000_001` and overflow-derived values fail
   configuration validation before worker creation, Task allocation, or host
   Effect. This is a hard capacity ceiling, not a requirement to materialize one
   million simultaneous Tasks and not a throughput, latency, or memory claim.

10. **Representative stress.** Normal repository tests run a bounded generated
    short-Task workload through the real checked frontend, runtime, and local
    scheduler, repeat it across one and multiple workers, and require identical
    outcome projections and exactly-once cleanup. Workload size, repetitions,
    worker counts, and all limits are source constants; completion has no time
    threshold. Larger soak runs may be ignored/manual but cannot replace the
    normal bounded case or create compatibility claims.

11. **Shutdown oracle.** Normal, cancellation, Fault, quota, and contained host
    failure paths must wake and join every started worker. Successful scheduler
    publication requires a terminal structured runtime and cleanup count one for
    every retained Task. A contained panic or poisoned coordinator returns the
    bounded internal error category, never success or a catchable Ling Fault;
    no panic payload or host identity is published.

12. **Deterministic and production differential.** Reconstructed checked input,
    repeated deterministic seeds, opposite admitted ready choices, and local
    worker counts compare the outcome projection in clause 2. Deterministic
    trace canonical bytes remain internal typed evidence; local metrics may
    differ and cannot influence results. Interpreter/VM differential is not
    claimed because accepted Task bytecode/VM execution does not exist.

13. **Bounds and failure classification.** Runtime Task/scope/step/Fault limits,
    deterministic decision/tick/trace/exploration limits, and local queue,
    direct-child, transition, park/wake, shutdown, worker, and million-Task
    limits receive boundary evidence. Expected exhaustion uses existing
    `L-RUNTIME-0001` resource categories; invalid scheduler configuration and
    contained host implementation failure remain typed internal errors. No new
    diagnostic is allocated.

14. **Corpus and failure retention.** TASK-2206 uses Rust integration tests as
    the executable corpus. It adds no public trace, stress-result, replay, or
    benchmark schema. A discovered failure is retained as the smallest checked
    source plus explicit seed/configuration and asserted projection; machine
    duration and ambient host capacity are not oracle fields.

15. **Unicode and compatibility.** Negative and runtime evidence retains
    original UTF-8 byte spans across Chinese identifiers, BOM, CRLF, and
    differing source names/IDs. No Unicode table, normalization, Semantic ID,
    Audit Source, public schema, CLI exit contract, bytecode, ABI, or protocol
    version changes.

16. **Completion boundary.** TASK-2206 is complete only when executable evidence
    covers clauses 1 through 15; targeted tests pass; workspace Clippy, docs,
    governance, formatting, and the applicable offline test gates pass; and the
    implementation report names any environment-specific gate that was actually
    attempted but could not establish portable evidence.

## Conformance plan

- Add a TASK-2206 integration suite covering early parent Fault/cancellation,
  sibling Fault aggregation, logical deadline races, nested scopes, invalid
  detach, the exact million-Task configuration ceiling, bounded generated
  short-Task stress, and terminal shutdown snapshots.
- Reuse the DEC-0266 runtime and DEC-0267 deterministic-scheduler suites for
  opposite-order Fault aggregation, cancellation boundaries, canonical traces,
  exploration bounds, deadline ordering, and Unicode/BOM/CRLF reconstruction.
- Repeat production outcomes with one and multiple workers; compare terminal
  class/value, canonical Fault facts, final tree, cleanup counts, and admitted
  host events while excluding metrics and acquisition order.
- Run the targeted crates and repository gates offline with locked dependencies;
  document commands actually executed and do not convert host timing into pass
  criteria.

## Compatibility impact

- Source/runtime: adds no syntax or semantic operation; it freezes executable
  evidence for the already accepted Experimental Task profile.
- Scheduler configuration: caps the production local run-wide Task limit at
  exactly `1_000_000` and rejects larger values before execution.
- Diagnostics/schemas: reuses registered diagnostics and internal typed errors;
  adds no public schema, protocol, Semantic ID, trace, benchmark, or migration.
- Determinism/Unicode: strengthens projection tests; scheduling order remains
  nonsemantic and Unicode stays 17.0.0 with original UTF-8 byte spans.

## Unresolved alternatives

- User Resource finalizers, move/drop semantics, allocator quotas, valid
  capability-gated detach, source Clock/sleep, Task bytecode/VM/native execution,
  public replay/stress schemas, performance SLOs, work stealing, and Stable Task
  compatibility require later Accepted authority.
- Raising or removing the million-Task ceiling requires measured resource
  evidence and a new accepted decision; this decision makes no claim that every
  host can materialize the ceiling concurrently.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
