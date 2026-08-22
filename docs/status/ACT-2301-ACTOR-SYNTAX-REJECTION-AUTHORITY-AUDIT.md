# ACT-2301 Actor Syntax Rejection Authority Audit

## Outcome

`ACT-2301-ACTOR-SYNTAX-REJECTION` is a bounded negative-evidence child
authorized by Accepted `DEC-0090`. It proves only that an Actor-shaped
top-level declaration cannot reach the checked compiler pipeline under the
current Seed profile. The public `ACT-2301` target remains `BlockedSpec`.

## Normative basis

- `docs/LANGUAGE.md` §19 excludes Actor from v0.0.1 Seed.
- `docs/SEMANTICS.md` §19 reserves Actor behavior for the concurrent design
  gate and does not fix the complete identity/turn/message contract.
- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor identity, turn,
  mailbox, supervision, and remote-delivery authority before implementation.
- `DEC-0001` and `DEC-0002` fix the existing diagnostic registry and original
  UTF-8 byte-span units.
- `DEC-0090` limits this child to the existing parser/CLI rejection boundary.

## Evidence boundary

The fixture invokes `ling_cli::compile_source` with a source-shaped `actor`
declaration and checks `L-SYNTAX-0010`, bilingual JSON fields, and the exact
original byte span. It proves that no `Compiled` value or checked
`ProgramSnapshot` is returned.

It does not reserve an Actor keyword, add AST/HIR/Typed Core nodes, or define
identity, turns, state isolation, message ownership, mailbox, supervision,
remote delivery, scheduling, Fault, Effect, Capability, bytecode, VM, LSP,
schema, Semantic ID, or migration behavior.

## Intentionally deferred

Actor grammar, identity scope/reuse, turn/reentry, state isolation,
Sendability, mailbox/backpressure, supervision, remote delivery, runtime
lowering, and positive/differential fixtures remain blocked by the registered
Actor gaps and the missing Accepted Actor RFC.
