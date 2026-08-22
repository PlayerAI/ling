# TASK-2204-SCHEDULER-OBSERVATION Authority Audit

## Outcome

The bounded child `TASK-2204-SCHEDULER-OBSERVATION` is authorized by Accepted
`DEC-0094`. It records immutable, publish-disabled scheduler observations for
future evidence. Public `TASK-2204` remains `BlockedSpec`: this child does not
define a queue, virtual clock, seed interpretation, wake order, interleaving,
replay, or production scheduler.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires deterministic-scheduler and differential
  evidence before Task execution is promoted.
- `docs/SEMANTICS.md` §18 keeps Task outside the v0.0.1 Seed subset.
- `DEC-0021` authorizes deterministic internal compiler-query scheduling only;
  it does not authorize a Task scheduler.
- `DEC-0093` provides the preceding internal lifecycle-observation boundary.
- `DEC-0094` authorizes only structural scheduler observations and deterministic
  identity validation.

## Current implementation boundary

`SchedulerObservationTrace` validates nonzero trace, event, scope, and task
identities and rejects duplicate event identities. Observations are stored by
opaque event identity; labels remain evidence rather than queue or state
transitions. Source spans are retained as evidence and omitted from path-free
canonical bytes.

No queue, worker, virtual clock, seed-to-order algorithm, wake API,
interleaving explorer, replay protocol, parser, AST/HIR/typed-program
integration, bytecode/VM ABI, diagnostic, schema, Semantic ID, CLI/LSP command,
public protocol, or migration behavior was added.

## Evidence and deferred work

Focused tests cover validation, deterministic ordering, canonical-byte
independence from insertion order and source evidence, and duplicate/invalid
identity rejection. The parent remains blocked until an Accepted Task
authority defines seed mapping, ready/wake tie-breaks, virtual-clock rules,
fairness, exploration/replay equivalence, resource limits, scheduler ABI, and
interpreter/VM differential and migration evidence.
