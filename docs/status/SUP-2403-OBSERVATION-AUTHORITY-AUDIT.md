# SUP-2403-OBSERVATION Authority Audit

## Outcome

The bounded child `SUP-2403-OBSERVATION` is authorized by Accepted `DEC-0103`.
It records only the names of future supervision scenarios and structural
observation labels in an internal test corpus. Public `SUP-2403` remains
`BlockedSpec`: no Actor execution, restart policy, mailbox cleanup, state
restore, Fault outcome, or runtime fixture contract is defined.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted supervision, runtime,
  determinism/replay, and differential contracts before recovery behavior.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` leave Actor/Task execution outside
  the Seed subset and do not define supervision test outcomes.
- `DEC-0101` and `DEC-0102` authorize the observation identities and labels
  consumed by this corpus.
- `DEC-0103` authorizes this child only; the supervision gap remains open.

## Current implementation boundary

`supervision_evidence.rs` names seven planned scenarios and a vocabulary-only
case, builds existing `SupervisorObservationModel` values, and checks that
identity ordering produces deterministic canonical bytes. Scenario names and
labels are test metadata only; the tests do not execute an Actor, schedule a
restart, inspect a mailbox, restore state, or assert a Fault result.

No fixture schema, runtime harness, scheduler, restart/budget/circuit policy,
mailbox operation, state snapshot protocol, diagnostic, Semantic ID, CLI/LSP
command, public protocol, or migration behavior was added. Stale `zero`
command names remain absent from implementation surfaces.

## Evidence and deferred work

Focused tests cover all planned scenario names, deterministic forward/reverse
fixture ordering, and the complete Supervisor vocabulary. The parent remains
blocked until accepted authority defines executable fixtures, transitions,
outcomes, cleanup, replay, and interpreter/VM/runtime differential evidence.
