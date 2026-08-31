# ACT-2305 Authority Audit: Actor Runtime

## Outcome

`ACT-2305` is implemented under Accepted `DEC-0274`. `TASK-2201` through
`TASK-2206` and `ACT-2301` through `ACT-2304` are Done, so the runtime consumes
their immutable checked Task and Actor contracts without adding a source-level
Actor operation or backend ABI.

The implementation is a publish-disabled Experimental Rust embedding in
`ling-eval`. Public Actor-bearing `run`, `test`, `build`, REPL, bytecode, VM,
Native and Wasm routes remain outside this authority and continue to stop at
`L-ACTOR-0002`.

## Normative traceability

- The G2 execution package is non-normative. Accepted DEC-0274, not the plan,
  authorizes the internal runtime vocabulary and implementation boundary.
- Accepted DEC-0270 fixes run-scoped, nonzero, unique, nonreusable `ActorId`
  incarnations and checked state isolation; DEC-0271 fixes the closed
  `SendableLocal` Value set and message schema; DEC-0272 fixes checked capacity,
  Reject/Full and per-sender admission order; DEC-0273 fixes one-message,
  non-suspending, no-reentry and publish-on-return turns. DEC-0274 authorizes
  their first local executable composition.
- Accepted DEC-0266 through DEC-0269 provide the structured Task runtime,
  explicit driver, local scheduler and conformance boundary, but explicitly
  defer Actor crossing. They cannot silently define Actor ownership or
  scheduling.
- `docs/SEMANTICS.md` describes future Actor identity, private state, bounded
  mailbox, Sendable messages, one-turn processing, supervision, and
  `RemoteActorRef`; DEC-0274 closes only the internal local runtime slice.
  Source Actor operations remain non-executable.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require lifecycle/error
  observability, explicit bounded delivery, and resource cleanup, but do not
  define stable runtime commands, event payloads, scheduling guarantees,
  process boundaries, or compatibility/migration rules.
- `GAP-ACTOR-AWAIT-REENTRY-001` is resolved for the non-suspending v0.2 profile
  by DEC-0273. `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open for restart,
  escalation, budget and supervisor-visible Fault behavior; Accepted DEC-0274
  intentionally closes only the ACT-2305 runtime slice. The remote-delivery
  gap remains Open but does not block a strictly local non-serializable runtime.

## Current implementation evidence

- `crates/ling-eval/src/actor_runtime.rs` contains the single run-owned bounded
  registry, monotonic identities, typed local references and envelopes, FIFO
  mailboxes, explicit ready/step driver, atomic state publication, lifecycle,
  Fault containment and canonical stop/shutdown implementation.
- Runtime construction revalidates Checked Actor owner/type/schema/mailbox/
  turn/expression/binding/type/effect correspondence. Evaluation consumes only
  the checked initializer and transition expressions and reuses the existing
  closed `TaskValue` boundary.
- `LocalTaskControl` provides the accepted root ownership bridge. An unhandled
  Actor Fault requests root cancellation; root cancellation closes admission,
  drains queues, and cleans live Actors in Actor-ID order before runtime
  termination.
- `crates/ling-eval/tests/actor_runtime.rs` covers FIFO/order, typed rejection
  with payload return, resource failure atomicity, spawn/turn Faults, previous
  state, cancellation, cleanup, deterministic reconstruction and original
  Unicode/BOM/CRLF byte spans.
- `crates/ling-cli/tests/actor_boundary.rs` retains the public
  `L-ACTOR-0002` boundary. No Semantic Graph/runtime schema, public protocol,
  diagnostic, Actor bytecode or VM instruction was added.

## Accepted implementation boundary

Implementation remains inside DEC-0274's internal local profile. It does not
freeze later supervision, Replay, remote, public-source, parallel-scheduling or
backend semantics that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0099,
DEC-0266, DEC-0268, DEC-0270 through DEC-0273, Accepted DEC-0274, RFC-0001,
RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

Compatibility impact is restricted to the internal Experimental `ling-eval`
embedding. No source grammar, public CLI behavior, diagnostic allocation,
Semantic Graph/schema/protocol, Semantic ID, bytecode/VM/native/package ABI, or
Unicode 17.0.0 behavior changed.

## Intentionally deferred

Source-level Actor operations, public execution, ActorRef values,
suspending/reentrant turns, self-send,
parallel-dispatch claims, watchdog timing, graceful drain, Supervisor
restart/escalation, Replay, serialization, RemoteRef/transport, Resource
finalizers, bytecode/VM/native Actor ABI, public runtime protocols, and Stable
compatibility remain intentionally deferred under their own authority gates.
