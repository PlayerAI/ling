# EFF-2105 Authority Audit: Effect Fuzz and Property Tests

## Outcome

`EFF-2105` remains correctly recorded as `BlockedSpec` for its full runtime and
differential target. Accepted `RFC-0006` and `DEC-0062` now authorize the
deterministic in-process model boundary, and `DEC-0067` closes the bounded
`EFF-2105-MODEL-PROPERTIES` evidence child. The parent still requires a
generator for small well-typed Core programs and comparison of interpreter and
VM results, residual Effect Rows, and Fault categories. It also requires
bounded randomized inputs that cannot cause host panics, unbounded recursion,
or canonical-output drift.

No Core generator, handler/effect oracle, residual-row comparator, new
property-test protocol, fuzz corpus, diagnostic allocation, or placeholder G2
API was added.

## Normative traceability

- The G2 execution package is non-normative; its fuzz/property-test outline
  does not authorize a new Typed Core generator, handler semantics, or a
  differential equivalence relation.
- EFF-2101 and EFF-2102, together with the bounded EFF-2103 Core/source/HIR
  children, now have accepted model authority. EFF-2104 remains `BlockedSpec`;
  without accepted handler runtime and interpreter/VM contracts there is no
  valid handler-bearing input or expected result for the full property oracle.
- `docs/SEMANTICS.md` and DEC-0010 define current Seed closed-row
  propagation, `State<T>` visibility, and Capability behavior, but leave row
  polymorphism, handler elimination, residual rows, and State masking open.
- DEC-0013 and RFC-0020 define current runtime Fault and host cancellation
  boundaries. They do not authorize handler Fault categories, residual-effect
  observations, randomized cancellation semantics, or a new differential
  protocol.
- `GAP-EFFECT-HANDLER-001` is resolved for the Experimental v0.2 model by
  RFC-0006, while runtime and differential evidence remain open;
  `GAP-EFFECT-STATE-MASKING-001` is resolved by the accepted visible-State rule.

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
- `crates/ling-effects/tests/model_properties.rs` provides only the bounded
  DEC-0067 permutation corpus for canonical rows, solver substitutions,
  handler residuals, and path-free graph/Core bytes; it is not the full target.
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

Until runtime, generator, and differential decisions are Accepted, a full
property test could encode the wrong handler execution semantics, accept
divergent interpreter/VM results, hide residual effects, or turn a generator
panic into an accidental language guarantee.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-effects`, `crates/ling-eval`, `crates/ling-bytecode`,
`crates/ling-vm`, and the excluded `fuzz` workspace.

No compiler, interpreter, VM, bytecode, fuzz protocol, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed. The
bounded model-property child adds offline tests only.

## Intentionally deferred

The full `EFF-2105` target can begin only after EFF-2104 and explicit
generator, residual-row, and interpreter/VM equivalence authority. The future
implementation must generate only checked, bounded Core, compare stable
semantic projections, retain reproducible seeds and minimized corpus entries,
and publish differential evidence without exposing a new public protocol
before acceptance. `EFF-2105-MODEL-PROPERTIES` is complete under DEC-0067 and
does not remove these parent blockers.
