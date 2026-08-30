# TASK-2206 Authority Audit: Task Conformance and Stress Tests

## Outcome

`TASK-2206` is complete under Accepted DEC-0269. Its former pre-Task audit was obsolete. Accepted
DEC-0264 through DEC-0268 and completed TASK-2201 through TASK-2205 now provide
the checked Task frontend, machine, structured lifecycle runtime, deterministic
test scheduler, and production local scheduler. The only remaining semantic
gate is the final conformance/stress oracle accepted by DEC-0269.

DEC-0269 is now `Accepted` implementation authority. No TASK-2206 runtime,
scheduler, diagnostic, schema, or public API change was made before acceptance.

## Normative traceability

- DEC-0264 accepts the exact Task surface and Checked Task Core. It rejects
  `detach` from the source and checked runtime profile.
- DEC-0265 accepts the checked Task state machine and reason-preserving cleanup
  edges without defining scheduling.
- DEC-0266 accepts lexical parent/child ownership, mandatory join, monotonic
  cancellation, canonical Fault aggregation, exactly-once runtime-owned
  cleanup, and explicit limits.
- DEC-0267 accepts deterministic test-only scheduling, logical ticks,
  `TaskDeadline`, typed internal traces, strict replay, and bounded exploration.
- DEC-0268 accepts the exact file/project interpreter `task main ()` entry,
  fixed local worker pool, bounded queue, wake/park, cancellation, structured
  shutdown/join, internal snapshots, and nonsemantic metrics.
- `docs/ROADMAP-1.0.md` requires G2 concurrency conformance, stress, modeled
  interleavings, and no unclassified panic/deadlock. It assigns user
  `Resource`, ownership/drop, finalizers, and allocator semantics to G3.
- The G2 execution package is non-normative. Its user-`Resource` and valid
  `detach` examples cannot override the accepted first Task profile or pull G3
  semantics into TASK-2206.

## Plan drift resolved by DEC-0269

| G2 plan item | Current accepted meaning | DEC-0269 decision |
| --- | --- | --- |
| Parent exits early | Parent cancellation/Fault drains children; normal return cannot bypass checked handle observation and join | Compare terminal tree, committed Effects, and cleanup count one |
| Child cancellation releases Resource | Only runtime-owned handle/frame/scope-registry cleanup exists in G2 | Test accepted cleanup identities; do not fabricate a user finalizer |
| Two children Fault together | DEC-0266 canonical aggregate, independent of occurrence order | Drive opposite ready orders and production sibling Faults |
| Timeout races completion | DEC-0267 logical `TaskDeadline`; no wall clock | Test applied-before-terminal and unapplied-after-terminal cases |
| Nested scopes | Inner scopes drain before outer scopes | Exercise at least two lexical levels and descendants |
| Reject invalid detach | No accepted syntax, Capability, checked command, or runtime transition | Require registered frontend rejection before Checked Core publication |
| One million short Tasks | No recursive spawn or performance SLO is accepted | Make `1_000_000` the exact local Task-limit ceiling and reject larger values before execution |
| Shutdown loses no cleanup | DEC-0268 publishes success only after structured terminal state and worker join | Cover normal, cancellation, Fault, quota, and contained failure paths |

The one-million rule is a capacity boundary, not evidence that every host can
materialize one million simultaneous Tasks and not a throughput, latency, or
memory promise. Normal CI still needs a generated, bounded workload through the
real checked frontend/runtime/local scheduler.

## Existing executable evidence

- `crates/ling-effects` tests cover Task syntax, linear handles, scope
  ownership, Checked Task Core identities, and negative checked-publication
  boundaries.
- `crates/ling-eval/tests/task_runtime.rs` covers explicit ready driving,
  cancellation before start and suspension, committed host Effects, nested
  scopes, opposite-order sibling Fault aggregation, Fault/cancellation
  precedence, quotas, transitive Faults, cleanup counts, and invalid driver
  input.
- `crates/ling-eval/tests/task_scheduler.rs` covers repeated/reconstructed
  canonical traces, logical deadlines, equal-tick ordering, host failures,
  replay mismatch, bounded exploration, scheduler limits, and Unicode/BOM/CRLF
  source reconstruction.
- `crates/ling-eval/tests/task_local_scheduler.rs` covers one/multiple workers,
  queue and direct-child bounds, wake/park, host cancellation, worker/host panic
  containment, Fault source evidence, final snapshots, cleanup, and
  Unicode/BOM/CRLF execution.
- File/project CLI tests retain all non-interpreter Task rejections while
  allowing only the exact accepted interpreter Task entry.

## Implemented executable evidence

1. The production local-scheduler configuration ceiling accepts `1_000_000`
   and rejecting `1_000_001` before workers or host Effects.
2. Focused TASK-2206 integration evidence ties together early parent Fault,
   sibling Fault aggregation, nested-scope drain, shutdown snapshots, and
   worker-count differential outcomes.
3. A bounded generated short-Task stress case has explicit workload size,
   repetitions, worker counts, runtime/scheduler limits, no time threshold, and
   exactly-once cleanup assertions.
4. An explicit source attempt to use `detach` fails with the existing
   registered frontend diagnostic before Checked Task publication.
5. The implementation report maps every plan bullet to accepted clauses,
   executable tests, actual commands, compatibility, determinism, Unicode, and
   deferred G3/VM/public-protocol work.

## Accepted implementation authority

Accepted DEC-0269 freezes:

- the observable comparison projection;
- logical deadline and Fault-race precedence;
- the G2 runtime-owned cleanup interpretation and G3 user-Resource deferral;
- invalid-detach rejection scope;
- the exact million-Task configuration ceiling and representative stress rule;
- shutdown/panic containment evidence;
- test-corpus retention, Unicode/span, determinism, diagnostics, schema, and
  compatibility boundaries.

TASK-2206 may now implement only this bounded evidence. Any user Resource,
valid detach, wall-clock, Task VM/native, or public protocol behavior remains
outside the decision and requires separate Accepted authority.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0264 through DEC-0268,
Accepted DEC-0269, `GAP-STRUCTURED-TASK-001`, the G2 execution plan, and the
current Task frontend/runtime/scheduler/local-scheduler/CLI tests.

Acceptance changes documentation authority only. It does not change diagnostics, public
schemas, Semantic IDs, source spans, interpreter/VM behavior, scheduler order,
runtime limits, or Unicode 17.0.0 behavior.

## Intentionally deferred

User Resource finalizers and allocator quotas, valid capability-gated detach,
source Clock/sleep and wall-clock time, recursive spawn, Task bytecode/VM/native
execution, public trace/replay/stress/benchmark protocols, work stealing,
performance SLOs, and Stable Task compatibility remain separately governed.
