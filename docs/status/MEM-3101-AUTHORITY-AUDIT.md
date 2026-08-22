# MEM-3101 Authority Audit: Type Classification Model

## Outcome

`MEM-3101` is correctly recorded as `BlockedSpec`. The G3 plan requires every
Checked Core type to carry a memory kind (`Value`, `Managed`, or `Resource`)
and asks for default/derived kinds, generic kind constraints, composition,
copy/move, equality/hash/serialization effects, and Semantic Graph/Audit
Source projections. This is the foundation of the v0.3 ownership and native
ABI model, but RFC-N301 is absent and no accepted memory-model RFC authorizes
those semantics.

The bounded child `MEM-3101-SEED-VALUE`, authorized by DEC-0061, records only
the existing Seed completed-type `Value` classification and does not unblock
the future memory model.

No memory-kind type, kind constraint, copy/move rule, layout/identity
classification, graph field, diagnostic, protocol, or placeholder G3 API was
added.

## Normative traceability

- The G3 execution package is non-normative. Its `Value`/`Managed`/`Resource`
  sketch cannot authorize type rules, ABI/layout, ownership, serialization,
  or public Checked Core fields.
- MEM-3101 explicitly depends on RFC-N301, but no RFC-N301 (or accepted
  replacement) exists in the repository. RFC-0001 remains a Draft baseline
  under DEC-0018, and `GAP-OWNERSHIP-MODEL-001` remains Open with RFC-0007 as
  the candidate authority.
- Accepted DEC-0008 and DEC-0009 deliberately constrain the v0.0.1 Seed to
  value semantics and reject Resource, Borrow, `&mut`, implicit aliasing, and
  Borrow edges. They do not define future Managed/Resource identity, generic
  kind algebra, native layout, or cross-profile compatibility.
- `docs/SEMANTICS.md` describes future `Value`, `Managed`, and `Resource`
  categories and says Resource has identity, affine movement, Borrow, and
  deterministic release, while excluding the full model from the Seed. It
  does not fix classification inference, composition, public kind
  constraints, equality/hash/serialization, or graph encoding.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require an accepted ownership,
  region, memory, Resource, Managed, Native, and FFI boundary before v0.3
  implementation. Existing accepted Seed RFCs cover only Seed values,
  aggregates, mutable places, Effects, and VM behavior.
- `GAP-OWNERSHIP-MODEL-001` is Open, blocks MEM-3101 through MEM-3104 and
  related ownership tasks, and states that memory categories, Copy/Move,
  borrowing, aliasing, region escape, drop order, Managed roots/finalization,
  and Profile boundaries are not accepted.

## Current implementation evidence

- The workspace has no Managed or Resource type, memory-kind lattice, generic
  kind constraint, ownership/region checker, layout contract, Resource drop
  operation, native backend, or kind projection in Checked Core, Semantic
  Graph, or Audit Source. The Seed checker and VM use value semantics only.
- `Type::seed_type_class` exposes `SeedTypeClass::Value` for completed Seed
  forms and returns no class for unresolved variables or error sentinels; it
  adds no Managed/Resource or ownership information.
- Existing primitive, tuple, record, variant, array, closure, Effect, and
  Capability handling is authorized Seed behavior, not evidence for future
  memory classification. Rust ownership, allocation, enum layout, and hash
  order are intentionally non-semantic.
- No diagnostic or fixture defines kind mismatch, illegal composition,
  copy/move violation, identity/equality change, serialization restriction,
  Unicode/CRLF/BOM span behavior at a memory boundary, or Interpreter/VM/
  Native differential result. No versioned public memory protocol is
  registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. The three kinds, their identity/aliasing/lifetime invariants, default and
   derived classification for primitives, records, tuples, ADTs, arrays,
   closures, functions, Effects, and capabilities, plus an explicit rule for
   opaque or foreign types.
2. Generic kind constraints and composition, inference and public signature
   projection, variance/trait interaction, separate-compilation compatibility,
   and the profile-specific subset exposed to Explore, Native, and Critical.
3. Copy/Move/Clone and equality/hash/serialization rules, including how
   canonical bytes, Semantic IDs, Audit Source, and graph identity respond to
   Managed or Resource identity and mutable state.
4. Checked Core representation and lowering, ownership/region/drop and
   suspension/Actor boundaries, resource cleanup on return, Error, Fault,
   cancellation, and termination, plus ABI/layout/FFI constraints without
   exposing Rust implementation details.
5. Stable bilingual diagnostics, error-code allocation, schema and protocol
   versioning, migration/compatibility behavior, determinism, and explicit
   Unicode 17.0.0 source-span preservation.
6. Executable positive/negative/migration/property/drop-order/profile and
   interpreter/VM/Native differential fixtures covering all type categories,
   generic constraints, kind composition, serialization, equality/hash,
   cancellation, failure, and illegal aliasing.

Until those decisions are Accepted, implementing memory classification would
freeze source compatibility, safety, ABI/layout, serialization, and backend
legality that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0008, DEC-0009, DEC-0010,
DEC-0012, DEC-0013, DEC-0018, RFC-0001,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
memory kind, ownership, Managed, Resource, Native ABI, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`MEM-3101-SEED-VALUE` is complete under DEC-0061. The parent `MEM-3101` can
begin only after RFC-0007 (or an accepted replacement) defines
the memory-kind model and after the ownership, region, drop, Managed, Native,
and FFI dependencies agree on its Checked Core and public projections. The
future implementation must preserve Seed Value semantics, consume accepted
types and checked Core only, avoid Rust-layout leakage, and publish kind,
copy/move, identity, profile, and interpreter/VM/Native evidence before
exposing v0.3 memory behavior.
