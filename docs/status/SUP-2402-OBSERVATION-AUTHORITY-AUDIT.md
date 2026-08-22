# SUP-2402-OBSERVATION Authority Audit

## Outcome

The bounded child `SUP-2402-OBSERVATION` is authorized by Accepted `DEC-0102`.
It records immutable, publish-disabled restart-budget and circuit observation
identities, optional opaque Actor instances, and structural labels for future
evidence. Public `SUP-2402` remains `BlockedSpec`: this child does not define
clocks, counters, windows, backoff, circuit transitions, Fault provenance,
queries, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor supervision,
  determinism/replay, runtime, and differential contracts before restart
  control.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` prohibit unlimited rapid restart
  but do not define budget units, clocks, transitions, or query protocols.
- `DEC-0095` through `DEC-0101` provide the preceding opaque Actor, message,
  mailbox, turn, runtime, property, and Supervisor observation boundaries.
- `DEC-0102` authorizes this child only and leaves the supervision/replay gaps
  open.

## Current implementation boundary

`BudgetObservationModel` validates nonzero observation identities, optional
nonzero Actor identities, and duplicate-free observations, then stores them in
deterministic identity order. `RestartCount`, `Window`, `Backoff`,
`MaxRestarts`, `FaultProvenance`, `CircuitClosed`, `CircuitOpen`, and
`CircuitHalfOpen` are labels only. Source spans are retained as evidence and
omitted from path-free canonical bytes.

No counter, time/logical clock, backoff scheduler, circuit state machine, Fault
store, runtime query, administration protocol, diagnostic, Semantic ID,
CLI/LSP command, public protocol, or migration behavior was added. Stale
`zero` command names remain absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover identity validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, duplicate rejection, and
invalid Actor rejection. The parent remains blocked until Accepted supervision
and replay authorities define clocks, budgets, recovery transitions, provenance,
queries, and executable stress/differential evidence.
