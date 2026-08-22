# TASK-2202-STATE-MACHINE-MODEL Authority Audit

## Outcome

The bounded child `TASK-2202-STATE-MACHINE-MODEL` is authorized by Accepted
`DEC-0092`. It validates a publish-disabled structural graph for future Task
state-machine lowering. Public `TASK-2202` remains `BlockedSpec`: no actual
lowering, continuation ABI, bytecode, verifier, runtime, or scheduler behavior
is defined by this child.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted suspension, cleanup,
  cancellation, deterministic-scheduler, and differential contracts before
  Structured Task support is promoted.
- `docs/SEMANTICS.md` §18 keeps Task outside the v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as evidence.
- `DEC-0091` provides the preceding internal Task identity boundary.
- `DEC-0092` authorizes only state/local/transition identity validation and
  path-free canonical bytes.

## Current implementation boundary

`StateMachineModel` records opaque state, continuation, live-local, and
transition identities. It validates a nonzero task and entry, unique states
and transitions, duplicate-free live locals, known endpoints, and unique
`(from, to, kind)` edges. `Resume`, `Cancel`, `Cleanup`, and `Fault` are labels
only; they do not execute or authorize those operations. Source spans are
retained as evidence and omitted from canonical bytes.

No parser, AST/HIR/typed-program integration, lowering pass, live-local type or
borrow rule, bytecode opcode, verifier, VM/native ABI, interpreter behavior,
runtime, scheduler, diagnostic, schema, Semantic ID, CLI/LSP command, public
protocol, or migration behavior was added.

## Evidence and deferred work

Focused tests cover deterministic ordering and canonical bytes, all structural
edge labels, duplicate/unknown/invalid graph cases, and source-span
independence. The parent remains blocked until an Accepted Task authority
defines continuation/frame layout, liveness and ownership across suspension,
edge semantics, bytecode/versioning, verifier limits, source maps, and
interpreter/VM differential and migration evidence.
