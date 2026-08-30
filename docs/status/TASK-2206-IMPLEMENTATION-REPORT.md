# TASK-2206 Implementation Report: Task Conformance and Stress Tests

## Outcome

`TASK-2206` is complete under Accepted DEC-0269. Implementation commit
`d0c542d3b7caa8d8300ab2bb3021d35235773e91` adds the exact production-local
Task-count ceiling and executable conformance/stress evidence for the
Experimental Structured Task profile accepted by DEC-0264 through DEC-0268.

The implementation does not add Task syntax, user Resource finalizers, valid
detach, wall-clock behavior, Task bytecode/VM/native execution, public traces,
benchmark schemas, or performance guarantees.

## Normative clauses covered

- DEC-0269 clauses 1–2: every positive runtime case traverses source parsing,
  AST/HIR lowering, resolution, type checking, Checked Task Core/machine
  publication, and `TaskRuntime`; worker-count comparisons use only terminal,
  canonical Fault, Task-path, cleanup, and host-event projections.
- Clauses 3–5: early parent Fault drains its registered child, nested and
  cancellation suites retain exactly-once runtime-owned cleanup, and two
  independently faulting siblings publish one canonical two-Fault aggregate.
- Clause 6: the existing DEC-0267 deadline suite proves applied-before-terminal,
  unapplied-after-terminal, and equal-tick canonical ordering without wall time.
- Clauses 7–8: nested scopes/descendants finish before root publication, while
  a source attempt to reference `detach` fails as registered `L-NAME-0001`
  before Checked Task publication.
- Clause 9: `1_000_000` is accepted as the exact production-local run-wide
  Task-count ceiling; `1_000_001` and `usize::MAX` fail configuration validation
  before workers, Task allocation, or host Effects.
- Clauses 10–11: a generated 64-child checked program runs eight times with one
  worker and eight times with four workers; all 1,040 resulting Task instances
  reach terminal snapshots with cleanup count one and every worker exits.
- Clauses 12–13: existing deterministic/runtime bounds plus the new local
  worker differential, ceiling, direct-child, queue, transition, cancellation,
  Fault, panic, and shutdown cases cover the accepted failure classes.
- Clauses 14–16: executable evidence remains Rust integration tests with fixed
  constants and no timing threshold or public protocol; repository gates and
  the environment-specific shell-fixture result are recorded below.

## Plan-item evidence

| TASK-2206 plan item | Executable evidence | Result |
| --- | --- | --- |
| Parent exits early | `early_parent_fault_drains_every_registered_child_before_shutdown` plus DEC-0266 cancellation cases | Child is terminal, cleanup count is one, all workers exit |
| Child cancellation releases Resource | `host_cancellation_wakes_scheduler_and_drains_cleanup` and `cancellation_drains_children_and_cleans_each_task_once` | Accepted runtime-owned handles/frames/scope registries clean once; user Resource remains G3 |
| Two children Fault together | `independently_faulting_siblings_publish_one_canonical_aggregate` and `opposite_explicit_schedules_produce_the_same_canonical_fault_set` | Canonical two-Fault aggregate and identical cleanup |
| Timeout races completion | `deadline_at_zero_cancels_before_the_first_runtime_step`, `terminal_deadline_is_recorded_without_changing_the_completed_task`, and equal-tick test | Logical-time precedence only; no wall clock |
| Nested scopes | `nested_scopes_and_descendants_close_before_root_publication` and runtime scope-bound tests | Inner/descendant drain precedes root publication |
| Reject invalid detach | `detach_attempt_is_rejected_before_checked_task_publication` | Registered bilingual `L-NAME-0001`, original byte span, no Checked Core |
| One-million Task resource bound | `exact_million_task_ceiling_is_accepted_and_larger_limit_is_rejected_atomically` | Exact ceiling accepted; larger/maximum values rejected before Effect |
| Shutdown loses no cleanup | local normal/Fault/cancellation/stress/panic suites and terminal snapshot assertions | Every successful publication is terminal with cleanup one; workers join |

## Implementation

### Production-local Task ceiling

`crates/ling-eval/src/task_local_scheduler.rs` defines the private DEC-0269
capacity boundary `MAX_LOCAL_TASKS = 1_000_000`. `validate_config` rejects a
larger `TaskRuntimeLimits::max_tasks` value with the typed internal reason
`runtime_task_limit_exceeds_1000000` before direct-child preflight, runtime
construction, worker creation, allocation, or host Effect execution.

The constant is deliberately private. It does not create a CLI flag, public
protocol field, performance SLO, or claim that every host can materialize the
ceiling concurrently.

