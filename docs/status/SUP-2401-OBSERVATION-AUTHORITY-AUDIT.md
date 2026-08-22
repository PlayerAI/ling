# SUP-2401-OBSERVATION Authority Audit

## Outcome

The bounded child `SUP-2401-OBSERVATION` is authorized by Accepted
`DEC-0101`. It records immutable, publish-disabled Supervisor observation
identities, optional opaque Actor instances, and structural supervision labels
for future evidence. Public `SUP-2401` remains `BlockedSpec`: this child does
not define child ownership, restart, stop, escalation, strategy, budgets, state
restore, Fault channels, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor mailbox, runtime,
  supervision, determinism, and differential contracts before recovery behavior.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset while naming future supervision concepts.
- `DEC-0095` through `DEC-0100` provide the preceding opaque Actor, message,
  mailbox, turn, runtime, and property observation boundaries.
- `DEC-0101` authorizes this child only and leaves the supervision gap open.

## Current implementation boundary

`SupervisorObservationModel` validates nonzero observation identities, optional
nonzero Actor identities, and duplicate-free observations, then stores them in
deterministic identity order. `ChildSpec`, `Restart`, `Stop`, `Escalate`,
`OneForOne`, `RestForOne`, `Transient`, `Permanent`, `Temporary`, `StateRestore`,
and `FaultChannel` are labels only. Source spans are retained as evidence and
omitted from path-free canonical bytes.

No Supervisor type, child registry, restart/stop/escalate operation, strategy
state machine, restart budget, state snapshot/restore, Fault channel,
diagnostic, Semantic ID, CLI/LSP command, public protocol, or migration
behavior was added.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid Actor rejection. The parent remains blocked until Accepted Actor and
supervision authorities define recovery semantics, budgets, state restore,
Fault provenance, shutdown, and executable stress/differential evidence.
