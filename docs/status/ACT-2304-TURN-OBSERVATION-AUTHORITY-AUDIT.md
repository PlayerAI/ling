# ACT-2304-TURN-OBSERVATION Authority Audit

## Outcome

The bounded child `ACT-2304-TURN-OBSERVATION` is authorized by Accepted
`DEC-0098`. It records immutable, publish-disabled turn identities, optional
opaque Actor owners, and structural turn vocabulary labels for future evidence.
Public `ACT-2304` remains `BlockedSpec`: this child does not define await,
reentry, state guards, self-send, watchdog, scheduler, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor turn, await/reentry,
  mailbox, supervision, and differential contracts before execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` keep Actor/Task execution outside
  the v0.0.1 Seed subset while naming future turn constraints.
- `DEC-0095`, `DEC-0096`, and `DEC-0097` provide the preceding opaque Actor,
  message-schema, and mailbox identity boundaries.
- `DEC-0098` authorizes this child only and leaves
  `GAP-ACTOR-AWAIT-REENTRY-001` open.

## Current implementation boundary

`TurnObservationModel` validates nonzero turn identities, optional nonzero
Actor-type owners, and duplicate-free observations, then stores them in
deterministic identity order. `NoAwait`, `FreezeAndRelease`, `ForbidReentry`,
`GuardedReentry`, `SelfSend`, and `Watchdog` are labels only. Source spans are
retained as evidence and omitted from path-free canonical bytes.

No await form, turn lifecycle, state-version token, reentry guard, self-send
operation, mailbox route, watchdog limit/event, scheduler hook,
cancellation/Fault transition, serializer, runtime, diagnostic, Semantic ID,
CLI/LSP command, public protocol, or migration behavior was added.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid owner rejection. The parent remains blocked until an Accepted Actor
authority defines turn and await semantics, state/borrow guards, self-send and
watchdog behavior, cancellation/supervision, and executable differential/
interleaving evidence.
