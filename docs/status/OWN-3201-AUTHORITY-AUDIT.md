# OWN-3201 Authority Audit: Place and Move Analysis

## Outcome

`OWN-3201` is correctly recorded as `BlockedSpec`. The G3 plan proposes a
single Typed Core representation for local/field/index places, projections,
move, copy, borrow, and mutable borrow, with dataflow for initialized, moved,
partially moved, branch joins, loop fixed points, match destructuring, closure
captures, task/actor boundaries, and first-move diagnostics. The accepted Seed
place/mutation slice does not authorize this future ownership calculus.

No future move/borrow dataflow, place form, ownership state, diagnostic,
lowering, protocol, or placeholder G3 API was added. Accepted Seed mutable
place behavior remains unchanged.

## Normative traceability

- The G3 execution package is non-normative. Its dataflow sketch cannot
  authorize ownership/borrow syntax, Typed Core nodes, diagnostics, closure or
  concurrency boundaries, or backend behavior.
- OWN-3201 depends on the missing RFC-N302/RFC-0007 ownership authority. No
  accepted RFC-N302 or RFC-0007 exists; RFC-0001 remains a Draft under DEC-0018
  and `GAP-OWNERSHIP-MODEL-001` remains Open.
- Accepted RFC-0017 and DEC-0009 authorize only Seed mutable local/record-field
  Place lowering, value copies, and rejection of parameters, temporaries,
  Borrow types, and alias-visible mutation. They do not authorize future
  move/borrow states, partial moves, closure capture, or Task/Actor transfer.
- `docs/SEMANTICS.md` describes future Place/Move/Borrow/Region concepts and
  states that full Borrow/Move parameter patterns require a later RFC. It does
  not fix a unified ownership lattice, dataflow joins/fixed points,
  destructuring/capture rules, public lifetime projection, or suspension
  boundaries.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted Ownership,
  Borrow, Region, Drop, memory-kind, Actor/Task, Native, and FFI contracts
  before v0.3 implementation.
- `GAP-OWNERSHIP-MODEL-001` is Open and blocks OWN-3201, OWN-3202, OWN-3203,
  and OWN-3205; `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` separately blocks public
  lifetime/region decisions for later ownership tasks.

## Current implementation evidence

- The workspace implements the accepted Seed mutable-place checker/lowering,
  not a general ownership analysis. It has no move/borrow/borrow_mut Typed
  Core forms, partial-move state, loop fixed-point ownership solver, closure
  capture ownership, Task/Actor transfer analysis, or future diagnostic
  contract.
- Existing place CFG joins, record-copy behavior, VM update operations, and
  Rust borrow checking are not evidence for Ling move/borrow semantics. Rust
  ownership and allocation remain non-semantic.
- No diagnostic or fixture defines first/second move, partial move, invalid
  projection, branch/loop join, destructuring, closure capture, suspension or
  Actor-turn escape, Unicode/CRLF/BOM span preservation, or
  interpreter/VM/Native differential behavior for the future model.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Typed Core nodes and judgments for local/field/index places, projections,
   move/copy/borrow/borrow_mut, initialization, partial moves, reinitialization,
   destructuring, and closure capture, including aggregate and generic types.
2. Dataflow and fixed-point rules for branches, loops, match joins, nested
   projections, exceptions/Errors/Faults, cancellation, Task/Actor turns,
   suspension/await, and Resource/Managed boundaries; specify soundness and
   termination expectations.
3. Public lifetime/region inference and projection, cross-package and
   separate-compilation compatibility, Trait interaction, FFI/Native/ABI
   transfer, pinning, and migration from the accepted Seed Place slice.
4. Stable bilingual diagnostics and error codes identifying the first move,
   illegal use, partial move, alias conflict, escape, or boundary violation,
   with canonical source spans, Semantic Graph/Audit Source, deterministic
   output, and Unicode 17.0.0 handling.
5. Executable positive/negative/migration/property/fuzz/differential fixtures
   covering branches, loops, match, closures, aggregates, generic values,
   Error/Fault/cancellation, Task/Actor boundaries, FFI, and
   interpreter/VM/Native parity without unchecked-AST execution.

Until those decisions are Accepted, implementing general Place/Move analysis
would freeze type compatibility, diagnostics, safety, lifetime, concurrency,
ABI, and backend legality that the language authority intentionally leaves
open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0009, DEC-0012, DEC-0013,
DEC-0018, RFC-0001, RFC-0017,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, future ownership analysis, move/borrow semantics,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`OWN-3201` can begin only after MEM-3101 through MEM-3104 and RFC-0007 (or an
accepted replacement) define memory kinds, Copy/Move, Resource/Managed,
Borrow/Region, Drop, suspension, and FFI boundaries. The future implementation
must preserve accepted Seed Place behavior, consume accepted types and checked
Core only, avoid Rust ownership leakage, and publish dataflow,
diagnostic, lifetime, boundary, and interpreter/VM/Native evidence before
exposing v0.3 ownership behavior.
