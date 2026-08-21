# OWN-3202 Authority Audit: Borrow Exclusivity

## Outcome

`OWN-3202` is correctly recorded as `BlockedSpec`. The G3 plan sketches the
usual rule of any number of immutable borrows or one mutable borrow, then
explicitly defers compatibility details to an RFC. It also requires place
overlap, field splitting, conservative index aliasing, call-site automatic
borrowing, temporary lifetimes, mutable-place requirements, and iterator
mutation conflicts. None of those future rules is authorized by the accepted
Seed boundary.

No borrow type, exclusivity checker, overlap/alias analysis, automatic borrow,
temporary lifetime rule, diagnostic, protocol, or placeholder G3 API was added.

## Normative traceability

- The G3 execution package is non-normative. Its exclusivity sketch cannot
  authorize borrow syntax, alias compatibility, lifetime inference, or
  diagnostics.
- OWN-3202 depends on the missing RFC-N302/RFC-0007 ownership authority. No
  accepted RFC-N302 or RFC-0007 exists; RFC-0001 remains a Draft under DEC-0018
  and `GAP-OWNERSHIP-MODEL-001` remains Open.
- Accepted DEC-0009 explicitly excludes Resource, Borrow, `&mut`, implicit
  references, and Borrow Edges from v0.0.1 while authorizing only local
  mutable-place writes and Value copies. It cannot be extended by
  implementation into future borrow compatibility.
- `docs/SEMANTICS.md` gives a future immutable/mutable borrow sketch, overlap
  rule, lifetime bound, Pin/Region requirement across suspension, Actor-turn
  restriction, and non-aliasing requirement for Kernel slices. It does not fix
  place-overlap algebra, field/index splitting, automatic reborrows, temporary
  lifetime, iterator mutation, public lifetimes, or diagnostics.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted Borrow,
  Ownership, Region, Drop, memory-kind, concurrency, Native, and FFI contracts
  before v0.3 implementation.
- `GAP-OWNERSHIP-MODEL-001` is Open and blocks OWN-3202; its required evidence
  includes negative, property, drop-order, differential, and Profile cases.

## Current implementation evidence

- The workspace has no Borrow/`&mut` type, exclusivity relation, place-overlap
  solver, field-splitting or index-alias policy, automatic borrow insertion,
  temporary-lifetime analysis, iterator mutation checker, or future
  diagnostics. The accepted Seed checker rejects implicit alias-visible
  mutation instead.
- Existing Rust borrow checking, CFG/Place lowering, record-copy semantics,
  and VM update operations are implementation mechanisms, not Ling borrow
  semantics. No public lifetime or ABI is exposed.
- No diagnostic or fixture defines overlapping immutable/mutable borrows,
  index alias conservatism, field disjointness, iterator invalidation,
  temporary escape, call-site reborrow, suspension/Actor boundary, Unicode/
  CRLF/BOM span preservation, or interpreter/VM/Native differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Borrow and mutable-borrow types/judgments, alias identity, place-overlap
   algebra, field splitting, index/dynamic projection policy, pattern and
   destructuring behavior, and interaction with Copy/Move, Resource, Managed,
   and Traits.
2. Automatic borrow/reborrow insertion (if any), call-site and coercion rules,
   temporary lifetime/extension, iterator/container mutation behavior, and
   rejection versus conservative approximation for uncertain aliases.
3. Lifetime/Region constraints through functions, closures, branches, loops,
   returns, public APIs, Task/Actor turns, `await`/suspension, pinning, FFI,
   Native ABI, and Profile boundaries; specify migration and separate
   compilation compatibility.
4. Checked Core representation/lowering, stable bilingual diagnostics and
   error codes for overlap, escape, invalid mutable place, and use-after-borrow,
   canonical source spans, Semantic Graph/Audit Source, deterministic output,
   and Unicode 17.0.0 handling.
5. Executable positive/negative/migration/property/fuzz/differential fixtures
   for disjoint fields, dynamic indices, iterators, temporaries, reborrows,
   closures, aliases, cancellation, suspension/Actor boundaries, FFI, and
   interpreter/VM/Native parity without unchecked-AST execution.

Until those decisions are Accepted, implementing borrow exclusivity would
freeze safety, source compatibility, diagnostics, lifetimes, concurrency,
ABI, and backend legality that the language authority intentionally leaves
open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0009, DEC-0012, DEC-0013,
DEC-0018, RFC-0001,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, future borrow/exclusivity semantics, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`OWN-3202` can begin only after MEM-3101 through MEM-3104, OWN-3201, and
RFC-0007 (or an accepted replacement) define memory kinds, Copy/Move,
Resource/Managed, Place, Borrow, Region, Drop, suspension, and FFI boundaries.
The future implementation must preserve accepted Seed Place behavior, consume
accepted types and checked Core only, avoid Rust aliasing leakage, and publish
overlap, lifetime, iterator, boundary, diagnostic, and interpreter/VM/Native
evidence before exposing v0.3 borrowing behavior.
