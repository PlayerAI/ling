# ACT-2306-PROPERTY-OBSERVATION Authority Audit

## Outcome

The bounded child `ACT-2306-PROPERTY-OBSERVATION` is authorized by Accepted
`DEC-0100`. It records immutable, publish-disabled property observation
identities, optional opaque Actor instances, and structural property labels for
future evidence. Public `ACT-2306` is now `Done`: Accepted DEC-0274 supplies
its local runtime and Accepted DEC-0275 authorizes its completed bounded
internal property/stress evidence. This child still does not define property results,
stress thresholds, scheduling, replay, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor runtime, determinism,
  replay, and differential contracts before property evidence can certify
  execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset while naming future property constraints.
- `DEC-0095` through `DEC-0100` provide the preceding opaque Actor, message,
  mailbox, turn, runtime, and property observation boundaries.
- Accepted DEC-0274 provides the checked-Core-only local runtime that this
  observation child intentionally does not execute. Accepted DEC-0275 defines
  the parent task's bounded property/stress test contract.
- `DEC-0100` authorizes this child only and leaves the Actor/replay gaps open.

## Current implementation boundary

`PropertyObservationModel` validates nonzero observation identities, optional
nonzero Actor identities, and duplicate-free observations, then stores them in
deterministic identity order. `SerialState`, `ParallelActors`,
`BoundedMailbox`, `SlowConsumer`, `PostStopSend`, `FaultCleanup`,
`DeclaredOrdering`, and `ShutdownCleanup` are labels only. Source spans are
retained as evidence and omitted from path-free canonical bytes.

No property runner, stress harness, scheduler, replay format, fixture schema,
threshold, runtime, diagnostic, Semantic ID, CLI/LSP command, public protocol,
or migration behavior was added.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid Actor rejection. The parent is Done under Accepted DEC-0275 with
property relations, interleavings, resource/shutdown budgets, and executable
stress evidence in ling-eval. Replay remains a separate unresolved authority
rather than a prerequisite for this internal local-runtime decision.
