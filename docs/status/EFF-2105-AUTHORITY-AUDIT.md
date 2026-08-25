# EFF-2105 Authority Audit: Effect Fuzz and Property Tests

## Outcome

`EFF-2105` is complete in commit `3517ffcc`. Accepted `RFC-0006`,
`DEC-0062`, and `DEC-0067` authorize the deterministic in-process model child;
Accepted `DEC-0262` and completed `EFF-2104` provide Handler interpreter,
bytecode 1.4, VM, residual-row, Fault, State, cancellation, resource, and
differential semantics. Accepted `DEC-0263` closes the final authority gap for
the internal checked-source generator, replay/shrinking bounds, and
property-oracle contract without adding a public protocol or changing language
semantics.

The completed implementation adds only an internal checked-source generator,
residual-row comparator, differential oracle, and bounded shrinker. It adds no
public property-test protocol, persistent fuzz corpus, diagnostic allocation,
or placeholder G2 API.

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
  evidence is complete under EFF-2105;
  `GAP-EFFECT-STATE-MASKING-001` is resolved by the accepted visible-State rule.

## Current implementation evidence

- `ling-effects` checks Seed rows and the accepted Handler Core residual-row
  projection, deterministic call-graph propagation, and module Capability
  closure. EFF-2105 compares its canonical named-definition rows with exact
  verified bytecode rows.
- `ling-eval` and `ling-vm` execute checked Handler programs. The EFF-2105
  suite adds 96 generated checked-source cases to the retained hand-built
  differential, Fault, cancellation, and resource evidence.
- The excluded fuzz workspace continues to cover malformed source,
  lexer/parser, manifests, and bytecode decoding/verification. EFF-2105 is a
  separate in-process well-typed source generator and writes no corpus.
- `crates/ling-effects/tests/model_properties.rs` provides only the bounded
  DEC-0067 permutation corpus for canonical rows, solver substitutions,
  handler residuals, and path-free graph/Core bytes; it is not the full target.
- Commit `3517ffcc` implements only the generated domain and comparison rules
  accepted by DEC-0263; it does not infer semantics from existing randomized
  or hand-built checks.

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

The implementation follows those bounds exactly so a generator panic,
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

Source semantics, the interpreter, VM results, diagnostics, schemas, Semantic
IDs, Program IDs, source spans, public fuzz protocols, and Unicode 17.0.0 did
not change. The accepted row oracle corrected bytecode 1.4 lowering so ordinary
mutable lexical bindings use existing Cells and retain `State<T>`; bytecode
1.0–1.3 remain unchanged. The bounded model-property child remains separate.

## Intentionally deferred

The full `EFF-2105` target is complete under Accepted DEC-0263; see
`docs/status/EFF-2105-IMPLEMENTATION-REPORT.md`. Public replay formats,
persistent automatic corpus writes, new Effect producers, Task/Actor crossing,
Native/Wasm comparison, and Stable compatibility remain deferred.
`EFF-2105-MODEL-PROPERTIES` remains complete under DEC-0067.
