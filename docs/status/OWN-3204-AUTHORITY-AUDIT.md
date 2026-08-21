# OWN-3204 Authority Audit: Borrow Across `await` and Actor Turns

## Outcome

`OWN-3204` is correctly recorded as `BlockedSpec`. The G3 plan requires
rejecting borrows of stack/turn-local state across suspension by default,
allowing only explicitly lowered pinned/owned state-machine fields, forbidding
Actor-state borrows after a turn ends, rejecting ordinary borrows in remote
messages, and providing actionable diagnostics. These rules depend on both the
unaccepted ownership/region model and the unresolved Actor await/reentry
semantics.

No cross-`await` borrow checker, pinned state-machine field, Actor-turn lifetime
rule, message borrow gate, diagnostic, protocol, or placeholder G3 API was
added.

## Normative traceability

- The G3 execution package is non-normative. Its high-risk checks cannot
  authorize suspension, pinning, Actor reentry, message sendability, or
  lifetime behavior.
- OWN-3204 depends on RFC-N303/RFC-N302 and RFC-0007 for regions/borrowing, and
  on the Actor authority RFC-C203/RFC-0009. None is present or Accepted;
  RFC-0001 remains a Draft under DEC-0018.
- `GAP-ACTOR-AWAIT-REENTRY-001` is Open and says it is undecided whether an
  Actor may process another message while a turn is suspended and which state
  invariants span suspension. It blocks the Actor turn/reentry/runtime work.
- `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` and `GAP-OWNERSHIP-MODEL-001` remain Open,
  leaving public lifetime projection, Borrow/Region, aliasing, Drop, and
  Profile boundaries unaccepted. Accepted DEC-0009 only defines Seed value
  semantics and rejects Borrow/`&mut` in v0.0.1.
- `docs/SEMANTICS.md` states that `await` is a suspension point, Actor-state
  mutable Borrow cannot cross it without proof and RFC permission, ordinary
  Borrow cannot outlive an Actor turn, and remote messages cannot carry normal
  Borrow. These are design constraints, not a complete checked Core/runtime
  contract.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted Task/Actor
  suspension, ownership/region, Drop, message, replay, Native, and FFI rules
  before v0.3 implementation.

## Current implementation evidence

- The workspace has no `await`/suspension Core, Task/Actor runtime, pinned
  state-machine lowering, cross-turn lifetime checker, remote message borrow
  gate, or cleanup/cancellation integration. The Seed evaluator and VM are
  synchronous and have no Actor/Task/Remote behavior.
- Existing VM host cancellation and incremental query scheduling are unrelated
  to Ling suspension or Actor turns. Rust pinning/lifetimes and stack frames
  are implementation details and not language semantics.
- No diagnostic or fixture defines stack/turn-local escape, pinned-field
  eligibility, Actor reentry, message borrow rejection, cancellation/drop
  interactions, stale state, Unicode/CRLF/BOM spans, or
  interpreter/VM/Native differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Suspension and `await` semantics: state-machine fields, pinning/ownership,
   borrow lifetime and reborrow rules, stack/turn-local escape, cancellation,
   timeout, Drop, Fault, and partial initialization behavior.
2. Actor turn and reentry model: whether another message may run while a turn
   is suspended, state invariants across suspension, mailbox ordering,
   supervision/restart, Actor-state access, and cross-Actor/Task/remote message
   transfer restrictions.
3. Region/public lifetime, Borrow/Move, Resource/Managed, sendability,
   serialization/schema identity, FFI/Native/ABI, Capability/security, and
   migration/separate-compilation rules, including pinned/owned alternatives.
4. Checked Core/state-machine lowering, stable bilingual diagnostics and
   error-code allocation with actionable fixes, canonical source spans,
   Semantic Graph/Audit Source, deterministic output, and Unicode 17.0.0
   handling without exposing Rust pinning or allocation.
5. Executable positive/negative/migration/interleaving/state-invariant/replay/
   property/fuzz/differential fixtures for suspension, cancellation, Actor
   reentry, pinned fields, message boundaries, FFI, and interpreter/VM/Native
   parity without unchecked-AST execution.

Until those decisions are Accepted, implementing cross-`await` or Actor-turn
borrows would freeze lifetime safety, state invariants, ordering, cancellation,
diagnostics, ABI, and backend legality that the language authority intentionally
leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0009, DEC-0010, DEC-0012,
DEC-0013, DEC-0018, RFC-0001,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, future suspension/borrow semantics, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`OWN-3204` can begin only after MEM-3101 through MEM-3104, OWN-3201 through
OWN-3203, the Actor/Task work, and RFC-0007/RFC-0009 (or accepted replacements)
define memory kinds, Copy/Move, Borrow/Region, Drop, suspension, Actor
reentry, message sendability, and FFI boundaries. The future implementation
must preserve accepted Seed behavior, consume accepted types and checked Core
only, make pinned/owned state explicit, and publish suspension, cancellation,
interleaving, state-invariant, message, diagnostic, and interpreter/VM/Native
evidence before exposing cross-`await` borrowing.
