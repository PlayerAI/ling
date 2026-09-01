# SUP-2402 Implementation Report

## Outcome

SUP-2402 is complete under Accepted DEC-0277. Implementation commit
`be9b8dd842815cff100267a645b9bef88b13c369` adds one crate-private,
publish-disabled `RestartOneBudgeted` policy to the real DEC-0276 local
Supervisor and DEC-0274 checked-Core Actor runtime.

The slice is deliberately internal and Experimental. It does not make Actor or
Supervisor execution available through Ling source, the CLI, the interpreter,
bytecode, the VM, Native, Wasm, LSP, or an editor. Every public Actor-bearing
execution route continues to stop at `L-ACTOR-0002`.

## Normative clauses covered

- DEC-0277 clauses 1-3: the implementation consumes successful immutable
  `CheckedProgram` Actor definitions, retains existing `ContainOne`, and adds
  exactly one opt-in private `RestartOneBudgeted` policy over a fixed,
  duplicate-free child set.
- Clauses 4-6: construction validates an immutable nonzero
  `(max_restarts, window_ticks, backoff_ticks)` configuration, preflights one
  full history window against runtime bounds, and uses an explicit checked
  `u64` logical clock with exact half-open per-slot attempt windows.
- Clauses 7-11: turn Fault acknowledgement records complete bounded state
  before suppression, fixed backoff delays replacement, exhausted budgets open
  a circuit, expiry permits one HalfOpen probe, and both successful publication
  and initializer Fault consume an attempt.
- Clauses 9 and 12: each successful replacement receives a fresh monotonically
  allocated Actor ID, empty mailbox, and initializer-only state. Old state,
  messages, and references are never restored or transferred. Each slot retains
  only the accepted canonical last-Fault projection.
- Clauses 13-16: serialized coordinator boundaries process due slots in
  canonical `ActorTypeId` order; invalid, overflowing, or resource-unrecordable
  transitions use root fallback; stop/cancellation cancels pending recovery;
  snapshots remain private deterministic test evidence.
- Clauses 17-18: public protocols, diagnostics, schemas, Semantic IDs, CLI/LSP,
  bytecode/VM/backend behavior, Replay, and Stable compatibility are unchanged.

## Implementation

`crates/ling-eval/src/actor_runtime.rs` now has a private restart spawn path.
Initializer Faults on that path retire the reserved Actor ID, retain canonical
Fault/event accounting, and return control to the Supervisor without publishing
an Actor or failing the root solely for that expected recovery outcome. The
ordinary spawn path retains its previous terminal behavior. Aggregate recovery
preflight checks created/live Actor, command, Fault, shutdown-work, and event
capacity before a multi-slot boundary mutates restart state.

`crates/ling-eval/src/actor_supervisor.rs` now owns the private restart budget,
per-slot attempt history, logical tick, Backoff/Restarting/CircuitOpen lifecycle,
Closed/Open/HalfOpen circuit state, deadlines, and last-Fault evidence. Due
slots are selected by canonical Actor type order and replacement is completed
inside the serialized `advance_to` boundary, so no caller can observe a
partially published incarnation.

The implementation keeps policy code in the existing private Supervisor and
runtime modules. It adds no placeholder crate, public facade, serializer,
backend adapter, or speculative strategy abstraction.

## Executable evidence

Eight focused Supervisor tests cover:

- fixed backoff, fresh Actor identity, initializer state, empty mailbox, closed
  old reference, and unchanged sibling state;
- exact half-open attempt-window expiry, circuit opening, one HalfOpen probe,
  success closure, and retained active attempts;
- ordinary and HalfOpen initializer Faults, attempt consumption, reopen
  deadlines, no same-boundary retry, and later recovery;
- zero configuration, construction history bounds, clock regression, tick
  overflow, and aggregate runtime resource fallback;
- simultaneous due slots in canonical `ActorTypeId` order;
- owner cancellation during pending recovery with no new Actor or duplicate
  cleanup; and
- Unicode identifiers/text reconstructed from BOM/CRLF sources with identical
  deterministic projection and original UTF-8 Fault spans.

Commands executed on 2026-09-01:

- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 53 unit,
  12 Actor runtime, 13 local scheduler, 20 Task runtime, and 14 Task scheduler
  tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings` —
  passed.
- `cargo test -p ling-cli --test actor_boundary --locked --offline` — passed:
  10 tests, including the unchanged public `L-ACTOR-0002` execution boundary.
- `cargo test --workspace --all-targets --locked --offline` — passed.
- `cargo fmt --all -- --check` and `git diff --check` — passed.

## Compatibility impact

- Diagnostics, schemas, Semantic IDs, protocols, package/ABI versions, stored
  data, and dependencies: unchanged.
- Public source and tooling: unchanged; no restart command, value, Effect,
  Capability, query, metric, or serialized trace exists.
- Determinism: recovery uses only explicit logical ticks, checked arithmetic,
  canonical Actor identities, and serialized coordinator order. It exposes no
  wall time, thread order, addresses, allocation, paths, or Rust debug text.
- Unicode: remains 17.0.0. Original UTF-8 byte spans remain authoritative.

## Specification gaps and deferred work

No conflict was found inside DEC-0277's scoped private contract. The broader
`GAP-ACTOR-MAILBOX-SUPERVISOR-001` and
`GAP-DETERMINISTIC-REPLAY-001` remain Open.

SUP-2403 executable supervision evidence, Wait/drop/coalescing policies, state
snapshot/restore, mailbox transfer, stable references, dynamic or nested trees,
group restart, escalation, parallel recovery, public Fault/budget queries,
metrics, Replay, remote/backend execution, fairness, liveness, performance,
migration, and Stable compatibility remain intentionally deferred pending their
own Accepted authority and evidence.
