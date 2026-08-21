# EFF-2105 Authority Audit: Effect Fuzz and Property Tests

## Outcome

`EFF-2105` is correctly recorded as `BlockedSpec`. The G2 plan requires a
generator for small well-typed Core programs and comparison of interpreter and
VM results, residual Effect Rows, and Fault categories. It also requires
bounded randomized inputs that cannot cause host panics, unbounded recursion,
or canonical-output drift. The handler, row-polymorphism, residual-effect,
Fault, and VM equivalence contracts needed to define those properties are not
accepted.

No Core generator, handler/effect oracle, residual-row comparator, new
property-test protocol, fuzz corpus, diagnostic allocation, or placeholder G2
API was added.

## Normative traceability

- The G2 execution package is non-normative; its fuzz/property-test outline
  does not authorize a new Typed Core generator, handler semantics, or a
  differential equivalence relation.
- EFF-2101 through EFF-2104 are `BlockedSpec`; without accepted Effect Row,
  handler Core, resume, and interpreter/VM contracts there is no valid
  handler-bearing input or expected result for a property oracle.
- `docs/SEMANTICS.md` and DEC-0010 define current Seed closed-row
  propagation, `State<T>` visibility, and Capability behavior, but leave row
  polymorphism, handler elimination, residual rows, and State masking open.
- DEC-0013 and RFC-0020 define current runtime Fault and host cancellation
  boundaries. They do not authorize handler Fault categories, residual-effect
  observations, randomized cancellation semantics, or a new differential
  protocol.
- `GAP-EFFECT-HANDLER-001` leaves matching, elimination, nesting, resumption,
  Capability interaction, unhandled failure, and differential evidence open;
  `GAP-EFFECT-STATE-MASKING-001` leaves mutable-State visibility open. Their
  candidate RFC-0006 is not Accepted.

## Current implementation evidence

- `ling-effects` checks only Seed's closed `EffectRow` labels (`Console.Write`
  and `State<T>`), deterministic call-graph propagation, and module
  Capability closure. It has no row variables, handler subtraction,
  residual-effect result, or property-test oracle.
- `ling-eval` and `ling-vm` already have a fixed Seed differential harness for
  logical host events, Unit results, and stable Runtime Fault projections. The
  harness accepts checked snapshots/verified bytecode, but it does not expose
  handler operations or residual Effect Rows.
- The excluded fuzz workspace currently covers source, lexer/parser,
  manifests, and bytecode decoding/verification. It has no well-typed Core
  generator, interpreter/VM effect differential target, bounded handler
  recursion model, or effect-specific corpus.
- Existing randomized or differential checks cannot be promoted to EFF-2105
  by inference: doing so would freeze unaccepted handler semantics and could
  make host behavior, Fault mapping, or canonical bytes part of the language
  contract.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the complete checked Typed Core input domain, Effect Row variables and
   canonicalization, handler operation/continuation model, residual-row/result
   representation, and State/Capability/aliasing rules;
2. the interpreter reference semantics and VM lowering/ABI, including the
   equivalence relation for values, committed host effects, Fault categories,
   cancellation, resource limits, and malformed input rejection;
3. generator constraints and shrinking rules for only well-typed, bounded,
   source-mapped Core; recursion, allocation, depth, output, and diagnostic
   limits; and the definition of a host panic/infinite-recursion failure;
4. canonical ordering and stable comparison of Effect Rows, Fault facts,
   Semantic IDs, Audit Source, source spans, diagnostics, and migration
   fixtures, including Unicode/CRLF/BOM behavior; and
5. executable positive/negative/migration/differential fixtures for pure and
   effectful programs, single/nested handlers, propagation, resume
   cardinality, handler Faults, mutable State, cancellation, missing and
   unhandled operations, malformed bytecode, resource limits, deterministic
   output, and no unchecked-AST execution.

Until these decisions are Accepted, a property test could encode the wrong
handler semantics, accept divergent interpreter/VM results, hide residual
effects, or turn a generator panic into an accidental language guarantee.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-effects`, `crates/ling-eval`, `crates/ling-bytecode`,
`crates/ling-vm`, and the excluded `fuzz` workspace.

No compiler, interpreter, VM, bytecode, fuzz protocol, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`EFF-2105` can begin only after EFF-2101 through EFF-2104 and an Accepted
RFC-0006 (or replacement), followed by explicit generator, residual-row, and
interpreter/VM equivalence authority. The future implementation must generate
only checked, bounded Core, compare stable semantic projections, retain
reproducible seeds and minimized corpus entries, and publish differential
evidence without exposing a new public protocol before acceptance.
