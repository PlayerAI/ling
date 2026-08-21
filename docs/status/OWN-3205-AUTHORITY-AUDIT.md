# OWN-3205 Authority Audit: Drop-order Lowering

## Outcome

`OWN-3205` is correctly recorded as `BlockedSpec`. The G3 plan asks the
compiler to insert implicit Drop operations into Cleanup Core with a declared
or RFC-defined order and to cover branches, early returns, `?`/Error, Faults,
cancellation, partial initialization, explicit rejection of panic/unwind as a
Ling mechanism, and explicit drop-failure rules. The required Resource, Drop,
ownership, failure, cancellation, and ABI semantics are not accepted.

No Cleanup Core, implicit Drop insertion, drop-order rule, partial-
initialization cleanup, cancellation cleanup, drop-failure mapping,
diagnostic, protocol, or placeholder G3 API was added.

## Normative traceability

- The G3 execution package is non-normative. Its cleanup checklist cannot
  authorize destruction order, implicit operations, failure aggregation,
  cancellation, or backend unwinding behavior.
- OWN-3205 depends on the missing RFC-N304/RFC-0007 ownership and Drop
  authority, plus the Resource model from MEM-3103. No accepted RFC-N304 or
  RFC-0007 exists; RFC-0001 remains a Draft under DEC-0018.
- `GAP-OWNERSHIP-MODEL-001` is Open and explicitly leaves Drop order,
  Resource/Managed boundaries, aliasing, and Profile behavior unaccepted. It
  blocks OWN-3205 and the memory/ownership subgroup.
- Accepted DEC-0009 defines only Seed Value and mutable-place behavior and
  excludes Resource/Borrow/`&mut`; DEC-0013/RFC-0018 define existing
  main/runtime Fault normalization but do not define Resource Drop or cleanup
  failure semantics.
- `docs/SEMANTICS.md` sketches deterministic Resource Drop, forbids silent
  arbitrary network work, requires explicit handling of potentially failing
  cleanup, bounds Drop Effect in Critical, and separates Managed cycle cleanup.
  It does not fix order, partial initialization, branch/early-return/
  cancellation behavior, failure aggregation, or Cleanup Core representation.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted Resource,
  Ownership/Region, Effect/Fault, Task/Actor cancellation, Native, and FFI
  rules before v0.3 implementation.

## Current implementation evidence

- The workspace has no Resource type or ownership checker, Cleanup Core,
  implicit Drop operation, cleanup CFG, partial-initialization state, drop
  order, cancellation/Actor termination cleanup, or Resource Drop Effect/Fault.
  Seed evaluator and VM execute Value semantics only.
- Existing Rust `Drop`, unwinding, panic containment, VM host cancellation,
  and CFG cleanup are implementation details and cannot define Ling cleanup
  timing or failures. No runtime path exposes a deterministic Resource Drop.
- No diagnostic or fixture defines branch/early-return/`?`/Error/Fault/
  cancellation cleanup, partial initialization, drop ordering, cleanup failure,
  panic/unwind rejection, Unicode/CRLF/BOM spans, or interpreter/VM/Native
  differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Resource identity, ownership, Move/Borrow/Region interactions, implicit
   versus explicit Drop, aggregate/branch/loop/closure/drop order, reverse
   declaration order versus another rule, and partial initialization/replace
   semantics.
2. Cleanup Core operations and lowering for normal return, early return,
   `?`/Error, Fault, cancellation, timeout, Task/Actor termination, process
   shutdown, and panic/unwind containment; define idempotence, partial cleanup,
   primary-versus-cleanup failure aggregation, Effects/Faults, and boundedness.
3. Managed-cycle separation, Capability/network restrictions, Native/FFI ABI
   transfer and target package rules, Profile/Critical constraints, migration,
   and optimization/determinism requirements without Rust unwinding leakage.
4. Stable bilingual diagnostics and error codes, canonical source spans,
   Semantic Graph/Audit Source, schema/protocol registration, deterministic
   output, and Unicode 17.0.0 handling for cleanup and failure paths.
5. Executable positive/negative/migration/property/drop-order/differential
   fixtures covering branches, loops, early exits, `?`, Error, Fault,
   cancellation, partial initialization, nested Resources, FFI, actor/task
   termination, and interpreter/VM/Native parity without unchecked-AST
   execution.

Until those decisions are Accepted, implementing Drop-order lowering would
freeze cleanup timing, failure behavior, source compatibility, safety, ABI,
and backend legality that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0009, DEC-0010, DEC-0012,
DEC-0013, DEC-0018, RFC-0001, RFC-0018,
`docs/ling_execution_plan/07-G3-V0.3-NATIVE.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/authority.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
Seed Place lowering, future Resource/Drop semantics, Cleanup Core, diagnostic,
schema, Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`OWN-3205` can begin only after MEM-3101 through MEM-3104, OWN-3201 through
OWN-3204, and RFC-0007/RFC-N304 (or accepted replacements) define memory kinds,
Copy/Move, Resource/Managed, Borrow/Region, Drop, suspension, cancellation,
Effects/Faults, and FFI boundaries. The future implementation must preserve
accepted Seed behavior, consume accepted types and checked Core only, make
cleanup deterministic and bounded, and publish drop-order, failure,
cancellation, Profile, FFI, and interpreter/VM/Native evidence before
exposing v0.3 cleanup behavior.
