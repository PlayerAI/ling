# MEM-3102 Authority Audit: Value Layout and Copy/Move

## Outcome

`MEM-3102` is correctly recorded as `BlockedSpec`. The G3 plan asks for
implementation details such as small-value inlining, unobservable register/
stack choices, Ling-defined Copy rather than Rust `Copy`, restricted implicit
copies, move/copy performance guidance, and explicit ABI/serialization rules
for overflow, padding, and endianness. These choices require the unaccepted
memory-kind, ownership, ABI, and serialization authorities.

No Value layout, Copy/Move trait, implicit-copy rule, move checker, ABI field,
padding/endianness behavior, performance diagnostic, protocol, or placeholder
G3 API was added.

## Normative traceability

- The G3 execution package is non-normative. Its optimization and layout
  notes cannot authorize observable representation, Copy/Move semantics, ABI,
  serialization, or public diagnostics.
- MEM-3102 depends on the missing RFC-N301 and the memory/ownership authority.
  No RFC-N301 or accepted replacement exists; RFC-0001 remains a Draft under
  DEC-0018, and `GAP-OWNERSHIP-MODEL-001` remains Open with RFC-0007 as the
  candidate authority.
- Accepted DEC-0008 and DEC-0009 define Seed value semantics: records copy as
  independent Values, parameters use value semantics, Resource/Borrow/`&mut`
  and implicit aliasing are not implemented. They intentionally do not define
  future Copy/Move traits, layouts, ABI, or optimization boundaries.
- `docs/SEMANTICS.md` says Copy/Move and Resource rules are future memory
  behavior and that Rust ownership, allocation, layout, padding, and hash
  order must not become Ling semantics. It does not fix inline thresholds,
  representation, copy legality, or serialization bytes.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted ownership,
  memory, Native ABI, and FFI rules before v0.3 implementation and require
  optimization to preserve Typed Core behavior.
- `GAP-OWNERSHIP-MODEL-001` is Open, blocks MEM-3101 through MEM-3104 and
  related ownership tasks, and explicitly leaves Copy/Move, aliasing, drop,
  and Profile boundaries unaccepted.

## Current implementation evidence

- The workspace has no future Copy/Move trait, ownership checker, layout
  contract, inline/stack representation policy, padding/endianness schema,
  Native ABI, or Value layout projection. Seed aggregates and VM values are
  interpreted according to accepted value semantics without exposing storage.
- Existing Rust `Copy`, allocation, enum layout, register choice, stack choice,
  overflow behavior, and hash order are implementation details and are not
  evidence for Ling Copy/Move or ABI semantics.
- No diagnostic or fixture defines illegal copy/move, use-after-move,
  representation/serialization mismatch, padding/endianness, overflow at an
  ABI boundary, Unicode/CRLF/BOM source-span preservation, or interpreter/VM/
  Native differential behavior. No versioned layout or ABI protocol is
  registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Copy/Move/Clone legality for every Value form and generic kind, including
   implicit-copy conditions, explicit operations, aliasing/identity, closure
   capture, recursive/aggregate values, and interaction with Traits and
   separate compilation.
2. Representation and layout obligations independent of optimization:
   size/alignment, overflow, padding, endianness, discriminants, niche rules,
   pointer/reference identity, deterministic equality/hash, and what remains
   unobservable to Ling programs.
3. Typed Core and Semantic Graph/Audit Source projection, canonical bytes,
   serialization, Semantic IDs, profile constraints, compatibility/migration,
   and versioned Native/FFI ABI boundaries.
4. Move/use-after-move diagnostics, resource cleanup and failure behavior,
   cancellation/Actor-turn/await boundaries, compiler optimization proof
   obligations, and prohibition on Rust ownership/layout leakage.
5. Stable bilingual diagnostics, schema/protocol registration, Unicode
   17.0.0 source-span handling, deterministic output, and explicit performance
   guidance that cannot change semantics.
6. Executable positive/negative/migration/property/differential fixtures for
   Copy/Move, generic constraints, closures, aggregates, serialization,
   overflow/padding/endianness at ABI boundaries, optimization equivalence,
   and interpreter/VM/Native parity.

Until those decisions are Accepted, implementing Value layout or Copy/Move
would freeze source compatibility, safety, ABI, serialization, and backend
legality that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0008, DEC-0009, DEC-0012,
DEC-0013, DEC-0018, RFC-0001,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
memory kind, Value layout, Copy/Move rule, ownership, Native ABI, diagnostic,
schema, Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior
changed.

## Intentionally deferred

`MEM-3102` can begin only after MEM-3101 and RFC-0007 (or an accepted
replacement) define memory kinds, ownership, Copy/Move, and ABI/serialization
boundaries. The future implementation must preserve Seed Value behavior,
consume accepted types and checked Core only, keep optimization unobservable,
and publish copy/move, representation, serialization, profile, and
interpreter/VM/Native differential evidence before exposing v0.3 layout
behavior.