### Frontend rejection evidence

`crates/ling-effects/tests/structured_task_frontend.rs` parses an attempted
`detach handle` expression, lowers it to unresolved HIR, and proves resolution
fails with `ResolveErrorKind::UndefinedName`, registered code `L-NAME-0001`,
the logical source name, and the original UTF-8 byte start. Type checking and
Checked Task publication never occur.

### Production conformance and bounded stress

`crates/ling-eval/tests/task_local_scheduler.rs` adds:

- a shared terminal-tree assertion requiring every retained Task to be
  `Completed`, `Cancelled`, or `Faulted` with cleanup count one;
- early-parent-Fault and nested-scope/descendant shutdown cases with one and
  four workers;
- a two-sibling Fault case whose canonical Fault/snapshot projection is equal
  across worker counts;
- exact-ceiling, first-above-ceiling, and maximum-integer atomic validation;
- a generated 64-child checked source, run for 8 repetitions at each of 1 and
  4 workers with explicit queue, child, transition, park/wake, shutdown,
  runtime Task/scope/step/Fault limits and no duration threshold.

## Specification gaps and conflicts

The lower-authority G2 plan names user `Resource` release and valid/invalid
detach together with the first Task conformance slice. Accepted DEC-0266 and
DEC-0269 limit G2 cleanup to runtime-owned handle/frame/scope-registry state,
while `ROADMAP-1.0` assigns user Resource/drop/finalizer semantics to G3.
DEC-0264 and DEC-0269 keep detach unavailable and require frontend rejection.

The implementation follows the higher authority and does not fabricate either
feature. DEC-0264 through DEC-0269 resolve `GAP-STRUCTURED-TASK-001` for the
v0.2 profile by making detach unavailable. Future user Resource composition is
tracked by `GAP-OWNERSHIP-MODEL-001`; adding valid detach would require new
Accepted authority rather than silently reopening this implementation.

## Tests and gates executed

The following commands completed successfully on 2026-08-30:

```text
cargo fmt --all -- --check
cargo clippy -p ling-effects -p ling-eval --all-targets -- -D warnings
cargo test -p ling-effects --test structured_task_frontend
cargo test -p ling-eval --test task_local_scheduler
cargo test -p ling-eval --test task_runtime
cargo test -p ling-eval --test task_scheduler
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -- --skip installed_shells_accept_the_committed_fixtures
cargo xtask governance check-all
cargo xtask status verify
cargo xtask docs verify
```

The four targeted integration suites passed 57 tests. The workspace test gate
passed with only the independently known host-shell fixture filtered.

The following environment-sensitive fixture was also attempted unfiltered and
failed for a host-path reason unrelated to Ling semantics or TASK-2206:

```text
cargo test -p ling-cli --test completion installed_shells_accept_the_committed_fixtures -- --exact --nocapture
```

The installed WSL `bash` converted the Windows fixture path
`D:\Coding\Ling\...\ling.bash` to `D:CodingLing...ling.bash`, then reported
`No such file or directory`. The committed completion bytes themselves passed
`every_shell_matches_its_exact_utf8_lf_fixture`; no Task test, Clippy target, or
other workspace test failed.

## Compatibility impact

- Diagnostics: no allocation or payload change. Invalid detach uses existing
  bilingual `L-NAME-0001`; over-ceiling configuration remains an internal typed
  scheduler error and exposes no Rust debug data.
- Schemas/protocols: no public trace, stress, benchmark, replay, Task-tree,
  metrics, artifact, bytecode, ABI, Audit Source, or protocol version change.
- Semantic IDs: no source, Checked Task Core, machine, or canonical-byte change.
- CLI: no command, flag, output, exit, file/project entry, or rejection-route
  change. The existing CLI configuration is below the ceiling.
- Determinism: one/four-worker runs compare only the DEC-0269 projection;
  worker order and metrics remain nonsemantic. Stress constants are explicit
  and have no time threshold.
- Unicode: Unicode remains 17.0.0. Existing Chinese/BOM/CRLF cases remain green,
  and the detach negative test asserts original UTF-8 byte evidence.

## Intentionally deferred

- user Resource ownership, finalizers, allocator quotas, and observable drop;
- valid capability-gated detach and orphan/error-channel policy;
- source Clock/sleep, wall-clock deadlines, and I/O readiness;
- recursive/dynamic spawn and a workload that materializes one million Tasks;
- Task bytecode/VM/native/Wasm execution and interpreter/VM differential claims;
- public trace/replay/stress/benchmark schemas, performance SLOs, work stealing,
  and Stable Task compatibility.
