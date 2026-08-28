# TASK-2203 Implementation Report

## Outcome

TASK-2203 is complete under Accepted DEC-0266. Implementation commit
`e8765790c421f3437049562d69c9aa6d487b5464` adds an internal Experimental,
publish-disabled `TaskRuntime` to `ling-eval`. The runtime consumes only a
successful `CheckedProgram`, a selected Task definition, validated argument
values, matching DEC-0264 Task Core and DEC-0265 Task-machine evidence, and
explicit non-zero bounds.

This milestone does not add a scheduler policy, public Task entry point,
bytecode/VM Task ABI, wall-clock timeout, detach, worker pool, Replay, Actor
crossing, or Stable protocol. Public file/project run, test, build, REPL,
artifact, bytecode 1.0 through 1.4, and VM routes continue to reject checked
Tasks with `L-TASK-0004`.

## Normative clauses covered

- DEC-0266 clauses 1–3: construction validates the exact checked Task
  definition/Core/machine pair, signature and structurally valid values before
  publishing the caller-selected root. Runtime Task identities are canonical
  lexical `Root/(spawn TaskId)*` paths, independent of allocation, wake, and
  driver order.
- Clauses 4–6: the evaluator is resumable only at checked scope, spawn, await,
  return, and host-Effect boundaries. Scope and spawn expression identities
  link runtime transitions back to Checked Core. Child registration publishes
  the argument environment, cleanup obligation, owning-scope handle, and child
  identity atomically before either parent or child can be selected again.
- Clause 5's amendment is implemented in the checker: unconsumed same-scope
  handles remain linear runtime-registry obligations across suspension and are
  deliberately absent from DEC-0265 value-frame slots. Cross-scope handles and
  every previously rejected mutable/aggregate/closure/Handler form remain
  rejected.
- Clauses 7–8: await consumes the exact scope-owned handle once, suspends
  without an implicit wake execution, resumes completed values through the
  checked continuation, and retains every Task until structured close. Normal
  close requires observed handles; cancellation and Fault drains consume only
  ownership obligations and never fabricate values or successful awaits.
- Clauses 9–11: cancellation is explicit, monotonic, and idempotent; it marks
  live descendants with `Ancestor`, preserves committed host Effects, drains
  children before owners, and enters each checked cleanup identity exactly
  once. Fault dominates cancellation, owner propagation becomes selectable at
  detection time, sibling Faults selected before propagation are retained, and
  final causes are ordered by canonical Task path with owner-first precedence.
- Clauses 10 and 13 reuse bilingual `L-RUNTIME-0001` for bounded-resource and
  canonical Task-Fault aggregate diagnostics. Runtime Task, nested-scope,
  lifecycle-step, and retained-Fault limits fail at their checked boundary and
  enter the same cleanup drain. Invalid driver selection is rejected before a
  task is removed or transitioned.
- Clauses 12 and 14: `ready()` returns the canonical sorted selectable set and
  `step(id)` executes only the caller-selected bounded segment. The kernel
  never chooses a Task, reads time, parks a thread, or assigns an interleaving
  policy. Results and traces remain internal and publish-disabled.

## Evidence

- `crates/ling-eval/src/task_runtime.rs` contains the checked construction
  boundary, runtime Task/scope registries, explicit ready-set driver, bounded
  lifecycle transitions, cancellation drain, exactly-once cleanup, and
  deterministic Fault aggregation.
- `crates/ling-eval/src/machine.rs` refactors the existing checked evaluator
  into a resumable CEK evaluation without changing ordinary Seed/Handler
  entry points. Direct and fused Task operations stop only at checked runtime
  boundaries; a completed host Console Effect is a separate step boundary.
- `crates/ling-eval/tests/task_runtime.rs` has 20 positive and negative tests.
  They cover explicit selection, spawn registration, suspension/wake, fused
  `let!`, multiple same-scope handles, cancellation before start/after spawn/
  while suspended/after committed output/during Fault drain, host failure,
  children-first cleanup, owner/child/sibling/transitive Faults, opposite
  schedule sequences, all four explicit bounds, invalid-driver atomicity,
  signature and nominal-value validation, Chinese identifiers, physical-path
  independence, and original BOM/CRLF UTF-8 Fault spans.
- Existing frontend tests continue to cover cross-scope use, duplicate
  observation, leaks, mutable suspension values, recursive spawn chains, and
  canonical Task Core/machine bytes. Existing CLI, project, bytecode, and VM
  tests retain the `L-TASK-0004` public execution boundary.

Executed gates against the implementation tree:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets --quiet`
- `cargo test -p ling-cli --test task_boundary`
- `cargo xtask governance check-all`
- the workspace run included all 174 `xtask` tests

## Specification gaps and conflicts

Accepted DEC-0266 resolves the prior DEC-0264 conflict for another live,
same-scope Task handle at suspension by retaining that handle only in the
runtime scope registry. Existing canonical Task Core and Task-machine bytes do
not encode the newly added expression lookup fields, so previously accepted
bytes remain unchanged.

No further language-semantic conflict was resolved in code.
`GAP-STRUCTURED-TASK-001` remains open for TASK-2204 through TASK-2206. The
Rust global allocator has no recoverable Ling-visible failure protocol; this
kernel preflights every authorized semantic count bound, while process-level
allocator abort remains outside the publishable Task result surface. A
recoverable memory budget or allocator adapter requires separate Accepted
resource authority.

## Compatibility, determinism, and Unicode impact

- Diagnostics: no code was allocated. `L-RUNTIME-0001` gains internal
  `resource_limit`, `task_driver`, and `task_fault_aggregate` facts;
  `L-TASK-0004` retains its existing public meaning.
- Schemas/protocols: no public schema, protocol inventory entry, artifact,
  Semantic Graph revision, Audit Source revision, bytecode revision, Replay
  format, CLI command, or LSP surface changed.
- Semantic IDs and checked bytes: expression identities used for runtime
  lookup are source evidence omitted from canonical Task Core and machine
  bytes. Existing Semantic IDs and bytes remain stable.
- Determinism: B-tree registries, lexical Task paths, explicit driver input,
  and canonical Fault ordering exclude allocation, insertion, wake, and
  occurrence order. Opposite sibling-Fault schedules reconstruct the same
  final cause set and cleanup counts.
- Unicode: original source spans remain UTF-8 byte offsets, BOM/CRLF evidence
  is preserved, Chinese identifiers execute through the checked boundary, and
  Unicode remains pinned to 17.0.0.

## Intentionally deferred

TASK-2204 retains deterministic scheduling, seeded choice, virtual time, and
trace export. TASK-2205 retains production worker/queue/wake behavior,
metrics, shutdown, and any public execution integration. TASK-2206 retains
stress, million-short-task, race, shutdown, and final conformance evidence.
Task bytecode/VM/native ABI, public Task roots, detach, logical Deadline
injection, user Resource finalizers, recoverable allocator integration,
Replay, Actor crossing, migration, and Stable compatibility remain separately
governed.
