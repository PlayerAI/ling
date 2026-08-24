# EFF-2105 Authority Audit: Effect Fuzz and Property Tests

## Outcome

`EFF-2105` is ready for implementation. Accepted `RFC-0006`,
`DEC-0062`, and `DEC-0067` authorize the deterministic in-process model child;
accepted `DEC-0262` and completed `EFF-2104` now provide Handler interpreter,
bytecode 1.4, VM, residual-row, Fault, State, cancellation, resource, and
differential semantics. Accepted `DEC-0263` closes the final authority gap for
the internal checked-source generator, replay/shrinking bounds, and
property-oracle contract without adding a public protocol or changing language
semantics.

No Core generator, handler/effect oracle, residual-row comparator, new
property-test protocol, fuzz corpus, diagnostic allocation, or placeholder G2
API was added.

## Normative traceability

- The G2 execution package is non-normative; its fuzz/property-test outline
  does not authorize a new Typed Core generator, handler semantics, or a
  differential equivalence relation.
- EFF-2101 through EFF-2104 now have accepted model, checked lowering,
  interpreter, bytecode 1.4, verifier, VM, and differential authority.
- `docs/SEMANTICS.md` and DEC-0010 define current Seed closed-row
  propagation, `State<T>` visibility, and Capability behavior, but leave row
  polymorphism, handler elimination, residual rows, and State masking open.
- DEC-0013, RFC-0020, and DEC-0262 define the relevant Runtime Fault,
  cancellation, Handler residual-row, and differential boundaries. They do not
  select a general generated-input domain or deterministic shrink/replay rule.
- `GAP-EFFECT-HANDLER-001` is resolved for the Experimental v0.2 model by
  RFC-0006 and its accepted runtime/bytecode decisions; generated property
  evidence remains open under EFF-2105;
  `GAP-EFFECT-STATE-MASKING-001` is resolved by the accepted visible-State rule.

## Current implementation evidence

- `ling-effects` checks Seed rows and the accepted Handler Core residual-row
  projection, deterministic call-graph propagation, and module Capability
  closure. It has no general generated-program property oracle.
- `ling-eval` and `ling-vm` now execute checked Handler programs and the VM
  differential suite compares values, events, resume counts, committed Cell
  observations, Faults, cancellation boundaries, and deterministic bytecode.
  Those hand-built cases do not constitute a generated property domain or
  shrinking contract.
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

## Accepted implementation authority

Accepted DEC-0263 defines:

1. generator constraints and shrinking rules for only well-typed, bounded,
   source-mapped Core; recursion, allocation, depth, output, and diagnostic
   limits; and the definition of a host panic/infinite-recursion failure;
2. canonical ordering and stable comparison of Effect Rows, Fault facts,
   Semantic IDs, Audit Source, source spans, diagnostics, and migration
   fixtures, including Unicode/CRLF/BOM behavior within generated cases; and
3. executable positive/negative/differential fixtures for pure and
   effectful programs, single/nested handlers, propagation, resume
   cardinality, handler Faults, mutable State, cancellation, missing and
   unhandled operations, malformed bytecode, resource limits, deterministic
   output, and no unchecked-AST execution.

The implementation must follow those bounds exactly so a generator panic,
unstable shrink, biased input domain, or host observation cannot become an
accidental language guarantee.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0262,
the EFF-2104 implementation report, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-effects`, `crates/ling-eval`, `crates/ling-bytecode`,
`crates/ling-vm`, and the excluded `fuzz` workspace.

No compiler, interpreter, VM, bytecode, fuzz protocol, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed. The
bounded model-property child adds offline tests only.

## Intentionally deferred

The full `EFF-2105` target may now begin under Accepted DEC-0263. The
implementation must generate only checked, bounded Core,
compare stable semantic projections, retain reproducible seeds and minimized
corpus entries, and publish differential evidence without exposing a public
protocol. `EFF-2105-MODEL-PROPERTIES` remains complete under DEC-0067.
