# TASK-2204 Implementation Report

## Outcome

TASK-2204 is complete under Accepted DEC-0267. Implementation commit
`e550f4fd8d5f348bcb8415f5cbac23b9f852a719` adds a deterministic,
publish-disabled Task test driver to `ling-eval`. It drives only the Accepted
DEC-0266 checked `TaskRuntime`; it does not interpret AST or unchecked HIR.

This milestone does not add a production scheduler, worker pool, public Task
entry point, wall-clock or sleep API, I/O wake source, public trace file,
bytecode/VM Task ABI, Replay protocol, detach, Actor crossing, or Stable
compatibility promise. Existing CLI, project, artifact, bytecode, and VM Task
routes continue to reject execution with `L-TASK-0004`.

## Normative clauses covered

- DEC-0267 clauses 1–3 and 15: the driver is isolated in `ling-eval`, consumes
  a checked root recipe plus validated arguments, requires explicit non-zero
  runtime and scheduler bounds, canonicalizes deadline inputs, and selects
  only from the runtime's strictly ordered ready snapshot.
- Clauses 4–7: the exact SplitMix64 transition consumes one draw per selection;
  logical time starts at zero and advances once per successful runtime step;
  due deadlines are injected in `(tick, TaskPath)` order as DEC-0266
  `Deadline` cancellation before selection. No wall time, thread parking, or
  fabricated wake queue participates.
- Clauses 8–10: a bounded deterministic Console script records committed and
  failed host calls. A caught host panic becomes the existing Runtime Fault
  boundary and emits no guessed host event. Immutable typed traces validate
  limits, version, consecutive identities, monotonic ticks, canonical ready
  sets and deadlines, selected membership, one terminal closure, cleanup, and
  canonical Fault ordering before publication.
- Clause 10's canonical fixture bytes include configuration, checked-runtime
  identity, input deadlines, test-host script, and ordered events. They exclude
  source IDs, filesystem paths, source spans, Rust debug data, allocation and
  container order, and host timing; exact UTF-8 spans remain sidecar evidence.
- Clause 11: replay reconstructs a fresh checked runtime, verifies the runtime
  identity, consumes every recorded choice without seed fallback, and compares
  deadline, ready-set, tick, selection, step, host, terminal, cleanup, and Fault
  event bytes immediately. The first mismatch reports its event identity.
- Clauses 12–14: exploration is canonical breadth-first over explicit
  Task-path prefixes, reconstructs each run, rejects duplicate prefixes, and
  returns an explicit incomplete result for run, depth, ready-width, decision,
  tick, or trace exhaustion. Driver failures remain internal Rust errors and
  never become catchable Ling data or production scheduling semantics.

## Evidence

- `crates/ling-eval/src/task_scheduler.rs` implements explicit limits and
  configuration, SplitMix64 selection, logical ticks, deadline injection,
  deterministic host behavior, typed trace validation/canonical bytes, strict
  replay, and bounded breadth-first exploration.
- `crates/ling-eval/src/task_runtime.rs` adds only the narrow hooks required by
  the accepted driver: validated Task-path reconstruction, canonical retained
  Task-path observation, and internal cancellation-cause injection. Existing
  requested cancellation remains source-compatible.
- `crates/ling-eval/tests/task_scheduler.rs` has 14 integration tests covering
  repeated seeds, reconstructed checked inputs, Unicode/BOM/CRLF identity,
  span-sidecar exclusion, pre-start and terminal deadlines, equal-tick order,
  unknown children, host success/failure/panic, replay and identity mismatch,
  complete and incomplete exploration, and every explicit bound.
- Five module tests freeze SplitMix64 vectors and exercise malformed traces,
  deadline canonicalization, and first-event replay divergence across
  selection, tick, ready, step, deadline, host, and terminal mutations.
- The existing 20 DEC-0266 runtime integration tests retain cancellation,
  cleanup, owner/child/transitive Fault, resource, suspension, committed-host,
  Unicode-span, and opposite-explicit-schedule coverage. CLI Task boundary tests
  retain public `L-TASK-0004` rejection.

Executed gates against the implementation and completion-evidence tree:

- `cargo fmt --all -- --check`
- `cargo test -p ling-eval --all-targets --quiet`
- `cargo test -p ling-cli --test task_boundary --quiet`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets --quiet`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `git diff --check`

## Specification gaps and conflicts

Accepted DEC-0267 resolves the former TASK-2204 seed, virtual-time, deadline,
trace, replay, and exploration choices. Review found and corrected one draft
implementation mismatch: a test-host panic must publish no synthetic scheduler
event; it is now normalized only through the existing Runtime Fault boundary.
No language-semantic conflict was resolved in code.

`GAP-STRUCTURED-TASK-001` remains open for TASK-2205 and TASK-2206. The accepted
test driver deliberately cannot determine production worker ownership,
queue/wake/park behavior, fairness, shutdown, metrics, public execution,
resource finalizers, detach, stress limits, or final conformance rules.

## Compatibility, determinism, and Unicode impact

- Diagnostics: no code or meaning changed. Internal scheduler/replay failures
  are typed Rust errors; runtime Faults retain `L-RUNTIME-0001`, and public Task
  execution retains `L-TASK-0004`.
- Schemas/protocols: no public schema, protocol-inventory entry, artifact,
  Semantic Graph or Audit revision, CLI/LSP command, bytecode, VM, ABI, or file
  decoder was added. Typed trace bytes are internal fixtures only.
- Determinism: canonical Task paths, B-tree ordering, exact SplitMix64 vectors,
  explicit logical ticks, bounded reconstruction, and per-event replay exclude
  filesystem, allocation, hash-map, wall-clock, and host-timing behavior.
- Unicode: source IDs, names, and exact UTF-8 spans remain sidecar evidence;
  BOM/CRLF and Chinese text reconstruction is covered, and Unicode remains
  pinned to 17.0.0.

## Intentionally deferred

TASK-2205 retains the production local scheduler, workers, queues, wake/park,
fairness boundaries, quotas, shutdown, metrics, host integration, and any
public execution route. TASK-2206 retains stress, million-short-task, race,
shutdown, resource cleanup, and final Task conformance evidence. Public Replay,
Clock/sleep, I/O readiness, Task bytecode/VM/native ABI, detach, user Resource
finalizers, Actor crossing, migration, and Stable compatibility remain
separately governed.
