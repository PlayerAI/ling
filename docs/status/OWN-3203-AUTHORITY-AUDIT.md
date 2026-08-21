# OWN-3203 Authority Audit: Region Inference

## Outcome

`OWN-3203` is correctly recorded as `BlockedSpec`. The G3 plan proposes
lexical and non-lexical regions, returned borrows, closure captures, public API
region parameters, outlives constraints, escapes from local/Actor/Task scopes,
and suspension crossing, with explicit public lifetimes when inference is not
stable. The accepted language documents deliberately leave the public lifetime
boundary to an RFC.

No region variable, lifetime inference solver, outlives constraint, public
lifetime projection, escape checker, diagnostic, protocol, or placeholder G3
API was added.

## Normative traceability

- The G3 execution package is non-normative. Its inference goals cannot
  authorize lifetime syntax, public ABI, borrow escape rules, or diagnostics.
- OWN-3203 depends on the missing RFC-N303/RFC-0007 ownership and region
  authority. No accepted RFC-N303 or RFC-0007 exists; RFC-0001 remains a Draft
  under DEC-0018.
- `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` is Open and explicitly says it is
  undecided whether public lifetimes are fully inferred or must appear in
  signatures and compatibility checks. It blocks OWN-3203, OWN-3204, and
  OWN-3207.
- `GAP-OWNERSHIP-MODEL-001` is also Open and leaves Borrow, aliasing, region
  escape, Drop, Managed, and Profile boundaries unaccepted. Accepted DEC-0009
  only defines the v0.0.1 Seed value/mutable-place boundary.
- `docs/SEMANTICS.md` sketches lexical/non-lexical region concepts, says
  Borrow cannot outlive its object, requires Pin/Region constraints across
  suspension, and lists public lifetime inference as an unresolved RFC
  question. It does not define inference, constraints, public canonical
  projection, or migration behavior.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require accepted Ownership,
  Borrow, Region, Drop, memory-kind, Task/Actor, Native, and FFI contracts
  before v0.3 implementation.

## Current implementation evidence

- The workspace has no region variables, lexical/non-lexical lifetime solver,
  outlives graph, returned-borrow analysis, closure region capture, public
  lifetime projection, local/Actor/Task escape checker, or suspension crossing
  analysis. The Seed checker intentionally has no Borrow or lifetime Core.
- Existing Rust lifetimes, CFG scopes, VM frames, and source spans are
  implementation details, not Ling region semantics. No public ABI or
  separate-compilation compatibility rule exposes them.
- No diagnostic or fixture defines returned-borrow failure, closure capture,
  local/Actor/Task escape, public lifetime ambiguity, cross-package region
  compatibility, FFI region transfer, Unicode/CRLF/BOM span preservation, or
  interpreter/VM/Native differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Region/lifetime variables, lexical versus non-lexical scope, outlives
   constraints, inference algorithm, fixed points/termination, reborrowing,
   and interaction with Place, Copy/Move, Borrow, Resource, Managed, Traits,
   closures, aggregates, and generics.
2. Returned borrows, closure captures, local/Actor/Task escape, suspension and
   `await`, pinning, cancellation, Drop, public API region parameters, and
   separate-compilation/compatibility rules, including the explicit-vs-
   inferred public lifetime decision and migration path.
3. Checked Core and Semantic Graph/Audit Source projections, canonical bytes/
   Semantic IDs, Native/FFI/ABI and Profile boundaries, stable bilingual
   diagnostics for inference/escape/ambiguity failures, deterministic output,
   and Unicode 17.0.0 source-span handling.
4. Executable positive/negative/migration/cross-package/region-escape/FFI/
   property/fuzz/differential fixtures for nested scopes, loops, branches,
   returns, closures, mutable borrows, Task/Actor turns, suspension,
   cancellation, and interpreter/VM/Native parity without unchecked-AST
   execution.

Until those decisions are Accepted, implementing region inference would freeze
public API compatibility, lifetime safety, diagnostics, concurrency, ABI, FFI,
and backend legality that the language authority intentionally leaves open.

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
Seed Place lowering, future region/lifetime semantics, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`OWN-3203` can begin only after MEM-3101 through MEM-3104, OWN-3201/OWN-3202,
and RFC-0007 (or an accepted replacement) define memory kinds, Copy/Move,
Place/Borrow, Resource/Managed, Drop, suspension, public lifetime projection,
and FFI boundaries. The future implementation must preserve accepted Seed
behavior, consume accepted types and checked Core only, avoid Rust lifetime
leakage, and publish inference, escape, public-API, cross-package, FFI,
diagnostic, and interpreter/VM/Native evidence before exposing v0.3 regions.
