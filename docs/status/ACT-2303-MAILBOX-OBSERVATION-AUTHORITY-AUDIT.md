# ACT-2303-MAILBOX-OBSERVATION Authority Audit

## Outcome

The bounded child `ACT-2303-MAILBOX-OBSERVATION` is authorized by Accepted
`DEC-0097`. It records immutable, publish-disabled mailbox identities,
optional opaque Actor owners, and structural policy labels for future evidence.
Public `ACT-2303` remains `BlockedSpec`: this child does not define capacity,
queue, send, backpressure, ordering, supervision, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor mailbox, ordering,
  supervision, and differential contracts before executable delivery.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset while naming future bounded-mailbox constraints.
- `DEC-0010` defines Seed State and Capability behavior only; it does not
  authorize Actor mailbox ownership or send semantics.
- `DEC-0095` and `DEC-0096` provide the preceding opaque Actor and message
  schema identity boundaries.
- `DEC-0097` authorizes this child only and leaves
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001` open.

## Current implementation boundary

`MailboxObservationModel` validates nonzero mailbox identities, optional
nonzero Actor-type owners, and duplicate-free observations, then stores them in
deterministic identity order. `Wait`, `Reject`, `DropNewest`, `DropOldest`, and
`Coalesce` are labels only. Source spans are retained as evidence and omitted
from path-free canonical bytes.

No capacity value, queue storage, enqueue/dequeue operation, send result,
suspension point, Backpressure Effect, overflow algorithm, coalescing key,
ordering/fairness rule, close/termination rule, supervision transition,
serializer, runtime, diagnostic, Semantic ID, CLI/LSP command, public protocol,
or migration behavior was added.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid owner rejection. The parent remains blocked until an Accepted Actor
authority defines capacity, queue ownership, send/backpressure outcomes,
ordering, shutdown, supervision, and executable differential/stress evidence.
