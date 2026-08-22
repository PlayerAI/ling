# ACT-2305-RUNTIME-OBSERVATION Authority Audit

## Outcome

The bounded child `ACT-2305-RUNTIME-OBSERVATION` is authorized by Accepted
`DEC-0099`. It records immutable, publish-disabled runtime observation
identities, optional opaque Actor instances, and structural lifecycle labels for
future evidence. Public `ACT-2305` remains `BlockedSpec`: this child does not
define spawn, stop, dispatch, lifecycle, Fault, registry, scheduler, ABI, or
runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Task, Actor, mailbox,
  supervision, runtime, and differential contracts before execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset while naming future runtime constraints.
- `DEC-0095` through `DEC-0098` provide the preceding opaque Actor, message,
  mailbox, and turn observation boundaries.
- `DEC-0099` authorizes this child only and leaves the Actor/Task gaps open.

## Current implementation boundary

`RuntimeObservationModel` validates nonzero observation identities, optional
nonzero Actor identities, and duplicate-free observations, then stores them in
deterministic identity order. `Spawn`, `Start`, `Dispatch`, `Suspend`, `Stop`,
`Stopped`, `Failed`, and `Restart` are labels only. Source spans are retained
as evidence and omitted from path-free canonical bytes.

No runtime crate, spawn/stop/dispatch operation, typed envelope, mailbox
storage, lifecycle state machine, registry, scheduler hook, Task integration,
Fault schema, serializer, runtime, diagnostic, Semantic ID, CLI/LSP command,
public protocol, or migration behavior was added.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid Actor rejection. The parent remains blocked until Accepted Actor/Task
authorities define runtime ownership, lifecycle/dispatch ABI, Fault provenance,
registry/shutdown behavior, scheduling, and executable differential/stress
evidence.
