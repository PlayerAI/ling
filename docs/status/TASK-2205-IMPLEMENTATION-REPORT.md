# TASK-2205 Implementation Report

## Outcome

TASK-2205 is complete under Accepted DEC-0268. Implementation commit
`330e1b2c18ffbb7e59a07297abf59e92d6954fa5` adds the first
correctness-oriented production local Task scheduler and the exact checked
`task main ()` file/project interpreter entry. Evaluation still consumes only
Checked Task Core and DEC-0265 machines through the DEC-0266 `TaskRuntime`.

The CLI uses a fixed four-worker configuration that does not inspect host CPU
count. Task execution remains unavailable in `test`, `build`, REPL, project
artifacts, bytecode, VM, Native, Wasm, LSP, and editor routes; those boundaries
continue to use `L-TASK-0004` before execution or publication.

## Normative clauses covered

- DEC-0268 clauses 1–3: `locate_run_entry` distinguishes an ordinary
  `let main ()` from the exact checked `task main ()`. Only file/project
  interpreter `run` dispatches the Task entry; ordinary Task-containing value
  entries and every non-run route retain the previous rejection boundary.
- Clauses 4 and 11: `LocalTaskSchedulerConfig` validates non-zero worker,
  queue, runtime, direct-child, transition, park/wake, and shutdown bounds
  before starting a worker. Worker count is capped at 64, queue capacity cannot
  exceed the runtime Task limit, and every checked lexical scope is preflighted
  against the direct-spawn-site limit before a host Effect.
- Clauses 5–7: a scoped fixed worker pool shares one mutex/condition-variable
  coordinator. One worker leases the exact runtime and remaining central FIFO
  for one step; all other workers park while the queue is temporarily empty.
  The worker restores the lease, refreshes only the canonical `ready()` set,
  wakes peers, and never executes two runtime transitions concurrently.
- Clauses 8–10: the queue has unique canonical `TaskPath` membership. Host
  cancellation is observed only at scheduling boundaries and requests root
  cancellation through DEC-0266. Root termination wakes all workers; every
  worker exits and is joined before one result is published. Host/worker panic,
  mutex poison, membership corruption, lease collision, and join failure map
  to bounded typed scheduler errors without returning a Ling value or panic
  payload.
- Clauses 12–14: final in-process snapshots contain only epoch, canonical Task
  path, runtime state, cleanup count, and root state. Monotonic metrics cannot
  select work or alter runtime results. Different worker counts may change
  acquisition and Effect order, but tests require identical terminal class,
  Task tree, and exactly-once cleanup for Effect-independent programs.
- Clauses 15–16: runtime and scheduler bounds reuse `L-RUNTIME-0001`; no
  diagnostic allocation, public schema, protocol inventory entry, Semantic ID,
  artifact, bytecode, VM ABI, Replay format, or Unicode-version change was
  introduced. Help and bilingual README text state the exact supported and
  rejected execution surfaces.

## Evidence

- `crates/ling-eval/src/task_local_scheduler.rs` implements explicit validated
  configuration, fixed scoped workers, bounded FIFO/set membership,
  mutex/condition-variable park/wake, cancellation, shutdown/join, panic and
  poison containment, final snapshots, and nonsemantic metrics.
- `crates/ling-eval/src/lib.rs`, `machine.rs`, and `task_scheduler.rs` replace
  interpreter-only `Rc`/`RefCell` cells with `Arc`/`Mutex` and atomic
  continuation counters. This ownership change makes a checked runtime safely
  movable between scoped workers without changing serialized interpreter
  evaluation.
- `crates/ling-effects/src/lib.rs` validates exact ordinary/Task run entries,
  including module name, explicit Unit pattern, checked parameter, and Unit
  result. Unit tests cover value entry, Task entry, invalid pattern, and invalid
  result cases.
- `crates/ling-eval/tests/task_local_scheduler.rs` has eight integration tests
  covering one/four-worker repeated runs, real park/wake, canonical final
  snapshots, exactly-once cleanup, invalid pools, lexical child preflight,
  queue/transition exhaustion, host-boundary cancellation, host panic,
  structured host Fault, and Chinese/BOM/CRLF source evidence.
- CLI integration tests cover successful file and project Task main execution,
  unchanged ordinary-main Task rejection, retained file/project `test` and
  project `build` rejection, no artifact publication, and truthful help text.

Executed gates against the implementation and completion-evidence tree:

- `cargo fmt --all -- --check`
- `cargo test -p ling-eval --all-targets --quiet`
- `cargo test -p ling-effects --all-targets --quiet`
- `cargo test -p ling-cli --test task_boundary --test project_commands --test help --quiet`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets --quiet -- --skip installed_shells_accept_the_committed_fixtures`
- `cargo xtask docs verify`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `git diff --check`

The unskipped full workspace invocation reached the existing
`installed_shells_accept_the_committed_fixtures` test and failed because WSL
removed separators from the Windows fixture path (`D:CodingLing...`). Its two
platform-independent completion tests passed, and the rerun skipped only that
environment-dependent installed-shell probe.

## Specification gaps and conflicts

No language-semantic conflict was resolved in code. Accepted DEC-0268 supplied
all TASK-2205 observable choices. The implementation deliberately serializes
runtime transitions; the fixed pool is a correctness and lifecycle boundary,
not a parallel-speedup claim.

The original interpreter used non-`Send` ownership internally. The smallest
safe implementation replaced shared evaluator cells and handler-continuation
state with standard-library synchronized equivalents. No allocation, lock
acquisition, worker identity, or poisoning detail is exposed as Ling behavior.

`GAP-STRUCTURED-TASK-001` remains open only for TASK-2206 resource/detach,
stress, million-short-task, race, and final conformance authority.

## Compatibility, determinism, and Unicode impact

- CLI: exact checked `task main ()` now succeeds only in file/project
  interpreter `run`. Existing ordinary Seed/Handler runs and all retained Task
  rejection surfaces are covered by regression tests.
- Rust API: the Experimental `Console` capability now requires `Send`, and the
  local scheduler exposes only in-process Rust configuration, control, result,
  snapshot, metric, and error types. No stable public wire protocol exists.
- Diagnostics/schemas: existing `L-RUNTIME-0001` and `L-TASK-0004` allocations
  are reused. No schema, Semantic Graph, Audit Source, artifact, bytecode, VM,
  ABI, or Replay version changed.
- Determinism: scheduler acquisition and admitted cross-Task Effect order are
  intentionally nonsemantic. Canonical Task identities, retained Fault order,
  cleanup multiplicity, final snapshots, type safety, and memory safety remain
  invariant across tested worker counts and repeated reconstruction.
- Unicode: original byte spans and source names survive worker execution;
  Chinese identifiers/text with UTF-8 BOM and CRLF are covered. Unicode remains
  pinned to 17.0.0.

## Intentionally deferred

TASK-2206 retains high-volume stress, million-short-task performance, extended
race/shutdown matrices, public conformance closure, detach, user Resource
finalizers and quotas, recoverable allocation budgets, and Stable scheduling
evidence. Work stealing, priorities, affinity, public Task-tree/metrics APIs,
Clock/sleep, I/O readiness, Task `test`/`build`/REPL/artifact execution,
bytecode/VM/native Task ABI, Replay, Actor crossing, and migration remain later
Accepted work.
