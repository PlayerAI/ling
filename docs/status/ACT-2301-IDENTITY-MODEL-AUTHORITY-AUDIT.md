# ACT-2301-IDENTITY-MODEL Authority Audit

## Outcome

The bounded child `ACT-2301-IDENTITY-MODEL` is authorized by Accepted
`DEC-0095`. It records immutable, publish-disabled Actor type, instance, and
reference identities for future evidence. Public `ACT-2301` remains
`BlockedSpec`: this child does not define Actor syntax, turns, state isolation,
message ownership, serialization, or runtime behavior.

## Normative traceability

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor identity, state,
  sendability, mailbox, supervision, and differential contracts before Actor
  execution.
- `docs/SEMANTICS.md` keeps Actor outside the v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as evidence.
- `DEC-0090` authorizes only negative Actor-shaped syntax evidence.
- `DEC-0095` authorizes only structural identity/reference validation.

## Current implementation boundary

`ActorIdentityModel` validates nonzero type, actor, and reference identities,
duplicate-free identity sets, known actor types, and known reference targets.
Local and Remote reference labels are structural observations only. Source
spans are retained as evidence and omitted from path-free canonical bytes.

No parser, AST/HIR/typed-program integration, turn checker, state-isolation or
borrow rule, Sendable judgment, mailbox, scheduler, serialization schema,
runtime, diagnostic, Semantic ID, CLI/LSP command, public protocol, or
migration behavior was added.

## Evidence and deferred work

Focused tests cover validation, deterministic ordering, canonical-byte
independence from source evidence and insertion order, and invalid/duplicate/
unknown identity rejection. The parent remains blocked until an Accepted Actor
authority defines identity lifetime/reuse, turn/state ownership, borrow and
Sendable rules, local/remote serialization, runtime ABI, and interpreter/VM
differential and migration evidence.
