# MEM-3103 Authority Audit: Resource Definition and Drop Contract

## Outcome

`MEM-3103` is correctly recorded as `BlockedSpec`. The G3 plan requires a
Resource with unique ownership, use-after-move rejection, explicit or derived
Drop, Effect/Fault rules, cancellation-safe cleanup, an FFI transfer mode, and
no substitution of GC finalization for deterministic cleanup. These semantics
are not authorized until the ownership, memory-kind, Effect/Failure, Managed,
and FFI contracts are accepted.

No Resource type, ownership token, Drop operation, cleanup lowering, Drop
Effect/Fault, cancellation hook, FFI transfer mode, diagnostic, protocol, or
placeholder G3 API was added.

## Normative traceability

- The G3 execution package is non-normative. Its Resource checklist cannot
  authorize affine ownership, destruction timing, failure effects, FFI ABI, or
  runtime cleanup behavior.
- MEM-3103 depends on MEM-3101/MEM-3102 and the missing RFC-N301/RFC-0007
  authority. No accepted memory/ownership RFC exists; RFC-0001 remains a Draft
  under DEC-0018 and `GAP-OWNERSHIP-MODEL-001` remains Open.
- Accepted DEC-0009 explicitly says v0.0.1 Seed does not implement Resource,
  Borrow, `&mut`, implicit reference passing, or Borrow Edges. It defines only
  local Value mutation and independent record-copy behavior, not future Drop,
  cancellation, FFI transfer, or Resource identity.
- `docs/SEMANTICS.md` sketches deterministic Resource Drop, forbids silent
  arbitrary network work, requires explicit handling for cleanup that can fail,
  bounds Drop Effects in Critical, and separates Managed cycle cleanup. It does
  not fix ownership transfer, drop order, partial cleanup, failure aggregation,
  cancellation, or ABI/FFI transfer semantics; v0.0.1 has no Resource Core.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted ownership,
  memory, Effect/Fault, resource, Native, and FFI rules before v0.3 behavior.
- `GAP-OWNERSHIP-MODEL-001` is Open, blocks MEM-3101 through MEM-3104 and
  related ownership work, and leaves Copy/Move, aliasing, drop order, Managed
  roots/finalization, and Profile boundaries unaccepted.

## Current implementation evidence

- The workspace has no Resource type, unique-owner identity, move/use-after-
  move checker, Drop lowering, cleanup stack, cancellation-safe destructor,
  resource Effect/Fault, FFI transfer mode, or Managed finalizer boundary. The
  Seed checker and VM use Value semantics only.
- Existing host handles are Seed Capability injections, not Ling Resources;
  Rust `Drop`, allocation, panic, thread unwinding, or finalizer behavior is
  not a language contract. No runtime path exposes cleanup timing or failure as
  Ling semantics.
- No diagnostic or fixture defines double move, use-after-move, drop order,
  partial cleanup, cleanup failure, cancellation/Actor termination, FFI
  transfer, GC-vs-deterministic release, Unicode/CRLF/BOM spans, or
  interpreter/VM/Native differential behavior. No versioned Resource/Drop
  protocol is registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Resource identity and unique ownership, Move/borrow/use-after-move rules,
   ownership transfer and return semantics, derived versus explicit Drop, drop
   order for aggregates/branches/loops/closures, and interaction with Generic
   kinds, Traits, Actors, and suspension.
2. Cleanup behavior on normal return, Error, Fault, cancellation, timeout,
   task/Actor termination, panic containment, partial failure, and process
   shutdown; specify whether cleanup is infallible, an explicit Effect, or a
   typed Fault and how failures aggregate without hiding the primary failure.
3. Checked Core operations and lowering, deterministic and bounded cleanup,
   capability/network restrictions, Managed-cycle separation, profile rules,
   and proof obligations that prevent Rust unwinding/allocation behavior from
   becoming Ling semantics.
4. Typed FFI transfer modes, ABI ownership/lifetime/thread/reentrancy rules,
   pinning, error/fault mapping, target-package trust, and compatibility/
   migration for Resource APIs and serialized identities.
5. Stable bilingual diagnostics, error-code and schema/protocol registration,
   canonical Semantic IDs/Audit Source, deterministic output, and Unicode
   17.0.0 source-span preservation.
6. Executable positive/negative/migration/property/drop-order/differential
   fixtures covering moves, aliases, branches, loops, closures, cancellation,
   Faults, Actor termination, FFI transfer, cleanup failure, resource limits,
   and interpreter/VM/Native parity.

Until those decisions are Accepted, implementing Resource or Drop would freeze
source compatibility, safety, cleanup timing, failure behavior, ABI, and
backend legality that the language authority intentionally leaves open.

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
memory kind, Value layout, Resource, Drop, ownership, Native ABI, diagnostic,
schema, Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`MEM-3103` can begin only after MEM-3101/MEM-3102 and RFC-0007 (or an accepted
replacement) define memory kinds, Copy/Move, ownership, Drop, Effects/Faults,
and FFI boundaries. The future implementation must preserve Seed Value
semantics, consume accepted types and checked Core only, make cleanup
deterministic and bounded, and publish drop-order, cancellation, failure,
profile, FFI, and interpreter/VM/Native evidence before exposing Resource
behavior.
